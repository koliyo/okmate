mod common;

use std::fs;
use std::process::Command;

use common::{okmate_bin, temp_dir, write_index};

fn write_collection_index(root: &std::path::Path, dir: &str, heading: &str) {
    let path = root.join(dir);
    fs::create_dir_all(&path).unwrap();
    fs::write(
        path.join("index.md"),
        format!("# {heading}\n\nExisting grouping.\n"),
    )
    .unwrap();
}

#[test]
fn concept_dry_run_does_not_write() {
    let root = temp_dir("concept-dry");
    write_index(&root);
    let output = Command::new(okmate_bin())
        .arg("concept")
        .arg(&root)
        .arg("--type")
        .arg("Explanation")
        .arg("--id")
        .arg("guides/onboarding")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Would create"), "{stdout}");
    assert!(stdout.contains("type: Explanation"), "{stdout}");
    assert!(!stdout.contains("owners:"), "{stdout}");
    assert!(!root.join("guides/onboarding.md").exists());
}

#[test]
fn concept_apply_writes_and_second_apply_collides() {
    let root = temp_dir("concept-apply");
    write_index(&root);
    let output = Command::new(okmate_bin())
        .arg("concept")
        .arg(&root)
        .arg("--type")
        .arg("Explanation")
        .arg("--id")
        .arg("guides/onboarding")
        .arg("--title")
        .arg("Onboarding")
        .arg("--apply")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let body = fs::read_to_string(root.join("guides/onboarding.md")).unwrap();
    assert!(body.contains("type: Explanation"), "{body}");
    assert!(body.contains("title: Onboarding"), "{body}");
    assert!(!body.contains("owners:"), "{body}");
    assert!(!body.contains("generated:"), "{body}");

    let again = Command::new(okmate_bin())
        .arg("concept")
        .arg(&root)
        .arg("--type")
        .arg("Explanation")
        .arg("--id")
        .arg("guides/onboarding")
        .arg("--apply")
        .output()
        .unwrap();
    assert!(!again.status.success());
    let stderr = String::from_utf8_lossy(&again.stderr);
    assert!(stderr.contains("already exists"), "{stderr}");
}

#[test]
fn concept_evidence_requires_named_inputs() {
    let root = temp_dir("concept-evidence");
    write_index(&root);
    let output = Command::new(okmate_bin())
        .arg("concept")
        .arg(&root)
        .arg("--type")
        .arg("Runbook")
        .arg("--id")
        .arg("runbooks/restore")
        .arg("--profile")
        .arg("evidence")
        .arg("--apply")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--title"), "{stderr}");
    assert!(stderr.contains("--owner"), "{stderr}");
    assert!(!root.join("runbooks/restore.md").exists());
}

#[test]
fn concept_evidence_writes_when_inputs_are_explicit() {
    let root = temp_dir("concept-evidence-ok");
    write_index(&root);
    let output = Command::new(okmate_bin())
        .arg("concept")
        .arg(&root)
        .arg("--type")
        .arg("Runbook")
        .arg("--id")
        .arg("runbooks/restore")
        .arg("--profile")
        .arg("evidence")
        .arg("--title")
        .arg("Restore")
        .arg("--description")
        .arg("Recover a host.")
        .arg("--authority")
        .arg("descriptive")
        .arg("--owner")
        .arg("human:nils")
        .arg("--generated-by")
        .arg("process:test")
        .arg("--apply")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let body = fs::read_to_string(root.join("runbooks/restore.md")).unwrap();
    assert!(body.contains("owners: [human:nils]"), "{body}");
    assert!(body.contains("generated: { by: process:test"), "{body}");
    let check = Command::new(okmate_bin())
        .arg("check")
        .arg(&root)
        .arg("--profile")
        .arg("evidence")
        .output()
        .unwrap();
    assert!(
        check.status.success(),
        "{}",
        String::from_utf8_lossy(&check.stderr)
    );
}

