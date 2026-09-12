use std::collections::BTreeSet;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use okf::Profile;
use serde::{Deserialize, Serialize};

use crate::CheckFormat;
use crate::config::{self, valid_id};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InitTemplate {
    Minimal,
    SoftwareProject,
}

#[derive(Clone, Debug)]
pub struct InitOptions {
    pub path: PathBuf,
    pub title: String,
    pub bare: bool,
    pub template: Option<InitTemplate>,
    pub collections: Vec<String>,
    pub template_file: Option<PathBuf>,
    pub apply: bool,
    pub format: CheckFormat,
    pub register: bool,
    pub id: Option<String>,
    pub agents: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct InitPlan {
    pub root: PathBuf,
    pub template: String,
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
    pub target: PathBuf,
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

struct CollectionSpec {
    path: String,
    heading: String,
    blurb: String,
}

const SOFTWARE_PROJECT: &[(&str, &str, &str)] = &[
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

const GENERIC_BASENAMES: &[&str] = &["knowledge", "docs", "okf", "kb"];

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct TemplateFile {
    #[serde(default)]
    collections: Vec<TemplateCollection>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct TemplateCollection {
    path: String,
    heading: Option<String>,
    blurb: Option<String>,
}

pub fn run(opts: InitOptions) -> Result<()> {
    let plan = plan_bundle(&opts)?;
    preflight_staged_bundle(&plan)?;
    if opts.apply {
        let written = apply_plan(&plan)?;
        let report = crate::check(&plan.root, Profile::Strict)?;
        if report.has_errors() {
            let formatted = report.terminal();
            if !formatted.is_empty() {
                eprintln!("{formatted}");
            }
            bail!(
                "initialized bundle failed strict check; files were written under {} and were not registered",
                plan.root.display()
            );
        }
        if let Some(register) = &plan.register
            && let Err(error) = save_register(register, &plan.root)
        {
            bail!(
                "bundle files were written but registration failed: {error}; written: {}",
                display_written(&written)
            );
        }
    }
    match opts.format {
        CheckFormat::Json => println!("{}", serde_json::to_string_pretty(&plan)?),
        CheckFormat::Terminal => print_terminal(&plan, opts.apply),
    }
    Ok(())
}

pub fn apply_plan(plan: &InitPlan) -> Result<Vec<PathBuf>> {
    for file in &plan.files {
        if file.op != InitOp::Create {
            continue;
        }
        if file.target.exists() {
            bail!(
                "{} already exists; refusing to overwrite",
                file.target.display()
            );
        }
    }
    let mut written = Vec::new();
    for file in &plan.files {
        if file.op != InitOp::Create {
            continue;
        }
        if let Some(parent) = file.target.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }
        let mut out = match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&file.target)
        {
            Ok(out) => out,
            Err(error) => {
                bail!(
                    "failed to create {}: {error}. Files already created: {}. Registration was not changed.",
                    file.target.display(),
                    display_written(&written)
                );
            }
        };
        if let Err(error) = out.write_all(file.contents.as_bytes()) {
            written.push(file.target.clone());
            bail!(
                "failed to write {}: {error}. Files already created: {}. Registration was not changed.",
                file.target.display(),
                display_written(&written)
            );
        }
        written.push(file.target.clone());
    }
    Ok(written)
}

pub fn plan_bundle(opts: &InitOptions) -> Result<InitPlan> {
    if opts.id.is_some() && !opts.register {
        bail!("`--id` requires `--register`");
    }

    let collections = resolve_collections(opts)?;
    let template_name = template_name(opts);

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
        let git = git_toplevel(&root)?;
        if same_existing_dir(&git, &root) {
            bail!(
                "`--agents` would write untyped `AGENTS.md` into a repository-root bundle. \
                 Use a child directory (`okmate init knowledge --agents`) or keep agent \
                 instructions outside the corpus."
            );
        }
        Some(git)
    } else {
        None
    };

    let register = if opts.register {
        Some(plan_register(opts, &root)?)
    } else {
        None
    };

    let union_log = match &git_root {
        Some(git) => union_log_state(git, &root),
        None => UnionLog::None,
    };

    let mut files = Vec::new();
    files.push(create_file(
        &root,
        "index.md",
        root_index(&opts.title, &collections),
    ));
    files.push(create_file(&root, "log.md", log_markdown(union_log)));
    for collection in &collections {
        files.push(create_file(
            &root,
            &format!("{}/index.md", collection.path),
            collection_index(collection),
        ));
    }

    let mut gitattributes_hint = None;
    if let Some(git) = &git_root {
        let bundle_rel = bundle_relpath(git, &root);
        for (relative, body) in agent_files(&bundle_rel) {
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
            if union_log != UnionLog::Observed {
                gitattributes_hint = Some(attr_line);
            }
        } else {
            files.push(create_file(git, ".gitattributes", format!("{attr_line}\n")));
        }
    }

    preflight_collisions(&files)?;

    Ok(InitPlan {
        root,
        template: template_name,
        files,
        register,
        gitattributes_hint,
    })
}

fn display_written(written: &[PathBuf]) -> String {
    if written.is_empty() {
        "none".into()
    } else {
        written
            .iter()
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn template_name(opts: &InitOptions) -> String {
    if opts.template_file.is_some() {
        "custom".into()
    } else if opts.template == Some(InitTemplate::SoftwareProject) {
        "software-project".into()
    } else {
        "minimal".into()
    }
}

fn resolve_collections(opts: &InitOptions) -> Result<Vec<CollectionSpec>> {
    if opts.bare {
        if opts.template == Some(InitTemplate::SoftwareProject) {
            bail!(
                "`--bare` is an alias for `--template minimal` and cannot be combined with `--template software-project`"
            );
        }
        if opts.template_file.is_some() {
            bail!("`--bare` cannot be combined with `--template-file`");
        }
        if !opts.collections.is_empty() {
            bail!("`--bare` cannot be combined with `--collection`");
        }
        return Ok(Vec::new());
    }
    if opts.template.is_some() && opts.template_file.is_some() {
        bail!("`--template` and `--template-file` cannot be combined");
    }

    let mut collections = match (&opts.template, &opts.template_file) {
        (None, None) | (Some(InitTemplate::Minimal), None) => Vec::new(),
        (Some(InitTemplate::SoftwareProject), None) => software_project_collections(),
        (None, Some(path)) => load_template_file(path)?,
        (Some(_), Some(_)) => unreachable!(),
    };

    for raw in &opts.collections {
        collections.push(collection_from_path(raw)?);
    }

    let mut seen = BTreeSet::new();
    for collection in &collections {
        if !seen.insert(collection.path.clone()) {
            bail!("duplicate collection path `{}`", collection.path);
        }
    }
    Ok(collections)
}

fn software_project_collections() -> Vec<CollectionSpec> {
    SOFTWARE_PROJECT
        .iter()
        .map(|(path, heading, blurb)| CollectionSpec {
            path: (*path).to_string(),
            heading: (*heading).to_string(),
            blurb: (*blurb).to_string(),
        })
        .collect()
}

fn load_template_file(path: &Path) -> Result<Vec<CollectionSpec>> {
    if !path.is_file() {
        bail!("template file {} does not exist", path.display());
    }
    let source =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    let parsed: TemplateFile =
        toml::from_str(&source).with_context(|| format!("failed to parse {}", path.display()))?;
    parsed
        .collections
        .into_iter()
        .map(|entry| {
            let path = parse_collection_path(&entry.path)?;
            Ok(CollectionSpec {
                heading: entry
                    .heading
                    .filter(|heading| !heading.trim().is_empty())
                    .unwrap_or_else(|| heading_from_path(&path)),
                blurb: entry
                    .blurb
                    .filter(|blurb| !blurb.trim().is_empty())
                    .unwrap_or_else(|| "No records yet.".to_string()),
                path,
            })
        })
        .collect()
}

fn collection_from_path(raw: &str) -> Result<CollectionSpec> {
    let path = parse_collection_path(raw)?;
    Ok(CollectionSpec {
        heading: heading_from_path(&path),
        blurb: "No records yet.".into(),
        path,
    })
}

fn parse_collection_path(raw: &str) -> Result<String> {
    let normalized = raw.replace('\\', "/");
    let path = Path::new(&normalized);
    if path.is_absolute() {
        bail!("collection path `{raw}` must be relative to the bundle");
    }
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => {
                let part = part.to_string_lossy();
                if part.is_empty() || part == "." || part == ".." {
                    bail!("collection path `{raw}` must not contain `.` or `..`");
                }
                parts.push(part.into_owned());
            }
            Component::CurDir | Component::ParentDir => {
                bail!("collection path `{raw}` must not contain `.` or `..`");
            }
            Component::Prefix(_) | Component::RootDir => {
                bail!("collection path `{raw}` must be relative to the bundle");
            }
        }
    }
    if parts.is_empty() {
        bail!("collection path `{raw}` must be a relative directory");
    }
    Ok(parts.join("/"))
}

fn heading_from_path(path: &str) -> String {
    let last = path.rsplit('/').next().unwrap_or(path);
    last.split(['-', '_'])
        .filter(|word| !word.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn preflight_collisions(files: &[InitFile]) -> Result<()> {
    for file in files {
        if file.op == InitOp::Create && file.target.exists() {
            bail!(
                "{} already exists; refusing to overwrite",
                file.target.display()
            );
        }
    }
    Ok(())
}

fn preflight_staged_bundle(plan: &InitPlan) -> Result<()> {
    let stage = unique_temp("okmate-init-stage");
    let result = (|| {
        fs::create_dir_all(&stage)
            .with_context(|| format!("failed to create {}", stage.display()))?;
        for file in &plan.files {
            if file.dest_root != plan.root || file.op != InitOp::Create {
                continue;
            }
            let dest = stage.join(&file.relative);
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&dest, &file.contents)?;
        }
        copy_existing_markdown(&plan.root, &stage, plan)?;
        let report = crate::check(&stage, Profile::Strict)?;
        if report.has_errors() {
            let formatted = report.terminal();
            if !formatted.is_empty() {
                eprintln!("{formatted}");
            }
            bail!(
                "planned bundle failed strict check; no files were written. \
                 Use a child directory if the target already contains ordinary Markdown \
                 (README, AGENTS, CONTRIBUTING)."
            );
        }
        Ok(())
    })();
    let _ = fs::remove_dir_all(&stage);
    result
}

fn copy_existing_markdown(src: &Path, dest: &Path, plan: &InitPlan) -> Result<()> {
    if !src.is_dir() {
        return Ok(());
    }
    let planned: BTreeSet<String> = plan
        .files
        .iter()
        .filter(|file| file.dest_root == plan.root)
        .map(|file| file.relative.to_string_lossy().replace('\\', "/"))
        .collect();
    let mut found = Vec::new();
    collect_markdown(src, src, &mut found)?;
    for relative in found {
        if planned.contains(&relative) {
            continue;
        }
        let from = src.join(&relative);
        let to = dest.join(&relative);
        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(&from, &to).with_context(|| format!("failed to stage {}", from.display()))?;
    }
    Ok(())
}

fn collect_markdown(root: &Path, directory: &Path, out: &mut Vec<String>) -> Result<()> {
    let mut entries = fs::read_dir(directory)
        .with_context(|| format!("failed to read {}", directory.display()))?
        .collect::<std::io::Result<Vec<_>>>()?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        if entry.file_name().as_encoded_bytes().starts_with(b".") {
            continue;
        }
        let path = entry.path();
        if path.is_dir() {
            collect_markdown(root, &path, out)?;
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("md") {
            out.push(relative_path(root, &path));
        }
    }
    Ok(())
}

fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn unique_temp(prefix: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    std::env::temp_dir().join(format!("{prefix}-{}-{nonce}", std::process::id()))
}

fn save_register(register: &RegisterIntent, root: &Path) -> Result<()> {
    let path = config::config_path();
    let mut config = config::load_or_default(&path);
    let stored = fs::canonicalize(root)
        .map(|path| path.display().to_string())
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
        .and_then(|name| name.to_str())
        .unwrap_or("knowledge");
    if !is_generic_basename(name) {
        let id = kebab_case(name);
        if valid_id(&id) {
            return Ok(id);
        }
        bail!("could not derive a valid root id from `{name}`; pass --id");
    }
    if let Ok(git) = git_toplevel(root)
        && let Some(repo) = git.file_name().and_then(|name| name.to_str())
        && !is_generic_basename(repo)
    {
        let id = kebab_case(repo);
        if valid_id(&id) {
            return Ok(id);
        }
    }
    let id = kebab_case(&opts.title);
    if valid_id(&id) {
        return Ok(id);
    }
    bail!("could not derive a valid root id from `{name}`; pass --id");
}

fn is_generic_basename(name: &str) -> bool {
    let folded = name.trim_start_matches('.').to_ascii_lowercase();
    GENERIC_BASENAMES.contains(&folded.as_str())
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

fn same_existing_dir(git: &Path, bundle: &Path) -> bool {
    let git_abs = fs::canonicalize(git).unwrap_or_else(|_| absolute_path(git));
    let bundle_abs = existing_or_absolute(bundle);
    if let Ok(canonical_bundle) = fs::canonicalize(&bundle_abs) {
        return canonical_bundle == git_abs;
    }
    bundle_abs == git_abs
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum UnionLog {
    None,
    Written,
    Observed,
}

fn union_log_state(git: &Path, bundle: &Path) -> UnionLog {
    let line = gitattributes_line(git, bundle);
    let path = git.join(".gitattributes");
    if !path.exists() {
        return UnionLog::Written;
    }
    let Ok(existing) = fs::read_to_string(&path) else {
        return UnionLog::None;
    };
    if existing.lines().any(|candidate| candidate.trim() == line) {
        UnionLog::Observed
    } else {
        UnionLog::None
    }
}

fn gitattributes_line(git: &Path, bundle: &Path) -> String {
    let rel = log_relpath(git, bundle);
    format!("{} merge=union", gitattributes_quote(&rel))
}

fn log_relpath(git: &Path, bundle: &Path) -> String {
    let git_abs = fs::canonicalize(git).unwrap_or_else(|_| absolute_path(git));
    let bundle_abs = existing_or_absolute(bundle);
    let rel = match bundle_abs.strip_prefix(&git_abs) {
        Ok(rel) if rel.as_os_str().is_empty() => PathBuf::from("log.md"),
        Ok(rel) => rel.join("log.md"),
        Err(_) => PathBuf::from("log.md"),
    };
    rel.to_string_lossy().replace('\\', "/")
}

fn bundle_relpath(git: &Path, bundle: &Path) -> String {
    let git_abs = fs::canonicalize(git).unwrap_or_else(|_| absolute_path(git));
    let bundle_abs = existing_or_absolute(bundle);
    match bundle_abs.strip_prefix(&git_abs) {
        Ok(rel) if rel.as_os_str().is_empty() => ".".into(),
        Ok(rel) => rel.to_string_lossy().replace('\\', "/"),
        Err(_) => bundle.to_string_lossy().replace('\\', "/"),
    }
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

fn agent_files(bundle_rel: &str) -> [(&'static str, String); 3] {
    [
        ("AGENTS.md", agents_md(bundle_rel)),
        (
            ".cursor/rules/write-knowledge.mdc",
            write_knowledge_mdc(bundle_rel),
        ),
        (
            ".agents/skills/manage-knowledge/SKILL.md",
            manage_skill_md(bundle_rel),
        ),
    ]
}

fn shell_quote(path: &str) -> String {
    if path.is_empty() {
        return ".".into();
    }
    if path
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '/' | '.' | '_' | '-'))
    {
        path.to_string()
    } else {
        format!("'{}'", path.replace('\'', "'\\''"))
    }
}

fn gitattributes_quote(path: &str) -> String {
    if path
        .chars()
        .any(|ch| ch.is_whitespace() || matches!(ch, '"' | '\\'))
    {
        format!("\"{}\"", path.replace('\\', "\\\\").replace('"', "\\\""))
    } else {
        path.to_string()
    }
}

fn agents_md(bundle_rel: &str) -> String {
    let quoted = shell_quote(bundle_rel);
    format!(
        "# Agent instructions\n\
         \n\
         - Keep `{bundle_rel}/**/*.md` inert Markdown with OKF YAML.\n\
         - After knowledge edits: `okmate check {quoted} --profile strict`.\n\
         - Durable plans, reports, audits, and status belong in `{bundle_rel}/`.\n\
         - Follow `.cursor/rules/write-knowledge.mdc` and\n\
           `.agents/skills/manage-knowledge/SKILL.md`.\n"
    )
}

fn write_knowledge_mdc(bundle_rel: &str) -> String {
    let quoted = shell_quote(bundle_rel);
    format!(
        "---\n\
         description: >-\n\
           Write reports, plans, research, audits, and other durable project writing as\n\
           OKF records in {bundle_rel}/.\n\
         alwaysApply: false\n\
         ---\n\
         \n\
         # Write to the knowledge database\n\
         \n\
         Default destination is an OKF Markdown record under this repository's\n\
         `{bundle_rel}/`. Do not leave the document in chat, at the repo root, or as a\n\
         loose Markdown file, unless the user names that location.\n\
         \n\
         Follow `.agents/skills/manage-knowledge/SKILL.md` for retrieve, author, and\n\
         validate. Keep `{bundle_rel}/**/*.md` inert Markdown with OKF YAML.\n\
         \n\
         After knowledge edits: `okmate check {quoted} --profile strict`.\n\
         \n\
         ## Collection by intent\n\
         \n\
         | User says | Collection | `type` | Default `authority` |\n\
         | --- | --- | --- | --- |\n\
         | write a plan / implementation plan | `{bundle_rel}/plans/` | Implementation Plan | exploratory |\n\
         | write a report / research | `{bundle_rel}/research/` | Research Report | exploratory |\n\
         | audit / findings vs current behavior | `{bundle_rel}/audits/` | Audit | descriptive |\n\
         | status snapshot / results | `{bundle_rel}/status/` | Status | descriptive |\n\
         \n\
         Architecture and decisions stay at `{bundle_rel}/architecture/` and\n\
         `{bundle_rel}/decisions/`. Do not start implementation plan phases unless the\n\
         user asks.\n"
    )
}

fn manage_skill_md(bundle_rel: &str) -> String {
    let quoted = shell_quote(bundle_rel);
    format!(
        "---\n\
         name: manage-knowledge\n\
         description: Query, inspect, validate, author, or review a local OKF knowledge bundle with okmate.\n\
         ---\n\
         \n\
         # Manage knowledge\n\
         \n\
         Use the checked-in `{bundle_rel}/` bundle as the canonical database and `okmate`\n\
         as its deterministic interface.\n\
         \n\
         Keep `{bundle_rel}/**/*.md` inert Markdown with OKF YAML. Do not add executable\n\
         declarations.\n\
         \n\
         ## Validate\n\
         \n\
         ```sh\n\
         okmate check {quoted} --profile strict\n\
         ```\n\
         \n\
         ## Retrieve\n\
         \n\
         ```sh\n\
         okmate inspect --profile strict catalog {quoted}\n\
         ```\n\
         \n\
         Search authored records with `rg` when the concept ID is unknown. Inspect a\n\
         known concept with `okmate inspect --profile strict concept CONCEPT_ID {quoted}`.\n\
         \n\
         ## Author\n\
         \n\
         Write durable plans, reports, audits, and status as OKF records under\n\
         `{bundle_rel}/`. Follow `.cursor/rules/write-knowledge.mdc` for collection\n\
         routing. Do not start implementation plan phases unless the user asks.\n"
    )
}

fn print_terminal(plan: &InitPlan, applied: bool) {
    let creates = plan
        .files
        .iter()
        .filter(|file| file.op == InitOp::Create)
        .count();
    let verb = if applied { "Created" } else { "Would create" };
    println!(
        "{verb} {creates} files under {} (template: {})",
        plan.root.display(),
        plan.template
    );
    for file in &plan.files {
        let extra = file.dest_root != plan.root;
        let location = if extra {
            file.target.display().to_string()
        } else {
            file.relative.display().to_string()
        };
        match file.op {
            InitOp::Create if applied => println!("  wrote   {location}"),
            InitOp::Create => println!("  create  {location}  ({} bytes)", file.bytes_hint),
            InitOp::Skip if applied => {
                println!("  skipped  {location} (already exists)")
            }
            InitOp::Skip => println!("  skip    {location} (already exists)"),
        }
    }
    if let Some(register) = &plan.register {
        let prefix = if applied {
            "Registered"
        } else {
            "Would register"
        };
        println!(
            "{prefix} directory root `{}` at {} (incoming = {})",
            register.id, register.path, register.incoming
        );
    }
    if let Some(hint) = &plan.gitattributes_hint {
        println!("Add to `.gitattributes`: {hint}");
    }
    if applied {
        let quoted = shell_quote(&plan.root.display().to_string());
        println!("Next: `okmate check {quoted}` then `okmate view {quoted}`");
    } else {
        println!("Dry run; pass --apply to write.");
    }
}

fn create_file(dest_root: &Path, relative: &str, contents: String) -> InitFile {
    let relative = PathBuf::from(relative);
    let target = dest_root.join(&relative);
    InitFile {
        bytes_hint: contents.len(),
        op: InitOp::Create,
        target,
        contents,
        dest_root: dest_root.to_path_buf(),
        relative,
    }
}

fn skip_file(dest_root: &Path, relative: &str) -> InitFile {
    let relative = PathBuf::from(relative);
    let target = dest_root.join(&relative);
    InitFile {
        relative,
        op: InitOp::Skip,
        bytes_hint: 0,
        target,
        contents: String::new(),
        dest_root: dest_root.to_path_buf(),
    }
}

fn root_index(title: &str, collections: &[CollectionSpec]) -> String {
    let mut body = format!("---\nokf_version: \"0.2\"\n---\n\n# {title}\n");
    if !collections.is_empty() {
        body.push('\n');
        for collection in collections {
            body.push_str(&format!(
                "* [{}]({}/) - {}\n",
                collection.heading, collection.path, collection.blurb
            ));
        }
    }
    body
}

fn collection_index(collection: &CollectionSpec) -> String {
    let body = if collection.path == "audits" {
        "Reserved for findings against current behavior."
    } else {
        "No records yet."
    };
    format!("# {}\n\n{body}\n", collection.heading)
}

fn log_markdown(union: UnionLog) -> String {
    let today = utc_date();
    let merge = match union {
        UnionLog::Written | UnionLog::Observed => {
            "\n\
             Git merges this file with the built-in `union` driver (see `.gitattributes`).\n\
             Independent bullets under the same `## YYYY-MM-DD` heading combine instead of\n\
             conflicting. Add a new list item; do not reword another session's bullet in\n\
             the same change.\n"
        }
        UnionLog::None => "\n",
    };
    format!(
        "# Knowledge log\n\
         {merge}\
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
