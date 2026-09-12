mod common;

use std::fs;
use std::process::Command;

use common::{okmate_bin, temp_dir, write_index};

#[test]
fn help_lists_init() {
    let output = Command::new(okmate_bin()).arg("--help").output().unwrap();
    assert!(output.status.success());
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        text.contains("init"),
        "expected help listing init, got: {text}"
    );
}

#[test]
fn init_help_lists_flags() {
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg("--help")
        .output()
        .unwrap();
    assert!(output.status.success());
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(text.contains("--apply"), "{text}");
    assert!(text.contains("--bare"), "{text}");
    assert!(text.contains("--template"), "{text}");
    assert!(text.contains("--collection"), "{text}");
    assert!(text.contains("--template-file"), "{text}");
    assert!(text.contains("--title"), "{text}");
    assert!(text.contains("--format"), "{text}");
    assert!(text.contains("--register"), "{text}");
    assert!(text.contains("--agents"), "{text}");
    assert!(text.contains("--id"), "{text}");
}

#[test]
fn dry_run_prints_index_and_does_not_create() {
    let root = temp_dir("init-dry");
    let target = root.join("knowledge");
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("index.md"), "{stdout}");
    assert!(!target.join("index.md").exists());
    assert!(!target.exists() || fs::read_dir(&target).map(|d| d.count()).unwrap_or(0) == 0);
}

#[test]
fn format_json_parses() {
    let root = temp_dir("init-json");
    let target = root.join("knowledge");
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: serde_json::Value =
        serde_json::from_str(String::from_utf8_lossy(&output.stdout).trim()).unwrap();
    let files = value["files"].as_array().expect("files array");
    assert!(
        files.iter().any(|file| file["relative"] == "index.md"),
        "{value}"
    );
    assert!(files.iter().any(|file| file["op"] == "create"), "{value}");
}

#[test]
fn occupied_bundle_errors() {
    let root = temp_dir("init-occupied");
    write_index(&root);
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&root)
        .output()
        .unwrap();
    assert!(!output.status.success());
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        text.contains("okf_version") || text.contains("already an OKF bundle"),
        "{text}"
    );
    assert!(!root.join("log.md").exists());
}

#[test]
fn bare_omits_architecture_index() {
    let root = temp_dir("init-bare");
    let target = root.join("knowledge");
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .arg("--bare")
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: serde_json::Value =
        serde_json::from_str(String::from_utf8_lossy(&output.stdout).trim()).unwrap();
    let files = value["files"].as_array().expect("files array");
    assert!(
        files.iter().any(|file| file["relative"] == "index.md"),
        "{value}"
    );
    assert!(
        files.iter().any(|file| file["relative"] == "log.md"),
        "{value}"
    );
    assert!(
        !files
            .iter()
            .any(|file| file["relative"] == "architecture/index.md"),
        "{value}"
    );
}

fn check_json(root: &std::path::Path) -> (bool, String) {
    let output = Command::new(okmate_bin())
        .arg("check")
        .arg(root)
        .arg("--profile")
        .arg("strict")
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();
    (
        output.status.success(),
        String::from_utf8_lossy(&output.stdout).into_owned(),
    )
}

#[test]
fn apply_creates_check_clean_knowledge_child() {
    let parent = temp_dir("init-apply");
    let target = parent.join("knowledge");
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .arg("--apply")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(target.join("index.md").is_file());
    assert!(target.join("log.md").is_file());
    assert!(!target.join("architecture/index.md").exists());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("wrote   index.md"), "{stdout}");
    assert!(stdout.contains("template: minimal"), "{stdout}");
    assert!(stdout.contains("okmate check"), "{stdout}");
    let (ok, json) = check_json(&target);
    assert!(ok, "{json}");
}

#[test]
fn second_apply_fails_without_mutating() {
    let parent = temp_dir("init-reapply");
    let target = parent.join("knowledge");
    let first = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .arg("--apply")
        .output()
        .unwrap();
    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    let before = fs::read(target.join("index.md")).unwrap();
    let second = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .arg("--apply")
        .output()
        .unwrap();
    assert!(!second.status.success());
    let after = fs::read(target.join("index.md")).unwrap();
    assert_eq!(before, after);
    assert!(!target.join("architecture/index.md.bak").exists());
}

