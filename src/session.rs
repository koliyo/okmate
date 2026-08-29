use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

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

const DEFAULT_FONT: u16 = 100;
const MIN_FONT: u16 = 80;
const MAX_FONT: u16 = 160;
const FONT_STEP: u16 = 10;
const MIN_CH: u16 = 45;
const MAX_CH: u16 = 100;
const DEFAULT_CH: u16 = 66;

fn default_true() -> bool {
    true
}

fn default_font_size() -> u16 {
    DEFAULT_FONT
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
    #[serde(default)]
    pub bundle: Option<PathBuf>,
    #[serde(default)]
    pub workspace: bool,
    #[serde(default)]
    pub nav_mode: NavMode,
    #[serde(default = "default_font_size")]
    pub font_size: u16,
    #[serde(default)]
    pub main_width: Option<u16>,
    #[serde(default = "default_true")]
    pub wrap: bool,
    #[serde(default)]
    pub full_width: bool,
    #[serde(default = "default_true")]
    pub nav_visible: bool,
    #[serde(default = "default_true")]
    pub toc_visible: bool,
    #[serde(default)]
    pub nav_width: Option<String>,
    #[serde(default)]
    pub outline_width: Option<String>,
    #[serde(default)]
    pub open_path: Option<String>,
    #[serde(default)]
    pub open_hash: Option<String>,
    #[serde(default)]
    pub main_scroll: Option<u32>,
    #[serde(default)]
    pub nav_sections: BTreeMap<String, bool>,
    #[serde(default)]
    pub nav_scroll: Option<u32>,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            bundle: None,
            workspace: false,
            nav_mode: NavMode::default(),
            font_size: DEFAULT_FONT,
            main_width: None,
            wrap: true,
            full_width: false,
            nav_visible: true,
            toc_visible: true,
            nav_width: None,
            outline_width: None,
            open_path: None,
            open_hash: None,
            main_scroll: None,
            nav_sections: BTreeMap::new(),
            nav_scroll: None,
        }
    }
}

impl Session {
    pub fn merge_prefs(&mut self, value: &serde_json::Value) {
        let Some(obj) = value.as_object() else {
            return;
        };
        if let Some(size) = obj.get("font_size").and_then(as_u16) {
            self.font_size = clamp_font(size);
        }
        if obj.contains_key("main_width") {
            self.main_width = obj.get("main_width").and_then(as_u16).map(clamp_ch);
        }
        if let Some(wrap) = obj.get("wrap").and_then(serde_json::Value::as_bool) {
            self.wrap = wrap;
        }
        if let Some(full) = obj.get("full_width").and_then(serde_json::Value::as_bool) {
            self.full_width = full;
        }
        if let Some(visible) = obj.get("nav_visible").and_then(serde_json::Value::as_bool) {
            self.nav_visible = visible;
        }
        if let Some(visible) = obj.get("toc_visible").and_then(serde_json::Value::as_bool) {
            self.toc_visible = visible;
        }
        if obj.contains_key("nav_width") {
            self.nav_width =
                sanitize_track(obj.get("nav_width").and_then(serde_json::Value::as_str));
        }
        if obj.contains_key("outline_width") {
            self.outline_width =
                sanitize_track(obj.get("outline_width").and_then(serde_json::Value::as_str));
        }
        if obj.contains_key("open_path") {
            self.open_path = sanitize_open_path(obj.get("open_path").and_then(as_opt_str));
        }
        if obj.contains_key("open_hash") {
            self.open_hash = sanitize_open_hash(obj.get("open_hash").and_then(as_opt_str));
        }
        if obj.contains_key("main_scroll") {
            self.main_scroll = obj.get("main_scroll").and_then(as_u32).map(clamp_scroll);
        }
        if let Some(map) = obj.get("nav_sections").and_then(|value| value.as_object()) {
            let mut sections = BTreeMap::new();
            for (key, value) in map {
                let Some(key) = sanitize_nav_section_key(key) else {
                    continue;
                };
                let Some(open) = value.as_bool() else {
                    continue;
                };
                sections.insert(key, open);
                if sections.len() >= MAX_NAV_SECTIONS {
                    break;
                }
            }
            self.nav_sections = sections;
        }
        if obj.contains_key("nav_scroll") {
            self.nav_scroll = obj.get("nav_scroll").and_then(as_u32).map(clamp_scroll);
        }
    }

