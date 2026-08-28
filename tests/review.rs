mod common;

use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::process::Command;

use axum::body::Body;
use axum::extract::connect_info::MockConnectInfo;
use axum::http::{Request, StatusCode};
use common::{temp_dir, valid_strict_concept, write_index};
use http_body_util::BodyExt;
use okf::Profile;
use tower::ServiceExt;

fn git(repo: &Path, args: &[&str]) {
    let status = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .status()
        .unwrap();
    assert!(status.success(), "git {args:?} failed");
}

fn git_knowledge() -> (PathBuf, PathBuf) {
    let repo = temp_dir("review-git");
    git(&repo, &["init", "--initial-branch=main"]);
    git(&repo, &["config", "user.email", "test@example.com"]);
    git(&repo, &["config", "user.name", "Test"]);
    git(&repo, &["config", "commit.gpgsign", "false"]);
    let knowledge = repo.join("knowledge");
    fs::create_dir_all(&knowledge).unwrap();
    write_index(&knowledge);
    fs::write(
        knowledge.join("draft.md"),
        valid_strict_concept("Draft", "", "Needs verify.\n"),
    )
    .unwrap();
    fs::write(
        knowledge.join("pending.md"),
        valid_strict_concept(
            "Pending",
            "verified:\n  - { by: human:nils, at: 2026-08-01T00:00:00Z }\n",
            "Ready to promote.\n",
        ),
    )
    .unwrap();
    fs::write(
        knowledge.join("explore.md"),
        "---\ntype: Architecture\ntitle: Explore\ndescription: Test concept Explore.\ntags: [domain/okf, concern/architecture]\nstatus: draft\ngenerated: { by: process:test, at: 2026-08-17T00:00:00Z }\nauthority: exploratory\nowners: [human:nils]\nverified:\n  - { by: human:nils, at: 2026-08-01T00:00:00Z }\n---\n\n# Explore\n\nExploratory.\n",
    )
    .unwrap();
    let inferred = okf::resolve_bundle(&repo).unwrap();
    assert_eq!(inferred, fs::canonicalize(&knowledge).unwrap());
    (repo, inferred)
}

