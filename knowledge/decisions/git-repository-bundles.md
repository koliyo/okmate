---
type: Decision
title: Git working trees as the v1 authoring host
description: OKMate authoring initially targets local git working trees only; infer the OKF bundle from a root index.md with okf_version at the repo root or one directory down; agent jobs use CLI harnesses, not vendor APIs.
tags: [domain/okmate, domain/okf, concern/authoring, concern/architecture, concern/agents]
status: draft
generated: { by: process:cursor, at: 2026-08-28T17:50:00Z }
stale_after: 2026-11-28
authority: exploratory
owners: [human:nils]
sources:
  - id: research
    resource: ../research/okmate/review-queue-authoring.md
    title: Review-queue authoring, prompt query, and colocated code
    author: process:cursor
    last_modified: 2026-08-28
  - id: plan
    resource: ../plans/okmate/verify-promote.md
    title: Verify and promote from the review UI
    author: process:cursor
    last_modified: 2026-08-28
  - id: preview
    resource: ../../okf/src/preview.rs
    title: enclosing_bundle_root and is_bundle_root_index
    author: process:git
    last_modified: 2026-08-28
  - id: validate
    resource: ../../okf/src/validate.rs
    title: git_repository_root via rev-parse --show-toplevel
    author: process:git
    last_modified: 2026-08-28
  - id: lib
    resource: ../../okf/src/lib.rs
    title: Root index.md may only contain okf_version
    author: process:git
    last_modified: 2026-08-28
  - id: multi-roots
    resource: ../plans/okf/multi-knowledge-roots.md
    title: Directory roots writable; git URL roots are snapshots
    author: process:cursor
    last_modified: 2026-08-25
---

# Git working trees as the v1 authoring host

## Context

Verify, promote, and later prompt-authoring assume a knowledge bundle that
sits beside code in the same git checkout. Provenance already walks from
`knowledge/` to `git rev-parse --show-toplevel`. Settings still allow a
bare folder or a fetched git URL snapshot. Vendor agent SDKs need API
keys that many desktop users do not have.[^research][^validate][^multi-roots]

## Decision

1. **Authoring requires a local git working tree.** Verify, promote, and
   later writes run only when `git_repository_root` succeeds on the open
   path. Fetched git-cache snapshots stay read-only. Detached folders
   without git are preview-only.

2. **Do not ask for the knowledge sub-path.** Given a git toplevel, find
   OKF bundle roots by testing `index.md` that contains `okf_version`
   (the same predicate as `is_bundle_root_index`). Check the repo root
   and each **immediate** child directory. Do **not** recurse deeper:
   collection `index.md` files are not bundle roots, and a deep walk
   would also hit `target/`, `node_modules/`, and nested clones.[^preview][^lib]

3. **Ambiguity.** Zero hits → error naming the rule. One hit → use it.
   Several hits → if exactly one child is named `knowledge`, use that;
   otherwise error and list paths. Do not guess among `docs/` vs
   `handbook/`.

4. **Agent dispatch, when it exists, is CLI harnesses only** (`agent -p`,
   `claude -p`, `codex exec`, `pi -p`, or a custom argv). Do not embed
   Cursor/Anthropic/OpenAI SDKs or require `CURSOR_API_KEY` /
   `ANTHROPIC_API_KEY` for the desktop loop.[^research]

This decision is not approved.

## Consequences

`okmate view <repo>` can resolve `./knowledge` without a `bundle`
config key. Users pick a project folder, not an OKF folder. Verify
buttons hide or fail closed when git is missing. A later phase may
re-open non-git directories and recursive discovery; it is not v1.

## Current disposition

Exploratory draft. The first implementation slice is
[verify and promote](/plans/okmate/verify-promote.md).[^plan]

[^research]: Ask vs Author cwd; git toplevel; CLI vs API.
[^plan]: Phased verify/promote; discovery tests.
[^preview]: Walk-up bundle detection already uses `okf_version`, not every `index.md`.
[^validate]: Provenance git toplevel.
[^lib]: Only the bundle-root index may carry `okf_version`.
[^multi-roots]: Writable directories vs read-only git URL cache.