#[test]
fn index_proposes_unlisted_then_apply_is_idempotent() {
    let root = temp_dir("index-update");
    write_index(&root);
    write_collection_index(&root, "guides", "Guides");
    fs::write(
        root.join("guides/listed.md"),
        "---\ntype: Explanation\ntitle: Listed\ndescription: Already linked.\n---\n\n# Listed\n",
    )
    .unwrap();
    fs::write(
        root.join("guides/unlisted.md"),
        "---\ntype: Explanation\ntitle: Unlisted\ndescription: Needs an index link.\n---\n\n# Unlisted\n",
    )
    .unwrap();
    let mut index = fs::read_to_string(root.join("guides/index.md")).unwrap();
    index.push_str("\n* [Listed](listed.md) - Already linked.\n* [Gone](missing.md)\n");
    fs::write(root.join("guides/index.md"), index).unwrap();

    let dry = Command::new(okmate_bin())
        .arg("index")
        .arg(&root)
        .output()
        .unwrap();
    assert!(
        dry.status.success(),
        "{}",
        String::from_utf8_lossy(&dry.stderr)
    );
    let stdout = String::from_utf8_lossy(&dry.stdout);
    assert!(stdout.contains("Would update guides/index.md"), "{stdout}");
    assert!(stdout.contains("+* [Unlisted](unlisted.md)"), "{stdout}");
    assert!(stdout.contains("unresolved"), "{stdout}");
    let before = fs::read_to_string(root.join("guides/index.md")).unwrap();
    assert!(before.contains("Existing grouping."), "{before}");
    assert!(!before.contains("unlisted.md"), "{before}");

    let apply = Command::new(okmate_bin())
        .arg("index")
        .arg(&root)
        .arg("--apply")
        .output()
        .unwrap();
    assert!(
        apply.status.success(),
        "{}",
        String::from_utf8_lossy(&apply.stderr)
    );
    let after = fs::read_to_string(root.join("guides/index.md")).unwrap();
    assert!(after.contains("Existing grouping."), "{after}");
    assert!(
        after.contains("* [Listed](listed.md) - Already linked."),
        "{after}"
    );
    assert!(
        after.contains("* [Unlisted](unlisted.md) - Needs an index link."),
        "{after}"
    );
    assert!(after.find("Existing grouping.").unwrap() < after.find("Unlisted").unwrap());

    let again = Command::new(okmate_bin())
        .arg("index")
        .arg(&root)
        .arg("--apply")
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();
    assert!(again.status.success());
    let json = String::from_utf8_lossy(&again.stdout);
    assert!(
        !json.contains("unlisted"),
        "second apply should not propose the same addition: {json}"
    );
}

#[test]
fn handbook_record_can_be_created_indexed_and_checked() {
    let root = temp_dir("handbook-author");
    write_index(&root);
    write_collection_index(&root, "agentic-development", "Agentic development");
    fs::write(
        root.join("agentic-development/task-steering.md"),
        "---\ntype: Explanation\ntitle: Task steering\ndescription: Delegate work.\n---\n\n# Task steering\n",
    )
    .unwrap();
    fs::write(
        root.join("agentic-development/index.md"),
        "# Agentic development\n\n* [Task steering](task-steering.md) - How to delegate.\n",
    )
    .unwrap();

    let create = Command::new(okmate_bin())
        .arg("concept")
        .arg(&root)
        .arg("--type")
        .arg("Explanation")
        .arg("--id")
        .arg("agentic-development/review-loop")
        .arg("--title")
        .arg("Review loop")
        .arg("--description")
        .arg("Keep delegated work on a reviewable path.")
        .arg("--apply")
        .output()
        .unwrap();
    assert!(
        create.status.success(),
        "{}",
        String::from_utf8_lossy(&create.stderr)
    );

    let index = Command::new(okmate_bin())
        .arg("index")
        .arg(&root)
        .arg("--apply")
        .output()
        .unwrap();
    assert!(
        index.status.success(),
        "{}",
        String::from_utf8_lossy(&index.stderr)
    );
    let listing = fs::read_to_string(root.join("agentic-development/index.md")).unwrap();
    assert!(listing.contains("Task steering"), "{listing}");
    assert!(listing.contains("Review loop"), "{listing}");

    let check = Command::new(okmate_bin())
        .arg("check")
        .arg(&root)
        .arg("--profile")
        .arg("base")
        .output()
        .unwrap();
    assert!(
        check.status.success(),
        "{}",
        String::from_utf8_lossy(&check.stderr)
    );
}

#[test]
fn empty_scaffold_and_duplicate_overview_are_style() {
    let root = temp_dir("style-layout");
    write_index(&root);
    write_collection_index(&root, "empty", "Knowledge");
    fs::write(root.join("okmate.toml"), "version = 1\n").unwrap();

    let report = okmate::check(&root, okf::Profile::Base).unwrap();
    assert!(!report.has_errors(), "{:?}", report.diagnostics);
    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "OKMATE5004"),
        "{:?}",
        report.diagnostics
    );
    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "OKMATE5005"),
        "{:?}",
        report.diagnostics
    );
}
