---
type: Decision
title: Git working trees as the v1 authoring host
description: OKMate writes target local git working trees; discovery lists versioned OKF roots by marker without preferring knowledge/; explicit paths and registry entries stay first; agent jobs use CLI harnesses, not vendor APIs.
tags: [domain/okmate, domain/okf, concern/authoring, concern/architecture, concern/agents]
status: stable
generated: { by: process:cursor, at: 2026-09-12T10:20:00Z }
stale_after: 2026-12-12
authority: normative
owners: [human:nils]
sources:
  - id: research
    resource: ../research/okmate/heterogeneous-bundle-authoring.md
    title: Registry inventory, public examples, and implementation findings
  - id: plan
    resource: ../plans/okmate/heterogeneous-bundle-authoring.md
    title: Support heterogeneous OKF bundle authoring
  - id: review
    resource: ../research/okmate/review-queue-authoring.md
    title: Review-queue authoring, prompt query, and colocated code
  - id: verify
    resource: ../plans/okmate/verify-promote.md
    title: Verify and promote from the review UI
  - id: preview
    resource: ../../okf/src/preview.rs
    title: enclosing_bundle_root and is_bundle_root_index
  - id: load
    resource: ../../okf/src/load.rs
    title: Bundle discovery and reserved-file handling
  - id: config
    resource: ../../src/config.rs
    title: Directory and git root registry configuration
  - id: nav
    resource: ../../src/nav.rs
    title: Separated versus merged navigation
  - id: multi-roots
    resource: ../plans/okf/multi-knowledge-roots.md
    title: Directory roots writable; git URL roots are snapshots
---

# Git working trees as the v1 authoring host

## Context

Verify, promote, and later prompt-authoring assume a knowledge bundle that
sits beside code in the same git checkout. Provenance already walks from a
bundle path to `git rev-parse --show-toplevel`. Settings still allow a bare
folder or a fetched git URL snapshot. Vendor agent SDKs need API keys that
many desktop users do not have.[^review][^multi-roots]

An earlier draft inferred a bundle from the repo root plus immediate
children and preferred a child named `knowledge/` when several hits
remained. That rule would hide Studio-like `docs/`, gem-like `.okf/`, and
nested component roots, and it would guess among equally valid
corpora.[^research][^plan][^preview]

Approved 2026-09-12: keep the Git write boundary; replace one-level
`knowledge/`-preferring inference with bounded marker discovery that lists
ambiguity. Do not invent a human verification event for this revision.

## Decision

1. **Authoring writes require a local git working tree.** Verify, promote,
   and later writes run only when `git_repository_root` succeeds on the
   open path. Fetched git-cache snapshots stay read-only. Detached folders
   without git are preview-only.[^review][^multi-roots]

2. **Explicit bundle paths and registry entries retain priority.** A path
   the user or config already named is that root. Discovery does not
   replace `--register` IDs, and it does not auto-register what it
   finds.[^config]

3. **Discover versioned roots by marker, not basename.** A bundle root is a
   directory whose `index.md` carries `okf_version` (the same predicate as
   `is_bundle_root_index`). Scan a selected repository or container with
   bounded depth, ignore rules, and a reported visit limit. Include hidden
   `.okf/` and nested component locations. Do not recurse into `.git`,
   `target/`, `node_modules/`, or other ignored names. Collection
   `index.md` files without `okf_version` are not bundle roots.[^preview][^load]

4. **Ambiguity is visible.** Zero versioned hits → not a bundle; markerless
   trees may be listed as candidates with their evidence, not claimed as
   roots. One hit → that root. Several hits → list every path and require
   an explicit choice. Do not prefer `knowledge/` over `docs/`, `.okf/`,
   or another versioned child.[^research][^plan]

5. **Never flatten a container of bundles into one root.** Passing a git
   toplevel or parent folder that is not itself a versioned bundle must
   not load every nested Markdown tree as one corpus.[^load][^preview]

6. **Separated navigation is the predictable mixed-scope view.** Merged
   paths are a display choice, not semantic equivalence. Same-named types,
   collections, or concepts in two bundles stay distinct unless an author
   declares a convention.[^nav]

7. **Agent dispatch, when it exists, is CLI harnesses only** (`agent -p`,
   `claude -p`, `codex exec`, `pi -p`, or a custom argv). Do not embed
   Cursor/Anthropic/OpenAI SDKs or require `CURSOR_API_KEY` /
   `ANTHROPIC_API_KEY` for the desktop loop.[^review]

## Consequences

`okmate discover` lists candidates. `okmate view` on an explicit bundle
path is unchanged. `okmate view` on a container that is not a bundle
selects the unique candidate or errors with the list. Verify buttons hide
or fail closed when git is missing. Later work may re-open non-git writes;
it is not v1.

## Current disposition

Approved. Discovery listing and container `view` resolution are implemented
in this revision as Phase 5 of
[heterogeneous bundle authoring](/plans/okmate/heterogeneous-bundle-authoring.md);
do not log that phase complete until hosted CI succeeds.
The [verify and promote](/plans/okmate/verify-promote.md) plan still owns
the write UX; it must not revive `knowledge/`-preferring inference.[^verify][^plan]

[^research]: Five registered roots; directory name is not identity; list ambiguity.
[^plan]: Phase 5 bound: marker scan, no flatten, no `knowledge/` privilege; Git write boundary may stay.
[^review]: Ask vs Author cwd; git toplevel; CLI vs API.
[^verify]: Phased verify/promote; must follow this discovery contract.
[^preview]: Walk-up bundle detection already uses `okf_version`, not every `index.md`.
[^load]: Directory input was treated as one corpus; nested versioned indexes are not collection indexes.
[^config]: Existing root IDs and local/git paths remain authoritative.
[^nav]: Equal collection paths merge only in merged display mode.
[^multi-roots]: Writable directories vs read-only git URL cache.