    pub fn sanitize(&mut self) {
        self.font_size = clamp_font(self.font_size);
        if let Some(width) = self.main_width {
            self.main_width = Some(clamp_ch(width));
        }
        self.nav_width = sanitize_track(self.nav_width.as_deref());
        self.outline_width = sanitize_track(self.outline_width.as_deref());
        self.open_path = sanitize_open_path(self.open_path.as_deref());
        self.open_hash = sanitize_open_hash(self.open_hash.as_deref());
        if let Some(scroll) = self.main_scroll {
            self.main_scroll = Some(clamp_scroll(scroll));
        }
        self.nav_sections
            .retain(|key, _| sanitize_nav_section_key(key).is_some());
        if self.nav_sections.len() > MAX_NAV_SECTIONS {
            self.nav_sections = self
                .nav_sections
                .iter()
                .take(MAX_NAV_SECTIONS)
                .map(|(key, open)| (key.clone(), *open))
                .collect();
        }
        if let Some(scroll) = self.nav_scroll {
            self.nav_scroll = Some(clamp_scroll(scroll));
        }
    }

    pub fn location_href(&self) -> Option<String> {
        let path = self.open_path.as_deref()?;
        Some(match self.open_hash.as_deref() {
            Some(hash) if !hash.is_empty() => format!("{path}#{hash}"),
            _ => path.to_string(),
        })
    }

    pub fn html_style(&self) -> String {
        let mut parts = Vec::new();
        if self.font_size != DEFAULT_FONT {
            parts.push(format!("font-size: {}%", self.font_size));
        }
        if let Some(ch) = self.main_width {
            parts.push(format!("--okmate-main-max-width: {ch}ch"));
        }
        if let Some(width) = &self.nav_width {
            parts.push(format!("--okmate-nav-width: {width}"));
        }
        if let Some(width) = &self.outline_width {
            parts.push(format!("--okmate-outline-width: {width}"));
        }
        parts.join("; ")
    }

    pub fn reading_width(&self) -> u16 {
        self.main_width.unwrap_or(DEFAULT_CH)
    }
}

fn as_u16(value: &serde_json::Value) -> Option<u16> {
    value
        .as_u64()
        .and_then(|n| u16::try_from(n).ok())
        .or_else(|| value.as_i64().and_then(|n| u16::try_from(n).ok()))
}

fn as_u32(value: &serde_json::Value) -> Option<u32> {
    value
        .as_u64()
        .and_then(|n| u32::try_from(n).ok())
        .or_else(|| value.as_i64().and_then(|n| u32::try_from(n).ok()))
}

fn as_opt_str(value: &serde_json::Value) -> Option<&str> {
    value.as_str()
}

const MAX_SCROLL: u32 = 10_000_000;

fn clamp_scroll(value: u32) -> u32 {
    value.min(MAX_SCROLL)
}

pub(crate) fn sanitize_open_path(value: Option<&str>) -> Option<String> {
    let raw = value?.trim();
    if raw.is_empty() {
        return None;
    }
    let path = raw.split(['?', '#']).next().unwrap_or(raw);
    if path.contains("..") || path.contains('\\') || path.starts_with("/__okmate") {
        return None;
    }
    if !path.starts_with('/') {
        return None;
    }
    if !path.bytes().all(|byte| {
        byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'-' | b'_' | b'.' | b'@')
    }) {
        return None;
    }
    Some(crate::workspace::normalize_route(path))
}

const MAX_NAV_SECTIONS: usize = 500;

fn sanitize_nav_section_key(key: &str) -> Option<String> {
    let key = key.trim();
    if key.is_empty() || key.len() > 200 || key.contains("..") {
        return None;
    }
    if !key
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'-' | b'_' | b'@'))
    {
        return None;
    }
    Some(key.to_string())
}

