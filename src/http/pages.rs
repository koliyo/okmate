use std::convert::Infallible;
use std::path::PathBuf;
use std::time::Instant;

use axum::extract::{Request, State};
use axum::http::{HeaderMap, HeaderValue, Method};
use axum::middleware::Next;
use axum::response::{Html, IntoResponse, Response, Sse, sse::Event};
use datastar::prelude::PatchElements;
use futures_util::stream;

use crate::http::AppState;
use crate::site;
use crate::timings::server_timing_header;

pub async fn datastar_get(State(state): State<AppState>, request: Request, next: Next) -> Response {
    if request.method() != Method::GET {
        return next.run(request).await;
    }
    let path = request.uri().path().to_string();
    let query = request.uri().query().map(str::to_string);
    if path.starts_with("/__okmate") || path.ends_with(".json") {
        return next.run(request).await;
    }
    if is_datastar(request.headers()) {
        let Some((fragment, reload_ms, render_ms)) =
            render_main_fragment(&state, &path, query.as_deref())
        else {
            return next.run(request).await;
        };
        persist_open_path_async(state.session_path.clone(), path).await;
        let bytes = fragment.len();
        let patch = PatchElements::new(fragment);
        let mut response = Sse::new(stream::once(async move {
            Ok::<Event, Infallible>(patch.write_as_axum_sse_event())
        }))
        .into_response();
        if let Ok(value) = HeaderValue::from_str(&server_timing_header(reload_ms, render_ms, bytes))
        {
            response.headers_mut().insert("server-timing", value);
        }
        return response;
    }
    let Some((html, reload_ms, render_ms)) = render_full_page(&state, &path, query.as_deref())
    else {
        return next.run(request).await;
    };
    persist_open_path_async(state.session_path.clone(), path).await;
    let bytes = html.len();
    let mut response = Html(html).into_response();
    if let Ok(value) = HeaderValue::from_str(&server_timing_header(reload_ms, render_ms, bytes)) {
        response.headers_mut().insert("server-timing", value);
    }
    response
}

async fn persist_open_path_async(session_path: PathBuf, route: String) {
    let _ = tokio::task::spawn_blocking(move || {
        crate::session::persist_open_path_to(&session_path, &route);
    })
    .await;
}

pub fn is_datastar(headers: &HeaderMap) -> bool {
    headers
        .get("datastar-request")
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.eq_ignore_ascii_case("true"))
}

fn render_main_fragment(
    state: &AppState,
    path: &str,
    query: Option<&str>,
) -> Option<(String, f64, f64)> {
    let started = Instant::now();
    let document = live_document(state, path, crate::views::WindowQuery::from_raw(query))?;
    let fragment = document.render_main_fragment().ok()?;
    Some((fragment, 0.0, started.elapsed().as_secs_f64() * 1000.0))
}

fn render_full_page(
    state: &AppState,
    path: &str,
    query: Option<&str>,
) -> Option<(String, f64, f64)> {
    let started = Instant::now();
    let document = live_document(state, path, crate::views::WindowQuery::from_raw(query))?;
    let html = site::render_document(document).ok()?;
    Some((html, 0.0, started.elapsed().as_secs_f64() * 1000.0))
}

pub fn live_document(
    state: &AppState,
    path: &str,
    query: crate::views::WindowQuery,
) -> Option<crate::views::Document> {
    let workspace = state.workspace.read().ok()?;
    if workspace.is_empty() {
        return None;
    }
    let session = crate::preview::load_session_from(&state.session_path);
    let mut document = site::page_for_route_nav(&workspace, path, session.nav_mode)?;
    crate::site::apply_reading_prefs(&mut document, &session);
    if session.open_path.as_deref() == Some(crate::workspace::normalize_route(path).as_str()) {
        document.main_scroll = session.main_scroll.unwrap_or(0);
    }
    if document.page_kind == crate::views::PageKind::Settings {
        let config = crate::config::load_or_default(&state.config_path);
        document.config_path = state.config_path.display().to_string();
        document.settings_roots = crate::http::settings_roots(&config);
    }
    if document.page_kind == crate::views::PageKind::Review {
        crate::views::apply_review_window(&mut document, &query);
    }
    if document.page_kind == crate::views::PageKind::Log {
        crate::views::apply_log_window(&mut document, &query);
    }
    Some(document)
}

pub async fn review_window(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<crate::views::WindowQuery>,
) -> Response {
    let Some(document) = live_document(&state, "/review/", params) else {
        return axum::http::StatusCode::NOT_FOUND.into_response();
    };
    match document.render_queue_fragment() {
        Ok(html) => Html(html).into_response(),
        Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn log_window(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<crate::views::WindowQuery>,
) -> Response {
    let Some(document) = live_document(&state, "/log/", params) else {
        return axum::http::StatusCode::NOT_FOUND.into_response();
    };
    match document.render_main_fragment() {
        Ok(html) => Html(html).into_response(),
        Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
