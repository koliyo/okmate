---
type: Implementation Plan
title: Viewer responsiveness
description: Ship an okmate timings pipeline, then make view clicks read memory, keep preview load off the check path, stop writing the full live HTML tree, and window review and log with invisible overscan preload.
tags: [domain/okmate, concern/performance, concern/rendering, concern/architecture]
status: draft
generated: { by: process:cursor, at: 2026-08-27T11:50:00Z }
stale_after: 2026-11-27
authority: exploratory
owners: [human:nils]
sources:
  - id: research
    resource: ../../research/okmate/viewer-responsiveness.md
    title: Viewer click-path latency and large chrome pages
    author: process:cursor
    last_modified: 2026-08-27
  - id: pages
    resource: ../../../src/http/pages.rs
    title: Datastar GET reloads the workspace
    author: process:git
    last_modified: 2026-08-27
  - id: http
    resource: ../../../src/http/mod.rs
    title: AppState and ServeDir fallback
    author: process:git
    last_modified: 2026-08-27
  - id: workspace
    resource: ../../../src/workspace.rs
    title: Workspace reload and multi-root load
    author: process:git
    last_modified: 2026-08-27
  - id: preview
    resource: ../../../src/preview.rs
    title: View prepare, watch rebuild, session
    author: process:git
    last_modified: 2026-08-27
  - id: site
    resource: ../../../src/site.rs
    title: write_html_pages and page_for_route_nav
    author: process:git
    last_modified: 2026-08-27
  - id: cli
    resource: ../../../src/cli.rs
    title: view profile default and retrieval benchmark command
    author: process:git
    last_modified: 2026-08-27
  - id: readme
    resource: ../../../README.md
    title: Published okmate CLI table
    author: process:git
    last_modified: 2026-08-27
  - id: ast
    resource: ../../../okf/src/ast.rs
    title: LoadOptions profile and provenance
    author: process:git
    last_modified: 2026-08-26
  - id: okf-load
    resource: ../../../okf/src/lib.rs
    title: load_with_cache and LoadTimings
    author: process:git
    last_modified: 2026-08-26
  - id: parse-cache
    resource: ../../../okf/src/parse_cache.rs
    title: ParseCache memory and directory persistence
    author: process:git
    last_modified: 2026-08-26
  - id: queue
    resource: ../../../templates/fragments/queue.html
    title: Full review table
    author: process:git
    last_modified: 2026-08-27
  - id: log-tpl
    resource: ../../../templates/fragments/log.html
    title: Full log list
    author: process:git
    last_modified: 2026-08-27
  - id: review-js
    resource: ../../../assets/review.js
    title: Client-side review filter
    author: process:git
    last_modified: 2026-08-26
  - id: nav-js
    resource: ../../../assets/nav.js
    title: Keep-nav Datastar patch
    author: process:git
    last_modified: 2026-08-27
  - id: load-plan
    resource: ../okf/okf-load-performance.md
    title: OKF load-performance improvements
    author: process:cursor
    last_modified: 2026-08-26
  - id: multi-plan
    resource: extended-multi-bundle.md
    title: Extended multi-bundle viewer
    author: process:cursor
    last_modified: 2026-08-27
  - id: shell-plan
    resource: viewer-shell-parity.md
    title: Viewer shell parity
    author: process:cursor
    last_modified: 2026-08-26
  - id: overview
    resource: ../../architecture/system-overview.md
    title: OKMate system overview
    author: process:cursor
    last_modified: 2026-08-27
  - id: engine-readme
    resource: ../../../okf/README.md
    title: Portable OKF engine boundary
    author: process:git
    last_modified: 2026-08-26
  - id: cargo
    resource: ../../../Cargo.toml
    title: Dev-profile package opt-level
    author: process:git
    last_modified: 2026-08-27
---

# Viewer responsiveness

## Goal

Make `okmate view` feel instant: a sidebar click to a small document
returns from memory in tens of milliseconds, and `/review/` plus `/log/`
first-paint a viewport that scrolls continuously while nearby segments
preload. Ship a durable `okmate timings` pipeline first so this work and
later investigations measure the same spans instead of throwaway
benches. Responsiveness is a core product goal, not a polish pass.[^research][^overview][^cli]

