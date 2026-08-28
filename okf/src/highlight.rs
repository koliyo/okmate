use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::sync::OnceLock;

use comrak::adapters::SyntaxHighlighterAdapter;
use comrak::html::write_opening_tag;
use syntect::easy::ScopeRegionIterator;
use syntect::parsing::{ParseState, ScopeStack, SyntaxReference, SyntaxSet};
use syntect::util::LinesWithEndings;

pub(crate) struct TokHighlighter {
    syntaxes: SyntaxSet,
}

pub(crate) fn adapter() -> &'static TokHighlighter {
    static HIGHLIGHTER: OnceLock<TokHighlighter> = OnceLock::new();
    HIGHLIGHTER.get_or_init(|| TokHighlighter {
        syntaxes: SyntaxSet::load_defaults_newlines(),
    })
}

impl SyntaxHighlighterAdapter for TokHighlighter {
    fn write_highlighted(
        &self,
        output: &mut dyn fmt::Write,
        lang: Option<&str>,
        code: &str,
    ) -> fmt::Result {
        output.write_str(&highlight_fence(&self.syntaxes, lang, code))
    }

    fn write_pre_tag(
        &self,
        output: &mut dyn fmt::Write,
        mut attributes: HashMap<&'static str, Cow<'_, str>>,
    ) -> fmt::Result {
        merge_class(&mut attributes, "okmate-code-block");
        if let Some(lang) = language_from_class(attributes.get("class").map(|c| c.as_ref())) {
            attributes.insert("data-language", Cow::Owned(lang.to_string()));
        }
        write_opening_tag(output, "pre", attributes)
    }

    fn write_code_tag(
        &self,
        output: &mut dyn fmt::Write,
        mut attributes: HashMap<&'static str, Cow<'_, str>>,
    ) -> fmt::Result {
        if let Some(lang) = language_from_class(attributes.get("class").map(|c| c.as_ref())) {
            attributes.insert("data-language", Cow::Owned(lang.to_string()));
        }
        merge_class(&mut attributes, "okmate-code");
        write_opening_tag(output, "code", attributes)
    }
}

fn merge_class(attributes: &mut HashMap<&'static str, Cow<'_, str>>, extra: &str) {
    match attributes.get("class") {
        Some(existing) if !existing.is_empty() => {
            let merged = format!("{existing} {extra}");
            attributes.insert("class", Cow::Owned(merged));
        }
        _ => {
            attributes.insert("class", Cow::Owned(extra.to_string()));
        }
    }
}

fn language_from_class(class: Option<&str>) -> Option<&str> {
    class?
        .split_whitespace()
        .find_map(|token| token.strip_prefix("language-"))
        .filter(|lang| !lang.is_empty())
}

fn highlight_fence(syntaxes: &SyntaxSet, lang: Option<&str>, code: &str) -> String {
    match syntax_for_lang(syntaxes, lang) {
        Some(syntax) => match highlight_source(syntaxes, syntax, code) {
            Ok(html) => html,
            Err(_) => escape_html(code),
        },
        None => escape_html(code),
    }
}

fn syntax_for_lang<'a>(syntaxes: &'a SyntaxSet, lang: Option<&str>) -> Option<&'a SyntaxReference> {
    let lang = lang?.trim();
    if lang.is_empty() {
        return None;
    }
    let syntax = syntaxes.find_syntax_by_token(lang)?;
    if syntax.name.eq_ignore_ascii_case("Plain Text") {
        return None;
    }
    Some(syntax)
}

fn highlight_source(
    syntaxes: &SyntaxSet,
    syntax: &SyntaxReference,
    code: &str,
) -> Result<String, syntect::Error> {
    let mut parse_state = ParseState::new(syntax);
    let mut stack = ScopeStack::new();
    let mut html = String::new();
    let mut current_class: Option<&'static str> = None;
    let mut current_text = String::new();

    for line in LinesWithEndings::from(code) {
        let ops = parse_state.parse_line(line, syntaxes)?;
        for (segment, op) in ScopeRegionIterator::new(&ops, line) {
            stack.apply(op)?;
            if segment.is_empty() {
                continue;
            }
            let class = class_for_scopes(&stack);
            if class != current_class {
                flush_token(&mut html, current_class, &mut current_text);
                current_class = class;
            }
            current_text.push_str(segment);
        }
    }
    flush_token(&mut html, current_class, &mut current_text);
    Ok(html)
}

