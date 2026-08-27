use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use okf::{Bundle, Profile};

use crate::roots::{ResolvedRoot, SyncMode};

#[derive(Clone, Debug)]
pub struct WorkspaceMember {
    pub id: String,
    pub path: PathBuf,
    pub bundle: Bundle,
}

#[derive(Clone, Debug)]
pub struct ViewTarget {
    pub workspace: Workspace,
    pub open_path: String,
}

#[derive(Clone, Debug, Default)]
pub struct Workspace {
    members: Vec<WorkspaceMember>,
}

impl Workspace {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn from_loaded(id: impl Into<String>, path: PathBuf, bundle: Bundle) -> Self {
        Self {
            members: vec![WorkspaceMember {
                id: id.into(),
                path,
                bundle,
            }],
        }
    }

    pub fn load_single(path: &Path, profile: Profile) -> Result<Self> {
        let path = path.to_path_buf();
        let bundle = okf::load(&path, profile)?;
        Ok(Self::from_loaded(id_from_path(&path), path, bundle))
    }

    pub fn from_resolved(roots: Vec<ResolvedRoot>, profile: Profile) -> Result<Self> {
        let members = roots
            .into_iter()
            .filter(ResolvedRoot::enabled)
            .filter_map(|root| root.path.map(|path| (root.id, path)))
            .collect();
        Self::load_members(members, profile)
    }

    pub fn from_members(mut members: Vec<WorkspaceMember>) -> Self {
        members.sort_by(|left, right| left.id.cmp(&right.id));
        Self { members }
    }

    pub fn load_members(specs: Vec<(String, PathBuf)>, profile: Profile) -> Result<Self> {
        let mut members = Vec::with_capacity(specs.len());
        for (id, path) in specs {
            let bundle = okf::load(&path, profile)
                .with_context(|| format!("failed to load knowledge root `{id}`"))?;
            members.push(WorkspaceMember { id, path, bundle });
        }
        Ok(Self::from_members(members))
    }

    pub fn for_view(
        path: Option<&Path>,
        profile: Profile,
        config_path: &Path,
        cache_parent: &Path,
        session_bundle: Option<&Path>,
    ) -> Result<ViewTarget> {
        if let Some(path) = path {
            let target = okf::resolve_preview_path(path)?;
            let workspace = Self::load_single(&target.root, profile)?;
            return Ok(ViewTarget {
                workspace,
                open_path: target.open_path,
            });
        }
        let config = crate::config::load_or_default(config_path);
        if !config.roots.is_empty() {
            let resolved = crate::roots::resolve_all(&config, cache_parent, SyncMode::Auto);
            let enabled: Vec<ResolvedRoot> =
                resolved.into_iter().filter(ResolvedRoot::enabled).collect();
            if enabled.len() >= 2 {
                return Ok(ViewTarget {
                    workspace: Self::from_resolved(enabled, profile)?,
                    open_path: "/".into(),
                });
            }
        }
        if let Some(bundle) = session_bundle.filter(|path| path.is_dir()) {
            return Ok(ViewTarget {
                workspace: Self::load_single(bundle, profile)?,
                open_path: "/".into(),
            });
        }
        let default = PathBuf::from("knowledge");
        if default.is_dir() {
            return Ok(ViewTarget {
                workspace: Self::load_single(&default, profile)?,
                open_path: "/".into(),
            });
        }
        bail!("pass a knowledge bundle path, or open one first so ~/.okmate/state remembers it");
    }

    pub fn reload(&self, profile: Profile) -> Result<Self> {
        let specs = self
            .members
            .iter()
            .map(|member| (member.id.clone(), member.path.clone()))
            .collect();
        Self::load_members(specs, profile)
    }

    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    pub fn is_multi(&self) -> bool {
        self.members.len() > 1
    }

    pub fn len(&self) -> usize {
        self.members.len()
    }

    pub fn members(&self) -> &[WorkspaceMember] {
        &self.members
    }

    pub fn primary(&self) -> Option<&WorkspaceMember> {
        self.members.first()
    }

    pub fn primary_path(&self) -> Option<&Path> {
        self.primary().map(|member| member.path.as_path())
    }

    pub fn get(&self, id: &str) -> Option<&WorkspaceMember> {
        self.members.iter().find(|member| member.id == id)
    }

    pub fn watch_paths(&self) -> Vec<PathBuf> {
        self.members
            .iter()
            .map(|member| member.path.clone())
            .collect()
    }

    pub fn document_href(&self, root_id: &str, concept_id: &str) -> String {
        let id = concept_id.trim_matches('/');
        if self.is_multi() {
            format!("/@{root_id}/{id}/")
        } else {
            format!("/{id}/")
        }
    }

