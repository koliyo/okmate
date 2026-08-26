use askama::Template;
use okf::Bundle;

mod governance;

pub use governance::{
    ConceptMeta, DiagnosticRow, ProvenanceItem, RecentDoc, StatCard, concept_meta, diagnostic_rows,
    governance_stats, recent_leaf_documents,
};

#[derive(Clone, Debug, Default)]
pub struct NavNode {
    pub href: String,
    pub title: String,
    pub current: bool,
    pub open: bool,
    pub children: Vec<NavNode>,
}

#[derive(Clone, Debug)]
pub struct TocEntry {
    pub id: String,
    pub text: String,
    pub level: u8,
}

#[derive(Clone, Debug)]
pub struct SettingsRoot {
    pub id: String,
    pub kind: String,
    pub detail: String,
    pub incoming: String,
    pub token_env: String,
    pub has_token: bool,
    pub warning: String,
}

#[derive(Clone, Debug, Default)]
pub struct ReviewRow {
    pub href: String,
    pub title: String,
    pub id: String,
    pub status: String,
    pub action: String,
    pub concept_type: String,
    pub authority: String,
    pub trust_slug: String,
    pub trust_label: String,
    pub verifier: String,
    pub action_detail: String,
    pub pill_class: String,
    pub is_action_required: bool,
    pub search: String,
}

macro_rules! document_template {
    ($name:ident, $path:literal) => {
        #[derive(Template)]
        #[template(path = $path)]
        pub struct $name {
            pub title: String,
            pub page_kind: String,
            pub nav: Vec<NavNode>,
            pub toc: Vec<TocEntry>,
            pub article_html: String,
            pub concept_type: String,
            pub status: String,
            pub authority: String,
            pub review_rows: Vec<ReviewRow>,
            pub stats: Vec<StatCard>,
            pub recents: Vec<RecentDoc>,
            pub diagnostics: Vec<DiagnosticRow>,
            pub meta: ConceptMeta,
            pub message: String,
            pub config_path: String,
            pub settings_roots: Vec<SettingsRoot>,
        }

        impl From<Document> for $name {
            fn from(document: Document) -> Self {
                Self {
                    title: document.title,
                    page_kind: document.page_kind,
                    nav: document.nav,
                    toc: document.toc,
                    article_html: document.article_html,
                    concept_type: document.concept_type,
                    status: document.status,
                    authority: document.authority,
                    review_rows: document.review_rows,
                    stats: document.stats,
                    recents: document.recents,
                    diagnostics: document.diagnostics,
                    meta: document.meta,
                    message: document.message,
                    config_path: document.config_path,
                    settings_roots: document.settings_roots,
                }
            }
        }
    };
}

document_template!(PageTemplate, "page.html");
document_template!(HomeTemplate, "home.html");
document_template!(ReviewTemplate, "review.html");
document_template!(SettingsTemplate, "settings.html");
document_template!(SettingsFragmentTemplate, "fragments/settings.html");
document_template!(MainFragmentTemplate, "fragments/main.html");
document_template!(QueueFragmentTemplate, "fragments/queue.html");

pub struct Document {
    pub title: String,
    pub page_kind: String,
    pub nav: Vec<NavNode>,
    pub toc: Vec<TocEntry>,
    pub article_html: String,
    pub concept_type: String,
    pub status: String,
    pub authority: String,
    pub review_rows: Vec<ReviewRow>,
    pub stats: Vec<StatCard>,
    pub recents: Vec<RecentDoc>,
    pub diagnostics: Vec<DiagnosticRow>,
    pub meta: ConceptMeta,
    pub message: String,
    pub config_path: String,
    pub settings_roots: Vec<SettingsRoot>,
}

impl Document {
    pub fn render_page(self) -> askama::Result<String> {
        PageTemplate::from(self).render()
    }

    pub fn render_home(self) -> askama::Result<String> {
        HomeTemplate::from(self).render()
    }

    pub fn render_review(self) -> askama::Result<String> {
        ReviewTemplate::from(self).render()
    }

    pub fn render_settings(self) -> askama::Result<String> {
        SettingsTemplate::from(self).render()
    }

    pub fn render_settings_fragment(self) -> askama::Result<String> {
        SettingsFragmentTemplate::from(self).render()
    }

    pub fn render_main_fragment(self) -> askama::Result<String> {
        MainFragmentTemplate::from(self).render()
    }

    pub fn render_queue_fragment(self) -> askama::Result<String> {
        QueueFragmentTemplate::from(self).render()
    }
}

