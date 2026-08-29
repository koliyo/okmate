//! Portable Open Knowledge Format (OKF) parsing, validation, search, graph, and artifact engine.

pub(crate) mod artifact;
pub(crate) mod ast;
pub(crate) mod benchmark;
pub(crate) mod diagnostic;
pub(crate) mod frontmatter;
pub(crate) mod graph;
mod highlight;
pub(crate) mod load;
pub(crate) mod markdown;
pub(crate) mod parse_cache;
pub(crate) mod preview;
pub(crate) mod provenance;
pub(crate) mod review;
pub(crate) mod search;
pub(crate) mod validate;

use std::collections::BTreeSet;
use std::path::Path;
use std::time::{Duration, Instant};

use anyhow::{Result, bail};

use artifact::{ConceptInspect, absolute};
use benchmark::run_benchmark;
use graph::resolve_graph;
use load::{discover_markdown, parse_files_parallel, relative_path};
use search::search as search_chunks;
use validate::{
    validate_index_membership, validate_lifecycle_and_sources, validate_lifecycle_and_sources_with,
    validate_route_collisions, validate_unique_ids,
};

pub use artifact::build_artifacts;
pub use ast::{
    BuildSummary, Bundle, CheckReport, Concept, Edge, Heading, HeadingSection, Index, InspectKind,
    KnowledgeFilter, Link, LoadOptions, Log, Profile, Span, TrustTier,
};
pub use benchmark::{
    RetrievalBenchmark, RetrievalQuestion, RetrievalQuestionResult, RetrievalReport,
};
pub use diagnostic::{Diagnostic, Severity, SourceLocation};
pub use graph::published_href;
pub use parse_cache::{PARSE_CACHE_VERSION, ParseCache};
pub use preview::{PreviewTarget, resolve_preview_path};
pub use review::{ActionKind, ConceptAction, classify_concept_action};
pub use search::{concept_is_stale, concept_trust_tier};
pub use validate::{latest_human_verification, metadata_string_array, string_field};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LoadTimings {
    pub discover: Duration,
    pub parse: Duration,
    pub graph: Duration,
    pub provenance: Option<Duration>,
    pub parse_cache_hits: u32,
    pub parse_cache_misses: u32,
}

#[derive(Debug)]
pub struct LoadResult {
    pub bundle: Bundle,
    pub timings: LoadTimings,
}

pub fn check(root: &Path, profile: Profile) -> Result<CheckReport> {
    let bundle = load(root, profile)?;
    Ok(CheckReport {
        diagnostics: bundle.diagnostics,
    })
}

pub fn load(root: &Path, profile: Profile) -> Result<Bundle> {
    Ok(load_timed(root, LoadOptions::new(profile))?.bundle)
}

pub fn load_timed(root: &Path, options: LoadOptions) -> Result<LoadResult> {
    load_with_cache(root, options, None)
}

pub fn load_with_cache(
    root: &Path,
    options: LoadOptions,
    mut cache: Option<&mut ParseCache>,
) -> Result<LoadResult> {
    let root = absolute(root)?;
    if !root.is_dir() {
        bail!("knowledge bundle {} is not a directory", root.display());
    }

    let discover_started = Instant::now();
    let mut paths = Vec::new();
    discover_markdown(&root, &mut paths)?;
    paths.sort();
    let discover = discover_started.elapsed();

    let parse_started = Instant::now();
    let mut concepts = Vec::new();
    let mut indexes = Vec::new();
    let mut logs = Vec::new();
    let mut diagnostics = Vec::new();
    let mut parse_cache_hits = 0;
    let mut parse_cache_misses = 0;
    let mut live_paths = BTreeSet::new();
    if let Some(cache) = cache.as_mut() {
        cache.begin(options.profile);
    }

    let mut misses = Vec::new();
    for path in paths {
        let relative = relative_path(&root, &path);
        live_paths.insert(relative.clone());
        let fingerprint = parse_cache::file_fingerprint(&path);
        if let Some(document) = cache
            .as_ref()
            .and_then(|cache| fingerprint.and_then(|fingerprint| cache.get(&relative, fingerprint)))
            .cloned()
        {
            parse_cache::apply_cached(
                &document,
                &mut concepts,
                &mut indexes,
                &mut logs,
                &mut diagnostics,
            );
            parse_cache_hits += 1;
            continue;
        }
        parse_cache_misses += 1;
        misses.push((path, relative, fingerprint));
    }

    let parsed = parse_files_parallel(&root, options.profile, misses)?;
    for parsed in parsed {
        concepts.extend(parsed.concepts);
        indexes.extend(parsed.indexes);
        logs.extend(parsed.logs);
        diagnostics.extend(parsed.diagnostics);
        if let (Some(cache), Some(fingerprint), Some(document)) =
            (cache.as_mut(), parsed.fingerprint, parsed.document)
        {
            cache.insert(parsed.relative, fingerprint, document);
        }
    }
    if let Some(cache) = cache.as_mut() {
        cache.retain_paths(&live_paths);
    }

    concepts.sort_by(|a, b| a.id.cmp(&b.id));
    indexes.sort_by(|a, b| a.path.cmp(&b.path));
    logs.sort_by(|a, b| a.path.cmp(&b.path));
    validate_unique_ids(&concepts, &mut diagnostics);
    validate_route_collisions(&concepts, &indexes, &mut diagnostics);
    if options.profile == Profile::Strict {
        validate_index_membership(&concepts, &indexes, &mut diagnostics);
    }
    let parse = parse_started.elapsed();

    let graph_started = Instant::now();
    let graph = resolve_graph(&concepts, &indexes, &mut diagnostics);
    let graph_duration = graph_started.elapsed();

    let provenance = if options.provenance {
        let provenance_started = Instant::now();
        validate_lifecycle_and_sources(&root, &concepts, &mut diagnostics);
        Some(provenance_started.elapsed())
    } else if options.profile == Profile::Strict {
        validate_lifecycle_and_sources_with(&root, &concepts, &mut diagnostics, false);
        Some(Duration::ZERO)
    } else {
        None
    };

    diagnostics.sort_by(|a, b| {
        (&a.path, a.location.as_ref().map(|span| span.start), a.code).cmp(&(
            &b.path,
            b.location.as_ref().map(|span| span.start),
            b.code,
        ))
    });
    let version = indexes
        .iter()
        .find(|index| index.path == "index.md")
        .and_then(|index| index.version.clone());

    Ok(LoadResult {
        bundle: Bundle {
            root,
            version,
            concepts,
            indexes,
            logs,
            graph,
            diagnostics,
        },
        timings: LoadTimings {
            discover,
            parse,
            graph: graph_duration,
            provenance,
            parse_cache_hits,
            parse_cache_misses,
        },
    })
}