Evidence and machine-local timings are in
[Viewer click-path latency and large chrome pages](/research/okmate/viewer-responsiveness.md).
Those numbers are not a latency SLA.[^research]

## Out of bound

- Teaching `okf::load` / `check` / `inspect` / `search` / `build` to load
  a partial catalog or a multi-root registry. Nav, review, and goto still
  need the full in-memory workspace after one session load.[^load-plan][^multi-plan][^engine-readme]
- Changing `okmate check --profile strict` provenance or CI diagnostics.
- Redesigning action classification, approve/write, or review columns.
- Merged-versus-separated nav semantics, Cmd-K ranking, or collection
  hover copy.[^multi-plan]
- Replacing Datastar with a different morph library, or replacing
  `#okmate-nav` on ordinary document clicks.[^shell-plan][^nav-js]
- Page-number pagination UI, infinite “Load more” buttons, or virtual
  scrolling libraries as a required dependency.
- A CI latency gate, criterion harness in `okf`, or treating machine-local
  timings as a portability contract.[^load-plan][^okf-load]
- Replacing `okmate benchmark` (retrieval quality) with a latency tool.
  The new command is a sibling, not a rewrite.[^cli][^readme]
- Authoring UI, writable roots, or minting an approved Decision.
- Mixing unrelated working-tree changes into phase commits.

## Constraints that do not move

- `okf` stays UI-neutral. Preview policy (when to call `load`, whether
  provenance runs, where `ParseCache` lives) is okmate’s.[^engine-readme][^okf-load]
- Workspace remains a list of per-root `okf::load` results in okmate, not
  a merged engine catalog.[^multi-plan]
- Ordinary Datastar GET patches main and toc and does **not** replace
  `#okmate-nav`.[^shell-plan][^nav-js]
- Landmarks stay `#okmate-*`. HTML/CSS/JS stay in this crate.[^overview]
- Tokens never appear in HTML, fragments, or window JSON.
- `okmate view <path>` stays that one bundle. `okmate build` still writes
  a full static tree with `/{id}/` routes.[^site][^multi-plan]
- Durable membership stays `config.toml`. Session stays server-owned.
- Timing JSON and `Server-Timing` never include tokens, secrets, or
  resolved credentials (same rule as `okmate roots --format json`).[^readme]

## Target contract

### Measurement pipeline

`okmate timings [path]` is the supported investigation surface. No path
uses the same workspace resolution as `view` (configured roots when two
or more are enabled). An explicit path stays one bundle. `--format
terminal|json` (default terminal). Optional `--scenario
load|site|click|review|log|watch|all` (default `all`). `--profile` and
`--provenance` match the preview policy of the revision under test.[^cli][^readme][^research]

The JSON object is versioned (`timings_version`), machine-local, and
lists named spans a later research record can cite without a `/tmp`
crate:

| Span group | What it records |
| --- | --- |
| `roots[]` | Per-root `okf::load_timed` (`discover`, `parse`, `graph`, `provenance`, cache hits/misses, concept/log/diagnostic counts) |
| `workspace` | `load_members` / reload wall, member count, concept total |
| `site` | Preview write wall, file count, byte total, largest path (omit after live preview stops writing every page) |
| `pages[]` | Route, kind, `page_for_route` ms, fragment render ms, fragment bytes, article bytes, review row count, log entry count |
| `click` | Current Datastar request path for a small concept (includes `reload` until Phase 2 removes it) |

`click` / `review` / `log` pick deterministic routes from the loaded
workspace (first leaf concept; `/review/`; `/log/`). They do not start
a browser. Live `view` also emits `Server-Timing` on Datastar GET
(`reload`, `render`, `bytes`) so a running window can be curled.[^pages][^okf-load]

`okmate benchmark` stays retrieval hit-rate. `okf` still returns ordinary
`Duration`s; snapshot types stay in okmate.[^cli][^engine-readme]

### Click path

A Datastar document GET reads the in-memory workspace. It does not call
`okf::load` or `Workspace::reload`. Disk edits become visible after the
existing watch debounce updates that same shared workspace, or after a
full process restart.[^pages][^workspace][^preview]

