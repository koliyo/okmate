---
type: Implementation Plan
title: Extended multi-bundle viewer
description: Load configured OKF roots as one preview workspace with separated and merged sidebar modes, always-merged dashboard recents and log, and collection summaries on nav hover.
tags: [domain/okmate, concern/architecture, concern/rendering, concern/tooling]
status: draft
generated: { by: process:cursor, at: 2026-08-27T08:10:00Z }
stale_after: 2026-11-27
authority: exploratory
owners: [human:nils]
sources:
  - id: research
    resource: ../../research/okmate/extended-multi-bundle.md
    title: Extended multi-bundle viewer research
    author: process:cursor
    last_modified: 2026-08-27
  - id: overview
    resource: ../../architecture/system-overview.md
    title: Okmate system overview
    author: process:cursor
    last_modified: 2026-08-26
  - id: multi-roots
    resource: ../okf/multi-knowledge-roots.md
    title: Multiple knowledge roots (registry contract)
    author: process:cursor
    last_modified: 2026-08-25
  - id: dash-plan
    resource: dashboard-parity.md
    title: Okmate dashboard parity
    author: process:cursor
    last_modified: 2026-08-26
  - id: shell-plan
    resource: viewer-shell-parity.md
    title: Okmate viewer shell parity
    author: process:cursor
    last_modified: 2026-08-26
  - id: site
    resource: ../../../src/site.rs
    title: Askama routes, nav forest, and HTML build
    author: process:git
    last_modified: 2026-08-27
  - id: preview
    resource: ../../../src/preview.rs
    title: Live preview, session, and single-root watch
    author: process:git
    last_modified: 2026-08-27
  - id: pages
    resource: ../../../src/http/pages.rs
    title: Datastar GET fragment
    author: process:git
    last_modified: 2026-08-27
  - id: http
    resource: ../../../src/http/mod.rs
    title: AppState
    author: process:git
    last_modified: 2026-08-27
  - id: gov
    resource: ../../../src/views/governance.rs
    title: Recents and stats
    author: process:git
    last_modified: 2026-08-27
  - id: views
    resource: ../../../src/views/mod.rs
    title: Document and NavNode views
    author: process:git
    last_modified: 2026-08-27
  - id: home
    resource: ../../../templates/fragments/home.html
    title: Dashboard markup
    author: process:git
    last_modified: 2026-08-27
  - id: nav-items
    resource: ../../../templates/nav_items.html
    title: Unrolled collection nav
    author: process:git
    last_modified: 2026-08-27
  - id: nav-js
    resource: ../../../assets/nav.js
    title: Keep-nav sync
    author: process:git
    last_modified: 2026-08-27
  - id: ast
    resource: ../../../okf/src/ast.rs
    title: Index and Log types
    author: process:git
    last_modified: 2026-08-27
  - id: build-test
    resource: ../../../tests/build.rs
    title: Built home asserts Knowledge Collections
    author: process:git
    last_modified: 2026-08-27
---

# Extended multi-bundle viewer

## Goal

Give `okmate view` a workspace of every enabled configured knowledge root:
separated and merged sidebar trees, dashboard recents and bundle log always
merged across those roots, collection blurbs on sidebar hover, and stable
`/@<id>/` document URLs — without merging catalogs in `okf` or changing
single-root `check` / `inspect` / `search` / `build`.[^research][^overview]

## Out of bound

- Teaching `okf::load`, `check`, `inspect`, `search`, or `build` to consume
  the whole registry. Static `okmate build` stays one HTML tree with
  `/{id}/` routes.[^overview][^multi-roots]
- Rewriting the historical multi-roots phases, edge-matrix settings, git
  poll-while-viewing, or OKF3010/OKF3011 workspace check.[^multi-roots]
- Merging search indexes, graph HTML, or Cmd-K ranking beyond showing
  `root` on each hit.
- Redesigning `/review/` filters, approve/write, or action classification.
  Workspace review only concatenates existing rows plus a source label.
- Authoring UI, writable git roots, or minting an approved Decision.
- Changing concept IDs or collection nesting rules.
- Native `title`-only tooltips as the collection summary.
- Mixing unrelated working-tree changes into phase commits.

## Constraints that do not move

- Workspace is `Vec<(ResolvedRoot, Bundle)>` in okmate. Each bundle is one
  `okf::load`.[^research][^overview]
- HTML, CSS, and JS stay in this crate. Landmarks stay `#okmate-*`. CSS
  prefix `okmate-`.[^shell-plan]
