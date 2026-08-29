use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use okf::{BuildSummary, Bundle, Profile};
use serde::Serialize;

use crate::html_util::{strip_leading_h1, wrap_article_tables};
use crate::nav::{canonical_collection_href, collection_owners, collection_title, nav_tree};
use crate::preview::NavMode;
use crate::views::{
    Crumb, DASHBOARD_LOG_LIMIT, Document, TocEntry, action_rows, concept_meta_with_graph,
    diagnostic_rows, governance_stats, merged_log, recent_leaf_documents, review_rows,
    take_log_entries, toc_from_headings,
};
use crate::workspace::{Workspace, WorkspaceMember, id_from_path, normalize_route};

const APP_CSS: &str = include_str!("../assets/app.css");
const DATASTAR_JS: &str = include_str!("../assets/datastar.js");
const GOTO_JS: &str = include_str!("../assets/goto.js");
const NAV_JS: &str = include_str!("../assets/nav.js");
const RESIZE_JS: &str = include_str!("../assets/resize.js");
const READING_JS: &str = include_str!("../assets/reading.js");
const TOC_JS: &str = include_str!("../assets/toc.js");
const REVIEW_JS: &str = include_str!("../assets/review.js");
const LOG_JS: &str = include_str!("../assets/log.js");
const TABLES_JS: &str = include_str!("../assets/tables.js");
const META_JS: &str = include_str!("../assets/meta.js");

#[derive(Serialize)]
struct NavPage {
    title: String,
    route: String,
    path: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    description: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    collection: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    root: String,
}

pub fn build(root: &Path, output: &Path, profile: Profile) -> Result<BuildSummary> {
    let bundle = okf::load(root, profile)?;
    if bundle.has_errors() {
        bail!("knowledge bundle has validation errors");
    }
    let summary = okf::build_artifacts(&bundle, output)?;
    let workspace = Workspace::from_loaded(id_from_path(root), root.to_path_buf(), bundle);
    write_html_pages(&workspace, output, NavMode::Separated)?;
    write_preview_shell(&workspace, output)?;
    Ok(summary)
}

pub fn build_workspace(workspace: &Workspace, output: &Path) -> Result<()> {
    build_workspace_nav(workspace, output)
}

pub fn build_workspace_nav(workspace: &Workspace, output: &Path) -> Result<()> {
    write_preview_shell(workspace, output)
}

fn write_preview_shell(workspace: &Workspace, output: &Path) -> Result<()> {
    write_pages_json(workspace, output)?;
    write_assets(output)?;
    Ok(())
}

pub fn write_html_pages(workspace: &Workspace, output: &Path, nav_mode: NavMode) -> Result<()> {
    let mut routes = vec![
        "/".to_string(),
        "/review/".into(),
        "/log/".into(),
        "/settings/".into(),
    ];
    for member in workspace.members() {
        for concept in &member.bundle.concepts {
            routes.push(workspace.document_href(&member.id, &concept.id));
        }
        for index in &member.bundle.indexes {
            let Some(collection) = index.path.strip_suffix("/index.md") else {
                continue;
            };
            routes.push(workspace.collection_href(&member.id, collection));
        }
    }
    routes.sort();
    routes.dedup();
    for route in routes {
        let Some(page) = page_for_route_nav(workspace, &route, nav_mode) else {
            continue;
        };
        write_route(output, &route, render_document(page)?)?;
    }
    Ok(())
}

pub fn page_for_route(workspace: &Workspace, route: &str) -> Option<Document> {
    page_for_route_nav(workspace, route, NavMode::Separated)
}

pub fn page_for_route_nav(
    workspace: &Workspace,
    route: &str,
    nav_mode: NavMode,
) -> Option<Document> {
    let route = normalize_route(route);
    match route.as_str() {
        "/" => Some(
            document(workspace, "/", "Knowledge", Vec::new(), nav_mode)
                .with_kind(crate::views::PageKind::Home)
                .with_home(workspace),
        ),
        "/review/" => Some(
            document(
                workspace,
                "/review/",
                "Knowledge Governance & Review Queue",
                Vec::new(),
                nav_mode,
            )
            .with_kind(crate::views::PageKind::Review)
            .with_review(workspace),
        ),
        "/log/" => Some(
            document(workspace, "/log/", "Log", Vec::new(), nav_mode)
                .with_kind(crate::views::PageKind::Log)
                .with_log(workspace),
        ),
        "/settings/" => Some(settings_document(workspace, nav_mode)),
        other => {
            let (member, id) = workspace.parse_document_route(other)?;
            let bundle = &member.bundle;
            if let Some(concept) = bundle.concepts.iter().find(|concept| concept.id == id) {
                let title = okf::string_field(&concept.metadata, "title").unwrap_or(&concept.id);
                Some(
                    document(
                        workspace,
                        other,
                        title,
                        toc_from_headings(&concept.headings),
                        nav_mode,
                    )
                    .with_kind(crate::views::PageKind::Page)
                    .with_article(&workspace.rewrite_article(&member.id, &concept.article_html))
                    .with_meta(workspace, member, concept),
                )
            } else if let Some(index) = bundle
                .indexes
                .iter()
                .find(|index| index.path.strip_suffix("/index.md") == Some(id.as_str()))
            {
                if nav_mode == NavMode::Merged
                    && workspace.is_multi()
                    && collection_owners(workspace, &id).len() > 1
                {
                    Some(merged_collection_document(workspace, other, &id, nav_mode))
                } else {
                    Some(
                        document(
                            workspace,
                            other,
                            &collection_title(index),
                            toc_from_headings(&index.headings),
                            nav_mode,
                        )
                        .with_kind(crate::views::PageKind::Page)
                        .with_article(&workspace.rewrite_article(&member.id, &index.article_html)),
                    )
                }
            } else {
                None
            }
        }
    }
}

