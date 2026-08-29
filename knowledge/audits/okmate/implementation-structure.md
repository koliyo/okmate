---
type: Audit
title: Okmate implementation structure
description: Crate boundaries are sound; the cost is oversized modules, a leaky okf public surface, a god view-model, duplicated nav construction, and request-path I/O on the async runtime.
tags: [domain/okmate, domain/okf, concern/architecture, concern/tooling]
status: draft
generated: { by: process:cursor, at: 2026-08-29T11:45:00Z }
stale_after: 2026-11-29
authority: descriptive
owners: [human:nils]
sources:
  - id: overview
    resource: ../../architecture/system-overview.md
    title: OKMate system overview
    author: process:cursor
    last_modified: 2026-08-28
  - id: readme
    resource: ../../../README.md
    title: OKMate README
    author: process:git
    last_modified: 2026-08-28
  - id: okf-readme
    resource: ../../../okf/README.md
    title: Portable OKF engine README
    author: process:git
    last_modified: 2026-08-28
  - id: cargo
    resource: ../../../Cargo.toml
    title: Workspace and okmate crate manifest
    author: process:git
    last_modified: 2026-08-29
  - id: okf-cargo
    resource: ../../../okf/Cargo.toml
    title: okf crate manifest
    author: process:git
    last_modified: 2026-08-29
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
  - id: okf-ast
    resource: ../../../okf/src/ast.rs
    title: Bundle, Concept, LoadOptions
    author: process:git
    last_modified: 2026-08-26
  - id: okf-md
    resource: ../../../okf/src/markdown.rs
    title: Comrak body parse and article HTML
    author: process:git
    last_modified: 2026-08-28
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
    title: AppState, router, lock usage
    author: process:git
    last_modified: 2026-08-28
  - id: settings
    resource: ../../../src/http/settings.rs
    title: Settings POST and empty Document
    author: process:git
    last_modified: 2026-08-29
  - id: lib
    resource: ../../../src/lib.rs
    title: Application crate root and clap wrappers
    author: process:git
    last_modified: 2026-08-29
  - id: cli
    resource: ../../../src/cli.rs
    title: clap CLI
    author: process:git
    last_modified: 2026-08-27
  - id: config
    resource: ../../../src/config.rs
    title: UserConfig parse and save
    author: process:git
    last_modified: 2026-08-27
  - id: window
    resource: ../../../src/views/window.rs
    title: Hand-rolled query parse
    author: process:git
    last_modified: 2026-08-27
  - id: resp
    resource: ../../plans/okmate/viewer-responsiveness.md
    title: Viewer responsiveness plan
    author: process:cursor
    last_modified: 2026-08-27
  - id: plan
    resource: ../../plans/okmate/implementation-structure.md
    title: Implementation-structure plan
    author: process:cursor
    last_modified: 2026-08-29
---

# Okmate implementation structure

## Verdict

The **crate diagram is sound**. `okf` is a UI-neutral engine; this crate owns
CLI, Askama, Axum, Datastar, and an optional desktop host; `knowledge/` is
inert; `okmate-ops` is operator tooling. That split matches the published
contract and is the right one to keep.[^overview][^readme][^okf-readme]

What has drifted is **internal shape**, not the product boundary. After the
extract from Rocci and the responsiveness work, a few files absorbed session,
nav, load, git, and HTML string surgery. The code is readable and tested. It
is not yet idiomatic as a two-crate library-plus-app: `okf` exports more than
a portable engine should, `thiserror` is declared and unused, and the
application mixes view-model, persistence, and HTTP on the request path.

Remediation is [implementation-structure](/plans/okmate/implementation-structure.md).
No new commands, pages, or engine checks.[^plan]

## What already holds

- **Owning layers.** Parse, graph, search, artifacts, and profiles live in
  `okf/`. HTML, HTTP, desktop, config, and session live in this crate. Desktop
  is a feature (`h35-desktop`). CLI-only tests use `--no-default-features`.[^cargo][^okf-readme][^overview]
- **Workspace as a list of loads.** Multi-root is several `okf::load` results
  plus href rewriting, not a merged engine catalog. Single-root routes stay
  unprefixed. That is the intended contract.[^workspace][^resp]
- **Load policy.** `LoadOptions` splits profile from provenance. Preview
  defaults provenance off. `ParseCache` is engine-owned; cache directory
  choice is okmate-owned.[^okf-lib][^okf-ast][^workspace]