- Ordinary Datastar GET still patches main and toc and **does not** replace
  `#okmate-nav`. Mode toggle is the documented exception (full reload, or
  one nav-only patch).[^shell-plan][^nav-js]
- Recents stay leaf concepts sorted by `generated.at`, exclude collection
  indexes, limit ten across the union.[^dash-plan][^gov]
- Tokens never appear in HTML, `pages.json`, recents, or log rows.
- Durable membership stays `config.toml`. Nav mode stays in
  `session.json` (server-owned), not Datastar signals.[^preview]
- Explicit `okmate view <path>` remains that one bundle. `/{id}/` URLs
  unchanged for single-root preview and `build`.[^site]

## Current behavior

The registry and settings UI already list many roots. The viewer loads
one session or CLI path, builds one nav forest, one recents list, and a
home page that dumps root `index.md` under “Knowledge Collections”.
`log.md` is validated and then unused in HTML. Evidence:
[extended-multi-bundle research](/research/okmate/extended-multi-bundle.md).[^research][^home][^ast]

## Target contract

### When a workspace is active

No CLI path (desktop no-args included) **and** two or more enabled
resolved roots from `config.toml`. One enabled root, empty config, or
explicit path: today's single-bundle view.[^preview][^research]

### URLs

| Surface | Single-root | Workspace |
| --- | --- | --- |
| Dashboard, review, settings | `/`, `/review/`, `/settings/` | Same |
| Concept / collection | `/{id}/` | `/@<root-id>/<id>/` |
| Cmd-K / `pages.json` | `route`, `path`, `collection` | plus `root` |

Rewrite workspace `article_html` so intra-bundle `/{id}/` and `okf:<id>/…`
point at `/@<id>/…` when that root is loaded; leave unresolved `okf:`
hrefs unchanged.[^research]

### Sidebar

- **Separated** (default when a workspace is active): each root is a
  top-level folder (VS Code workspace folders). Existing collection forest
  hangs under it. `section_key` is namespaced (`okmate/plans`).
- **Merged**: union by relative collection/concept path. Sibling leaves
  may share a path; every leaf (and colliding collection) shows the root
  id. `section_key` is the relative path (`plans`).
- Toggle in the nav chrome; hidden unless two or more roots are loaded.
  Persist `nav_mode` on `session.json`. Toggle reloads or patches
  `#okmate-nav` only.
- Recursive nav template (or equivalent) so a root folder plus nested
  collections does not require a fourth hand-unrolled copy of
  `nav_items.html`.[^nav-items][^site]
- `nav.js` current-section matching understands `/@<id>/` prefixes.

### Dashboard (both modes)

- Recents: union of leaves, ten items, `generated.at` order, collection
  badge, **root badge** when more than one bundle is loaded. Always the
  merged list even if the sidebar is separated.[^gov][^home]
- Remove `<h2>Knowledge Collections</h2>` and do **not** render bundle
  root `index.md` on `/`.[^home][^build-test]
- Below recents: merged bundle log (`#okmate-log`). Parse each root's
  `log.md` into date headings plus bullets; union by date descending;
  tag each bullet with root id; skip missing logs.[^ast][^research]
- Keep review CTA and stat grid. Workspace stats are summed counts.
  Workspace `/review/` concatenates rows with a source column.

### Collection hover

On each collection `summary`, show the first non-heading prose paragraph
of that collection's `index.md` (not the child link list). Keyboard
accessible (`aria-describedby` or equivalent). Merged collections that
exist in several roots may stack blurbs with root labels.

## Phases

### Phase 1 — Workspace load and `/@id/` routes

**Bound:** A `Workspace` (or equivalent) of enabled `okf::load` results.
`AppState` holds it (single-root is a workspace of one). `view` with no
path and ≥2 enabled config roots loads all of them; explicit path stays
one bundle. Document routes `/@<id>/<concept>/` when `len > 1`.
`page_for_route` / Datastar GET resolve those routes. Watch every loaded
directory. `pages.json` includes `root` when needed. Intra-bundle and
`okf:` href rewrite in workspace HTML. Session may record that the window
is a workspace without changing config membership.[^http][^pages][^preview][^site]

**Out of bound:** Nav mode toggle. Dashboard log. Collection hover.

**Tests:** Two temp bundles with a unique path and a colliding
`plans/shared.md`; GET `/@a/plans/shared/` and `/@b/plans/shared/` return
distinct titles; single-root `/{id}/` still works; Datastar fragment still
omits `#okmate-nav`.

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`.

**Owner:** `src/preview.rs`, `src/http/`, `src/site.rs`, `src/views/`.

### Phase 2 — Dashboard recents, log, drop collections

**Bound:** `recent_leaf_documents` over the workspace (ten, `generated.at`,
root badge when `len > 1`). Parse `log.md` in okmate; render `#okmate-log`
under recents. Delete Knowledge Collections heading and home
`article_html`. Summed stats. Review rows gain `root` / source when
workspace. Update `tests/build.rs` and dashboard unit tests.[^home][^gov][^build-test][^dash-plan]

