use anyhow::{Result, bail};
use sha2::{Digest, Sha256};

pub fn file_hash(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest
        .iter()
        .fold(String::with_capacity(64), |mut out, byte| {
            use std::fmt::Write;
            let _ = write!(out, "{byte:02x}");
            out
        })
}

pub fn append_verification(source: &str, by: &str, at: &str) -> Result<String> {
    if !by.starts_with("human:") {
        bail!("verification actor must start with `human:`");
    }
    let frontmatter = required_frontmatter(source)?;
    let yaml_start = frontmatter.yaml.start as usize;
    let yaml_end = frontmatter.yaml.end as usize;
    let yaml = &source[yaml_start..yaml_end];
    let item = format!("  - {{ by: {by}, at: {at} }}\n");
    let Some(verified) = find_top_level_key(yaml, "verified") else {
        let insert = format!("verified:\n{item}");
        return Ok(splice(source, yaml_end, yaml_end, &insert));
    };
    let value = verified.value.trim();
    if !value.is_empty() && value != "[]" && value != "null" && value != "~" {
        bail!("verified must be a YAML list, not `{value}`");
    }
    let mut insert_at = yaml_start + verified.block_end;
    let mut prefix = String::new();
    if !value.is_empty() {
        let line_end = yaml_start + verified.line_end;
        prefix.push_str(&source[line_end..insert_at]);
        return Ok(splice(
            source,
            yaml_start + verified.colon + 1,
            insert_at,
            &format!("\n{item}{prefix}"),
        ));
    }
    if !yaml[..verified.block_end].ends_with('\n') {
        prefix.push('\n');
        insert_at = yaml_start + verified.block_end;
    }
    Ok(splice(
        source,
        insert_at,
        insert_at,
        &format!("{prefix}{item}"),
    ))
}

pub fn set_status(source: &str, from: &str, to: &str) -> Result<String> {
    let frontmatter = required_frontmatter(source)?;
    let yaml_start = frontmatter.yaml.start as usize;
    let yaml_end = frontmatter.yaml.end as usize;
    let yaml = &source[yaml_start..yaml_end];
    let Some(status) = find_top_level_key(yaml, "status") else {
        bail!("frontmatter has no `status` field");
    };
    let current = unquote(status.value.trim());
    if current != from {
        bail!("status is `{current}`, expected `{from}`");
    }
    let value_start = yaml_start + status.colon + 1;
    let line_end = yaml_start + status.line_end;
    let suffix = if source.as_bytes().get(line_end.saturating_sub(1)) == Some(&b'\n') {
        "\n"
    } else {
        ""
    };
    Ok(splice(
        source,
        value_start,
        line_end,
        &format!(" {to}{suffix}"),
    ))
}

fn required_frontmatter(source: &str) -> Result<okf::Frontmatter> {
    match okf::split_frontmatter(source, true) {
        Ok(Some(frontmatter)) => Ok(frontmatter),
        Ok(None) => bail!("concept requires YAML frontmatter"),
        Err(message) => bail!("{message}"),
    }
}

struct KeySpan {
    colon: usize,
    line_end: usize,
    block_end: usize,
    value: String,
}

fn find_top_level_key(yaml: &str, name: &str) -> Option<KeySpan> {
    let mut offset = 0;
    let mut found: Option<(usize, usize, String)> = None;
    for line in yaml.split_inclusive('\n') {
        let start = offset;
        offset += line.len();
        if found.is_some() {
            if is_top_level_key_line(line) {
                offset = start;
                break;
            }
            continue;
        }
        let trimmed = line.trim_end_matches(['\r', '\n']);
        let Some(rest) = trimmed.strip_prefix(name) else {
            continue;
        };
        let Some(rest) = rest.strip_prefix(':') else {
            continue;
        };
        if trimmed.len() != name.len() + 1 + rest.len() {
            continue;
        }
        found = Some((start + name.len(), offset, rest.to_string()));
    }
    found.map(|(colon, line_end, value)| KeySpan {
        colon,
        line_end,
        block_end: offset,
        value,
    })
}

