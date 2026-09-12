use std::collections::BTreeMap;
use std::fs;
use std::path::{Component, Path};

use okf::{Bundle, Diagnostic, string_field};

const CONVENTIONS_FILE: &str = "okmate.toml";

pub fn apply(bundle: &mut Bundle) {
    let path = bundle.root.join(CONVENTIONS_FILE);
    let Ok(source) = fs::read_to_string(&path) else {
        return;
    };
    let conventions = match parse_conventions(&source) {
        Ok(parsed) => parsed,
        Err(message) => {
            bundle.diagnostics.push(Diagnostic::style_warning(
                "OKMATE5003",
                CONVENTIONS_FILE,
                message,
            ));
            sort_diagnostics(&mut bundle.diagnostics);
            return;
        }
    };
    for key in &conventions.unknown_keys {
        bundle.diagnostics.push(Diagnostic::style_warning(
            "OKMATE5003",
            CONVENTIONS_FILE,
            format!("unknown okmate.toml key `{key}` is ignored"),
        ));
    }
    for warning in conventions.parse_warnings {
        bundle.diagnostics.push(Diagnostic::style_warning(
            "OKMATE5003",
            CONVENTIONS_FILE,
            warning,
        ));
    }
    if let Some(guide) = &conventions.authoring_guide {
        let guide_path = bundle.root.join(guide);
        if !guide_path.is_file() {
            bundle.diagnostics.push(Diagnostic::style_warning(
                "OKMATE5003",
                CONVENTIONS_FILE,
                format!("authoring_guide `{guide}` was not found in the bundle"),
            ));
        }
    }
    let preferred_types = conventions.preferred_types;
    let preferred_tags = conventions.preferred_tags;
    let type_paths = conventions.type_paths;
    let guide_hint = conventions
        .authoring_guide
        .as_deref()
        .map(|guide| format!("; see `{guide}`"))
        .unwrap_or_default();

    for concept in &bundle.concepts {
        let Some(kind) = string_field(&concept.metadata, "type") else {
            continue;
        };
        if !preferred_types.is_empty() && !preferred_types.iter().any(|expected| expected == kind) {
            bundle.diagnostics.push(Diagnostic::style_warning(
                "OKMATE5001",
                concept.path.clone(),
                format!("okmate.toml preferred_types does not include `{kind}`{guide_hint}"),
            ));
        }
        if let Some(expected) = type_paths.get(kind)
            && !path_matches_type(&concept.path, expected)
        {
            bundle.diagnostics.push(Diagnostic::style_warning(
                "OKMATE5002",
                concept.path.clone(),
                format!("okmate.toml type_paths expects `{kind}` under `{expected}/`"),
            ));
        }
        if !preferred_tags.is_empty() {
            for tag in okf::metadata_string_array(&concept.metadata, "tags") {
                if !preferred_tags.iter().any(|expected| expected == &tag) {
                    bundle.diagnostics.push(Diagnostic::style_warning(
                        "OKMATE5003",
                        concept.path.clone(),
                        format!("okmate.toml preferred_tags does not include `{tag}`{guide_hint}"),
                    ));
                }
            }
        }
    }
    sort_diagnostics(&mut bundle.diagnostics);
}

struct Conventions {
    preferred_types: Vec<String>,
    preferred_tags: Vec<String>,
    type_paths: BTreeMap<String, String>,
    authoring_guide: Option<String>,
    unknown_keys: Vec<String>,
    parse_warnings: Vec<String>,
}

