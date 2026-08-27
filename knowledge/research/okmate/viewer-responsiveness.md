---
type: Research Report
title: Viewer click-path latency and large chrome pages
description: Sidebar navigation is slow because every Datastar GET reloads every workspace bundle with Strict git provenance; review and log then ship the full table or log in one patch.
tags: [domain/okmate, concern/performance, concern/rendering, concern/architecture]
status: draft
generated: { by: process:cursor, at: 2026-08-27T11:50:00Z }
stale_after: 2026-11-27
authority: exploratory
owners: [human:nils]
sources:
  - id: plan
    resource: ../../plans/okmate/viewer-responsiveness.md
    title: Viewer responsiveness plan
    author: process:cursor
    last_modified: 2026-08-27
  - id: pages
    resource: ../../../src/http/pages.rs
    title: Datastar GET reloads the workspace
    author: process:git
    last_modified: 2026-08-27
  - id: http
    resource: ../../../src/http/mod.rs
    title: AppState clones Workspace per request
    author: process:git
    last_modified: 2026-08-27
  - id: workspace
    resource: ../../../src/workspace.rs
    title: Workspace::reload calls okf::load per member
    author: process:git
    last_modified: 2026-08-27
  - id: preview
    resource: ../../../src/preview.rs
    title: View prepare writes the full HTML tree before bind
    author: process:git
    last_modified: 2026-08-27
  - id: site
    resource: ../../../src/site.rs
    title: write_html_pages and page_for_route_nav
    author: process:git
    last_modified: 2026-08-27
  - id: cli
    resource: ../../../src/cli.rs
    title: view defaults to Profile::Strict; benchmark is retrieval
    author: process:git
    last_modified: 2026-08-27
  - id: readme
    resource: ../../../README.md
    title: Published okmate CLI table
    author: process:git
    last_modified: 2026-08-27
  - id: ast
    resource: ../../../okf/src/ast.rs
    title: LoadOptions provenance defaults on with Strict
    author: process:git
    last_modified: 2026-08-26
  - id: okf-load
    resource: ../../../okf/src/lib.rs
    title: load, load_timed, and load_with_cache
    author: process:git
    last_modified: 2026-08-26
  - id: parse-cache
    resource: ../../../okf/src/parse_cache.rs
    title: ParseCache unused by okmate view
    author: process:git
    last_modified: 2026-08-26
  - id: cargo
    resource: ../../../Cargo.toml
    title: Dev opt-level for comrak and okf only
    author: process:git
    last_modified: 2026-08-27
  - id: nav-tpl
    resource: ../../../templates/nav.html
    title: Sidebar renders nav_items twice
    author: process:git
    last_modified: 2026-08-27
  - id: queue
    resource: ../../../templates/fragments/queue.html
    title: Review queue renders every row
    author: process:git
    last_modified: 2026-08-27
  - id: log-tpl
    resource: ../../../templates/fragments/log.html
    title: Log page renders every merged entry
    author: process:git
    last_modified: 2026-08-27
  - id: review-js
    resource: ../../../assets/review.js
    title: Client filter walks every review row
    author: process:git
    last_modified: 2026-08-26
  - id: nav-js
    resource: ../../../assets/nav.js
    title: Datastar click patches main and toc only
    author: process:git
    last_modified: 2026-08-27
  - id: load-status
    resource: ../../status/okf-load-performance.md
    title: OKF load-performance improvement results
    author: process:cursor
    last_modified: 2026-08-26
  - id: load-plan
    resource: ../../plans/okf/okf-load-performance.md
    title: OKF load-performance improvements
    author: process:cursor
    last_modified: 2026-08-26
  - id: multi-plan
    resource: ../../plans/okmate/extended-multi-bundle.md
    title: Extended multi-bundle viewer
    author: process:cursor
    last_modified: 2026-08-27
  - id: overview
    resource: ../../architecture/system-overview.md
    title: OKMate system overview
    author: process:cursor
    last_modified: 2026-08-27
  - id: gov
    resource: ../../../src/views/governance.rs
    title: merged_log and review_needs_attention
    author: process:git
    last_modified: 2026-08-27
  - id: engine-readme
    resource: ../../../okf/README.md
    title: Portable OKF engine load timings and ParseCache
    author: process:git
    last_modified: 2026-08-26
