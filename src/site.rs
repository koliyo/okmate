use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use okf::{BuildSummary, Bundle, Profile};
use serde::Serialize;

use crate::preview::NavMode;
use crate::views::{
    Crumb, Document, NavNode, action_rows, compact_type_label, concept_meta, diagnostic_rows,
    governance_stats, merged_log, recent_leaf_documents, review_rows, toc_from_headings,
};
use crate::workspace::{Workspace, WorkspaceMember, id_from_path, normalize_route};

const APP_CSS: &str = include_str!("../assets/app.css");
const DATASTAR_JS: &str = include_str!("../assets/datastar.js");
const GOTO_JS: &str = include_str!("../assets/goto.js");
const NAV_JS: &str = include_str!("../assets/nav.js");
const RESIZE_JS: &str = include_str!("../assets/resize.js");
const TOC_JS: &str = include_str!("../assets/toc.js");
const REVIEW_JS: &str = include_str!("../assets/review.js");

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
    let summary = okf::build(root, output, profile)?;
    let bundle = okf::load(root, profile)?;
    let workspace = Workspace::from_loaded(id_from_path(root), root.to_path_buf(), bundle);
    write_site(&workspace, output, NavMode::Separated)?;
    Ok(summary)
}

pub fn build_workspace(workspace: &Workspace, output: &Path) -> Result<()> {
    build_workspace_nav(workspace, output, NavMode::Separated)
}

pub fn build_workspace_nav(workspace: &Workspace, output: &Path, nav_mode: NavMode) -> Result<()> {
    write_site(workspace, output, nav_mode)
}

fn write_site(workspace: &Workspace, output: &Path, nav_mode: NavMode) -> Result<()> {
    write_html_pages(workspace, output, nav_mode)?;
    write_pages_json(workspace, output)?;
    write_assets(output)?;
    Ok(())
}

pub fn write_html_pages(workspace: &Workspace, output: &Path, nav_mode: NavMode) -> Result<()> {
    let mut routes = vec!["/".to_string(), "/review/".into(), "/settings/".into()];
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
                .with_kind("home")
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
            .with_kind("review")
            .with_review(workspace),
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
                    .with_kind("page")
                    .with_article(&workspace.rewrite_article(&member.id, &concept.article_html))
                    .with_meta(concept, &bundle.diagnostics),
                )
            } else {
                bundle
                    .indexes
                    .iter()
                    .find(|index| index.path.strip_suffix("/index.md") == Some(id.as_str()))
                    .map(|index| {
                        document(
                            workspace,
                            other,
                            &collection_title(index),
                            toc_from_headings(&index.headings),
                            nav_mode,
                        )
                        .with_kind("page")
                        .with_article(&workspace.rewrite_article(&member.id, &index.article_html))
                    })
            }
        }
    }
}

fn render_document(document: Document) -> Result<String> {
    match document.page_kind.as_str() {
        "home" => document.render_home(),
        "review" => document.render_review(),
        "settings" => document.render_settings(),
        _ => document.render_page(),
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
        page_kind: "page".into(),
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
        crumbs: breadcrumbs(workspace, route, title),
        diagnostics: Vec::new(),
        meta: crate::views::ConceptMeta::default(),
        message: String::new(),
        config_path: String::new(),
        settings_roots: Vec::new(),
    }
}