fn app(root: PathBuf, output: PathBuf, config: PathBuf, peer: [u8; 4]) -> axum::Router {
    let state = okmate::http::AppState::new(output, root, Profile::Strict, config);
    okmate::http::router(state).layer(MockConnectInfo(SocketAddr::from((peer, 40000))))
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

fn write_actor(config: &Path, actor: &str) {
    fs::write(config, format!("actor = \"{actor}\"\n")).unwrap();
}

fn hash(root: &Path, name: &str) -> String {
    okmate::author::file_hash(&fs::read(root.join(name)).unwrap())
}

fn post_body(action: &str, concept: &str, hash: &str) -> String {
    format!("action={action}&concept={concept}&hash={hash}&return=/review/")
}

#[tokio::test]
async fn verify_appends_and_clears_initial_verification() {
    let (_repo, root) = git_knowledge();
    let output = temp_dir("review-out");
    okmate::site::build(&root, &output, Profile::Strict).unwrap();
    let config = temp_dir("review-cfg").join("config.toml");
    write_actor(&config, "human:nils");
    let digest = hash(&root, "draft.md");
    let app = app(root.clone(), output, config, [127, 0, 0, 1]);
    let response = app
        .oneshot(
            Request::post("/__okmate/review")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(post_body("verify", "draft", &digest)))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = body_text(response).await;
    assert!(
        body.contains("id=\"okmate-main\"") || body.contains("id=\"okmate-queue\""),
        "{body}"
    );
    assert!(
        !body.contains("Initial human review"),
        "queue should drop InitialVerification: {body}"
    );
    let source = fs::read_to_string(root.join("draft.md")).unwrap();
    assert!(source.contains("by: human:nils"), "{source}");
    assert!(source.contains("status: draft"), "{source}");
}

#[tokio::test]
async fn datastar_verify_returns_main_patch() {
    let (_repo, root) = git_knowledge();
    let output = temp_dir("review-ds-out");
    okmate::site::build(&root, &output, Profile::Strict).unwrap();
    let config = temp_dir("review-ds-cfg").join("config.toml");
    write_actor(&config, "human:nils");
    let digest = hash(&root, "draft.md");
    let app = app(root, output, config, [127, 0, 0, 1]);
    let response = app
        .oneshot(
            Request::post("/__okmate/review")
                .header("datastar-request", "true")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(post_body("verify", "draft", &digest)))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = body_text(response).await;
    assert!(body.contains("id=\"okmate-main\""), "{body}");
    assert!(
        !body.to_ascii_lowercase().contains("<html"),
        "patch should not be a full document: {body}"
    );
}

#[tokio::test]
async fn stale_hash_conflicts() {
    let (_repo, root) = git_knowledge();
    let output = temp_dir("review-cas-out");
    okmate::site::build(&root, &output, Profile::Strict).unwrap();
    let config = temp_dir("review-cas-cfg").join("config.toml");
    write_actor(&config, "human:nils");
    let digest = hash(&root, "draft.md");
    let state = okmate::http::AppState::new(output, root.clone(), Profile::Strict, config);
    let app = okmate::http::router(state.clone())
        .layer(MockConnectInfo(SocketAddr::from(([127, 0, 0, 1], 40000))));
    let first = app
        .oneshot(
            Request::post("/__okmate/review")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(post_body("verify", "draft", &digest)))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(first.status(), StatusCode::OK);
    let app = okmate::http::router(state)
        .layer(MockConnectInfo(SocketAddr::from(([127, 0, 0, 1], 40000))));
    let second = app
        .oneshot(
            Request::post("/__okmate/review")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(post_body("verify", "draft", &digest)))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(second.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn promote_without_verify_is_rejected() {
    let (_repo, root) = git_knowledge();
    let output = temp_dir("review-promote-out");
    okmate::site::build(&root, &output, Profile::Strict).unwrap();
    let config = temp_dir("review-promote-cfg").join("config.toml");
    write_actor(&config, "human:nils");
    let digest = hash(&root, "draft.md");
    let app = app(root, output, config, [127, 0, 0, 1]);
    let response = app
        .oneshot(
            Request::post("/__okmate/review")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(post_body("promote", "draft", &digest)))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn exploratory_promote_is_rejected() {
    let (_repo, root) = git_knowledge();
    let output = temp_dir("review-explore-out");
    okmate::site::build(&root, &output, Profile::Strict).unwrap();
    let config = temp_dir("review-explore-cfg").join("config.toml");
    write_actor(&config, "human:nils");
    let digest = hash(&root, "explore.md");
    let app = app(root, output, config, [127, 0, 0, 1]);
    let response = app
        .oneshot(
            Request::post("/__okmate/review")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(post_body("promote", "explore", &digest)))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn review_post_rejects_non_loopback() {
    let (_repo, root) = git_knowledge();
    let output = temp_dir("review-nl-out");
    okmate::site::build(&root, &output, Profile::Strict).unwrap();
    let config = temp_dir("review-nl-cfg").join("config.toml");
    write_actor(&config, "human:nils");
    let digest = hash(&root, "draft.md");
    let app = app(root, output, config, [10, 0, 0, 1]);
    let response = app
        .oneshot(
            Request::post("/__okmate/review")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(post_body("verify", "draft", &digest)))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn promote_sets_stable_after_human_verification() {
    let (_repo, root) = git_knowledge();
    let output = temp_dir("review-stable-out");
    okmate::site::build(&root, &output, Profile::Strict).unwrap();
    let config = temp_dir("review-stable-cfg").join("config.toml");
    write_actor(&config, "human:nils");
    let digest = hash(&root, "pending.md");
    let app = app(root.clone(), output, config, [127, 0, 0, 1]);
    let response = app
        .oneshot(
            Request::post("/__okmate/review")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(post_body("promote", "pending", &digest)))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let source = fs::read_to_string(root.join("pending.md")).unwrap();
    assert!(source.contains("status: stable"), "{source}");
    assert!(source.contains("by: human:nils"), "{source}");
}