---

# Viewer click-path latency and large chrome pages

## Claim

Live `okmate view` feels slow because **every sidebar click re-parses every
configured knowledge root**, including Strict git provenance, then Askama
emits the **entire** review table or merged log in one Datastar patch.
Once a workspace is already in memory, rendering a small document is
milliseconds. Large chrome pages are a second problem: they must window
and preload, not dump every row into the DOM.[^pages][^workspace][^queue][^plan]

This is exploratory measurement on this machine, 2026-08-27, against the
three directory roots `okmate roots --no-sync` resolved
(`okmate`, `rocci`, `rocci-spotify`; 197 Markdown files, 151 concepts).
Numbers are evidence, not a latency SLA.[^load-status]

The recommended work is the
[viewer responsiveness plan](/plans/okmate/viewer-responsiveness.md).[^plan]

## What the user feels

`cargo run -- view` (debug, default `desktop` feature) opens a workspace
of every enabled config root. Sidebar links use Datastar `@get`, which
patches `#okmate-main` and `#okmate-toc` and leaves `#okmate-nav` in
place.[^nav-js][^multi-plan][^overview]

That GET does **not** use the bundles already sitting in `AppState`. It
calls `Workspace::reload`, which is `okf::load` per member, with
`Profile::Strict` and therefore git provenance on.[^pages][^workspace][^cli][^ast]

```text
render_main_fragment
  -> workspace.reload(profile)
       -> okf::load(okmate) + okf::load(rocci) + okf::load(rocci-spotify)
  -> page_for_route_nav (builds nav tree even though the fragment omits nav)
  -> render_main_fragment
```

A full page refresh still hits the prewritten `ServeDir` tree. Clicks
never do; they always take the reload path.[^http][^pages]

## Measured click path

`okf::load_timed` / `Workspace::load_members` on this machine:

| Path | Debug wall | Release wall |
| --- | ---: | ---: |
| `okmate` Strict + provenance | 1188ms | 154ms |
| `rocci` Strict + provenance | 5449ms | 603ms |
| `rocci-spotify` Strict + provenance | 602ms | 58ms |
| **three-root `reload` (the click)** | **7111ms** | **697ms** |

Debug parse dominates (`rocci` parse 5062ms). Release parse is acceptable;
release provenance on `rocci` is still 376ms. A second `reload` is the
same cost: **okmate view does not pass `ParseCache`**. The engine already
has `load_with_cache`; preview never calls it.[^okf-load][^parse-cache][^engine-readme]

With an in-memory `ParseCache` (still Strict + provenance), a second
debug load of `rocci` is 369ms — parse hits, git provenance remains.
Cached three-root debug reload would be roughly 400ms, not 7s, and still
too slow for a click. The click must not load at all.[^parse-cache]

After the workspace is already loaded, `page_for_route_nav` plus
`render_main_fragment` on this machine:

| Route | Fragment | Rows / entries | Debug page+frag |
| --- | ---: | ---: | ---: |
| `/@okmate/architecture/system-overview/` | 6.6 KiB | article 3.7 KiB | 1.3ms |
| `/` home | 8.5 KiB | 5 log bullets | 3.3ms |
| `/log/` | 112 KiB | 210 bullets | 2.6ms |
| large Rocci plan | 89 KiB | article 76 KiB | 1.5ms |
| `/review/` | 310 KiB | 151 rows | 5.1ms |

Askama is not why a small document click feels frozen. The 7s (debug)
or 0.7s (release) `okf::load` of the whole workspace is.[^pages][^site]

