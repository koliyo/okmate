---
type: Implementation Plan
title: Release version workflow
description: Replace promote tag with okmate-ops release that accepts patch, minor, major, an exact v* pin, or dev, without commit-message auto-tag.
tags: [domain/ops, domain/okmate, concern/ci, concern/tooling]
status: draft
generated: { by: process:cursor, at: 2026-08-28T10:26:09Z }
stale_after: 2026-11-28
authority: exploratory
owners: [human:nils]
sources:
  - id: research
    resource: ../../research/ops/release-version-workflow.md
    title: Release version workflow research
    author: process:cursor
    last_modified: 2026-08-28
  - id: promote
    resource: ../../../okmate-ops/src/okmate_ops/release.py
    title: Version commit, wait Test, tag, tap
    author: process:git
    last_modified: 2026-08-27
  - id: version
    resource: ../../../okmate-ops/src/okmate_ops/version.py
    title: Shared crate version rewrite
    author: process:git
    last_modified: 2026-08-27
  - id: ops-cli
    resource: ../../../okmate-ops/src/okmate_ops/cli.py
    title: okmate-ops command list
    author: process:git
    last_modified: 2026-08-27
  - id: ops-plan
    resource: okmate-ops.md
    title: okmate-ops uv toolkit
    author: process:cursor
    last_modified: 2026-08-27
  - id: readme
    resource: ../../../README.md
    title: OKMate README publish steps
    author: process:git
    last_modified: 2026-08-27
  - id: self-update
    resource: ../okmate/standalone-self-update.md
    title: Standalone Okmate self-update
    author: process:cursor
    last_modified: 2026-08-27
  - id: agents
    resource: ../../../AGENTS.md
    title: Rolling dev tag and promote fetch
    author: process:git
    last_modified: 2026-08-27
  - id: devops
    resource: ../../../.agents/skills/okmate-devops/SKILL.md
    title: DevOps skill promote tag notes
    author: process:git
    last_modified: 2026-08-27
---

# Release version workflow

## Goal

Give maintainers one `okmate-ops release` command that resolves
`patch|minor|major|vX.Y.Z|dev`, then runs today's version-commit →
wait `Test` → tag → Homebrew tap pipeline, without inferring versions
from commit messages. Today's CLI only lists `promote` / `tag`.[^research][^promote][^ops-cli][^ops-plan]

## Out of bound

Commit-message or Conventional Commits auto-tag. `Release-As` trailers.
Adopting cargo-release, release-please, release-plz, or semantic-release
as the publisher. crates.io. Changelog generation. Per-crate versions.
`promote staging|production`. Changing Test-then-tag, Sparkle
`SUFeedURL`, or immutable `v*`. Signing and Release.yml packaging.
h35-ops (same grammar can follow later; do not edit that repo here).

## Constraints that do not move

- A human (or an explicit `workflow_dispatch` later) chooses to cut.
  The CLI may compute the next patch/minor/major; it must not tag
  because a commit landed.[^research][^self-update]
- Shared `okf` / `okmate` version. Rewrite the same files
  `apply_release_version` already owns.[^version]
- Wait for hosted `Test` before pushing `v*` or moving `dev`.[^promote]
- `v*` stays immutable unless `--force`. `dev` force-moves and does not
  rewrite crate versions.[^readme]
- Fetch the target branch only. Do not force-fetch all tags.[^agents]
- Python 3.12, stdlib-only `okmate-ops` runtime, pytest in `dev`.

## Recommended CLI

```text
okmate-ops release <patch|minor|major|vX.Y.Z|dev> [--from BRANCH] [--force] [--dry-run]
```

`promote tag …` remains a deprecated alias through Phase 4, then
drops.[^research]

## Phase 1: Rename the verb

**Bound:** Route `release` to the existing promote pipeline. Accept the
same arguments as today's `promote tag` (`vX.Y.Z`, `dev`, `--from`,
`--force`). Help and usage strings say `release`. Keep `promote tag`
as an alias that prints one stderr deprecation line. Do not add bump
levels yet.