`AppState` must not deep-clone `Bundle` per request. Share
`Arc<RwLock<Workspace>>` (or equivalent) with the watch task.[^http]

### Preview load versus check

`view` keeps Strict schema rules and defaults **git provenance off**,
matching the earlier engine preview-versus-check split. `check --profile
strict` is unchanged. An explicit `--provenance` flag turns preview git
checks back on.[^cli][^ast][^load-plan]

Watch (and optional process-restart) reuse `ParseCache` under
`OKMATE_CACHE`, versioned as the engine already supports.[^parse-cache][^okf-load]

### Live preview output

Live `view` writes assets (`/__okmate/*`, `pages.json`) and serves
document HTML from memory. It does not write one `index.html` per
concept before bind or on every save. `okmate build` still does.[^site][^preview]

### Review and log windows

`/review/` and `/log/` first paint a viewport-sized window plus one
segment of overscan above and below. Scroll stays one list: no page
numbers, no “next page” control. Approaching a window edge fetches the
adjacent segment and morphs it in; far segments leave the DOM.
Scrollbar position reflects the full filtered length (spacer or
equivalent estimated height).[^queue][^log-tpl][^research]

Filter and search on review are **server-side** over the already-loaded
rows. Client hide-all-rows is incompatible with windowing.[^review-js]

Home keeps the existing five-entry log cap. Action-required review
rows stay complete on first paint when that set is small; the “all
concepts” table is the windowed surface.

## Success targets (local, not a contract)

After Phase 1, `okmate timings --format json --scenario all` on a tiny
fixture prints the named span groups above; a later investigation can
re-run it on real roots without writing a new harness.[^cli]

On this machine, after Phase 2, a debug Datastar GET for a small
concept must not invoke `okf::load`. Server time for that GET should
sit in the tens of milliseconds, not seconds. Prove it with `okmate
timings --scenario click`.[^research]

After Phase 3, debug first-open parse of a cached unchanged bundle
should look like today’s `ParseCache` hit path (milliseconds of parse,
not multi-second reparse).[^parse-cache]

After Phase 5, the first `/review/` and `/log/` fragments must not
contain every row or bullet of a three-root fixture the size of the
measured workspace (151 concepts / 210 log entries).[^research]

## Phases

### Phase 1 — Measurement pipeline

**Bound:** Add `okmate timings [path]` with `--format terminal|json` and
`--scenario load|site|click|review|log|watch|all`. Resolve roots the
same way `view` does. Map `okf::LoadTimings` into an okmate JSON
snapshot (`timings_version: 1`) plus the `workspace`, `site`, `pages`,
and `click` groups in the contract table. Datastar GET (and the
timings click scenario) attach `Server-Timing` for `reload`, `render`,
and `bytes`. Document the command on the README CLI table. A tiny
in-crate fixture test asserts the JSON has those keys and that
`roots[].timings.parse` is present. `watch` scenario is a second
`load_members` (or `load_with_cache` once Phase 3 exists) on the same
fixture, not a live notify loop.[^cli][^readme][^okf-load][^pages][^engine-readme]

**Out of bound:** Changing request-path reload, provenance defaults,
`write_html_pages`, or review/log windowing. No CI job that fails on
milliseconds. No new types in `okf`.

**Tests:** `okmate timings` on a temp single-root fixture exits 0;
`--format json` parses as an object with `timings_version`, `roots`,
`workspace`, and `pages`; `--scenario click` includes a small-document
route and fragment byte count. `okmate benchmark` still runs a
retrieval TOML unchanged. `cargo test -p okmate --no-default-features`.

**Exit:** those tests, README row present, and
`cargo fmt --all -- --check`.

**Owner:** `src/cli.rs`, new `src/timings.rs` (or equivalent),
`src/http/pages.rs` for `Server-Timing`, `README.md`, a timings test.

### Phase 2 — Clicks read memory