fn sanitize_open_hash(value: Option<&str>) -> Option<String> {
    let raw = value?.trim().trim_start_matches('#');
    if raw.is_empty() || raw.len() > 200 {
        return None;
    }
    if !raw
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
    {
        return None;
    }
    Some(raw.to_string())
}

pub fn persist_open_path_to(path: &Path, route: &str) {
    let Some(open_path) = sanitize_open_path(Some(route)) else {
        return;
    };
    let mut session = load_session_from(path);
    if session.open_path.as_deref() == Some(open_path.as_str()) {
        return;
    }
    session.open_path = Some(open_path);
    session.open_hash = None;
    session.main_scroll = None;
    log_write(path, &session);
}

fn clamp_font(value: u16) -> u16 {
    let clamped = value.clamp(MIN_FONT, MAX_FONT);
    let stepped = ((clamped + FONT_STEP / 2) / FONT_STEP) * FONT_STEP;
    stepped.clamp(MIN_FONT, MAX_FONT)
}

fn clamp_ch(value: u16) -> u16 {
    value.clamp(MIN_CH, MAX_CH)
}

fn sanitize_track(value: Option<&str>) -> Option<String> {
    let value = value?.trim();
    if value.is_empty() {
        return None;
    }
    let bytes = value.as_bytes();
    let mut i = 0;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i == 0 {
        return None;
    }
    if i < bytes.len() && bytes[i] == b'.' {
        i += 1;
        let frac = i;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        if i == frac {
            return None;
        }
    }
    match &value[i..] {
        "px" | "rem" => Some(value.to_string()),
        _ => None,
    }
}
pub fn state_dir() -> PathBuf {
    crate::paths::state_dir()
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
    log_write(path, &session);
}

pub fn persist_workspace_to(path: &Path, workspace: &Workspace) {
    let mut session = load_session_from(path);
    session.workspace = workspace.is_multi();
    session.bundle = workspace.primary_path().map(Path::to_path_buf);
    log_write(path, &session);
}

pub fn persist_nav_mode_to(path: &Path, nav_mode: NavMode) {
    let mut session = load_session_from(path);
    session.nav_mode = nav_mode;
    log_write(path, &session);
}

pub fn persist_prefs_to(path: &Path, patch: &serde_json::Value) {
    let mut session = load_session_from(path);
    session.merge_prefs(patch);
    log_write(path, &session);
}

pub fn write_session(path: &Path, session: &Session) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    let json = serde_json::to_string_pretty(session).context("failed to encode session")?;
    let tmp = path.with_extension("tmp");
    fs::write(&tmp, json).with_context(|| format!("failed to write {}", tmp.display()))?;
    fs::rename(&tmp, path).with_context(|| format!("failed to replace {}", path.display()))?;
    Ok(())
}

fn log_write(path: &Path, session: &Session) {
    if let Err(error) = write_session(path, session) {
        eprintln!("okmate: failed to write {}: {error:#}", path.display());
    }
}

pub fn load_session() -> Session {
    load_session_from(&session_path())
}