#[test]
fn bare_apply_is_check_clean() {
    let parent = temp_dir("init-bare-apply");
    let target = parent.join("knowledge");
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .arg("--bare")
        .arg("--apply")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(target.join("index.md").is_file());
    assert!(target.join("log.md").is_file());
    assert!(!target.join("architecture/index.md").exists());
    let (ok, json) = check_json(&target);
    assert!(ok, "{json}");
}

#[test]
fn apply_refuses_existing_okf_version() {
    let root = temp_dir("init-apply-occupied");
    write_index(&root);
    let before = fs::read(root.join("index.md")).unwrap();
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&root)
        .arg("--apply")
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert_eq!(fs::read(root.join("index.md")).unwrap(), before);
    assert!(!root.join("log.md").exists());
}

#[test]
fn register_apply_adds_directory_root() {
    let parent = temp_dir("init-register");
    let target = parent.join("knowledge");
    let config = parent.join("config.toml");
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .arg("--apply")
        .arg("--register")
        .arg("--id")
        .arg("my-bundle")
        .env("OKMATE_CONFIG", &config)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let saved = fs::read_to_string(&config).unwrap();
    assert!(saved.contains("id = \"my-bundle\""), "{saved}");
    assert!(saved.contains("kind = \"directory\""), "{saved}");
    assert!(saved.contains("incoming = \"allow\""), "{saved}");
    let canonical = target.canonicalize().unwrap();
    assert!(saved.contains(&canonical.display().to_string()), "{saved}");
}

#[test]
fn register_duplicate_id_fails() {
    let parent = temp_dir("init-dup");
    let target = parent.join("knowledge");
    let config = parent.join("config.toml");
    fs::write(
        &config,
        "poll = \"5m\"\n[[roots]]\nid = \"my-bundle\"\nkind = \"directory\"\npath = \"/tmp/other\"\n",
    )
    .unwrap();
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .arg("--apply")
        .arg("--register")
        .arg("--id")
        .arg("my-bundle")
        .env("OKMATE_CONFIG", &config)
        .output()
        .unwrap();
    assert!(!output.status.success());
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(text.contains("duplicate root id"), "{text}");
    assert!(!target.join("index.md").exists());
    let saved = fs::read_to_string(&config).unwrap();
    assert_eq!(saved.matches("id = \"my-bundle\"").count(), 1, "{saved}");
}

#[test]
fn agents_apply_writes_and_skips_existing() {
    let repo = temp_dir("init-agents");
    git_init(&repo);
    let target = repo.join("knowledge");
    fs::write(repo.join("AGENTS.md"), "keep me\n").unwrap();
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .arg("--apply")
        .arg("--agents")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        fs::read_to_string(repo.join("AGENTS.md")).unwrap(),
        "keep me\n"
    );
    assert!(
        repo.join(".cursor/rules/write-knowledge.mdc").is_file(),
        "missing write-knowledge.mdc"
    );
    assert!(
        repo.join(".agents/skills/manage-knowledge/SKILL.md")
            .is_file(),
        "missing manage-knowledge skill"
    );
    let attrs = fs::read_to_string(repo.join(".gitattributes")).unwrap();
    assert!(attrs.contains("knowledge/log.md merge=union"), "{attrs}");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("skipped  AGENTS.md") || stdout.contains("skip"),
        "{stdout}"
    );
    let skill = fs::read_to_string(repo.join(".agents/skills/manage-knowledge/SKILL.md")).unwrap();
    assert!(
        skill.contains("okmate check knowledge --profile strict"),
        "{skill}"
    );
    assert!(!skill.contains("okmate check docs "), "{skill}");
}

#[test]
fn agents_without_git_errors() {
    let parent = temp_dir("init-nogit");
    let target = parent.join("knowledge");
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .arg("--agents")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(text.contains("--agents") && text.contains("git"), "{text}");
    assert!(!target.join("index.md").exists());
}

fn json_files(stdout: &str) -> Vec<serde_json::Value> {
    let value: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    value["files"].as_array().expect("files").clone()
}

#[test]
fn software_project_creates_legacy_collections() {
    let parent = temp_dir("init-software");
    let target = parent.join("knowledge");
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .arg("--template")
        .arg("software-project")
        .arg("--apply")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(target.join("architecture/index.md").is_file());
    assert!(target.join("audits/index.md").is_file());
    let index = fs::read_to_string(target.join("index.md")).unwrap();
    assert!(index.contains("[Architecture](architecture/)"), "{index}");
    let (ok, json) = check_json(&target);
    assert!(ok, "{json}");
}

