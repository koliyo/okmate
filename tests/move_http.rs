mod common;

use std::fs;
use std::net::SocketAddr;

use axum::body::Body;
use axum::extract::connect_info::MockConnectInfo;
use axum::http::{Request, StatusCode};
use common::{temp_dir, write_index};
use http_body_util::BodyExt;
use okf::Profile;
use tower::ServiceExt;

fn app(root: std::path::PathBuf, output: std::path::PathBuf, peer: [u8; 4]) -> axum::Router {
    let config = temp_dir("move-http-cfg").join("config.toml");
    okmate::http::router(okmate::http::AppState::new(
        output,
        root,
        Profile::Base,
        config,
    ))
    .layer(MockConnectInfo(SocketAddr::from((peer, 40000))))
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
    let root = temp_dir("move-http-src");
    write_index(&root);
    fs::create_dir_all(root.join("research")).unwrap();
    fs::create_dir_all(root.join("audits")).unwrap();
    fs::write(
        root.join("research/index.md"),
        "# Research\n\n* [Topic](topic.md) - Topic.\n",
    )
    .unwrap();
    fs::write(root.join("audits/index.md"), "# Audits\n").unwrap();
    fs::write(
        root.join("research/topic.md"),
        "---\ntype: Explanation\ntitle: Topic\ndescription: Topic record.\n---\n\n# Topic\n\nBody.\n",
    )
    .unwrap();
    let output = temp_dir("move-http-out");
    okmate::site::build(&root, &output, Profile::Base).unwrap();
    (root, output)
}

#[tokio::test]
async fn move_get_is_loopback_only() {
    let (root, output) = fixture();
    let app = app(root, output, [10, 0, 0, 1]);
    let response = app
        .oneshot(
            Request::get("/__okmate/move?from=research/topic&to=audits/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn move_apply_then_reload_sees_new_id() {
    let (root, output) = fixture();
    let app = app(root.clone(), output, [127, 0, 0, 1]);
    let response = app
        .clone()
        .oneshot(
            Request::post("/__okmate/move")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"from":"research/topic","to":"audits/","apply":true}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = body_text(response).await;
    assert!(
        body.contains("\"to\":\"audits/topic\"") || body.contains("\"to\": \"audits/topic\""),
        "{body}"
    );
    assert!(body.contains("/audits/topic/"), "{body}");
    assert!(!root.join("research/topic.md").exists());
    assert!(root.join("audits/topic.md").exists());

    let page = body_text(
        app.oneshot(Request::get("/audits/topic/").body(Body::empty()).unwrap())
            .await
            .unwrap(),
    )
    .await;
    assert!(page.contains("Topic"), "{page}");
    assert!(page.contains("data-okmate-live"), "{page}");
}

#[tokio::test]
async fn static_build_is_not_live() {
    let (root, output) = fixture();
    let home = fs::read_to_string(output.join("index.html")).unwrap();
    assert!(!home.contains("data-okmate-live"), "{home}");
    assert!(output.join("__okmate").join("move.js").is_file());
    let _ = root;
}
