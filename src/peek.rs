use okf::{Concept, Index};
use serde::Serialize;

use crate::html_util::{first_prose_paragraph, plaintext};
use crate::nav::collection_title;
use crate::workspace::{Workspace, WorkspaceMember, normalize_route};

const EXCERPT_LIMIT: usize = 280;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PeekKind {
    Document,
    Heading,
    Footnote,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Peek {
    pub kind: PeekKind,
    #[serde(rename = "type")]
    pub concept_type: String,
    pub document_title: String,
    pub title: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
    pub excerpt: String,
}

pub fn peek(workspace: &Workspace, path: &str, hash: &str) -> Option<Peek> {
    let (path, hash_from_path) = split_hash(path);
    let hash = if hash.is_empty() {
        hash_from_path
    } else {
        hash
    };
    let hash = hash.trim_start_matches('#');
    let route = normalize_route(path);
    if Workspace::chrome_route(&route) {
        return Some(chrome_peek(workspace, &route));
    }
    let (member, id) = workspace.parse_document_route(&route)?;
    if let Some(concept) = member
        .bundle
        .concepts
        .iter()
        .find(|concept| concept.id == id)
    {
        return Some(peek_concept(workspace, member, concept, hash));
    }
    let index = member
        .bundle
        .indexes
        .iter()
        .find(|index| index.path.strip_suffix("/index.md") == Some(id.as_str()))?;
    Some(peek_index(index, hash))
}

fn peek_concept(
    workspace: &Workspace,
    member: &WorkspaceMember,
    concept: &Concept,
    hash: &str,
) -> Peek {
    let article = workspace.rewrite_article(&member.id, &concept.article_html);
    let (concept_type, document_title, description) = concept_fields(concept);
    let lead = clip(&first_prose_paragraph(&article));
    if let Some(label) = footnote_label(hash) {
        if let Some(peek) = footnote_peek(
            workspace,
            member,
            concept,
            &article,
            label,
            &concept_type,
            &document_title,
            &description,
        ) {
            return peek;
        }
        return document_peek(concept_type, document_title, description, lead);
    }
    if !hash.is_empty()
        && let Some((heading_text, excerpt)) = heading_excerpt(concept, &article, hash)
    {
        return Peek {
            kind: PeekKind::Heading,
            concept_type,
            document_title,
            title: heading_text,
            description,
            excerpt,
        };
    }
    document_peek(concept_type, document_title, description, lead)
}

fn peek_index(index: &Index, hash: &str) -> Peek {
    let document_title = collection_title(index);
    let lead = clip(&first_prose_paragraph(&index.article_html));
    if !hash.is_empty()
        && let Some((heading_text, excerpt)) = index_heading_excerpt(index, hash)
    {
        return Peek {
            kind: PeekKind::Heading,
            concept_type: String::new(),
            document_title: document_title.clone(),
            title: heading_text,
            description: String::new(),
            excerpt,
        };
    }
    document_peek(String::new(), document_title, String::new(), lead)
}

fn chrome_peek(workspace: &Workspace, route: &str) -> Peek {
    let title = match route {
        "/review/" => "Knowledge Governance & Review Queue",
        "/log/" => "Log",
        "/settings/" => "Settings",
        _ => "Knowledge",
    };
    let excerpt = workspace
        .primary()
        .and_then(|member| {
            member
                .bundle
                .indexes
                .iter()
                .find(|index| index.path == "index.md")
        })
        .map(|index| clip(&first_prose_paragraph(&index.article_html)))
        .unwrap_or_default();
    document_peek(String::new(), title.to_string(), String::new(), excerpt)
}

fn document_peek(
    concept_type: String,
    document_title: String,
    description: String,
    excerpt: String,
) -> Peek {
    Peek {
        kind: PeekKind::Document,
        title: document_title.clone(),
        concept_type,
        document_title,
        description,
        excerpt,
    }
}

fn concept_fields(concept: &Concept) -> (String, String, String) {
    let concept_type = okf::string_field(&concept.metadata, "type")
        .unwrap_or("Concept")
        .to_string();
    let document_title = okf::string_field(&concept.metadata, "title")
        .unwrap_or(&concept.id)
        .to_string();
    let description = okf::string_field(&concept.metadata, "description")
        .unwrap_or("")
        .to_string();
    (concept_type, document_title, description)
}

fn footnote_peek(
    workspace: &Workspace,
    member: &WorkspaceMember,
    concept: &Concept,
    article: &str,
    label: &str,
    concept_type: &str,
    document_title: &str,
    description: &str,
) -> Option<Peek> {
    let definition_html = footnote_definition_html(article, label)?;
    let mut excerpt_parts = Vec::new();
    let definition = plaintext_without_backrefs(definition_html);
    if !definition.is_empty() {
        excerpt_parts.push(definition);
    }
    let source = source_entry(concept, label);
    let title = source
        .and_then(|source| source.get("title"))
        .and_then(|value| value.as_str())
        .filter(|title| !title.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| format!("[^{label}]"));
    if let Some(author) = source
        .and_then(|source| source.get("author"))
        .and_then(|value| value.as_str())
        .filter(|author| !author.is_empty())
    {
        excerpt_parts.push(author.to_string());
    }
    if let Some(resource) = source
        .and_then(|source| source.get("resource"))
        .and_then(|value| value.as_str())
        .filter(|resource| !resource.is_empty())
    {
        excerpt_parts.push(resource.to_string());
    }
    if let Some((path, frag)) = first_hashed_in_bundle_href(definition_html)
        && let Some((_, linked_excerpt)) =
            linked_heading_excerpt(workspace, member, concept, path, frag)
        && !linked_excerpt.is_empty()
    {
        excerpt_parts.push(linked_excerpt);
    }
    Some(Peek {
        kind: PeekKind::Footnote,
        concept_type: concept_type.to_string(),
        document_title: document_title.to_string(),
        title,
        description: description.to_string(),
        excerpt: clip(&excerpt_parts.join(" ")),
    })
}

fn heading_excerpt(concept: &Concept, article: &str, id: &str) -> Option<(String, String)> {
    if let Some(section) = concept
        .heading_sections
        .iter()
        .find(|section| section.id == id)
    {
        return Some((
            section.heading_text.clone(),
            clip(&section.body_texts.join(" ")),
        ));
    }
    html_heading_excerpt(article, id)
}

fn index_heading_excerpt(index: &Index, id: &str) -> Option<(String, String)> {
    if let Some((heading_text, excerpt)) = html_heading_excerpt(&index.article_html, id) {
        return Some((heading_text, excerpt));
    }
    index
        .headings
        .iter()
        .find(|heading| heading.id == id)
        .map(|heading| (heading.text.clone(), String::new()))
}

fn linked_heading_excerpt(
    workspace: &Workspace,
    member: &WorkspaceMember,
    current: &Concept,
    path: &str,
    hash: &str,
) -> Option<(String, String)> {
    let hash = hash.trim_start_matches('#');
    if hash.is_empty() || footnote_label(hash).is_some() {
        return None;
    }
    if path.is_empty() {
        return heading_excerpt(
            current,
            &workspace.rewrite_article(&member.id, &current.article_html),
            hash,
        );
    }
    let route = normalize_route(path);
    let (target_member, id) = workspace.parse_document_route(&route)?;
    let concept = target_member
        .bundle
        .concepts
        .iter()
        .find(|concept| concept.id == id)?;
    heading_excerpt(
        concept,
        &workspace.rewrite_article(&target_member.id, &concept.article_html),
        hash,
    )
}

fn footnote_label(hash: &str) -> Option<&str> {
    let hash = hash.trim_start_matches('#');
    if let Some(label) = hash.strip_prefix("fn-") {
        return (!label.is_empty()).then_some(label);
    }
    let rest = hash.strip_prefix("fnref-")?;
    let label = strip_numbered_fnref(rest);
    (!label.is_empty()).then_some(label)
}

fn strip_numbered_fnref(rest: &str) -> &str {
    if let Some((label, suffix)) = rest.rsplit_once('-')
        && !suffix.is_empty()
        && suffix.chars().all(|ch| ch.is_ascii_digit())
    {
        return label;
    }
    rest
}

fn footnote_definition_html<'a>(article_html: &'a str, label: &str) -> Option<&'a str> {
    let marker = format!("id=\"fn-{label}\"");
    let at = article_html.find(&marker)?;
    let li = article_html[..at].rfind("<li")?;
    let after = &article_html[li..];
    let end = after.find("</li>")?;
    Some(&after[..end + 5])
}

