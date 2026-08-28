use std::fs;
use std::path::Path;

use okmate::views::{ConceptMeta, Document, NavNode, ReviewRow, TocEntry};

fn golden(name: &str) -> String {
    fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/golden")
            .join(name),
    )
    .unwrap()
}

fn assert_contains_golden(html: &str, fixture: &str) {
    for line in golden(fixture).lines() {
        let needle = line.trim();
        if needle.is_empty() {
            continue;
        }
        assert!(
            html.contains(needle),
            "missing {needle} from {fixture}: {html}"
        );
    }
}

fn sample() -> Document {
    Document {
        title: "Hello".into(),
        page_kind: "page".into(),
        nav: vec![NavNode {
            href: "/".into(),
            title: "Dashboard".into(),
            current: true,
            open: false,
            children: Vec::new(),
            section_key: String::new(),
            root: String::new(),
            summary: String::new(),
            attention: false,
        }],
        toc: vec![TocEntry {
            id: "section".into(),
            text: "Section".into(),
            level: 2,
        }],
        article_html: "<h1>Hello</h1>".into(),
        concept_type: "Architecture".into(),
        status: "draft".into(),
        authority: "descriptive".into(),
        review_rows: vec![ReviewRow {
            href: "/hello/".into(),
            title: "Hello".into(),
            id: "hello".into(),
            status: "draft".into(),
            action: "Clean".into(),
            ..ReviewRow::default()
        }],
        action_rows: Vec::new(),
        stats: Vec::new(),
        recents: Vec::new(),
        log_days: Vec::new(),
        show_root: false,
        nav_mode: "separated".into(),
        show_nav_mode: false,
        crumbs: Vec::new(),
        diagnostics: Vec::new(),
        meta: ConceptMeta::default(),
        message: String::new(),
        config_path: "~/.okmate/config.toml".into(),
        actor: String::new(),
        settings_roots: Vec::new(),
        review_window: okmate::views::ListWindow::default(),
        log_window: okmate::views::ListWindow::default(),
        html_style: String::new(),
        reading_wrap: true,
        reading_full: false,
        reading_nav: true,
        reading_toc: true,
        reading_font: 100,
        reading_width: 66,
        main_scroll: 0,
    }
}

#[test]
fn shell_landmarks_match_golden() {
    let html = sample().render_page().unwrap();
    assert_contains_golden(&html, "shell-landmarks.txt");
}

#[test]
fn settings_patch_matches_golden() {
    let mut document = sample();
    document.page_kind = "settings".into();
    let html = document.render_settings_fragment().unwrap();
    assert_contains_golden(&html, "settings-patch.txt");
    assert!(!html.to_ascii_lowercase().contains("<html"));
}

#[test]
fn queue_region_matches_golden() {
    let mut document = sample();
    document.page_kind = "review".into();
    let html = document.render_queue_fragment().unwrap();
    assert_contains_golden(&html, "queue-region.txt");
    assert!(!html.to_ascii_lowercase().contains("<html"));
}