**Bound:** Datastar GET and full-page live routes render from
`AppState`’s workspace. `Workspace::reload` is not on the request path.
`AppState.workspace` is `Arc<RwLock<Workspace>>` (or equivalent) shared
with `watch_rebuild`, which swaps the inner value after a successful
reload. `okmate timings --scenario click` must show `reload` at zero
(or omitted) on a warm workspace. Ordinary clicks still omit
`#okmate-nav`.[^pages][^http][^preview][^nav-js]

**Out of bound:** Provenance policy, `ParseCache`, dropping
`write_html_pages`, review/log windowing. Expanding the timings schema
beyond click-span names.

**Tests:** Build a tiny fixture, Datastar-GET a concept, change that
Markdown on disk **without** a watch tick, Datastar-GET again: body
still has the old title. After a programmatic workspace swap (the watch
path), the next GET shows the new title. Existing “fragment has main+toc
and not nav” test still passes.

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`.

**Owner:** `src/http/`, `src/preview.rs`, `src/workspace.rs`,
`tests/navigation.rs`.

### Phase 3 — Preview load policy and parse reuse

**Bound:** `view` uses `LoadOptions` with Strict schema and provenance
off by default; `--provenance` restores git checks. `check` is
untouched. Watch (and first open) call `load_with_cache`. Persist
`ParseCache` under `OKMATE_CACHE` with the engine version stamp.
Raise dev-profile `opt-level` for remaining Markdown/YAML parse crates
that a debug load still spends time in (`yaml-rust` and any comrak
Unicode helpers that show up). `okmate timings --scenario load` and
`watch` report cache hits on a second pass.[^cli][^ast][^okf-load][^parse-cache][^cargo][^load-plan]

**Out of bound:** Request-path reload (Phase 2). Static site write
policy. Windowing.

**Tests:** Unit or CLI-level assertion that default view `LoadOptions`
has `provenance == false` and `profile == Strict`. Watch-style second
`load_with_cache` on an unchanged fixture reports `parse_cache_misses == 0`.
`okmate check knowledge --profile strict` still runs provenance (existing
OKF4006/4007/4008 behavior).

**Exit:** `cargo test -p okf`, `cargo test -p okmate --no-default-features`,
and `cargo fmt --all -- --check`.

**Owner:** `src/preview.rs`, `src/workspace.rs`, `src/cli.rs`,
`Cargo.toml`, README preview note.

### Phase 4 — Live preview does not write every page

**Bound:** `view` writes `/__okmate/*` and `pages.json` only (plus a
settings host when there is no bundle). Document and chrome routes
render on demand from the shared workspace — Datastar fragment or full
page. `write_html_pages` remains the `okmate build` path. Watch no
longer rewrites the concept HTML tree.[^site][^preview][^http]

**Out of bound:** Windowing. Changing `build` output shape. Rewriting
nav templates. The `site` span may become empty or note “assets only”;
do not invent a second write path just to keep the old number.

**Tests:** `okmate build` of a fixture still writes
`hello/index.html`. A view-output directory after `build_workspace` /
the new preview writer has assets and `pages.json` but not one HTML file
per concept. HTTP GET `/hello/` and Datastar GET `/hello/` still return
the concept body.

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`.

**Owner:** `src/site.rs`, `src/preview.rs`, `src/http/`, `tests/build.rs`.

### Phase 5 — Windowed review and log

**Bound:** `/review/` all-concepts table and `/log/` list first-paint a
bounded window (on the order of 40 rows / 20 bullets, plus one overscan
segment). A small JS observer (or Datastar scroll handler) requests the
adjacent segment when the sentinel approaches the viewport and patches
it in; segments outside overscan are removed. Spacer (or equivalent)
keeps the scrollbar honest. No page-number UI. Review filter/search
become query or Datastar payload on the window endpoint; `review.js`
must not assume every row is in the DOM. Action table and home log cap
stay eager when small. Window endpoints are loopback preview routes
under `/__okmate/` or the same Datastar GET with explicit window
params.[^queue][^log-tpl][^review-js][^research]

**Out of bound:** Virtualizing ordinary concept articles. Changing
`classify_concept_action`. Redesigning review columns. Shipping a
third-party virtual-list package.

**Tests:** A fixture with more concepts than the window size: first
`/review/` fragment contains the first window and a sentinel, and does
not contain the last concept title. Adjacent-window request returns the
next titles. Filter `draft` window request omits a stable-only fixture
row. Log fixture with many days: first `/log/` fragment omits the oldest
day’s last bullet. `cargo test -p okmate --no-default-features`.

**Exit:** those tests plus `cargo fmt --all -- --check`.

**Owner:** `src/http/pages.rs`, `src/views/`,
`templates/fragments/queue.html`, `templates/fragments/log.html`,
`assets/review.js`, optional `assets/log.js`.

### Phase 6 — Optional chrome and large articles

**Bound:** Only if Phase 5 is in and `okmate timings` still shows a hitch.
(1) Concept articles above a documented HTML-size or heading-count
threshold use the same invisible segment preload (by heading). (2)
Sidebar markup includes `nav_items` once; CSS shows the mobile or
desktop chrome. (3) Any leftover debug parse crates from a fresh
profile. Skip any item that the re-measure does not justify.[^site][^research]

**Out of bound:** Starting this phase to “be complete.” Changing engine
Markdown parse.

**Exit:** Same test commands as Phase 5 for whatever item shipped;
update the research record with `okmate timings --format json` output.

**Owner:** templates, `assets/`, `src/site.rs` as needed.

### Phase 7 — Record the new baseline

**Bound:** Re-run `okmate timings --format json --scenario all` on the
three-root (or current config) workspace in debug and release. Paste
those machine-local numbers into the research record (or a short status
snapshot). Do not log the phase complete until hosted CI on that
revision is green. Do not hand-roll a `/tmp` crate for this step.

**Out of bound:** Further feature work.

**Exit:** Updated measurements committed; `okmate check knowledge
--profile strict --format terminal`.

## Non-goals

- A portable engine SLA or CI perf budget.
- Partial-graph `okf::load` for `view path/to/concept.md`.
- Replacing Askama or Datastar.
- Making `--profile base` the default for `check` or `view`.
- Turning `okmate benchmark` into a latency tool.

## Dependency on earlier plans

Phase 1 is new measurement surface in okmate; it maps engine
`LoadTimings` the way rocci-okf `--profile-report` did, without
importing that CLI.[^load-plan][^okf-load] Phase 2 assumes the
multi-bundle workspace and keep-nav Datastar contract already described
for the extended viewer and shell parity.[^multi-plan][^shell-plan]
Phase 3 reuses engine `LoadOptions` and `ParseCache`; it does not
reopen skipped engine Phase 5.[^load-plan]

[^research]: Click stall is three-root `reload` (debug ~7s); fragment render of a small page is ~1ms; review fragment is ~310 KiB / 151 rows.
[^pages]: Today’s Datastar GET calls `workspace.reload`.
[^http]: `AppState` is cloned per request and currently owns `Workspace` by value.
[^workspace]: `reload` is `load_members` over every root.
[^preview]: Prepare writes the full tree before bind; watch reloads a private clone.
[^site]: `write_html_pages` emits every concept; `build` must keep doing that.
[^cli]: `view` defaults to Strict; `benchmark` is retrieval quality, not click latency.
[^readme]: Published CLI table has no timings command today.
[^ast]: Strict `LoadOptions` default provenance on.
[^okf-load]: `load_with_cache` is the reuse API; `LoadTimings` already exist on the engine side.
[^parse-cache]: Directory persistence is caller-provided; okmate should pass `OKMATE_CACHE`.
[^queue]: Full `review_rows` loop.
[^log-tpl]: Full `log_days` loop on the log page.
[^review-js]: Client filter requires every row present.
[^nav-js]: Keep-nav is a product constraint, not optional.
[^load-plan]: Engine cache and provenance batching already shipped; preview policy was rocci-okf, not okmate.
[^multi-plan]: Workspace is N loads at session start; clicks must not repeat them.
[^shell-plan]: Main+toc patch, nav stays.
[^overview]: Application crate owns the preview HTTP and HTML.
[^engine-readme]: Engine stays UI-neutral.
[^cargo]: Only `comrak` and `okf` are opt-level 3 in dev today.
