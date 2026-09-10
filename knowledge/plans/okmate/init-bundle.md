---
type: Implementation Plan
title: Initialize a new OKF bundle from the okmate CLI
description: Ship `okmate init` as a create-only, plan-then-apply scaffold for a local OKF v0.2 tree that `check` already understands, without a TUI studio, concept `new`, or git remotes.
tags: [domain/okmate, domain/okf, concern/authoring, concern/developer-experience, concern/tooling]
status: draft
generated: { by: process:cursor, at: 2026-09-10T19:40:00Z }
stale_after: 2026-12-10
authority: exploratory
owners: [human:nils]
sources:
  - id: gaps
    resource: ../../research/okmate/okf-tool-gaps.md
    title: OKMate feature gaps versus the OKF tool ecosystem
    author: process:cursor
    last_modified: 2026-08-28
  - id: serradura
    resource: ../../research/okmate/serradura-okf.md
    title: OKMate versus serradura/okf
    author: process:cursor
    last_modified: 2026-08-28
  - id: decision
    resource: ../../decisions/git-repository-bundles.md
    title: Git working trees as the v1 authoring host
    author: process:cursor
    last_modified: 2026-08-28
  - id: nested
    resource: ../../decisions/nested-okf-collections.md
    title: Nest large OKF collections under okf, okmate, and ops
    author: process:cursor
    last_modified: 2026-08-26
  - id: overview
    resource: ../../architecture/system-overview.md
    title: OKMate system overview
    author: process:cursor
    last_modified: 2026-08-30
  - id: cli
    resource: ../../../src/cli.rs
    title: okmate CLI surface
    author: process:git
    last_modified: 2026-09-04
  - id: config
    resource: ../../../src/config.rs
    title: UserConfig roots TOML, valid_id, save
    author: process:git
    last_modified: 2026-08-28
  - id: settings
    resource: ../../../src/http/settings.rs
    title: add_directory settings action
    author: process:git
    last_modified: 2026-08-28
  - id: preview
    resource: ../../../okf/src/preview.rs
    title: is_bundle_root_index
    author: process:git
    last_modified: 2026-08-28
  - id: load
    resource: ../../../okf/src/load.rs
    title: Root index may only contain okf_version
    author: process:git
    last_modified: 2026-08-28
  - id: readme
    resource: ../../../README.md
    title: Published OKMate CLI table
    author: process:git
    last_modified: 2026-09-04
  - id: check-tests
    resource: ../../../tests/check.rs
    title: CLI check tests via CARGO_BIN_EXE_okmate
    author: process:git
    last_modified: 2026-08-28
  - id: w4g1
    resource: https://github.com/W4G1/okf
    title: W4G1 okf init, new, and studio TUI
    author: organization:w4g1
  - id: workbench
    resource: https://github.com/koizumikento/okf-workbench
    title: Workbench init templates and plan-then-apply writes
    author: human:koizumikento
  - id: okfcli
    resource: https://github.com/okfcli/okf
    title: okfcli init of an empty bundle
    author: organization:okfcli
  - id: openknowledge
    resource: https://github.com/openknowledge-sh/openknowledge
    title: openknowledge scaffold and setup
    author: organization:openknowledge-sh
---

# Initialize a new OKF bundle from the okmate CLI

## Purpose and authority

This plan executes the **`init` half** of the scaffolding gap in
[OKMate feature gaps versus the OKF tool ecosystem](/research/okmate/okf-tool-gaps.md).
It does not start a phase by being written. The record is exploratory.[^gaps]

Peer CLIs already treat `init` as the first verb after install. W4G1/okf
creates `index.md`, `log.md`, and a sample concept unless `--bare`; `okf
studio` is a later TUI over a live bundle, not the scaffolder.
Workbench initializes Minimal / Software Project / Data & Analytics
trees with create-only writes and requires `--apply` for non-interactive
CLI writes. okfcli and akdira's toolkit create an empty bundle directory.
Open Knowledge splits `scaffold` (files) from `setup` (agent-guided).
OKMate should copy the **batch verb and the fail-closed write model**, not
studio's explorer/graph/trust tabs.[^gaps][^w4g1][^workbench][^okfcli][^openknowledge]

## Goal

`okmate init [path]` prints a create-only file plan for a new OKF v0.2
bundle and, with `--apply`, writes it. The default path is `knowledge`,
matching `check` / `build`. After apply, `okmate check <path> --profile
strict` has no errors. Optional `--register` appends a directory root to
`~/.okmate/config.toml`. Optional `--agents` writes create-only agent
routing beside the git toplevel.[^cli][^gaps][^config][^readme]

## Out of bound