These numbers came from a throwaway `/tmp` crate that linked `okf` and
okmate. okmate has `benchmark` for retrieval quality and the engine has
`load_timed`, but there is no checked-in `okmate timings` /
`--profile-report` surface. Later investigations should not repeat that
harness; the paired plan’s first phase is that pipeline.[^cli][^readme][^plan][^okf-load]

## First open and watch

`prepare` loads the workspace, then `write_html_pages` emits **every**
route as a full HTML document **before the server binds**. Watch does
the same after each debounce.[^preview][^site]

On this three-root workspace:

| Step | Debug | Release |
| --- | ---: | ---: |
| First `load_members` | 7117ms | 702ms |
| `write_html_pages` | 845ms | 335ms |
| Files / bytes written | 203 files, 37.8 MiB | same |
| Largest file | `review/index.html` 479 KiB | same |

Each static page includes the full sidebar **twice** (mobile `details`
and desktop copy). A 6.6 KiB article fragment sits inside a 175 KiB
document; home is 177 KiB; log is 280 KiB; review is 479 KiB.[^nav-tpl][^site]

`AppState` is `Clone`. Axum clones it per request. `Workspace` / `Bundle`
deep-clone every `article_html`. Today that clone is wasted because
`reload` throws the data away; after a memory-serve fix, the handle must
be `Arc` (and watch must share it), not a per-request clone.[^http][^workspace][^preview]

Watch rebuilds a **different** `Workspace` clone and only writes files.
It does not update `AppState.workspace`. Freshness on click today comes
from the accidental full reload. A memory-serve change has to plug watch
into the same `Arc`, or clicks go stale.[^preview][^pages]

## Review and log are full dumps

`/review/` concatenates a row per concept across every loaded root
(151 here) plus the action subset, stats, and every diagnostic. The
template loops the whole `review_rows` vec. `review.js` binds every
`.okmate-row` and hides rows in the DOM for filter/search.[^queue][^review-js][^multi-plan]

`/log/` parses each root `log.md` on every page build and emits every
date and bullet (210 here; Rocci `log.md` is 75 KiB). Home already
caps the merged log at five entries; the log page does not.[^gov][^log-tpl]

That is the right product surface to **window**: first paint a viewport
plus overscan, keep scroll continuous, preload the next and previous
segments, and do **not** show page numbers. Client-side hide-all-rows
cannot survive that; filter/search have to re-query the window.[^plan][^review-js]

Large articles (Rocci plans in the 60–80 KiB HTML class) are a later,
thresholded version of the same idea. They are not why a 4 KiB
architecture note is slow.[^site]

## Prior engine work that preview does not use

The [OKF load-performance](/plans/okf/okf-load-performance.md) work
already shipped `load_timed`, batched provenance, `ParseCache`, and
(in Rocci) a preview path that defaults provenance **off**. Phase 5
bounded concept-path load was skipped because a single-bundle release
`load` was already sub-second.[^load-plan][^load-status][^engine-readme]

okmate `view` still:

- defaults to Strict **with** provenance[^cli][^ast]
- calls `okf::load` with **no** cache on every click and every watch
  tick[^pages][^workspace][^preview]
- multiplies that by every enabled root[^multi-plan]

The workspace is supposed to be “each bundle is one `okf::load`” at
session start, not once per navigation.[^multi-plan]

Dev-profile `opt-level = 3` covers `comrak` and `okf` only. `yaml-rust`
and the rest of the parse graph stay unoptimized, which is why debug
parse of `rocci` is ~5s while release is ~226ms. That hurts
`cargo run -- view` first open. It is not the click-path architecture
bug.[^cargo][^load-status]

## What not to blame

- Datastar morph of a **small** main+toc fragment (a few KiB) after the
  server returns.
- Askama render of a single concept (sub-millisecond here).
- `nav_tree` / `review_needs_attention` on the fragment path (~1ms).
  Wasteful, not the stall.
