use std::collections::BTreeMap;

use crate::html_util::first_prose_paragraph;
use crate::session::NavMode;
use crate::views::{NavNode, review_needs_attention, type_color};
use crate::workspace::{Workspace, WorkspaceMember, normalize_route};

pub fn nav_tree(workspace: &Workspace, current: &str, nav_mode: NavMode) -> Vec<NavNode> {
    let current = normalize_route(current);
    let mut review = leaf("/review/", "Review queue", &current);
    review.attention = review_needs_attention(workspace);
    let mut items = vec![
        leaf("/", "Dashboard", &current),
        review,
        leaf("/log/", "Log", &current),
        leaf("/settings/", "Settings", &current),
    ];
    if workspace.is_multi() {
        match nav_mode {
            NavMode::Merged => items.extend(nav_forest(workspace, &current, ForestKind::Merged)),
            NavMode::Separated => {
                for member in workspace.members() {
                    let prefix = format!("/@{}/", member.id);
                    let active = current.starts_with(&prefix);
                    items.push(NavNode {
                        href: String::new(),
                        title: format!("@{}", member.id),
                        current: active,
                        open: active,
                        children: nav_forest(
                            workspace,
                            &current,
                            ForestKind::Separated {
                                member,
                                section_prefix: member.id.as_str(),
                            },
                        ),
                        section_key: member.id.clone(),
                        root: member.id.clone(),
                        summary: String::new(),
                        attention: false,
                        type_color: String::new(),
                        collection: String::new(),
                    });
                }
            }
        }
    } else if let Some(member) = workspace.primary() {
        items.extend(nav_forest(
            workspace,
            &current,
            ForestKind::Separated {
                member,
                section_prefix: "",
            },
        ));
    }
    items
}

enum ForestKind<'a> {
    Separated {
        member: &'a WorkspaceMember,
        section_prefix: &'a str,
    },
    Merged,
}

fn nav_forest(workspace: &Workspace, current: &str, kind: ForestKind<'_>) -> Vec<NavNode> {
    let mut by_path: BTreeMap<String, NavNode> = BTreeMap::new();
    let mut owners: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let members: Vec<&WorkspaceMember> = match kind {
        ForestKind::Separated { member, .. } => vec![member],
        ForestKind::Merged => workspace.members().iter().collect(),
    };
    let merged = matches!(kind, ForestKind::Merged);

    for member in &members {
        for index in &member.bundle.indexes {
            let Some(path) = index.path.strip_suffix("/index.md") else {
                continue;
            };
            owners
                .entry(path.to_string())
                .or_default()
                .push(member.id.clone());
            by_path.entry(path.to_string()).or_insert_with(|| {
                let href = if merged {
                    String::new()
                } else {
                    workspace.collection_href(&member.id, path)
                };
                let active = if merged {
                    collection_is_current(workspace, path, current)
                } else {
                    current == href || current.starts_with(&href)
                };
                NavNode {
                    href: href.clone(),
                    title: collection_title(index),
                    current: active,
                    open: active,
                    children: Vec::new(),
                    section_key: match kind {
                        ForestKind::Separated { section_prefix, .. } => {
                            namespaced_key(section_prefix, path)
                        }
                        ForestKind::Merged => path.to_string(),
                    },
                    root: String::new(),
                    summary: if merged {
                        String::new()
                    } else {
                        collection_summary(index)
                    },
                    attention: false,
                    type_color: String::new(),
                    collection: String::new(),
                }
            });
        }
    }

    let paths: Vec<String> = by_path.keys().cloned().collect();
    for member in &members {
        for concept in &member.bundle.concepts {
            if by_path.contains_key(&concept.id) {
                continue;
            }
            let Some(owner) = owning_collection(&paths, &concept.id) else {
                continue;
            };
            let title = okf::string_field(&concept.metadata, "title").unwrap_or(&concept.id);
            if let Some(node) = by_path.get_mut(&owner) {
                let href = workspace.document_href(&member.id, &concept.id);
                let mut item = colored_leaf(&href, title, current, concept_type_color(concept));
                if merged {
                    item.root = member.id.clone();
                }
                node.children.push(item);
            }
        }
    }
    for node in by_path.values_mut() {
        node.children.sort_by(|left, right| {
            left.title
                .cmp(&right.title)
                .then(left.href.cmp(&right.href))
        });
    }

    let (children_of, mut roots) = parent_links(&paths);
    let section_prefix = match kind {
        ForestKind::Separated { section_prefix, .. } => section_prefix,
        ForestKind::Merged => "",
    };

    fn take_node(
        path: &str,
        current: &str,
        workspace: &Workspace,
        merged: bool,
        section_prefix: &str,
        by_path: &mut BTreeMap<String, NavNode>,
        children_of: &BTreeMap<String, Vec<String>>,
        owners: &BTreeMap<String, Vec<String>>,
    ) -> NavNode {
        let mut node = by_path.remove(path).expect("nav node");
        if let Some(child_paths) = children_of.get(path) {
            for child in child_paths {
                node.children.push(take_node(
                    child,
                    current,
                    workspace,
                    merged,
                    section_prefix,
                    by_path,
                    children_of,
                    owners,
                ));
            }
        }
        if merged {
            let empty = Vec::new();
            finalize_merged_collection(
                workspace,
                path,
                owners.get(path).unwrap_or(&empty),
                node,
                current,
            )
        } else {
            let root_id = owners
                .get(path)
                .and_then(|ids| ids.first())
                .map(String::as_str)
                .unwrap_or("");
            finalize_collection(workspace, root_id, section_prefix, path, node, current)
        }
    }

    roots.sort();
    roots
        .into_iter()
        .map(|path| {
            take_node(
                &path,
                current,
                workspace,
                merged,
                section_prefix,
                &mut by_path,
                &children_of,
                &owners,
            )
        })
        .collect()
}

