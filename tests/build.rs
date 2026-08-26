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
    assert!(home.contains("id=\"okmate-recents\""), "{home}");
    assert!(home.contains("Open review queue"), "{home}");
    assert!(home.contains("Knowledge Collections"), "{home}");
    assert!(home.contains("Total"), "{home}");

    let concept = fs::read_to_string(output.join("hello").join("index.html")).unwrap();
    assert!(concept.contains("id=\"okmate-nav\""), "{concept}");
    assert!(concept.contains("id=\"okmate-main\""), "{concept}");
    assert!(concept.contains("id=\"okmate-toc\""), "{concept}");
    assert!(concept.contains("Details"));
    assert!(concept.contains("okmate-concept-meta"), "{concept}");
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

    assert!(output.join("__okmate").join("app.css").is_file());
    let css = fs::read_to_string(output.join("__okmate").join("app.css")).unwrap();
    assert!(css.contains("--okmate-nav-width"), "{css}");
    assert!(css.contains(".okmate-col-resizer"), "{css}");
    assert!(css.contains(".okmate-toc-link.is-current"), "{css}");
    assert!(output.join("__okmate").join("resize.js").is_file());
    assert!(output.join("__okmate").join("toc.js").is_file());
    assert!(home.contains("/__okmate/resize.js"), "{home}");
    assert!(home.contains("/__okmate/toc.js"), "{home}");
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
    assert!(routes.contains(&"/settings/"));
}