fn plaintext_without_backrefs(html: &str) -> String {
    let mut cleaned = html.to_string();
    while let Some(start) = cleaned.find("<a ") {
        let rest = &cleaned[start..];
        let is_backref = rest.contains("footnote-backref")
            || rest
                .get(..80)
                .is_some_and(|head| head.contains("href=\"#fnref-"));
        let Some(end) = rest.find("</a>") else { break };
        if is_backref {
            cleaned.replace_range(start..start + end + 4, " ");
        } else if let Some(tag_end) = rest.find('>') {
            cleaned.replace_range(start..start + tag_end + 1, " ");
            if let Some(close) = cleaned[start..].find("</a>") {
                cleaned.replace_range(start + close..start + close + 4, " ");
            }
        } else {
            break;
        }
    }
    plaintext(&cleaned)
}

fn first_hashed_in_bundle_href(html: &str) -> Option<(&str, &str)> {
    let mut rest = html;
    loop {
        let Some(at) = rest.find("href=\"") else {
            return None;
        };
        rest = &rest[at + 6..];
        let Some(end) = rest.find('"') else {
            return None;
        };
        let href = &rest[..end];
        rest = &rest[end + 1..];
        if href.starts_with("http:")
            || href.starts_with("https:")
            || href.starts_with("mailto:")
            || href.starts_with("/__okmate")
        {
            continue;
        }
        if let Some(frag) = href.strip_prefix('#') {
            if frag.starts_with("fn-") || frag.starts_with("fnref-") || frag.is_empty() {
                continue;
            }
            return Some(("", frag));
        }
        let Some((path, frag)) = href.split_once('#') else {
            continue;
        };
        if frag.is_empty() || frag.starts_with("fn-") || frag.starts_with("fnref-") {
            continue;
        }
        return Some((path, frag));
    }
}

