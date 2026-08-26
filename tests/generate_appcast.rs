use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

use common::temp_dir;

mod common;

#[test]
fn generate_appcast_helper_is_flat_stdin_and_silent() {
    let root = env!("CARGO_MANIFEST_DIR");
    let scratch = temp_dir("generate-appcast");
    let inbox = scratch.join("inbox");
    fs::create_dir_all(&inbox).unwrap();
    fs::write(inbox.join("Okmate.zip"), b"zip").unwrap();

    let tool = scratch.join("generate_appcast");
    fs::write(
        &tool,
        "#!/bin/sh\nset -eu\nprintf 'args=%s\\n' \"$*\" > \"$0.out\"\ncat > \"$0.in\"\n",
    )
    .unwrap();
    let mut perms = fs::metadata(&tool).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&tool, perms).unwrap();

    let secret = "unit-test-eddsa-private-key";
    let output = Command::new(format!("{root}/packaging/macos/generate-appcast.sh"))
        .env("GENERATE_APPCAST", &tool)
        .env("SPARKLE_EDDSA_PRIVATE_KEY", secret)
        .arg(&inbox)
        .arg("https://github.com/koliyo/okmate/releases/download/v1.2.3/")
        .output()
        .expect("run generate-appcast.sh");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "generate-appcast.sh failed: {stderr}"
    );
    assert!(!stdout.contains(secret), "{stdout}");
    assert!(!stderr.contains(secret), "{stderr}");

    let args = fs::read_to_string(format!("{}.out", tool.display())).unwrap();
    assert!(args.contains("--maximum-deltas 0"), "{args}");
    assert!(
        args.contains(
            "--download-url-prefix https://github.com/koliyo/okmate/releases/download/v1.2.3/"
        ),
        "{args}"
    );
    assert!(args.contains("--ed-key-file -"), "{args}");
    assert_eq!(
        fs::read_to_string(format!("{}.in", tool.display())).unwrap(),
        secret
    );
}

#[test]
fn generate_appcast_helper_rejects_nested_inbox() {
    let root = env!("CARGO_MANIFEST_DIR");
    let scratch = temp_dir("generate-appcast-nested");
    let inbox = scratch.join("inbox");
    fs::create_dir_all(inbox.join("nested")).unwrap();
    let output = Command::new(format!("{root}/packaging/macos/generate-appcast.sh"))
        .env("GENERATE_APPCAST", "/usr/bin/true")
        .env("SPARKLE_EDDSA_PRIVATE_KEY", "secret")
        .arg(&inbox)
        .arg("https://example.invalid/")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("flat"), "{stderr}");
    assert!(!stderr.contains("secret"), "{stderr}");
}

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
    assert!(workflow.contains("releases/download/"));
}
