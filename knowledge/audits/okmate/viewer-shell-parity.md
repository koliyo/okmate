---
type: Audit
title: Okmate viewer shell versus last rocci-okf
description: Governance page content was ported; three-pane chrome, outline spy, and sidebar keep-nav from the last rocci-okf review shell were not.
tags: [domain/okmate, concern/rendering, concern/tooling]
status: draft
generated: { by: process:cursor, at: 2026-08-26T20:40:00Z }
stale_after: 2026-11-26
authority: descriptive
owners: [human:nils]
sources:
  - id: theme
    resource: https://github.com/koliyo/rocci/blob/ff00ad092a3de51bf5ef1fce599fd65cbdd74424/crates/rocci-okf/templates/OkfTheme.rocci
    title: Last KnowledgeShell composition (rocci-okf before extract)
    author: organization:github
  - id: extract-commit
    resource: https://github.com/koliyo/rocci/commit/43e6f1f4d81a2b98e258f95e169680365e7b1a05
    title: feat(okf) move OKF tooling to okmate
    author: organization:github
  - id: goto
    resource: ../../../../rocci/crates/rocci-ui/assets/goto.js
    title: rocci-ui keep-nav, fold persist, and column swap
    author: process:git
    last_modified: 2026-08-26
  - id: toc-js
    resource: ../../../../rocci/crates/rocci-ui/assets/toc.js
    title: rocci-ui outline scroll spy
    author: process:git
    last_modified: 2026-08-26
  - id: resize-js
    resource: ../../../../rocci/crates/rocci-ui/assets/resize.js
    title: rocci-ui column resize handles
    author: process:git
    last_modified: 2026-08-26
  - id: chrome-css
    resource: ../../../../rocci/crates/rocci-theme/src/themes/chrome.css
    title: rd-shell grid, toc, and resizer styles
    author: process:git
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
  - id: home
    resource: ../../../templates/fragments/home.html
    title: Home governance markup
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
  - id: goto-okmate
    resource: ../../../assets/goto.js
    title: Okmate Cmd-K only
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
  - id: extract
    resource: ../../plans/okf/okmate.md
    title: Okmate extract plan keep-nav contract
    author: process:cursor
    last_modified: 2026-08-26
  - id: dash
    resource: ../../plans/okmate/dashboard-parity.md
    title: Dashboard content parity plan
    author: process:cursor
    last_modified: 2026-08-26
  - id: plan
    resource: ../../plans/okmate/viewer-shell-parity.md
    title: Viewer shell parity implementation plan
    author: process:cursor
    last_modified: 2026-08-26
---

# Okmate viewer shell versus last rocci-okf

## Scope

