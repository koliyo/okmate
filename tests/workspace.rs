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
        workspace: okmate::http::share_workspace(workspace),
        profile: Profile::Strict,
        config_path: std::env::temp_dir().join("okmate-ws-unused.toml"),
        session_path: std::env::temp_dir().join(format!(
            "okmate-ws-session-{}-{}.json",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        )),
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
    let (_a, _b, workspace, output) = two_bundles();
    okmate::site::write_html_pages(&workspace, &output, okmate::preview::NavMode::Separated)
        .unwrap();
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

#[test]
fn workspace_merged_nav_unions_plans_with_distinct_leaves() {
    let (_a, _b, workspace, output) = two_bundles();
    okmate::site::write_html_pages(&workspace, &output, okmate::preview::NavMode::Merged).unwrap();
    let html = fs::read_to_string(output.join("index.html")).unwrap();
    assert!(html.contains("data-okmate-nav-section=\"plans\""), "{html}");
    assert!(
        !html.contains("data-okmate-nav-section=\"a/plans\""),
        "{html}"
    );
    assert!(
        !html.contains("data-okmate-nav-section=\"b/plans\""),
        "{html}"
    );
    assert!(html.contains("href=\"/@a/plans/shared/\""), "{html}");
    assert!(html.contains("href=\"/@b/plans/shared/\""), "{html}");
    let plans = html
        .split("data-okmate-nav-section=\"plans\"")
        .nth(1)
        .expect("plans section");
    let plans = plans.split("<details").next().unwrap_or(plans);
    assert!(plans.contains("href=\"/@a/plans/shared/\""), "{plans}");
    assert!(plans.contains("href=\"/@b/plans/shared/\""), "{plans}");
    assert!(html.contains("id=\"okmate-nav-mode\""), "{html}");
    assert!(html.contains("/__okmate/nav-mode?mode=merged"), "{html}");
}

