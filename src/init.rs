use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use okf::Profile;
use serde::Serialize;

use crate::CheckFormat;
use crate::config::{self, valid_id};

#[derive(Clone, Debug)]
pub struct InitOptions {
    pub path: PathBuf,
    pub title: String,
    pub bare: bool,
    pub apply: bool,
    pub format: CheckFormat,
    pub register: bool,
    pub id: Option<String>,
    pub agents: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct InitPlan {
    pub root: PathBuf,
    pub files: Vec<InitFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register: Option<RegisterIntent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gitattributes_hint: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct RegisterIntent {
    pub id: String,
    pub path: String,
    pub incoming: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct InitFile {
    pub relative: PathBuf,
    pub op: InitOp,
    pub bytes_hint: usize,
    #[serde(skip)]
    pub contents: String,
    #[serde(skip)]
    pub dest_root: PathBuf,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InitOp {
    Create,
    Skip,
}

const COLLECTIONS: &[(&str, &str, &str)] = &[
    (
        "architecture",
        "Architecture",
        "Current system contracts and boundaries.",
    ),
    (
        "decisions",
        "Decisions",
        "Implemented and approved choices with their consequences.",
    ),
    (
        "status",
        "Status",
        "Dated implementation state and known limitations.",
    ),
    (
        "plans",
        "Plans",
        "Active and migrated implementation plans.",
    ),
    (
        "research",
        "Research",
        "Non-normative evidence and synthesis.",
    ),
    ("audits", "Audits", "Findings against current behavior."),
];

pub fn run(opts: InitOptions) -> Result<()> {
    let plan = plan_bundle(&opts)?;
    if opts.apply {
        apply_plan(&plan)?;
        if let Some(register) = &plan.register {
            save_register(register, &plan.root)?;
        }
        let report = crate::check(&plan.root, Profile::Strict)?;
        if report.has_errors() {
            let formatted = report.terminal();
            if !formatted.is_empty() {
                eprintln!("{formatted}");
            }
            bail!("initialized bundle failed strict check");
        }
    }
    match opts.format {
        CheckFormat::Json => println!("{}", serde_json::to_string_pretty(&plan)?),
        CheckFormat::Terminal => print_terminal(&plan, opts.apply),
    }
    Ok(())
}

pub fn apply_plan(plan: &InitPlan) -> Result<()> {
    for file in &plan.files {
        if file.op != InitOp::Create {
            continue;
        }
        let dest = file.dest_root.join(&file.relative);
        if dest.exists() {
            bail!("{} already exists; refusing to overwrite", dest.display());
        }
    }
    for file in &plan.files {
        if file.op != InitOp::Create {
            continue;
        }
        let dest = file.dest_root.join(&file.relative);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }
        let mut out = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&dest)
            .with_context(|| format!("failed to create {}", dest.display()))?;
        out.write_all(file.contents.as_bytes())
            .with_context(|| format!("failed to write {}", dest.display()))?;
    }
    Ok(())
}

pub fn plan_bundle(opts: &InitOptions) -> Result<InitPlan> {
    if opts.id.is_some() && !opts.register {
        bail!("`--id` requires `--register`");
    }

    let root = opts.path.clone();
    let index = root.join("index.md");
    if index.is_file() {
        let source = fs::read_to_string(&index)
            .with_context(|| format!("failed to read {}", index.display()))?;
        if has_okf_version(&source) {
            bail!(
                "{} is already an OKF bundle (`index.md` has `okf_version`)",
                root.display()
            );
        }
    }

    let git_root = if opts.agents {
        Some(git_toplevel(&root)?)
    } else {
        None
    };

    let register = if opts.register {
        Some(plan_register(opts, &root)?)
    } else {
        None
    };

    let mut files = Vec::new();
    files.push(create_file(
        &root,
        "index.md",
        root_index(&opts.title, opts.bare),
    ));
    files.push(create_file(&root, "log.md", log_markdown()));
    if !opts.bare {
        for (dir, heading, _) in COLLECTIONS {
            files.push(create_file(
                &root,
                &format!("{dir}/index.md"),
                collection_index(heading, dir),
            ));
        }
    }

    let mut gitattributes_hint = None;
    if let Some(git) = &git_root {
        for (relative, body) in agent_files() {
            let dest = git.join(relative);
            if dest.exists() {
                files.push(skip_file(git, relative));
            } else {
                files.push(create_file(git, relative, body));
            }
        }
        let attr_line = gitattributes_line(git, &root);
        if git.join(".gitattributes").exists() {
            files.push(skip_file(git, ".gitattributes"));
            gitattributes_hint = Some(attr_line);
        } else {
            files.push(create_file(git, ".gitattributes", format!("{attr_line}\n")));
        }
    }

    Ok(InitPlan {
        root,
        files,
        register,
        gitattributes_hint,
    })
}

fn save_register(register: &RegisterIntent, root: &Path) -> Result<()> {
    let path = config::config_path();
    let mut config = config::load_or_default(&path);
    let stored = fs::canonicalize(root)
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| register.path.clone());
    config::push_directory_root(&mut config, &register.id, &stored)?;
    config::save(&config, &path)?;
    Ok(())
}

fn plan_register(opts: &InitOptions, root: &Path) -> Result<RegisterIntent> {
    let id = register_id(opts, root)?;
    let config = config::load_or_default(&config::config_path());
    if config.roots.iter().any(|root| root.id() == id) {
        bail!("duplicate root id `{id}`");
    }
    Ok(RegisterIntent {
        id,
        path: absolute_path(root).display().to_string(),
        incoming: "allow".into(),
    })
}

fn register_id(opts: &InitOptions, root: &Path) -> Result<String> {
    if let Some(id) = &opts.id {
        if !valid_id(id) {
            bail!("invalid root id `{id}`");
        }
        return Ok(id.clone());
    }
    let abs = absolute_path(root);
    let name = abs
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("knowledge");
    let id = kebab_case(name);
    if !valid_id(&id) {
        bail!("could not derive a valid root id from `{name}`; pass --id");
    }
    Ok(id)
}

fn kebab_case(name: &str) -> String {
    let mut out = String::new();
    let mut pending = false;
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            if pending && !out.is_empty() {
                out.push('-');
            }
            out.push(ch.to_ascii_lowercase());
            pending = false;
        } else {
            pending = true;
        }
    }
    out
}

