---
type: Implementation Plan
title: Verify and promote from the review UI
description: Ship loopback verify and promote on live preview for git working trees, inferring the OKF bundle from a one-level okf_version index.md search, without Markdown editors or vendor agent APIs.
tags: [domain/okmate, domain/okf, concern/review, concern/authoring, concern/developer-experience]
status: draft
generated: { by: process:cursor, at: 2026-08-28T17:50:00Z }
stale_after: 2026-11-28
authority: exploratory
owners: [human:nils]
sources:
  - id: research
    resource: ../../research/okmate/review-queue-authoring.md
    title: Review-queue authoring, prompt query, and colocated code
    author: process:cursor
    last_modified: 2026-08-28
  - id: decision
    resource: ../../decisions/git-repository-bundles.md
    title: Git working trees as the v1 authoring host
    author: process:cursor
    last_modified: 2026-08-28
  - id: review-engine
    resource: ../../../okf/src/review.rs
    title: classify_concept_action ActionKind
    author: process:git
    last_modified: 2026-08-28
  - id: preview-okf
    resource: ../../../okf/src/preview.rs
    title: is_bundle_root_index and enclosing_bundle_root
    author: process:git
    last_modified: 2026-08-28
  - id: validate
    resource: ../../../okf/src/validate.rs
    title: git_repository_root and latest_human_verification
    author: process:git
    last_modified: 2026-08-28
  - id: settings-http
    resource: ../../../src/http/settings.rs
    title: Loopback POST, re-read, Datastar morph
    author: process:git
    last_modified: 2026-08-28
  - id: http
    resource: ../../../src/http/mod.rs
    title: Live routes; settings is the only POST mutation
    author: process:git
    last_modified: 2026-08-28
  - id: queue
    resource: ../../../templates/fragments/queue.html
    title: Review queue fragment
    author: process:git
    last_modified: 2026-08-28
  - id: article
    resource: ../../../templates/fragments/article.html
    title: Concept meta alert
    author: process:git
    last_modified: 2026-08-28
  - id: skill
    resource: ../../../.agents/skills/manage-okmate-knowledge/SKILL.md
    title: Never invent human verification
    author: process:git
    last_modified: 2026-08-26
  - id: extract
    resource: ../okf/okmate.md
    title: Okmate extract; queue writes out of bound there
    author: process:cursor
    last_modified: 2026-08-26
  - id: config
    resource: ../../../src/config.rs
    title: UserConfig roots TOML
    author: process:git
    last_modified: 2026-08-28
---

# Verify and promote from the review UI

## Purpose and authority

This plan executes the verify/promote slice of
[review-queue authoring research](/research/okmate/review-queue-authoring.md)
under [git working trees as the v1 authoring host](/decisions/git-repository-bundles.md).
It does not start a phase by being written. The record is exploratory.[^research][^decision]

## Goal

On live `okmate view`, a reviewer can **Verify** (append a `human:` event)
and **Promote** (`draft` → `stable`) for a concept in a git working tree.
The OKF bundle path is inferred. After a successful POST the queue and
concept meta remorph from a fresh classify. No Markdown editor. No agent
process.[^research][^review-engine][^settings-http]

## Out of bound

- Frontmatter forms, body editors, or `lint --fix`.
- Ask/query UI, Author-from-prompt, Fix Errors, Refresh Stale, Track/Commit.
- Spawning `agent`, `claude`, `codex`, `pi`, or any vendor SDK / HTTP
  completions API.
- Writable fetched git-cache roots (`GitRoot` snapshots).
- Recursing the repo for `index.md` beyond the toplevel and its immediate
  children.
- Changing `classify_concept_action`.
- Git commit/push of the mutation (the working tree file changes; the
  human commits).
- Minting an approved Decision. Static `okmate build` HTML must not grow
  these controls.[^extract][^decision]

## Constraints that do not move

- Records stay inert Markdown. Okmate writes YAML by surgical edit, then
  `okf` re-reads and classifies.[^skill]
