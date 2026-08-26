use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

use common::temp_dir;

mod common;

#[test]
fn assemble_script_writes_plist_keys_and_executable() {
    let root = env!("CARGO_MANIFEST_DIR");
    let scratch = temp_dir("package-layout");
    let binary = scratch.join("okmate");
    fs::write(&binary, b"okmate-binary").unwrap();
    let dest = scratch.join("Okmate.app");
    let status = Command::new(format!("{root}/packaging/macos/assemble.sh"))
        .arg(&binary)
        .arg(&dest)
        .arg("1.2.3")
        .arg("10.0.0")
        .status()
        .expect("run assemble.sh");
    assert!(status.success(), "assemble.sh failed: {status}");

    let exe = dest.join("Contents/MacOS/okmate");
    assert_eq!(fs::read(&exe).unwrap(), b"okmate-binary");
    assert_ne!(fs::metadata(&exe).unwrap().permissions().mode() & 0o111, 0);

    let plist = fs::read_to_string(dest.join("Contents/Info.plist")).unwrap();
    assert!(plist.contains("com.koliyo.okmate"), "{plist}");
    assert!(plist.contains("<string>1.2.3</string>"), "{plist}");
    assert!(plist.contains("<string>10.0.0</string>"), "{plist}");
    assert!(!plist.contains("@VERSION@"), "{plist}");
    assert!(!plist.contains("@BUNDLE_VERSION@"), "{plist}");
    assert_eq!(
        fs::read(dest.join("Contents/PkgInfo")).unwrap(),
        b"APPL????"
    );
}
