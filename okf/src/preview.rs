use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde_json::Value;

use crate::frontmatter::{parse_yaml_mapping, split_frontmatter};
use crate::validate::git_repository_root;
use crate::{absolute, relative_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreviewTarget {
    pub root: PathBuf,
    pub open_path: String,
}

impl PreviewTarget {
    pub fn bundle(root: PathBuf) -> Self {
        Self {
            root,
            open_path: "/".into(),
        }
    }

    pub fn concept(root: PathBuf, id: impl Into<String>) -> Self {
        let id = id.into();
        Self {
            root,
            open_path: format!("/{id}/"),
        }
    }
}

pub fn resolve_preview_path(path: &Path) -> Result<PreviewTarget> {
    let path = absolute(path)?;
    if path.is_dir() {
        let root = fs::canonicalize(&path)
            .with_context(|| format!("failed to resolve knowledge root {}", path.display()))?;
        if dir_is_bundle_root(&root) {
            return Ok(PreviewTarget::bundle(root));
        }
        if git_repository_root(&root).as_ref() == Some(&root) {
            return Ok(PreviewTarget::bundle(resolve_bundle(&root)?));
        }
        return Ok(PreviewTarget::bundle(root));
    }
    if !path.is_file() {
        bail!("no such knowledge path: {}", path.display());
    }
    if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
        bail!(
            "unsupported file for `okmate view`: {}; expected a knowledge bundle directory or a .md file",
            path.display()
        );
    }
    let canonical =
        fs::canonicalize(&path).with_context(|| format!("failed to resolve {}", path.display()))?;
    let Some(root) = enclosing_bundle_root(&canonical)? else {
        bail!(
            "{} is not inside an OKF bundle; pass a bundle directory or a Markdown file under one",
            path.display()
        );
    };
    let relative = relative_path(&root, &canonical);
    match relative.rsplit('/').next() {
        Some("index.md") if relative == "index.md" => Ok(PreviewTarget::bundle(root)),
        Some("index.md") => bail!(
            "{} is a collection index, not a concept; preview the bundle with `okmate view {}`",
            path.display(),
            root.display()
        ),
        Some("log.md") => bail!(
            "{} is a knowledge log, not a concept; preview the bundle with `okmate view {}`",
            path.display(),
            root.display()
        ),
        _ => {
            let id = relative.strip_suffix(".md").unwrap_or(&relative);
            Ok(PreviewTarget::concept(root, id))
        }
    }
}

fn enclosing_bundle_root(file: &Path) -> Result<Option<PathBuf>> {
    let mut dir = file.parent();
    while let Some(current) = dir {
        let index = current.join("index.md");
        if index.is_file() {
            let source = fs::read_to_string(&index)
                .with_context(|| format!("failed to read {}", index.display()))?;
            if is_bundle_root_index(&source) {
                let root = fs::canonicalize(current).with_context(|| {
                    format!("failed to resolve knowledge root {}", current.display())
                })?;
                return Ok(Some(root));
            }
        }
        dir = current.parent();
    }
    Ok(None)
}

pub fn is_bundle_root_index(source: &str) -> bool {
    let Ok(Some(frontmatter)) = split_frontmatter(source, false) else {
        return false;
    };
    let Ok(metadata) = parse_yaml_mapping(frontmatter.yaml.of(source)) else {
        return false;
    };
    metadata
        .get("okf_version")
        .and_then(Value::as_str)
        .is_some()
}

pub fn discover_bundles(git_toplevel: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    if let Some(root) = bundle_dir(git_toplevel) {
        found.push(root);
    }
    let Ok(entries) = fs::read_dir(git_toplevel) else {
        return found;
    };
    let mut entries = entries.filter_map(Result::ok).collect::<Vec<_>>();
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        if entry.file_name().as_encoded_bytes().starts_with(b".") {
            continue;
        }
        let path = entry.path();
        if path.is_dir()
            && let Some(root) = bundle_dir(&path)
        {
            found.push(root);
        }
    }
    found
}

pub fn resolve_bundle(git_toplevel: &Path) -> Result<PathBuf> {
    let found = discover_bundles(git_toplevel);
    match found.as_slice() {
        [] => bail!(
            "no OKF bundle found under {}; expected index.md with okf_version at the repo root or in an immediate child directory",
            git_toplevel.display()
        ),
        [one] => Ok(one.clone()),
        many => {
            let knowledge: Vec<_> = many
                .iter()
                .filter(|path| path.file_name().and_then(|name| name.to_str()) == Some("knowledge"))
                .collect();
            if let [one] = knowledge.as_slice() {
                return Ok((*one).clone());
            }
            let listed = many
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
                .join(", ");
            bail!(
                "multiple OKF bundles found under {}: {listed}",
                git_toplevel.display()
            )
        }
    }
}

fn dir_is_bundle_root(dir: &Path) -> bool {
    bundle_dir(dir).is_some()
}