- `okmate new` / concept templates / Attested Computation stubs.
- Managed `index.md` regeneration, `fmt`, `mv` / `rm` / split / merge.
- A TUI (`okf studio`), VS Code workbench, or 3D graph as an init UI.
- `git init`, commit, GitHub remotes, or `gh repo create`.
- Overwriting or surgically editing existing files (including merging a
  line into an existing `.gitattributes` or `AGENTS.md`).
- `--force` replacing a bundle. Idempotent re-init of an existing bundle.
- Write MCP, vendor agent APIs, or spawning `agent` / `claude` / `codex`.
- Changing `okf` public API or `is_bundle_root_index` visibility unless a
  later phase is blocked without it.
- Minting an approved Decision. Settings UI for init.[^gaps][^w4g1][^workbench][^decision]

## Constraints that do not move

- Records stay inert Markdown. Root `index.md` frontmatter is only
  `okf_version: "0.2"`.[^load]
- Writes are create-only and collision-safe. Existing `index.md` with
  `okf_version` at the target is an error, not a merge. Workbench's
  plan-then-`--apply` is the write model: default is a plan, `--apply`
  publishes it.[^gaps][^workbench]
- Default path remains `knowledge` so `okmate init --apply` in a git
  checkout matches how this repo, Rocci, and h35-internal lay out a
  child bundle. `okmate init . --apply` is the W4G1 "bundle at this
  directory" form.[^cli][^decision]
- Type collections stay type-first and flat
  (`architecture/`, `decisions/`, `status/`, `plans/`, `research/`,
  `audits/`). Do not nest this product's `okf` / `okmate` / `ops` areas
  into a generic scaffold. Area directories appear later, when a record
  exists.[^nested]
- Do not register a root unless `--register`. Reuse `valid_id`,
  `Incoming::Allow`, and `save`; do not duplicate settings POST
  parsing.[^config][^settings]
- `--agents` files live at the git toplevel (the parent of `knowledge/`
  when the bundle is a child). Skip them when git is missing. Create-only;
  if `AGENTS.md` already exists, omit it and list it in the plan as
  skipped.[^decision][^workbench]
- Do not default `--owner` from `git config user.name`. Actor for a
  future sample concept, if any, is an explicit flag or settings
  `actor`.[^decision]
- Do not mix unrelated working-tree changes into phase commits.

## Current behavior

`okmate` has no `init` / `new`. Scaffolding is hand-written Markdown.
`check` defaults to path `knowledge`. A bundle root is an `index.md`
whose frontmatter contains `okf_version`. Settings can add a directory
root after the tree exists. Authoring research still assumes a git
working tree; init must not require git to write a valid bundle, and must
not pretend a missing git toplevel is fatal.[^cli][^preview][^settings][^gaps][^serradura]

## Inspiration (what to copy, what not to)

| Peer | Take | Leave |
| --- | --- | --- |
| W4G1 `okf init [path] --title --bare` | Path + title + bare (no sample concept). Post-init verb is `okmate view`, not `okf studio`. | Sample Policy concept, `okf new`, TUI studio, `lint --fix`. |
| Workbench CLI | Plan printed first; `--apply` required to write; create-only; templates named; agent files optional and previewed. | Three product templates (Software Project / Data & Analytics), Wasm/VSIX, managed-region edits of existing `AGENTS.md`. |
| okfcli `okf init` | Empty bundle is enough. JSON-friendly later. | Regenerating every `index.md` as part of init. |
| Open Knowledge `scaffold` | Optional agent handoff (`--agents` / skip). | Agent-harness `setup`, hosted runtime, spec vendoring. |

OKMate already chose a desktop review window over a graph TUI. Init
should print "next: `okmate check <path>` then `okmate view <path>`".
Do not add `okmate studio`.[^gaps][^w4g1][^overview]

## Phases

### Phase 1 — Plan type and dry-run CLI

**Bound:** `src/init.rs` plus `Commands::Init` in `src/cli.rs`:

```text
okmate init [path]
  --title <string>
  --bare
  --apply
  --format terminal|json
```

- `path` default `knowledge`.
- Without `--apply`, print the plan and write nothing. Exit 0.
- `--format json` emits the plan object, no secrets.
- `InitPlan { root: PathBuf, files: Vec<InitFile> }` where
  `InitFile { relative: PathBuf, op: Create, bytes_hint: usize }`.
- `fn plan_bundle(opts: &InitOptions) -> Result<InitPlan>`.
- Detect an existing bundle at `path` by reading `path/index.md` and
  treating `okf_version` in frontmatter as occupied (inline check; do not
  export `is_bundle_root_index` yet). Error, do not plan over it.
- `--bare`: only `index.md` and `log.md`.
- Default template: those two plus collection `index.md` files listed
  under Goal, each body a heading and "No records yet." (audits: reserved
  one-liner). Root index lists the collections. `log.md` uses the union
  driver note and a `## YYYY-MM-DD` heading with one scaffold bullet.
- `--title` sets the root index `#` heading (default: `Knowledge`).

