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
