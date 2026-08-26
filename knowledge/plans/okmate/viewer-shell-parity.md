---
type: Implementation Plan
title: Okmate viewer shell parity with last rocci-okf
description: Restore viewport-locked three panes, resize, outline spy, and keep-nav sidebar behavior in Askama and Okmate JS without depending on rocci crates.
tags: [domain/okmate, concern/rendering, concern/tooling]
status: draft
generated: { by: process:cursor, at: 2026-08-26T20:40:00Z }
stale_after: 2026-11-26
authority: exploratory
owners: [human:nils]
sources:
  - id: audit
    resource: ../../audits/okmate/viewer-shell-parity.md
    title: Viewer shell versus last rocci-okf
    author: process:cursor
    last_modified: 2026-08-26
  - id: extract
    resource: ../okf/okmate.md
    title: Okmate extractable Rust OKF mate
    author: process:cursor
    last_modified: 2026-08-26
  - id: dash
    resource: dashboard-parity.md
    title: Dashboard content parity plan
    author: process:cursor
    last_modified: 2026-08-26
  - id: css
    resource: ../../../assets/app.css
    title: Okmate shell CSS
    author: process:git
    last_modified: 2026-08-26
  - id: base
    resource: ../../../templates/base.html
    title: Okmate document chrome
    author: process:git
    last_modified: 2026-08-26
  - id: nav-tpl
    resource: ../../../templates/nav.html
    title: Okmate sidebar markup
    author: process:git
    last_modified: 2026-08-26
  - id: toc-tpl
    resource: ../../../templates/toc.html
    title: Okmate outline markup
    author: process:git
    last_modified: 2026-08-26
  - id: fragment
    resource: ../../../templates/fragments/main.html
    title: Datastar main and toc fragment
    author: process:git
    last_modified: 2026-08-26
  - id: site
    resource: ../../../src/site.rs
    title: Nav tree and page render
    author: process:git
    last_modified: 2026-08-26
  - id: pages
    resource: ../../../src/http/pages.rs
    title: Datastar GET patch
    author: process:git
    last_modified: 2026-08-26
  - id: views
    resource: ../../../src/views/mod.rs
    title: Document and NavNode views
    author: process:git
    last_modified: 2026-08-26
  - id: md
    resource: ../../../okf/src/markdown.rs
    title: Heading IDs versus article HTML
    author: process:git
    last_modified: 2026-08-26
  - id: nav-test
    resource: ../../../tests/navigation.rs
    title: Datastar fragment keeps nav out of the patch
    author: process:git
    last_modified: 2026-08-26
  - id: goto
    resource: ../../../../rocci/crates/rocci-ui/assets/goto.js
    title: rocci-ui keep-nav reference
    author: process:git
    last_modified: 2026-08-26
  - id: toc-js
    resource: ../../../../rocci/crates/rocci-ui/assets/toc.js
    title: rocci-ui outline spy reference
    author: process:git
    last_modified: 2026-08-26
  - id: resize-js
    resource: ../../../../rocci/crates/rocci-ui/assets/resize.js
    title: rocci-ui resize reference
    author: process:git
    last_modified: 2026-08-26
---

# Okmate viewer shell parity with last rocci-okf

## Goal

Make `okmate view` feel like the last rocci-okf review chrome: three
independently scrolling panes, drag-resize, outline highlight as the
article scrolls, and a sidebar that keeps scroll and folds while updating
the current page.[^audit][^extract]

## Out of bound

Depending on any `rocci-*` crate or copying `rocci-ui` as a dependency.
Restoring `#okf-*` IDs or `/__rocci_okf/`. Graph or search HTML. Review
write/approve. Settings edge-matrix. Changing `classify_concept_action`.
Reopening [dashboard-parity](dashboard-parity.md) content columns unless
a leftover is listed in Phase 5.[^extract][^dash]

## Constraints that do not move

- HTML and JS stay in this crate. `okf` stays UI-neutral except Phase 2
  heading IDs on `article_html`.[^extract][^md]
- Landmarks stay `#okmate-nav`, `#okmate-main`, `#okmate-toc`. CSS prefix
  `okmate-`. Storage keys `okmate-*`.
- Datastar `@get` still patches main and toc and **does not** replace the
  sidebar node. Sync current/open onto the kept tree instead.[^extract][^nav-test][^pages]
- Port behavior from last `presentation.rs` + `goto.js` / `toc.js` /
  `resize.js`; rewrite selectors and names.[^goto][^toc-js][^resize-js]
- Do not mix unrelated working-tree changes into phase commits.

## Phases

### Phase 1 — Viewport-locked panes and resize

**Bound:** `.okmate-shell` fills `100dvh` (minus optional
`--okmate-chrome-top` / `--okmate-chrome-bottom`). Nav, main, and toc
each `overflow-y: auto`. Grid columns
`var(--okmate-nav-width, 16.5rem) minmax(0, 1fr) var(--okmate-outline-width, 13.5rem)`.
Hide the toc column when it has no links (do not leave an empty third
track). Add `assets/resize.js`: handles on `#okmate-nav` and
`#okmate-toc`, persist to `localStorage`, keyboard arrows on the
separator. Load it from `base.html` and `write_assets`. Narrow viewport
(`max-width: 48rem`): stack, hide toc column and handles.[^css][^base][^resize-js][^audit]

**Out of bound:** Outline spy. Nav markup. Desktop IPC for widths.