Compared the last `rocci-okf` review shell (Rocci commit before
[extract](https://github.com/koliyo/rocci/commit/43e6f1f4d81a2b98e258f95e169680365e7b1a05),
plus shared `rocci-ui` / `rocci-theme` chrome that still lives in
`../rocci`) with current `okmate view` HTML. Authority is descriptive of
those trees as of 2026-08-26. Product choice stays Askama + Datastar in
this crate; IDs stay `#okmate-*`.[^extract][^extract-commit]

[Dashboard parity](/plans/okmate/dashboard-parity.md) already covers home
recents, review tables, concept meta, and preview ports. This audit is
the chrome that plan left out.[^dash]

Remediation is [viewer-shell-parity](/plans/okmate/viewer-shell-parity.md).[^plan]

## What already matches

Home prepends recent leaves, a review CTA, a six-card stat grid, and
Knowledge Collections before `index.md`. Review has a needs-action table,
full queue, filters, and diagnostics. Concept pages carry governance
badges and alerts. Landmarks `#okmate-nav`, `#okmate-main`, and
`#okmate-toc` exist. Datastar `@get` patches main and toc and is tested
to omit the sidebar from the SSE fragment.[^home][^dash][^nav-test][^pages][^fragment][^extract]

## Findings

### F1 — Three panes share the window scroller and have no resizers

Last rust `html_page_for` used `.rd-shell` as a relative three-column
grid with `--rocci-nav-width` / `--rocci-outline-width`, sticky
`#okf-nav` and `#okf-toc` each `max-height` viewport and `overflow-y:
auto`, plus `.rocci-col-resizer` handles from `resize.js`. `OkfTheme`
composed the same nav / main / `#okf-toc` hosts. Shared theme CSS
matches that grid.[^theme][^chrome-css][^resize-js][^extract-commit]

Okmate uses a fixed `16.5rem / 1fr / 13.5rem` grid, no width variables,
no separator, no resize script. `#okmate-main` is not a pane scroller.
`html, body` grow with the article, so the window is the only document
scroll. Nav and toc are sticky with overflow, but there is nothing to
drag and no persisted column widths.[^css][^base]

### F2 — Outline does not spy, and heading IDs are not on the article

Last rust pages inlined `toc.js` when headings existed. That script
marks `.rd-toc-link` / `.outline-link` as `is-current` on capture-phase
scroll and smooth-scrolls hash clicks into the nearest overflow
ancestor. CSS paints the current border.[^toc-js][^chrome-css][^extract-commit]

Okmate lists `#okmate-toc` links with class `okmate-toc-link` and hover
styles only. `assets/goto.js` is Cmd-K. No spy script is loaded.
`okf` assigns heading IDs while walking the AST, then `comrak::format_html`
without `header_ids`, so `article_html` is typically `<h2>…</h2>` with no
`id`. Outline `href="#…"` therefore has no targets.[^toc-tpl][^goto-okmate][^md][^base]

### F3 — Sidebar markup and client behavior are a different product

Last `render_nav_tree` emitted a mobile `details.okf-nav-menu` plus
`.okf-desktop-nav`. Collection groups were `<details data-rocci-nav-section>`
with a **span** summary, an **Overview** child to the collection index,
nested collections, then leaves. `goto.js` kept `#okf-nav` across page
swaps, copied `is-current` / `data-rocci-nav-current`, persisted open
sections in `sessionStorage`, remembered sidebar `scrollTop`, and
animated folds. Category rows were not navigable links.[^goto][^extract-commit][^theme]

Okmate `nav_tree` puts an `<a data-on:click__prevent="@get(…)"` **inside**
`<summary>`. There is no Overview row, no `data-okmate-nav-section`, no
mobile menu. The Datastar patch is only `#okmate-main` plus
`#okmate-toc`. After a click the kept sidebar still shows the previous
`is-current` and open set. That matches the extract plan’s “keep nav”
test and omits the sync half of `goto.js`.[^nav-tpl][^site][^pages][^fragment][^nav-test][^extract]

### F4 — Dashboard chrome leftovers

Content parity is in place; visual chrome is not. Last home recents used
a two-column subgrid (title / type). Okmate wraps title and badge in a
flex row. Rust pages wrapped articles in breadcrumbs. Narrow viewports
had a Knowledge details menu and an in-main “On this page” details
block. Those are absent.[^home][^css][^extract-commit][^theme]

## Non-findings

Do not treat `#okf-*` IDs, `/__rocci_okf/`, or a `rocci-*` crate
dependency as missing product. The extract plan renamed landmarks on
purpose.[^extract]

Cmd-K exists in a thinner form. Settings and review **writes** were
already out of the old rust shell’s live path for several widgets.

## Severity

F3 is why the sidebar “does not work at all like it used to”: in-app
navigation is `@get` by default, and the kept tree never updates. F2
makes the outline inert even on a full page load. F1 is the missing
app-layout feel. F4 is polish after F1–F3.

[^theme]: `KnowledgeShell` placed `#okf-nav`, `#okf-main`, optional mobile outline details, and `#okf-toc`.
[^extract-commit]: Parent of this commit still had `presentation.rs` `html_page_for`, `DEFAULT_CSS`, `render_nav_tree`, and inlined toc script.
[^goto]: `NAV_KEEP` / `MAIN_SWAP` / `TOC_SWAP`, `syncKeptNav`, section storage, sidebar scroll restore.
[^toc-js]: `syncSpy` toggles `is-current`; click handler scrolls the overflow ancestor.
[^resize-js]: Mounts handles on `#okf-nav` / `#okf-toc` inside `.rd-shell`; persists CSS variables.
[^chrome-css]: `.rd-shell` grid and `.rocci-col-resizer`.
[^css]: Fixed three columns; no resizer; main is not `overflow-y: auto` in a locked viewport.
[^base]: Scripts are datastar, goto (palette), review. No toc or resize.
[^nav-tpl]: Link-in-summary plus `@get` on every row, including categories.
[^toc-tpl]: `okmate-toc-link` hash hrefs; no `is-current` class in markup or CSS.
[^fragment]: Fragment is main then toc only.
[^home]: Recents, CTA, stats include; no breadcrumbs.
[^site]: `nav_tree` / `nav_forest` have `open` from route prefix, no Overview child.
[^pages]: Datastar GET returns `PatchElements` of the main fragment.
[^goto-okmate]: Palette only; no nav sync.
[^md]: `comrak_options` has no `header_ids`; IDs live on `Heading` only.
[^nav-test]: Asserts the patch does not contain `id="okmate-nav"`.
[^extract]: Keep `#okmate-nav`; swap main and toc; no `rocci-ui`.
[^dash]: Home / review / meta / ports; chrome out of bound.
[^plan]: Phased restore of layout, spy, nav contract, and keep-nav sync.