#[test]
fn bare_is_alias_for_minimal() {
    let parent = temp_dir("init-bare-alias");
    let target = parent.join("knowledge");
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .arg("--bare")
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();
    assert!(output.status.success());
    let value: serde_json::Value =
        serde_json::from_str(String::from_utf8_lossy(&output.stdout).trim()).unwrap();
    assert_eq!(value["template"], "minimal");
}

#[test]
fn bare_rejects_software_project() {
    let parent = temp_dir("init-bare-conflict");
    let target = parent.join("knowledge");
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .arg("--bare")
        .arg("--template")
        .arg("software-project")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let text = String::from_utf8_lossy(&output.stderr);
    assert!(text.contains("--bare"), "{text}");
    assert!(!target.join("index.md").exists());
}

#[test]
fn custom_collections_are_created() {
    let parent = temp_dir("init-custom");
    let target = parent.join("knowledge");
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .arg("--collection")
        .arg("runbooks")
        .arg("--collection")
        .arg("services/hosts")
        .arg("--apply")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(target.join("runbooks/index.md").is_file());
    assert!(target.join("services/hosts/index.md").is_file());
    assert!(!target.join("architecture/index.md").exists());
    let index = fs::read_to_string(target.join("index.md")).unwrap();
    assert!(index.contains("[Runbooks](runbooks/)"), "{index}");
    assert!(index.contains("[Hosts](services/hosts/)"), "{index}");
    let (ok, json) = check_json(&target);
    assert!(ok, "{json}");
}

#[test]
fn reserved_collection_paths_are_rejected() {
    let parent = temp_dir("init-escape");
    let target = parent.join("knowledge");
    for bad in ["../outside", "/tmp/abs", ".", ".."] {
        let output = Command::new(okmate_bin())
            .arg("init")
            .arg(&target)
            .arg("--collection")
            .arg(bad)
            .output()
            .unwrap();
        assert!(!output.status.success(), "accepted {bad}");
        assert!(!target.join("index.md").exists());
    }
}

#[test]
fn template_collection_collision_is_rejected() {
    let parent = temp_dir("init-collide");
    let target = parent.join("knowledge");
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .arg("--template")
        .arg("software-project")
        .arg("--collection")
        .arg("architecture")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let text = String::from_utf8_lossy(&output.stderr);
    assert!(text.contains("duplicate collection"), "{text}");
}

#[test]
fn template_file_loads_local_collections() {
    let parent = temp_dir("init-toml");
    let template = parent.join("seed.toml");
    fs::write(
        &template,
        "[[collections]]\npath = \"metrics\"\nheading = \"Metrics\"\nblurb = \"Named measures.\"\n",
    )
    .unwrap();
    let target = parent.join("knowledge");
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .arg("--template-file")
        .arg(&template)
        .arg("--apply")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let index = fs::read_to_string(target.join("index.md")).unwrap();
    assert!(
        index.contains("[Metrics](metrics/) - Named measures."),
        "{index}"
    );
    assert!(target.join("metrics/index.md").is_file());
}

#[test]
fn template_and_template_file_are_rejected() {
    let parent = temp_dir("init-both");
    let template = parent.join("seed.toml");
    fs::write(&template, "[[collections]]\npath = \"metrics\"\n").unwrap();
    let target = parent.join("knowledge");
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .arg("--template")
        .arg("minimal")
        .arg("--template-file")
        .arg(&template)
        .output()
        .unwrap();
    assert!(!output.status.success());
}

#[test]
fn docs_and_nested_roots_render_actual_paths() {
    let repo = temp_dir("init-docs");
    git_init(&repo);
    let docs = repo.join("docs");
    let nested = repo.join("docs").join("kb");
    let hidden = repo.join(".okf");
    for (target, expected) in [
        (docs.as_path(), "okmate check docs --profile strict"),
        (nested.as_path(), "okmate check docs/kb --profile strict"),
        (hidden.as_path(), "okmate check .okf --profile strict"),
    ] {
        let output = Command::new(okmate_bin())
            .arg("init")
            .arg(target)
            .arg("--apply")
            .arg("--agents")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}: {}",
            target.display(),
            String::from_utf8_lossy(&output.stderr)
        );
        let skill =
            fs::read_to_string(repo.join(".agents/skills/manage-knowledge/SKILL.md")).unwrap();
        assert!(skill.contains(expected), "missing {expected} in {skill}");
        assert!(
            !skill.contains("okmate check knowledge --profile strict"),
            "{skill}"
        );
        let _ = fs::remove_file(repo.join(".agents/skills/manage-knowledge/SKILL.md"));
        let _ = fs::remove_file(repo.join("AGENTS.md"));
        let _ = fs::remove_file(repo.join(".cursor/rules/write-knowledge.mdc"));
        let _ = fs::remove_file(repo.join(".gitattributes"));
    }
}

