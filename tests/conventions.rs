mod common;

use std::fs;

use common::{okmate_bin, temp_dir, write_index};
use okf::{DiagnosticLayer, Profile, Severity};

fn write_note(root: &std::path::Path, relative: &str, kind: &str, extra: &str) {
    if let Some(parent) = root.join(relative).parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(
        root.join(relative),
        format!(
            "---\ntype: {kind}\ntitle: Example\ndescription: A convention fixture.\ngenerated: {{ by: process:test, at: 2026-09-12T00:00:00Z }}\nauthority: descriptive\nowners: [human:nils]\n{extra}---\n\n# Example\n\nBody.\n"
        ),
    )
    .unwrap();
}

#[test]
fn missing_conventions_file_adds_no_style_findings() {
    let root = temp_dir("style-missing");
    write_index(&root);
    write_note(&root, "workflow.md", "Workflow", "tags: [ops]\n");

    let report = okmate::check(&root, Profile::Evidence).unwrap();
    assert!(
        !report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.layer == DiagnosticLayer::Style),
        "{:?}",
        report.diagnostics
    );
    assert!(!report.has_errors());
}

#[test]
fn declared_style_is_quiet_for_matching_records() {
    let root = temp_dir("style-declared");
    write_index(&root);
    write_note(&root, "runbooks/restore.md", "Runbook", "tags: [ops]\n");
    fs::write(
        root.join("okmate.toml"),
        "version = 1\n\
         preferred_types = [\"Runbook\"]\n\
         preferred_tags = [\"ops\"]\n\
         authoring_guide = \"index.md\"\n\
         \n\
         [type_paths]\n\
         Runbook = \"runbooks\"\n",
    )
    .unwrap();

    let report = okmate::check(&root, Profile::Evidence).unwrap();
    assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
}

#[test]
fn undeclared_type_and_path_mismatch_are_style_warnings() {
    let root = temp_dir("style-mismatch");
    write_index(&root);
    write_note(&root, "guides/restore.md", "Workflow", "tags: [adhoc]\n");
    fs::write(
        root.join("okmate.toml"),
        "version = 1\n\
         preferred_types = [\"Runbook\"]\n\
         preferred_tags = [\"ops\"]\n\
         extra_policy = true\n\
         \n\
         [type_paths]\n\
         Workflow = \"workflows\"\n",
    )
    .unwrap();

    let report = okmate::check(&root, Profile::Evidence).unwrap();
    assert!(!report.has_errors(), "{:?}", report.diagnostics);
    assert!(
        report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "OKMATE5001"
                && diagnostic.layer == DiagnosticLayer::Style
                && diagnostic.severity == Severity::Warning
        }),
        "{:?}",
        report.diagnostics
    );
    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "OKMATE5002"),
        "{:?}",
        report.diagnostics
    );
    assert!(
        report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "OKMATE5003" && diagnostic.path == "okmate.toml"
        }),
        "{:?}",
        report.diagnostics
    );
    assert!(
        report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "OKMATE5003" && diagnostic.message.contains("`adhoc`")
        }),
        "{:?}",
        report.diagnostics
    );

    let engine = okf::check(&root, Profile::Evidence).unwrap();
    assert!(
        !engine
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code.starts_with("OKMATE")),
        "{:?}",
        engine.diagnostics
    );
}

#[test]
fn style_warnings_appear_in_workspace_and_cli_json_without_failing_check() {
    let root = temp_dir("style-cli");
    write_index(&root);
    write_note(&root, "note.md", "Note", "");
    fs::write(
        root.join("okmate.toml"),
        "preferred_types = [\"Runbook\"]\n",
    )
    .unwrap();

    let workspace = okmate::workspace::Workspace::load_single(&root, Profile::Base).unwrap();
    assert!(
        workspace.members()[0]
            .bundle
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "OKMATE5001")
    );
    assert!(!workspace.members()[0].bundle.has_errors());

    let output = std::process::Command::new(okmate_bin())
        .arg("check")
        .arg(&root)
        .arg("--profile")
        .arg("base")
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("OKMATE5001"), "{stdout}");
    assert!(stdout.contains("\"layer\": \"style\""), "{stdout}");
}

#[test]
fn invalid_conventions_file_is_a_style_warning() {
    let root = temp_dir("style-invalid");
    write_index(&root);
    write_note(&root, "note.md", "Note", "");
    fs::write(root.join("okmate.toml"), "preferred_types = [\n").unwrap();

    let report = okmate::check(&root, Profile::Base).unwrap();
    assert!(!report.has_errors(), "{:?}", report.diagnostics);
    assert!(
        report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "OKMATE5003" && diagnostic.layer == DiagnosticLayer::Style
        }),
        "{:?}",
        report.diagnostics
    );
}
