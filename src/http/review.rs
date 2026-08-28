use std::collections::BTreeMap;
use std::convert::Infallible;
use std::fs;
use std::net::SocketAddr;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::anyhow;
use axum::extract::{ConnectInfo, Form, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response, Sse, sse::Event};
use datastar::prelude::PatchElements;
use futures_util::stream;
use okf::{ActionKind, Concept};

use crate::author::{append_verification, file_hash, set_status};
use crate::config::valid_actor;
use crate::http::AppState;
use crate::workspace::{Workspace, WorkspaceMember};

pub async fn post(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(fields): Form<BTreeMap<String, String>>,
) -> Response {
    if !addr.ip().is_loopback() {
        return (StatusCode::FORBIDDEN, "review POST is loopback-only").into_response();
    }
    match apply(&state, &fields) {
        Ok(()) => morph(&state, &headers, &fields),
        Err(error) => error.into_response(),
    }
}

fn apply(state: &AppState, fields: &BTreeMap<String, String>) -> Result<(), ReviewError> {
    let action = fields.get("action").map(String::as_str).unwrap_or("");
    let concept_id = required(fields, "concept")?;
    let expected_hash = required(fields, "hash")?;
    let root = fields
        .get("root")
        .map(String::as_str)
        .filter(|value| !value.is_empty());

    let config = crate::config::load_or_default(&state.config_path);
    let actor = config
        .actor
        .as_deref()
        .filter(|value| valid_actor(value))
        .ok_or(ReviewError::BadRequest(
            "reviewer actor is required; set a human: actor in Settings".into(),
        ))?;

    let workspace = state.workspace.read().expect("workspace lock");
    let (member, concept) = find_concept(&workspace, root, concept_id)?;
    if crate::roots::is_git_cache_snapshot(&member.path) {
        return Err(ReviewError::Forbidden(
            "git-cache snapshots are read-only".into(),
        ));
    }
    let Some(_) = okf::git_repository_root(&member.path) else {
        return Err(ReviewError::Forbidden(
            "authoring requires a git working tree".into(),
        ));
    };
    let path = member.path.join(&concept.path);
    let bytes = fs::read(&path).map_err(|error| {
        ReviewError::Internal(anyhow!("failed to read {}: {error}", path.display()))
    })?;
    if file_hash(&bytes) != expected_hash {
        return Err(ReviewError::Conflict(
            "concept changed; reload and try again".into(),
        ));
    }
    let source = String::from_utf8(bytes)
        .map_err(|_| ReviewError::BadRequest("concept file is not valid UTF-8".into()))?;
    let next = match action {
        "verify" => append_verification(&source, actor, &utc_rfc3339())
            .map_err(|error| ReviewError::BadRequest(error.to_string()))?,
        "promote" => promote(&source)?,
        other => {
            return Err(ReviewError::BadRequest(format!(
                "unknown review action `{other}`"
            )));
        }
    };
    drop(workspace);
    fs::write(&path, next.as_bytes()).map_err(|error| {
        ReviewError::Internal(anyhow!("failed to write {}: {error}", path.display()))
    })?;
    reload(state)?;
    Ok(())
}

fn promote(source: &str) -> Result<String, ReviewError> {
    let metadata = parse_metadata(source)?;
    let status = okf::string_field(&metadata, "status").unwrap_or("draft");
    let authority = okf::string_field(&metadata, "authority").unwrap_or("descriptive");
    if authority == "exploratory" {
        return Err(ReviewError::BadRequest(
            "cannot promote an exploratory record".into(),
        ));
    }
    if status != "draft" {
        return Err(ReviewError::BadRequest(format!(
            "promote requires status draft, got `{status}`"
        )));
    }
    if okf::latest_human_verification(&metadata).is_none() {
        return Err(ReviewError::BadRequest(
            "promote requires a human: verification event".into(),
        ));
    }
    set_status(source, "draft", "stable")
        .map_err(|error| ReviewError::BadRequest(error.to_string()))
}

fn parse_metadata(
    source: &str,
) -> Result<std::collections::BTreeMap<String, serde_json::Value>, ReviewError> {
    let frontmatter = okf::split_frontmatter(source, true)
        .map_err(ReviewError::BadRequest)?
        .ok_or_else(|| ReviewError::BadRequest("concept requires YAML frontmatter".into()))?;
    okf::parse_yaml_mapping(frontmatter.yaml.of(source)).map_err(ReviewError::BadRequest)
}