fn html_heading_excerpt(html: &str, id: &str) -> Option<(String, String)> {
    let marker = format!("id=\"{id}\"");
    let at = html.find(&marker)?;
    let heading_start = html[..at].rfind("<h")?;
    let after_heading = &html[heading_start..];
    let close_at = after_heading.find("</h")?;
    let close_end = after_heading[close_at..].find('>')?;
    let heading_html = &after_heading[..close_at];
    let Some(inner_start) = heading_html.find('>') else {
        return None;
    };
    let heading_text = plaintext(&heading_html[inner_start + 1..]);
    let mut rest = &after_heading[close_at + close_end + 1..];
    let mut body = String::new();
    loop {
        rest = rest.trim_start();
        if rest.is_empty() || rest.starts_with("<h") || rest.starts_with("<section") {
            break;
        }
        if let Some(after) = rest.strip_prefix("<p") {
            let Some(gt) = after.find('>') else { break };
            let inner = &after[gt + 1..];
            let Some(end) = inner.find("</p>") else { break };
            let text = plaintext(&inner[..end]);
            if !text.is_empty() {
                if !body.is_empty() {
                    body.push(' ');
                }
                body.push_str(&text);
            }
            rest = &inner[end + 4..];
            continue;
        }
        if let Some(end) = rest.find('<') {
            if end == 0 {
                if let Some(tag_end) = rest.find('>') {
                    rest = &rest[tag_end + 1..];
                    continue;
                }
            }
            break;
        }
        break;
    }
    Some((heading_text, clip(&body)))
}

fn source_entry<'a>(
    concept: &'a Concept,
    id: &str,
) -> Option<&'a serde_json::Map<String, serde_json::Value>> {
    concept
        .metadata
        .get("sources")?
        .as_array()?
        .iter()
        .filter_map(|value| value.as_object())
        .find(|source| source.get("id").and_then(|value| value.as_str()) == Some(id))
}

fn split_hash(path: &str) -> (&str, &str) {
    match path.split_once('#') {
        Some((path, hash)) => (path, hash),
        None => (path, ""),
    }
}

fn clip(text: &str) -> String {
    let mut out = String::new();
    for ch in text.chars() {
        if out.chars().count() >= EXCERPT_LIMIT {
            break;
        }
        out.push(ch);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn footnote_label_reads_comrak_ids() {
        assert_eq!(footnote_label("fn-s21"), Some("s21"));
        assert_eq!(footnote_label("#fnref-s21"), Some("s21"));
        assert_eq!(footnote_label("fnref-s21-2"), Some("s21"));
        assert_eq!(footnote_label("s21"), None);
        assert_eq!(footnote_label("details"), None);
    }

    #[test]
    fn clip_stops_at_character_limit() {
        assert_eq!(clip(&"a".repeat(300)).chars().count(), EXCERPT_LIMIT);
    }
}
