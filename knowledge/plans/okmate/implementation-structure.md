---
type: Implementation Plan
title: Implementation structure
description: Split oversized modules, shrink the okf public surface, type page kinds, and take session I/O off the Datastar GET path—no new product features.
tags: [domain/okmate, domain/okf, concern/architecture, concern/tooling]
status: draft
generated: { by: process:cursor, at: 2026-08-29T11:45:00Z }
stale_after: 2026-11-29
authority: exploratory
owners: [human:nils]
sources:
  - id: audit
    resource: ../../audits/okmate/implementation-structure.md
    title: Okmate implementation-structure audit
    author: process:cursor
    last_modified: 2026-08-29
  - id: overview
    resource: ../../architecture/system-overview.md
    title: OKMate system overview
    author: process:cursor
    last_modified: 2026-08-28
  - id: okf-readme
    resource: ../../../okf/README.md
    title: Portable OKF engine README
    author: process:git
    last_modified: 2026-08-28
  - id: okf-lib
    resource: ../../../okf/src/lib.rs
    title: Engine load pipeline and crate re-exports
    author: process:git
    last_modified: 2026-08-28
  - id: okf-validate
    resource: ../../../okf/src/validate.rs
    title: Metadata, lifecycle, and git provenance
    author: process:git
    last_modified: 2026-08-26
  - id: okf-cargo
    resource: ../../../okf/Cargo.toml
    title: okf crate manifest
    author: process:git
    last_modified: 2026-08-29
  - id: site
    resource: ../../../src/site.rs
    title: Site build, page_for_route, nav forests
    author: process:git
    last_modified: 2026-08-29
  - id: preview
    resource: ../../../src/preview.rs
    title: Session, view prepare, file watch
    author: process:git
    last_modified: 2026-08-29
  - id: workspace
    resource: ../../../src/workspace.rs
    title: Multi-root workspace
    author: process:git
    last_modified: 2026-08-29
  - id: views
    resource: ../../../src/views/mod.rs
    title: Document view-model and Askama templates
    author: process:git
    last_modified: 2026-08-29
  - id: pages
    resource: ../../../src/http/pages.rs
    title: Datastar GET and live_document
    author: process:git
    last_modified: 2026-08-28
  - id: http
    resource: ../../../src/http/mod.rs
    title: AppState and router
    author: process:git
    last_modified: 2026-08-28
  - id: settings
    resource: ../../../src/http/settings.rs
    title: Settings POST
    author: process:git
    last_modified: 2026-08-29
  - id: config
    resource: ../../../src/config.rs
    title: UserConfig and home_dir
    author: process:git
    last_modified: 2026-08-27
  - id: window
    resource: ../../../src/views/window.rs
    title: WindowQuery parse
    author: process:git
    last_modified: 2026-08-27
  - id: resp
    resource: viewer-responsiveness.md
    title: Viewer responsiveness plan
    author: process:cursor
    last_modified: 2026-08-27
  - id: engine-readme
    resource: ../../../okf/README.md
    title: Portable OKF engine boundary
    author: process:git
    last_modified: 2026-08-28
---

# Implementation structure

## Goal

Make the existing two-crate layout match how the code is already
described: a small portable `okf` API, application modules that each own
one concern, and a request path that renders from memory without writing
session files. Behavior, CLI, HTML landmarks, and validation codes stay
the same.[^audit][^overview]

Findings and evidence:
[Okmate implementation structure](/audits/okmate/implementation-structure.md).[^audit]

## Out of bound

- New commands, routes, review actions, MCP, lint/SARIF, or authoring UI.
- Changing `okf::load` / `check` / `inspect` / `search` / `build` results,
  diagnostic codes, or profile semantics.
- Merging roots inside the engine, or changing `/@id/` vs unprefixed
  routes.[^workspace][^resp]
- Replacing Askama, Datastar, or `h35-desktop`.
- Typed frontmatter structs that drop unknown YAML keys.
- Hash-map concept indexes unless a phase’s timings show a need.
- clap enums on `okf` types.
- tokio `RwLock` for `Workspace`.
- Minting an approved Decision.
- Mixing unrelated working-tree changes into phase commits.

## Constraints that do not move

- `okf` stays UI-neutral. HTML, HTTP, session paths, and cache directories
  stay in this crate.[^engine-readme][^overview]
