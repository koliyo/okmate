use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::Duration;

use anyhow::{Context, Result, bail};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use okf::{LoadOptions, Profile};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::http::{bind_addr, output_path, router};
use crate::site;
use crate::workspace::Workspace;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NavMode {
    #[default]
    Separated,
    Merged,
}

impl NavMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Separated => "separated",
            Self::Merged => "merged",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "separated" => Some(Self::Separated),
            "merged" => Some(Self::Merged),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
    #[serde(default)]
    pub bundle: Option<PathBuf>,
    #[serde(default)]
    pub workspace: bool,
    #[serde(default)]
    pub nav_mode: NavMode,
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
    let nav_mode = load_session().nav_mode;
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
    eprintln!(
        "okmate: serving {label} at http://{}{}",
        bound, target.open_path
    );

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
    let initial_url = format!(
        "{}{}",
        home_url.trim_end_matches('/'),
        if target.open_path.starts_with('/') {
            target.open_path.clone()
        } else {
            format!("/{}", target.open_path)
        }
    );
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

pub fn state_dir() -> PathBuf {
    if let Some(path) = env::var_os("OKMATE_STATE") {
        return PathBuf::from(path);
    }
    home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".okmate")
        .join("state")
}

pub fn session_path() -> PathBuf {
    state_dir().join("session.json")
}

pub fn persist_bundle(root: &Path) {
    persist_bundle_to(&session_path(), root);
}

pub fn persist_workspace(workspace: &Workspace) {
    persist_workspace_to(&session_path(), workspace);
}

pub fn persist_bundle_to(path: &Path, root: &Path) {
    let mut session = load_session_from(path);
    session.bundle = Some(root.to_path_buf());
    session.workspace = false;
    write_session(path, &session);
}

pub fn persist_workspace_to(path: &Path, workspace: &Workspace) {
    let mut session = load_session_from(path);
    session.workspace = workspace.is_multi();
    session.bundle = workspace.primary_path().map(Path::to_path_buf);
    write_session(path, &session);
}

pub fn persist_nav_mode_to(path: &Path, nav_mode: NavMode) {
    let mut session = load_session_from(path);
    session.nav_mode = nav_mode;
    write_session(path, &session);
}

fn write_session(path: &Path, session: &Session) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(session) {
        let tmp = path.with_extension("tmp");
        if fs::write(&tmp, json).is_ok() {
            let _ = fs::rename(&tmp, path);
        }
    }
}

pub fn load_session() -> Session {
    load_session_from(&session_path())
}

pub fn load_session_from(path: &Path) -> Session {
    let Ok(content) = fs::read_to_string(path) else {
        return Session::default();
    };
    serde_json::from_str(&content).unwrap_or_default()
}

fn home_dir() -> Option<PathBuf> {
    env::var_os("HOME")
        .or_else(|| env::var_os("USERPROFILE"))
        .map(PathBuf::from)
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
    for path in workspace.read().expect("workspace lock").watch_paths() {
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
        let snapshot = workspace.read().expect("workspace lock").clone();
        match snapshot.reload_with(options, Some(&cache_parent)) {
            Ok(reloaded) => {
                let nav_mode = load_session().nav_mode;
                if let Err(error) = site::build_workspace_nav(&reloaded, &output, nav_mode) {
                    eprintln!("okmate: rebuild failed: {error:#}");
                }
                *workspace.write().expect("workspace lock") = reloaded;
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
    fn persist_bundle_writes_session_file() {
        let dir =
            std::env::temp_dir().join(format!("okmate-state-{}-{}", std::process::id(), "persist"));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("session.json");
        persist_bundle_to(&path, Path::new("/tmp/knowledge"));
        let session = load_session_from(&path);
        assert_eq!(session.bundle.as_deref(), Some(Path::new("/tmp/knowledge")));
        assert!(!session.workspace);
        persist_workspace_to(
            &path,
            &Workspace::from_members(vec![
                crate::workspace::WorkspaceMember {
                    id: "a".into(),
                    path: PathBuf::from("/tmp/a"),
                    bundle: empty_bundle("/tmp/a"),
                },
                crate::workspace::WorkspaceMember {
                    id: "b".into(),
                    path: PathBuf::from("/tmp/b"),
                    bundle: empty_bundle("/tmp/b"),
                },
            ]),
        );
        let session = load_session_from(&path);
        assert!(session.workspace);
        assert_eq!(session.bundle.as_deref(), Some(Path::new("/tmp/a")));
        let _ = fs::remove_dir_all(dir);
    }

    fn empty_bundle(root: &str) -> okf::Bundle {
        okf::Bundle {
            root: PathBuf::from(root),
            version: None,
            concepts: Vec::new(),
            indexes: Vec::new(),
            logs: Vec::new(),
            graph: Vec::new(),
            diagnostics: Vec::new(),
        }
    }
}