fn owning_collection(paths: &[String], concept_id: &str) -> Option<String> {
    paths
        .iter()
        .filter(|name| concept_id == name.as_str() || concept_id.starts_with(&format!("{name}/")))
        .max_by_key(|name| name.len())
        .cloned()
}

fn parent_links(paths: &[String]) -> (BTreeMap<String, Vec<String>>, Vec<String>) {
    let mut children_of: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut roots = Vec::new();
    for path in paths {
        let parent = paths
            .iter()
            .filter(|candidate| path.starts_with(&format!("{candidate}/")))
            .max_by_key(|candidate| candidate.len());
        if let Some(parent) = parent {
            children_of
                .entry(parent.clone())
                .or_default()
                .push(path.clone());
        } else {
            roots.push(path.clone());
        }
    }
    (children_of, roots)
}

fn leaf(href: &str, title: &str, current: &str) -> NavNode {
    colored_leaf(href, title, current, String::new())
}

fn colored_leaf(href: &str, title: &str, current: &str, type_color: String) -> NavNode {
    NavNode {
        href: href.into(),
        title: title.into(),
        current: href == current,
        open: false,
        children: Vec::new(),
        section_key: String::new(),
        root: String::new(),
        summary: String::new(),
        attention: false,
        type_color,
        collection: String::new(),
    }
}

fn concept_type_color(concept: &okf::Concept) -> String {
    type_color(okf::string_field(&concept.metadata, "type").unwrap_or("Concept"))
}

fn namespaced_key(prefix: &str, path: &str) -> String {
    if prefix.is_empty() {
        path.to_string()
    } else {
        format!("{prefix}/{path}")
    }
}

fn collection_summary(index: &okf::Index) -> String {
    first_prose_paragraph(&index.article_html)
}

pub fn collection_title(index: &okf::Index) -> String {
    if let Some(heading) = index.headings.iter().find(|heading| heading.level == 1) {
        return heading.text.clone();
    }
    index
        .path
        .strip_suffix("/index.md")
        .and_then(|collection| collection.rsplit('/').next())
        .unwrap_or(index.path.as_str())
        .to_string()
}

pub fn collection_owners(workspace: &Workspace, path: &str) -> Vec<String> {
    workspace
        .members()
        .iter()
        .filter(|member| {
            member
                .bundle
                .indexes
                .iter()
                .any(|index| index.path.strip_suffix("/index.md") == Some(path))
        })
        .map(|member| member.id.clone())
        .collect()
}

