---
type: Implementation Plan
title: okmate-ops uv toolkit
description: Slim Python 3.12 operator package for local CI replay, promote tag including movable dev, and PR checkout.
tags: [domain/ops, domain/okmate, concern/ci, concern/tooling]
status: draft
generated: { by: process:cursor, at: 2026-08-26T16:30:00Z }
stale_after: 2026-11-26
authority: exploratory
owners: [human:nils]
sources:
  - id: pyproject
    resource: ../../../okmate-ops/pyproject.toml
    title: okmate-ops uv project metadata
    author: process:cursor
    last_modified: 2026-08-26
  - id: readme
    resource: ../../../README.md
    title: Okmate README
    author: process:cursor
    last_modified: 2026-08-26
---

# okmate-ops uv toolkit

## Goal

Give CI and localhost one `uv run --no-dev okmate-ops` surface so hosted
`.github/workflows/ci.yml` and local replay cannot drift.[^pyproject][^readme]

## Commands

- `ci` — `cargo fmt --all -- --check`, `cargo test -p okf`,
  `cargo test -p okmate --no-default-features`, and `okmate check knowledge`
  when `knowledge/index.md` exists
- `pr-checkout` — list open PRs or checkout `pr/<branch>`
- `promote tag vX.Y.Z` and `promote tag dev` — wait for the hosted `Test`
  check, then push the tag (`dev` is force-moved)

## Out of bound

Rocci site, origin, deploy, workspace-deps, editors, and
`promote staging|production`.

## Constraints that do not move

Python 3.12, hatchling, stdlib-only runtime, pytest in the `dev` group,
committed `uv.lock`. Do not force-fetch all git tags.

[^pyproject]: Package metadata and script entry.
[^readme]: Development and rolling `dev` tag documentation.