#[tokio::test]
async fn nav_mode_toggle_switches_trees() {
    let (a, _b, workspace, output) = two_bundles();
    let session = temp_dir("ws-nav-session").join("session.json");
    let state = okmate::http::AppState {
        output: output.clone(),
        root: a,
        workspace: okmate::http::share_workspace(workspace),
        profile: Profile::Strict,
        config_path: std::env::temp_dir().join("okmate-ws-unused.toml"),
        session_path: session.clone(),
    };
    let app = okmate::http::router(state.clone());
    let separated = body_text(
        app.clone()
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap(),
    )
    .await;
    assert!(
        separated.contains("data-okmate-nav-section=\"a/plans\""),
        "{separated}"
    );

    let merged_response = app
        .clone()
        .oneshot(
            Request::get("/__okmate/nav-mode?mode=merged")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert!(
        merged_response.status().is_redirection(),
        "{}",
        merged_response.status()
    );
    let merged = body_text(
        app.clone()
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap(),
    )
    .await;
    assert!(
        merged.contains("data-okmate-nav-section=\"plans\""),
        "{merged}"
    );
    assert!(
        !merged.contains("data-okmate-nav-section=\"a/plans\""),
        "{merged}"
    );
    let stored = fs::read_to_string(&session).unwrap();
    assert!(stored.contains("merged"), "{stored}");

    let _ = app
        .oneshot(
            Request::get("/__okmate/nav-mode?mode=separated")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let back = body_text(
        okmate::http::router(state)
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap(),
    )
    .await;
    assert!(
        back.contains("data-okmate-nav-section=\"a/plans\""),
        "{back}"
    );
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
    okmate::site::write_html_pages(&workspace, &output, okmate::preview::NavMode::Separated)
        .unwrap();
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

#[test]
fn dashboard_log_is_truncated_and_full_log_is_in_nav() {
    let root = temp_dir("ws-log-limit");
    write_index(&root);
    fs::write(
        root.join("log.md"),
        "# Knowledge log\n\n## 2026-08-26\n\n- Six.\n- Five.\n\n## 2026-08-25\n\n- Four.\n- Three.\n- Two.\n\n## 2026-08-24\n\n- One.\n",
    )
    .unwrap();
    let output = temp_dir("ws-log-limit-out");
    okmate::site::build(&root, &output, Profile::Strict).unwrap();
    let home = fs::read_to_string(output.join("index.html")).unwrap();
    assert!(home.contains("Six."), "{home}");
    assert!(home.contains("Two."), "{home}");
    assert!(!home.contains("One."), "{home}");
    assert!(home.contains("Recent log"), "{home}");
    assert!(home.contains("href=\"/log/\""), "{home}");
    assert!(!home.contains("Open review queue"), "{home}");
    let log = fs::read_to_string(output.join("log").join("index.html")).unwrap();
    assert!(log.contains("id=\"okmate-log\""), "{log}");
    assert!(log.contains("One."), "{log}");
    assert!(log.contains("Six."), "{log}");
    let h2 = log
        .split_once("id=\"bundle-log\"")
        .map(|(_, rest)| rest)
        .unwrap_or(&log);
    assert!(h2.contains(">Log<"), "{log}");
}

#[test]
fn collection_hover_uses_first_prose_paragraph_not_child_list() {
    let a = temp_dir("ws-blurb-a");
    let b = temp_dir("ws-blurb-b");
    write_index(&a);
    write_index(&b);
    fs::create_dir_all(a.join("plans")).unwrap();
    fs::create_dir_all(b.join("plans")).unwrap();
    fs::create_dir_all(a.join("empty")).unwrap();
    fs::write(
        a.join("plans").join("index.md"),
        "# Plans\n\nAlpha plans live here.\n\n- [Shared](shared.md)\n",
    )
    .unwrap();
    fs::write(
        b.join("plans").join("index.md"),
        "# Plans\n\nBeta plans live here.\n",
    )
    .unwrap();
    fs::write(a.join("empty").join("index.md"), "# Empty\n").unwrap();
    fs::write(
        a.join("plans").join("shared.md"),
        valid_strict_concept("Alpha Shared", "", "Shared body.\n"),
    )
    .unwrap();
    fs::write(
        b.join("plans").join("shared.md"),
        valid_strict_concept("Beta Shared", "", "Shared body.\n"),
    )
    .unwrap();
    let workspace = Workspace::load_members(
        vec![("a".into(), a.clone()), ("b".into(), b.clone())],
        Profile::Strict,
    )
    .unwrap();
    let output = temp_dir("ws-blurb-out");
    okmate::site::build_workspace(&workspace, &output).unwrap();
    okmate::site::write_html_pages(&workspace, &output, okmate::preview::NavMode::Separated)
        .unwrap();
    let html = fs::read_to_string(output.join("index.html")).unwrap();
    let blurbs: Vec<_> = html
        .split("class=\"okmate-nav-blurb\"")
        .skip(1)
        .map(|rest| rest.split("</span>").next().unwrap_or(rest))
        .collect();
    assert!(
        blurbs
            .iter()
            .any(|blurb| blurb.contains("Alpha plans live here.")),
        "{html}"
    );
    assert!(
        !html.contains("class=\"okmate-nav-blurb\">Empty"),
        "empty collection body must omit the popover: {html}"
    );
    assert!(html.contains(">Overview<"), "{html}");
    assert!(
        !html
            .split("class=\"okmate-nav-blurb\"")
            .skip(1)
            .any(|rest| rest
                .split("</span>")
                .next()
                .unwrap_or("")
                .contains("[Shared]")),
        "hover must not dump the child list: {html}"
    );

    okmate::site::write_html_pages(&workspace, &output, okmate::preview::NavMode::Merged).unwrap();
    let merged = fs::read_to_string(output.join("index.html")).unwrap();
    let merged_blurb = merged
        .split("class=\"okmate-nav-blurb\"")
        .nth(1)
        .and_then(|rest| rest.split("</span>").next())
        .expect("merged blurb");
    assert!(
        merged_blurb.contains("a: Alpha plans live here."),
        "{merged_blurb}"
    );
    assert!(
        merged_blurb.contains("b: Beta plans live here."),
        "{merged_blurb}"
    );
}
