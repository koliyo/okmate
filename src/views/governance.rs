use okf::{Bundle, Concept, Diagnostic, Severity, TrustTier};

use crate::workspace::Workspace;

#[derive(Clone, Debug, Default)]
pub struct StatCard {
    pub value: String,
    pub label: String,
    pub tone: String,
}

#[derive(Clone, Debug, Default)]
pub struct RecentDoc {
    pub href: String,
    pub title: String,
    pub collection: String,
    pub root: String,
}

#[derive(Clone, Debug, Default)]
pub struct LogEntry {
    pub text: String,
    pub root: String,
}

#[derive(Clone, Debug, Default)]
pub struct LogDay {
    pub date: String,
    pub entries: Vec<LogEntry>,
}

#[derive(Clone, Debug, Default)]
pub struct DiagnosticRow {
    pub severity: String,
    pub code: String,
    pub path: String,
    pub message: String,
}

#[derive(Clone, Debug, Default)]
pub struct ProvenanceItem {
    pub label: String,
    pub value: String,
    pub unverified: bool,
}

#[derive(Clone, Debug, Default)]
pub struct ConceptMeta {
    pub trust_slug: String,
    pub trust_label: String,
    pub stale: bool,
    pub stale_after: String,
    pub description: String,
    pub alert: String,
    pub provenance: Vec<ProvenanceItem>,
    pub drift_sources: Vec<String>,
    pub file_hash: String,
    pub root: String,
    pub href: String,
    pub id: String,
    pub show_verify: bool,
    pub show_promote: bool,
    pub author_enabled: bool,
    pub author_title: String,
}

pub fn governance_stats(workspace: &Workspace) -> Vec<StatCard> {
    let mut total = 0;
    let mut stable = 0;
    let mut draft = 0;
    let mut action = 0;
    let mut stale = 0;
    let mut diagnostics = 0;
    for member in workspace.members() {
        diagnostics += member.bundle.diagnostics.len();
        for concept in &member.bundle.concepts {
            total += 1;
            let status = okf::string_field(&concept.metadata, "status").unwrap_or("draft");
            if okf::search::concept_is_stale(&concept.metadata) {
                stale += 1;
            }
            if status == "stable" {
                stable += 1;
            } else {
                draft += 1;
            }
            if okf::classify_concept_action(concept, &member.bundle.diagnostics).is_action_required
            {
                action += 1;
            }
        }
    }
    vec![
        StatCard {
            value: total.to_string(),
            label: "Total".into(),
            tone: String::new(),
        },
        StatCard {
            value: stable.to_string(),
            label: "Stable".into(),
            tone: String::new(),
        },
        StatCard {
            value: draft.to_string(),
            label: "Draft".into(),
            tone: String::new(),
        },
        StatCard {
            value: action.to_string(),
            label: "Action".into(),
            tone: if action > 0 {
                "action".into()
            } else {
                String::new()
            },
        },
        StatCard {
            value: stale.to_string(),
            label: "Stale".into(),
            tone: String::new(),
        },
        StatCard {
            value: diagnostics.to_string(),
            label: "Diagnostics".into(),
            tone: String::new(),
        },
    ]
}

pub fn recent_leaf_documents(workspace: &Workspace, limit: usize) -> Vec<RecentDoc> {
    let mut docs = Vec::new();
    for member in workspace.members() {
        for concept in &member.bundle.concepts {
            if concept.path.ends_with("/index.md")
                || concept.path == "index.md"
                || is_collection_id(&member.bundle, &concept.id)
            {
                continue;
            }
            docs.push((member, concept));
        }
    }
    docs.sort_by(|left, right| {
        generated_at(right.1)
            .cmp(generated_at(left.1))
            .then_with(|| left.1.id.cmp(&right.1.id))
            .then_with(|| left.0.id.cmp(&right.0.id))
    });
    docs.into_iter()
        .take(limit)
        .map(|(member, concept)| {
            let title = okf::string_field(&concept.metadata, "title").unwrap_or(&concept.id);
            RecentDoc {
                href: workspace.document_href(&member.id, &concept.id),
                title: title.to_string(),
                collection: longest_collection_prefix(&member.bundle, &concept.id).to_string(),
                root: if workspace.is_multi() {
                    member.id.clone()
                } else {
                    String::new()
                },
            }
        })
        .collect()
}

