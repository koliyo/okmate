use std::convert::Infallible;
use std::time::Instant;

use axum::extract::{Request, State};
use axum::http::{HeaderMap, HeaderValue, Method};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response, Sse, sse::Event};
use datastar::prelude::PatchElements;
use futures_util::stream;

use crate::http::AppState;
use crate::site;
use crate::timings::server_timing_header;

pub async fn datastar_get(State(state): State<AppState>, request: Request, next: Next) -> Response {
    if request.method() != Method::GET || !is_datastar(request.headers()) {
        return next.run(request).await;
    }
    let path = request.uri().path();
    if path.starts_with("/__okmate") || path.ends_with(".json") {
        return next.run(request).await;
    }
    let Some((fragment, reload_ms, render_ms)) = render_main_fragment(&state, path) else {
        return next.run(request).await;
    };
    let bytes = fragment.len();
    let patch = PatchElements::new(fragment);
    let mut response = Sse::new(stream::once(async move {
        Ok::<Event, Infallible>(patch.write_as_axum_sse_event())
    }))
    .into_response();
    if let Ok(value) = HeaderValue::from_str(&server_timing_header(reload_ms, render_ms, bytes)) {
        response.headers_mut().insert("server-timing", value);
    }
    response
}

pub fn is_datastar(headers: &HeaderMap) -> bool {
    headers
        .get("datastar-request")
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.eq_ignore_ascii_case("true"))
}

fn render_main_fragment(state: &AppState, path: &str) -> Option<(String, f64, f64)> {
    let started = Instant::now();
    let workspace = state.workspace.reload(state.profile).ok()?;
    let reload_ms = started.elapsed().as_secs_f64() * 1000.0;
    if workspace.is_empty() {
        return None;
    }
    let started = Instant::now();
    let mut document = site::page_for_route_nav(
        &workspace,
        path,
        crate::preview::load_session_from(&state.session_path).nav_mode,
    )?;
    if document.page_kind == "settings" {
        let config = crate::config::load_or_default(&state.config_path);
        document.config_path = state.config_path.display().to_string();
        document.settings_roots = crate::http::settings_roots(&config);
    }
    let fragment = document.render_main_fragment().ok()?;
    Some((
        fragment,
        reload_ms,
        started.elapsed().as_secs_f64() * 1000.0,
    ))
}
