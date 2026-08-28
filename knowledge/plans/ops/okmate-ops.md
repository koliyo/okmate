---
type: Implementation Plan
title: okmate-ops uv toolkit
description: Slim Python 3.12 operator package for local CI replay, versioned release, movable dev, and PR checkout.
tags: [domain/ops, domain/okmate, concern/ci, concern/tooling]
status: draft
generated: { by: process:cursor, at: 2026-08-28T10:45:00Z }
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
    title: release after version commit
    author: process:cursor
    last_modified: 2026-08-27
  - id: cask
    resource: https://github.com/koliyo/homebrew-okmate/blob/main/Casks/okmate.rb
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
- `release patch|minor|major|vX.Y.Z` — resolve the next shared crate
  version (or pin `vX.Y.Z`), write it to Cargo crate versions and
  `Cargo.lock`, push that commit to the target branch (default `main`),
  wait for hosted `Test` on the version commit, push the immutable tag,
  then bump `Casks/okmate.rb` on `koliyo/homebrew-okmate`[^promote][^cask]
- `release dev` — wait for hosted `Test` and force-move the rolling
  prerelease tag (no version rewrite)[^promote]
- Hosted **Cut release** runs that same command via `workflow_dispatch`
  from `main` (no `release` environment). Hosted **Release** packages
  an existing tag only (tag push or dispatch *from that tag*).

## Out of bound

Rocci site, origin, deploy, workspace-deps, editors, and
`promote staging|production`.

## Constraints that do not move

Python 3.12, hatchling, stdlib-only runtime, pytest in the `dev` group,
committed `uv.lock`. Do not force-fetch all git tags.

[^pyproject]: Package metadata and script entry.
[^readme]: Development, Homebrew tap, and rolling `dev` tag documentation.
[^promote]: Version commit on the target branch, then annotated `v*` or force-moved `dev`.
[^cask]: `koliyo/homebrew-okmate` installs the Sparkle `Okmate.zip` and `binary` links `$(brew --prefix)/bin/okmate`.
