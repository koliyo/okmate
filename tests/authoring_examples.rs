use std::path::PathBuf;

use okf::{Profile, Severity, check};

fn example(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("docs/examples")
        .join(name)
}

fn profile_name(profile: Profile) -> &'static str {
    match profile {
        Profile::Base => "base",
        Profile::Evidence => "evidence",
        Profile::Strict => "strict",
    }
}

fn diagnostics(name: &str, profile: Profile) -> Vec<(String, Severity)> {
    let report = check(&example(name), profile).unwrap_or_else(|error| {
        panic!("check {name} with {}: {error}", profile_name(profile));
    });
    report
        .diagnostics
        .into_iter()
        .map(|diagnostic| (diagnostic.code.to_string(), diagnostic.severity))
        .collect()
}

fn assert_clean(name: &str, profile: Profile) {
    let found = diagnostics(name, profile);
    assert!(
        found.is_empty(),
        "{name} with {} should have no diagnostics, got {found:?}",
        profile_name(profile)
    );
}

#[test]
fn minimal_passes_base() {
    assert_clean("minimal", Profile::Base);
}

#[test]
fn software_archive_passes_strict() {
    assert_clean("software-archive", Profile::Strict);
}

#[test]
fn software_archive_passes_evidence() {
    assert_clean("software-archive", Profile::Evidence);
}

#[test]
fn engineering_handbook_passes_base() {
    assert_clean("engineering-handbook", Profile::Base);
}

#[test]
fn operations_passes_base() {
    assert_clean("operations", Profile::Base);
}

#[test]
fn data_catalog_passes_base() {
    assert_clean("data-catalog", Profile::Base);
}

#[test]
fn handbook_strict_warns_on_product_vocabulary() {
    let found = diagnostics("engineering-handbook", Profile::Strict);
    assert!(
        found
            .iter()
            .any(|(code, severity)| { code == "OKF2002" && *severity == Severity::Warning }),
        "strict should warn on handbook types, got {found:?}"
    );
}
