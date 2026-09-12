use std::path::PathBuf;

use okf::{DiagnosticLayer, Profile, Severity, check, load};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/compatibility")
        .join(name)
}

fn codes(root: &str, profile: Profile) -> Vec<(&'static str, Severity)> {
    let report = check(&fixture(root), profile).expect("check fixture");
    report
        .diagnostics
        .into_iter()
        .map(|diagnostic| (diagnostic.code, diagnostic.severity))
        .collect()
}

fn has(codes: &[(&str, Severity)], code: &str, severity: Severity) -> bool {
    codes
        .iter()
        .any(|(found, found_severity)| *found == code && *found_severity == severity)
}

fn no_errors(codes: &[(&str, Severity)]) -> bool {
    codes
        .iter()
        .all(|(_, severity)| *severity != Severity::Error)
}

#[test]
fn malformed_yaml_is_a_format_error_on_base() {
    let found = codes("malformed-yaml", Profile::Base);
    assert!(has(&found, "OKF1003", Severity::Error), "{found:?}");
}

#[test]
fn citation_missing_definition_is_a_format_error() {
    let found = codes("citation-missing-definition", Profile::Base);
    assert!(has(&found, "OKF4003", Severity::Error), "{found:?}");
}

#[test]
fn citation_unused_source_is_a_format_warning() {
    let found = codes("citation-unused-source", Profile::Base);
    assert!(has(&found, "OKF4002", Severity::Warning), "{found:?}");
    assert!(no_errors(&found), "{found:?}");
}

#[test]
fn peer_root_extension_is_an_advisory_warning() {
    let found = codes("peer-root-extension", Profile::Base);
    assert!(has(&found, "OKF1011", Severity::Warning), "{found:?}");
    assert!(no_errors(&found), "{found:?}");
}

#[test]
fn advisory_root_extension_does_not_hide_malformed_yaml() {
    let found = codes("advisory-root-and-malformed", Profile::Base);
    assert!(has(&found, "OKF1011", Severity::Warning), "{found:?}");
    assert!(has(&found, "OKF1003", Severity::Error), "{found:?}");
}

#[test]
fn malformed_root_index_is_still_a_parse_failure() {
    let found = codes("malformed-root-index", Profile::Base);
    assert!(has(&found, "OKF1003", Severity::Error), "{found:?}");
}

#[test]
fn v0_1_and_v0_2_versions_are_readable() {
    let v01 = codes("v0.1-version", Profile::Base);
    assert!(!v01.iter().any(|(code, _)| *code == "OKF1012"), "{v01:?}");
    assert!(no_errors(&v01), "{v01:?}");

    let v02 = codes("peer-root-extension", Profile::Base);
    assert!(!v02.iter().any(|(code, _)| *code == "OKF1012"), "{v02:?}");
}

#[test]
fn verified_mapping_is_normalized() {
    let found = codes("verified-mapping", Profile::Base);
    assert!(
        !found.iter().any(|(code, _)| *code == "OKF1010"),
        "{found:?}"
    );
    assert!(no_errors(&found), "{found:?}");

    let bundle = load(&fixture("verified-mapping"), Profile::Base).expect("load");
    let concept = bundle
        .concepts
        .iter()
        .find(|concept| concept.id == "verified")
        .expect("verified concept");
    assert!(
        concept
            .metadata
            .get("verified")
            .and_then(|value| value.as_array())
            .is_some_and(|events| events.len() == 1)
    );
}

#[test]
fn stale_after_timestamp_is_accepted() {
    let found = codes("stale-after-timestamp", Profile::Base);
    assert!(
        !found.iter().any(|(code, _)| *code == "OKF1006"),
        "{found:?}"
    );
    assert!(no_errors(&found), "{found:?}");
}

#[test]
fn unknown_metadata_is_preserved_without_a_base_warning() {
    let found = codes("unknown-metadata", Profile::Base);
    assert!(
        !found.iter().any(|(code, _)| *code == "OKF2001"),
        "{found:?}"
    );
    assert!(no_errors(&found), "{found:?}");

    let bundle = load(&fixture("unknown-metadata"), Profile::Base).expect("load");
    let extra = bundle
        .concepts
        .iter()
        .find(|concept| concept.id == "extra")
        .expect("extra concept");
    assert_eq!(
        extra
            .metadata
            .get("custom_review_lane")
            .and_then(|value| value.as_str()),
        Some("maintainers")
    );
}

#[test]
fn unknown_metadata_warns_on_evidence_and_round_trips() {
    let found = codes("unknown-metadata", Profile::Evidence);
    assert!(has(&found, "OKF2001", Severity::Warning), "{found:?}");
    let bundle = load(&fixture("unknown-metadata"), Profile::Evidence).expect("load");
    let extra = bundle
        .concepts
        .iter()
        .find(|concept| concept.id == "extra")
        .expect("extra concept");
    assert_eq!(
        extra
            .metadata
            .get("custom_review_lane")
            .and_then(|value| value.as_str()),
        Some("maintainers")
    );
}