**Out of bound:** Sidebar folder modes. Hover chrome.

**Tests:** Home HTML has `#okmate-recents` and `#okmate-log`, no
`Knowledge Collections`; two-bundle fixture orders recents by
`generated.at` and shows both log days; single-bundle `build` still
emits recents and a log section when `log.md` exists.

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`.

**Owner:** `src/views/governance.rs`, `templates/fragments/home.html`,
`src/site.rs`, `tests/build.rs`.

### Phase 3 — Sidebar separated mode

**Bound:** When a workspace is active, wrap the existing forest in one
top-level `details` per root (id as title). Recursive (or extra-level)
nav template. `NavNode` may gain `root` / `summary`. Namespaced
`section_key`. Goto haystack includes root. Default tree for `len > 1`.
Single-root nav markup unchanged (no extra
wrapper).[^nav-items][^site][^shell-plan][^views]

**Out of bound:** Merged union tree. Mode toggle.

**Tests:** Built/served HTML for two roots contains two top-level
`data-okmate-nav-section` root keys and nested collections under each;
single-root fixture still matches today's Overview/span-summary
contract.

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`.

**Owner:** `src/site.rs`, `templates/nav_items.html`, `src/views/mod.rs`,
`assets/nav.js`.

### Phase 4 — Merged mode and toggle

**Bound:** Path-union forest; colliding leaves keep distinct `/@id/`
hrefs and show root source. `nav_mode: separated | merged` in
`session.json`; control in nav chrome; hidden unless `len > 1`; default
separated. Toggle reloads or patches `#okmate-nav` only. `nav.js` prefix
matching for `/@id/` and merged section keys. Recents stay the merged
list from Phase 2 regardless of mode.[^preview][^nav-js][^research]

**Out of bound:** Collection hover copy.

**Tests:** Same two-bundle fixture: merged HTML has one `plans` section
containing two `shared` leaves with different hrefs; toggle endpoint or
reload switches trees; Datastar document GET still omits nav.

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`.

**Owner:** `src/site.rs`, `src/preview.rs`, `templates/nav.html`,
`assets/nav.js`.

### Phase 5 — Collection hover summaries

**Bound:** First non-heading paragraph of each collection `index.md` on
the collection `summary` (CSS popover or equivalent, keyboard
accessible). Stack per-root blurbs when a merged section has several
indexes. No full child-list HTML in the hover.[^ast][^nav-items]

**Out of bound:** New collection pages, changing index authoring rules.

**Tests:** Fixture index with a blurb plus a list; HTML contains the blurb
in the hover region and not as a replacement for Overview; empty body
indexes omit the popover.

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`.

**Owner:** `src/site.rs`, `templates/nav_items.html`, `assets/app.css`.

## Status

Exploratory; no phase started. Evidence:
[extended-multi-bundle research](/research/okmate/extended-multi-bundle.md).[^research]
Does not execute [multi-knowledge-roots](/plans/okf/multi-knowledge-roots.md);
it opens the viewer slice that plan left out of bound.[^multi-roots]

[^research]: Current viewer is one `okf::load`; registry is not a workspace.
[^overview]: `check` / `inspect` / `search` / `build` stay single-root.
[^multi-roots]: Registry, git cache, and `okf:` spelling; merged review site was out of bound.
[^dash-plan]: Ten-leaf recents by `generated.at`; Knowledge Collections on home.
[^shell-plan]: Ordinary GET does not replace `#okmate-nav`.
[^site]: `page_for_route` and `nav_forest` take one `Bundle`.
[^preview]: Session is one `bundle` path; watch is one directory.
[^pages]: Datastar fragment loads `state.root` only.
[^http]: `AppState.root` is a single path.
[^gov]: `recent_leaf_documents` and `governance_stats` take one `Bundle`.
[^views]: `NavNode` has no root or summary fields.
[^home]: Home template lists recents then Knowledge Collections then `article_html`.
[^nav-items]: Hand-unrolled three-level collection tree.
[^nav-js]: Current-section prefix assumes `/{collection}/`.
[^ast]: Collection `Index.article_html`; `Log` has no rendered body.
[^build-test]: `okmate build` asserts the Knowledge Collections heading.