pub fn toc_from_headings(headings: &[okf::Heading]) -> Vec<TocEntry> {
    headings
        .iter()
        .filter(|heading| (2..=3).contains(&heading.level))
        .map(|heading| TocEntry {
            id: heading.id.clone(),
            text: heading.text.clone(),
            level: heading.level,
        })
        .collect()
}

pub fn review_rows(bundle: &Bundle) -> Vec<ReviewRow> {
    bundle
        .concepts
        .iter()
        .map(|concept| {
            let action = okf::classify_concept_action(concept, &bundle.diagnostics);
            let title = okf::string_field(&concept.metadata, "title").unwrap_or(&concept.id);
            let status = okf::string_field(&concept.metadata, "status").unwrap_or("draft");
            let authority =
                okf::string_field(&concept.metadata, "authority").unwrap_or("descriptive");
            let concept_type = okf::string_field(&concept.metadata, "type").unwrap_or("Concept");
            let trust = okf::search::concept_trust_tier(&concept.metadata);
            let (trust_slug, trust_label) = match trust {
                okf::TrustTier::HumanReviewed => ("human", "human-reviewed"),
                okf::TrustTier::Generated => ("generated", "generated"),
                okf::TrustTier::Unverified => ("unverified", "unverified"),
            };
            let verifier = okf::latest_human_verification(&concept.metadata)
                .map(|(_, value)| value.to_string())
                .unwrap_or_else(|| "None".into());
            let tags = okf::metadata_string_array(&concept.metadata, "tags");
            let search = format!(
                "{} {} {} {} {} {} {}",
                title,
                concept.id,
                concept_type,
                status,
                authority,
                action.detail,
                tags.join(" ")
            )
            .to_lowercase();
            ReviewRow {
                href: format!("/{}/", concept.id),
                title: title.to_string(),
                id: concept.id.clone(),
                status: status.to_string(),
                action: action.label,
                concept_type: concept_type.to_string(),
                authority: authority.to_string(),
                trust_slug: trust_slug.into(),
                trust_label: trust_label.into(),
                verifier,
                action_detail: action.detail,
                pill_class: action.pill_class.to_string(),
                is_action_required: action.is_action_required,
                search,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_document(toc: Vec<TocEntry>) -> Document {
        Document {
            title: "Hello".into(),
            page_kind: "page".into(),
            nav: vec![
                NavNode {
                    href: "/".into(),
                    title: "Dashboard".into(),
                    current: true,
                    open: false,
                    children: Vec::new(),
                },
                NavNode {
                    href: "/review/".into(),
                    title: "Review queue".into(),
                    current: false,
                    open: false,
                    children: Vec::new(),
                },
            ],
            toc,
            article_html: "<h1>Hello</h1><p>Body</p>".into(),
            concept_type: "Architecture".into(),
            status: "draft".into(),
            authority: "descriptive".into(),
            review_rows: Vec::new(),
            stats: Vec::new(),
            recents: Vec::new(),
            diagnostics: Vec::new(),
            meta: ConceptMeta::default(),
            message: String::new(),
            config_path: "~/.okmate/config.toml".into(),
            settings_roots: Vec::new(),
        }
    }

    #[test]
    fn page_template_contains_shell_landmarks() {
        let html = sample_document(vec![TocEntry {
            id: "section".into(),
            text: "Section".into(),
            level: 2,
        }])
        .render_page()
        .unwrap();
        assert!(html.contains("id=\"okmate-nav\""), "{html}");
        assert!(html.contains("id=\"okmate-main\""), "{html}");
        assert!(html.contains("id=\"okmate-toc\""), "{html}");
        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("/__okmate/app.css"));
        assert!(html.contains("data-on:click__prevent"), "{html}");
        assert!(html.contains("/__okmate/goto.js"), "{html}");
    }

    #[test]
    fn settings_template_is_empty_state() {
        let html = sample_document(Vec::new()).render_settings().unwrap();
        assert!(html.contains("id=\"okmate-settings\""));
        assert!(html.contains("id=\"okmate-nav\""));
        assert!(html.contains("No roots yet"));
        assert!(html.contains("Choose folder"));
        assert!(html.contains("pick-folder"));
    }

    #[test]
    fn review_template_contains_queue_region() {
        let mut document = sample_document(Vec::new());
        document.page_kind = "review".into();
        document.review_rows = vec![ReviewRow {
            href: "/hello/".into(),
            title: "Hello".into(),
            id: "hello".into(),
            status: "draft".into(),
            action: "Clean".into(),
            ..ReviewRow::default()
        }];
        let html = document.render_review().unwrap();
        assert!(html.contains("id=\"okmate-queue\""));
        assert!(html.contains("Hello"));
    }

    #[test]
    fn main_fragment_patches_article_and_toc() {
        let html = sample_document(vec![TocEntry {
            id: "section".into(),
            text: "Section".into(),
            level: 2,
        }])
        .render_main_fragment()
        .unwrap();
        assert!(html.contains("id=\"okmate-main\""), "{html}");
        assert!(html.contains("id=\"okmate-toc\""), "{html}");
        assert!(html.contains("<h1>Hello</h1>"));
        assert!(!html.to_ascii_lowercase().contains("<html"));
        assert!(!html.contains("id=\"okmate-nav\""));
    }

    #[test]
    fn queue_fragment_is_the_review_region() {
        let mut document = sample_document(Vec::new());
        document.page_kind = "review".into();
        document.review_rows = vec![ReviewRow {
            href: "/hello/".into(),
            title: "Hello".into(),
            id: "hello".into(),
            status: "draft".into(),
            action: "Clean".into(),
            ..ReviewRow::default()
        }];
        let html = document.render_queue_fragment().unwrap();
        assert!(html.contains("id=\"okmate-queue\""));
        assert!(!html.to_ascii_lowercase().contains("<html"));
    }

    fn test_concept(id: &str, path: &str, title: &str, status: &str, at: &str) -> okf::Concept {
        let mut metadata = std::collections::BTreeMap::new();
        metadata.insert("title".into(), serde_json::json!(title));
        metadata.insert("status".into(), serde_json::json!(status));
        metadata.insert(
            "generated".into(),
            serde_json::json!({ "by": "process:test", "at": at }),
        );
        okf::Concept {
            id: id.into(),
            path: path.into(),
            metadata,
            body_span: okf::Span::new(0, 0),
            body_location: okf::SourceLocation {
                start: 0,
                end: 0,
                line: 1,
                column: 1,
            },
            headings: Vec::new(),
            heading_sections: Vec::new(),
            links: Vec::new(),
            source_ids: Default::default(),
            footnote_ids: Default::default(),
            article_html: String::new(),
        }
    }

    fn test_bundle(concepts: Vec<okf::Concept>) -> Bundle {
        Bundle {
            root: std::path::PathBuf::from("."),
            version: Some("0.2".into()),
            concepts,
            indexes: vec![okf::Index {
                path: "plans/index.md".into(),
                version: None,
                body_span: okf::Span::new(0, 0),
                headings: Vec::new(),
                links: Vec::new(),
                article_html: String::new(),
            }],
            logs: Vec::new(),
            graph: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    #[test]
    fn governance_stats_count_lifecycle() {
        let bundle = test_bundle(vec![
            test_concept(
                "plans/a",
                "plans/a.md",
                "A",
                "stable",
                "2026-08-01T00:00:00Z",
            ),
            test_concept(
                "plans/b",
                "plans/b.md",
                "B",
                "draft",
                "2026-08-02T00:00:00Z",
            ),
        ]);
        let stats = governance_stats(&bundle);
        assert_eq!(stats[0].label, "Total");
        assert_eq!(stats[0].value, "2");
        assert_eq!(stats[1].value, "1");
        assert_eq!(stats[2].value, "1");
        assert_eq!(stats[3].label, "Action");
        assert_eq!(stats[5].label, "Diagnostics");
    }

    #[test]
    fn recent_leaf_documents_skip_indexes_and_sort_generated_at() {
        let mut concepts = Vec::new();
        for index in 0..12 {
            concepts.push(test_concept(
                &format!("plans/doc-{index:02}"),
                &format!("plans/doc-{index:02}.md"),
                &format!("Doc {index:02}"),
                "draft",
                &format!("2026-08-{:02}T12:00:00Z", index + 1),
            ));
        }
        let bundle = test_bundle(concepts);
        let recents = recent_leaf_documents(&bundle, 10);
        assert_eq!(recents.len(), 10);
        assert_eq!(recents[0].href, "/plans/doc-11/");
        assert_eq!(recents[0].title, "Doc 11");
        assert_eq!(recents[0].collection, "plans");
        assert!(recents.iter().all(|doc| doc.href != "/plans/"));
        assert!(!recents.iter().any(|doc| doc.href == "/plans/doc-01/"));
    }
}
