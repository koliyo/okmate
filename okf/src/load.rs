use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde_json::Value;

use crate::ast::{Concept, Index, Log, Profile, Span};
use crate::diagnostic::Diagnostic;
use crate::frontmatter::{lines_with_offsets, location, parse_yaml_mapping, split_frontmatter};
use crate::markdown::{MarkdownOutput, parse_markdown_body};
use crate::parse_cache;
use crate::validate::{collect_source_ids, is_date, validate_metadata};

pub(crate) struct ParsedFile {
    pub(crate) relative: String,
    pub(crate) fingerprint: Option<parse_cache::FileFingerprint>,
    pub(crate) concepts: Vec<Concept>,
    pub(crate) indexes: Vec<Index>,
    pub(crate) logs: Vec<Log>,
    pub(crate) diagnostics: Vec<Diagnostic>,
    pub(crate) document: Option<parse_cache::CachedDocument>,
}

pub(crate) fn parse_files_parallel(
    root: &Path,
    profile: Profile,
    misses: Vec<(PathBuf, String, Option<parse_cache::FileFingerprint>)>,
) -> Result<Vec<ParsedFile>> {
    if misses.len() <= 1 {
        return misses
            .into_iter()
            .map(|(path, relative, fingerprint)| {
                parse_one_file(root, &path, relative, fingerprint, profile)
            })
            .collect();
    }

    let workers = std::thread::available_parallelism()
        .map(std::num::NonZero::get)
        .unwrap_or(1)
        .min(misses.len());
    let remaining = misses.len();
    let queue = std::sync::Mutex::new(misses.into_iter());
    std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for _ in 0..workers {
            handles.push(scope.spawn(|| {
                let mut parsed = Vec::new();
                loop {
                    let next = queue
                        .lock()
                        .unwrap_or_else(|poisoned| poisoned.into_inner())
                        .next();
                    let Some((path, relative, fingerprint)) = next else {
                        break;
                    };
                    parsed.push(parse_one_file(root, &path, relative, fingerprint, profile)?);
                }
                Ok::<Vec<ParsedFile>, anyhow::Error>(parsed)
            }));
        }
        let mut parsed = Vec::with_capacity(remaining);
        for handle in handles {
            let chunk = match handle.join() {
                Ok(chunk) => chunk?,
                Err(_) => bail!("parse worker panicked"),
            };
            parsed.extend(chunk);
        }
        Ok(parsed)
    })
}

fn parse_one_file(
    root: &Path,
    path: &Path,
    relative: String,
    fingerprint: Option<parse_cache::FileFingerprint>,
    profile: Profile,
) -> Result<ParsedFile> {
    let mut concepts = Vec::new();
    let mut indexes = Vec::new();
    let mut logs = Vec::new();
    let mut diagnostics = Vec::new();
    let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    let source = match String::from_utf8(bytes) {
        Ok(source) => source,
        Err(_) => {
            diagnostics.push(Diagnostic::error(
                "OKF1001",
                relative.clone(),
                None,
                "document is not valid UTF-8",
            ));
            let document = Some(parse_cache::capture_cached(&[], &[], &[], &diagnostics));
            return Ok(ParsedFile {
                relative,
                fingerprint,
                concepts,
                indexes,
                logs,
                diagnostics,
                document,
            });
        }
    };
    match path.file_name().and_then(|name| name.to_str()) {
        Some("index.md") => parse_index(root, &relative, &source, &mut indexes, &mut diagnostics),
        Some("log.md") => parse_log(&relative, &source, &mut logs, &mut diagnostics),
        _ => parse_concept(&relative, &source, profile, &mut concepts, &mut diagnostics),
    }
    let document = Some(parse_cache::capture_cached(
        &concepts,
        &indexes,
        &logs,
        &diagnostics,
    ));
    Ok(ParsedFile {
        relative,
        fingerprint,
        concepts,
        indexes,
        logs,
        diagnostics,
        document,
    })
}

fn parse_concept(
    relative: &str,
    source: &str,
    profile: Profile,
    concepts: &mut Vec<Concept>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let frontmatter = match split_frontmatter(source, true) {
        Ok(Some(frontmatter)) => frontmatter,
        Ok(None) => return,
        Err(message) => {
            diagnostics.push(Diagnostic::error(
                "OKF1002",
                relative,
                Some(location(source, Span::point(0))),
                message,
            ));
            let body = Span::new(0, source.len());
            let parsed = parse_markdown_body(relative, source, body, diagnostics);
            push_partial_concept(relative, source, body, parsed, concepts);
            return;
        }
    };
    let metadata = match parse_yaml_mapping(frontmatter.yaml.of(source)) {
        Ok(metadata) => metadata,
        Err(message) => {
            diagnostics.push(Diagnostic::error(
                "OKF1003",
                relative,
                Some(location(source, frontmatter.yaml)),
                message,
            ));
            let parsed = parse_markdown_body(relative, source, frontmatter.body, diagnostics);
            push_partial_concept(relative, source, frontmatter.body, parsed, concepts);
            return;
        }
    };
    validate_metadata(
        relative,
        source,
        frontmatter.yaml,
        &metadata,
        profile,
        diagnostics,
    );

    let parsed = parse_markdown_body(relative, source, frontmatter.body, diagnostics);

    let (footnote_ids, defined_footnotes) = (parsed.footnote_ids, parsed.defined_footnotes);
    let source_ids = collect_source_ids(relative, &metadata, diagnostics);
    for footnote in &footnote_ids {
        if !source_ids.contains(footnote) {
            diagnostics.push(Diagnostic::error(
                "OKF4001",
                relative,
                None,
                format!("footnote `{footnote}` has no matching sources[].id"),
            ));
        }
        if !defined_footnotes.contains(footnote) {
            diagnostics.push(Diagnostic::error(
                "OKF4003",
                relative,
                None,
                format!("footnote `{footnote}` has no definition in the body"),
            ));
        }
    }
    for source_id in &source_ids {
        if !footnote_ids.contains(source_id) {
            diagnostics.push(Diagnostic::warning(
                "OKF4002",
                relative,
                None,
                format!("source id `{source_id}` is not used by a body footnote"),
            ));
        }
    }

    let id = relative.strip_suffix(".md").unwrap_or(relative).to_string();
    concepts.push(Concept {
        id,
        path: relative.to_string(),
        metadata,
        body_span: frontmatter.body,
        body_location: location(source, frontmatter.body),
        headings: parsed.headings,
        heading_sections: parsed.heading_sections,
        links: parsed.links,
        source_ids,
        footnote_ids,
        article_html: parsed.article_html,
    });
}

