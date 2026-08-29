use std::path::{Path, PathBuf};
use std::sync::{Arc, PoisonError, RwLock};
use std::time::Duration;

use anyhow::{Context, Result, bail};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use okf::{LoadOptions, Profile};
use tokio::sync::mpsc;

use crate::http::{bind_addr, output_path, router};
use crate::site;
use crate::workspace::Workspace;

pub use crate::session::{
    NavMode, Session, load_session, load_session_from, persist_bundle, persist_bundle_to,
    persist_nav_mode_to, persist_open_path_to, persist_prefs_to, persist_workspace,
    persist_workspace_to, session_path, state_dir,
};

pub fn restored_view_location(
    workspace: &Workspace,
    session: &Session,
    explicit: bool,
    fallback: &str,
) -> (String, Option<String>, Option<u32>) {
    let fallback = crate::session::sanitize_open_path(Some(fallback)).unwrap_or_else(|| "/".into());
    if explicit {
        return (fallback, None, None);
    }
    let Some(saved) = session.open_path.as_deref() else {
        return (fallback, None, None);
    };
    if crate::site::page_for_route_nav(workspace, saved, session.nav_mode).is_none() {
        return (fallback, None, None);
    }
    (
        saved.to_string(),
        session.open_hash.clone(),
        session.main_scroll,
    )
}

pub struct ViewOptions {
    pub path: Option<PathBuf>,
    pub output: Option<PathBuf>,
    pub profile: Profile,
    pub provenance: bool,
    pub public: bool,
    pub port: u16,
    pub no_window: bool,
    pub allow_missing_bundle: bool,
}

pub fn view_load_options(profile: Profile, provenance: bool) -> LoadOptions {
    LoadOptions::new(profile).with_provenance(provenance)
}

pub struct ServerReady {
    pub home_url: String,
    pub initial_url: String,
}

pub fn run(options: ViewOptions) -> Result<()> {
    if options.no_window {
        let runtime = tokio::runtime::Runtime::new().context("failed to start tokio runtime")?;
        runtime.block_on(run_headless(options))
    } else {
        #[cfg(feature = "desktop")]
        {
            crate::desktop::run(options)
        }
        #[cfg(not(feature = "desktop"))]
        {
            bail!("okmate was built without the desktop feature; pass --no-window")
        }
    }
}

pub fn home_url(bound: impl std::fmt::Display) -> String {
    format!("http://{bound}/")
}

async fn run_headless(options: ViewOptions) -> Result<()> {
    prepare(options).await?.serve().await
}

#[cfg(feature = "desktop")]
pub(crate) async fn serve_ready(
    options: ViewOptions,
    ready: std::sync::mpsc::Sender<Result<ServerReady>>,
) -> Result<()> {
    match prepare(options).await {
        Ok(prepared) => {
            let _ = ready.send(Ok(ServerReady {
                home_url: prepared.home_url.clone(),
                initial_url: prepared.initial_url.clone(),
            }));
            prepared.serve().await
        }
        Err(error) => {
            let _ = ready.send(Err(anyhow::anyhow!("{error:#}")));
            Err(error)
        }
    }
}

struct PreparedView {
    listener: tokio::net::TcpListener,
    state: crate::http::AppState,
    #[cfg_attr(not(feature = "desktop"), allow(dead_code))]
    home_url: String,
    #[cfg_attr(not(feature = "desktop"), allow(dead_code))]
    initial_url: String,
}

impl PreparedView {
    async fn serve(self) -> Result<()> {
        axum::serve(
            self.listener,
            router(self.state).into_make_service_with_connect_info::<std::net::SocketAddr>(),
        )
        .await
        .context("okmate view server stopped")
    }
}

async fn prepare(options: ViewOptions) -> Result<PreparedView> {
    let load_options = view_load_options(options.profile, options.provenance);
    let cache_parent = crate::config::cache_dir();
    let target = match Workspace::for_view(
        options.path.as_deref(),
        load_options,
        &crate::config::config_path(),
        &cache_parent,
        load_session().bundle.as_deref(),
    ) {
        Ok(target) => Some(target),
        Err(error) if options.allow_missing_bundle && options.path.is_none() => {
            eprintln!("okmate: {error:#}; opening settings");
            None
        }
        Err(error) => return Err(error),
    };
    let Some(target) = target else {
        return prepare_settings_host(options).await;
    };
    persist_workspace(&target.workspace);
    let session = load_session();
    let nav_mode = session.nav_mode;
    let (open_path, open_hash, _) = restored_view_location(
        &target.workspace,
        &session,
        options.path.is_some(),
        &target.open_path,
    );
    persist_open_path_to(&session_path(), &open_path);
    let root = target
        .workspace
        .primary_path()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("workspace"));
    let output = output_path(options.output.as_deref(), &root);
    site::build_workspace_nav(&target.workspace, &output, nav_mode)?;

    let addr = bind_addr(options.public, options.port);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("failed to bind {addr}"))?;
    let bound = listener
        .local_addr()
        .context("failed to read bound address")?;
    let label = if target.workspace.is_multi() {
        format!("{} roots", target.workspace.len())
    } else {
        root.display().to_string()
    };
    eprintln!("okmate: serving {label} at http://{}{}", bound, open_path);

    let workspace = crate::http::share_workspace(target.workspace);
    let watch_workspace = workspace.clone();
    let watch_output = output.clone();
    tokio::spawn(async move {
        if let Err(error) =
            watch_rebuild(watch_workspace, watch_output, load_options, cache_parent).await
        {
            eprintln!("okmate: watch stopped: {error:#}");
        }
    });

    let home_url = home_url(bound);
    let path = if open_path.starts_with('/') {
        open_path
    } else {
        format!("/{open_path}")
    };
    let initial_url = match open_hash.as_deref() {
        Some(hash) if !hash.is_empty() => {
            format!("{}{path}#{hash}", home_url.trim_end_matches('/'))
        }
        _ => format!("{}{path}", home_url.trim_end_matches('/')),
    };
    Ok(PreparedView {
        listener,
        state: crate::http::AppState {
            output,
            root,
            workspace,
            profile: options.profile,
            config_path: crate::config::config_path(),
            session_path: session_path(),
        },
        home_url,
        initial_url,
    })
}

