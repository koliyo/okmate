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
    assert!(
        alpha_body.contains("<title>Alpha Shared</title>"),
        "{alpha_body}"
    );
    let alpha_main = alpha_body
        .split_once("id=\"okmate-main\"")
        .map(|(_, rest)| rest)
        .unwrap_or(&alpha_body);
    assert!(alpha_main.contains("Alpha Shared"), "{alpha_main}");
    assert!(!alpha_main.contains("Beta Shared"), "{alpha_main}");

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
    assert!(
        beta_body.contains("<title>Beta Shared</title>"),
        "{beta_body}"
    );
    let beta_main = beta_body
        .split_once("id=\"okmate-main\"")
        .map(|(_, rest)| rest)
        .unwrap_or(&beta_body);
    assert!(beta_main.contains("Beta Shared"), "{beta_main}");
    assert!(!beta_main.contains("Alpha Shared"), "{beta_main}");

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

#[test]
fn workspace_nav_wraps_each_root_as_a_top_level_section() {
    let (_a, _b, _workspace, output) = two_bundles();
    let html = fs::read_to_string(output.join("index.html")).unwrap();
    assert!(html.contains("data-okmate-nav-section=\"a\""), "{html}");
    assert!(html.contains("data-okmate-nav-section=\"b\""), "{html}");
    assert!(
        html.contains("data-okmate-nav-section=\"a/plans\""),
        "{html}"
    );
    assert!(
        html.contains("data-okmate-nav-section=\"b/plans\""),
        "{html}"
    );
    assert!(html.contains("href=\"/@a/plans/shared/\""), "{html}");
    assert!(html.contains("href=\"/@b/plans/shared/\""), "{html}");
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

fn concept_at(title: &str, at: &str) -> String {
    format!(
        "---\ntype: Architecture\ntitle: {title}\ndescription: Test concept {title}.\ntags: [domain/okf, concern/architecture]\nstatus: draft\ngenerated: {{ by: process:test, at: {at} }}\nauthority: descriptive\nowners: [human:nils]\n---\n\n# {title}\n\nBody.\n"
    )
}

#[tokio::test]
async fn workspace_home_orders_recents_and_merges_log_days() {
    let a = temp_dir("ws-dash-a");
    let b = temp_dir("ws-dash-b");
    write_index(&a);
    write_index(&b);
    fs::create_dir_all(a.join("plans")).unwrap();
    fs::create_dir_all(b.join("plans")).unwrap();
    fs::write(a.join("plans").join("index.md"), "# Plans\n").unwrap();
    fs::write(b.join("plans").join("index.md"), "# Plans\n").unwrap();
    fs::write(
        a.join("plans").join("older.md"),
        concept_at("Older", "2026-08-10T00:00:00Z"),
    )
    .unwrap();
    fs::write(
        b.join("plans").join("newer.md"),
        concept_at("Newer", "2026-08-20T00:00:00Z"),
    )
    .unwrap();
    fs::write(
        a.join("log.md"),
        "# Knowledge log\n\n## 2026-08-10\n\n- Alpha day.\n",
    )
    .unwrap();
    fs::write(
        b.join("log.md"),
        "# Knowledge log\n\n## 2026-08-20\n\n- Beta day.\n",
    )
    .unwrap();
    let workspace = Workspace::load_members(
        vec![("a".into(), a.clone()), ("b".into(), b.clone())],
        Profile::Strict,
    )
    .unwrap();
    let output = temp_dir("ws-dash-out");
    okmate::site::build_workspace(&workspace, &output).unwrap();
    let home = fs::read_to_string(output.join("index.html")).unwrap();
    assert!(home.contains("id=\"okmate-recents\""), "{home}");
    assert!(home.contains("id=\"okmate-log\""), "{home}");
    assert!(!home.contains("Knowledge Collections"), "{home}");
    let recents = home
        .split_once("id=\"okmate-recents\"")
        .map(|(_, rest)| rest)
        .and_then(|rest| rest.split_once("id=\"okmate-log\""))
        .map(|(recents, _)| recents)
        .expect("recents section");
    let newer = recents.find("Newer").expect("newer first");
    let older = recents.find("Older").expect("older second");
    assert!(newer < older, "{recents}");
    assert!(home.contains("okmate-root\">a<"), "{home}");
    assert!(home.contains("okmate-root\">b<"), "{home}");
    let log = home
        .split_once("id=\"okmate-log\"")
        .map(|(_, rest)| rest)
        .expect("log section");
    let beta = log.find("2026-08-20").expect("newer log day");
    let alpha = log.find("2026-08-10").expect("older log day");
    assert!(beta < alpha, "{log}");
    assert!(home.contains("Alpha day."), "{home}");
    assert!(home.contains("Beta day."), "{home}");

    let app = app(a, output, workspace);
    let review = app
        .oneshot(Request::get("/review/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let review_body = body_text(review).await;
    assert!(review_body.contains(">Source<"), "{review_body}");
}