#[test]
fn unknown_type_is_readable_on_base_and_evidence_and_warns_on_strict() {
    let base = codes("unknown-type", Profile::Base);
    assert!(!base.iter().any(|(code, _)| *code == "OKF2002"), "{base:?}");
    assert!(no_errors(&base), "{base:?}");

    let evidence = codes("unknown-type", Profile::Evidence);
    assert!(
        !evidence.iter().any(|(code, _)| *code == "OKF2002"),
        "{evidence:?}"
    );
    assert!(no_errors(&evidence), "{evidence:?}");

    let strict = codes("unknown-type", Profile::Strict);
    assert!(has(&strict, "OKF2002", Severity::Warning), "{strict:?}");
    assert!(no_errors(&strict), "{strict:?}");
}

#[test]
fn type_only_custom_concept_is_readable_on_base() {
    let base = codes("type-only-workflow", Profile::Base);
    assert!(no_errors(&base), "{base:?}");
    assert!(!base.iter().any(|(code, _)| *code == "OKF2002"), "{base:?}");
}

#[test]
fn type_only_custom_concept_fails_evidence_not_format_vocabulary() {
    let report = check(&fixture("type-only-workflow"), Profile::Evidence).expect("check");
    assert!(report.has_errors(), "{:?}", report.diagnostics);
    assert!(
        report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "OKF2003"
                && diagnostic.layer == DiagnosticLayer::Evidence
                && diagnostic.severity == Severity::Error
        }),
        "{:?}",
        report.diagnostics
    );
    assert!(
        !report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "OKF2002"),
        "{:?}",
        report.diagnostics
    );
}

#[test]
fn evidenced_runbook_passes_evidence_with_arbitrary_tags() {
    let found = codes("evidenced-runbook", Profile::Evidence);
    assert!(found.is_empty(), "{found:?}");
    let strict = codes("evidenced-runbook", Profile::Strict);
    assert!(has(&strict, "OKF2002", Severity::Warning), "{strict:?}");
    assert!(has(&strict, "OKF2004", Severity::Error), "{strict:?}");
    assert!(has(&strict, "OKF2005", Severity::Error), "{strict:?}");
}

#[test]
fn missing_domain_tag_is_strict_policy_not_evidence() {
    let base = codes("missing-domain-tag", Profile::Base);
    assert!(!base.iter().any(|(code, _)| *code == "OKF2004"), "{base:?}");
    assert!(no_errors(&base), "{base:?}");

    let evidence = codes("missing-domain-tag", Profile::Evidence);
    assert!(
        !evidence
            .iter()
            .any(|(code, _)| *code == "OKF2004" || *code == "OKF2005"),
        "{evidence:?}"
    );
    assert!(no_errors(&evidence), "{evidence:?}");

    let strict = codes("missing-domain-tag", Profile::Strict);
    assert!(has(&strict, "OKF2004", Severity::Error), "{strict:?}");
}

#[test]
fn html_comment_is_not_an_authoring_error() {
    let found = codes("html-comment", Profile::Base);
    assert!(
        !found.iter().any(|(code, _)| *code == "OKF2009"),
        "{found:?}"
    );
    assert!(no_errors(&found), "{found:?}");

    let bundle = load(&fixture("html-comment"), Profile::Base).expect("load");
    let concept = bundle
        .concepts
        .iter()
        .find(|concept| concept.id == "comment")
        .expect("comment concept");
    assert!(
        !concept.article_html.contains("<!--"),
        "{}",
        concept.article_html
    );
}

#[test]
fn unsafe_html_is_an_isolation_error_and_is_not_rendered() {
    let found = codes("unsafe-html", Profile::Base);
    assert!(has(&found, "OKF2009", Severity::Error), "{found:?}");

    let bundle = load(&fixture("unsafe-html"), Profile::Base).expect("load");
    let concept = bundle
        .concepts
        .iter()
        .find(|concept| concept.id == "scripted")
        .expect("scripted concept");
    assert!(
        !concept
            .article_html
            .to_ascii_lowercase()
            .contains("<script"),
        "{}",
        concept.article_html
    );
}

#[test]
fn rocdown_declaration_is_an_isolation_error() {
    let found = codes("rocdown-declaration", Profile::Base);
    assert!(has(&found, "OKF2007", Severity::Error), "{found:?}");
}

#[test]
fn evidence_errors_name_the_evidence_layer() {
    let report = check(&fixture("type-only-workflow"), Profile::Evidence).expect("check");
    for diagnostic in &report.diagnostics {
        if diagnostic.code == "OKF2003" {
            assert_eq!(diagnostic.layer, DiagnosticLayer::Evidence);
            let text = diagnostic.to_string();
            assert!(text.contains(" evidence "), "{text}");
        }
    }
}