pub(crate) fn render_document(document: Document) -> Result<String> {
    match document.page_kind {
        crate::views::PageKind::Home => document.render_home(),
        crate::views::PageKind::Log => document.render_log(),
        crate::views::PageKind::Review => document.render_review(),
        crate::views::PageKind::Settings => document.render_settings(),
        crate::views::PageKind::Page => document.render_page(),
    }
    .map_err(|error| anyhow::anyhow!(error))
}

fn document(
    workspace: &Workspace,
    route: &str,
    title: &str,
    toc: Vec<crate::views::TocEntry>,
    nav_mode: NavMode,
) -> Document {
    Document {
        title: title.to_string(),
        page_kind: crate::views::PageKind::Page,
        nav: nav_tree(workspace, route, nav_mode),
        toc,
        article_html: String::new(),
        concept_type: String::new(),
        status: String::new(),
        authority: String::new(),
        review_rows: Vec::new(),
        action_rows: Vec::new(),
        stats: Vec::new(),
        recents: Vec::new(),
        log_days: Vec::new(),
        show_root: false,
        nav_mode: nav_mode.as_str().into(),
        show_nav_mode: workspace.is_multi(),
        crumbs: breadcrumbs(workspace, route, title, nav_mode),
        diagnostics: Vec::new(),
        meta: crate::views::ConceptMeta::default(),
        message: String::new(),
        config_path: String::new(),
        settings_roots: Vec::new(),
        review_window: crate::views::ListWindow::default(),
        log_window: crate::views::ListWindow::default(),
        html_style: String::new(),
        reading_wrap: true,
        reading_full: false,
        reading_nav: true,
        reading_toc: true,
        reading_font: 100,
        reading_width: 66,
        main_scroll: 0,
        nav_scroll: 0,
    }
}

pub fn apply_reading_prefs(document: &mut Document, session: &crate::preview::Session) {
    document.html_style = session.html_style();
    document.reading_wrap = session.wrap;
    document.reading_full = session.full_width;
    document.reading_nav = session.nav_visible;
    document.reading_toc = session.toc_visible;
    document.reading_font = session.font_size;
    document.reading_width = session.reading_width();
    document.nav_scroll = session.nav_scroll.unwrap_or(0);
    apply_nav_sections(&mut document.nav, &session.nav_sections);
}

fn apply_nav_sections(
    nodes: &mut [crate::views::NavNode],
    saved: &std::collections::BTreeMap<String, bool>,
) {
    if saved.is_empty() {
        return;
    }
    for node in nodes {
        if !node.section_key.is_empty()
            && let Some(open) = saved.get(&node.section_key)
        {
            node.open = *open;
        }
        apply_nav_sections(&mut node.children, saved);
    }
}

fn breadcrumbs(workspace: &Workspace, route: &str, title: &str, nav_mode: NavMode) -> Vec<Crumb> {
    let route = normalize_route(route);
    if Workspace::chrome_route(&route) {
        return Vec::new();
    }
    let mut crumbs = Vec::new();
    let Some((member, id)) = workspace.parse_document_route(&route) else {
        return crumbs;
    };
    if workspace.is_multi() && nav_mode == NavMode::Separated {
        crumbs.push(Crumb {
            href: String::new(),
            title: format!("@{}", member.id),
            current: false,
        });
    }
    let parts: Vec<&str> = id.split('/').filter(|part| !part.is_empty()).collect();
    let mut acc = String::new();
    for (index, segment) in parts.iter().enumerate() {
        if !acc.is_empty() {
            acc.push('/');
        }
        acc.push_str(segment);
        let href = if workspace.is_multi()
            && nav_mode == NavMode::Merged
            && !collection_owners(workspace, &acc).is_empty()
        {
            canonical_collection_href(workspace, &acc)
        } else {
            workspace.document_href(&member.id, &acc)
        };
        let last = index + 1 == parts.len();
        let crumb_title = if last {
            title.to_string()
        } else {
            ancestor_title(&member.bundle, &acc, segment)
        };
        crumbs.push(Crumb {
            href,
            title: crumb_title,
            current: last,
        });
    }
    crumbs
}

