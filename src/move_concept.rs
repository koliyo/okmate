use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

use anyhow::{Context, Result, bail};
use okf::{
    Bundle, Link, Profile, SourceLocation, resolve_bundle_path, split_fragment, string_field,
};
use serde::Serialize;
use serde_json::Value;

use crate::CheckFormat;

#[derive(Clone, Debug)]
pub struct MoveOptions {
    pub root: PathBuf,
    pub from: String,
    pub to: String,
    pub apply: bool,
    pub format: CheckFormat,
}

#[derive(Clone, Debug, Serialize)]
pub struct MovePlan {
    pub root: String,
    pub from: String,
    pub to: String,
    pub from_path: String,
    pub to_path: String,
    pub apply: bool,
    pub edits: Vec<MoveEdit>,
    pub notes: Vec<String>,
    pub writes: Vec<MoveWrite>,
    pub delete: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct MoveEdit {
    pub path: String,
    pub kind: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct MoveWrite {
    pub path: String,
    pub contents: String,
}

pub fn run(opts: MoveOptions) -> Result<()> {
    let plan = plan_move(&opts)?;
    if opts.apply {
        apply_move(&opts.root, &plan)?;
    }
    match opts.format {
        CheckFormat::Json => println!("{}", serde_json::to_string_pretty(&plan)?),
        CheckFormat::Terminal => print_plan(&plan, opts.apply),
    }
    Ok(())
}

pub fn plan_move(opts: &MoveOptions) -> Result<MovePlan> {
    if !opts.root.is_dir() {
        bail!(
            "knowledge bundle {} is not a directory",
            opts.root.display()
        );
    }
    let bundle = okf::load(&opts.root, Profile::Base)?;
    let from_id = normalize_concept_id(&opts.from)?;
    let to_id = resolve_destination(&from_id, &opts.to)?;
    if from_id == to_id {
        bail!("concept `{from_id}` is already at the destination");
    }
    let from_path = format!("{from_id}.md");
    let to_path = format!("{to_id}.md");
    let Some(concept) = bundle.concepts.iter().find(|concept| concept.id == from_id) else {
        bail!("concept `{from_id}` was not found");
    };
    if bundle.concepts.iter().any(|other| other.id == to_id) || opts.root.join(&to_path).exists() {
        bail!("concept `{to_id}` already exists");
    }
    if bundle
        .indexes
        .iter()
        .any(|index| index.path == format!("{to_id}/index.md"))
    {
        bail!("destination `{to_id}` collides with a collection index");
    }

    let original = read_bundle_files(&opts.root, &bundle)?;
    let mut files = original.clone();
    let mut edits = Vec::new();
    let mut notes = Vec::new();

    let source_index = index_path(&parent_dir(&from_path));
    let dest_index = index_path(&parent_dir(&to_path));
    let same_collection = source_index == dest_index;

    if !same_collection
        && let Some(index) = bundle
            .indexes
            .iter()
            .find(|index| index.path == source_index)
    {
        match remove_membership(&mut files, index, &from_path) {
            MembershipEdit::Removed => edits.push(MoveEdit {
                path: source_index.clone(),
                kind: "index-remove".into(),
            }),
            MembershipEdit::Rewritten => {
                edits.push(MoveEdit {
                    path: source_index.clone(),
                    kind: "href".into(),
                });
                notes.push(format!(
                    "{source_index} listed `{from_id}` outside a list item; href rewritten, remove it by hand if it should not stay"
                ));
            }
            MembershipEdit::Missing => {}
        }
    }

    for other in &bundle.concepts {
        let path = if other.id == from_id {
            from_path.as_str()
        } else {
            other.path.as_str()
        };
        rewrite_links(
            &mut files,
            &mut edits,
            path,
            &other.links,
            &from_path,
            &to_path,
            other.id == from_id,
        )?;
    }
    for index in &bundle.indexes {
        rewrite_links(
            &mut files,
            &mut edits,
            &index.path,
            &index.links,
            &from_path,
            &to_path,
            false,
        )?;
    }

    rewrite_sources(&mut files, &mut edits, concept, &from_path, &to_path)?;

    let moved = files
        .remove(&from_path)
        .ok_or_else(|| anyhow::anyhow!("failed to read {}", from_path))?;
    files.insert(to_path.clone(), moved);

    if !same_collection {
        if files.contains_key(&dest_index) {
            if append_membership(&mut files, &dest_index, concept, &to_path) {
                edits.push(MoveEdit {
                    path: dest_index,
                    kind: "index-append".into(),
                });
            }
        } else {
            notes.push(format!(
                "destination collection index `{dest_index}` is missing; concept will not be listed"
            ));
        }
    }

    if let Some(note) = type_path_note(&opts.root, concept, &to_path) {
        notes.push(note);
    }
    notes.extend(unlinked_mentions(&original, &from_id, &from_path, &edits));

    let mut writes = Vec::new();
    for (path, contents) in &files {
        let previous = if path == &to_path {
            original.get(&from_path)
        } else {
            original.get(path)
        };
        if path == &to_path || previous.map(String::as_str) != Some(contents.as_str()) {
            writes.push(MoveWrite {
                path: path.clone(),
                contents: contents.clone(),
            });
        }
    }
    writes.sort_by(|left, right| left.path.cmp(&right.path));
    edits.sort_by(|left, right| (&left.path, &left.kind).cmp(&(&right.path, &right.kind)));
    edits.dedup_by(|left, right| left.path == right.path && left.kind == right.kind);
    notes.sort();
    notes.dedup();

    Ok(MovePlan {
        root: opts.root.display().to_string(),
        from: from_id,
        to: to_id,
        from_path: from_path.clone(),
        to_path,
        apply: opts.apply,
        edits,
        notes,
        writes,
        delete: Some(from_path),
    })
}

pub fn apply_move(root: &Path, plan: &MovePlan) -> Result<()> {
    if let Some(parent) = root.join(&plan.to_path).parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    for write in &plan.writes {
        let path = root.join(&write.path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }
        fs::write(&path, &write.contents)
            .with_context(|| format!("failed to write {}", path.display()))?;
    }
    if plan.from_path != plan.to_path {
        let source = root.join(plan.delete.as_deref().unwrap_or(&plan.from_path));
        if source.exists() {
            fs::remove_file(&source)
                .with_context(|| format!("failed to remove {}", source.display()))?;
        }
    }
    Ok(())
}

fn read_bundle_files(root: &Path, bundle: &Bundle) -> Result<BTreeMap<String, String>> {
    let mut files = BTreeMap::new();
    let mut paths = BTreeSet::new();
    for concept in &bundle.concepts {
        paths.insert(concept.path.as_str());
    }
    for index in &bundle.indexes {
        paths.insert(index.path.as_str());
    }
    for log in &bundle.logs {
        paths.insert(log.path.as_str());
    }
    for path in paths {
        let source = fs::read_to_string(root.join(path))
            .with_context(|| format!("failed to read {path}"))?;
        files.insert(path.to_string(), source);
    }
    Ok(files)
}

fn rewrite_links(
    files: &mut BTreeMap<String, String>,
    edits: &mut Vec<MoveEdit>,
    file_path: &str,
    links: &[Link],
    from_path: &str,
    to_path: &str,
    is_moved: bool,
) -> Result<()> {
    let Some(source) = files.get(file_path).cloned() else {
        return Ok(());
    };
    let mut next = source.clone();
    for link in links {
        let Some(replacement) = rewrite_link(file_path, &link.url, from_path, to_path, is_moved)
        else {
            continue;
        };
        if replacement == link.url {
            continue;
        }
        let updated = replace_href(&next, &link.location, &link.url, &replacement)
            .or_else(|| replace_href_fallback(&next, &link.url, &replacement));
        let Some(updated) = updated else {
            continue;
        };
        next = updated;
        let kind = if is_moved && !resolves_to(file_path, &link.url, from_path) {
            "outbound"
        } else {
            "href"
        };
        edits.push(MoveEdit {
            path: file_path.to_string(),
            kind: kind.into(),
        });
    }
    files.insert(file_path.to_string(), next);
    Ok(())
}

fn rewrite_link(
    source_path: &str,
    raw: &str,
    from_path: &str,
    to_path: &str,
    is_moved: bool,
) -> Option<String> {
    let (path, fragment) = split_fragment(raw);
    if external_url(path) || path.starts_with('#') {
        return None;
    }
    if resolves_to(source_path, path, from_path) {
        let from_file = if is_moved { to_path } else { source_path };
        return Some(retarget_href(raw, path, to_path, fragment, from_file));
    }
    if is_moved && !path.starts_with('/') {
        let resolved = resolve_bundle_path(source_path, path)?;
        let new_rel = relative_from(to_path, &resolved);
        if new_rel == path {
            return None;
        }
        return Some(with_fragment(&new_rel, fragment));
    }
    None
}

fn retarget_href(
    _raw: &str,
    path: &str,
    to_path: &str,
    fragment: Option<&str>,
    source_path: &str,
) -> String {
    let href = if path.starts_with('/') {
        format!("/{to_path}")
    } else {
        relative_from(source_path, to_path)
    };
    with_fragment(&href, fragment)
}

fn rewrite_sources(
    files: &mut BTreeMap<String, String>,
    edits: &mut Vec<MoveEdit>,
    concept: &okf::Concept,
    from_path: &str,
    to_path: &str,
) -> Result<()> {
    if parent_dir(from_path) == parent_dir(to_path) {
        return Ok(());
    }
    let Some(sources) = concept.metadata.get("sources").and_then(Value::as_array) else {
        return Ok(());
    };
    let Some(source) = files.get(from_path).cloned() else {
        return Ok(());
    };
    let mut next = source;
    let mut changed = false;
    for entry in sources {
        let Some(resource) = entry.get("resource").and_then(Value::as_str) else {
            continue;
        };
        let Some(updated) = rebase_resource(from_path, to_path, resource) else {
            continue;
        };
        if updated == resource {
            continue;
        }
        if let Some(replaced) = replace_resource_value(&next, resource, &updated) {
            next = replaced;
            changed = true;
        }
    }
    if changed {
        files.insert(from_path.to_string(), next);
        edits.push(MoveEdit {
            path: from_path.to_string(),
            kind: "source-resource".into(),
        });
    }
    Ok(())
}

fn append_membership(
    files: &mut BTreeMap<String, String>,
    dest_index: &str,
    concept: &okf::Concept,
    to_path: &str,
) -> bool {
    let Some(source) = files.get_mut(dest_index) else {
        return false;
    };
    let href = relative_from_dir(&parent_dir(dest_index), to_path);
    if source.contains(&format!("]({href})")) || source.contains(&format!("](/{to_path})")) {
        return false;
    }
    let title = string_field(&concept.metadata, "title").unwrap_or(&concept.id);
    let description = string_field(&concept.metadata, "description").unwrap_or("");
    if !source.ends_with('\n') {
        source.push('\n');
    }
    if !source.ends_with("\n\n") {
        source.push('\n');
    }
    if description.is_empty() {
        source.push_str(&format!("* [{title}]({href})\n"));
    } else {
        source.push_str(&format!("* [{title}]({href}) - {description}\n"));
    }
    true
}

enum MembershipEdit {
    Removed,
    Rewritten,
    Missing,
}

fn remove_membership(
    files: &mut BTreeMap<String, String>,
    index: &okf::Index,
    from_path: &str,
) -> MembershipEdit {
    let Some(source) = files.get(&index.path).cloned() else {
        return MembershipEdit::Missing;
    };
    let Some(link) = index
        .links
        .iter()
        .find(|link| resolves_to(&index.path, &link.url, from_path))
    else {
        return MembershipEdit::Missing;
    };
    let line_idx = (link.location.line as usize).saturating_sub(1);
    let Some((start, end, line)) = nth_line(&source, line_idx) else {
        return MembershipEdit::Missing;
    };
    if is_list_item(line) {
        let mut next = String::new();
        next.push_str(&source[..start]);
        next.push_str(&source[end..]);
        files.insert(index.path.clone(), next);
        MembershipEdit::Removed
    } else {
        MembershipEdit::Rewritten
    }
}

fn unlinked_mentions(
    original: &BTreeMap<String, String>,
    from_id: &str,
    from_path: &str,
    edits: &[MoveEdit],
) -> Vec<String> {
    let edited: BTreeSet<&str> = edits
        .iter()
        .filter(|edit| edit.kind == "href" || edit.kind == "index-remove")
        .map(|edit| edit.path.as_str())
        .collect();
    let mut notes = Vec::new();
    for (path, source) in original {
        if path == from_path {
            continue;
        }
        for (idx, line) in source.lines().enumerate() {
            if !line.contains(from_id) && !line.contains(from_path) {
                continue;
            }
            if edited.contains(path.as_str()) && (line.contains("](") || is_list_item(line)) {
                continue;
            }
            notes.push(format!(
                "unlinked mention of `{from_id}` in {path}:{}",
                idx + 1
            ));
        }
    }
    notes
}

fn type_path_note(root: &Path, concept: &okf::Concept, to_path: &str) -> Option<String> {
    let source = fs::read_to_string(root.join("okmate.toml")).ok()?;
    let table = source.parse::<toml::Table>().ok()?;
    let kind = string_field(&concept.metadata, "type")?;
    let expected = table.get("type_paths")?.as_table()?.get(kind)?.as_str()?;
    let expected = expected.trim_matches('/');
    let dir = parent_dir(to_path);
    let matches = if expected.is_empty() {
        dir.is_empty()
    } else {
        dir == expected || dir.starts_with(&format!("{expected}/"))
    };
    if matches {
        None
    } else {
        Some(format!(
            "okmate.toml type_paths expects `{kind}` under `{expected}/`"
        ))
    }
}

fn resolve_destination(from_id: &str, raw: &str) -> Result<String> {
    let trimmed = raw.replace('\\', "/");
    let stem = from_id.rsplit('/').next().unwrap_or(from_id);
    if trimmed == "/" || trimmed == "." || trimmed == "./" {
        return normalize_concept_id(stem);
    }
    if trimmed.ends_with('/') {
        let prefix = trimmed.trim_end_matches('/');
        if prefix.is_empty() || prefix == "." {
            return normalize_concept_id(stem);
        }
        return normalize_concept_id(&format!("{prefix}/{stem}"));
    }
    normalize_concept_id(trimmed.trim_end_matches(".md"))
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

fn parent_dir(path: &str) -> String {
    path.rsplit_once('/')
        .map(|(dir, _)| dir.to_string())
        .unwrap_or_default()
}

fn index_path(dir: &str) -> String {
    if dir.is_empty() {
        "index.md".into()
    } else {
        format!("{dir}/index.md")
    }
}

fn resolves_to(source_path: &str, raw: &str, target_path: &str) -> bool {
    let (path, _) = split_fragment(raw);
    if external_url(path) || path.starts_with('#') {
        return false;
    }
    resolve_bundle_path(source_path, path).is_some_and(|resolved| resolved == target_path)
}

fn relative_from(from_file: &str, to_path: &str) -> String {
    relative_from_dir(&parent_dir(from_file), to_path)
}

fn relative_from_dir(from_dir: &str, to_path: &str) -> String {
    let trailing = to_path.ends_with('/');
    let from = split_parts(from_dir);
    let to = split_parts(to_path.trim_end_matches('/'));
    let mut i = 0;
    while i < from.len() && i < to.len() && from[i] == to[i] {
        i += 1;
    }
    let mut parts = vec!["..".to_string(); from.len().saturating_sub(i)];
    parts.extend(to[i..].iter().cloned());
    let mut href = parts.join("/");
    if href.is_empty() {
        href = to.last().cloned().unwrap_or_else(|| ".".into());
    }
    if trailing && !href.ends_with('/') {
        href.push('/');
    }
    href
}

fn rebase_resource(from_path: &str, to_path: &str, resource: &str) -> Option<String> {
    if external_url(resource) || Path::new(resource).is_absolute() {
        return None;
    }
    let target = join_parts(&parent_dir(from_path), resource);
    Some(parts_to_relative(&parent_dir(to_path), &target))
}

fn join_parts(base_dir: &str, rel: &str) -> Vec<String> {
    let mut parts = split_parts(base_dir);
    for part in split_parts(rel) {
        if part == ".." {
            if parts.last().map(String::as_str) == Some("..") || parts.is_empty() {
                parts.push("..".into());
            } else {
                parts.pop();
            }
        } else {
            parts.push(part);
        }
    }
    parts
}

fn parts_to_relative(from_dir: &str, target: &[String]) -> String {
    let from = split_parts(from_dir);
    let mut i = 0;
    while i < from.len() && i < target.len() && from[i] == target[i] && from[i] != ".." {
        i += 1;
    }
    let mut parts = vec!["..".to_string(); from.len().saturating_sub(i)];
    parts.extend(target[i..].iter().cloned());
    if parts.is_empty() {
        ".".into()
    } else {
        parts.join("/")
    }
}

fn split_parts(path: &str) -> Vec<String> {
    path.replace('\\', "/")
        .split('/')
        .filter(|part| !part.is_empty() && *part != ".")
        .map(ToString::to_string)
        .collect()
}

fn with_fragment(href: &str, fragment: Option<&str>) -> String {
    match fragment {
        Some(fragment) if !fragment.is_empty() => format!("{href}#{fragment}"),
        _ => href.to_string(),
    }
}

fn external_url(url: &str) -> bool {
    url.contains("://") || url.starts_with("mailto:") || url.starts_with("okf:")
}

fn replace_href(source: &str, location: &SourceLocation, old: &str, new: &str) -> Option<String> {
    let start = location.start as usize;
    let end = (location.end as usize).min(source.len());
    if start >= end || end > source.len() {
        return None;
    }
    let slice = &source[start..end];
    let replaced = replace_href_in(slice, old, new)?;
    Some(format!(
        "{}{}{}",
        &source[..start],
        replaced,
        &source[end..]
    ))
}

fn replace_href_fallback(source: &str, old: &str, new: &str) -> Option<String> {
    replace_href_in(source, old, new)
}

fn replace_href_in(slice: &str, old: &str, new: &str) -> Option<String> {
    let wrapped = format!("]({old})");
    if let Some(idx) = slice.rfind(&wrapped) {
        let at = idx + 2;
        return Some(format!(
            "{}{}{}",
            &slice[..at],
            new,
            &slice[at + old.len()..]
        ));
    }
    let titled = format!("]({old} ");
    if let Some(idx) = slice.rfind(&titled) {
        let at = idx + 2;
        return Some(format!(
            "{}{}{}",
            &slice[..at],
            new,
            &slice[at + old.len()..]
        ));
    }
    let idx = slice.rfind(old)?;
    Some(format!(
        "{}{}{}",
        &slice[..idx],
        new,
        &slice[idx + old.len()..]
    ))
}

fn replace_resource_value(source: &str, old: &str, new: &str) -> Option<String> {
    for candidate in [
        format!("resource: {old}"),
        format!("resource: \"{old}\""),
        format!("resource: '{old}'"),
    ] {
        if source.contains(&candidate) {
            let replacement = candidate.replacen(old, new, 1);
            return Some(source.replacen(&candidate, &replacement, 1));
        }
    }
    None
}

fn nth_line(source: &str, line_idx: usize) -> Option<(usize, usize, &str)> {
    let mut start = 0;
    for (idx, line) in source.split_inclusive('\n').enumerate() {
        let end = start + line.len();
        if idx == line_idx {
            return Some((start, end, line.trim_end_matches(['\n', '\r'])));
        }
        start = end;
    }
    None
}

fn is_list_item(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ ")
}

fn print_plan(plan: &MovePlan, applied: bool) {
    let verb = if applied { "Moved" } else { "Would move" };
    println!("{verb} {} → {}", plan.from_path, plan.to_path);
    for edit in &plan.edits {
        println!("  {} ({})", edit.path, edit.kind);
    }
    for note in &plan.notes {
        println!("note: {note}");
    }
    if !applied {
        println!("Dry run; pass --apply to write.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destination_slash_keeps_stem() {
        assert_eq!(
            resolve_destination("research/foo", "audits/").unwrap(),
            "audits/foo"
        );
        assert_eq!(resolve_destination("research/foo", "/").unwrap(), "foo");
    }

    #[test]
    fn relative_from_crosses_directories() {
        assert_eq!(
            relative_from("research/a.md", "audits/b.md"),
            "../audits/b.md"
        );
        assert_eq!(relative_from("a.md", "audits/b.md"), "audits/b.md");
        assert_eq!(relative_from("audits/b.md", "audits/c.md"), "c.md");
    }

    #[test]
    fn rebase_resource_escapes_bundle() {
        assert_eq!(
            rebase_resource(
                "research/okmate/foo.md",
                "audits/foo.md",
                "../../../src/cli.rs"
            )
            .unwrap(),
            "../../src/cli.rs"
        );
    }
}
