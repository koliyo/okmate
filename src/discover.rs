use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::Serialize;

use crate::CheckFormat;

pub const DEFAULT_MAX_DEPTH: u32 = 5;
pub const DEFAULT_MAX_VISITS: u32 = 512;

const SKIP_DIR_NAMES: &[&str] = &[
    ".git",
    ".svn",
    ".hg",
    "target",
    "node_modules",
    "dist",
    "build",
    "vendor",
    ".direnv",
    ".venv",
    "venv",
    "__pycache__",
    ".okmate",
];

#[derive(Clone, Debug)]
pub struct DiscoverOptions {
    pub start: PathBuf,
    pub max_depth: u32,
    pub max_visits: u32,
    pub format: CheckFormat,
}

#[derive(Clone, Debug, Serialize)]
pub struct DiscoverReport {
    pub start: String,
    pub bundles: Vec<FoundBundle>,
    pub markerless: Vec<MarkerlessCandidate>,
    pub truncated: bool,
    pub visits: u32,
    pub max_depth: u32,
    pub max_visits: u32,
}

#[derive(Clone, Debug, Serialize)]
pub struct FoundBundle {
    pub path: String,
    pub relative: String,
    pub version: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct MarkerlessCandidate {
    pub path: String,
    pub relative: String,
    pub has_index: bool,
    pub markdown_files: usize,
    pub evidence: String,
}

struct ScanState {
    start: PathBuf,
    max_depth: u32,
    max_visits: u32,
    visits: u32,
    truncated: bool,
    bundles: Vec<FoundBundle>,
    markerless: Vec<MarkerlessCandidate>,
}

pub fn run(opts: DiscoverOptions) -> Result<()> {
    let report = scan(&opts)?;
    match opts.format {
        CheckFormat::Json => println!("{}", serde_json::to_string_pretty(&report)?),
        CheckFormat::Terminal => print_terminal(&report),
    }
    Ok(())
}

pub fn scan(opts: &DiscoverOptions) -> Result<DiscoverReport> {
    let start = fs::canonicalize(&opts.start)
        .with_context(|| format!("failed to resolve {}", opts.start.display()))?;
    if !start.is_dir() {
        bail!("{} is not a directory", start.display());
    }
    let mut state = ScanState {
        start: start.clone(),
        max_depth: opts.max_depth,
        max_visits: opts.max_visits,
        visits: 0,
        truncated: false,
        bundles: Vec::new(),
        markerless: Vec::new(),
    };
    state.walk(&start, 0)?;
    state
        .bundles
        .sort_by(|left, right| left.relative.cmp(&right.relative));
    let bundle_paths: Vec<PathBuf> = state
        .bundles
        .iter()
        .map(|found| PathBuf::from(&found.path))
        .collect();
    state.markerless.retain(|candidate| {
        let path = Path::new(&candidate.path);
        !bundle_paths.iter().any(|root| path.starts_with(root))
    });
    state
        .markerless
        .sort_by(|left, right| left.relative.cmp(&right.relative));
    Ok(DiscoverReport {
        start: start.display().to_string(),
        bundles: state.bundles,
        markerless: state.markerless,
        truncated: state.truncated,
        visits: state.visits,
        max_depth: opts.max_depth,
        max_visits: opts.max_visits,
    })
}

pub fn is_bundle_dir(path: &Path) -> bool {
    let index = path.join("index.md");
    let Ok(source) = fs::read_to_string(index) else {
        return false;
    };
    okf::is_bundle_root_index(&source)
}

pub fn resolve_container(path: &Path) -> Result<okf::PreviewTarget> {
    if path.is_file() {
        return okf::resolve_preview_path(path);
    }
    let canonical =
        fs::canonicalize(path).with_context(|| format!("failed to resolve {}", path.display()))?;
    if is_bundle_dir(&canonical) {
        return Ok(okf::PreviewTarget::bundle(canonical));
    }
    let report = scan(&DiscoverOptions {
        start: canonical.clone(),
        max_depth: DEFAULT_MAX_DEPTH,
        max_visits: DEFAULT_MAX_VISITS,
        format: CheckFormat::Terminal,
    })?;
    match report.bundles.as_slice() {
        [only] => Ok(okf::PreviewTarget::bundle(PathBuf::from(&only.path))),
        [] => bail!("{}", empty_message(&report)),
        _ => bail!("{}", ambiguous_message(&report)),
    }
}

impl ScanState {
    fn walk(&mut self, dir: &Path, depth: u32) -> Result<()> {
        if self.visits >= self.max_visits {
            self.truncated = true;
            return Ok(());
        }
        self.visits += 1;
        self.inspect_dir(dir)?;
        if depth >= self.max_depth {
            return Ok(());
        }
        let mut names: Vec<_> = fs::read_dir(dir)
            .with_context(|| format!("failed to read {}", dir.display()))?
            .collect::<std::io::Result<Vec<_>>>()?;
        names.sort_by_key(|entry| entry.file_name());
        for entry in names {
            let file_type = entry.file_type()?;
            if file_type.is_symlink() || !file_type.is_dir() {
                continue;
            }
            let name = entry.file_name();
            let Some(name) = name.to_str() else {
                continue;
            };
            if skip_dir(name) {
                continue;
            }
            self.walk(&entry.path(), depth + 1)?;
            if self.truncated {
                return Ok(());
            }
        }
        Ok(())
    }