async fn prepare_settings_host(options: ViewOptions) -> Result<PreparedView> {
    let output = output_path(options.output.as_deref(), Path::new("settings"));
    site::write_settings_host(&output)?;
    let addr = bind_addr(options.public, options.port);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("failed to bind {addr}"))?;
    let bound = listener
        .local_addr()
        .context("failed to read bound address")?;
    let home_url = home_url(bound);
    let initial_url = format!("{}settings/", home_url);
    eprintln!("okmate: serving settings at {initial_url}");
    Ok(PreparedView {
        listener,
        state: crate::http::AppState {
            output,
            root: PathBuf::from("/"),
            workspace: crate::http::share_workspace(Workspace::empty()),
            profile: options.profile,
            config_path: crate::config::config_path(),
            session_path: session_path(),
        },
        home_url,
        initial_url,
    })
}

pub fn resolve_target(path: Option<&Path>) -> Result<okf::PreviewTarget> {
    if let Some(path) = path {
        return okf::resolve_preview_path(path);
    }
    if let Some(bundle) = load_session().bundle.filter(|path| path.is_dir()) {
        return Ok(okf::PreviewTarget::bundle(bundle));
    }
    let default = PathBuf::from("knowledge");
    if default.is_dir() {
        return okf::resolve_preview_path(&default);
    }
    bail!("pass a knowledge bundle path, or open one first so ~/.okmate/state remembers it");
}

async fn watch_rebuild(
    workspace: Arc<RwLock<Workspace>>,
    output: PathBuf,
    options: LoadOptions,
    cache_parent: PathBuf,
) -> Result<()> {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let mut watcher = RecommendedWatcher::new(
        move |event| {
            let _ = tx.send(event);
        },
        Config::default(),
    )
    .context("failed to start knowledge watcher")?;
    for path in workspace
        .read()
        .unwrap_or_else(PoisonError::into_inner)
        .watch_paths()
    {
        watcher
            .watch(&path, RecursiveMode::Recursive)
            .with_context(|| format!("failed to watch {}", path.display()))?;
    }

    loop {
        let Some(event) = rx.recv().await else {
            break;
        };
        if event.is_err() {
            continue;
        }
        let debounce = tokio::time::sleep(Duration::from_millis(200));
        tokio::pin!(debounce);
        loop {
            tokio::select! {
                next = rx.recv() => {
                    if next.is_none() {
                        break;
                    }
                }
                _ = &mut debounce => break,
            }
        }
        let snapshot = workspace
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .clone();
        match snapshot.reload_with(options, Some(&cache_parent)) {
            Ok(reloaded) => {
                let nav_mode = load_session().nav_mode;
                if let Err(error) = site::build_workspace_nav(&reloaded, &output, nav_mode) {
                    eprintln!("okmate: rebuild failed: {error:#}");
                }
                *workspace.write().unwrap_or_else(PoisonError::into_inner) = reloaded;
            }
            Err(error) => eprintln!("okmate: reload failed: {error:#}"),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn restored_location_keeps_valid_route_and_drops_unknown() {
        let workspace = Workspace::empty();
        let mut session = Session {
            open_path: Some("/review/".into()),
            open_hash: Some("queue".into()),
            main_scroll: Some(40),
            ..Session::default()
        };
        let (path, hash, scroll) = restored_view_location(&workspace, &session, false, "/");
        assert_eq!(path, "/review/");
        assert_eq!(hash.as_deref(), Some("queue"));
        assert_eq!(scroll, Some(40));

        session.open_path = Some("/missing/doc/".into());
        let (path, hash, scroll) = restored_view_location(&workspace, &session, false, "/");
        assert_eq!(path, "/");
        assert!(hash.is_none());
        assert!(scroll.is_none());

        session.open_path = Some("/review/".into());
        let (path, hash, scroll) = restored_view_location(&workspace, &session, true, "/log/");
        assert_eq!(path, "/log/");
        assert!(hash.is_none() && scroll.is_none());
    }
}
