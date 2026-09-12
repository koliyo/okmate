use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use okf::{Bundle, Profile, string_field};
use serde::Serialize;

use crate::CheckFormat;

#[derive(Clone, Debug)]
pub struct ConceptOptions {
    pub root: PathBuf,
    pub id: String,
    pub kind: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub authority: Option<String>,
    pub owners: Vec<String>,
    pub tags: Vec<String>,
    pub status: Option<String>,
    pub generated_by: Option<String>,
    pub apply: bool,
    pub profile: Profile,
    pub format: CheckFormat,
}

#[derive(Clone, Debug)]
pub struct IndexOptions {
    pub root: PathBuf,
    pub apply: bool,
    pub format: CheckFormat,
}

#[derive(Clone, Debug, Serialize)]
pub struct ConceptPlan {
    pub root: String,
    pub id: String,
    pub path: String,
    pub apply: bool,
    pub contents: String,
    pub missing: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct IndexPlan {
    pub root: String,
    pub apply: bool,
    pub changes: Vec<IndexChange>,
}

#[derive(Clone, Debug, Serialize)]
pub struct IndexChange {
    pub path: String,
    pub additions: Vec<IndexAddition>,
    pub unresolved: Vec<String>,
    pub preview: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct IndexAddition {
    pub id: String,
    pub title: String,
    pub href: String,
    pub description: String,
}

pub fn run_concept(opts: ConceptOptions) -> Result<()> {
    let plan = plan_concept(&opts)?;
    if !plan.missing.is_empty() {
        bail!(
            "{} profile requires {}",
            opts.profile.as_str(),
            plan.missing.join(", ")
        );
    }
    let dest = opts.root.join(&plan.path);
    if dest.exists() {
        bail!("concept `{}` already exists at {}", plan.id, dest.display());
    }
    if opts.apply {
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }
        fs::write(&dest, &plan.contents)
            .with_context(|| format!("failed to write {}", dest.display()))?;
    }
    match opts.format {
        CheckFormat::Json => println!("{}", serde_json::to_string_pretty(&plan)?),
        CheckFormat::Terminal => print_concept(&plan, opts.apply),
    }
    Ok(())
}

pub fn run_index(opts: IndexOptions) -> Result<()> {
    let plan = plan_index(&opts)?;
    if opts.apply {
        apply_index(&opts.root, &plan)?;
    }
    match opts.format {
        CheckFormat::Json => println!("{}", serde_json::to_string_pretty(&plan)?),
        CheckFormat::Terminal => print_index(&plan, opts.apply),
    }
    Ok(())
}

fn plan_concept(opts: &ConceptOptions) -> Result<ConceptPlan> {
    if !opts.root.is_dir() {
        bail!(
            "knowledge bundle {} is not a directory",
            opts.root.display()
        );
    }
    let id = normalize_concept_id(&opts.id)?;
    let path = format!("{id}.md");
    let title = opts.title.clone();
    let missing = missing_evidence(opts, title.as_deref());
    let heading = title.clone().unwrap_or_else(|| heading_from_id(&id));
    let contents = render_concept(opts, &heading, title.as_deref());
    Ok(ConceptPlan {
        root: opts.root.display().to_string(),
        id,
        path,
        apply: opts.apply,
        contents,
        missing,
    })
}

fn missing_evidence(opts: &ConceptOptions, title: Option<&str>) -> Vec<String> {
    if !opts.profile.requires_evidence() {
        return Vec::new();
    }
    let mut missing = Vec::new();
    if title.is_none_or(str::is_empty) {
        missing.push("--title".into());
    }
    if opts.description.as_deref().is_none_or(str::is_empty) {
        missing.push("--description".into());
    }
    if opts.generated_by.as_deref().is_none_or(str::is_empty) {
        missing.push("--generated-by".into());
    }
    if opts.authority.as_deref().is_none_or(str::is_empty) {
        missing.push("--authority".into());
    }
    if opts.owners.is_empty() {
        missing.push("--owner".into());
    }
    if opts.profile.requires_product_vocabulary() {
        if opts.status.as_deref().is_none_or(str::is_empty) {
            missing.push("--status".into());
        }
        if !opts.tags.iter().any(|tag| tag.starts_with("domain/")) {
            missing.push("--tag domain/…".into());
        }
    }
    missing
}

fn render_concept(opts: &ConceptOptions, heading: &str, title: Option<&str>) -> String {
    let mut yaml = vec![format!("type: {}", opts.kind)];
    if let Some(title) = title.filter(|value| !value.is_empty()) {
        yaml.push(format!("title: {title}"));
    }
    if let Some(description) = opts
        .description
        .as_deref()
        .filter(|value| !value.is_empty())
    {
        yaml.push(format!("description: {description}"));
    }
    if !opts.tags.is_empty() {
        yaml.push(format!("tags: [{}]", opts.tags.join(", ")));
    }
    if let Some(status) = opts.status.as_deref().filter(|value| !value.is_empty()) {
        yaml.push(format!("status: {status}"));
    }
    if let Some(generated_by) = opts
        .generated_by
        .as_deref()
        .filter(|value| !value.is_empty())
    {
        yaml.push(format!(
            "generated: {{ by: {generated_by}, at: {} }}",
            utc_rfc3339()
        ));
    }
    if let Some(authority) = opts.authority.as_deref().filter(|value| !value.is_empty()) {
        yaml.push(format!("authority: {authority}"));
    }
    if !opts.owners.is_empty() {
        yaml.push(format!("owners: [{}]", opts.owners.join(", ")));
    }
    format!(
        "---\n{}\n---\n\n# {heading}\n\n{}\n",
        yaml.join("\n"),
        body_for_type(&opts.kind)
    )
}

fn body_for_type(kind: &str) -> String {
    match kind {
        "Explanation" => {
            "## What this is\n\nReplace this with the model, tradeoffs, and a worked example.\n\n\
             ## When it applies\n\nState the situation this explanation covers.\n\n\
             ## Limits\n\nWhat this does not claim."
                .into()
        }
        "How-to Guide" => "## Prerequisites\n\nWhat must already be true.\n\n\
             ## Steps\n\n1. First action.\n2. Next action.\n\n\
             ## Success\n\nHow you know it worked.\n\n\
             ## Recovery\n\nWhat to do if a step fails."
            .into(),
        "Reference" => "## Scope\n\nWhat this page inventories.\n\n\
             ## Fields\n\nStable names and meanings.\n\n\
             ## Examples\n\nShort, copyable usage."
            .into(),
        "Research Report" => "## Question\n\nWhat this investigation asked.\n\n\
             ## Method\n\nHow evidence was gathered.\n\n\
             ## Findings\n\nWhat the evidence shows, with uncertainty.\n\n\
             ## Follow-up\n\nWhat a maintained guide would still need."
            .into(),
        "Audit" => "## Environment\n\nWhere and when this was observed.\n\n\
             ## Method\n\nHow the observation was made.\n\n\
             ## Findings\n\nWhat was true in that environment."
            .into(),
        "Runbook" => "## Trigger\n\nWhen to use this response.\n\n\
             ## Steps\n\n1. First action.\n2. Next action.\n\n\
             ## Rollback\n\nHow to undo or halt."
            .into(),
        "Dataset" | "Table" | "Metric" => "## Identity\n\nWhat this asset is.\n\n\
             ## Grain\n\nThe unit of one row or observation.\n\n\
             ## Related\n\nOwners and neighboring assets."
            .into(),
        "Workflow" => "## Roles\n\nWho participates.\n\n\
             ## Stages\n\nThe sequence of handoffs.\n\n\
             ## Done\n\nWhat complete looks like."
            .into(),
        _ => "Replace this body. Do not invent owners, citations, or human approval.".into(),
    }
}

fn plan_index(opts: &IndexOptions) -> Result<IndexPlan> {
    let bundle = okf::load(&opts.root, Profile::Base)?;
    let mut by_dir: BTreeMap<String, Vec<&okf::Concept>> = BTreeMap::new();
    for concept in &bundle.concepts {
        let dir = concept_dir(&concept.path);
        by_dir.entry(dir.to_string()).or_default().push(concept);
    }
    let mut changes = Vec::new();
    for index in &bundle.indexes {
        let dir = concept_dir(&index.path);
        let Some(members) = by_dir.get(dir) else {
            let unresolved = unresolved_links(&bundle, index);
            if unresolved.is_empty() {
                continue;
            }
            changes.push(IndexChange {
                path: index.path.clone(),
                additions: Vec::new(),
                unresolved,
                preview: String::new(),
            });
            continue;
        };
        let listed = listed_targets(index);
        let mut additions = Vec::new();
        for concept in members {
            if listed.iter().any(|target| target == &concept.id) {
                continue;
            }
            let href = relative_href(dir, &concept.path);
            additions.push(IndexAddition {
                id: concept.id.clone(),
                title: string_field(&concept.metadata, "title")
                    .unwrap_or(&concept.id)
                    .to_string(),
                href,
                description: string_field(&concept.metadata, "description")
                    .unwrap_or("")
                    .to_string(),
            });
        }
        let unresolved = unresolved_links(&bundle, index);
        if additions.is_empty() && unresolved.is_empty() {
            continue;
        }
        let source = fs::read_to_string(opts.root.join(&index.path))
            .with_context(|| format!("failed to read {}", index.path))?;
        let preview = if additions.is_empty() {
            String::new()
        } else {
            unified_preview(&index.path, &source, &additions)
        };
        changes.push(IndexChange {
            path: index.path.clone(),
            additions,
            unresolved,
            preview,
        });
    }
    Ok(IndexPlan {
        root: opts.root.display().to_string(),
        apply: opts.apply,
        changes,
    })
}

fn apply_index(root: &Path, plan: &IndexPlan) -> Result<()> {
    for change in &plan.changes {
        if change.additions.is_empty() {
            continue;
        }
        let path = root.join(&change.path);
        let mut source = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        if !source.ends_with('\n') {
            source.push('\n');
        }
        if !source.ends_with("\n\n") {
            source.push('\n');
        }
        for addition in &change.additions {
            source.push_str(&addition_line(addition));
        }
        fs::write(&path, source).with_context(|| format!("failed to write {}", path.display()))?;
    }
    Ok(())
}

fn addition_line(addition: &IndexAddition) -> String {
    if addition.description.is_empty() {
        format!("* [{}]({})\n", addition.title, addition.href)
    } else {
        format!(
            "* [{}]({}) - {}\n",
            addition.title, addition.href, addition.description
        )
    }
}

fn unified_preview(path: &str, source: &str, additions: &[IndexAddition]) -> String {
    let mut preview = format!("--- {path}\n+++ {path}\n");
    for addition in additions {
        preview.push_str(&format!("+{}", addition_line(addition)));
    }
    let _ = source;
    preview
}

fn listed_targets(index: &okf::Index) -> BTreeSet<String> {
    let dir = concept_dir(&index.path);
    index
        .links
        .iter()
        .filter_map(|link| resolve_md_id(dir, &link.url))
        .collect()
}

fn unresolved_links(bundle: &Bundle, index: &okf::Index) -> Vec<String> {
    let dir = concept_dir(&index.path);
    let mut missing = Vec::new();
    for link in &index.links {
        let Some(id) = resolve_md_id(dir, &link.url) else {
            continue;
        };
        let exists = bundle.concepts.iter().any(|concept| concept.id == id)
            || bundle
                .indexes
                .iter()
                .any(|other| other.path.trim_end_matches(".md") == id);
        if !exists {
            missing.push(link.url.clone());
        }
    }
    missing.sort();
    missing.dedup();
    missing
}

fn resolve_md_id(index_dir: &str, url: &str) -> Option<String> {
    let href = url.split(['?', '#']).next().unwrap_or(url);
    if href.is_empty() || href.contains("://") || href.starts_with("mailto:") {
        return None;
    }
    let trimmed = href.trim_start_matches('/');
    if !trimmed.ends_with(".md")
        && !trimmed.ends_with('/')
        && Path::new(trimmed).extension().is_some()
    {
        return None;
    }
    let relative = if trimmed.ends_with('/') {
        format!("{trimmed}index.md")
    } else if trimmed.ends_with(".md") {
        trimmed.to_string()
    } else {
        return None;
    };
    let joined = if relative.starts_with('/') || index_dir.is_empty() {
        relative.trim_start_matches('/').to_string()
    } else {
        format!("{index_dir}/{relative}")
    };
    let mut parts = Vec::new();
    for component in joined.split('/') {
        match component {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            other => parts.push(other),
        }
    }
    let path = parts.join("/");
    Some(path.trim_end_matches(".md").to_string())
}

fn relative_href(index_dir: &str, concept_path: &str) -> String {
    if index_dir.is_empty() {
        return concept_path.to_string();
    }
    let prefix = format!("{index_dir}/");
    concept_path
        .strip_prefix(&prefix)
        .unwrap_or(concept_path)
        .to_string()
}

fn concept_dir(path: &str) -> &str {
    path.rsplit_once('/').map(|(dir, _)| dir).unwrap_or("")
}

fn normalize_concept_id(raw: &str) -> Result<String> {
    let normalized = raw.replace('\\', "/").trim_end_matches(".md").to_string();
    if normalized.is_empty() {
        bail!("concept id must not be empty");
    }
    let path = Path::new(&normalized);
    if path.is_absolute() {
        bail!("concept id `{raw}` must be relative to the bundle");
    }
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => {
                let part = part.to_string_lossy();
                if part.is_empty() || part == "." || part == ".." || part == "index" {
                    bail!("concept id `{raw}` must not use `.`, `..`, or `index`");
                }
                parts.push(part.into_owned());
            }
            Component::CurDir | Component::ParentDir => {
                bail!("concept id `{raw}` must not use `.` or `..`");
            }
            Component::Prefix(_) | Component::RootDir => {
                bail!("concept id `{raw}` must be relative to the bundle");
            }
        }
    }
    Ok(parts.join("/"))
}