- Bounded concept-path `okf::load` (skipped engine Phase 5). Nav, review,
  and goto need the catalog in memory; they should not **re-parse** it.

## Implications for the plan

1. **Ship `okmate timings` first.** Same spans as this record (`load`,
   workspace reload, site write, per-route fragment size/time, click
   path), as JSON and terminal, plus live `Server-Timing` on Datastar
   GET. Do not keep measuring in `/tmp`.[^plan][^cli][^readme]
2. **Clicks must read `AppState`, not disk.** Share workspace with watch
   through `Arc<RwLock<…>>` (or equivalent). This is the responsiveness
   fix for small documents.[^pages][^http][^preview]
3. **Preview load is not `check`.** Keep Strict schema; default
   provenance off on `view`; persist `ParseCache` across watch ticks
   (and optionally process restarts) under `OKMATE_CACHE`.[^ast][^load-plan]
4. **Live preview should not write 38 MiB of HTML** before bind or on
   every save. `okmate build` stays a full static tree.[^site][^preview]
5. **Review and log window** with invisible overscan preload. Filters
   become server-side window queries. Do not invent page-number UI.[^queue][^log-tpl]
6. Optional later: heading-windowed large articles; one nav tree in the
   DOM; remaining dev-profile parse crates.

Success on this machine looks like: debug sidebar click to a small
document returns in tens of milliseconds; `/review/` and `/log/` first
paint a viewport without shipping every row; first `cargo run -- view`
open is load-once, not load-per-click.[^plan]

[^plan]: Paired implementation plan; this record is evidence, not a phase log.
[^pages]: `render_main_fragment` calls `state.workspace.reload(state.profile)` on every Datastar GET.
[^http]: `AppState` is `Clone` and holds `Workspace` by value; the router layers Datastar GET in front of `ServeDir`.
[^workspace]: `reload` rebuilds member specs and `okf::load`s each path; no `ParseCache`.
[^preview]: `prepare` calls `build_workspace_nav` before bind; `watch_rebuild` reloads and rewrites the tree on a clone that is not `AppState`.
[^site]: `write_html_pages` walks every concept and index; `page_for_route_nav` always builds `nav_tree`.
[^cli]: `Commands::View` defaults `ProfileArg::Strict`; `benchmark` is retrieval hit-rate, not view latency.
[^readme]: README CLI table lists `benchmark` and `view` but no timings command.
[^ast]: `LoadOptions::new` sets `provenance: profile == Profile::Strict`.
[^okf-load]: `load` is `load_timed` without a cache; `load_with_cache` exists and is unused by okmate.
[^parse-cache]: `ParseCache` keys path + mtime + size; okmate preview never constructs one.
[^cargo]: Workspace `profile.dev.package` lists `comrak` and `okf` only.
[^nav-tpl]: `nav.html` includes `nav_items.html` in both the mobile menu and `.okmate-desktop-nav`.
[^queue]: `fragments/queue.html` `{% for row in review_rows %}` and `{% for row in action_rows %}`.
[^log-tpl]: `fragments/log.html` loops every `log_days` entry with no limit on `page_kind == "log"`.
[^review-js]: Filter and search set `row.hidden` on the full NodeList.
[^nav-js]: Datastar patches `#okmate-main`; MutationObserver syncs nav current state without replacing nav.
[^load-status]: Engine Phases 1–4 shipped; preview-versus-check provenance split is recorded for rocci-okf, not okmate view.
[^load-plan]: Skipped bounded concept-path load; parse cache and provenance batching live in `okf`.
[^multi-plan]: Workspace is `Vec` of `okf::load` results; Datastar GET must not replace nav.
[^overview]: Application crate owns Askama, Axum, Datastar, and desktop preview.
[^gov]: `merged_log` re-reads each `log.md`; `DASHBOARD_LOG_LIMIT` is 5; `review_needs_attention` scans every concept.
[^engine-readme]: Engine documents `load_timed` and directory `ParseCache`; it does not choose preview policy.