**Out of bound:** `--register`, `--agents`, `.gitattributes`, git, apply.

**Tests:** `tests/init.rs` via `okmate_bin()`: help lists `init`; dry-run
on a temp dir prints `index.md` and does not create it; `--format json`
parses; occupied bundle errors; `--bare` omits `architecture/index.md`.[^check-tests]

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`.

**Owner:** `src/init.rs`, `src/cli.rs`, `src/lib.rs` if needed,
`tests/init.rs`, `tests/common/mod.rs` only if a tiny helper is shared.

### Phase 2 — Apply create-only writes

**Bound:** `fn apply_plan(plan: &InitPlan) -> Result<()>`:

- Refuse if any planned path exists.
- Create parent directories; write files; no overwrite.
- After write, `okf::check(&root, Profile::Strict)` must report no
  errors. Surface engine errors if the template is wrong.
- Terminal apply output: written relative paths, then the check/view
  hint.

**Out of bound:** config.toml, agent files, git.

**Tests:** `--apply` on a missing `knowledge/` child creates a tree that
`okmate check <that> --profile strict --format json` accepts; second
`--apply` fails without mutating; `--bare --apply` check-clean; refuse
when `index.md` already has `okf_version`.

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`.

**Owner:** `src/init.rs`, `tests/init.rs`.

### Phase 3 — Register and agent extras

**Bound:**

- `--register` after a successful apply (or on dry-run: include a
  config mutation in the plan and still require `--apply` to save).
  `id` from `--id`, else kebab-case of the bundle directory name through
  `valid_id`. Duplicate id errors. Path stored absolute. `incoming =
  allow`. Reuse `load_or_default` + `save`. Tests set `OKMATE_CONFIG` to
  a temp file (same pattern as settings tests).
- `--agents`: extra create-only files at git toplevel when
  `git rev-parse --show-toplevel` succeeds: `AGENTS.md`,
  `.cursor/rules/write-knowledge.mdc`,
  `.agents/skills/manage-knowledge/SKILL.md`. Bodies are generic (inert
  Markdown, `okmate check knowledge --profile strict`, write-knowledge
  routing to `knowledge/`). Skip each path that exists; never patch.
- If the git toplevel has no `.gitattributes`, create one with
  `{bundle-rel}/log.md merge=union`. If it exists, skip and print the
  line to add.

**Out of bound:** `git init`, remotes, skill installer as a separate
command (`okmate skills install` stays the later gaps item).

**Tests:** `--register --apply` with `OKMATE_CONFIG` adds a directory
root; duplicate id fails; `--agents --apply` in a temp git repo writes
the three files and skips when `AGENTS.md` is pre-created; no git →
`--agents` errors or no-ops with a clear message (pick one and test it:
error).

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`.

**Owner:** `src/init.rs`, `src/config.rs` only if a small
`push_directory_root` helper avoids copying settings POST, `tests/init.rs`.

### Phase 4 — Published CLI contract

**Bound:** README CLI table and example block: `okmate init`,
`okmate init --apply`, `okmate init . --bare --apply`,
`okmate init --apply --register --id my-bundle`. Clap `about` text.
No website, no skill rewrite in this repository beyond README.

**Out of bound:** Product site `/agents/` page.

**Tests:** `no_subcommand_prints_help` (or sibling) mentions `init`.

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check` and
`okmate check knowledge --profile strict --format terminal`.

**Owner:** `README.md`, `src/cli.rs` help strings, `tests/check.rs` or
`tests/init.rs` help assertion.

## Validation (every phase)

```sh
cargo fmt --all -- --check
cargo test -p okmate --no-default-features
okmate check knowledge --profile strict --format terminal
```

`cargo test -p okf` only if a phase touched `okf/`. Report lifecycle and
provenance warnings separately from errors.

[^gaps]: No init/new today; Workbench plan-then-apply; peers listed.
[^serradura]: Scaffolding named as a gap versus okf-gem produce/migrate.
[^decision]: Git working tree for later authoring; one-level bundle inference.
[^nested]: Type-first collections; areas only when a record exists.
[^overview]: Engine versus CLI versus knowledge ownership.
[^cli]: Subcommands; default `knowledge` on check/build; no init.
[^config]: Directory roots, `valid_id`, atomic TOML save.
[^settings]: `add_directory` already registers a folder.
[^preview]: Bundle-root predicate via `okf_version`.
[^load]: Root index frontmatter may only contain `okf_version`.
[^readme]: CLI table agents copy.
[^check-tests]: Binary integration tests.
[^w4g1]: `okf init --title --bare`; studio is a later TUI.
[^workbench]: `--apply` / `--check`; create-only; optional agent files.
[^okfcli]: `okf init ./my-bundle` empty tree.
[^openknowledge]: `scaffold` versus agent `setup`.