- Workspace remains a list of per-root `okf::load` results.[^workspace]
- Datastar GET patches main and toc and does not replace `#okmate-nav`.[^resp]
- Landmarks stay `#okmate-*`. Tokens never appear in HTML or roots JSON.
- `okmate build` still writes the full static tree; live `view` still
  writes `pages.json` plus assets, not every `index.html`.[^site][^resp]
- Loopback-only settings and prefs POST.
- Existing tests that pin HTML landmarks, nav hrefs, session sanitizers,
  and engine diagnostics stay green without rewriting their assertions
  except where a phase explicitly moves a type.

## Target contract

After this plan, a contributor can name the file for: engine load, engine
schema, engine git provenance, application session, application nav,
application HTTP. `okf`’s rustdoc/public items are the operations in
`okf/README.md` plus AST types and the helpers this crate already needs
(`string_field`, `classify_concept_action`, `ParseCache`, `LoadOptions`,
graph/preview path helpers). Session persist is not inside page render.

## Phases

### Phase 1 — Engine public surface

**Bound:** Reduce `okf/src/lib.rs` `pub use` to the portable operations
and types. Move the rest to `pub(crate)` (or module-private). Either
introduce a small `thiserror` error type for I/O and usage failures
(`not a directory`, missing inspect id) **or** drop `thiserror` from
`okf/Cargo.toml` and the README dependency list so they match the code.
Do not change diagnostic structs or `anyhow` at the okmate CLI boundary
in this phase unless the error type maps 1:1. Update `okf/README.md` to
name the actual public surface.[^okf-lib][^okf-cargo][^okf-readme][^audit]

**Out of bound:** Splitting `validate.rs` or `lib.rs` parse functions
(Phase 2). Changing check/inspect JSON.

**Tests:** `cargo test -p okf`. Grep or a unit test that
`days_from_civil` / `lines_with_offsets` are not reachable as
`okf::days_from_civil` if they were public only for this crate. Existing
engine tests unedited except imports if a helper moved.

**Exit:** `cargo test -p okf`, `cargo test -p okmate --no-default-features`,
`cargo fmt --all -- --check`.

**Owner:** `okf/src/lib.rs`, `okf/Cargo.toml`, `okf/README.md`, any okmate
imports that used a now-private helper (switch to a remaining public
accessor or a crate-local copy only if the helper is not part of the
engine contract).

### Phase 2 — Split engine load and provenance

**Bound:** Move discover/parse/index/log from `okf/src/lib.rs` into
`okf/src/load.rs` (name flexible). Move git subprocess helpers and
`GitModification` out of `validate.rs` into `okf/src/provenance.rs` (or
`git.rs`). `validate.rs` keeps metadata, uniqueness, index membership,
and lifecycle *rules*; it calls provenance. `lib.rs` keeps `load`,
`load_timed`, `load_with_cache`, `check`, `inspect`, `search`, `build`.
No behavior change.[^okf-lib][^okf-validate][^audit]

**Out of bound:** Public API shrink (Phase 1). Application modules.

**Tests:** `cargo test -p okf` (including provenance and parallel parse).
No new diagnostics.

**Exit:** those tests and `cargo fmt --all -- --check`.

**Owner:** `okf/src/lib.rs`, new modules, `okf/src/validate.rs`.

### Phase 3 — Session module

**Bound:** Move `Session`, `NavMode`, sanitizers, `load_session*`,
`persist_*`, `session_path`, `state_dir` from `preview.rs` into
`src/session.rs` (or `src/state.rs`). `preview.rs` keeps `ViewOptions`,
`prepare`, watch, bind. One `home_dir` used by `config.rs` and session
(private in `config` and re-exported, or a tiny `src/paths.rs` with
`home_dir` / `cache_dir` / `state_dir` / `config_path` only—no new
config format). `write_session` returns `Result` and callers log on
failure instead of `let _ =`. Workspace lock reads use
`unwrap_or_else(PoisonError::into_inner)` (or equivalent) instead of
`expect` that panics the server.[^preview][^config][^http][^audit]

**Out of bound:** Changing sanitizer rules or session JSON keys. Moving
watch rebuild off `notify`. tokio `RwLock`.

**Tests:** Existing `preview.rs` session tests move with the type and
stay unedited in assertion. `cargo test -p okmate --no-default-features`.

**Exit:** those tests and `cargo fmt --all -- --check`.

**Owner:** `src/preview.rs`, new session/paths module, `src/config.rs`,
`src/http/mod.rs`.

### Phase 4 — One nav forest; site modules

