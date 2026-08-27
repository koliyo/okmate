mod common;

use std::fs;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use common::{temp_dir, valid_strict_concept, write_index};
use http_body_util::BodyExt;
use okf::Profile;
use okmate::workspace::Workspace;
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

fn write_workspace_bundle(root: &std::path::Path, shared_title: &str, unique: bool) {
    write_index(root);
    fs::create_dir_all(root.join("plans")).unwrap();
    fs::write(root.join("plans").join("index.md"), "# Plans\n").unwrap();
    fs::write(
        root.join("plans").join("shared.md"),
        valid_strict_concept(shared_title, "", "Shared body.\n"),
    )
    .unwrap();
    if unique {
        fs::write(
            root.join("unique.md"),
            valid_strict_concept("Unique", "", "Only in a.\n"),
        )
        .unwrap();
    }
}

fn two_bundles() -> (
    std::path::PathBuf,
    std::path::PathBuf,
    Workspace,
    std::path::PathBuf,
) {
    let a = temp_dir("ws-a");
    let b = temp_dir("ws-b");
    write_workspace_bundle(&a, "Alpha Shared", true);
    write_workspace_bundle(&b, "Beta Shared", false);
    let workspace = Workspace::load_members(
        vec![("a".into(), a.clone()), ("b".into(), b.clone())],
        Profile::Strict,
    )
    .unwrap();
    let output = temp_dir("ws-out");
    okmate::site::build_workspace(&workspace, &output).unwrap();
    (a, b, workspace, output)
}

fn app(root: std::path::PathBuf, output: std::path::PathBuf, workspace: Workspace) -> axum::Router {
    okmate::http::router(okmate::http::AppState {
        output,
        root,
        workspace,
        profile: Profile::Strict,
        config_path: std::env::temp_dir().join("okmate-ws-unused.toml"),
    })
}

#[tokio::test]
async fn workspace_get_keeps_colliding_ids_on_prefixed_routes() {
    let (a, _b, workspace, output) = two_bundles();
    let app = app(a, output.clone(), workspace);

    let alpha = app
        .clone()
        .oneshot(
            Request::get("/@a/plans/shared/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(alpha.status(), StatusCode::OK);
    let alpha_body = body_text(alpha).await;
    assert!(alpha_body.contains("Alpha Shared"), "{alpha_body}");
    assert!(!alpha_body.contains("Beta Shared"), "{alpha_body}");

    let beta = app
        .clone()
        .oneshot(
            Request::get("/@b/plans/shared/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(beta.status(), StatusCode::OK);
    let beta_body = body_text(beta).await;
    assert!(beta_body.contains("Beta Shared"), "{beta_body}");
    assert!(!beta_body.contains("Alpha Shared"), "{beta_body}");

    let unique = app
        .oneshot(Request::get("/@a/unique/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(unique.status(), StatusCode::OK);
    let unique_body = body_text(unique).await;
    assert!(unique_body.contains("Unique"), "{unique_body}");

    let pages: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(output.join("pages.json")).unwrap()).unwrap();
    let shared: Vec<&str> = pages
        .as_array()
        .unwrap()
        .iter()
        .filter(|page| page["path"] == "plans/shared.md")
        .map(|page| page["root"].as_str().unwrap())
        .collect();
    assert!(shared.contains(&"a"), "{pages}");
    assert!(shared.contains(&"b"), "{pages}");
}

#[tokio::test]
async fn workspace_datastar_fragment_omits_nav() {
    let (a, _b, workspace, output) = two_bundles();
    let app = app(a, output, workspace);
    let response = app
        .oneshot(
            Request::get("/@a/plans/shared/")
                .header("datastar-request", "true")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = body_text(response).await;
    assert!(body.contains("id=\"okmate-main\""), "{body}");
    assert!(body.contains("Alpha Shared"), "{body}");
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
async fn single_root_unprefixed_routes_still_work() {
    let root = temp_dir("ws-single");
    write_workspace_bundle(&root, "Solo Shared", false);
    let output = temp_dir("ws-single-out");
    okmate::site::build(&root, &output, Profile::Strict).unwrap();
    let app = okmate::http::router(okmate::http::AppState::new(
        output,
        root,
        Profile::Strict,
        std::env::temp_dir().join("okmate-ws-single.toml"),
    ));
    let response = app
        .oneshot(Request::get("/plans/shared/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = body_text(response).await;
    assert!(body.contains("Solo Shared"), "{body}");
    assert!(body.contains("id=\"okmate-nav\""), "{body}");
}