    pub fn collection_href(&self, root_id: &str, collection: &str) -> String {
        self.document_href(root_id, collection)
    }

    pub fn chrome_route(route: &str) -> bool {
        matches!(
            normalize_route(route).as_str(),
            "/" | "/review/" | "/log/" | "/settings/"
        )
    }

    pub fn parse_document_route(&self, route: &str) -> Option<(&WorkspaceMember, String)> {
        let route = normalize_route(route);
        if Self::chrome_route(&route) {
            return None;
        }
        let trimmed = route.trim_matches('/');
        if self.is_multi() {
            let rest = trimmed.strip_prefix('@')?;
            let (root_id, concept) = rest.split_once('/')?;
            let member = self.get(root_id)?;
            Some((member, concept.trim_matches('/').to_string()))
        } else {
            let member = self.primary()?;
            Some((member, trimmed.to_string()))
        }
    }

    pub fn rewrite_article(&self, owner_id: &str, html: &str) -> String {
        if !self.is_multi() {
            return html.to_string();
        }
        rewrite_hrefs(html, |href| self.rewrite_href(owner_id, href))
    }

    fn rewrite_href(&self, owner_id: &str, href: &str) -> String {
        if let Some(rest) = href.strip_prefix("okf:") {
            return self.rewrite_okf(rest).unwrap_or_else(|| href.to_string());
        }
        self.rewrite_local(owner_id, href)
    }

    fn rewrite_okf(&self, rest: &str) -> Option<String> {
        let (path, fragment) = split_fragment(rest);
        let (root_id, bundle_path) = path.split_once('/')?;
        if self.get(root_id).is_none() {
            return None;
        }
        let id = bundle_path.strip_suffix(".md").unwrap_or(bundle_path);
        Some(with_fragment(&self.document_href(root_id, id), fragment))
    }

    fn rewrite_local(&self, owner_id: &str, href: &str) -> String {
        if href.starts_with("/@") || href.starts_with("/__okmate") {
            return href.to_string();
        }
        let Some(path) = href.strip_prefix('/') else {
            return href.to_string();
        };
        let (path, fragment) = split_fragment(path);
        let id = path.trim_matches('/');
        if id.is_empty() || id == "review" || id == "log" || id == "settings" {
            return href.to_string();
        }
        with_fragment(&self.document_href(owner_id, id), fragment)
    }
}

pub fn id_from_path(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .filter(|name| crate::config::valid_id(name))
        .unwrap_or("bundle")
        .to_string()
}

pub fn normalize_route(route: &str) -> String {
    let path = route.split(['?', '#']).next().unwrap_or(route);
    if path.is_empty() || path == "/" {
        return "/".into();
    }
    format!("/{}/", path.trim_matches('/'))
}

fn split_fragment(value: &str) -> (&str, Option<&str>) {
    match value.split_once('#') {
        Some((path, fragment)) => (path, Some(fragment)),
        None => (value, None),
    }
}

fn with_fragment(href: &str, fragment: Option<&str>) -> String {
    match fragment {
        Some(fragment) if !fragment.is_empty() => format!("{href}#{fragment}"),
        _ => href.to_string(),
    }
}

