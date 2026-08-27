mod common;

use std::fs;
use std::process::Command;

use common::{okmate_bin, temp_dir, valid_strict_concept, write_index};

fn write_fixture() -> std::path::PathBuf {
    let root = temp_dir("timings");
    write_index(&root);
    fs::write(
        root.join("hello.md"),
        valid_strict_concept("Hello", "", "A small document.\n"),
    )
    .unwrap();
    root
}

fn run_ok(args: &[&str]) -> String {
    let output = Command::new(okmate_bin()).args(args).output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn timings_json_has_contract_keys() {
    let root = write_fixture();
    let stdout = run_ok(&[
        "timings",
        root.to_str().unwrap(),
        "--format",
        "json",
        "--scenario",
        "all",
    ]);
    let value: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    assert_eq!(value["timings_version"], 1);
    assert!(value["roots"].is_array(), "{value}");
    assert!(value["workspace"].is_object(), "{value}");
    assert!(value["pages"].is_array(), "{value}");
    assert!(value["roots"][0]["timings"]["parse"].is_number(), "{value}");
}

#[test]
fn timings_click_includes_small_document_route() {
    let root = write_fixture();
    let stdout = run_ok(&[
        "timings",
        root.to_str().unwrap(),
        "--format",
        "json",
        "--scenario",
        "click",
    ]);
    let value: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    let click = &value["click"];
    assert_eq!(click["route"], "/hello/");
    assert!(click["fragment_bytes"].as_u64().unwrap() > 0, "{value}");
    assert!(value["pages"][0]["fragment_bytes"].as_u64().unwrap() > 0);
}

#[test]
fn benchmark_still_runs_retrieval_toml() {
    let root = write_fixture();
    let bench = root.join("bench.toml");
    fs::write(
        &bench,
        r#"version = 1
top_k = 5
minimum_hit_rate = 1.0

[[questions]]
id = "hello"
question = "Where is hello described?"
query = "small document"
expected_concepts = ["hello"]
"#,
    )
    .unwrap();
    let stdout = run_ok(&[
        "benchmark",
        bench.to_str().unwrap(),
        root.to_str().unwrap(),
        "--profile",
        "strict",
    ]);
    let value: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    assert!(value["threshold_met"].as_bool().unwrap());
}
