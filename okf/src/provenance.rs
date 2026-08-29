use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::validate::parse_timestamp;

pub(crate) static GIT_INVOCATIONS: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Debug, Default)]
pub(crate) struct GitProvenance {
    last_modified: BTreeMap<String, GitModification>,
    dirty: BTreeSet<String>,
}

impl GitProvenance {
    pub(crate) fn modification(&self, relative: &str) -> GitModification {
        self.last_modified
            .get(relative)
            .copied()
            .unwrap_or(GitModification::Untracked)
    }

    pub(crate) fn is_dirty(&self, relative: &str) -> bool {
        self.dirty.contains(relative)
    }
}

pub(crate) fn git_repository_root(root: &Path) -> Option<PathBuf> {
    let output = git_command(root)
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .ok()?;
    output.status.success().then(|| {
        let path = PathBuf::from(String::from_utf8_lossy(&output.stdout).trim().to_string());
        path.canonicalize().unwrap_or(path)
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum GitModification {
    Tracked(i64),
    Untracked,
    Unknown,
}

pub(crate) fn git_path_key(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn git_command(dir: &Path) -> Command {
    GIT_INVOCATIONS.fetch_add(1, Ordering::Relaxed);
    let mut cmd = Command::new("git");
    cmd.arg("-C").arg(dir);
    cmd
}

pub(crate) fn load_git_provenance(
    repository: &Path,
    relatives: &BTreeSet<String>,
) -> GitProvenance {
    if relatives.is_empty() {
        return GitProvenance::default();
    }
    let dirty = git_dirty_paths(repository);
    let timestamps = git_last_modified_paths(repository, relatives);
    let last_modified = relatives
        .iter()
        .map(|relative| {
            let modification = match &timestamps {
                None => GitModification::Unknown,
                Some(found) => found
                    .get(relative)
                    .copied()
                    .map(GitModification::Tracked)
                    .unwrap_or(GitModification::Untracked),
            };
            (relative.clone(), modification)
        })
        .collect();
    GitProvenance {
        last_modified,
        dirty,
    }
}

fn git_dirty_paths(repository: &Path) -> BTreeSet<String> {
    let output = git_command(repository)
        .args(["status", "--porcelain", "-z", "--untracked-files=no"])
        .output();
    let Ok(output) = output else {
        return BTreeSet::new();
    };
    if !output.status.success() {
        return BTreeSet::new();
    }
    parse_porcelain_z(&output.stdout)
}

fn parse_porcelain_z(stdout: &[u8]) -> BTreeSet<String> {
    let mut dirty = BTreeSet::new();
    for record in stdout.split(|byte| *byte == 0) {
        if record.len() < 4 || record[2] != b' ' {
            continue;
        }
        let xy = &record[..2];
        if xy == b"??" || xy == b"!!" {
            continue;
        }
        let path = String::from_utf8_lossy(&record[3..]).replace('\\', "/");
        if !path.is_empty() {
            dirty.insert(path);
        }
    }
    dirty
}

fn git_last_modified_paths(
    repository: &Path,
    relatives: &BTreeSet<String>,
) -> Option<BTreeMap<String, i64>> {
    let mut cmd = git_command(repository);
    cmd.args(["log", "--format=%cI", "--name-only", "--"]);
    for relative in relatives {
        cmd.arg(relative);
    }
    let output = cmd.output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(parse_git_log_name_only(
        &String::from_utf8_lossy(&output.stdout),
        relatives,
    ))
}

fn parse_git_log_name_only(stdout: &str, wanted: &BTreeSet<String>) -> BTreeMap<String, i64> {
    let mut current_ts = None;
    let mut found = BTreeMap::new();
    for line in stdout.lines() {
        if found.len() == wanted.len() {
            break;
        }
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(timestamp) = parse_timestamp(line) {
            current_ts = Some(timestamp);
            continue;
        }
        let key = line.replace('\\', "/");
        if wanted.contains(&key)
            && let Some(timestamp) = current_ts
        {
            found.entry(key).or_insert(timestamp);
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validate::parse_timestamp;
    use crate::{Profile, load};
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp(name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("okf-provenance-{name}-{nonce}"));
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

    fn git_commit(repo: &Path, message: &str, date: &str) {
        let status = Command::new("git")
            .arg("-C")
            .arg(repo)
            .args(["commit", "-m", message])
            .env("GIT_AUTHOR_DATE", date)
            .env("GIT_COMMITTER_DATE", date)
            .env("GIT_AUTHOR_NAME", "Test")
            .env("GIT_AUTHOR_EMAIL", "test@example.com")
            .env("GIT_COMMITTER_NAME", "Test")
            .env("GIT_COMMITTER_EMAIL", "test@example.com")
            .status()
            .unwrap();
        assert!(status.success(), "git commit {message} failed");
    }

    fn concept(sources: &str) -> String {
        format!(
            "---\ntype: Architecture\ntitle: Provenance\ndescription: Provenance fixture.\ntags: [domain/okf, concern/validation]\nstatus: draft\ngenerated: {{ by: process:test, at: 2026-07-01T00:00:00Z }}\nverified:\n  - {{ by: human:nils, at: 2026-08-05T00:00:00Z }}\nauthority: descriptive\nowners: [human:nils]\nsources:\n{sources}---\n\n# Provenance\n\nBody.[^tracked]\n\n[^tracked]: Tracked source.\n"
        )
    }

    #[test]
    fn parse_git_log_keeps_first_timestamp_per_path() {
        let wanted = BTreeSet::from(["a.rs".to_string(), "b.rs".to_string()]);
        let found = parse_git_log_name_only(
            "2026-08-10T00:00:00Z\n\na.rs\n\n2026-08-01T00:00:00Z\n\na.rs\nb.rs\n",
            &wanted,
        );
        assert_eq!(
            found["a.rs"],
            parse_timestamp("2026-08-10T00:00:00Z").unwrap()
        );
        assert_eq!(
            found["b.rs"],
            parse_timestamp("2026-08-01T00:00:00Z").unwrap()
        );
    }

    #[test]
    fn parse_porcelain_z_lists_dirty_tracked_paths() {
        let stdout = b" M tracked.txt\0?? untracked.txt\0A  staged.txt\0";
        let dirty = parse_porcelain_z(stdout);
        assert!(dirty.contains("tracked.txt"));
        assert!(dirty.contains("staged.txt"));
        assert!(!dirty.contains("untracked.txt"));
    }

    #[test]
    fn batched_provenance_emits_stable_codes_with_constant_git_calls() {
        let root = temp("batch");
        git(&root, &["init", "--initial-branch=main"]);
        git(&root, &["config", "user.email", "test@example.com"]);
        git(&root, &["config", "user.name", "Test"]);
        git(&root, &["config", "commit.gpgsign", "false"]);

        fs::write(root.join("tracked.txt"), "tracked v1\n").unwrap();
        fs::write(root.join("dirty.txt"), "dirty v1\n").unwrap();
        git(&root, &["add", "tracked.txt", "dirty.txt"]);
        git_commit(&root, "initial", "2026-08-01T00:00:00Z");

        fs::write(root.join("tracked.txt"), "tracked v2\n").unwrap();
        git(&root, &["add", "tracked.txt"]);
        git_commit(&root, "update tracked", "2026-08-10T00:00:00Z");
        fs::write(root.join("dirty.txt"), "dirty v2\n").unwrap();
        fs::write(root.join("untracked.txt"), "new\n").unwrap();

        fs::write(
            root.join("note.md"),
            concept(
                "  - id: tracked\n    resource: tracked.txt\n    title: Tracked\n  - id: dirty\n    resource: dirty.txt\n    title: Dirty\n  - id: untracked\n    resource: untracked.txt\n    title: Untracked\n",
            ),
        )
        .unwrap();
        fs::write(
            root.join("copy.md"),
            concept("  - id: tracked\n    resource: tracked.txt\n    title: Tracked again\n"),
        )
        .unwrap();

        GIT_INVOCATIONS.store(0, Ordering::SeqCst);
        let bundle = load(&root, Profile::Strict).expect("load strict fixture");
        let git_calls = GIT_INVOCATIONS.load(Ordering::SeqCst);
        assert!(
            git_calls <= 3,
            "expected constant git invocations, got {git_calls}"
        );

        let codes: Vec<_> = bundle
            .diagnostics
            .iter()
            .map(|diagnostic| (diagnostic.code, diagnostic.message.as_str()))
            .collect();
        assert!(
            codes.iter().any(|(code, message)| {
                *code == "OKF4006" && message.contains("source `tracked`")
            }),
            "missing OKF4006: {codes:?}"
        );
        assert!(
            codes.iter().any(|(code, message)| {
                *code == "OKF4008" && message.contains("source `dirty`")
            }),
            "missing OKF4008: {codes:?}"
        );
        assert!(
            codes.iter().any(|(code, message)| {
                *code == "OKF4007" && message.contains("source `untracked`")
            }),
            "missing OKF4007: {codes:?}"
        );

        let _ = fs::remove_dir_all(root);
    }
}