- **Safety habits.** Settings and prefs POST are loopback-only. Git tokens
  redact in `Debug`. Config and session use temp-file rename. Open-path and
  CSS-track sanitizers reject `..` and `/__okmate`.[^settings][^config][^preview][^http]
- **Tests sit on the owning boundary.** Engine tests in `okf`; CLI and HTTP
  in `tests/` and crate unit tests. clap port defaults and session
  round-trips are covered.[^cli][^preview]
- **Hypermedia stack.** Askama templates, Datastar morph for main/toc, static
  `okmate build` via `write_html_pages`, live preview writing only
  `pages.json` plus assets. That split is already the responsiveness
  contract.[^site][^resp]

## Findings

### F1 — `okf` public surface is a kitchen sink

`okf/src/lib.rs` re-exports parse helpers (`split_frontmatter`,
`lines_with_offsets`, `location`), validation internals
(`collect_source_ids`, `days_from_civil`, `git_last_modified`,
`git_path_dirty`, `filesystem_modified_at`, `current_utc_date`), and
markdown utilities. A portable engine crate should export `load` /
`check` / `inspect` / `search` / `build` / `benchmark`, the AST types, and
the few helpers callers need (`string_field`, `published_href`). The rest
belongs in `pub(crate)` or a focused module.[^okf-lib]

`thiserror` is in `okf/Cargo.toml` and named in `okf/README.md`. Nothing
in `okf/src` uses it. Engine errors are `anyhow`. For a library crate that
is the wrong default: callers cannot match on load failure, and the
declared dependency is dead.[^okf-cargo][^okf-readme][^okf-lib]

`Concept.metadata` as `BTreeMap<String, Value>` is the right call (unknown
keys must round-trip). That is not a finding. The finding is exporting git
and civil-date helpers as if they were the format API.

### F2 — Engine load and validation are two oversized files

`okf/src/lib.rs` is 663 lines: timed load, parallel parse, concept/index/log
parsers, directory discover, and concept lookup. `okf/src/validate.rs` is
1022 lines: frontmatter schema, index membership, lifecycle, and git
subprocess provenance (`Command::new("git")`). Schema rules and git
I/O are different reasons to change.[^okf-lib][^okf-validate]

Parallel parse via `thread::scope` and a mutex queue is a reasonable
stdlib choice (no rayon). Concept lookup is linear `find` plus stem
disambiguation; that is fine at current bundle size. The issue is file
cohesion, not the algorithms.

### F3 — `preview.rs` owns session, prefs, and the view server

954 lines mix `Session` / `NavMode`, CSS and path sanitizers, session
JSON I/O, `ViewOptions`, bind/prepare, settings-host fallback, and
`notify` watch rebuild. Session is durable state under `OKMATE_STATE`.
View prepare is process lifetime. They share a file because both run at
`view` start.[^preview]

`home_dir` is duplicated with `config.rs`. `write_session` swallows I/O
errors (`let _ = fs::rename`). Watch and HTTP use
`RwLock::expect("workspace lock")`, so a poisoned lock aborts the
process.[^preview][^config][^http]

### F4 — `site.rs` is a 1192-line nav and HTML monolith

`page_for_route_nav`, breadcrumbs, two nav forests (separated and merged),
collection merge, table wrapping, and ad-hoc HTML plaintext all live in
one module. `nav_forest` and `nav_forest_merged` repeat the same
path-tree construction (indexes → owner assignment → parent links →
`take_node` / `take_merged` → finalize). The merge variant adds `@root`
labels; the algorithm is the same.[^site]

Article HTML is then scraped with `find("<h1")`, `find("<p")`, and
`href="` walks (`wrap_article_tables`, `first_prose_paragraph`,
`rewrite_hrefs` in `workspace.rs`). Headings and links already exist on
`Concept` / `Index` from comrak. Collection summaries should use those,
not a second HTML parser. Href rewrite for multi-root prefixes is a
real application concern; the string walk is the fragile part.[^site][^workspace][^okf-md]

`site::build` calls `okf::build` then `okf::load` on the same root: two
full loads for one `okmate build`.[^site]

`build_workspace_nav` takes `nav_mode` and ignores it (`_nav_mode`). Live
preview no longer writes the HTML tree; the unused parameter is leftover
from that change.[^site][^resp]