pub fn merged_log(workspace: &Workspace) -> Vec<LogDay> {
    let mut by_date: std::collections::BTreeMap<String, Vec<LogEntry>> =
        std::collections::BTreeMap::new();
    for member in workspace.members() {
        for log in &member.bundle.logs {
            let path = member.path.join(&log.path);
            let Ok(body) = std::fs::read_to_string(&path) else {
                continue;
            };
            for (date, bullets) in parse_log_markdown(&body) {
                let entries = by_date.entry(date).or_default();
                for text in bullets {
                    entries.push(LogEntry {
                        text,
                        root: if workspace.is_multi() {
                            member.id.clone()
                        } else {
                            String::new()
                        },
                    });
                }
            }
        }
    }
    by_date
        .into_iter()
        .rev()
        .map(|(date, entries)| LogDay { date, entries })
        .collect()
}

pub const DASHBOARD_LOG_LIMIT: usize = 5;

pub fn take_log_entries(days: &[LogDay], limit: usize) -> Vec<LogDay> {
    let mut remaining = limit;
    let mut out = Vec::new();
    for day in days {
        if remaining == 0 {
            break;
        }
        let take = day.entries.len().min(remaining);
        out.push(LogDay {
            date: day.date.clone(),
            entries: day.entries[..take].to_vec(),
        });
        remaining -= take;
    }
    out
}

pub fn review_needs_attention(workspace: &Workspace) -> bool {
    workspace.members().iter().any(|member| {
        member.bundle.concepts.iter().any(|concept| {
            okf::classify_concept_action(concept, &member.bundle.diagnostics).is_action_required
        })
    })
}

pub fn parse_log_markdown(body: &str) -> Vec<(String, Vec<String>)> {
    let mut days = Vec::new();
    let mut current: Option<(String, Vec<String>)> = None;
    for line in body.lines() {
        let trimmed = line.trim();
        if let Some(date) = trimmed.strip_prefix("## ") {
            let date = date.trim();
            if is_log_date(date) {
                if let Some(day) = current.take() {
                    days.push(day);
                }
                current = Some((date.to_string(), Vec::new()));
                continue;
            }
        }
        let Some((_, bullets)) = current.as_mut() else {
            continue;
        };
        let bullet = trimmed
            .strip_prefix("- ")
            .or_else(|| trimmed.strip_prefix("* "));
        if let Some(text) = bullet {
            let text = text.trim();
            if !text.is_empty() {
                bullets.push(text.to_string());
            }
        }
    }
    if let Some(day) = current {
        days.push(day);
    }
    days
}

fn is_log_date(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes.iter().enumerate().all(|(index, byte)| {
            if index == 4 || index == 7 {
                true
            } else {
                byte.is_ascii_digit()
            }
        })
}

pub fn diagnostic_rows(workspace: &Workspace) -> Vec<DiagnosticRow> {
    workspace
        .members()
        .iter()
        .flat_map(|member| {
            member
                .bundle
                .diagnostics
                .iter()
                .map(|diagnostic| DiagnosticRow {
                    severity: match diagnostic.severity {
                        Severity::Error => "Error".into(),
                        Severity::Warning => "Warning".into(),
                    },
                    code: diagnostic.code.to_string(),
                    path: diagnostic.path.clone(),
                    message: diagnostic.message.clone(),
                })
        })
        .collect()
}