**Bound:** Extract nav tree construction so separated and merged share
the path-tree walk; merge-only behavior stays `@root` on leaves,
canonical collection href, merged summaries. Extract HTML helpers
(`wrap_article_tables`, and any remaining scrapers) to `src/site/` or
`src/nav.rs` + `src/html_util.rs`. Collection hover/summary text prefers
existing `Index.headings` / a first body excerpt already on the model if
one exists; do not add a markdown field. Remove the unused `nav_mode`
parameter from `build_workspace_nav` or use it if live preview still
needs it for `pages.json` (it should not). `site::build` loads the bundle
once: `okf::load` then `okf::build_artifacts`, or `okf::build` returns
enough to skip the second load—**without** changing artifact bytes.[^site][^audit]

**Out of bound:** Changing merged vs separated hrefs or Overview rows.
Rewriting `workspace::rewrite_hrefs` to a DOM parser. New nav UI.

**Tests:** `tests/workspace.rs` nav href and merged collection tests
unedited. Golden HTML if present. `cargo test -p okmate --no-default-features`.

**Exit:** those tests and `cargo fmt --all -- --check`.

**Owner:** `src/site.rs`, `src/workspace.rs` only if href helper moves
with nav, `tests/workspace.rs`.

### Phase 5 — `PageKind` and `Document` construction

**Bound:** Replace `page_kind: String` with an enum (`Home`, `Review`,
`Log`, `Settings`, `Page`) that templates can still match (Askama
`Display` or a method `as_str` used in templates). Add
`Document::for_settings_host` (or `Default` plus kind) so settings.rs
does not list thirty fields. Point `WindowQuery` at the same serde
fields as `WindowParams` (one parse). Do not split `Document` into five
structs if that forces five morph payloads; one struct with a kind is
enough.[^views][^settings][^window][^pages][^audit]

**Out of bound:** New template files, new landmarks, dropping the
`document_template!` macro unless the enum change makes a smaller macro
obvious.

**Tests:** Template landmark tests in `src/views/mod.rs` unedited except
kind construction. `cargo test -p okmate --no-default-features`.

**Exit:** those tests and `cargo fmt --all -- --check`.

**Owner:** `src/views/`, `templates/**`, `src/http/pages.rs`,
`src/http/settings.rs`, `src/site.rs`.

### Phase 6 — Request-path I/O

**Bound:** `live_document` (or equivalent) **only reads** workspace and
session. Persisting `open_path` happens in the HTTP handler after a
successful render, and the write is `spawn_blocking` or a dedicated
non-request task—not `std::fs` on the Axum worker. Settings POST builds
the fragment from `state.workspace` (and empty workspace for
settings-host), not `okf::load(&state.root)`. Keep loopback checks.
`okmate timings --scenario click` still shows `reload` at zero on a warm
workspace.[^pages][^settings][^preview][^resp][^audit]

**Out of bound:** Changing what gets persisted (path, hash, scroll). New
prefs fields. Watch-loop rewrite.

**Tests:** Existing navigation tests: Datastar GET still patches
main+toc. A test that a GET does not require a writable session path
(or that persist failure does not 500 the page). Settings fragment tests
still round-trip add/remove. `cargo test -p okmate --no-default-features`.

**Exit:** those tests and `cargo fmt --all -- --check`.

**Owner:** `src/http/pages.rs`, `src/http/settings.rs`, session persist
helpers, `tests/navigation.rs` / `tests/settings.rs` as needed.

## Order

Phase 1 before 2 (surface then split). Phase 3 before 6 (session API
then request path). Phase 4 and 5 are independent of 1–3 after 3’s
imports settle; 5 may touch `site.rs` so prefer 4 before 5. Phase 6
last.

[^audit]: Findings F1–F6 against current files.
[^overview]: Crate and settings ownership.
[^okf-readme]: Documented engine dependencies and scope.
[^okf-lib]: Current re-exports and load body.
[^okf-validate]: Git mixed with schema.
[^okf-cargo]: Unused `thiserror`.
[^site]: Double load, duplicate forests, unused nav_mode.
[^preview]: Session mixed with view server.
[^workspace]: Per-root workspace contract.
[^views]: String `page_kind` and shared `Document`.
[^pages]: Persist inside render.
[^http]: Lock `expect`.
[^settings]: POST reload from `state.root`.
[^config]: Duplicate `home_dir`.
[^window]: Second query parser.
[^resp]: In-memory clicks; preview write policy.
[^engine-readme]: UI-neutral engine boundary.