fn is_top_level_key_line(line: &str) -> bool {
    let trimmed = line.trim_end_matches(['\r', '\n']);
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return false;
    }
    let Some(first) = trimmed.chars().next() else {
        return false;
    };
    if first.is_whitespace() || first == '-' {
        return false;
    }
    trimmed.contains(':')
}

fn splice(source: &str, start: usize, end: usize, insert: &str) -> String {
    let mut out = String::with_capacity(source.len() + insert.len());
    out.push_str(&source[..start]);
    out.push_str(insert);
    out.push_str(&source[end..]);
    out
}

fn unquote(value: &str) -> &str {
    if value.len() >= 2 {
        let bytes = value.as_bytes();
        if (bytes[0] == b'"' && bytes[value.len() - 1] == b'"')
            || (bytes[0] == b'\'' && bytes[value.len() - 1] == b'\'')
        {
            return &value[1..value.len() - 1];
        }
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    const BODY: &str = "# Title\n\nBody text.\n";

    fn concept(extra: &str) -> String {
        format!(
            "---\ntype: Architecture\ntitle: Example\ncustom_key: keep-me\n{extra}---\n\n{BODY}"
        )
    }

    #[test]
    fn file_hash_is_hex_sha256() {
        let digest = file_hash(b"okmate");
        assert_eq!(digest.len(), 64);
        assert!(digest.chars().all(|ch| matches!(ch, '0'..='9' | 'a'..='f')));
        assert_ne!(digest, file_hash(b"okmate "));
        assert_eq!(
            file_hash(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn append_verification_inserts_missing_list() {
        let source = concept("status: draft\n");
        let out = append_verification(&source, "human:nils", "2026-08-28T00:00:00Z").unwrap();
        assert!(out.contains("custom_key: keep-me"));
        assert!(out.contains("verified:\n  - { by: human:nils, at: 2026-08-28T00:00:00Z }\n"));
        assert!(out.ends_with(BODY));
        assert!(out.contains("status: draft"));
    }

    #[test]
    fn append_verification_appends_to_existing_list() {
        let source =
            concept("verified:\n  - { by: human:nils, at: 2026-08-01T00:00:00Z }\nstatus: draft\n");
        let out = append_verification(&source, "human:nils", "2026-08-28T00:00:00Z").unwrap();
        assert!(out.contains(
            "verified:\n  - { by: human:nils, at: 2026-08-01T00:00:00Z }\n  - { by: human:nils, at: 2026-08-28T00:00:00Z }\nstatus: draft\n"
        ));
        assert!(out.contains("custom_key: keep-me"));
        assert!(out.ends_with(BODY));
    }

    #[test]
    fn append_verification_rejects_process_actor() {
        let source = concept("status: draft\n");
        let error = append_verification(&source, "process:cursor", "2026-08-28T00:00:00Z")
            .unwrap_err()
            .to_string();
        assert!(error.contains("human:"), "{error}");
        assert_eq!(source, concept("status: draft\n"));
    }

    #[test]
    fn append_verification_rejects_malformed_frontmatter() {
        let error = append_verification("# No frontmatter\n", "human:nils", "2026-08-28T00:00:00Z")
            .unwrap_err()
            .to_string();
        assert!(error.contains("frontmatter"), "{error}");
        let error = append_verification(
            "---\ntitle: Unclosed\n",
            "human:nils",
            "2026-08-28T00:00:00Z",
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("closing"), "{error}");
    }

    #[test]
    fn set_status_replaces_matching_line() {
        let source = concept("status: draft\nauthority: descriptive\n");
        let out = set_status(&source, "draft", "stable").unwrap();
        assert!(out.contains("status: stable\n"));
        assert!(!out.contains("status: draft"));
        assert!(out.contains("custom_key: keep-me"));
        assert!(out.contains("authority: descriptive"));
        assert!(out.ends_with(BODY));
    }

    #[test]
    fn set_status_refuses_unexpected_value() {
        let source = concept("status: stable\n");
        let error = set_status(&source, "draft", "stable")
            .unwrap_err()
            .to_string();
        assert!(error.contains("stable"), "{error}");
        assert!(error.contains("draft"), "{error}");
    }
}