fn ancestor_title(bundle: &Bundle, path: &str, segment: &str) -> String {
    if let Some(index) = bundle
        .indexes
        .iter()
        .find(|index| index.path.strip_suffix("/index.md") == Some(path))
    {
        return collection_title(index);
    }
    if let Some(concept) = bundle.concepts.iter().find(|concept| concept.id == path) {
        return okf::string_field(&concept.metadata, "title")
            .unwrap_or(&concept.id)
            .to_string();
    }
    segment.to_string()
}

fn settings_document(workspace: &Workspace, nav_mode: NavMode) -> Document {
    let config = crate::config::load().unwrap_or_default();
    let mut document = document(
        workspace,
        "/settings/",
        "Knowledge roots",
        Vec::new(),
        nav_mode,
    );
    document.config_path = crate::config::config_path().display().to_string();
    document.settings_roots = crate::http::settings_roots(&config);
    document.with_kind(crate::views::PageKind::Settings)
}

impl Document {
    fn with_kind(mut self, kind: crate::views::PageKind) -> Self {
        self.page_kind = kind;
        self
    }

    fn with_article(mut self, html: &str) -> Self {
        self.article_html = wrap_article_tables(html);
        self
    }

    fn with_meta(
        mut self,
        workspace: &Workspace,
        member: &WorkspaceMember,
        concept: &okf::Concept,
    ) -> Self {
        self.concept_type = okf::string_field(&concept.metadata, "type")
            .unwrap_or("Concept")
            .to_string();
        self.status = okf::string_field(&concept.metadata, "status")
            .unwrap_or("draft")
            .to_string();
        self.authority = okf::string_field(&concept.metadata, "authority")
            .unwrap_or("descriptive")
            .to_string();
        self.meta = concept_meta_with_graph(
            concept,
            &member.bundle.diagnostics,
            Some((workspace, member)),
        );
        self
    }

    fn with_review(mut self, workspace: &Workspace) -> Self {
        self.review_rows = review_rows(workspace);
        self.action_rows = action_rows(&self.review_rows);
        self.stats = governance_stats(workspace);
        self.diagnostics = diagnostic_rows(workspace);
        self.show_root = workspace.is_multi();
        self
    }

    fn with_home(mut self, workspace: &Workspace) -> Self {
        self.recents = recent_leaf_documents(workspace, 10);
        self.stats = governance_stats(workspace);
        self.log_days = take_log_entries(&merged_log(workspace), DASHBOARD_LOG_LIMIT);
        self.show_root = workspace.is_multi();
        self
    }

    fn with_log(mut self, workspace: &Workspace) -> Self {
        self.log_days = merged_log(workspace);
        self.show_root = workspace.is_multi();
        self
    }
}

fn write_route(output: &Path, route: &str, html: String) -> Result<()> {
    let path = route_to_path(output, route);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    fs::write(&path, html).with_context(|| format!("failed to write {}", path.display()))
}

fn route_to_path(output: &Path, route: &str) -> PathBuf {
    if route == "/" {
        output.join("index.html")
    } else {
        output.join(route.trim_matches('/')).join("index.html")
    }
}

pub(crate) fn write_settings_host(output: &Path) -> Result<()> {
    write_assets(output)?;
    let config_path = crate::config::config_path();
    let config = crate::config::load_or_default(&config_path);
    let html = crate::http::render_page(
        &Workspace::empty(),
        &config,
        None,
        &config_path,
        &crate::preview::load_session(),
    );
    write_route(output, "/settings/", html.clone())?;
    write_route(output, "/", html)
}

fn write_pages_json(workspace: &Workspace, output: &Path) -> Result<()> {
    fs::create_dir_all(output).with_context(|| format!("failed to create {}", output.display()))?;
    fs::write(
        output.join("pages.json"),
        format!("{}\n", serde_json::to_string_pretty(&nav_pages(workspace))?),
    )
    .context("failed to write pages.json")
}