#[test]
fn space_containing_root_is_quoted() {
    let repo = temp_dir("init-space");
    git_init(&repo);
    let target = repo.join("my docs");
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .arg("--apply")
        .arg("--agents")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let skill = fs::read_to_string(repo.join(".agents/skills/manage-knowledge/SKILL.md")).unwrap();
    assert!(
        skill.contains("okmate check 'my docs' --profile strict"),
        "{skill}"
    );
    let attrs = fs::read_to_string(repo.join(".gitattributes")).unwrap();
    assert!(attrs.contains("\"my docs/log.md\" merge=union"), "{attrs}");
}

#[test]
fn generic_basename_uses_repository_id() {
    let parent = temp_dir("init-generic");
    let repo = parent.join("acme");
    fs::create_dir(&repo).unwrap();
    git_init(&repo);
    let target = repo.join("docs");
    let config = parent.join("config.toml");
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .arg("--apply")
        .arg("--register")
        .env("OKMATE_CONFIG", &config)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let saved = fs::read_to_string(&config).unwrap();
    assert!(saved.contains("id = \"acme\""), "{saved}");
    assert!(!saved.contains("id = \"docs\""), "{saved}");
}

#[test]
fn json_includes_extra_targets() {
    let repo = temp_dir("init-json-target");
    git_init(&repo);
    let target = repo.join("docs");
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .arg("--agents")
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let files = json_files(&String::from_utf8_lossy(&output.stdout));
    let agents = files
        .iter()
        .find(|file| file["relative"] == "AGENTS.md")
        .expect("AGENTS.md");
    let target_path = agents["target"].as_str().expect("target");
    assert!(target_path.ends_with("AGENTS.md"), "{target_path}");
    assert_ne!(agents["target"], agents["relative"]);
}

#[test]
fn root_agents_is_rejected_before_writes() {
    let repo = temp_dir("init-root-agents");
    git_init(&repo);
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&repo)
        .arg("--apply")
        .arg("--agents")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(text.contains("AGENTS.md"), "{text}");
    assert!(text.contains("child"), "{text}");
    assert!(!repo.join("index.md").exists());
    assert!(!repo.join("AGENTS.md").exists());
}

#[test]
fn root_bundle_without_agents_is_allowed() {
    let repo = temp_dir("init-root-ok");
    git_init(&repo);
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&repo)
        .arg("--apply")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(repo.join("index.md").is_file());
    let log = fs::read_to_string(repo.join("log.md")).unwrap();
    assert!(
        !log.contains("union driver"),
        "log should not claim a union driver without --agents: {log}"
    );
    let (ok, json) = check_json(&repo);
    assert!(ok, "{json}");
}

#[test]
fn existing_markdown_fails_before_mutation() {
    let repo = temp_dir("init-readme");
    git_init(&repo);
    fs::write(repo.join("README.md"), "# Hello\n").unwrap();
    let config = repo.join("config.toml");
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&repo)
        .arg("--apply")
        .arg("--register")
        .arg("--id")
        .arg("blocked")
        .env("OKMATE_CONFIG", &config)
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(!repo.join("index.md").exists());
    assert!(!config.exists() || !fs::read_to_string(&config).unwrap().contains("blocked"));
}

#[test]
fn no_git_apply_still_writes() {
    let parent = temp_dir("init-nogit-apply");
    let target = parent.join("knowledge");
    let output = Command::new(okmate_bin())
        .arg("init")
        .arg(&target)
        .arg("--apply")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(target.join("index.md").is_file());
    let log = fs::read_to_string(target.join("log.md")).unwrap();
    assert!(!log.contains("union driver"), "{log}");
}

fn git_init(dir: &std::path::Path) {
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(["init", "-b", "main"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git init failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
