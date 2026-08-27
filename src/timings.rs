use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use okf::{LoadOptions, LoadTimings, ParseCache, Profile};
use serde::Serialize;

use crate::preview::view_load_options;
use crate::site;
use crate::workspace::{Workspace, id_from_path, parse_cache_dir};

pub const TIMINGS_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimingsFormat {
    Terminal,
    Json,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimingsScenario {
    Load,
    Site,
    Click,
    Review,
    Log,
    Watch,
    All,
}

pub struct TimingsOptions {
    pub path: Option<PathBuf>,
    pub format: TimingsFormat,
    pub scenario: TimingsScenario,
    pub profile: Profile,
    pub provenance: Option<bool>,
}

#[derive(Clone, Debug, Serialize)]
pub struct TimingsSnapshot {
    pub timings_version: u32,
    pub profile: String,
    pub provenance: bool,
    pub roots: Vec<RootTiming>,
    pub workspace: WorkspaceTiming,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<SiteTiming>,
    pub pages: Vec<PageTiming>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click: Option<ClickTiming>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch: Option<WorkspaceTiming>,
}

#[derive(Clone, Debug, Serialize)]
pub struct RootTiming {
    pub id: String,
    pub path: String,
    pub concept_count: usize,
    pub log_count: usize,
    pub diagnostic_count: usize,
    pub timings: SpanTimings,
}

#[derive(Clone, Debug, Serialize)]
pub struct SpanTimings {
    pub discover: f64,
    pub parse: f64,
    pub graph: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance: Option<f64>,
    pub parse_cache_hits: u32,
    pub parse_cache_misses: u32,
}

#[derive(Clone, Debug, Serialize)]
pub struct WorkspaceTiming {
    pub load_members_ms: f64,
    pub member_count: usize,
    pub concept_total: usize,
    pub parse_cache_hits: u32,
    pub parse_cache_misses: u32,
}

#[derive(Clone, Debug, Serialize)]
pub struct SiteTiming {
    pub write_ms: f64,
    pub file_count: usize,
    pub byte_total: u64,
    pub largest_path: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct PageTiming {
    pub route: String,
    pub kind: String,
    pub page_for_route_ms: f64,
    pub fragment_render_ms: f64,
    pub fragment_bytes: usize,
    pub article_bytes: usize,
    pub review_row_count: usize,
    pub log_entry_count: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct ClickTiming {
    pub route: String,
    pub reload_ms: f64,
    pub render_ms: f64,
    pub fragment_bytes: usize,
}

pub fn run(options: TimingsOptions) -> Result<()> {
    let snapshot = measure(&options)?;
    match options.format {
        TimingsFormat::Json => println!("{}", serde_json::to_string_pretty(&snapshot)?),
        TimingsFormat::Terminal => print_terminal(&snapshot),
    }
    Ok(())
}

pub fn measure(options: &TimingsOptions) -> Result<TimingsSnapshot> {
    let load_options = load_options(options);
    let specs = resolve_specs(options.path.as_deref())?;
    let cache_parent = tempfile_dir("timings-parse")?;
    let mut roots = Vec::with_capacity(specs.len());
    for (id, path) in &specs {
        roots.push(measure_root(id, path, load_options, &cache_parent)?);
    }

    let started = Instant::now();
    let (workspace, workspace_cache) =
        load_members_timed(&specs, load_options, Some(&cache_parent))?;
    let workspace_timing = WorkspaceTiming {
        load_members_ms: millis(started.elapsed()),
        member_count: workspace.len(),
        concept_total: workspace
            .members()
            .iter()
            .map(|member| member.bundle.concepts.len())
            .sum(),
        parse_cache_hits: workspace_cache.0,
        parse_cache_misses: workspace_cache.1,
    };

    let include = options.scenario;
    let site = if matches!(include, TimingsScenario::Site | TimingsScenario::All) {
        Some(measure_site(&workspace)?)
    } else {
        None
    };

    let mut pages = Vec::new();
    if matches!(
        include,
        TimingsScenario::Click
            | TimingsScenario::Review
            | TimingsScenario::Log
            | TimingsScenario::All
    ) {
        if matches!(include, TimingsScenario::Click | TimingsScenario::All)
            && let Some(route) = first_leaf_route(&workspace)
        {
            pages.push(measure_page(&workspace, &route)?);
        }
        if matches!(include, TimingsScenario::Review | TimingsScenario::All) {
            pages.push(measure_page(&workspace, "/review/")?);
        }
        if matches!(include, TimingsScenario::Log | TimingsScenario::All) {
            pages.push(measure_page(&workspace, "/log/")?);
        }
    }

    let click = if matches!(include, TimingsScenario::Click | TimingsScenario::All) {
        Some(measure_click(&workspace, options.profile)?)
    } else {
        None
    };

    let watch = if matches!(include, TimingsScenario::Watch | TimingsScenario::All) {
        let started = Instant::now();
        let (reloaded, cache) = load_members_timed(&specs, load_options, Some(&cache_parent))?;
        Some(WorkspaceTiming {
            load_members_ms: millis(started.elapsed()),
            member_count: reloaded.len(),
            concept_total: reloaded
                .members()
                .iter()
                .map(|member| member.bundle.concepts.len())
                .sum(),
            parse_cache_hits: cache.0,
            parse_cache_misses: cache.1,
        })
    } else {
        None
    };

    Ok(TimingsSnapshot {
        timings_version: TIMINGS_VERSION,
        profile: profile_name(options.profile).into(),
        provenance: load_options.provenance,
        roots,
        workspace: workspace_timing,
        site,
        pages,
        click,
        watch,
    })
}

fn load_options(options: &TimingsOptions) -> LoadOptions {
    view_load_options(options.profile, options.provenance.unwrap_or(false))
}

fn resolve_specs(path: Option<&Path>) -> Result<Vec<(String, PathBuf)>> {
    if let Some(path) = path {
        let target = okf::resolve_preview_path(path)?;
        return Ok(vec![(id_from_path(&target.root), target.root)]);
    }
    let target = Workspace::for_view(
        None,
        view_load_options(Profile::Base, false),
        &crate::config::config_path(),
        &crate::config::cache_dir(),
        crate::preview::load_session().bundle.as_deref(),
    )?;
    Ok(target
        .workspace
        .members()
        .iter()
        .map(|member| (member.id.clone(), member.path.clone()))
        .collect())
}

fn measure_root(
    id: &str,
    path: &Path,
    options: LoadOptions,
    cache_parent: &Path,
) -> Result<RootTiming> {
    let dir = parse_cache_dir(cache_parent, id);
    let mut cache = ParseCache::load_dir(&dir, options.profile);
    let loaded = okf::load_with_cache(path, options, Some(&mut cache))
        .with_context(|| format!("failed to time knowledge root `{id}`"))?;
    cache.save_dir(&dir)?;
    Ok(RootTiming {
        id: id.to_string(),
        path: path.display().to_string(),
        concept_count: loaded.bundle.concepts.len(),
        log_count: loaded.bundle.logs.len(),
        diagnostic_count: loaded.bundle.diagnostics.len(),
        timings: SpanTimings::from(&loaded.timings),
    })
}

fn load_members_timed(
    specs: &[(String, PathBuf)],
    options: LoadOptions,
    cache_parent: Option<&Path>,
) -> Result<(Workspace, (u32, u32))> {
    let mut hits = 0;
    let mut misses = 0;
    let mut members = Vec::with_capacity(specs.len());
    for (id, path) in specs {
        if let Some(parent) = cache_parent {
            let dir = parse_cache_dir(parent, id);
            let mut cache = ParseCache::load_dir(&dir, options.profile);
            let loaded = okf::load_with_cache(path, options, Some(&mut cache))?;
            cache.save_dir(&dir)?;
            hits += loaded.timings.parse_cache_hits;
            misses += loaded.timings.parse_cache_misses;
            members.push(crate::workspace::WorkspaceMember {
                id: id.clone(),
                path: path.clone(),
                bundle: loaded.bundle,
            });
        } else {
            let loaded = okf::load_timed(path, options)?;
            members.push(crate::workspace::WorkspaceMember {
                id: id.clone(),
                path: path.clone(),
                bundle: loaded.bundle,
            });
        }
    }
    Ok((Workspace::from_members(members), (hits, misses)))
}

fn measure_site(workspace: &Workspace) -> Result<SiteTiming> {
    let output = tempfile_dir("timings-site")?;
    let started = Instant::now();
    site::build_workspace(workspace, &output)?;
    let write_ms = millis(started.elapsed());
    let (file_count, byte_total, largest_path) = walk_files(&output)?;
    let _ = fs::remove_dir_all(&output);
    Ok(SiteTiming {
        write_ms,
        file_count,
        byte_total,
        largest_path: if largest_path.is_empty() {
            "assets only".into()
        } else {
            largest_path
        },
    })
}

fn measure_page(workspace: &Workspace, route: &str) -> Result<PageTiming> {
    let started = Instant::now();
    let document = site::page_for_route(workspace, route)
        .with_context(|| format!("no page for route {route}"))?;
    let page_for_route_ms = millis(started.elapsed());
    let kind = document.page_kind.clone();
    let article_bytes = document.article_html.len();
    let review_row_count = document.review_rows.len();
    let log_entry_count = document.log_days.iter().map(|day| day.entries.len()).sum();
    let started = Instant::now();
    let fragment = document
        .render_main_fragment()
        .map_err(|error| anyhow::anyhow!("{error}"))?;
    Ok(PageTiming {
        route: route.to_string(),
        kind,
        page_for_route_ms,
        fragment_render_ms: millis(started.elapsed()),
        fragment_bytes: fragment.len(),
        article_bytes,
        review_row_count,
        log_entry_count,
    })
}

fn measure_click(workspace: &Workspace, _profile: Profile) -> Result<ClickTiming> {
    let route = first_leaf_route(workspace).context("workspace has no leaf concept")?;
    let started = Instant::now();
    let document =
        site::page_for_route(workspace, &route).with_context(|| format!("no page for {route}"))?;
    let fragment = document
        .render_main_fragment()
        .map_err(|error| anyhow::anyhow!("{error}"))?;
    Ok(ClickTiming {
        route,
        reload_ms: 0.0,
        render_ms: millis(started.elapsed()),
        fragment_bytes: fragment.len(),
    })
}

fn first_leaf_route(workspace: &Workspace) -> Option<String> {
    workspace.members().iter().find_map(|member| {
        member
            .bundle
            .concepts
            .first()
            .map(|concept| workspace.document_href(&member.id, &concept.id))
    })
}

impl From<&LoadTimings> for SpanTimings {
    fn from(value: &LoadTimings) -> Self {
        Self {
            discover: millis(value.discover),
            parse: millis(value.parse),
            graph: millis(value.graph),
            provenance: value.provenance.map(millis),
            parse_cache_hits: value.parse_cache_hits,
            parse_cache_misses: value.parse_cache_misses,
        }
    }
}

fn print_terminal(snapshot: &TimingsSnapshot) {
    println!(
        "okmate timings v{}  profile={}  provenance={}",
        snapshot.timings_version, snapshot.profile, snapshot.provenance
    );
    for root in &snapshot.roots {
        println!(
            "root {}  parse={:.3}ms  concepts={}  cache={}/{}",
            root.id,
            root.timings.parse,
            root.concept_count,
            root.timings.parse_cache_hits,
            root.timings.parse_cache_misses
        );
    }
    println!(
        "workspace  load_members={:.3}ms  members={}  concepts={}",
        snapshot.workspace.load_members_ms,
        snapshot.workspace.member_count,
        snapshot.workspace.concept_total
    );
    if let Some(site) = &snapshot.site {
        println!(
            "site  write={:.3}ms  files={}  bytes={}  largest={}",
            site.write_ms, site.file_count, site.byte_total, site.largest_path
        );
    }
    for page in &snapshot.pages {
        println!(
            "page {}  kind={}  page={:.3}ms  fragment={:.3}ms  bytes={}",
            page.route,
            page.kind,
            page.page_for_route_ms,
            page.fragment_render_ms,
            page.fragment_bytes
        );
    }
    if let Some(click) = &snapshot.click {
        println!(
            "click {}  reload={:.3}ms  render={:.3}ms  bytes={}",
            click.route, click.reload_ms, click.render_ms, click.fragment_bytes
        );
    }
    if let Some(watch) = &snapshot.watch {
        println!(
            "watch  load_members={:.3}ms  members={}",
            watch.load_members_ms, watch.member_count
        );
    }
}

fn walk_files(root: &Path) -> Result<(usize, u64, String)> {
    let mut file_count = 0;
    let mut byte_total = 0;
    let mut largest_bytes = 0u64;
    let mut largest_path = String::new();
    fn walk(
        dir: &Path,
        root: &Path,
        file_count: &mut usize,
        byte_total: &mut u64,
        largest_bytes: &mut u64,
        largest_path: &mut String,
    ) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk(
                    &path,
                    root,
                    file_count,
                    byte_total,
                    largest_bytes,
                    largest_path,
                )?;
            } else {
                let len = entry.metadata()?.len();
                *file_count += 1;
                *byte_total += len;
                if len >= *largest_bytes {
                    *largest_bytes = len;
                    *largest_path = path
                        .strip_prefix(root)
                        .unwrap_or(&path)
                        .display()
                        .to_string();
                }
            }
        }
        Ok(())
    }
    walk(
        root,
        root,
        &mut file_count,
        &mut byte_total,
        &mut largest_bytes,
        &mut largest_path,
    )?;
    Ok((file_count, byte_total, largest_path))
}

fn tempfile_dir(name: &str) -> Result<PathBuf> {
    let path = std::env::temp_dir().join(format!(
        "okmate-{name}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));
    fs::create_dir_all(&path)?;
    Ok(path)
}

fn profile_name(profile: Profile) -> &'static str {
    match profile {
        Profile::Base => "base",
        Profile::Strict => "strict",
    }
}

fn millis(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}

pub fn server_timing_header(reload_ms: f64, render_ms: f64, bytes: usize) -> String {
    format!("reload;dur={reload_ms:.3}, render;dur={render_ms:.3}, bytes;dur={bytes}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_timing_lists_reload_render_bytes() {
        let header = server_timing_header(12.5, 1.25, 400);
        assert!(header.contains("reload;dur=12.500"), "{header}");
        assert!(header.contains("render;dur=1.250"), "{header}");
        assert!(header.contains("bytes;dur=400"), "{header}");
    }
}
