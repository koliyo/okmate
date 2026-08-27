use std::convert::Infallible;
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
    let path = request.uri().path();
    let query = request.uri().query().map(str::to_string);
    if path.starts_with("/__okmate") || path.ends_with(".json") {
        return next.run(request).await;
    }
    if is_datastar(request.headers()) {
        let Some((fragment, reload_ms, render_ms)) =
            render_main_fragment(&state, path, query.as_deref())
        else {
            return next.run(request).await;
        };
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
    let Some((html, reload_ms, render_ms)) = render_full_page(&state, path, query.as_deref())
    else {
        return next.run(request).await;
    };
    let bytes = html.len();
    let mut response = Html(html).into_response();
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

fn render_main_fragment(
    state: &AppState,
    path: &str,
    query: Option<&str>,
) -> Option<(String, f64, f64)> {
    let started = Instant::now();
    let document = live_document(state, path, query)?;
    let fragment = document.render_main_fragment().ok()?;
    Some((fragment, 0.0, started.elapsed().as_secs_f64() * 1000.0))
}

fn render_full_page(
    state: &AppState,
    path: &str,
    query: Option<&str>,
) -> Option<(String, f64, f64)> {
    let started = Instant::now();
    let document = live_document(state, path, query)?;
    let html = site::render_document(document).ok()?;
    Some((html, 0.0, started.elapsed().as_secs_f64() * 1000.0))
}

pub fn live_document(
    state: &AppState,
    path: &str,
    query: Option<&str>,
) -> Option<crate::views::Document> {
    let workspace = state.workspace.read().ok()?;
    if workspace.is_empty() {
        return None;
    }
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
    let window = crate::views::WindowQuery::from_raw(query);
    if document.page_kind == "review" {
        crate::views::apply_review_window(&mut document, &window);
    }
    if document.page_kind == "log" {
        crate::views::apply_log_window(&mut document, &window);
    }
    Some(document)
}

#[derive(serde::Deserialize, Default)]
pub struct WindowParams {
    pub start: Option<usize>,
    pub filter: Option<String>,
    pub q: Option<String>,
}

pub async fn review_window(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<WindowParams>,
) -> Response {
    let query = window_query(&params);
    let Some(document) = live_document(&state, "/review/", Some(&query)) else {
        return axum::http::StatusCode::NOT_FOUND.into_response();
    };
    match document.render_queue_fragment() {
        Ok(html) => Html(html).into_response(),
        Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn log_window(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<WindowParams>,
) -> Response {
    let query = window_query(&params);
    let Some(document) = live_document(&state, "/log/", Some(&query)) else {
        return axum::http::StatusCode::NOT_FOUND.into_response();
    };
    match document.render_main_fragment() {
        Ok(html) => Html(html).into_response(),
        Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

fn window_query(params: &WindowParams) -> String {
    format!(
        "start={}&filter={}&q={}",
        params.start.unwrap_or(0),
        params.filter.as_deref().unwrap_or(""),
        params.q.as_deref().unwrap_or("")
    )
}