fn git_toplevel(bundle: &Path) -> Result<PathBuf> {
    let cwd = git_probe_dir(bundle);
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(&cwd)
        .output()
        .context("failed to run git rev-parse --show-toplevel")?;
    if !output.status.success() {
        bail!("`--agents` requires a git working tree (`git rev-parse --show-toplevel` failed)");
    }
    let path = String::from_utf8(output.stdout)
        .context("git toplevel was not UTF-8")?
        .trim()
        .to_string();
    if path.is_empty() {
        bail!("`--agents` requires a git working tree");
    }
    Ok(PathBuf::from(path))
}

fn git_probe_dir(path: &Path) -> PathBuf {
    if path.is_dir() {
        return path.to_path_buf();
    }
    match path.parent() {
        Some(parent) if !parent.as_os_str().is_empty() && parent.is_dir() => parent.to_path_buf(),
        Some(parent) if !parent.as_os_str().is_empty() => git_probe_dir(parent),
        _ => std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
    }
}

fn gitattributes_line(git: &Path, bundle: &Path) -> String {
    let git_abs = fs::canonicalize(git).unwrap_or_else(|_| absolute_path(git));
    let bundle_abs = existing_or_absolute(bundle);
    let rel = match bundle_abs.strip_prefix(&git_abs) {
        Ok(rel) if rel.as_os_str().is_empty() => PathBuf::from("log.md"),
        Ok(rel) => rel.join("log.md"),
        Err(_) => PathBuf::from("log.md"),
    };
    format!("{} merge=union", rel.display())
}

