mod common;

use std::fs;
use std::path::Path;
use std::process::Command;

use common::{okmate_bin, temp_dir, valid_strict_concept, write_index};
use okf::Profile;
use okmate::discover::{
    DEFAULT_MAX_DEPTH, DEFAULT_MAX_VISITS, DiscoverOptions, resolve_container, scan,
};
use okmate::workspace::Workspace;

fn opts(start: &Path, max_depth: u32, max_visits: u32) -> DiscoverOptions {
    DiscoverOptions {
        start: start.to_path_buf(),
        max_depth,
        max_visits,
        format: okmate::CheckFormat::Terminal,
    }
}

fn write_versioned(dir: &Path, title: &str) {
    fs::create_dir_all(dir).unwrap();
    fs::write(
        dir.join("index.md"),
        format!("---\nokf_version: \"0.2\"\n---\n\n# {title}\n"),
    )
    .unwrap();
}

fn relatives(report: &okmate::discover::DiscoverReport) -> Vec<&str> {
    report
        .bundles
        .iter()
        .map(|found| found.relative.as_str())
        .collect()
}

#[test]
fn hidden_okf_and_nested_component_roots_are_found() {
    let root = temp_dir("discover-hidden");
    write_versioned(&root.join("docs"), "Docs");
    write_versioned(&root.join("docs").join("kb"), "Nested");
    write_versioned(&root.join("crates").join("gem").join(".okf"), "Gem");
    let report = scan(&opts(&root, DEFAULT_MAX_DEPTH, DEFAULT_MAX_VISITS)).unwrap();
    assert_eq!(relatives(&report), ["crates/gem/.okf", "docs", "docs/kb"]);
}

#[test]
fn zero_one_and_many_candidates() {
    let empty = temp_dir("discover-zero");
    fs::write(empty.join("README.md"), "# Notes\n").unwrap();
    let none = scan(&opts(&empty, DEFAULT_MAX_DEPTH, DEFAULT_MAX_VISITS)).unwrap();
    assert!(none.bundles.is_empty());
    assert!(!none.markerless.is_empty());
    let err = resolve_container(&empty).unwrap_err().to_string();
    assert!(err.contains("no versioned root index"), "{err}");
    assert!(err.contains("markerless"), "{err}");

    let unique = temp_dir("discover-one");
    write_versioned(&unique.join("docs"), "Docs");
    let one = scan(&opts(&unique, DEFAULT_MAX_DEPTH, DEFAULT_MAX_VISITS)).unwrap();
    assert_eq!(relatives(&one), ["docs"]);
    let target = resolve_container(&unique).unwrap();
    assert_eq!(target.root, fs::canonicalize(unique.join("docs")).unwrap());

    let many = temp_dir("discover-many");
    write_versioned(&many.join("docs"), "Docs");
    write_versioned(&many.join("knowledge"), "Knowledge");
    write_versioned(&many.join(".okf"), "Hidden okf");
    let listed = scan(&opts(&many, DEFAULT_MAX_DEPTH, DEFAULT_MAX_VISITS)).unwrap();
    assert_eq!(relatives(&listed), [".okf", "docs", "knowledge"]);
    let err = resolve_container(&many).unwrap_err().to_string();
    assert!(err.contains("ambiguous"), "{err}");
    assert!(err.contains("not preferring knowledge/"), "{err}");
    assert!(err.contains("docs"), "{err}");
    assert!(err.contains("knowledge"), "{err}");
}

#[test]
fn collection_index_is_not_a_bundle() {
    let root = temp_dir("discover-collection");
    fs::write(root.join("index.md"), "# Handbook\n\nQuestions.\n").unwrap();
    write_versioned(&root.join("docs"), "Docs");
    let report = scan(&opts(&root, DEFAULT_MAX_DEPTH, DEFAULT_MAX_VISITS)).unwrap();
    assert_eq!(relatives(&report), ["docs"]);
    assert!(
        report
            .markerless
            .iter()
            .any(|candidate| candidate.relative == "." && candidate.has_index)
    );
    let target = resolve_container(&root).unwrap();
    assert_eq!(target.root, fs::canonicalize(root.join("docs")).unwrap());
}