fn parse_conventions(source: &str) -> Result<Conventions, String> {
    let value: toml::Value =
        toml::from_str(source).map_err(|error| format!("invalid okmate.toml: {error}"))?;
    let toml::Value::Table(mut table) = value else {
        return Err("okmate.toml must be a table".into());
    };
    let mut parse_warnings = Vec::new();
    match table.remove("version") {
        None => {}
        Some(toml::Value::Integer(1)) => {}
        Some(other) => parse_warnings.push(format!(
            "okmate.toml version `{other}` is ignored; expected integer 1"
        )),
    }
    let preferred_types = take_string_array(&mut table, "preferred_types", &mut parse_warnings);
    let preferred_tags = take_string_array(&mut table, "preferred_tags", &mut parse_warnings);
    let authoring_guide = match table.remove("authoring_guide") {
        None => None,
        Some(toml::Value::String(value)) => match relative_dir(&value) {
            Ok(_) => Some(value),
            Err(message) => {
                parse_warnings.push(format!("authoring_guide {message}"));
                None
            }
        },
        Some(_) => {
            parse_warnings.push("authoring_guide must be a relative path string".into());
            None
        }
    };
    let type_paths = match table.remove("type_paths") {
        None => BTreeMap::new(),
        Some(toml::Value::Table(entries)) => {
            let mut paths = BTreeMap::new();
            for (kind, value) in entries {
                match value {
                    toml::Value::String(path) => match relative_dir(&path) {
                        Ok(normalized) => {
                            paths.insert(kind, normalized);
                        }
                        Err(message) => {
                            parse_warnings.push(format!("type_paths.{kind} {message}"));
                        }
                    },
                    _ => parse_warnings.push(format!(
                        "type_paths.{kind} must be a relative directory string"
                    )),
                }
            }
            paths
        }
        Some(_) => {
            parse_warnings.push("type_paths must be a table of type to directory".into());
            BTreeMap::new()
        }
    };
    let mut unknown_keys: Vec<String> = table.into_iter().map(|(key, _)| key).collect();
    unknown_keys.sort();
    Ok(Conventions {
        preferred_types,
        preferred_tags,
        type_paths,
        authoring_guide,
        unknown_keys,
        parse_warnings,
    })
}

fn take_string_array(
    table: &mut toml::Table,
    key: &str,
    warnings: &mut Vec<String>,
) -> Vec<String> {
    match table.remove(key) {
        None => Vec::new(),
        Some(toml::Value::Array(items)) => {
            let mut values = Vec::new();
            for item in items {
                match item {
                    toml::Value::String(value) => values.push(value),
                    _ => {
                        warnings.push(format!("`{key}` must be a list of strings"));
                        return Vec::new();
                    }
                }
            }
            values
        }
        Some(_) => {
            warnings.push(format!("`{key}` must be a list of strings"));
            Vec::new()
        }
    }
}

fn relative_dir(raw: &str) -> Result<String, String> {
    let normalized = raw.replace('\\', "/");
    let path = Path::new(&normalized);
    if path.is_absolute() {
        return Err(format!("`{raw}` must be relative to the bundle"));
    }
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => {
                let part = part.to_string_lossy();
                if part.is_empty() || part == "." || part == ".." {
                    return Err(format!("`{raw}` must not contain `.` or `..`"));
                }
                parts.push(part.into_owned());
            }
            Component::CurDir | Component::ParentDir => {
                return Err(format!("`{raw}` must not contain `.` or `..`"));
            }
            Component::Prefix(_) | Component::RootDir => {
                return Err(format!("`{raw}` must be relative to the bundle"));
            }
        }
    }
    Ok(parts.join("/"))
}

fn path_matches_type(path: &str, expected: &str) -> bool {
    let expected = expected.trim_matches('/');
    let dir = path.rsplit_once('/').map(|(dir, _)| dir).unwrap_or("");
    if expected.is_empty() {
        return dir.is_empty();
    }
    dir == expected || dir.starts_with(&format!("{expected}/"))
}

fn sort_diagnostics(diagnostics: &mut [Diagnostic]) {
    diagnostics.sort_by(|left, right| {
        (
            &left.path,
            left.location.as_ref().map(|span| span.start),
            left.code,
        )
            .cmp(&(
                &right.path,
                right.location.as_ref().map(|span| span.start),
                right.code,
            ))
    });
}