pub fn load_session_from(path: &Path) -> Session {
    let Ok(content) = fs::read_to_string(path) else {
        return Session::default();
    };
    let mut session: Session = serde_json::from_str(&content).unwrap_or_default();
    session.sanitize();
    session
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    use crate::workspace::Workspace;

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
        assert_eq!(session.font_size, DEFAULT_FONT);
        assert!(session.wrap);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn old_session_json_keeps_reading_defaults() {
        let dir = std::env::temp_dir().join(format!(
            "okmate-state-{}-{}",
            std::process::id(),
            "prefs-old"
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("session.json");
        fs::write(
            &path,
            r#"{"bundle":"/tmp/knowledge","workspace":false,"nav_mode":"merged"}"#,
        )
        .unwrap();
        let session = load_session_from(&path);
        assert_eq!(session.nav_mode, NavMode::Merged);
        assert_eq!(session.font_size, DEFAULT_FONT);
        assert_eq!(session.main_width, None);
        assert!(session.wrap && session.nav_visible && session.toc_visible);
        assert!(session.nav_sections.is_empty());
        assert!(session.nav_scroll.is_none());
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn persist_prefs_clamps_and_rejects_css() {
        let dir = std::env::temp_dir().join(format!(
            "okmate-state-{}-{}",
            std::process::id(),
            "prefs-clamp"
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("session.json");
        persist_prefs_to(
            &path,
            &serde_json::json!({
                "font_size": 113,
                "main_width": 70,
                "wrap": false,
                "nav_visible": false,
                "toc_visible": true,
                "nav_width": "264px",
                "outline_width": "url(evil)"
            }),
        );
        let session = load_session_from(&path);
        assert_eq!(session.font_size, 110);
        assert_eq!(session.main_width, Some(70));
        assert!(!session.wrap);
        assert!(!session.nav_visible);
        assert!(session.toc_visible);
        assert_eq!(session.nav_width.as_deref(), Some("264px"));
        assert_eq!(session.outline_width, None);
        assert!(session.html_style().contains("font-size: 110%"));
        assert!(
            session
                .html_style()
                .contains("--okmate-main-max-width: 70ch")
        );
        persist_prefs_to(&path, &serde_json::json!({ "main_width": null }));
        let session = load_session_from(&path);
        assert_eq!(session.main_width, None);
        assert_eq!(session.font_size, 110);
        persist_prefs_to(
            &path,
            &serde_json::json!({
                "nav_sections": { "plans": true, "../evil": false, "okmate/plans": false },
                "nav_scroll": 80
            }),
        );
        let session = load_session_from(&path);
        assert_eq!(session.nav_sections.get("plans"), Some(&true));
        assert_eq!(session.nav_sections.get("okmate/plans"), Some(&false));
        assert!(!session.nav_sections.contains_key("../evil"));
        assert_eq!(session.nav_scroll, Some(80));
        persist_prefs_to(&path, &serde_json::json!({ "font_size": 100 }));
        let session = load_session_from(&path);
        assert_eq!(session.nav_sections.get("plans"), Some(&true));
        assert_eq!(session.nav_scroll, Some(80));
        persist_prefs_to(
            &path,
            &serde_json::json!({
                "open_path": "/plans/cli-entry-points/#details",
                "open_hash": "details",
                "main_scroll": 240
            }),
        );
        let session = load_session_from(&path);
        assert_eq!(
            session.open_path.as_deref(),
            Some("/plans/cli-entry-points/")
        );
        assert_eq!(session.open_hash.as_deref(), Some("details"));
        assert_eq!(session.main_scroll, Some(240));
        assert_eq!(
            session.location_href().as_deref(),
            Some("/plans/cli-entry-points/#details")
        );
        persist_prefs_to(
            &path,
            &serde_json::json!({
                "open_path": "/__okmate/evil",
                "open_hash": "bad hash",
                "main_scroll": 99_000_000
            }),
        );
        let session = load_session_from(&path);
        assert_eq!(session.open_path, None);
        assert_eq!(session.open_hash, None);
        assert_eq!(session.main_scroll, Some(MAX_SCROLL));
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn persist_open_path_clears_stale_location() {
        let dir = std::env::temp_dir().join(format!(
            "okmate-state-{}-{}",
            std::process::id(),
            "open-path"
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("session.json");
        persist_prefs_to(
            &path,
            &serde_json::json!({
                "open_path": "/hello/",
                "open_hash": "body",
                "main_scroll": 12
            }),
        );
        persist_open_path_to(&path, "/review/");
        let session = load_session_from(&path);
        assert_eq!(session.open_path.as_deref(), Some("/review/"));
        assert!(session.open_hash.is_none());
        assert!(session.main_scroll.is_none());
        persist_open_path_to(&path, "/review/");
        let again = load_session_from(&path);
        assert_eq!(again.open_path.as_deref(), Some("/review/"));
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
