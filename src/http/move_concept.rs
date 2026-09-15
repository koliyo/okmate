use std::net::SocketAddr;
use std::sync::PoisonError;

use axum::Json;
use axum::extract::{ConnectInfo, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

use crate::CheckFormat;
use crate::http::AppState;
use crate::move_concept::{MoveOptions, apply_move, plan_move};
use crate::workspace::Workspace;

#[derive(Debug, Deserialize)]
pub struct MoveQuery {
    #[serde(default)]
    pub root: String,
    pub from: String,
    pub to: String,
    #[serde(default)]
    pub apply: bool,
}

#[derive(Debug, Serialize)]
struct MoveHttpResponse {
    #[serde(flatten)]
    plan: crate::move_concept::MovePlan,
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<String>,
}

pub async fn get(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(query): Query<MoveQuery>,
) -> Response {
    handle(addr, state, query).await
}

pub async fn post(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Json(query): Json<MoveQuery>,
) -> Response {
    handle(addr, state, query).await
}

async fn handle(addr: SocketAddr, state: AppState, query: MoveQuery) -> Response {
    if !addr.ip().is_loopback() {
        return (StatusCode::FORBIDDEN, "move is loopback-only").into_response();
    }
    match tokio::task::spawn_blocking(move || run_move(&state, query)).await {
        Ok(Ok(body)) => Json(body).into_response(),
        Ok(Err(error)) => (StatusCode::BAD_REQUEST, format!("{error:#}")).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{error:#}")).into_response(),
    }
}

fn run_move(state: &AppState, query: MoveQuery) -> anyhow::Result<MoveHttpResponse> {
    let workspace = state
        .workspace
        .read()
        .unwrap_or_else(PoisonError::into_inner);
    let (root_id, root_path) = writable_member(&workspace, &query.root, &state.cache_parent)?;
    drop(workspace);
    let opts = MoveOptions {
        root: root_path,
        from: query.from,
        to: query.to,
        apply: query.apply,
        format: CheckFormat::Json,
    };
    let mut plan = plan_move(&opts)?;
    let mut href = None;
    if query.apply {
        apply_move(&opts.root, &plan)?;
        reload_members(state)?;
        let workspace = state
            .workspace
            .read()
            .unwrap_or_else(PoisonError::into_inner);
        href = Some(workspace.document_href(&root_id, &plan.to));
        plan.apply = true;
    }
    Ok(MoveHttpResponse { plan, href })
}

fn reload_members(state: &AppState) -> anyhow::Result<()> {
    let snapshot = state
        .workspace
        .read()
        .unwrap_or_else(PoisonError::into_inner)
        .clone();
    let reloaded = snapshot.reload_with(state.load_options, Some(&state.cache_parent))?;
    crate::site::build_workspace_nav(&reloaded, &state.output)?;
    let paths = reloaded.watch_paths();
    state.replace_workspace(reloaded);
    let _ = state.watch_paths.send(paths);
    Ok(())
}

fn writable_member(
    workspace: &Workspace,
    root: &str,
    cache_parent: &std::path::Path,
) -> anyhow::Result<(String, std::path::PathBuf)> {
    let member = if root.is_empty() {
        workspace
            .primary()
            .ok_or_else(|| anyhow::anyhow!("no knowledge bundle is loaded"))?
    } else {
        workspace
            .get(root)
            .ok_or_else(|| anyhow::anyhow!("unknown knowledge root `{root}`"))?
    };
    if member.path.starts_with(cache_parent) {
        anyhow::bail!("git-cache roots are read-only");
    }
    Ok((member.id.clone(), member.path.clone()))
}
