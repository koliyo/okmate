mod common;

use std::fs;
use std::net::SocketAddr;

use axum::body::Body;
use axum::extract::connect_info::MockConnectInfo;
use axum::http::{Request, StatusCode};
use common::{temp_dir, valid_strict_concept, write_index};
use http_body_util::BodyExt;
use okf::Profile;
use tower::ServiceExt;

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

fn fixture() -> (
    std::path::PathBuf,
    std::path::PathBuf,
    okmate::workspace::Workspace,
) {
    let root = temp_dir("prefs-src");
    write_index(&root);
    fs::write(
        root.join("hello.md"),
        valid_strict_concept("Hello", "", "Body.\n"),
    )
    .unwrap();
    let workspace = okmate::workspace::Workspace::load_single(&root, Profile::Strict).unwrap();
    let output = temp_dir("prefs-out");
    okmate::site::build_workspace(&workspace, &output).unwrap();
    (root, output, workspace)
}

fn state(
    root: std::path::PathBuf,
    output: std::path::PathBuf,
    workspace: okmate::workspace::Workspace,
    session: std::path::PathBuf,
) -> okmate::http::AppState {
    okmate::http::AppState {
        output,
        root,
        workspace: okmate::http::share_workspace(workspace),
        profile: Profile::Strict,
        config_path: temp_dir("prefs-cfg").join("config.toml"),
        session_path: session,
    }
}

#[test]
fn build_html_does_not_bake_session_prefs() {
    let root = temp_dir("prefs-build-src");
    write_index(&root);
    fs::write(
        root.join("hello.md"),
        valid_strict_concept("Hello", "", "Body.\n"),
    )
    .unwrap();
    let session = temp_dir("prefs-build-session").join("session.json");
    okmate::preview::persist_prefs_to(
        &session,
        &serde_json::json!({
            "font_size": 110,
            "main_width": 70,
            "wrap": false
        }),
    );
    let workspace = okmate::workspace::Workspace::load_single(&root, Profile::Strict).unwrap();
    let output = temp_dir("prefs-build-out");
    okmate::site::write_html_pages(&workspace, &output, okmate::preview::NavMode::Separated)
        .unwrap();
    let home = fs::read_to_string(output.join("index.html")).unwrap();
    assert!(!home.contains("font-size: 110%"), "{home}");
    assert!(!home.contains("--okmate-main-max-width: 70ch"), "{home}");
    assert!(!home.contains("data-okmate-wrap=\"off\""), "{home}");
    assert!(home.contains("value=\"66\""), "{home}");
}

#[tokio::test]
async fn live_html_seeds_reading_prefs() {
    let (root, output, workspace) = fixture();
    let session = temp_dir("prefs-live-session").join("session.json");
    okmate::preview::persist_prefs_to(
        &session,
        &serde_json::json!({
            "font_size": 110,
            "main_width": 70,
            "wrap": false,
            "nav_visible": false,
            "nav_width": "264px"
        }),
    );
    let app = okmate::http::router(state(root, output, workspace, session));
    let html = body_text(
        app.oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap(),
    )
    .await;
    assert!(html.contains("font-size: 110%"), "{html}");
    assert!(html.contains("--okmate-main-max-width: 70ch"), "{html}");
    assert!(html.contains("--okmate-nav-width: 264px"), "{html}");
    assert!(html.contains("data-okmate-wrap=\"off\""), "{html}");
    assert!(html.contains("data-okmate-nav=\"off\""), "{html}");
    assert!(html.contains("value=\"70\""), "{html}");
    assert!(html.contains(">110%</button>"), "{html}");
}

#[tokio::test]
async fn live_html_seeds_open_document_scroll() {
    let (root, output, workspace) = fixture();
    let session = temp_dir("prefs-location-session").join("session.json");
    okmate::preview::persist_prefs_to(
        &session,
        &serde_json::json!({
            "open_path": "/",
            "open_hash": "home",
            "main_scroll": 180
        }),
    );
    let app = okmate::http::router(state(root, output, workspace, session.clone()));
    let html = body_text(
        app.oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap(),
    )
    .await;
    assert!(html.contains("data-okmate-main-scroll=\"180\""), "{html}");
    let stored = okmate::preview::load_session_from(&session);
    assert_eq!(stored.open_path.as_deref(), Some("/"));
    assert_eq!(stored.open_hash.as_deref(), Some("home"));
}

#[tokio::test]
async fn live_html_seeds_nav_sections_from_session() {
    let root = temp_dir("prefs-nav-src");
    write_index(&root);
    fs::create_dir_all(root.join("plans")).unwrap();
    fs::write(root.join("plans").join("index.md"), "# Plans\n").unwrap();
    fs::write(
        root.join("plans").join("nested.md"),
        valid_strict_concept("Nested", "", "Body.\n"),
    )
    .unwrap();
    let workspace = okmate::workspace::Workspace::load_single(&root, Profile::Strict).unwrap();
    let output = temp_dir("prefs-nav-out");
    okmate::site::build_workspace(&workspace, &output).unwrap();
    let session = temp_dir("prefs-nav-session").join("session.json");
    okmate::preview::persist_prefs_to(
        &session,
        &serde_json::json!({
            "nav_sections": { "plans": true },
            "nav_scroll": 42
        }),
    );
    let app = okmate::http::router(state(root, output, workspace, session));
    let html = body_text(
        app.oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap(),
    )
    .await;
    assert!(
        html.contains("data-okmate-nav-section=\"plans\" open"),
        "{html}"
    );
    assert!(html.contains("data-okmate-nav-scroll=\"42\""), "{html}");
}

#[tokio::test]
async fn prefs_post_merges_and_clamps() {
    let (root, output, workspace) = fixture();
    let session = temp_dir("prefs-post-session").join("session.json");
    let app = okmate::http::router(state(root, output, workspace, session.clone()))
        .layer(MockConnectInfo(SocketAddr::from(([127, 0, 0, 1], 40000))));
    let response = app
        .oneshot(
            Request::post("/__okmate/prefs")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"font_size":113,"main_width":70,"nav_width":"12.5rem","outline_width":"url(evil)"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    let stored = okmate::preview::load_session_from(&session);
    assert_eq!(stored.font_size, 110);
    assert_eq!(stored.main_width, Some(70));
    assert_eq!(stored.nav_width.as_deref(), Some("12.5rem"));
    assert_eq!(stored.outline_width, None);
}

#[tokio::test]
async fn prefs_post_is_loopback_only() {
    let (root, output, workspace) = fixture();
    let session = temp_dir("prefs-deny-session").join("session.json");
    let app = okmate::http::router(state(root, output, workspace, session.clone()))
        .layer(MockConnectInfo(SocketAddr::from(([8, 8, 8, 8], 40000))));
    let response = app
        .oneshot(
            Request::post("/__okmate/prefs")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"font_size":160}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    assert!(!session.exists());
}
