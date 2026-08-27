mod common;

use std::fs;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use common::{temp_dir, valid_strict_concept, write_index};
use http_body_util::BodyExt;
use okf::Profile;
use tower::ServiceExt;

fn app(root: std::path::PathBuf, output: std::path::PathBuf) -> axum::Router {
    okmate::http::router(okmate::http::AppState::new(
        output,
        root,
        Profile::Strict,
        std::env::temp_dir().join("okmate-window-unused.toml"),
    ))
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

fn large_review_fixture() -> (std::path::PathBuf, std::path::PathBuf) {
    let root = temp_dir("window-src");
    write_index(&root);
    for i in 0..90 {
        fs::write(
            root.join(format!("item-{i:02}.md")),
            valid_strict_concept(&format!("Item {i:02}"), "", "Body.\n"),
        )
        .unwrap();
    }
    fs::write(
        root.join("omega.md"),
        valid_strict_concept("Omega Last", "", "Last concept.\n"),
    )
    .unwrap();
    fs::write(
        root.join("stable.md"),
        "---\ntype: Architecture\ntitle: Stable Only\ndescription: Test concept Stable Only.\ntags: [domain/okf, concern/architecture]\nstatus: stable\ngenerated: { by: process:test, at: 2026-08-17T00:00:00Z }\nauthority: descriptive\nowners: [human:nils]\n---\n\n# Stable Only\n\nStable body.\n",
    )
    .unwrap();
    let output = temp_dir("window-out");
    let workspace = okmate::workspace::Workspace::load_single(&root, Profile::Strict).unwrap();
    okmate::site::build_workspace(&workspace, &output).unwrap();
    (root, output)
}

fn log_fixture() -> (std::path::PathBuf, std::path::PathBuf) {
    let root = temp_dir("window-log-src");
    write_index(&root);
    let mut body = String::from("# Knowledge log\n\n");
    for day in 0..10 {
        body.push_str(&format!("## 2026-08-{:02}\n\n", 27 - day));
        for n in 0..5 {
            body.push_str(&format!("- Day {day} bullet {n}.\n"));
        }
        body.push('\n');
    }
    body.push_str("## 2020-01-01\n\n- Ancient last.\n");
    fs::write(root.join("log.md"), body).unwrap();
    let output = temp_dir("window-log-out");
    let workspace = okmate::workspace::Workspace::load_single(&root, Profile::Strict).unwrap();
    okmate::site::build_workspace(&workspace, &output).unwrap();
    (root, output)
}

#[tokio::test]
async fn review_first_window_omits_last_concept() {
    let (root, output) = large_review_fixture();
    let app = app(root, output);
    let response = app
        .oneshot(
            Request::get("/review/")
                .header("datastar-request", "true")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = body_text(response).await;
    assert!(body.contains("Item 00"), "{body}");
    assert!(body.contains("data-okmate-sentinel"), "{body}");
    let window = body
        .split("id=\"okmate-review-window\"")
        .nth(1)
        .expect("review window");
    assert!(
        !window.contains("Omega Last"),
        "first window must not include the last concept: {window}"
    );
}

#[tokio::test]
async fn review_adjacent_window_returns_later_titles() {
    let (root, output) = large_review_fixture();
    let app = app(root, output);
    let response = app
        .oneshot(
            Request::get("/__okmate/review-window?start=40")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = body_text(response).await;
    assert!(
        body.contains("Item 40") || body.contains("Item 45"),
        "{body}"
    );
}

#[tokio::test]
async fn review_draft_filter_omits_stable_row() {
    let (root, output) = large_review_fixture();
    let app = app(root, output);
    let response = app
        .oneshot(
            Request::get("/__okmate/review-window?filter=draft")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = body_text(response).await;
    assert!(!body.contains("Stable Only"), "{body}");
}

#[tokio::test]
async fn log_first_window_omits_oldest_last_bullet() {
    let (root, output) = log_fixture();
    let app = app(root, output);
    let response = app
        .oneshot(
            Request::get("/log/")
                .header("datastar-request", "true")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = body_text(response).await;
    assert!(body.contains("Day 0 bullet 0"), "{body}");
    assert!(
        !body.contains("Ancient last."),
        "first log window must omit the oldest last bullet: {body}"
    );
}
