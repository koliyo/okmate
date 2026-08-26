use std::process::Command;

#[test]
fn sign_script_fails_closed_without_secrets() {
    let root = env!("CARGO_MANIFEST_DIR");
    let output = Command::new(format!("{root}/packaging/macos/sign.sh"))
        .env_remove("SIGN_DRY_RUN")
        .env_remove("APPLE_DEVELOPER_ID_APPLICATION")
        .env_remove("APPLE_API_KEY_ID")
        .env_remove("APPLE_API_ISSUER")
        .env_remove("APPLE_API_KEY")
        .arg(format!("{root}/packaging/macos"))
        .output()
        .expect("run sign.sh");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("missing signing secrets"), "{stderr}");
    assert!(stderr.contains("refusing to upload"), "{stderr}");
    let combined = format!("{}{}", String::from_utf8_lossy(&output.stdout), stderr);
    assert!(
        !combined.to_ascii_lowercase().contains("stapled"),
        "{combined}"
    );
}

#[test]
fn sign_script_dry_run_does_not_claim_notarization() {
    let root = env!("CARGO_MANIFEST_DIR");
    let output = Command::new(format!("{root}/packaging/macos/sign.sh"))
        .env("SIGN_DRY_RUN", "1")
        .env_remove("APPLE_DEVELOPER_ID_APPLICATION")
        .env_remove("APPLE_API_KEY_ID")
        .env_remove("APPLE_API_ISSUER")
        .env_remove("APPLE_API_KEY")
        .arg(format!("{root}/packaging/macos"))
        .output()
        .expect("run sign.sh dry-run");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(stdout.contains("dry-run"), "{stdout}");
    assert!(stdout.contains("fail-closed"), "{stdout}");
    assert!(!stdout.to_ascii_lowercase().contains("stapled"));
    assert!(!stderr.to_ascii_lowercase().contains("notarized"));
}

#[test]
fn release_workflow_signs_before_upload() {
    let workflow = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/.github/workflows/release.yml"
    ));
    assert!(workflow.contains("packaging/macos/sign.sh"));
    assert!(workflow.contains("APPLE_DEVELOPER_ID_APPLICATION"));
    assert!(workflow.contains("APPLE_API_KEY"));
}