- Loopback-only POST. Tokens never echoed. Same shape as settings:
  validate, write, re-read, morph a stable id.[^settings-http][^http]
- Authoring only if `git_repository_root` is `Some`. Hide buttons when
  git is missing; POST still 403/409 if reached.[^validate][^decision]
- Infer bundle with `okf_version` on `index.md`, not every collection
  index. Scan repo root + one directory level. Prefer a unique child
  named `knowledge` when several match.[^preview-okf][^decision]
- Append `verified`; never replace the list; never write `human:` from a
  process. Actor comes from settings, not `git user.name`.[^skill][^research]
- Promote is a separate control, default off the Verify path. Refuse
  Promote when `authority` is `exploratory`, when status is not `draft`,
  or when there is no `human:` event.[^review-engine]
- Compare-and-swap: POST carries a content hash of the concept file;
  mismatch fails without write.[^research]
- Do not mix unrelated working-tree changes into phase commits.

## Current behavior

Queue and concept pages display `ActionKind` only. The sole mutating POST
is `/__okmate/settings`. `okmate view` on a directory treats that
directory as the bundle root and does not infer `knowledge/` from a git
toplevel. `is_bundle_root_index` already exists but is private to preview
path resolution.[^http][^queue][^article][^preview-okf]

## Phases

### Phase 1 — Discover bundle in a git checkout

**Bound:** Public `okf` helpers:

- `is_bundle_root_index(source: &str) -> bool` (move from `preview.rs`)
- `fn discover_bundles(git_toplevel: &Path) -> Vec<PathBuf>`
  — candidates: `git_toplevel/index.md` and `git_toplevel/<child>/index.md`
  for each non-hidden immediate subdirectory (skip names starting with
  `.`). No recursion.
- `fn resolve_bundle(git_toplevel: &Path) -> Result<PathBuf>`
  — 0 → error; 1 → that path; many → if exactly one file_name is
  `knowledge`, that path, else error listing all.

`okmate view <dir>`: if `dir` is a git toplevel and not itself a bundle
root, use `resolve_bundle`. If `dir` is already a bundle (root index has
`okf_version`), keep current behavior. `git_repository_root` stays the
git test.

**Out of bound:** Settings UI, verify POST, changing `GitRoot` fetch.

**Tests:** temp git repos (and a non-git dir) covering: `knowledge/`
child; bundle at repo root; `plans/index.md` collection ignored;
two bundles without a `knowledge/` name → error; `knowledge/` plus
another hit → `knowledge/`; hidden `.foo/index.md` ignored.

**Exit:** `cargo test -p okf` and `cargo test -p okmate --no-default-features`
and `cargo fmt --all -- --check`.

**Owner:** `okf/src/preview.rs` (or a small `okf/src/discover.rs`),
`okf/src/lib.rs` re-exports, `src/preview.rs` / `src/workspace.rs` view
resolution.

### Phase 2 — Surgical verify and promote writes

**Bound:** Okmate module (not a general YAML emitter), e.g.
`src/author.rs`:

- `fn file_hash(bytes: &[u8]) -> String` (hex sha256)
- `fn append_verification(source: &str, by: &str, at: &str) -> Result<String>`
  — `by` must start with `human:`; insert or append a list item
  `- { by: …, at: … }`; preserve body and unknown keys; do not rewrite
  the whole mapping through a serializer if that drops comments (prefer
  line surgery on the `verified:` block / insert before closing `---`)
- `fn set_status(source: &str, from: &str, to: &str) -> Result<String>`
  — replace a `status:` line only when current value is `from`

Unit tests on fixture strings: missing `verified`, existing list, extra
custom keys survive, malformed frontmatter errors, `process:cursor` `by`
rejected.