fn push_partial_concept(
    relative: &str,
    source: &str,
    body: Span,
    parsed: MarkdownOutput,
    concepts: &mut Vec<Concept>,
) {
    let id = relative.strip_suffix(".md").unwrap_or(relative).to_string();
    concepts.push(Concept {
        id,
        path: relative.to_string(),
        metadata: BTreeMap::new(),
        body_span: body,
        body_location: location(source, body),
        headings: parsed.headings,
        heading_sections: parsed.heading_sections,
        links: parsed.links,
        source_ids: BTreeSet::new(),
        footnote_ids: parsed.footnote_ids,
        article_html: parsed.article_html,
    });
}

fn parse_index(
    root: &Path,
    relative: &str,
    source: &str,
    indexes: &mut Vec<Index>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let is_root = root.join(relative) == root.join("index.md");
    let mut version = None;
    let body = match split_frontmatter(source, false) {
        Ok(Some(frontmatter)) if is_root => {
            match parse_yaml_mapping(frontmatter.yaml.of(source)) {
                Ok(metadata) => {
                    for key in metadata.keys() {
                        if key != "okf_version" {
                            diagnostics.push(Diagnostic::error(
                                "OKF1011",
                                relative,
                                Some(location(source, frontmatter.yaml)),
                                format!("root index frontmatter may only contain `okf_version`, found `{key}`"),
                            ));
                        }
                    }
                    match metadata.get("okf_version").and_then(Value::as_str) {
                        Some("0.2") => version = Some("0.2".to_string()),
                        Some(other) => diagnostics.push(Diagnostic::error(
                            "OKF1012",
                            relative,
                            Some(location(source, frontmatter.yaml)),
                            format!("unsupported okf_version `{other}`"),
                        )),
                        None => diagnostics.push(Diagnostic::error(
                            "OKF1012",
                            relative,
                            Some(location(source, frontmatter.yaml)),
                            "okf_version must be a string",
                        )),
                    }
                }
                Err(message) => diagnostics.push(Diagnostic::error(
                    "OKF1003",
                    relative,
                    Some(location(source, frontmatter.yaml)),
                    message,
                )),
            }
            frontmatter.body
        }
        Ok(Some(frontmatter)) => {
            diagnostics.push(Diagnostic::error(
                "OKF1011",
                relative,
                Some(location(source, frontmatter.yaml)),
                "non-root index.md must not contain frontmatter",
            ));
            frontmatter.body
        }
        Ok(None) => Span::new(0, source.len()),
        Err(message) => {
            diagnostics.push(Diagnostic::error(
                "OKF1002",
                relative,
                Some(location(source, Span::point(0))),
                message,
            ));
            Span::new(0, source.len())
        }
    };
    let parsed = parse_markdown_body(relative, source, body, diagnostics);
    indexes.push(Index {
        path: relative.to_string(),
        version,
        body_span: body,
        headings: parsed.headings,
        links: parsed.links,
        article_html: parsed.article_html,
    });
}

fn parse_log(relative: &str, source: &str, logs: &mut Vec<Log>, diagnostics: &mut Vec<Diagnostic>) {
    if source.starts_with("---\n") || source.starts_with("---\r\n") {
        diagnostics.push(Diagnostic::error(
            "OKF1021",
            relative,
            Some(location(source, Span::point(0))),
            "log.md must not contain frontmatter",
        ));
    }
    for (offset, line) in lines_with_offsets(source) {
        if let Some(date) = line.trim_end_matches(['\r', '\n']).strip_prefix("## ")
            && !is_date(date)
        {
            diagnostics.push(Diagnostic::error(
                "OKF1022",
                relative,
                Some(location(source, Span::new(offset, offset + line.len()))),
                "log date headings must use YYYY-MM-DD",
            ));
        }
    }
    logs.push(Log {
        path: relative.to_string(),
        body_span: location(source, Span::new(0, source.len())),
    });
}

pub(crate) fn discover_markdown(directory: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    let mut entries = fs::read_dir(directory)
        .with_context(|| format!("failed to read {}", directory.display()))?
        .collect::<std::io::Result<Vec<_>>>()?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let path = entry.path();
        if entry.file_name().as_encoded_bytes().starts_with(b".") {
            continue;
        }
        if path.is_dir() {
            discover_markdown(&path, out)?;
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("md") {
            out.push(path);
        }
    }
    Ok(())
}

pub(crate) fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