    fn inspect_dir(&mut self, dir: &Path) -> Result<()> {
        let relative = relative_to(&self.start, dir);
        let index = dir.join("index.md");
        if index.is_file() {
            let source = fs::read_to_string(&index)
                .with_context(|| format!("failed to read {}", index.display()))?;
            if okf::is_bundle_root_index(&source) {
                let version = source
                    .lines()
                    .find_map(|line| {
                        line.trim_start()
                            .strip_prefix("okf_version:")
                            .map(|value| value.trim().trim_matches(['"', '\'']).to_string())
                    })
                    .filter(|value| !value.is_empty());
                self.bundles.push(FoundBundle {
                    path: dir.display().to_string(),
                    relative,
                    version,
                });
                return Ok(());
            }
            self.markerless.push(MarkerlessCandidate {
                path: dir.display().to_string(),
                relative,
                has_index: true,
                markdown_files: count_markdown(dir),
                evidence: "index.md without okf_version (collection or unversioned tree)".into(),
            });
            return Ok(());
        }
        let markdown_files = count_markdown(dir);
        if markdown_files > 0 {
            self.markerless.push(MarkerlessCandidate {
                path: dir.display().to_string(),
                relative,
                has_index: false,
                markdown_files,
                evidence: format!("{markdown_files} Markdown file(s) and no versioned root index"),
            });
        }
        Ok(())
    }
}

fn count_markdown(dir: &Path) -> usize {
    let Ok(entries) = fs::read_dir(dir) else {
        return 0;
    };
    entries
        .flatten()
        .filter(|entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some("md"))
        .count()
}

fn skip_dir(name: &str) -> bool {
    SKIP_DIR_NAMES.contains(&name) || (name.starts_with('.') && name != ".okf")
}

fn relative_to(start: &Path, dir: &Path) -> String {
    if start == dir {
        return ".".into();
    }
    dir.strip_prefix(start)
        .map(|path| path.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| dir.display().to_string())
}

fn print_terminal(report: &DiscoverReport) {
    if report.truncated {
        println!(
            "Scan truncated after {} directories (max-depth {}, max-visits {}).",
            report.visits, report.max_depth, report.max_visits
        );
    }
    match report.bundles.as_slice() {
        [] => println!("No OKF bundle roots under {}.", report.start),
        [only] => println!(
            "1 bundle under {}: {}",
            report.start,
            display_rel(&only.relative)
        ),
        bundles => {
            println!(
                "{} bundles under {} (ambiguous; pass one path):",
                bundles.len(),
                report.start
            );
            for bundle in bundles {
                println!("  {}", display_rel(&bundle.relative));
            }
        }
    }
    if !report.markerless.is_empty() {
        println!("Markerless candidates (not claimed as bundles):");
        for candidate in &report.markerless {
            println!(
                "  {}: {}",
                display_rel(&candidate.relative),
                candidate.evidence
            );
        }
    }
}

fn display_rel(relative: &str) -> &str {
    if relative.is_empty() { "." } else { relative }
}

fn empty_message(report: &DiscoverReport) -> String {
    let mut message = format!(
        "{} is not an OKF bundle and no versioned root index was found",
        report.start
    );
    if !report.markerless.is_empty() {
        message.push_str("; markerless candidates: ");
        message.push_str(
            &report
                .markerless
                .iter()
                .map(|candidate| display_rel(&candidate.relative).to_string())
                .collect::<Vec<_>>()
                .join(", "),
        );
    }
    message
}

fn ambiguous_message(report: &DiscoverReport) -> String {
    let listed = report
        .bundles
        .iter()
        .map(|bundle| display_rel(&bundle.relative).to_string())
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "ambiguous OKF roots under {} (not preferring knowledge/): {listed}",
        report.start
    )
}
