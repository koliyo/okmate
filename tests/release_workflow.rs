#[test]
fn release_workflow_filters_v_star_tags() {
    let workflow = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/.github/workflows/release.yml"
    ));
    assert!(workflow.contains("- \"v*\"") || workflow.contains("- 'v*'"));
    assert!(workflow.contains("github.ref_name != 'dev'"));
    assert!(workflow.contains("macos-latest"));
    assert!(workflow.contains("SPARKLE_EDDSA_PRIVATE_KEY"));
    assert!(workflow.contains("h35-desktop"));
}