fn flush_token(html: &mut String, class: Option<&'static str>, text: &mut String) {
    if text.is_empty() {
        return;
    }
    if let Some(class) = class {
        html.push_str("<span class=\"");
        html.push_str(class);
        html.push_str("\">");
        push_escaped(html, text);
        html.push_str("</span>");
    } else {
        push_escaped(html, text);
    }
    text.clear();
}

fn class_for_scopes(stack: &ScopeStack) -> Option<&'static str> {
    stack
        .as_slice()
        .iter()
        .rev()
        .find_map(|scope| class_for_scope_name(&scope.build_string()))
}

fn class_for_scope_name(name: &str) -> Option<&'static str> {
    if scope_has_prefix(name, "comment") {
        return Some("tok-comment");
    }
    if scope_has_prefix(name, "string") {
        return Some("tok-string");
    }
    if scope_has_prefix(name, "keyword.operator") {
        return Some("tok-operator");
    }
    if scope_has_prefix(name, "keyword") || scope_has_prefix(name, "storage") {
        return Some("tok-keyword");
    }
    if scope_has_prefix(name, "entity.name.function") || scope_has_prefix(name, "support.function")
    {
        return Some("tok-function");
    }
    if scope_has_prefix(name, "entity.name.tag") {
        return Some("tok-tag");
    }
    if scope_has_prefix(name, "entity.name.type")
        || scope_has_prefix(name, "entity.name.class")
        || scope_has_prefix(name, "support.class")
        || scope_has_prefix(name, "support.type")
    {
        return Some("tok-type");
    }
    if scope_has_prefix(name, "entity.other.attribute-name")
        || scope_has_prefix(name, "support.type.property-name")
        || scope_has_prefix(name, "meta.property-name")
    {
        return Some("tok-property");
    }
    if scope_has_prefix(name, "constant.numeric") {
        return Some("tok-number");
    }
    if scope_has_prefix(name, "constant.language") {
        return Some("tok-keyword");
    }
    if scope_has_prefix(name, "constant") {
        return Some("tok-number");
    }
    if scope_has_prefix(name, "variable") {
        return Some("tok-variable");
    }
    if scope_has_prefix(name, "punctuation") {
        return Some("tok-punctuation");
    }
    if scope_has_prefix(name, "entity.name") {
        return Some("tok-function");
    }
    None
}

fn scope_has_prefix(scope: &str, prefix: &str) -> bool {
    scope == prefix
        || scope
            .as_bytes()
            .get(prefix.len())
            .is_some_and(|byte| *byte == b'.')
            && scope.starts_with(prefix)
}

fn escape_html(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    push_escaped(&mut out, value);
    out
}

fn push_escaped(out: &mut String, value: &str) {
    for ch in value.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(ch),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highlights_shell_comments() {
        let html = highlight_fence(&adapter().syntaxes, Some("sh"), "# prefer path\necho hi\n");
        assert!(html.contains("tok-comment"), "{html}");
        assert!(html.contains("prefer path"), "{html}");
    }

    #[test]
    fn unknown_language_escapes_html() {
        let html = highlight_fence(
            &adapter().syntaxes,
            Some("unknown_lang"),
            "<script>alert(1)</script>",
        );
        assert_eq!(html, "&lt;script&gt;alert(1)&lt;/script&gt;");
        assert!(!html.contains("tok-"));
    }

    #[test]
    fn empty_language_escapes_html() {
        let html = highlight_fence(&adapter().syntaxes, Some(""), "a < b");
        assert_eq!(html, "a &lt; b");
    }
}