**Tests:** Built CSS/HTML contain `--okmate-nav-width`,
`.okmate-col-resizer`, and `resize.js`. Unit-test clamp helpers if they
are Rust; otherwise assert script and class names.

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`.

**Owner:** `assets/app.css`, `assets/resize.js`, `templates/base.html`,
`src/site.rs`.

### Phase 2 — Heading IDs and outline spy

**Bound:** Engine `article_html` headings carry the same IDs as
`Heading.id` (comrak `header_ids` if it matches `assign_heading_id`,
else a small post-pass). `assets/toc.js` spies `.okmate-toc-link` (and
mobile `.okmate-outline-link` if present), sets `is-current` /
`aria-current="location"`, and scrolls the article pane. CSS current
border matches the old outline. Re-run spy after Datastar morph.
`base.html` loads `toc.js`.[^md][^toc-tpl][^toc-js][^audit]

**Out of bound:** Changing heading slug rules used by graph fragments
except to emit those IDs in HTML.

**Tests:** `okf` fixture HTML contains `id="details"` (or the assigned
slug) on the heading. Okmate build/toc test asserts `is-current` class
in CSS and `toc.js` in the page. Engine tests that parse headings stay
green.

**Exit:** `cargo test -p okf`, `cargo test -p okmate --no-default-features`,
and `cargo fmt --all -- --check`.

**Owner:** `okf/src/markdown.rs`, `assets/toc.js`, `assets/app.css`.

### Phase 3 — Sidebar contract

**Bound:** Collection groups: `details.nav-section` with
`data-okmate-nav-section="{path}"`, `data-okmate-nav-current` when the
route is in the group, `open` when current or restored later. Summary is
a **span** (`nav-link nav-category`), not an `@get` link. First child
is Overview → `/{path}/`. Then nested collections, then leaves. Site
links (Dashboard, Review, Settings, Overview, leaves) keep `@get`.
Plus/minus or chevron on summary. Mobile `details.okmate-nav-menu`.
`NavNode` may gain `section_key` / `overview`.[^nav-tpl][^site][^views][^audit]

**Out of bound:** Client persist/sync (Phase 4). Breadcrumbs.

**Tests:** Built nav for a nested concept contains
`data-okmate-nav-section="plans"`, an Overview href to `/plans/`, and no
`@get` on the category summary text.

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`.

**Owner:** `templates/nav.html`, `src/site.rs`, `src/views/mod.rs`.

### Phase 4 — Keep-nav client

**Bound:** After each successful Datastar document `@get`, sync the kept
`#okmate-nav`: clear old `is-current` / `data-okmate-nav-current`, apply
from the fragment **or** from `pages.json` / a tiny nav-state payload
in the patch (prefer deriving from the new URL if the fragment has no
nav). Open ancestor sections for the current route. Persist fold state
and `#okmate-nav` `scrollTop` in `sessionStorage` (`okmate-nav-sections`,
`okmate-nav-scroll`). Intercept category summary clicks so toggle does
not navigate. Reset `#okmate-main` scroll on page change; honor hash.
May live in `assets/goto.js` or `assets/nav.js`. Do not replace
`#okmate-nav`.[^goto][^pages][^fragment][^nav-test][^extract]

**Out of bound:** Lane-specific rocci storage keys. Cmd-K redesign.

**Tests:** Keep the existing “patch has no `#okmate-nav`” test. Add a
render test that two routes differ in `is-current` / `data-okmate-nav-current`
on full documents. JS contract comments or a small node/deno test only
if the repo already runs one; otherwise document enhance hooks called
from a Datastar after-patch if the SDK allows, or a `MutationObserver`
on `#okmate-main`.

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`.

**Owner:** `assets/goto.js` or `assets/nav.js`, `templates/base.html`.

### Phase 5 — Home leftovers and mobile outline

**Bound:** Recents list: two-column subgrid like last `okf-recents-list`.
Concept and collection pages: breadcrumb row above the article (reuse
nav ancestors; Dashboard is the root). Mobile in-main
`details.okmate-outline-menu` when toc is hidden. Optional `rd-hr`
spacing only via existing okmate classes.[^dash][^audit]

**Out of bound:** Changing stat cards, queue columns, or concept-meta
fields.

**Tests:** Home HTML has the recents grid class; a concept page contains
a breadcrumb to its collection; mobile outline markup exists.

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`.

**Owner:** `templates/fragments/home.html`, article template,
`assets/app.css`, `src/site.rs`.

## Status

Exploratory; no phase started. Findings:
[viewer shell audit](/audits/okmate/viewer-shell-parity.md).[^audit]

[^audit]: F1 panes/resize, F2 spy and heading IDs, F3 sidebar, F4 leftovers.
[^extract]: Askama in this crate; keep `#okmate-nav`; no `rocci-ui`.
[^dash]: Content parity is a different plan; Phase 5 is visual leftovers only.
[^css]: Current grid is fixed rem tracks; main is not a pane scroller.
[^base]: No toc.js or resize.js.
[^nav-tpl]: Link-in-summary; no Overview; no section keys.
[^toc-tpl]: Links without spy classes wired.
[^fragment]: Main plus toc patch.
[^site]: Forest without Overview child.
[^pages]: `PatchElements` of `render_main_fragment`.
[^views]: `NavNode` is href/title/current/open/children.
[^md]: Heading IDs are not emitted on `article_html`.
[^nav-test]: Patch must not include `#okmate-nav`.
[^goto]: Reference for sync, persist, scroll restore.
[^toc-js]: Reference for spy and hash scroll.
[^resize-js]: Reference for handles and width persistence.