pub fn inspect(
    root: &Path,
    kind: InspectKind,
    concept_id: Option<&str>,
    profile: Profile,
) -> Result<String> {
    inspect_filtered(root, kind, concept_id, profile, &KnowledgeFilter::default())
}

pub fn inspect_filtered(
    root: &Path,
    kind: InspectKind,
    concept_id: Option<&str>,
    profile: Profile,
    filter: &KnowledgeFilter,
) -> Result<String> {
    let bundle = load(root, profile)?;
    match kind {
        InspectKind::Catalog => {
            let filtered = bundle
                .concepts
                .iter()
                .filter(|concept| filter.matches(concept))
                .map(ConceptInspect::from)
                .collect::<Vec<_>>();
            Ok(serde_json::to_string_pretty(&filtered)?)
        }
        InspectKind::Concept => {
            let Some(id) = concept_id else {
                bail!("inspect concept requires a concept id");
            };
            let concept = find_concept(&bundle.concepts, id)?;
            Ok(serde_json::to_string_pretty(&ConceptInspect::from(
                concept,
            ))?)
        }
        InspectKind::Graph => Ok(serde_json::to_string_pretty(&bundle.graph)?),
    }
}

pub fn search(
    root: &Path,
    query: &str,
    profile: Profile,
    filter: &KnowledgeFilter,
) -> Result<String> {
    let bundle = load(root, profile)?;
    Ok(serde_json::to_string_pretty(&search_chunks(
        &bundle, query, filter,
    ))?)
}

pub fn benchmark_retrieval(
    root: &Path,
    benchmark_path: &Path,
    profile: Profile,
) -> Result<RetrievalReport> {
    let bundle = load(root, profile)?;
    run_benchmark(&bundle, benchmark_path)
}

pub fn build(root: &Path, output: &Path, profile: Profile) -> Result<BuildSummary> {
    let bundle = load(root, profile)?;
    if bundle.has_errors() {
        bail!("knowledge bundle has validation errors");
    }
    build_artifacts(&bundle, output)
}

fn find_concept<'a>(concepts: &'a [Concept], id: &str) -> Result<&'a Concept> {
    if let Some(concept) = concepts.iter().find(|concept| concept.id == id) {
        return Ok(concept);
    }
    let matches: Vec<_> = concepts
        .iter()
        .filter(|concept| concept.id.rsplit('/').next() == Some(id))
        .collect();
    match matches.as_slice() {
        [concept] => Ok(concept),
        [] => bail!("unknown concept `{id}`"),
        _ => bail!(
            "ambiguous concept stem `{id}`; use a full id such as `{}`",
            matches[0].id
        ),
    }
}

#[cfg(test)]
mod public_surface {
    #[test]
    fn parse_helpers_are_crate_visible() {
        let _ = crate::validate::days_from_civil;
        let _ = crate::frontmatter::lines_with_offsets;
        let _ = crate::provenance::git_repository_root;
    }
}