fn existing_or_absolute(path: &Path) -> PathBuf {
    let abs = absolute_path(path);
    if let Ok(canonical) = fs::canonicalize(&abs) {
        return canonical;
    }
    if let Some(parent) = abs.parent()
        && let Ok(canonical_parent) = fs::canonicalize(parent)
    {
        return canonical_parent.join(abs.file_name().unwrap_or_default());
    }
    abs
}

fn absolute_path(path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(path)
    }
}

fn agent_files() -> [(&'static str, String); 3] {
    [
        ("AGENTS.md", agents_md()),
        (".cursor/rules/write-knowledge.mdc", write_knowledge_mdc()),
        (
            ".agents/skills/manage-knowledge/SKILL.md",
            manage_skill_md(),
        ),
    ]
}

fn agents_md() -> String {
    "# Agent instructions\n\
     \n\
     - Keep `knowledge/**/*.md` inert Markdown with OKF YAML.\n\
     - After knowledge edits: `okmate check knowledge --profile strict`.\n\
     - Durable plans, reports, audits, and status belong in `knowledge/`.\n\
     - Follow `.cursor/rules/write-knowledge.mdc` and\n\
       `.agents/skills/manage-knowledge/SKILL.md`.\n"
        .into()
}

fn write_knowledge_mdc() -> String {
    "---\n\
     description: >-\n\
       Write reports, plans, research, audits, and other durable project writing as\n\
       OKF records in knowledge/.\n\
     alwaysApply: false\n\
     ---\n\
     \n\
     # Write to the knowledge database\n\
     \n\
     Default destination is an OKF Markdown record under this repository's\n\
     `knowledge/`. Do not leave the document in chat, at the repo root, or as a\n\
     loose Markdown file, unless the user names that location.\n\
     \n\
     Follow `.agents/skills/manage-knowledge/SKILL.md` for retrieve, author, and\n\
     validate. Keep `knowledge/**/*.md` inert Markdown with OKF YAML.\n\
     \n\
     ## Collection by intent\n\
     \n\
     | User says | Collection | `type` | Default `authority` |\n\
     | --- | --- | --- | --- |\n\
     | write a plan / implementation plan | `knowledge/plans/` | Implementation Plan | exploratory |\n\
     | write a report / research | `knowledge/research/` | Research Report | exploratory |\n\
     | audit / findings vs current behavior | `knowledge/audits/` | Audit | descriptive |\n\
     | status snapshot / results | `knowledge/status/` | Status | descriptive |\n\
     \n\
     Architecture and decisions stay at `knowledge/architecture/` and\n\
     `knowledge/decisions/`. Do not start implementation plan phases unless the\n\
     user asks.\n"
        .into()
}

fn manage_skill_md() -> String {
    "---\n\
     name: manage-knowledge\n\
     description: Query, inspect, validate, author, or review a local OKF knowledge bundle with okmate.\n\
     ---\n\
     \n\
     # Manage knowledge\n\
     \n\
     Use the checked-in `knowledge/` bundle as the canonical database and `okmate`\n\
     as its deterministic interface.\n\
     \n\
     Keep `knowledge/**/*.md` inert Markdown with OKF YAML. Do not add executable\n\
     declarations.\n\
     \n\
     ## Validate\n\
     \n\
     ```sh\n\
     okmate check knowledge --profile strict\n\
     ```\n\
     \n\
     ## Retrieve\n\
     \n\
     ```sh\n\
     okmate inspect --profile strict catalog knowledge\n\
     ```\n\
     \n\
     Search authored records with `rg` when the concept ID is unknown. Inspect a\n\
     known concept with `okmate inspect --profile strict concept CONCEPT_ID knowledge`.\n\
     \n\
     ## Author\n\
     \n\
     Write durable plans, reports, audits, and status as OKF records under\n\
     `knowledge/`. Follow `.cursor/rules/write-knowledge.mdc` for collection\n\
     routing. Do not start implementation plan phases unless the user asks.\n"
        .into()
}

fn print_terminal(plan: &InitPlan, applied: bool) {
    let creates = plan
        .files
        .iter()
        .filter(|file| file.op == InitOp::Create)
        .count();
    if applied {
        println!("Created {creates} files under {}", plan.root.display());
        for file in &plan.files {
            match file.op {
                InitOp::Create => println!("  wrote  {}", file.relative.display()),
                InitOp::Skip => println!("  skipped  {} (already exists)", file.relative.display()),
            }
        }
        if let Some(register) = &plan.register {
            println!(
                "Registered directory root `{}` at {} (incoming = {})",
                register.id, register.path, register.incoming
            );
        }
        if let Some(hint) = &plan.gitattributes_hint {
            println!("Add to `.gitattributes`: {hint}");
        }
        println!(
            "Next: `okmate check {}` then `okmate view {}`",
            plan.root.display(),
            plan.root.display()
        );
        return;
    }
    println!("Would create {creates} files under {}", plan.root.display());
    for file in &plan.files {
        match file.op {
            InitOp::Create => println!(
                "  create  {}  ({} bytes)",
                file.relative.display(),
                file.bytes_hint
            ),
            InitOp::Skip => println!("  skip    {} (already exists)", file.relative.display()),
        }
    }
    if let Some(register) = &plan.register {
        println!(
            "Would register directory root `{}` at {} (incoming = {})",
            register.id, register.path, register.incoming
        );
    }
    if let Some(hint) = &plan.gitattributes_hint {
        println!("Add to `.gitattributes`: {hint}");
    }
    println!("Dry run; pass --apply to write.");
}

fn create_file(dest_root: &Path, relative: &str, contents: String) -> InitFile {
    InitFile {
        relative: PathBuf::from(relative),
        op: InitOp::Create,
        bytes_hint: contents.len(),
        contents,
        dest_root: dest_root.to_path_buf(),
    }
}

fn skip_file(dest_root: &Path, relative: &str) -> InitFile {
    InitFile {
        relative: PathBuf::from(relative),
        op: InitOp::Skip,
        bytes_hint: 0,
        contents: String::new(),
        dest_root: dest_root.to_path_buf(),
    }
}

fn root_index(title: &str, bare: bool) -> String {
    let mut body = format!("---\nokf_version: \"0.2\"\n---\n\n# {title}\n");
    if !bare {
        body.push('\n');
        for (dir, heading, blurb) in COLLECTIONS {
            body.push_str(&format!("* [{heading}]({dir}/) - {blurb}\n"));
        }
    }
    body
}

fn collection_index(heading: &str, dir: &str) -> String {
    let body = if dir == "audits" {
        "Reserved for findings against current behavior."
    } else {
        "No records yet."
    };
    format!("# {heading}\n\n{body}\n")
}

fn log_markdown() -> String {
    let today = utc_date();
    format!(
        "# Knowledge log\n\
         \n\
         Git merges this file with the built-in `union` driver (see `.gitattributes`).\n\
         Independent bullets under the same `## YYYY-MM-DD` heading combine instead of\n\
         conflicting. Add a new list item; do not reword another session's bullet in\n\
         the same change.\n\
         \n\
         ## {today}\n\
         \n\
         - Initialized this OKF v0.2 bundle with `okmate init`.\n"
    )
}

fn has_okf_version(source: &str) -> bool {
    let Some(yaml) = frontmatter_yaml(source) else {
        return false;
    };
    yaml.lines().any(|line| {
        let line = line.trim();
        line == "okf_version:" || line.starts_with("okf_version:")
    })
}

fn frontmatter_yaml(source: &str) -> Option<&str> {
    let mut lines = source.split_inclusive('\n');
    let first = lines.next()?;
    if first.trim_end_matches(['\r', '\n']) != "---" {
        return None;
    }
    let start = first.len();
    let mut offset = start;
    for line in lines {
        if line.trim_end_matches(['\r', '\n']) == "---" {
            return Some(&source[start..offset]);
        }
        offset += line.len();
    }
    None
}

fn utc_date() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let days = (seconds / 86_400) as i64 + 719_468;
    let era = if days >= 0 { days } else { days - 146_096 } / 146_097;
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02}")
}
