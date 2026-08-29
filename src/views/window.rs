use serde::Deserialize;

use super::{Document, LogDay, LogEntry, ReviewRow};

pub const REVIEW_WINDOW: usize = 40;
pub const REVIEW_OVERSCAN: usize = 40;
pub const LOG_WINDOW: usize = 20;
pub const LOG_OVERSCAN: usize = 20;
pub const ROW_HEIGHT_PX: usize = 48;

#[derive(Clone, Debug, Default)]
pub struct ListWindow {
    pub enabled: bool,
    pub total: usize,
    pub offset: usize,
    pub before: usize,
    pub after: usize,
    pub filter: String,
    pub query: String,
    pub prev_start: usize,
    pub next_start: usize,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct WindowQuery {
    #[serde(default, alias = "window")]
    pub start: usize,
    #[serde(default)]
    pub filter: String,
    #[serde(default)]
    pub q: String,
}

impl WindowQuery {
    pub fn from_raw(raw: Option<&str>) -> Self {
        serde_urlencoded::from_str(raw.unwrap_or("")).unwrap_or_default()
    }
}

pub fn apply_review_window(document: &mut Document, query: &WindowQuery) {
    let filtered: Vec<ReviewRow> = document
        .review_rows
        .iter()
        .filter(|row| matches_review(row, query))
        .cloned()
        .collect();
    let (slice, window) = slice_window(
        filtered.len(),
        query.start,
        REVIEW_WINDOW,
        REVIEW_OVERSCAN,
        query,
    );
    document.review_window = window;
    document.review_rows = filtered.into_iter().skip(slice.0).take(slice.1).collect();
}

pub fn apply_log_window(document: &mut Document, query: &WindowQuery) {
    let mut flat = Vec::new();
    for day in &document.log_days {
        for entry in &day.entries {
            flat.push((day.date.clone(), entry.clone()));
        }
    }
    let (range, window) = slice_window(flat.len(), query.start, LOG_WINDOW, LOG_OVERSCAN, query);
    document.log_window = window;
    let sliced = &flat[range.0..range.0 + range.1];
    let mut days: Vec<LogDay> = Vec::new();
    for (date, entry) in sliced {
        if days.last().map(|day| day.date.as_str()) != Some(date.as_str()) {
            days.push(LogDay {
                date: date.clone(),
                entries: Vec::new(),
            });
        }
        if let Some(day) = days.last_mut() {
            day.entries.push(LogEntry {
                text: entry.text.clone(),
                root: entry.root.clone(),
            });
        }
    }
    document.log_days = days;
}

fn matches_review(row: &ReviewRow, query: &WindowQuery) -> bool {
    let filter_ok = match query.filter.as_str() {
        "action" => row.is_action_required,
        "draft" => row.status == "draft",
        "stable" => row.status == "stable",
        _ => true,
    };
    let needle = query.q.to_ascii_lowercase();
    filter_ok && (needle.is_empty() || row.search.contains(&needle))
}

fn slice_window(
    total: usize,
    start: usize,
    window: usize,
    overscan: usize,
    query: &WindowQuery,
) -> ((usize, usize), ListWindow) {
    let start = start.min(total);
    let from = start.saturating_sub(overscan);
    let to = (start + window + overscan).min(total);
    let len = to.saturating_sub(from);
    (
        (from, len),
        ListWindow {
            enabled: true,
            total,
            offset: from,
            before: from,
            after: total.saturating_sub(to),
            filter: query.filter.clone(),
            query: query.q.clone(),
            prev_start: from.saturating_sub(window),
            next_start: (from + len).min(total),
        },
    )
}