pub fn concept_meta(concept: &Concept, diagnostics: &[Diagnostic]) -> ConceptMeta {
    let trust = okf::search::concept_trust_tier(&concept.metadata);
    let (trust_slug, trust_label) = match trust {
        TrustTier::HumanReviewed => ("human", "reviewed"),
        TrustTier::Generated => ("generated", "generated"),
        TrustTier::Unverified => ("unverified", "unverified"),
    };
    let stale = okf::search::concept_is_stale(&concept.metadata);
    let stale_after = okf::string_field(&concept.metadata, "stale_after").unwrap_or("");
    let action = okf::classify_concept_action(concept, diagnostics);
    let owners = okf::metadata_string_array(&concept.metadata, "owners");
    let generated = concept
        .metadata
        .get("generated")
        .and_then(|value| value.as_object());
    let mut provenance = Vec::new();
    if !owners.is_empty() {
        provenance.push(ProvenanceItem {
            label: "Owners".into(),
            value: owners.join(", "),
            unverified: false,
        });
    }
    if let Some((_, verifier)) = okf::latest_human_verification(&concept.metadata) {
        provenance.push(ProvenanceItem {
            label: "Verified".into(),
            value: verifier.to_string(),
            unverified: false,
        });
    } else {
        provenance.push(ProvenanceItem {
            label: "Verified".into(),
            value: "Unverified".into(),
            unverified: true,
        });
    }
    if let Some(generated) = generated {
        let by = generated
            .get("by")
            .and_then(|value| value.as_str())
            .unwrap_or("");
        let at = generated
            .get("at")
            .and_then(|value| value.as_str())
            .unwrap_or("");
        if !by.is_empty() {
            let value = if at.len() >= 10 {
                format!("{by} · {}", &at[..10])
            } else {
                by.to_string()
            };
            provenance.push(ProvenanceItem {
                label: "Generated".into(),
                value,
                unverified: false,
            });
        }
    }
    if !stale_after.is_empty() {
        provenance.push(ProvenanceItem {
            label: "Stale after".into(),
            value: stale_after.to_string(),
            unverified: false,
        });
    }
    let drift_sources = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.path == concept.path && diagnostic.code == "OKF4006")
        .map(|diagnostic| diagnostic.message.clone())
        .collect();
    ConceptMeta {
        trust_slug: trust_slug.into(),
        trust_label: trust_label.into(),
        stale,
        stale_after: stale_after.to_string(),
        description: okf::string_field(&concept.metadata, "description")
            .unwrap_or("")
            .to_string(),
        alert: if action.is_action_required {
            action.detail
        } else {
            String::new()
        },
        provenance,
        drift_sources,
        file_hash: String::new(),
        root: String::new(),
        href: String::new(),
        id: String::new(),
        show_verify: false,
        show_promote: false,
        author_enabled: false,
        author_title: String::new(),
    }
}

fn generated_at(concept: &Concept) -> &str {
    concept
        .metadata
        .get("generated")
        .and_then(|value| value.get("at"))
        .and_then(|value| value.as_str())
        .unwrap_or("")
}

fn longest_collection_prefix<'a>(bundle: &'a Bundle, concept_id: &str) -> &'a str {
    bundle
        .indexes
        .iter()
        .filter_map(|index| index.path.strip_suffix("/index.md"))
        .filter(|name| concept_id == *name || concept_id.starts_with(&format!("{name}/")))
        .max_by_key(|name| name.len())
        .unwrap_or("")
}

fn is_collection_id(bundle: &Bundle, id: &str) -> bool {
    bundle
        .indexes
        .iter()
        .any(|index| index.path == format!("{id}/index.md"))
}

#[cfg(test)]
mod tests {
    use super::{LogDay, LogEntry, parse_log_markdown, take_log_entries};

    #[test]
    fn parse_log_markdown_collects_dated_bullets() {
        let days = parse_log_markdown(
            "# Knowledge log\n\nIntro.\n\n## 2026-08-20\n\n- First day.\n\n## 2026-08-21\n\n- Newer.\n- Also newer.\n",
        );
        assert_eq!(days.len(), 2);
        assert_eq!(days[0].0, "2026-08-20");
        assert_eq!(days[0].1, vec!["First day."]);
        assert_eq!(days[1].0, "2026-08-21");
        assert_eq!(days[1].1, vec!["Newer.", "Also newer."]);
    }

    #[test]
    fn take_log_entries_keeps_newest_days_within_limit() {
        let days = vec![
            LogDay {
                date: "2026-08-21".into(),
                entries: vec![
                    LogEntry {
                        text: "Newest".into(),
                        root: String::new(),
                    },
                    LogEntry {
                        text: "Also new".into(),
                        root: String::new(),
                    },
                ],
            },
            LogDay {
                date: "2026-08-20".into(),
                entries: vec![LogEntry {
                    text: "Older".into(),
                    root: String::new(),
                }],
            },
        ];
        let limited = take_log_entries(&days, 2);
        assert_eq!(limited.len(), 1);
        assert_eq!(limited[0].date, "2026-08-21");
        assert_eq!(limited[0].entries.len(), 2);
        let split = take_log_entries(&days, 3);
        assert_eq!(split.len(), 2);
        assert_eq!(split[1].entries[0].text, "Older");
    }
}