**Out of bound:** HTTP, git.

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`.

**Owner:** `src/author.rs`, `src/lib.rs`.

### Phase 3 — Reviewer actor in config

**Bound:** `UserConfig` field `actor: Option<String>` (TOML
`actor = "human:nils"`). Settings form: one input, POST `action=actor`.
Empty actor allowed to save but Verify must refuse. Validate prefix
`human:` and the same id charset as root ids (or `[a-z0-9-]+` after the
prefix). Do not default from `git config user.name`.[^config]

**Out of bound:** Queue buttons.

**Tests:** POST loopback sets actor; invalid `process:cursor` rejected;
tokens still absent from HTML.

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`.

**Owner:** `src/config.rs`, `src/http/settings.rs`,
`templates/fragments/settings.html`.

### Phase 4 — Live POST and chrome

**Bound:** `POST /__okmate/review` loopback-only. Form fields:
`action` (`verify` | `promote`), `concept` (id), `hash` (required CAS),
optional `root` when the preview workspace is multi. Handler:

1. Reject non-loopback.
2. Require configured `actor`.
3. Resolve git toplevel from the member bundle path; refuse if none
   or if the member is a git-cache snapshot.
4. Load concept file; compare hash; 409 on mismatch.
5. `verify`: append event with server UTC RFC 3339; leave `status`.
6. `promote`: require existing `human:` verification, `status == draft`,
   `authority != exploratory`; then `set_status(…, "draft", "stable")`.
7. Reload workspace (existing parse cache path); morph
   `#okmate-main` / `#okmate-toc` (Datastar) or full document.

Chrome: on concept meta, when git+actor allow writes, show **Verify**
for `InitialVerification` / `PendingPromotion` / `ReverifySources` /
`ReverifyRegenerated` (human is accepting current text). Show **Promote**
only for `PendingPromotion`. Queue needs-action row: same two buttons
next to the pill, not a replacement for the link. Disabled + title when
actor or git is missing. Static `build` HTML omits the forms (live
preview only).

**Out of bound:** SSE jobs, agent argv, batch verify.

**Tests:** Axum oneshot on a temp git repo with inferred `knowledge/`:
verify appends and queue no longer lists InitialVerification; second
verify with stale hash 409; promote without verify 400; exploratory
promote 400; non-loopback 403; `build` fixture has no
`/__okmate/review`. Datastar-Request returns a patch containing
`id="okmate-main"`.

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`.

**Owner:** `src/http/review.rs`, `src/http/mod.rs`,
`templates/fragments/article.html`, `templates/fragments/queue.html`,
`src/views/*` as needed for `can_author`, `file_hash` on rows.

### Phase 5 — Dogfood on this repository

**Bound:** README or settings copy: pick the git project folder; bundle
is inferred. Run `okmate view` from this repo root (not `knowledge/`)
and confirm the window opens the inferred bundle. Manual check is not
the Exit; add a CLI or unit test that `resolve_bundle` on a clone of
this layout finds `knowledge/`. Optional: `okmate view` with no args
uses cwd’s git toplevel + infer when `./knowledge` is not passed.

**Out of bound:** Changing default `check knowledge` CLI paths.

**Exit:** `cargo test -p okf` and `cargo test -p okmate --no-default-features`
and `okmate check knowledge --profile strict --format terminal` and
`cargo fmt --all -- --check`.

**Owner:** `src/preview.rs` default path, `README.md` view paragraph only
if the invocation changed.

## Validation (every phase)

```sh
cargo fmt --all -- --check
cargo test -p okf
cargo test -p okmate --no-default-features
okmate check knowledge --profile strict --format terminal
```

Report lifecycle and provenance warnings separately from errors.

[^research]: Verify vs Promote inputs; CAS; no human: from agents; CLI not API.
[^decision]: Git-only authoring; one-level okf_version discovery; CLI harness later.
[^review-engine]: InitialVerification vs PendingPromotion vs Exploratory.
[^preview-okf]: Bundle root index predicate.
[^validate]: git toplevel and latest human verification.
[^settings-http]: Loopback morph contract.
[^http]: No review POST today.
[^queue]: Display-only action pills.
[^article]: Display-only alert.
[^skill]: Append verification; agents do not invent human: events.
[^extract]: Queue writes were out of bound for the extract plan.
[^config]: Settings TOML ownership.