**Owner:** `okmate-ops/src/okmate_ops/cli.py`,
`okmate-ops/src/okmate_ops/promote.py` (or a `release.py` move if
imports stay thin), `okmate-ops/tests/test_cli.py`,
`okmate-ops/tests/test_promote.py`.

**Exit:** `uv run --no-dev pytest okmate-ops/tests/test_cli.py
okmate-ops/tests/test_promote.py`. `okmate-ops release v1.2.3` and
`okmate-ops promote tag v1.2.3` both reach `promote_tag`. Usage
includes `release`.

**Out of bound:** README and skill rewrites (Phase 4). Semver math.

## Phase 2: Bump levels

**Bound:** Add `next_release_version(current, level) -> str` in
`version.py` for `patch|minor|major` on `X.Y.Z` (no prerelease on the
current crate version in this phase). Resolve the current version from
the fetched target SHA via `first_package_version` on both Cargo
tomls; exit if they differ or if lock rows disagree. Map the level to
`vX.Y.Z` and call the existing pipeline. Print the resolved tag before
waiting on CI.

**Owner:** `okmate-ops/src/okmate_ops/version.py`, promote/release
command parser, `okmate-ops/tests/test_version.py`, promote tests.

**Exit:** `uv run --no-dev pytest okmate-ops/tests`. `0.1.2` + `patch`
→ `0.1.3`; `minor` → `0.2.0`; `major` → `1.0.0`. Exact `v*` and `dev`
unchanged. Mismatched crate versions exit nonzero.

**Out of bound:** Prerelease increment (`alpha`/`rc`). Reading bump
level from git log.

## Phase 3: Dry-run

**Bound:** `--dry-run` fetches and resolves the tag, reports whether
release files already match, and does not commit, push, tag, wait on
CI, or touch the tap.

**Owner:** promote/release module and tests.

**Exit:** `uv run --no-dev pytest okmate-ops/tests`. A dry-run test
asserts `run` / `push_version_update` / `wait_for_promote_ci` /
`push_tap_version` are not called after resolve.

**Out of bound:** Interactive confirm prompts.

## Phase 4: Operator docs and alias removal

**Bound:** README publish section, `AGENTS.md`,
`.agents/skills/okmate-devops/SKILL.md`, `packaging/macos/README.md`,
and the command list in `knowledge/plans/ops/okmate-ops.md` use
`okmate-ops release …`. Drop the `promote` alias and its tests.
The devops skill still names `promote tag`.[^devops]
Mention the new command in `standalone-self-update.md` only where it
names the human gate (do not reopen Sparkle phases).

**Owner:** those docs plus CLI usage.

**Exit:** `rg -n "promote tag" README.md AGENTS.md
.agents/skills/okmate-devops/SKILL.md packaging/macos/README.md
okmate-ops` finds no operator instructions (code comments that say
"formerly promote" are allowed). `uv run --no-dev pytest
okmate-ops/tests`. `uv run --no-dev okmate-ops` usage lists `release`.

**Out of bound:** Rewriting historical knowledge log bullets.

## Tests

- Parser: levels, `v*`, `dev`, unknown token, `--dry-run`.
- Semver: patch/minor/major; reject `1.2` and current prerelease until
  a later phase.
- Alias (Phases 1–3 only): `promote tag` still works.
- Dry-run does not push.

## Validation

```sh
uv run --no-dev pytest okmate-ops/tests
okmate check knowledge --profile strict --format terminal
```

Hosted `okmate-ops ci` after the docs phase if this crate's knowledge
records changed.

[^research]: Naming, cargo-release grammar, reject commit-message tag.
[^promote]: Current version commit, Test wait, tag, tap.
[^ops-cli]: Top-level commands; `promote` only routes `tag`.
[^ops-plan]: Documents `promote tag` and puts environment promote out of bound.
[^version]: Shared rewrite helpers.
[^self-update]: Human gate and Sparkle channel rules.
[^readme]: Current `promote tag` publish steps.
[^agents]: Fetch target branch only.
[^devops]: Skill still documents `promote tag`.
