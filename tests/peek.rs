mod common;

use std::fs;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use common::{temp_dir, valid_strict_concept, write_index};
use http_body_util::BodyExt;
use okf::Profile;
use okmate::peek::{PeekKind, peek};
use tower::ServiceExt;

fn live_state(root: std::path::PathBuf, output: std::path::PathBuf) -> okmate::http::AppState {
    okmate::http::AppState::new(
        output,
        root,
        Profile::Strict,
        std::env::temp_dir().join("okmate-peek-unused.toml"),
    )
}

fn app(root: std::path::PathBuf, output: std::path::PathBuf) -> axum::Router {
    okmate::http::router(live_state(root, output))
}

async fn body_text(response: axum::http::Response<Body>) -> String {
    String::from_utf8(
        response
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes()
            .to_vec(),
    )
    .unwrap()
}

fn hello_fixture() -> (std::path::PathBuf, okmate::workspace::Workspace) {
    let root = temp_dir("peek-hello");
    write_index(&root);
    fs::write(
        root.join("hello.md"),
        valid_strict_concept(
            "Hello",
            "",
            "Intro paragraph that must not leak into the Details peek.\n\n## Details\n\nDetails body for the heading peek.\n",
        ),
    )
    .unwrap();
    let workspace = okmate::workspace::Workspace::load_single(&root, Profile::Strict).unwrap();
    (root, workspace)
}

fn citation_fixture() -> (std::path::PathBuf, okmate::workspace::Workspace) {
    let root = temp_dir("peek-cite");
    write_index(&root);
    fs::write(
        root.join("register.md"),
        valid_strict_concept(
            "Register",
            "",
            "Register lead that must not appear in the S21 peek.\n\n### S21\n\nSteinberger register context for the heading peek.\n",
        ),
    )
    .unwrap();
    fs::write(
        root.join("report.md"),
        valid_strict_concept(
            "Report",
            "sources:\n  - id: s21\n    resource: https://example.com/steinberger\n    title: Steinberger on citations\n    author: human:steinberger\n",
            "The report cites Steinberger.[^s21]\n\n[^s21]: Steinberger, 2021. [Research context](register.md#s21).\n",
        ),
    )
    .unwrap();
    let workspace = okmate::workspace::Workspace::load_single(&root, Profile::Strict).unwrap();
    (root, workspace)
}

#[test]
fn hello_heading_peek_uses_details_not_intro() {
    let (_root, workspace) = hello_fixture();
    let lead = peek(&workspace, "/hello/", "").expect("lead");
    assert_eq!(lead.kind, PeekKind::Document);
    assert!(lead.excerpt.contains("Intro paragraph"), "{lead:?}");
    assert!(!lead.excerpt.contains("Details body"), "{lead:?}");

    let details = peek(&workspace, "/hello/", "details").expect("details");
    assert_eq!(details.kind, PeekKind::Heading);
    assert_eq!(details.title, "Details");
    assert!(
        details
            .excerpt
            .contains("Details body for the heading peek"),
        "{details:?}"
    );
    assert!(!details.excerpt.contains("Intro paragraph"), "{details:?}");
    assert_eq!(lead.type_color, okmate::views::type_color("Architecture"));
    let chrome = peek(&workspace, "/review/", "").expect("chrome");
    assert!(chrome.type_color.is_empty(), "{chrome:?}");
    assert_eq!(chrome.document_title, "Review queue");
    assert_eq!(
        peek(&workspace, "/", "").expect("home").document_title,
        "Dashboard"
    );
    assert!(
        !serde_json::to_string(&chrome)
            .unwrap()
            .contains("type_color")
    );
}

#[test]
fn citation_chain_peeks_footnote_and_register_heading() {
    let (_root, workspace) = citation_fixture();
    let report = workspace
        .primary()
        .unwrap()
        .bundle
        .concepts
        .iter()
        .find(|concept| concept.id == "report")
        .unwrap();
    assert!(
        report.article_html.contains("id=\"fn-s21\""),
        "{}",
        report.article_html
    );

    let footnote = peek(&workspace, "/report/", "fn-s21").expect("footnote");
    assert_eq!(footnote.kind, PeekKind::Footnote);
    assert_eq!(footnote.title, "Steinberger on citations");
    assert!(
        !footnote.excerpt.contains("The report cites"),
        "{footnote:?}"
    );
    assert!(
        footnote.excerpt.contains("Steinberger, 2021")
            || footnote.excerpt.contains("Research context"),
        "{footnote:?}"
    );
    assert!(
        footnote.excerpt.contains("Steinberger register context")
            || footnote.excerpt.contains("https://example.com/steinberger"),
        "{footnote:?}"
    );

    let heading = peek(&workspace, "/register/", "s21").expect("register heading");
    assert_eq!(heading.kind, PeekKind::Heading);
    assert_eq!(heading.title, "S21");
    assert!(
        heading.excerpt.contains("Steinberger register context"),
        "{heading:?}"
    );
    assert!(!heading.excerpt.contains("Register lead"), "{heading:?}");
}