fn write_assets(output: &Path) -> Result<()> {
    let dir = output.join("__okmate");
    fs::create_dir_all(&dir).with_context(|| format!("failed to create {}", dir.display()))?;
    fs::write(dir.join("app.css"), APP_CSS).context("failed to write app.css")?;
    fs::write(dir.join("datastar.js"), DATASTAR_JS).context("failed to write datastar.js")?;
    fs::write(dir.join("goto.js"), GOTO_JS).context("failed to write goto.js")?;
    fs::write(dir.join("nav.js"), NAV_JS).context("failed to write nav.js")?;
    fs::write(dir.join("resize.js"), RESIZE_JS).context("failed to write resize.js")?;
    fs::write(dir.join("reading.js"), READING_JS).context("failed to write reading.js")?;
    fs::write(dir.join("toc.js"), TOC_JS).context("failed to write toc.js")?;
    fs::write(dir.join("review.js"), REVIEW_JS).context("failed to write review.js")?;
    fs::write(dir.join("log.js"), LOG_JS).context("failed to write log.js")?;
    fs::write(dir.join("tables.js"), TABLES_JS).context("failed to write tables.js")?;
    fs::write(dir.join("meta.js"), META_JS).context("failed to write meta.js")
}

fn nav_pages(workspace: &Workspace) -> Vec<NavPage> {
    let mut pages = vec![
        NavPage {
            title: "Dashboard".into(),
            route: "/".into(),
            path: "index.md".into(),
            description: String::new(),
            collection: String::new(),
            root: String::new(),
        },
        NavPage {
            title: "Review queue".into(),
            route: "/review/".into(),
            path: "review".into(),
            description: String::new(),
            collection: String::new(),
            root: String::new(),
        },
        NavPage {
            title: "Log".into(),
            route: "/log/".into(),
            path: "log".into(),
            description: String::new(),
            collection: String::new(),
            root: String::new(),
        },
        NavPage {
            title: "Settings".into(),
            route: "/settings/".into(),
            path: "settings".into(),
            description: String::new(),
            collection: String::new(),
            root: String::new(),
        },
    ];
    let root_label = |id: &str| {
        if workspace.is_multi() {
            id.to_string()
        } else {
            String::new()
        }
    };
    for member in workspace.members() {
        let root = root_label(&member.id);
        for concept in &member.bundle.concepts {
            let id = concept.id.trim_matches('/');
            pages.push(NavPage {
                title: okf::string_field(&concept.metadata, "title")
                    .unwrap_or(&concept.id)
                    .to_string(),
                route: workspace.document_href(&member.id, id),
                path: concept.path.clone(),
                description: okf::string_field(&concept.metadata, "description")
                    .unwrap_or("")
                    .to_string(),
                collection: collection_label(&concept.path),
                root: root.clone(),
            });
        }
        for index in &member.bundle.indexes {
            let Some(collection) = index.path.strip_suffix("/index.md") else {
                continue;
            };
            pages.push(NavPage {
                title: collection_title(index),
                route: workspace.collection_href(&member.id, collection),
                path: index.path.clone(),
                description: String::new(),
                collection: collection.to_string(),
                root: root.clone(),
            });
        }
    }
    pages.sort_by(|left, right| {
        left.route
            .cmp(&right.route)
            .then(left.path.cmp(&right.path))
            .then(left.root.cmp(&right.root))
    });
    pages
}

fn collection_label(path: &str) -> String {
    path.split('/')
        .next()
        .filter(|segment| !segment.is_empty() && *segment != path && *segment != "review")
        .unwrap_or("")
        .to_string()
}

fn merge_collection_indexes(
    workspace: &Workspace,
    path: &str,
    owners: &[String],
) -> (String, Vec<TocEntry>, String) {
    let mut title = String::new();
    let mut toc = Vec::new();
    let mut html = String::new();
    let headed = owners.len() > 1;
    for root_id in owners {
        let Some(member) = workspace
            .members()
            .iter()
            .find(|member| member.id == *root_id)
        else {
            continue;
        };
        let Some(index) = member
            .bundle
            .indexes
            .iter()
            .find(|index| index.path.strip_suffix("/index.md") == Some(path))
        else {
            continue;
        };
        if title.is_empty() {
            title = collection_title(index);
        }
        let source = if headed {
            strip_leading_h1(&index.article_html)
        } else {
            index.article_html.clone()
        };
        if headed {
            let hid = format!("bundle-{root_id}");
            html.push_str(&format!(
                "<h2 id=\"{hid}\" class=\"okmate-merged-root\">@{root_id}</h2>"
            ));
            toc.push(TocEntry {
                id: hid,
                text: format!("@{root_id}"),
                level: 2,
            });
        }
        html.push_str(&workspace.rewrite_merged_article(root_id, &source));
        toc.extend(toc_from_headings(&index.headings));
    }
    if title.is_empty() {
        title = path.rsplit('/').next().unwrap_or(path).to_string();
    }
    (title, toc, html)
}

fn merged_collection_document(
    workspace: &Workspace,
    route: &str,
    path: &str,
    nav_mode: NavMode,
) -> Document {
    let owners = collection_owners(workspace, path);
    let (title, toc, html) = merge_collection_indexes(workspace, path, &owners);
    document(workspace, route, &title, toc, nav_mode)
        .with_kind(crate::views::PageKind::Page)
        .with_article(&html)
}