fn find_concept<'a>(
    workspace: &'a Workspace,
    root: Option<&str>,
    id: &str,
) -> Result<(&'a WorkspaceMember, &'a Concept), ReviewError> {
    if let Some(root) = root {
        let member = workspace
            .get(root)
            .ok_or_else(|| ReviewError::BadRequest(format!("unknown knowledge root `{root}`")))?;
        let concept = member
            .bundle
            .concepts
            .iter()
            .find(|concept| concept.id == id || concept.id.rsplit('/').next() == Some(id))
            .ok_or_else(|| ReviewError::BadRequest(format!("unknown concept `{id}`")))?;
        return Ok((member, concept));
    }
    let matches: Vec<_> = workspace
        .members()
        .iter()
        .filter_map(|member| {
            member
                .bundle
                .concepts
                .iter()
                .find(|concept| concept.id == id || concept.id.rsplit('/').next() == Some(id))
                .map(|concept| (member, concept))
        })
        .collect();
    match matches.as_slice() {
        [(member, concept)] => Ok((member, concept)),
        [] => Err(ReviewError::BadRequest(format!("unknown concept `{id}`"))),
        _ => Err(ReviewError::BadRequest(format!(
            "ambiguous concept `{id}`; pass root"
        ))),
    }
}

fn reload(state: &AppState) -> Result<(), ReviewError> {
    let reloaded = {
        let workspace = state.workspace.read().expect("workspace lock");
        workspace
            .reload(state.profile)
            .map_err(ReviewError::Internal)?
    };
    state.replace_workspace(reloaded);
    Ok(())
}

fn morph(state: &AppState, headers: &HeaderMap, fields: &BTreeMap<String, String>) -> Response {
    let path = fields
        .get("return")
        .filter(|value| value.starts_with('/') && !value.starts_with("/__okmate"))
        .cloned()
        .or_else(|| {
            headers
                .get(axum::http::header::REFERER)
                .and_then(|value| value.to_str().ok())
                .and_then(super::referer_path)
        })
        .unwrap_or_else(|| "/review/".into());
    let Some(document) = super::pages::live_document(state, &path, None) else {
        return (StatusCode::NOT_FOUND, "review page not found").into_response();
    };
    if super::pages::is_datastar(headers) {
        let fragment = match document.render_main_fragment() {
            Ok(fragment) => fragment,
            Err(error) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response();
            }
        };
        let patch = PatchElements::new(fragment);
        return Sse::new(stream::once(async move {
            Ok::<Event, Infallible>(patch.write_as_axum_sse_event())
        }))
        .into_response();
    }
    match crate::site::render_document(document) {
        Ok(html) => axum::response::Html(html).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}

fn required<'a>(fields: &'a BTreeMap<String, String>, key: &str) -> Result<&'a str, ReviewError> {
    fields
        .get(key)
        .map(String::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| ReviewError::BadRequest(format!("missing {key}")))
}

fn utc_rfc3339() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or(0);
    let date = civil_date(secs);
    let tod = secs.rem_euclid(86_400);
    format!(
        "{date}T{:02}:{:02}:{:02}Z",
        tod / 3_600,
        (tod % 3_600) / 60,
        tod % 60
    )
}

fn civil_date(secs: i64) -> String {
    let days = secs.div_euclid(86_400) + 719_468;
    let era = if days >= 0 { days } else { days - 146_096 } / 146_097;
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02}")
}

enum ReviewError {
    Forbidden(String),
    Conflict(String),
    BadRequest(String),
    Internal(anyhow::Error),
}

impl IntoResponse for ReviewError {
    fn into_response(self) -> Response {
        match self {
            Self::Forbidden(message) => (StatusCode::FORBIDDEN, message).into_response(),
            Self::Conflict(message) => (StatusCode::CONFLICT, message).into_response(),
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, message).into_response(),
            Self::Internal(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
            }
        }
    }
}

pub fn author_enabled(path: &Path, actor_ok: bool) -> (bool, String) {
    if crate::roots::is_git_cache_snapshot(path) {
        return (false, "Fetched git-cache roots are read-only".into());
    }
    let git_ok = okf::git_repository_root(path).is_some();
    match (git_ok, actor_ok) {
        (true, true) => (true, String::new()),
        (false, false) => (
            false,
            "Needs a git working tree and a human: actor in Settings".into(),
        ),
        (false, true) => (false, "Authoring requires a git working tree".into()),
        (true, false) => (
            false,
            "Set a human: actor in Settings before verifying".into(),
        ),
    }
}

pub fn verify_kinds(kind: &ActionKind) -> bool {
    matches!(
        kind,
        ActionKind::InitialVerification
            | ActionKind::PendingPromotion
            | ActionKind::ReverifySources
            | ActionKind::ReverifyRegenerated
    )
}

pub fn promote_kind(kind: &ActionKind) -> bool {
    *kind == ActionKind::PendingPromotion
}
