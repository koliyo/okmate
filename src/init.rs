use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use okf::Profile;
use serde::Serialize;

use crate::CheckFormat;

#[derive(Clone, Debug)]
pub struct InitOptions {
    pub path: PathBuf,
    pub title: String,
    pub bare: bool,
    pub apply: bool,
    pub format: CheckFormat,
}

#[derive(Clone, Debug, Serialize)]
pub struct InitPlan {
    pub root: PathBuf,
    pub files: Vec<InitFile>,
}

#[derive(Clone, Debug, Serialize)]
pub struct InitFile {
    pub relative: PathBuf,
    pub op: InitOp,
    pub bytes_hint: usize,
    #[serde(skip)]
    pub contents: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InitOp {
    Create,
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
        let dest = plan.root.join(&file.relative);
        if dest.exists() {
            bail!("{} already exists; refusing to overwrite", dest.display());
        }
    }
    for file in &plan.files {
        if file.op != InitOp::Create {
            continue;
        }
        let dest = plan.root.join(&file.relative);
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

    let mut files = Vec::new();
    files.push(create_file("index.md", root_index(&opts.title, opts.bare)));
    files.push(create_file("log.md", log_markdown()));
    if !opts.bare {
        for (dir, heading, _) in COLLECTIONS {
            files.push(create_file(
                &format!("{dir}/index.md"),
                collection_index(heading, dir),
            ));
        }
    }

    Ok(InitPlan { root, files })
}

fn print_terminal(plan: &InitPlan, applied: bool) {
    if applied {
        println!(
            "Created {} files under {}",
            plan.files.len(),
            plan.root.display()
        );
        for file in &plan.files {
            println!("  wrote  {}", file.relative.display());
        }
        println!(
            "Next: `okmate check {}` then `okmate view {}`",
            plan.root.display(),
            plan.root.display()
        );
        return;
    }
    println!(
        "Would create {} files under {}",
        plan.files.len(),
        plan.root.display()
    );
    for file in &plan.files {
        println!(
            "  create  {}  ({} bytes)",
            file.relative.display(),
            file.bytes_hint
        );
    }
    println!("Dry run; pass --apply to write.");
}

fn create_file(relative: &str, contents: String) -> InitFile {
    InitFile {
        relative: PathBuf::from(relative),
        op: InitOp::Create,
        bytes_hint: contents.len(),
        contents,
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