### F5 — `Document` is a god struct; `page_kind` is a string

Every Askama template is generated from the same ~30-field struct via
`document_template!`. Home, review, log, settings, and page share
`review_rows`, `settings_roots`, `log_days`, reading prefs, and windows.
`page_kind` is compared as `"home" | "review" | "log" | "settings" |
"page"` in Rust and templates. Settings-without-bundle constructs the
struct by listing every field. That is the tax of one morph target, but
the type does not say which fields a kind may use.[^views][^settings][^pages]

`WindowQuery::from_raw` hand-parses `start` / `filter` / `q` and percent-decodes.
`review_window` already extracts `WindowParams` with serde. Two parsers
for the same query.[^window][^pages]

### F6 — Request path has side effects and extra loads

`live_document` reads the workspace, builds the page, **and writes
`session.json`** (`persist_open_path_to`) on every successful GET,
including Datastar clicks. That is blocking `std::fs` on the Axum worker.
The write is also a semantic mix: rendering a route should not be the
session store.[^pages][^preview]

Settings POST reloads with `okf::load(&state.root, …)` instead of the
in-memory workspace. `AppState.root` for the settings-only host is
`PathBuf::from("/")`. A POST after opening settings-without-bundle can
hit a useless load. The in-memory workspace is already on `AppState`.[^settings][^preview]

No `tokio::fs` or `spawn_blocking` anywhere. Watch rebuild already runs
on a spawned task (good). Session persist and settings fragment render
do not.[^pages][^preview]

## Non-findings

These look like issues and are not, or are already decided:

- **Untyped frontmatter** — lossless custom keys are the format. Typed
  accessors (`string_field`) are the right layer.[^okf-ast]
- **Clap `ProfileArg` wrappers** — clap `ValueEnum` on a crate-local enum
  mapping into `okf::Profile` is idiomatic. Do not force clap onto
  engine types.[^lib][^cli]
- **`std::sync::RwLock` for `Workspace`** — the lock is shared with a
  `notify` thread and Axum. `std` is correct; tokio's `RwLock` would
  still need blocking load inside. Poison handling is F3, not the lock
  kind.[^http][^resp]
- **Linear concept search in `page_for_route`** — bundle sizes in this
  repo are tens to low hundreds of records. A map can wait until a
  measurement says so.

## Rank

| ID | Cost if ignored | Fix shape |
| --- | --- | --- |
| F1 | Engine consumers (and this crate) couple to internals; dead `thiserror` | Shrink `pub use`; typed errors or drop the dep |
| F2 | Every schema or git change edits a 1k-line file | Split load vs parse vs provenance |
| F4 | Nav bugs get fixed twice; HTML scrapers drift from comrak | One forest builder; use parsed headings |
| F3 | Session tests and view tests share a 950-line module | `session` module |
| F6 | Click path does disk I/O; settings can load `/` | Persist after render; use workspace |
| F5 | New chrome fields touch every template | `PageKind` enum; `Document::default` |

[^overview]: Engine versus application versus knowledge ownership.
[^readme]: CLI, settings paths, desktop feature.
[^okf-readme]: UI-neutral engine; lists `thiserror` and `anyhow`.
[^cargo]: Workspace members `okf`; optional `h35-desktop`.
[^okf-cargo]: `thiserror = "2.0"` with no crate use.
[^okf-lib]: Re-exports and load/parse in `lib.rs`.
[^okf-validate]: Schema plus git provenance in one module.
[^okf-ast]: `Concept.metadata` as JSON map; `LoadOptions`.
[^okf-md]: Comrak already produces headings and links.
[^site]: `build` double-load; duplicated forests; HTML scrapers.
[^preview]: Session plus view server; swallowed session writes; watch lock.
[^workspace]: Per-root loads and href rewrite.
[^views]: Shared `Document` and `page_kind: String`.
[^pages]: `live_document` persists open path; Datastar GET.
[^http]: `RwLock::expect`; loopback router.
[^settings]: Empty `Document` literal; POST `okf::load`.
[^lib]: Thin `okf` wrappers and clap enums.
[^cli]: Subcommands and `ValueEnum` defaults.
[^config]: Atomic TOML save; `home_dir`.
[^window]: Manual query string parse.
[^resp]: Clicks from memory; preview does not write the HTML tree.
[^plan]: Refactor-only phases; no product features.
