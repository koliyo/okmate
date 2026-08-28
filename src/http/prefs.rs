use std::net::SocketAddr;

use axum::extract::{ConnectInfo, Json, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::http::AppState;

pub async fn post(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    if !addr.ip().is_loopback() {
        return (StatusCode::FORBIDDEN, "prefs POST is loopback-only").into_response();
    }
    crate::preview::persist_prefs_to(&state.session_path, &body);
    StatusCode::NO_CONTENT.into_response()
}