fn rewrite_hrefs(html: &str, mut rewrite: impl FnMut(&str) -> String) -> String {
    let mut out = String::with_capacity(html.len() + 32);
    let mut rest = html;
    const MARKER: &str = "href=\"";
    while let Some(idx) = rest.find(MARKER) {
        out.push_str(&rest[..idx + MARKER.len()]);
        rest = &rest[idx + MARKER.len()..];
        let Some(end) = rest.find('"') else {
            out.push_str(rest);
            return out;
        };
        out.push_str(&rewrite(&rest[..end]));
        out.push('"');
        rest = &rest[end + 1..];
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp(name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "okmate-ws-{}-{}-{}",
            name,
            std::process::id(),
            nonce
        ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).unwrap();
        path
    }

    fn write_bundle(root: &Path, shared_title: &str, unique: bool) {
        fs::write(
            root.join("index.md"),
            "---\nokf_version: \"0.2\"\n---\n\n# Knowledge\n",
        )
        .unwrap();
        fs::create_dir_all(root.join("plans")).unwrap();
        fs::write(root.join("plans").join("index.md"), "# Plans\n").unwrap();
        fs::write(
            root.join("plans").join("shared.md"),
            format!(
                "---\ntype: Architecture\ntitle: {shared_title}\ndescription: Shared concept.\ntags: [domain/okf, concern/architecture]\nstatus: draft\ngenerated: {{ by: process:test, at: 2026-08-17T00:00:00Z }}\nauthority: descriptive\nowners: [human:nils]\n---\n\n# {shared_title}\n\nSee [notes](okf:b/plans/shared.md#goal) and [local](/plans/shared/).\n"
            ),
        )
        .unwrap();
        if unique {
            fs::write(
                root.join("unique.md"),
                "---\ntype: Architecture\ntitle: Unique\ndescription: Only in a.\ntags: [domain/okf, concern/architecture]\nstatus: draft\ngenerated: { by: process:test, at: 2026-08-17T00:00:00Z }\nauthority: descriptive\nowners: [human:nils]\n---\n\n# Unique\n\nOnly here.\n",
            )
            .unwrap();
        }
    }

    fn two_bundles() -> (PathBuf, PathBuf, Workspace) {
        let a = temp("a");
        let b = temp("b");
        write_bundle(&a, "Alpha Shared", true);
        write_bundle(&b, "Beta Shared", false);
        let workspace = Workspace::load_members(
            vec![("a".into(), a.clone()), ("b".into(), b.clone())],
            Profile::Strict,
        )
        .unwrap();
        (a, b, workspace)
    }

    #[test]
    fn multi_workspace_prefixes_document_hrefs() {
        let (_a, _b, workspace) = two_bundles();
        assert!(workspace.is_multi());
        assert_eq!(
            workspace.document_href("a", "plans/shared"),
            "/@a/plans/shared/"
        );
        assert_eq!(
            workspace.document_href("b", "plans/shared"),
            "/@b/plans/shared/"
        );
        let (member, id) = workspace.parse_document_route("/@b/plans/shared/").unwrap();
        assert_eq!(member.id, "b");
        assert_eq!(id, "plans/shared");
        assert!(workspace.parse_document_route("/plans/shared/").is_none());
        assert!(workspace.parse_document_route("/").is_none());
    }

    #[test]
    fn single_workspace_keeps_unprefixed_routes() {
        let root = temp("one");
        write_bundle(&root, "Solo Shared", false);
        let workspace = Workspace::load_single(&root, Profile::Strict).unwrap();
        assert!(!workspace.is_multi());
        assert_eq!(
            workspace.document_href("ignored", "plans/shared"),
            "/plans/shared/"
        );
        let (_member, id) = workspace.parse_document_route("/plans/shared/").unwrap();
        assert_eq!(id, "plans/shared");
    }

    #[test]
    fn rewrite_prefixes_local_and_loaded_okf_hrefs() {
        let (_a, _b, workspace) = two_bundles();
        let html = workspace.rewrite_article(
            "a",
            r#"<p><a href="/plans/shared/">local</a> <a href="okf:b/plans/shared.md#goal">okf</a> <a href="okf:missing/x.md">skip</a> <a href="/review/">review</a></p>"#,
        );
        assert!(html.contains("href=\"/@a/plans/shared/\""), "{html}");
        assert!(html.contains("href=\"/@b/plans/shared/#goal\""), "{html}");
        assert!(html.contains("href=\"okf:missing/x.md\""), "{html}");
        assert!(html.contains("href=\"/review/\""), "{html}");
    }

    #[test]
    fn for_view_explicit_path_stays_single_even_with_two_config_roots() {
        let (a, b, _workspace) = two_bundles();
        let cfg_dir = temp("cfg");
        let config = cfg_dir.join("config.toml");
        fs::write(
            &config,
            format!(
                "[[roots]]\nid = \"a\"\nkind = \"directory\"\npath = \"{}\"\n[[roots]]\nid = \"b\"\nkind = \"directory\"\npath = \"{}\"\n",
                a.display(),
                b.display()
            ),
        )
        .unwrap();
        let loaded = Workspace::for_view(
            Some(&a),
            Profile::Strict,
            &config,
            &cfg_dir.join("cache"),
            None,
        )
        .unwrap();
        assert!(!loaded.workspace.is_multi());
        assert_eq!(
            loaded.workspace.primary().unwrap().path,
            a.canonicalize().unwrap()
        );

        let loaded =
            Workspace::for_view(None, Profile::Strict, &config, &cfg_dir.join("cache"), None)
                .unwrap();
        assert!(loaded.workspace.is_multi());
        assert_eq!(loaded.workspace.len(), 2);
        assert!(loaded.workspace.get("a").is_some());
        assert!(loaded.workspace.get("b").is_some());
        assert_eq!(loaded.open_path, "/");
    }
}