#[tokio::test]
async fn peek_http_returns_heading_and_footnote_json() {
    let (root, workspace) = citation_fixture();
    let output = temp_dir("peek-out");
    okmate::site::build_workspace(&workspace, &output).unwrap();
    let app = app(root, output);

    let details_src = hello_fixture();
    drop(details_src);

    let heading = app
        .clone()
        .oneshot(
            Request::get("/__okmate/peek?path=/register/&hash=s21")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(heading.status(), StatusCode::OK);
    let heading_body = body_text(heading).await;
    assert!(
        heading_body.contains("\"kind\":\"heading\""),
        "{heading_body}"
    );
    assert!(
        heading_body.contains("Steinberger register context"),
        "{heading_body}"
    );
    assert!(!heading_body.contains("Register lead"), "{heading_body}");

    let footnote = app
        .clone()
        .oneshot(
            Request::get("/__okmate/peek?path=/report/&hash=fn-s21")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(footnote.status(), StatusCode::OK);
    let footnote_body = body_text(footnote).await;
    assert!(
        footnote_body.contains("\"kind\":\"footnote\""),
        "{footnote_body}"
    );
    assert!(
        footnote_body.contains("Steinberger on citations"),
        "{footnote_body}"
    );
    assert!(
        !footnote_body.contains("The report cites"),
        "{footnote_body}"
    );

    let missing = app
        .clone()
        .oneshot(
            Request::get("/__okmate/peek?path=/missing/&hash=")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(missing.status(), StatusCode::NOT_FOUND);

    let script = app
        .oneshot(
            Request::get("/__okmate/peek.js")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(script.status(), StatusCode::OK);
}

#[tokio::test]
async fn peek_http_hello_details_is_section_not_intro() {
    let (root, workspace) = hello_fixture();
    let output = temp_dir("peek-hello-out");
    okmate::site::build_workspace(&workspace, &output).unwrap();
    let response = app(root, output)
        .oneshot(
            Request::get("/__okmate/peek?path=/hello/&hash=details")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = body_text(response).await;
    assert!(body.contains("Details body"), "{body}");
    assert!(!body.contains("Intro paragraph"), "{body}");
}

#[test]
fn peek_script_classifies_comrak_footnote_refs() {
    let js = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/peek.js"));
    assert!(js.contains("fnref-"), "{js}");
    assert!(js.contains("indexOf(\"fn-\")"), "{js}");
    assert!(js.contains(r#"removeAttribute("title")"#), "{js}");
    assert!(js.contains(r#"setAttribute("role", "tooltip")"#), "{js}");
}

#[test]
fn citation_article_html_has_footnote_ref_hrefs() {
    let (_root, workspace) = citation_fixture();
    let report = workspace
        .primary()
        .unwrap()
        .bundle
        .concepts
        .iter()
        .find(|concept| concept.id == "report")
        .unwrap();
    assert!(
        report.article_html.contains("href=\"#fn-s21\""),
        "{}",
        report.article_html
    );
    assert!(
        report.article_html.contains("fnref-s21") || report.article_html.contains("footnote-ref"),
        "{}",
        report.article_html
    );
}

#[tokio::test]
async fn live_page_includes_peek_script() {
    let (root, workspace) = hello_fixture();
    let output = temp_dir("peek-live-out");
    okmate::site::build_workspace(&workspace, &output).unwrap();
    assert!(output.join("__okmate").join("peek.js").is_file());
    let page = app(root, output)
        .oneshot(Request::get("/hello/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(page.status(), StatusCode::OK);
    let body = body_text(page).await;
    assert!(body.contains("/__okmate/peek.js"), "{body}");
}
