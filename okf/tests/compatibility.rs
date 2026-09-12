use std::path::PathBuf;

use okf::{Profile, Severity, check, load};

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
    assert!(
        !found
            .iter()
            .any(|(_, severity)| *severity == Severity::Error),
        "{found:?}"
    );
}

#[test]
fn peer_root_extension_is_rejected_as_a_reader_limit() {
    let found = codes("peer-root-extension", Profile::Base);
    assert!(has(&found, "OKF1011", Severity::Error), "{found:?}");
}

#[test]
fn v0_1_version_is_unsupported_on_base() {
    let found = codes("v0.1-version", Profile::Base);
    assert!(has(&found, "OKF1012", Severity::Error), "{found:?}");
}

#[test]
fn verified_mapping_is_not_normalized() {
    let found = codes("verified-mapping", Profile::Base);
    assert!(has(&found, "OKF1010", Severity::Error), "{found:?}");
}

#[test]
fn stale_after_timestamp_is_rejected() {
    let found = codes("stale-after-timestamp", Profile::Base);
    assert!(has(&found, "OKF1006", Severity::Error), "{found:?}");
}

#[test]
fn unknown_metadata_is_preserved_with_a_warning() {
    let found = codes("unknown-metadata", Profile::Base);
    assert!(has(&found, "OKF2001", Severity::Warning), "{found:?}");
    assert!(
        !found
            .iter()
            .any(|(_, severity)| *severity == Severity::Error),
        "{found:?}"
    );

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
fn unknown_type_is_readable_on_base_and_warns_on_strict() {
    let base = codes("unknown-type", Profile::Base);
    assert!(!base.iter().any(|(code, _)| *code == "OKF2002"), "{base:?}");
    assert!(
        !base
            .iter()
            .any(|(_, severity)| *severity == Severity::Error),
        "{base:?}"
    );

    let strict = codes("unknown-type", Profile::Strict);
    assert!(has(&strict, "OKF2002", Severity::Warning), "{strict:?}");
    assert!(
        !strict
            .iter()
            .any(|(_, severity)| *severity == Severity::Error),
        "{strict:?}"
    );
}

#[test]
fn missing_domain_tag_is_strict_policy_not_format() {
    let base = codes("missing-domain-tag", Profile::Base);
    assert!(!base.iter().any(|(code, _)| *code == "OKF2004"), "{base:?}");
    assert!(
        !base
            .iter()
            .any(|(_, severity)| *severity == Severity::Error),
        "{base:?}"
    );

    let strict = codes("missing-domain-tag", Profile::Strict);
    assert!(has(&strict, "OKF2004", Severity::Error), "{strict:?}");
}

#[test]
fn html_comment_is_an_isolation_error() {
    let found = codes("html-comment", Profile::Base);
    assert!(has(&found, "OKF2009", Severity::Error), "{found:?}");
}

#[test]
fn rocdown_declaration_is_an_isolation_error() {
    let found = codes("rocdown-declaration", Profile::Base);
    assert!(has(&found, "OKF2007", Severity::Error), "{found:?}");
}