fn heading_from_id(id: &str) -> String {
    let stem = id.rsplit('/').next().unwrap_or(id);
    let mut words = Vec::new();
    for word in stem.split('-') {
        if word.is_empty() {
            continue;
        }
        let mut chars = word.chars();
        let Some(first) = chars.next() else {
            continue;
        };
        words.push(format!("{}{}", first.to_uppercase(), chars.as_str()));
    }
    if words.is_empty() {
        stem.to_string()
    } else {
        words.join(" ")
    }
}

fn utc_rfc3339() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);
    let date = utc_date();
    let tod = seconds % 86_400;
    format!(
        "{date}T{:02}:{:02}:{:02}Z",
        tod / 3_600,
        (tod % 3_600) / 60,
        tod % 60
    )
}

fn utc_date() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
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

fn print_concept(plan: &ConceptPlan, applied: bool) {
    if applied {
        println!("Wrote {} (`{}`)", plan.path, plan.id);
    } else {
        println!("Would create {} (`{}`)", plan.path, plan.id);
        println!("{}", plan.contents);
        println!("Dry run; pass --apply to write.");
    }
}

fn print_index(plan: &IndexPlan, applied: bool) {
    if plan.changes.is_empty() {
        println!("No index updates.");
        return;
    }
    for change in &plan.changes {
        if !change.additions.is_empty() {
            let verb = if applied { "Updated" } else { "Would update" };
            println!(
                "{verb} {} ({} addition{})",
                change.path,
                change.additions.len(),
                if change.additions.len() == 1 { "" } else { "s" }
            );
            if !applied && !change.preview.is_empty() {
                print!("{}", change.preview);
            }
        }
        for missing in &change.unresolved {
            println!("unresolved {} -> {missing}", change.path);
        }
    }
    if !applied {
        println!("Dry run; pass --apply to write.");
    }
}