fn bundle_dir(dir: &Path) -> Option<PathBuf> {
    let index = dir.join("index.md");
    if !index.is_file() {
        return None;
    }
    let source = fs::read_to_string(&index).ok()?;
    if !is_bundle_root_index(&source) {
        return None;
    }
    Some(fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp(name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("okf-preview-{name}-{}-{nonce}", std::process::id()));
        fs::create_dir_all(&path).unwrap();
        path
    }

    fn git(repo: &Path, args: &[&str]) {
        let status = Command::new("git")
            .arg("-C")
            .arg(repo)
            .args(args)
            .status()
            .unwrap();
        assert!(status.success(), "git {args:?} failed");
    }

    fn git_repo(name: &str) -> PathBuf {
        let root = temp(name);
        git(&root, &["init", "--initial-branch=main"]);
        git(&root, &["config", "user.email", "test@example.com"]);
        git(&root, &["config", "user.name", "Test"]);
        git(&root, &["config", "commit.gpgsign", "false"]);
        root
    }

    fn write_bundle(dir: &Path) {
        fs::create_dir_all(dir).unwrap();
        fs::write(
            dir.join("index.md"),
            "---\nokf_version: \"0.2\"\n---\n\n# Knowledge\n",
        )
        .unwrap();
    }

    fn write_collection(dir: &Path) {
        fs::create_dir_all(dir).unwrap();
        fs::write(dir.join("index.md"), "# Plans\n").unwrap();
    }

    #[test]
    fn is_bundle_root_index_requires_okf_version() {
        assert!(is_bundle_root_index(
            "---\nokf_version: \"0.2\"\n---\n\n# Knowledge\n"
        ));
        assert!(!is_bundle_root_index("# Plans\n"));
        assert!(!is_bundle_root_index(
            "---\ntitle: Not a bundle\n---\n\n# Nope\n"
        ));
    }

    #[test]
    fn resolve_bundle_finds_this_repo_knowledge() {
        let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let git = git_repository_root(&crate_dir).expect("okf lives in a git checkout");
        let bundle = resolve_bundle(&git).unwrap();
        assert_eq!(
            bundle.file_name().and_then(|name| name.to_str()),
            Some("knowledge")
        );
        assert!(bundle.join("index.md").is_file(), "{}", bundle.display());
    }

    #[test]
    fn discover_prefers_knowledge_child_in_git_repo() {
        let repo = git_repo("knowledge-child");
        write_bundle(&repo.join("knowledge"));
        write_collection(&repo.join("plans"));
        let found = discover_bundles(&repo);
        assert_eq!(
            found,
            vec![fs::canonicalize(repo.join("knowledge")).unwrap()]
        );
        assert_eq!(
            resolve_bundle(&repo).unwrap(),
            fs::canonicalize(repo.join("knowledge")).unwrap()
        );
        let preview = resolve_preview_path(&repo).unwrap();
        assert_eq!(
            preview.root,
            fs::canonicalize(repo.join("knowledge")).unwrap()
        );
        let _ = fs::remove_dir_all(repo);
    }

    #[test]
    fn discover_keeps_bundle_at_git_toplevel() {
        let repo = git_repo("root-bundle");
        write_bundle(&repo);
        write_collection(&repo.join("plans"));
        assert_eq!(
            discover_bundles(&repo),
            vec![fs::canonicalize(&repo).unwrap()]
        );
        assert_eq!(
            resolve_bundle(&repo).unwrap(),
            fs::canonicalize(&repo).unwrap()
        );
        let preview = resolve_preview_path(&repo).unwrap();
        assert_eq!(preview.root, fs::canonicalize(&repo).unwrap());
        let _ = fs::remove_dir_all(repo);
    }

    #[test]
    fn discover_ignores_collection_index_and_hidden_dirs() {
        let repo = git_repo("ignore-hidden");
        write_bundle(&repo.join("knowledge"));
        write_collection(&repo.join("plans"));
        write_bundle(&repo.join(".foo"));
        let found = discover_bundles(&repo);
        assert_eq!(
            found,
            vec![fs::canonicalize(repo.join("knowledge")).unwrap()]
        );
        let _ = fs::remove_dir_all(repo);
    }

    #[test]
    fn resolve_errors_when_two_unnamed_bundles_exist() {
        let repo = git_repo("two-bundles");
        write_bundle(&repo.join("alpha"));
        write_bundle(&repo.join("beta"));
        let error = resolve_bundle(&repo).unwrap_err().to_string();
        assert!(error.contains("multiple OKF bundles"), "{error}");
        assert!(error.contains("alpha"), "{error}");
        assert!(error.contains("beta"), "{error}");
        let preview = resolve_preview_path(&repo).unwrap_err().to_string();
        assert!(preview.contains("multiple OKF bundles"), "{preview}");
        let _ = fs::remove_dir_all(repo);
    }

    #[test]
    fn resolve_prefers_knowledge_among_multiple_hits() {
        let repo = git_repo("knowledge-plus");
        write_bundle(&repo.join("docs"));
        write_bundle(&repo.join("knowledge"));
        assert_eq!(
            resolve_bundle(&repo).unwrap(),
            fs::canonicalize(repo.join("knowledge")).unwrap()
        );
        let preview = resolve_preview_path(&repo).unwrap();
        assert_eq!(
            preview.root,
            fs::canonicalize(repo.join("knowledge")).unwrap()
        );
        let _ = fs::remove_dir_all(repo);
    }

    #[test]
    fn non_git_dir_does_not_infer_child_bundle() {
        let root = temp("nongit");
        write_bundle(&root.join("knowledge"));
        let preview = resolve_preview_path(&root).unwrap();
        assert_eq!(preview.root, fs::canonicalize(&root).unwrap());
        assert_eq!(
            discover_bundles(&root),
            vec![fs::canonicalize(root.join("knowledge")).unwrap()]
        );
        let _ = fs::remove_dir_all(root);
    }
}
