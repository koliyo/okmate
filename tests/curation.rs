use std::path::PathBuf;

use okf::{KnowledgeFilter, Profile};
use serde_json::Value;

fn example(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("docs/examples")
        .join(name)
}

fn handbook() -> PathBuf {
    example("engineering-handbook")
}

fn archive() -> PathBuf {
    example("software-archive")
}

fn search_ids(root: &std::path::Path, query: &str, profile: Profile) -> Vec<String> {
    let json = okf::search(root, query, profile, &KnowledgeFilter::default()).unwrap();
    let chunks: Vec<Value> = serde_json::from_str(&json).unwrap();
    let mut ids = Vec::new();
    for chunk in chunks {
        if let Some(id) = chunk.get("concept_id").and_then(Value::as_str)
            && !ids.iter().any(|existing| existing == id)
        {
            ids.push(id.to_string());
        }
    }
    ids
}

#[test]
fn frozen_handbook_ids_and_question_routes_remain() {
    let root = handbook();
    for relative in [
        "agentic-development/task-steering.md",
        "developer-environments/restore-from-manifest.md",
        "reference/source-register.md",
        "audits/backup-snapshot.md",
        "agentic-development/evaluate-results.md",
        "research/model-comparison.md",
    ] {
        assert!(root.join(relative).is_file(), "{relative}");
    }
    let index = std::fs::read_to_string(root.join("index.md")).unwrap();
    for needle in [
        "How should I steer delegated work?",
        "How should I evaluate agent results?",
        "How do I restore a development environment?",
        "Where did these conclusions come from?",
        "What was observed on this machine?",
        "Where is the owning product contract?",
        "docs/examples/software-archive",
    ] {
        assert!(index.contains(needle), "missing {needle}");
    }
}

#[test]
fn source_register_type_and_environment_tags() {
    let register =
        std::fs::read_to_string(handbook().join("reference/source-register.md")).unwrap();
    assert!(register.contains("type: Reference"), "{register}");
    assert!(!register.contains("type: Research Report"), "{register}");
    let restore =
        std::fs::read_to_string(handbook().join("developer-environments/restore-from-manifest.md"))
            .unwrap();
    assert!(!restore.contains("domain/agentic"), "{restore}");
    let audit = std::fs::read_to_string(handbook().join("audits/backup-snapshot.md")).unwrap();
    assert!(audit.contains("type: Audit"), "{audit}");
    assert!(!audit.contains("type: How-to Guide"), "{audit}");
}

#[test]
fn handbook_retrieval_meets_frozen_questions() {
    let report = okf::benchmark_retrieval(
        &handbook(),
        &handbook().join("retrieval.toml"),
        Profile::Base,
    )
    .unwrap();
    assert!(
        report.threshold_met,
        "hit rate {:.2} questions {:?}",
        report.hit_rate, report.questions
    );
}

#[test]
fn product_contract_stays_in_the_archive() {
    let handbook_hits = search_ids(&handbook(), "portable engine parses OKF", Profile::Base);
    assert!(
        !handbook_hits
            .iter()
            .any(|id| id == "architecture/system-overview"),
        "handbook leaked product architecture: {handbook_hits:?}"
    );
    let archive_hits = search_ids(&archive(), "portable engine parses OKF", Profile::Strict);
    assert!(
        archive_hits
            .iter()
            .any(|id| id == "architecture/system-overview"),
        "archive missed product contract: {archive_hits:?}"
    );
    let report = okf::benchmark_retrieval(
        &archive(),
        &archive().join("retrieval.toml"),
        Profile::Strict,
    )
    .unwrap();
    assert!(report.threshold_met, "{:?}", report.questions);
}

#[test]
fn handbook_and_archive_stay_check_clean() {
    for (name, profile) in [
        ("engineering-handbook", Profile::Base),
        ("software-archive", Profile::Strict),
    ] {
        let report = okf::check(&example(name), profile).unwrap();
        assert!(
            !report.has_errors(),
            "{name} errors: {:?}",
            report.diagnostics
        );
        let unresolved: Vec<_> = report
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.code == "OKF3002" || diagnostic.code == "OKF3001")
            .map(|diagnostic| diagnostic.code)
            .collect();
        assert!(
            unresolved.is_empty(),
            "{name} link findings: {unresolved:?}"
        );
    }
}
