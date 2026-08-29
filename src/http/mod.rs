use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::{Path, PathBuf};
use std::sync::{Arc, PoisonError, RwLock};

use axum::Router;
use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::middleware;
use axum::response::Redirect;
use axum::routing::{get, post};
use okf::Profile;
use serde::Deserialize;
use tower_http::services::ServeDir;

mod pages;
mod prefs;
mod settings;

pub use settings::{render_fragment, render_page, settings_roots};

#[derive(Clone)]
pub struct AppState {
    pub output: PathBuf,
    pub root: PathBuf,
    pub workspace: Arc<RwLock<crate::workspace::Workspace>>,
    pub profile: Profile,
    pub config_path: PathBuf,
    pub session_path: PathBuf,
}

impl AppState {
    pub fn new(output: PathBuf, root: PathBuf, profile: Profile, config_path: PathBuf) -> Self {
        let workspace = crate::workspace::Workspace::load_single(&root, profile)
            .unwrap_or_else(|_| crate::workspace::Workspace::empty());
        Self {
            output,
            root,
            workspace: share_workspace(workspace),
            profile,
            config_path,
            session_path: crate::session::session_path(),
        }
    }

    pub fn replace_workspace(&self, workspace: crate::workspace::Workspace) {
        *self
            .workspace
            .write()
            .unwrap_or_else(PoisonError::into_inner) = workspace;
    }
}

pub fn share_workspace(
    workspace: crate::workspace::Workspace,
) -> Arc<RwLock<crate::workspace::Workspace>> {
    Arc::new(RwLock::new(workspace))
}

pub fn bind_addr(public: bool, port: u16) -> SocketAddr {
    let ip = if public {
        IpAddr::V4(Ipv4Addr::UNSPECIFIED)
    } else {
        IpAddr::V4(Ipv4Addr::LOCALHOST)
    };
    SocketAddr::new(ip, port)
}

pub fn router(state: AppState) -> Router {
    let output = state.output.clone();
    Router::new()
        .route("/__okmate/nav-mode", get(set_nav_mode))
        .route("/__okmate/prefs", post(prefs::post))
        .route("/__okmate/settings", post(settings::post))
        .route("/__okmate/review-window", get(pages::review_window))
        .route("/__okmate/log-window", get(pages::log_window))
        .nest_service("/__okmate", ServeDir::new(output.join("__okmate")))
        .fallback_service(ServeDir::new(output).append_index_html_on_directories(true))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            pages::datastar_get,
        ))
        .with_state(state)
}

#[derive(Deserialize)]
struct NavModeQuery {
    mode: String,
}

async fn set_nav_mode(
    State(state): State<AppState>,
    Query(query): Query<NavModeQuery>,
    headers: HeaderMap,
) -> Redirect {
    if let Some(mode) = crate::session::NavMode::parse(&query.mode) {
        crate::session::persist_nav_mode_to(&state.session_path, mode);
        let workspace = state
            .workspace
            .read()
            .unwrap_or_else(PoisonError::into_inner);
        let _ = crate::site::build_workspace_nav(&workspace, &state.output);
    }
    redirect_back(&headers)
}

fn redirect_back(headers: &HeaderMap) -> Redirect {
    let target = headers
        .get(axum::http::header::REFERER)
        .and_then(|value| value.to_str().ok())
        .and_then(referer_path)
        .unwrap_or_else(|| "/".to_string());
    Redirect::to(&target)
}

fn referer_path(referer: &str) -> Option<String> {
    let uri = referer.parse::<axum::http::Uri>().ok()?;
    let path = uri.path();
    if !path.starts_with('/') || path.starts_with("/__okmate") {
        return None;
    }
    let mut target = path.to_string();
    if let Some(query) = uri.query() {
        target.push('?');
        target.push_str(query);
    }
    Some(target)
}

pub fn output_path(output: Option<&Path>, root: &Path) -> PathBuf {
    output.map(Path::to_path_buf).unwrap_or_else(|| {
        std::env::temp_dir().join(format!(
            "okmate-view-{}-{}",
            std::process::id(),
            root.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("bundle")
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_bind_is_localhost() {
        let addr = bind_addr(false, 8000);
        assert!(addr.ip().is_loopback());
        assert_eq!(addr.port(), 8000);
    }

    #[test]
    fn public_bind_is_unspecified() {
        let addr = bind_addr(true, 9000);
        assert!(addr.ip().is_unspecified());
    }
}
