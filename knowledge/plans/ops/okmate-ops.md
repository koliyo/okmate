---
type: Implementation Plan
title: okmate-ops uv toolkit
description: Slim Python 3.12 operator package for local CI replay, versioned promote tag, movable dev, and PR checkout.
tags: [domain/ops, domain/okmate, concern/ci, concern/tooling]
status: draft
generated: { by: process:cursor, at: 2026-08-27T06:05:00Z }
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
    last_modified: 2026-08-27
  - id: promote
    resource: ../../../okmate-ops/src/okmate_ops/promote.py
    title: promote tag after version commit
    author: process:cursor
    last_modified: 2026-08-27
  - id: cask
    resource: ../../../Casks/okmate.rb
    title: Homebrew cask for Okmate.app from GitHub Releases
    author: process:cursor
    last_modified: 2026-08-27
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
- `promote tag vX.Y.Z` — write `X.Y.Z` to Cargo crate versions, `Cargo.lock`,
  and `Casks/okmate.rb`, push that commit to the target branch (default
  `main`), wait for hosted `Test` on the version commit, then push the
  immutable tag[^promote][^cask]
- `promote tag dev` — wait for hosted `Test` and force-move the rolling
  prerelease tag (no version rewrite)[^promote]

## Out of bound

Rocci site, origin, deploy, workspace-deps, editors, and
`promote staging|production`.

## Constraints that do not move

Python 3.12, hatchling, stdlib-only runtime, pytest in the `dev` group,
committed `uv.lock`. Do not force-fetch all git tags.

[^pyproject]: Package metadata and script entry.
[^readme]: Development, Homebrew tap, and rolling `dev` tag documentation.
[^promote]: Version commit on the target branch, then annotated `v*` or force-moved `dev`.
[^cask]: In-repo tap cask installs the Sparkle `Okmate.zip` and `binary` links `$(brew --prefix)/bin/okmate`.
