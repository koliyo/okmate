use okf::{Bundle, Concept, Diagnostic, Severity, TrustTier};

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
}

pub fn governance_stats(bundle: &Bundle) -> Vec<StatCard> {
    let total = bundle.concepts.len();
    let mut stable = 0;
    let mut draft = 0;
    let mut action = 0;
    let mut stale = 0;
    for concept in &bundle.concepts {
        let status = okf::string_field(&concept.metadata, "status").unwrap_or("draft");
        if okf::search::concept_is_stale(&concept.metadata) {
            stale += 1;
        }
        if status == "stable" {
            stable += 1;
        } else {
            draft += 1;
        }
        if okf::classify_concept_action(concept, &bundle.diagnostics).is_action_required {
            action += 1;
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
            value: bundle.diagnostics.len().to_string(),
            label: "Diagnostics".into(),
            tone: String::new(),
        },
    ]
}

pub fn recent_leaf_documents(bundle: &Bundle, limit: usize) -> Vec<RecentDoc> {
    let mut docs: Vec<&Concept> = bundle
        .concepts
        .iter()
        .filter(|concept| {
            !concept.path.ends_with("/index.md")
                && concept.path != "index.md"
                && !is_collection_id(bundle, &concept.id)
        })
        .collect();
    docs.sort_by(|left, right| {
        generated_at(right)
            .cmp(generated_at(left))
            .then_with(|| left.id.cmp(&right.id))
    });
    docs.into_iter()
        .take(limit)
        .map(|concept| {
            let title = okf::string_field(&concept.metadata, "title").unwrap_or(&concept.id);
            RecentDoc {
                href: format!("/{}/", concept.id.trim_matches('/')),
                title: title.to_string(),
                collection: longest_collection_prefix(bundle, &concept.id).to_string(),
            }
        })
        .collect()
}

pub fn diagnostic_rows(bundle: &Bundle) -> Vec<DiagnosticRow> {
    bundle
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
        .collect()
}

pub fn concept_meta(concept: &Concept, diagnostics: &[Diagnostic]) -> ConceptMeta {
    let trust = okf::search::concept_trust_tier(&concept.metadata);
    let (trust_slug, trust_label) = match trust {
        TrustTier::HumanReviewed => ("human", "human-reviewed"),
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
            provenance.push(ProvenanceItem {
                label: "Generated".into(),
                value: format!("{by} @ {at}"),
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
