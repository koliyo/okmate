mod common;

use std::fs;
use std::process::Command;

use common::{okmate_bin, temp_dir, valid_strict_concept, write_index};

#[test]
fn build_writes_engine_catalog_html_landmarks_and_pages_json() {
    let root = temp_dir("build-src");
    write_index(&root);
    fs::write(
        root.join("hello.md"),
        valid_strict_concept(
            "Hello",
            "",
            "Intro paragraph.\n\n## Details\n\nMore about the concept.\n",
        ),
    )
    .unwrap();
    fs::write(
        root.join("stale.md"),
        valid_strict_concept("Stale", "stale_after: 2000-01-01\n", "Expired record.\n"),
    )
    .unwrap();
    fs::write(
        root.join("log.md"),
        "# Knowledge log\n\n## 2026-08-20\n\n- Built a sample record.\n",
    )
    .unwrap();
    let output = temp_dir("build-out");

    let status = Command::new(okmate_bin())
        .arg("build")
        .arg(&root)
        .arg("-o")
        .arg(&output)
        .arg("--profile")
        .arg("strict")
        .status()
        .unwrap();
    assert!(status.success());

    assert!(output.join("catalog.json").is_file());
    let catalog: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(output.join("catalog.json")).unwrap()).unwrap();
    assert!(catalog.is_array());
    assert_eq!(catalog.as_array().unwrap().len(), 2);

    let home = fs::read_to_string(output.join("index.html")).unwrap();
    assert!(home.contains("id=\"okmate-nav\""), "{home}");
    assert!(home.contains("id=\"okmate-main\""), "{home}");
    assert!(home.contains("id=\"okmate-toolbar\""), "{home}");
    assert!(home.contains("id=\"okmate-main-width\""), "{home}");
    assert!(home.contains("id=\"okmate-font-larger\""), "{home}");
    assert!(home.contains("id=\"okmate-recents\""), "{home}");
    assert!(home.contains("okmate-recents-list"), "{home}");
    assert!(home.contains("id=\"okmate-log\""), "{home}");
    assert!(home.contains("2026-08-20"), "{home}");
    assert!(home.contains("Built a sample record."), "{home}");
    assert!(!home.contains("okmate-toc-link"), "{home}");
    assert!(!home.contains("Open review queue"), "{home}");
    assert!(home.contains("okmate-nav-attention"), "{home}");
    assert!(home.contains("href=\"/log/\""), "{home}");
    assert!(!home.contains("Knowledge Collections"), "{home}");
    assert!(home.contains("Total"), "{home}");
    let stats_at = home.find("okmate-stat-list").expect("stats");
    let recents_at = home.find("id=\"okmate-recents\"").expect("recents");
    assert!(stats_at < recents_at, "{home}");

    let concept = fs::read_to_string(output.join("hello").join("index.html")).unwrap();
    assert!(concept.contains("id=\"okmate-nav\""), "{concept}");
    assert!(concept.contains("id=\"okmate-main\""), "{concept}");
    assert!(concept.contains("id=\"okmate-toc\""), "{concept}");
    assert!(concept.contains("Details"));
    assert!(concept.contains("okmate-concept-meta"), "{concept}");
    assert!(
        concept.contains("class=\"okmate-outline-menu\""),
        "{concept}"
    );
    assert!(concept.contains("okmate-breadcrumbs"), "{concept}");
    assert!(concept.contains("generated"), "{concept}");
    assert!(concept.contains("Test concept Hello"), "{concept}");
    assert!(concept.contains("Owners"), "{concept}");

    let stale = fs::read_to_string(output.join("stale").join("index.html")).unwrap();
    assert!(stale.contains("okmate-badge-stale"), "{stale}");
    assert!(stale.contains("Review Action Required"), "{stale}");
    assert!(stale.contains("stale_after"), "{stale}");

    let review = fs::read_to_string(output.join("review").join("index.html")).unwrap();
    assert!(review.contains("id=\"okmate-queue\""));
    assert!(review.contains("id=\"okmate-search-input\""));
    assert!(review.contains("id=\"okmate-review-table\""));
    assert!(review.contains("Lifecycle / Authority"));
    assert!(review.contains("Trust &amp; Verification"));
    assert!(review.contains("Required Action"));
    assert!(review.contains("id=\"diagnostics\""));
    assert!(output.join("__okmate").join("review.js").is_file());

    let settings = fs::read_to_string(output.join("settings").join("index.html")).unwrap();
    assert!(settings.contains("id=\"okmate-settings\""));
    assert!(settings.contains("id=\"okmate-toolbar\""));
    assert!(settings.contains("Knowledge roots"));
    assert!(!settings.contains("Maximum line length"));

    assert!(output.join("__okmate").join("app.css").is_file());
    let css = fs::read_to_string(output.join("__okmate").join("app.css")).unwrap();
    assert!(css.contains("--okmate-nav-width"), "{css}");
    assert!(css.contains("--okmate-main-max-width"), "{css}");
    assert!(css.contains("--okmate-ui-font"), "{css}");
    assert!(css.contains("font-size: var(--okmate-ui-font)"), "{css}");
    assert!(
        css.contains("--okmate-toolbar-height: calc(2.65 * 16px)"),
        "{css}"
    );
    assert!(!css.contains("--okmate-toolbar-height: 2.65rem"), "{css}");
    assert!(css.contains("overflow-x: hidden"), "{css}");
    assert!(css.contains("overflow-x: auto"), "{css}");
    assert!(css.contains("overflow-wrap: anywhere"), "{css}");
    assert!(css.contains("data-okmate-wrap"), "{css}");
    assert!(css.contains("pointer-events: none"), "{css}");
    assert!(css.contains("66ch"), "{css}");
    assert!(css.contains(".okmate-col-resizer"), "{css}");
    assert!(css.contains(".okmate-toc-link.is-current"), "{css}");
    assert!(css.contains(".okmate-recents-list"), "{css}");
    assert!(css.contains("flex-direction: column"), "{css}");
    assert!(output.join("__okmate").join("resize.js").is_file());
    assert!(output.join("__okmate").join("reading.js").is_file());
    assert!(output.join("__okmate").join("toc.js").is_file());
    assert!(home.contains("/__okmate/resize.js"), "{home}");
    assert!(home.contains("/__okmate/reading.js"), "{home}");
    assert!(!home.contains("font-size: 110%"), "{home}");
    assert!(!home.contains("data-okmate-wrap=\"off\""), "{home}");
    let reading = fs::read_to_string(output.join("__okmate").join("reading.js")).unwrap();
    assert!(reading.contains("/__okmate/prefs"), "{reading}");
    assert!(reading.contains("localStorage.removeItem"), "{reading}");
    assert!(home.contains("/__okmate/toc.js"), "{home}");
    assert!(home.contains("/__okmate/nav.js"), "{home}");
    let pages: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(output.join("pages.json")).unwrap()).unwrap();
    let routes: Vec<&str> = pages
        .as_array()
        .unwrap()
        .iter()
        .map(|page| page["route"].as_str().unwrap())
        .collect();
    assert!(routes.contains(&"/"));
    assert!(routes.contains(&"/hello/"));
    assert!(routes.contains(&"/review/"));
    assert!(routes.contains(&"/log/"));
    assert!(routes.contains(&"/settings/"));
}