pub fn canonical_collection_href(workspace: &Workspace, path: &str) -> String {
    workspace
        .first_collection_owner(path)
        .map(|id| workspace.collection_href(id, path))
        .unwrap_or_else(|| format!("/{path}/"))
}

fn collection_is_current(workspace: &Workspace, path: &str, current: &str) -> bool {
    workspace.members().iter().any(|member| {
        let href = workspace.collection_href(&member.id, path);
        current == href || current.starts_with(&href)
    })
}

fn collection_page_is_current(workspace: &Workspace, path: &str, current: &str) -> bool {
    workspace
        .members()
        .iter()
        .any(|member| workspace.collection_href(&member.id, path) == current)
}

fn finalize_collection(
    workspace: &Workspace,
    root_id: &str,
    section_prefix: &str,
    path: &str,
    mut node: NavNode,
    current: &str,
) -> NavNode {
    let href = workspace.collection_href(root_id, path);
    let mut nested = Vec::new();
    let mut leaves = Vec::new();
    for child in node.children.drain(..) {
        if child.section_key.is_empty() {
            leaves.push(child);
        } else {
            nested.push(child);
        }
    }
    nested.sort_by(|left, right| {
        left.title
            .cmp(&right.title)
            .then(left.href.cmp(&right.href))
    });
    leaves.sort_by(|left, right| {
        left.title
            .cmp(&right.title)
            .then(left.href.cmp(&right.href))
    });
    let mut overview = leaf(&href, "Overview", current);
    overview.collection = path.to_string();
    let mut children = vec![overview];
    children.extend(nested);
    children.extend(leaves);
    node.children = children;
    node.section_key = namespaced_key(section_prefix, path);
    node.href = href.clone();
    node.current = current == href || current.starts_with(&href);
    node.open = node.current;
    node
}

fn finalize_merged_collection(
    workspace: &Workspace,
    path: &str,
    owners: &[String],
    mut node: NavNode,
    current: &str,
) -> NavNode {
    let mut nested = Vec::new();
    let mut leaves = Vec::new();
    for child in node.children.drain(..) {
        if child.section_key.is_empty() {
            leaves.push(child);
        } else {
            nested.push(child);
        }
    }
    nested.sort_by(|left, right| {
        left.title
            .cmp(&right.title)
            .then(left.href.cmp(&right.href))
    });
    leaves.sort_by(|left, right| {
        left.title
            .cmp(&right.title)
            .then(left.root.cmp(&right.root))
            .then(left.href.cmp(&right.href))
    });
    let href = canonical_collection_href(workspace, path);
    let mut overview = leaf(&href, "Overview", current);
    overview.collection = path.to_string();
    overview.current = collection_page_is_current(workspace, path, current);
    let mut children = vec![overview];
    children.extend(nested);
    children.extend(leaves);
    node.children = children;
    node.section_key = path.to_string();
    node.href = href;
    node.current = collection_is_current(workspace, path, current);
    node.open = node.current;
    if owners.len() == 1 {
        node.root = owners[0].clone();
    }
    node.summary = merged_collection_summary(workspace, path, owners);
    node
}

fn merged_collection_summary(workspace: &Workspace, path: &str, owners: &[String]) -> String {
    let mut parts = Vec::new();
    for root_id in owners {
        let Some(member) = workspace
            .members()
            .iter()
            .find(|member| member.id == *root_id)
        else {
            continue;
        };
        let Some(index) = member
            .bundle
            .indexes
            .iter()
            .find(|index| index.path.strip_suffix("/index.md") == Some(path))
        else {
            continue;
        };
        let text = collection_summary(index);
        if text.is_empty() {
            continue;
        }
        parts.push((root_id.as_str(), text));
    }
    match parts.as_slice() {
        [] => String::new(),
        [(_, text)] => text.clone(),
        many => many
            .iter()
            .map(|(root, text)| format!("{root}: {text}"))
            .collect::<Vec<_>>()
            .join("\n"),
    }
}