fn breadcrumbs(workspace: &Workspace, route: &str, title: &str) -> Vec<Crumb> {
    let route = normalize_route(route);
    if Workspace::chrome_route(&route) {
        return Vec::new();
    }
    let mut crumbs = vec![Crumb {
        href: "/".into(),
        title: "Dashboard".into(),
        current: false,
    }];
    let Some((member, id)) = workspace.parse_document_route(&route) else {
        return crumbs;
    };
    let parts: Vec<&str> = id.split('/').filter(|part| !part.is_empty()).collect();
    let mut acc = String::new();
    for (index, segment) in parts.iter().enumerate() {
        if !acc.is_empty() {
            acc.push('/');
        }
        acc.push_str(segment);
        let href = workspace.document_href(&member.id, &acc);
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

pub(crate) fn settings_shell(bundle: &Bundle) -> crate::views::Document {
    let workspace = Workspace::from_loaded("bundle", PathBuf::new(), bundle.clone());
    settings_document(&workspace, NavMode::Separated)
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
    document.with_kind("settings")
}

impl Document {
    fn with_kind(mut self, kind: &str) -> Self {
        self.page_kind = kind.to_string();
        self
    }

    fn with_article(mut self, html: &str) -> Self {
        self.article_html = html.to_string();
        self
    }

    fn with_meta(mut self, concept: &okf::Concept, diagnostics: &[okf::Diagnostic]) -> Self {
        self.concept_type =
            compact_type_label(okf::string_field(&concept.metadata, "type").unwrap_or("Concept"))
                .to_string();
        self.status = okf::string_field(&concept.metadata, "status")
            .unwrap_or("draft")
            .to_string();
        self.authority = okf::string_field(&concept.metadata, "authority")
            .unwrap_or("descriptive")
            .to_string();
        self.meta = concept_meta(concept, diagnostics);
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
    let html = crate::http::render_page(None, &config, None, &config_path);
    write_route(output, "/settings/", html.clone())?;
    write_route(output, "/", html)
}

fn write_pages_json(workspace: &Workspace, output: &Path) -> Result<()> {
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
    fs::write(dir.join("toc.js"), TOC_JS).context("failed to write toc.js")?;
    fs::write(dir.join("review.js"), REVIEW_JS).context("failed to write review.js")
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

fn collection_title(index: &okf::Index) -> String {
    if let Some(heading) = index.headings.iter().find(|heading| heading.level == 1) {
        return heading.text.clone();
    }
    index
        .path
        .strip_suffix("/index.md")
        .and_then(|collection| collection.rsplit('/').next())
        .unwrap_or(index.path.as_str())
        .to_string()
}

fn nav_tree(workspace: &Workspace, current: &str, nav_mode: NavMode) -> Vec<NavNode> {
    let current = normalize_route(current);
    let mut items = vec![
        leaf("/", "Dashboard", &current),
        leaf("/review/", "Review queue", &current),
        leaf("/settings/", "Settings", &current),
    ];
    if workspace.is_multi() {
        match nav_mode {
            NavMode::Merged => items.extend(nav_forest_merged(workspace, &current)),
            NavMode::Separated => {
                for member in workspace.members() {
                    let prefix = format!("/@{}/", member.id);
                    let active = current.starts_with(&prefix);
                    items.push(NavNode {
                        href: String::new(),
                        title: member.id.clone(),
                        current: active,
                        open: active,
                        children: nav_forest(workspace, member, &current, &member.id),
                        section_key: member.id.clone(),
                        root: member.id.clone(),
                        summary: String::new(),
                    });
                }
            }
        }
    } else if let Some(member) = workspace.primary() {
        items.extend(nav_forest(workspace, member, &current, ""));
    }
    items
}

fn leaf(href: &str, title: &str, current: &str) -> NavNode {
    NavNode {
        href: href.into(),
        title: title.into(),
        current: href == current,
        open: false,
        children: Vec::new(),
        section_key: String::new(),
        root: String::new(),
        summary: String::new(),
    }
}

fn namespaced_key(prefix: &str, path: &str) -> String {
    if prefix.is_empty() {
        path.to_string()
    } else {
        format!("{prefix}/{path}")
    }
}

fn nav_forest(
    workspace: &Workspace,
    member: &WorkspaceMember,
    current: &str,
    section_prefix: &str,
) -> Vec<NavNode> {
    let bundle = &member.bundle;
    let root_id = member.id.as_str();
    let mut by_path: BTreeMap<String, NavNode> = BTreeMap::new();
    for index in &bundle.indexes {
        let Some(path) = index.path.strip_suffix("/index.md") else {
            continue;
        };
        let href = workspace.collection_href(root_id, path);
        by_path.insert(
            path.to_string(),
            NavNode {
                href: href.clone(),
                title: collection_title(index),
                current: current == href || current.starts_with(&href),
                open: current == href || current.starts_with(&href),
                children: Vec::new(),
                section_key: namespaced_key(section_prefix, path),
                root: String::new(),
                summary: first_prose_paragraph(&index.article_html),
            },
        );
    }
    let paths: Vec<String> = by_path.keys().cloned().collect();
    for concept in &bundle.concepts {
        if by_path.contains_key(&concept.id) {
            continue;
        }
        let Some(owner) = paths
            .iter()
            .filter(|name| concept.id == **name || concept.id.starts_with(&format!("{name}/")))
            .max_by_key(|name| name.len())
            .cloned()
        else {
            continue;
        };
        let title = okf::string_field(&concept.metadata, "title").unwrap_or(&concept.id);
        if let Some(node) = by_path.get_mut(&owner) {
            let href = workspace.document_href(root_id, &concept.id);
            node.children.push(NavNode {
                href: href.clone(),
                title: title.to_string(),
                current: href == current,
                open: false,
                children: Vec::new(),
                section_key: String::new(),
                root: String::new(),
                summary: String::new(),
            });
        }
    }
    for node in by_path.values_mut() {
        node.children.sort_by(|left, right| {
            left.title
                .cmp(&right.title)
                .then(left.href.cmp(&right.href))
        });
    }

    let mut children_of: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut roots = Vec::new();
    for path in &paths {
        let parent = paths
            .iter()
            .filter(|candidate| path.starts_with(&format!("{candidate}/")))
            .max_by_key(|candidate| candidate.len());
        if let Some(parent) = parent {
            children_of
                .entry(parent.clone())
                .or_default()
                .push(path.clone());
        } else {
            roots.push(path.clone());
        }
    }

    fn take_node(
        path: &str,
        current: &str,
        workspace: &Workspace,
        root_id: &str,
        section_prefix: &str,
        by_path: &mut BTreeMap<String, NavNode>,
        children_of: &BTreeMap<String, Vec<String>>,
    ) -> NavNode {
        let mut node = by_path.remove(path).expect("nav node");
        if let Some(child_paths) = children_of.get(path) {
            for child in child_paths {
                node.children.push(take_node(
                    child,
                    current,
                    workspace,
                    root_id,
                    section_prefix,
                    by_path,
                    children_of,
                ));
            }
        }
        finalize_collection(workspace, root_id, section_prefix, path, node, current)
    }

    roots.sort();
    roots
        .into_iter()
        .map(|path| {
            take_node(
                &path,
                current,
                workspace,
                root_id,
                section_prefix,
                &mut by_path,
                &children_of,
            )
        })
        .collect()
}

fn finalize_collection(
    workspace: &Workspace,
    root_id: &str,
    section_prefix: &str,
    path: &str,
    mut node: NavNode,
    current: &str,
) -> NavNode {
    let href = workspace.collection_href(root_id, path);
    let mut nested = Vec::new();
    let mut leaves = Vec::new();
    for child in node.children.drain(..) {
        if child.section_key.is_empty() {
            leaves.push(child);
        } else {
            nested.push(child);
        }
    }
    nested.sort_by(|left, right| {
        left.title
            .cmp(&right.title)
            .then(left.href.cmp(&right.href))
    });
    leaves.sort_by(|left, right| {
        left.title
            .cmp(&right.title)
            .then(left.href.cmp(&right.href))
    });
    let mut children = vec![leaf(&href, "Overview", current)];
    children.extend(nested);
    children.extend(leaves);
    node.children = children;
    node.section_key = namespaced_key(section_prefix, path);
    node.href = href.clone();
    node.current = current == href || current.starts_with(&href);
    node.open = node.current;
    node
}

fn collection_is_current(workspace: &Workspace, path: &str, current: &str) -> bool {
    workspace.members().iter().any(|member| {
        let href = workspace.collection_href(&member.id, path);
        current == href || current.starts_with(&href)
    })
}

fn nav_forest_merged(workspace: &Workspace, current: &str) -> Vec<NavNode> {
    let mut by_path: BTreeMap<String, NavNode> = BTreeMap::new();
    let mut owners: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for member in workspace.members() {
        for index in &member.bundle.indexes {
            let Some(path) = index.path.strip_suffix("/index.md") else {
                continue;
            };
            owners
                .entry(path.to_string())
                .or_default()
                .push(member.id.clone());
            let active = collection_is_current(workspace, path, current);
            by_path.entry(path.to_string()).or_insert_with(|| NavNode {
                href: String::new(),
                title: collection_title(index),
                current: active,
                open: active,
                children: Vec::new(),
                section_key: path.to_string(),
                root: String::new(),
                summary: String::new(),
            });
        }
    }
    let paths: Vec<String> = by_path.keys().cloned().collect();
    for member in workspace.members() {
        for concept in &member.bundle.concepts {
            if by_path.contains_key(&concept.id) {
                continue;
            }
            let Some(owner) = paths
                .iter()
                .filter(|name| concept.id == **name || concept.id.starts_with(&format!("{name}/")))
                .max_by_key(|name| name.len())
                .cloned()
            else {
                continue;
            };
            let title = okf::string_field(&concept.metadata, "title").unwrap_or(&concept.id);
            if let Some(node) = by_path.get_mut(&owner) {
                let href = workspace.document_href(&member.id, &concept.id);
                node.children.push(NavNode {
                    href: href.clone(),
                    title: title.to_string(),
                    current: href == current,
                    open: false,
                    children: Vec::new(),
                    section_key: String::new(),
                    root: member.id.clone(),
                    summary: String::new(),
                });
            }
        }
    }
    for node in by_path.values_mut() {
        node.children.sort_by(|left, right| {
            left.title
                .cmp(&right.title)
                .then(left.href.cmp(&right.href))
        });
    }

    let mut children_of: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut roots = Vec::new();
    for path in &paths {
        let parent = paths
            .iter()
            .filter(|candidate| path.starts_with(&format!("{candidate}/")))
            .max_by_key(|candidate| candidate.len());
        if let Some(parent) = parent {
            children_of
                .entry(parent.clone())
                .or_default()
                .push(path.clone());
        } else {
            roots.push(path.clone());
        }
    }

    fn take_merged(
        path: &str,
        current: &str,
        workspace: &Workspace,
        by_path: &mut BTreeMap<String, NavNode>,
        children_of: &BTreeMap<String, Vec<String>>,
        owners: &BTreeMap<String, Vec<String>>,
    ) -> NavNode {
        let mut node = by_path.remove(path).expect("nav node");
        if let Some(child_paths) = children_of.get(path) {
            for child in child_paths {
                node.children.push(take_merged(
                    child,
                    current,
                    workspace,
                    by_path,
                    children_of,
                    owners,
                ));
            }
        }
        let empty = Vec::new();
        let path_owners = owners.get(path).unwrap_or(&empty);
        finalize_merged_collection(workspace, path, path_owners, node, current)
    }

    roots.sort();
    roots
        .into_iter()
        .map(|path| {
            take_merged(
                &path,
                current,
                workspace,
                &mut by_path,
                &children_of,
                &owners,
            )
        })
        .collect()
}

fn finalize_merged_collection(
    workspace: &Workspace,
    path: &str,
    owners: &[String],
    mut node: NavNode,
    current: &str,
) -> NavNode {
    let mut nested = Vec::new();
    let mut leaves = Vec::new();
    for child in node.children.drain(..) {
        if child.section_key.is_empty() {
            leaves.push(child);
        } else {
            nested.push(child);
        }
    }
    nested.sort_by(|left, right| {
        left.title
            .cmp(&right.title)
            .then(left.href.cmp(&right.href))
    });
    leaves.sort_by(|left, right| {
        left.title
            .cmp(&right.title)
            .then(left.root.cmp(&right.root))
            .then(left.href.cmp(&right.href))
    });
    let mut overviews: Vec<NavNode> = owners
        .iter()
        .map(|root_id| {
            let href = workspace.collection_href(root_id, path);
            let mut item = leaf(&href, "Overview", current);
            item.root = root_id.clone();
            item
        })
        .collect();
    overviews.sort_by(|left, right| left.root.cmp(&right.root));
    let mut children = overviews;
    children.extend(nested);
    children.extend(leaves);
    node.children = children;
    node.section_key = path.to_string();
    node.href = owners
        .first()
        .map(|root_id| workspace.collection_href(root_id, path))
        .unwrap_or_default();
    node.current = collection_is_current(workspace, path, current);
    node.open = node.current;
    if owners.len() == 1 {
        node.root = owners[0].clone();
    }
    node.summary = merged_collection_summary(workspace, path, owners);
    node
}

fn merged_collection_summary(workspace: &Workspace, path: &str, owners: &[String]) -> String {
    let mut parts = Vec::new();
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
        let text = first_prose_paragraph(&index.article_html);
        if text.is_empty() {
            continue;
        }
        parts.push((root_id.as_str(), text));
    }
    match parts.as_slice() {
        [] => String::new(),
        [(_, text)] => text.clone(),
        many => many
            .iter()
            .map(|(root, text)| format!("{root}: {text}"))
            .collect::<Vec<_>>()
            .join("\n"),
    }
}

fn first_prose_paragraph(article_html: &str) -> String {
    let mut rest = article_html;
    loop {
        let trimmed = rest.trim_start();
        let Some(after_open) = trimmed.strip_prefix("<h") else {
            rest = trimmed;
            break;
        };
        let Some(end) = after_open.find("</h") else {
            rest = trimmed;
            break;
        };
        let after_end = &after_open[end..];
        let Some(close) = after_end.find('>') else {
            rest = trimmed;
            break;
        };
        rest = &after_end[close + 1..];
    }
    let rest = rest.trim_start();
    let Some(after_p) = rest.strip_prefix("<p") else {
        return String::new();
    };
    let Some(gt) = after_p.find('>') else {
        return String::new();
    };
    let inner = &after_p[gt + 1..];
    let Some(end) = inner.find("</p>") else {
        return String::new();
    };
    plaintext(&inner[..end])
}

fn plaintext(html: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}
