mod common;

use std::fs;
use std::process::Command;

use common::{okmate_bin, temp_dir, write_index};

#[test]
fn help_lists_init() {
    let output = Command::new(okmate_bin()).arg("--help").output().unwrap();
    assert!(output.status.success());
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        text.contains("init"),
        "expected help listing init, got: {text}"
    );
}

#[test]
fn init_help_lists_flags() {
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg("--help")
        .output()
        .unwrap();
    assert!(output.status.success());
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(text.contains("--apply"), "{text}");
    assert!(text.contains("--bare"), "{text}");
    assert!(text.contains("--title"), "{text}");
    assert!(text.contains("--format"), "{text}");
}

#[test]
fn dry_run_prints_index_and_does_not_create() {
    let root = temp_dir("init-dry");
    let target = root.join("knowledge");
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("index.md"), "{stdout}");
    assert!(!target.join("index.md").exists());
    assert!(!target.exists() || fs::read_dir(&target).map(|d| d.count()).unwrap_or(0) == 0);
}

#[test]
fn format_json_parses() {
    let root = temp_dir("init-json");
    let target = root.join("knowledge");
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: serde_json::Value =
        serde_json::from_str(String::from_utf8_lossy(&output.stdout).trim()).unwrap();
    let files = value["files"].as_array().expect("files array");
    assert!(
        files.iter().any(|file| file["relative"] == "index.md"),
        "{value}"
    );
    assert!(files.iter().any(|file| file["op"] == "create"), "{value}");
}

#[test]
fn occupied_bundle_errors() {
    let root = temp_dir("init-occupied");
    write_index(&root);
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&root)
        .output()
        .unwrap();
    assert!(!output.status.success());
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        text.contains("okf_version") || text.contains("already an OKF bundle"),
        "{text}"
    );
    assert!(!root.join("log.md").exists());
}

#[test]
fn bare_omits_architecture_index() {
    let root = temp_dir("init-bare");
    let target = root.join("knowledge");
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .arg("--bare")
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: serde_json::Value =
        serde_json::from_str(String::from_utf8_lossy(&output.stdout).trim()).unwrap();
    let files = value["files"].as_array().expect("files array");
    assert!(
        files.iter().any(|file| file["relative"] == "index.md"),
        "{value}"
    );
    assert!(
        files.iter().any(|file| file["relative"] == "log.md"),
        "{value}"
    );
    assert!(
        !files
            .iter()
            .any(|file| file["relative"] == "architecture/index.md"),
        "{value}"
    );
}

fn check_json(root: &std::path::Path) -> (bool, String) {
    let output = Command::new(okmate_bin())
        .arg("check")
        .arg(root)
        .arg("--profile")
        .arg("strict")
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();
    (
        output.status.success(),
        String::from_utf8_lossy(&output.stdout).into_owned(),
    )
}

#[test]
fn apply_creates_check_clean_knowledge_child() {
    let parent = temp_dir("init-apply");
    let target = parent.join("knowledge");
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .arg("--apply")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(target.join("index.md").is_file());
    assert!(target.join("architecture/index.md").is_file());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("wrote  index.md"), "{stdout}");
    assert!(stdout.contains("okmate check"), "{stdout}");
    let (ok, json) = check_json(&target);
    assert!(ok, "{json}");
}

#[test]
fn second_apply_fails_without_mutating() {
    let parent = temp_dir("init-reapply");
    let target = parent.join("knowledge");
    let first = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .arg("--apply")
        .output()
        .unwrap();
    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    let before = fs::read(target.join("index.md")).unwrap();
    let second = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .arg("--apply")
        .output()
        .unwrap();
    assert!(!second.status.success());
    let after = fs::read(target.join("index.md")).unwrap();
    assert_eq!(before, after);
    assert!(!target.join("architecture/index.md.bak").exists());
}

#[test]
fn bare_apply_is_check_clean() {
    let parent = temp_dir("init-bare-apply");
    let target = parent.join("knowledge");
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .arg("--bare")
        .arg("--apply")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(target.join("index.md").is_file());
    assert!(target.join("log.md").is_file());
    assert!(!target.join("architecture/index.md").exists());
    let (ok, json) = check_json(&target);
    assert!(ok, "{json}");
}

#[test]
fn apply_refuses_existing_okf_version() {
    let root = temp_dir("init-apply-occupied");
    write_index(&root);
    let before = fs::read(root.join("index.md")).unwrap();
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&root)
        .arg("--apply")
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert_eq!(fs::read(root.join("index.md")).unwrap(), before);
    assert!(!root.join("log.md").exists());
}
