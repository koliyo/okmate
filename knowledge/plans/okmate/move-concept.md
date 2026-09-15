---
type: Implementation Plan
title: Move a concept with link-aware CLI and nav drag-drop
description: Ship `okmate move` as a plan-then-apply rewrite of a concept path, inbound Markdown hrefs, indexes, and relative sources, then expose the same planner on live preview via nav drag-and-drop with a confirmation dialog.
tags: [domain/okmate, domain/okf, concern/authoring, concern/developer-experience, concern/tooling]
status: draft
generated: { by: process:cursor, at: 2026-09-15T08:24:00Z }
stale_after: 2026-12-15
authority: exploratory
owners: [human:nils]
sources:
  - id: gaps
    resource: ../../research/okmate/okf-tool-gaps.md
    title: OKMate feature gaps versus the OKF tool ecosystem
    author: process:cursor
    last_modified: 2026-08-28
  - id: authoring-guide
    resource: ../../../docs/authoring.md
    title: Authoring an OKF bundle
    author: process:git
  - id: load
    resource: ../../../okf/src/load.rs
    title: Concept id is the bundle-relative path without .md
    author: process:git
  - id: graph
    resource: ../../../okf/src/graph.rs
    title: resolve_bundle_path and concept link graph
    author: process:git
  - id: authoring
    resource: ../../../src/authoring.rs
    title: concept and index plan-then-apply writes
    author: process:git
  - id: cli
    resource: ../../../src/cli.rs
    title: okmate CLI surface
    author: process:git
  - id: settings-http
    resource: ../../../src/http/settings.rs
    title: Loopback POST mutation for settings
    author: process:git
  - id: nav
    resource: ../../../src/nav.rs
    title: Collection and concept sidebar tree
    author: process:git
  - id: init-plan
    resource: init-bundle.md
    title: Init scaffold; mv was out of bound there
    author: process:cursor
  - id: w4g1
    resource: https://github.com/W4G1/okf
    title: W4G1 semantic mv with backlink rewrite
    author: organization:w4g1
---

# Move a concept with link-aware CLI and nav drag-drop

## Purpose and authority

Peer OKF CLIs already treat link-aware `mv` as a maintenance verb. W4G1
rewrites backlinks from a preview; this plan copies the
**plan-then-`--apply` write model** already used by `okmate concept` and
`okmate index`, not a TUI studio.[^gaps][^authoring][^w4g1][^init-plan]

Concept identity is the bundle-relative path without `.md`. Intra-bundle
links are ordinary Markdown (`/path.md` or relative). There are no
wikilinks. Inbound path links, collection `index.md` membership, and
relative `sources[].resource` values are deterministic to rewrite.
Unlinked prose, type/tag IA, other bundles, and redirect stubs stay
human or agent notes.[^authoring-guide][^load][^graph]

This record is exploratory. Writing it does not start a phase.

## Goal

`okmate move [root] --from ID --to ID-or-collection/` prints a rewrite
plan and, with `--apply`, relocates one concept file and updates
path-based Markdown links in that bundle. Live preview (`okmate view`)
offers the same plan from nav drag-and-drop onto a collection, with a
confirmation dialog, loopback-only.[^cli][^settings-http][^nav]

## Out of bound

- Collection-directory moves, `rm`, split, or merge.
- Redirect stubs, auto-changing `type` or tags, appending `log.md`.
- Rewriting other registered roots or `okf:` links in other bundles.
- Markdown body editors, vendor agent APIs, Write MCP.
- Static `okmate build` HTML applying moves.
- Minting an approved Decision.

## Constraints that do not move

- Records stay inert Markdown. Do not pretty-print bodies through
  Comrak; replace authored href substrings.[^graph][^authoring-guide]
- Default is a plan; `--apply` (or the dialog confirm) writes. Collisions
  fail closed, matching `concept` / `index`.[^authoring]
- `--to` ending in `/` keeps the stem. Dest `id` must pass the same
  relative-id rules as `okmate concept` (no `.`, `..`, or `index`).
- Preserve authored href style: bundle-root `/old.md` stays bundle-root;
  relative stays relative and is recomputed. Fragments stay.
- Source collection index: rewrite the listing href on same-collection
  rename; on a different collection, remove a bullet list item that
  contains the link, otherwise rewrite and note.
- Dest collection index: append a membership bullet without regrouping
  authored headings, same shape as `okmate index`. Missing dest index is
  a note, not a created file.
- Git-cache preview members and non-directory roots refuse writes.
- Do not mix unrelated working-tree changes into phase commits.

## Phases

### Phase 1 — Planner and CLI

**Bound:** `okmate move [root] --from --to` with `--apply` and
`--format json|terminal`. Planner in `src/move_concept.rs`. Re-export
`okf::resolve_bundle_path` and `split_fragment` if authoring would
otherwise drift from the graph. Surgical inbound hrefs, outbound
relative hrefs when the directory changes, relative
`sources[].resource`, index membership, unlinked-mention notes, optional
OKMATE5002-style type-path note. README and `docs/authoring.md`.

**Out of bound:** HTTP, nav drag-drop.

**Tests:** Dry-run writes nothing; apply rewrites bundle-root and
relative inbound links plus fragments; source index loses membership and
dest index gains it; dest collision fails; relative `sources[].resource`
recomputes; `--to audits/` keeps the stem.

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`. After knowledge and docs edits:
`okmate check knowledge --profile strict`.

**Owner:** `src/move_concept.rs`, `src/cli.rs`, `okf/src/lib.rs`,
`docs/authoring.md`, `README.md`, `tests/move_concept.rs`.

### Phase 2 — Live HTTP and nav drag-drop

**Bound:** Loopback `GET /__okmate/move` returns the JSON plan;
`POST /__okmate/move` with `apply` writes, reloads the workspace, and
returns the new document href. Nav concept leaves are draggable;
collection `<details>` are drop targets. Confirmation dialog shows from,
editable to, edit counts, and notes. Live preview only
(`data-okmate-live`). Full-page navigate after success so the sidebar
rebuilds; do not also show the workspace-reload dialog. Cross-root drops
and git-cache members refuse.

**Out of bound:** Static export applying moves; collection-tree drags.

**Tests:** Non-loopback 403; apply then reload sees the new id; `build`
HTML has no live attribute.

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`.

**Owner:** `src/http/move_concept.rs`, `src/http/mod.rs`, `src/nav.rs`,
`src/views/mod.rs`, `templates/`, `assets/move.js`.

[^gaps]: Medium gap 12 is link-aware rename/move with dry-run; W4G1 already rewrites backlinks.
[^authoring-guide]: ID is the path without `.md`; prefer bundle-root `/path.md` links.
[^load]: `parse_concept` sets `id` by stripping `.md` from the relative path.
[^graph]: `resolve_bundle_path` and `concept.links` / `index.links` are the rewrite inputs.
[^authoring]: `concept` and `index` print a plan; `--apply` writes.
[^cli]: Authoring commands are clap variants on `Commands` in `src/cli.rs`.
[^settings-http]: Settings POST is loopback-only; move follows that guard.
[^nav]: The sidebar is a nested collection tree of `<details>` plus concept leaves.
[^init-plan]: Init explicitly left `mv` / `rm` / split / merge out of bound.
[^w4g1]: Semantic `mv` with preview and backlink rewrite.