#[test]
fn scan_limits_and_ignored_trees() {
    let root = temp_dir("discover-limits");
    let mut deep = root.clone();
    for name in ["l1", "l2", "l3"] {
        deep = deep.join(name);
    }
    write_versioned(&deep, "Deep");
    write_versioned(&root.join("node_modules").join("pkg"), "Deps");
    write_versioned(&root.join(".git").join("hooks"), "Git");
    write_versioned(&root.join(".hidden"), "Hidden");
    let shallow = scan(&opts(&root, 1, DEFAULT_MAX_VISITS)).unwrap();
    assert!(
        !relatives(&shallow).iter().any(|path| path.contains("l3")),
        "{:?}",
        relatives(&shallow)
    );
    let ignored = scan(&opts(&root, DEFAULT_MAX_DEPTH, DEFAULT_MAX_VISITS)).unwrap();
    assert_eq!(relatives(&ignored), ["l1/l2/l3"]);
    let truncated = scan(&opts(&root, DEFAULT_MAX_DEPTH, 1)).unwrap();
    assert!(truncated.truncated);
    assert!(truncated.visits <= 1);
}

#[test]
fn explicit_bundle_path_wins_over_siblings() {
    let root = temp_dir("discover-override");
    write_versioned(&root.join("docs"), "Docs");
    write_versioned(&root.join("knowledge"), "Knowledge");
    let nested = root.join("docs").join("components").join("x").join(".okf");
    write_versioned(&nested, "Nested");
    let chosen = resolve_container(&root.join("docs")).unwrap();
    assert_eq!(chosen.root, fs::canonicalize(root.join("docs")).unwrap());
    let scan_docs = scan(&opts(
        &root.join("docs"),
        DEFAULT_MAX_DEPTH,
        DEFAULT_MAX_VISITS,
    ))
    .unwrap();
    assert_eq!(relatives(&scan_docs), [".", "components/x/.okf"]);
}

#[test]
fn separated_nav_keeps_conflicting_reference_meanings() {
    let a = temp_dir("discover-ref-a");
    let b = temp_dir("discover-ref-b");
    write_index(&a);
    write_index(&b);
    for (root, title, body) in [
        (&a, "Library reference", "Meaning in archive A."),
        (&b, "API reference", "Meaning in handbook B."),
    ] {
        fs::create_dir_all(root.join("reference")).unwrap();
        fs::write(
            root.join("reference").join("index.md"),
            format!("# {title}\n\n{body}\n"),
        )
        .unwrap();
        fs::write(
            root.join("reference").join("entry.md"),
            valid_strict_concept(title, "", body),
        )
        .unwrap();
    }
    let workspace =
        Workspace::load_members(vec![("a".into(), a), ("b".into(), b)], Profile::Strict).unwrap();
    let alpha = okmate::site::page_for_route(&workspace, "/@a/reference/").unwrap();
    let beta = okmate::site::page_for_route(&workspace, "/@b/reference/").unwrap();
    assert!(
        alpha.article_html.contains("archive A"),
        "{}",
        alpha.article_html
    );
    assert!(
        !alpha.article_html.contains("handbook B"),
        "{}",
        alpha.article_html
    );
    assert!(
        beta.article_html.contains("handbook B"),
        "{}",
        beta.article_html
    );
    assert!(
        !beta.article_html.contains("archive A"),
        "{}",
        beta.article_html
    );
    let home = okmate::site::page_for_route(&workspace, "/").unwrap();
    assert!(
        nav_has_href(&home.nav, "/@a/reference/") && nav_has_href(&home.nav, "/@b/reference/"),
        "missing separated reference routes"
    );
}

fn nav_has_href(nodes: &[okmate::views::NavNode], href: &str) -> bool {
    nodes
        .iter()
        .any(|node| node.href == href || nav_has_href(&node.children, href))
}

#[test]
fn discover_cli_lists_and_does_not_register() {
    let root = temp_dir("discover-cli");
    write_versioned(&root.join("docs"), "Docs");
    write_versioned(&root.join("knowledge"), "Knowledge");
    let output = Command::new(okmate_bin())
        .arg("discover")
        .arg(&root)
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"relative\": \"docs\""), "{stdout}");
    assert!(stdout.contains("\"relative\": \"knowledge\""), "{stdout}");
    assert!(!root.join("config.toml").exists());
}
