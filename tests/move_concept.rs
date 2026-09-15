mod common;

use std::fs;
use std::process::Command;

use common::{okmate_bin, temp_dir, write_index};

fn write_collection(root: &std::path::Path, dir: &str, heading: &str) {
    let path = root.join(dir);
    fs::create_dir_all(&path).unwrap();
    fs::write(
        path.join("index.md"),
        format!("# {heading}\n\nExisting grouping.\n"),
    )
    .unwrap();
}

fn write_concept(root: &std::path::Path, id: &str, title: &str, extra_yaml: &str, body: &str) {
    let path = format!("{id}.md");
    if let Some(parent) = std::path::Path::new(&path).parent() {
        fs::create_dir_all(root.join(parent)).unwrap();
    }
    fs::write(
        root.join(&path),
        format!(
            "---\ntype: Explanation\ntitle: {title}\ndescription: {title} record.\n{extra_yaml}---\n\n# {title}\n\n{body}\n"
        ),
    )
    .unwrap();
}

fn okmate() -> Command {
    Command::new(okmate_bin())
}

#[test]
fn move_dry_run_does_not_write() {
    let root = temp_dir("move-dry");
    write_index(&root);
    write_collection(&root, "research", "Research");
    write_collection(&root, "audits", "Audits");
    write_concept(
        &root,
        "research/topic",
        "Topic",
        "",
        "See [self](/research/topic.md).\n",
    );
    fs::write(
        root.join("research/index.md"),
        "# Research\n\n* [Topic](topic.md) - Topic record.\n",
    )
    .unwrap();

    let output = okmate()
        .arg("move")
        .arg(&root)
        .arg("--from")
        .arg("research/topic")
        .arg("--to")
        .arg("audits/")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Would move"), "{stdout}");
    assert!(stdout.contains("Dry run"), "{stdout}");
    assert!(root.join("research/topic.md").exists());
    assert!(!root.join("audits/topic.md").exists());
}

#[test]
fn move_apply_rewrites_links_and_indexes() {
    let root = temp_dir("move-apply");
    write_index(&root);
    write_collection(&root, "research", "Research");
    write_collection(&root, "audits", "Audits");
    write_concept(
        &root,
        "research/topic",
        "Topic",
        "sources:\n  - id: code\n    resource: ../../src/cli.rs\n",
        "Relative [peer](../guides/intro.md#setup) and root [peer](/guides/intro.md#setup).\n",
    );
    write_concept(
        &root,
        "guides/intro",
        "Intro",
        "",
        "See [topic](/research/topic.md#heading) and also [rel](../research/topic.md).\n",
    );
    write_collection(&root, "guides", "Guides");
    fs::write(
        root.join("research/index.md"),
        "# Research\n\n* [Topic](topic.md) - Topic record.\n",
    )
    .unwrap();
    fs::write(
        root.join("guides/index.md"),
        "# Guides\n\n* [Intro](intro.md) - Intro record.\n",
    )
    .unwrap();
    fs::write(
        root.join("okmate.toml"),
        "[type_paths]\nExplanation = \"guides\"\n",
    )
    .unwrap();

    let output = okmate()
        .arg("move")
        .arg(&root)
        .arg("--from")
        .arg("research/topic")
        .arg("--to")
        .arg("audits/")
        .arg("--apply")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!root.join("research/topic.md").exists());
    let moved = fs::read_to_string(root.join("audits/topic.md")).unwrap();
    assert!(
        moved.contains("[peer](../guides/intro.md#setup)"),
        "{moved}"
    );
    assert!(moved.contains("[peer](/guides/intro.md#setup)"), "{moved}");
    assert!(moved.contains("resource: ../../src/cli.rs"), "{moved}");

    let intro = fs::read_to_string(root.join("guides/intro.md")).unwrap();
    assert!(
        intro.contains("[topic](/audits/topic.md#heading)"),
        "{intro}"
    );
    assert!(intro.contains("[rel](../audits/topic.md)"), "{intro}");

    let research_index = fs::read_to_string(root.join("research/index.md")).unwrap();
    assert!(!research_index.contains("topic.md"), "{research_index}");
    let audits_index = fs::read_to_string(root.join("audits/index.md")).unwrap();
    assert!(audits_index.contains("(topic.md)"), "{audits_index}");
    assert!(audits_index.contains("Topic"), "{audits_index}");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("type_paths expects `Explanation` under `guides/`"),
        "{stdout}"
    );
}

#[test]
fn move_to_collection_keeps_stem() {
    let root = temp_dir("move-stem");
    write_index(&root);
    write_collection(&root, "research", "Research");
    write_collection(&root, "audits", "Audits");
    write_concept(&root, "research/topic", "Topic", "", "Body.\n");
    fs::write(
        root.join("research/index.md"),
        "# Research\n\n* [Topic](topic.md) - Topic record.\n",
    )
    .unwrap();

    let output = okmate()
        .arg("move")
        .arg(&root)
        .arg("--from")
        .arg("research/topic")
        .arg("--to")
        .arg("audits/")
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"to\": \"audits/topic\""), "{stdout}");
}

#[test]
fn move_dest_collision_fails() {
    let root = temp_dir("move-collide");
    write_index(&root);
    write_collection(&root, "research", "Research");
    write_collection(&root, "audits", "Audits");
    write_concept(&root, "research/topic", "Topic", "", "Body.\n");
    write_concept(&root, "audits/topic", "Existing", "", "Body.\n");

    let output = okmate()
        .arg("move")
        .arg(&root)
        .arg("--from")
        .arg("research/topic")
        .arg("--to")
        .arg("audits/")
        .arg("--apply")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("already exists"), "{stderr}");
}

#[test]
fn move_unlinked_mention_is_a_note() {
    let root = temp_dir("move-mention");
    write_index(&root);
    write_collection(&root, "research", "Research");
    write_collection(&root, "audits", "Audits");
    write_concept(&root, "research/topic", "Topic", "", "Body.\n");
    write_concept(
        &root,
        "guides/intro",
        "Intro",
        "",
        "The old path research/topic is cited in prose.\n",
    );
    write_collection(&root, "guides", "Guides");
    fs::write(
        root.join("research/index.md"),
        "# Research\n\n* [Topic](topic.md) - Topic record.\n",
    )
    .unwrap();

    let output = okmate()
        .arg("move")
        .arg(&root)
        .arg("--from")
        .arg("research/topic")
        .arg("--to")
        .arg("audits/")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("unlinked mention"), "{stdout}");
    assert!(stdout.contains("guides/intro.md"), "{stdout}");
}
