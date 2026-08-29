mod common;

use std::fs;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use common::{temp_dir, valid_strict_concept, write_index};
use http_body_util::BodyExt;
use okf::Profile;
use tower::ServiceExt;

fn live_state(root: std::path::PathBuf, output: std::path::PathBuf) -> okmate::http::AppState {
    okmate::http::AppState::new(
        output,
        root,
        Profile::Strict,
        std::env::temp_dir().join("okmate-nav-unused.toml"),
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

fn fixture() -> (std::path::PathBuf, std::path::PathBuf) {
    let root = temp_dir("nav-src");
    write_index(&root);
    fs::write(
        root.join("hello.md"),
        valid_strict_concept("Hello", "", "Intro.\n\n## Details\n\nBody.\n"),
    )
    .unwrap();
    let output = temp_dir("nav-out");
    okmate::site::build(&root, &output, Profile::Strict).unwrap();
    (root, output)
}

#[tokio::test]
async fn datastar_get_concept_returns_main_fragment() {
    let (root, output) = fixture();
    let app = app(root, output);
    let response = app
        .oneshot(
            Request::get("/hello/")
                .header("datastar-request", "true")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = body_text(response).await;
    assert!(body.contains("id=\"okmate-main\""), "{body}");
    assert!(body.contains("id=\"okmate-toc\""), "{body}");
    assert!(body.contains("Details"), "{body}");
    assert!(
        !body.to_ascii_lowercase().contains("<html"),
        "patch should not be a full document: {body}"
    );
    assert!(
        !body.contains("id=\"okmate-nav\""),
        "nav should stay in the DOM: {body}"
    );
}

#[tokio::test]
async fn datastar_get_succeeds_when_session_path_is_not_writable() {
    let (root, output) = fixture();
    let mut state = live_state(root, output);
    let blocker = temp_dir("nav-ro-session").join("not-a-dir");
    fs::write(&blocker, "x").unwrap();
    state.session_path = blocker.join("session.json");
    let app = okmate::http::router(state);
    let response = app
        .oneshot(
            Request::get("/hello/")
                .header("datastar-request", "true")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = body_text(response).await;
    assert!(body.contains("id=\"okmate-main\""), "{body}");
    assert!(body.contains("id=\"okmate-toc\""), "{body}");
}

#[tokio::test]
async fn datastar_get_reads_memory_until_workspace_swap() {
    let (root, output) = fixture();
    let state = live_state(root.clone(), output);
    let app = okmate::http::router(state.clone());
    let first = app
        .oneshot(
            Request::get("/hello/")
                .header("datastar-request", "true")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = body_text(first).await;
    assert!(body.contains("Hello"), "{body}");
    assert!(!body.contains("Renamed"), "{body}");

    fs::write(
        root.join("hello.md"),
        valid_strict_concept("Renamed", "", "Updated body.\n"),
    )
    .unwrap();
    let stale = okmate::http::router(state.clone())
        .oneshot(
            Request::get("/hello/")
                .header("datastar-request", "true")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = body_text(stale).await;
    assert!(body.contains("Hello"), "{body}");
    assert!(!body.contains("Renamed"), "{body}");

    let reloaded = okmate::workspace::Workspace::load_single(&root, Profile::Strict).unwrap();
    state.replace_workspace(reloaded);
    let fresh = okmate::http::router(state)
        .oneshot(
            Request::get("/hello/")
                .header("datastar-request", "true")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = body_text(fresh).await;
    assert!(body.contains("Renamed"), "{body}");
    assert!(body.contains("id=\"okmate-main\""), "{body}");
    assert!(!body.contains("id=\"okmate-nav\""), "{body}");
}

#[tokio::test]
async fn review_page_contains_queue_region() {
    let (root, output) = fixture();
    let app = app(root, output);
    let response = app
        .oneshot(Request::get("/review/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = body_text(response).await;
    assert!(body.contains("id=\"okmate-queue\""), "{body}");
    assert!(body.contains("Hello"), "{body}");
    assert!(body.contains("id=\"okmate-search-input\""), "{body}");
    assert!(body.contains("Lifecycle / Authority"), "{body}");
    assert!(body.contains("id=\"diagnostics\""), "{body}");
    assert!(body.contains("data-search="), "{body}");
}

#[tokio::test]
async fn collection_nav_uses_section_overview_and_span_summary() {
    let root = temp_dir("nav-plans-src");
    write_index(&root);
    fs::create_dir_all(root.join("plans")).unwrap();
    fs::write(root.join("plans").join("index.md"), "# Plans\n").unwrap();
    fs::write(
        root.join("plans").join("nested.md"),
        valid_strict_concept("Nested", "", "Body.\n"),
    )
    .unwrap();
    let output = temp_dir("nav-plans-out");
    okmate::site::build(&root, &output, Profile::Strict).unwrap();
    let html = fs::read_to_string(output.join("plans").join("nested").join("index.html")).unwrap();
    assert!(html.contains("data-okmate-nav-section=\"plans\""), "{html}");
    assert!(html.contains("href=\"/plans/\""), "{html}");
    assert!(html.contains(">Overview<"), "{html}");
    assert!(html.contains("class=\"okmate-nav-menu\""), "{html}");
    assert!(
        html.contains("<span class=\"nav-link nav-category\">"),
        "{html}"
    );
    assert!(
        !html.contains("nav-category\" href="),
        "category summary must not be a @get link: {html}"
    );
    assert!(
        !html.contains("nav-category\" data-on:click"),
        "category summary must not be a @get link: {html}"
    );
    assert!(html.contains("okmate-breadcrumbs"), "{html}");
    assert!(html.contains("href=\"/plans/\""), "{html}");
    assert!(html.contains(">Plans<"), "{html}");
}

#[test]
fn full_documents_differ_in_nav_current_markers() {
    let root = temp_dir("nav-current-src");
    write_index(&root);
    fs::create_dir_all(root.join("plans")).unwrap();
    fs::write(root.join("plans").join("index.md"), "# Plans\n").unwrap();
    fs::write(
        root.join("plans").join("nested.md"),
        valid_strict_concept("Nested", "", "Body.\n"),
    )
    .unwrap();
    let output = temp_dir("nav-current-out");
    okmate::site::build(&root, &output, Profile::Strict).unwrap();
    let home = fs::read_to_string(output.join("index.html")).unwrap();
    let nested =
        fs::read_to_string(output.join("plans").join("nested").join("index.html")).unwrap();
    assert!(
        home.contains("class=\"nav-link is-current\" href=\"/\""),
        "{home}"
    );
    assert!(
        !home.contains("data-okmate-nav-current"),
        "dashboard is not inside a collection: {home}"
    );
    assert!(
        nested
            .contains("class=\"nav-link is-current\" data-nav-depth=\"1\" href=\"/plans/nested/\""),
        "{nested}"
    );
    assert!(nested.contains("data-okmate-nav-current"), "{nested}");
    assert!(
        !nested.contains("class=\"nav-link is-current\" href=\"/\""),
        "{nested}"
    );
}

#[test]
fn nav_uses_fixed_icons_and_type_dots() {
    let root = temp_dir("nav-chrome-src");
    write_index(&root);
    fs::create_dir_all(root.join("plans")).unwrap();
    fs::write(root.join("plans").join("index.md"), "# Plans\n").unwrap();
    fs::write(
        root.join("plans").join("nested.md"),
        valid_strict_concept("Nested", "", "Body.\n"),
    )
    .unwrap();
    let output = temp_dir("nav-chrome-out");
    okmate::site::build(&root, &output, Profile::Strict).unwrap();
    let home = fs::read_to_string(output.join("index.html")).unwrap();
    assert!(home.contains("okmate-nav-fixed"), "{home}");
    assert!(home.contains("okmate-nav-tree"), "{home}");
    assert!(home.contains("okmate-nav-icon"), "{home}");
    assert!(home.contains("href=\"/\""), "{home}");
    assert!(home.contains("href=\"/review/\""), "{home}");
    assert!(home.contains("href=\"/log/\""), "{home}");
    assert!(home.contains("href=\"/settings/\""), "{home}");
    let architecture = okmate::views::type_color("Architecture");
    assert!(
        home.contains(&format!(
            "class=\"okmate-type-dot\" style=\"background: {architecture}\""
        )),
        "{home}"
    );
    assert!(
        home.contains("data-nav-depth=\"1\""),
        "tree rows must carry depth: {home}"
    );
    assert!(
        !home.contains(&format!(
            "class=\"okmate-type-dot\" style=\"background: {}\"",
            okmate::views::type_color("Index")
        )),
        "overview leaves must not use the Index type dot: {home}"
    );
}
