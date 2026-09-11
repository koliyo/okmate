---
type: Implementation Plan
title: Datastar-aligned client JavaScript
description: Keep vanilla ES modules beside Askama and Datastar; call @get through the pinned module; hook datastar-fetch instead of MutationObserver; enhance the Askama tab strip instead of rebuilding it in JS.
tags: [domain/okmate, concern/architecture, concern/rendering, concern/tooling, integration/datastar]
status: draft
generated: { by: process:cursor, at: 2026-09-11T08:31:00Z }
stale_after: 2026-12-11
authority: exploratory
owners: [human:nils]
sources:
  - id: research
    resource: ../../research/okmate/datastar-client-js.md
    title: Client-side JavaScript for the Datastar okmate viewer
    author: process:cursor
    last_modified: 2026-09-11
  - id: leptos
    resource: ../../research/okmate/leptos.md
    title: Leptos instead of Askama and Datastar
    author: process:cursor
    last_modified: 2026-09-11
  - id: extract
    resource: ../okf/okmate.md
    title: Okmate extractable Rust OKF mate
    author: process:cursor
    last_modified: 2026-09-11
  - id: peek-plan
    resource: peek-and-tabs.md
    title: Viewer peek previews and document tabs
    author: process:cursor
    last_modified: 2026-09-11
  - id: overview
    resource: ../../architecture/system-overview.md
    title: OKMate system overview
    author: process:cursor
    last_modified: 2026-08-30
  - id: readme
    resource: ../../../README.md
    title: Published OKMate stack
    author: process:git
    last_modified: 2026-09-10
  - id: base
    resource: ../../../templates/base.html
    title: Shell landmarks and staged scripts
    author: process:git
    last_modified: 2026-09-11
  - id: main-frag
    resource: ../../../templates/fragments/main.html
    title: Datastar patch is main plus toc
    author: process:git
    last_modified: 2026-09-11
  - id: pages
    resource: ../../../src/http/pages.rs
    title: Datastar GET PatchElements
    author: process:git
    last_modified: 2026-09-11
  - id: prefs
    resource: ../../../src/http/prefs.rs
    title: Prefs POST JSON 204
    author: process:git
    last_modified: 2026-09-11
  - id: site
    resource: ../../../src/site.rs
    title: include_str asset emit
    author: process:git
    last_modified: 2026-09-11
  - id: session
    resource: ../../../src/session.rs
    title: Session tabs and open_path
    author: process:git
    last_modified: 2026-09-11
  - id: nav-js
    resource: ../../../assets/nav.js
    title: Hidden-button @get and MutationObserver hub
    author: process:git
    last_modified: 2026-09-11
  - id: tabs-js
    resource: ../../../assets/tabs.js
    title: Client tab strip rebuild
    author: process:git
    last_modified: 2026-09-11
  - id: goto-js
    resource: ../../../assets/goto.js
    title: Cmd-K location.assign
    author: process:git
    last_modified: 2026-09-03
  - id: datastar-js
    resource: ../../../assets/datastar.js
    title: Pinned Datastar 1.0.2 public exports
    author: process:git
    last_modified: 2026-08-26
  - id: nav-test
    resource: ../../../tests/navigation.rs
    title: nav.js requestDocument string asserts
    author: process:git
    last_modified: 2026-09-11
  - id: prefs-test
    resource: ../../../tests/prefs.rs
    title: Tab strip omitted from Datastar fragment
    author: process:git
    last_modified: 2026-09-11
  - id: ds-actions
    resource: https://data-star.dev/reference/actions
    title: Datastar actions and datastar-fetch
    author: organization:starfederation
    last_modified: 2026-09-11
---

# Datastar-aligned client JavaScript

## Goal

Make live-preview chrome scripts **Datastar clients**, not a second renderer: ES modules that import the pinned `datastar.js` public API, subscribe to `datastar-fetch`, and enhance Askama HTML that stays outside the main/toc morph.[^research][^extract][^pages]

## Out of bound

TypeScript, `package.json`, esbuild/tsc, checked-in compiled JS. Datastar signals as the tab or document store. Patching `#okmate-tabs` or `#okmate-nav` from the document GET. Replacing Askama or Datastar (Leptos, WASM, Rocket, web-component rewrite of every widget). Peek JSON → HTML fragment. Review/log `fetch`+`replaceWith` → Datastar window patches. Changing `okf`. Product tab gestures from [peek-and-tabs](peek-and-tabs.md).[^leptos][^peek-plan][^overview]

## Constraints that do not move

- HTML and JS stay in this crate. `include_str!` remains the emit path. CI stays native Cargo plus `okmate check`.[^site][^readme][^overview]
- Landmarks `#okmate-nav`, `#okmate-main`, `#okmate-toc`. Tab strip stays a sibling **outside** `#okmate-main`. Datastar GET still patches main and toc only.[^main-frag][^pages][^prefs-test][^peek-plan]
- Server-owned session (`session.json`). Prefs persist may stay JSON **204**. No client domain store.[^extract][^prefs][^session]
- Pinned `assets/datastar.js` is the browser Datastar. Call its exported `actions` / `action`; do not parse `data-on` onto throwaway nodes.[^datastar-js][^ds-actions][^nav-js]
- CSS prefix `okmate-`. Classic `window.__okmate*` may shrink but must not grow a new global bus.

## Product contract

Keep the hypermedia loop. Sidebar and body navigation use `@get`. Chrome JS may measure, bind, and toggle classes on mounted nodes. It must not `replaceChildren` a landmark Askama already rendered, and it must not full-load a route the Datastar shell already knows how to patch.[^research][^nav-js][^tabs-js][^goto-js]

### Phase 1 — Shared module and `@get` from JS

**Bound:** Add `assets/core.js` as `type="module"`. Export `normalizeRoute`, in-app href classification, and `requestDocument(href)` that calls `import { actions } from './datastar.js'` against a **stable** host element in `base.html` (not a per-click probe button). Convert `nav.js` to a module that imports core; delete the hidden-button path. Wire `core.js` in `templates/base.html` and `src/site.rs`. Update string asserts that name `requestDocument`.[^nav-js][^nav-test][^datastar-js][^base][^site][^ds-actions]

**Out of bound:** Tabs rewrite. Converting every remaining IIFE. TypeScript.

**Tests:** `nav.js` / `core.js` contain no `document.createElement("button")` probe for `@get`. `include_str` still sees `requestDocument`. Rendered live page includes `/__okmate/core.js` as `type="module"`. Existing Datastar fragment tests still exclude `#okmate-nav`.

**Exit:** `cargo test -p okmate --no-default-features` and `cargo fmt --all -- --check`.

**Owner:** `assets/core.js`, `assets/nav.js`, `templates/base.html`, `src/site.rs`, `tests/navigation.rs`.

### Phase 2 — `datastar-fetch` instead of the main MutationObserver hub

**Bound:** `nav.js` runs post-patch work on document `datastar-fetch` when `detail.type === "finished"` and the request was an in-app document GET (not settings/prefs). Keep `DOMContentLoaded` `enhance`. Remove the `#okmate-main` `{ childList: true }` observer that fans out toc/resize/reading/tables/meta/tabs. Local observers that only bind new article nodes (peek titles, table handles) may stay if they do not call `afterDocumentPatch`.[^nav-js][^ds-actions][^research]

**Out of bound:** Tabs HTML ownership. Review/log window fetch.

**Tests:** `nav.js` no longer constructs `new MutationObserver` on `#okmate-main` as the patch bus (string assert). `datastar-fetch` is present. Fragment still omits nav and tabs.

**Exit:** `cargo test -p okmate --no-default-features` and `cargo fmt --all -- --check`.

**Owner:** `assets/nav.js`, optional small test in `tests/navigation.rs`.

### Phase 3 — Enhance the Askama tab strip

**Bound:** `tabs.js` stops using a client array plus `replaceChildren` as the HTML owner. Full-page Askama markup remains source on load (`readDom` seed is allowed). After navigation: update `is-current` / `aria-selected`, title text, and hash attributes; insert or remove **one** tab node by cloning a `<template id="okmate-tab-template">` in `base.html` when a background tab opens and the strip was absent. Persist still through `reading.js` JSON 204. Strip still hidden with fewer than two tabs. Datastar GET still omits `#okmate-tabs`.[^tabs-js][^base][^prefs][^prefs-test][^peek-plan][^session]

**Out of bound:** Prefs POST returning a tab-strip HTML fragment. Drag-reorder, Cmd+T, OS tear-off (still peek-and-tabs out of bound).

**Tests:** Two-tab full HTML still includes the strip; Datastar fragment still does not. `tabs.js` does not call `replaceChildren` on `#okmate-tabs`. Template id present in `base.html`.

**Exit:** `cargo test -p okmate --no-default-features` and `cargo fmt --all -- --check`.

**Owner:** `assets/tabs.js`, `templates/base.html`, existing `tests/prefs.rs`.

### Phase 4 — Cmd-K joins the Datastar path

**Bound:** `goto.js` `go(route)` calls `core.requestDocument` (or `__okmateNav.openRoute`) instead of `window.location.assign`. Palette stays a client island over Askama `<dialog>` and `/pages.json`.[^goto-js][^extract][^research]

**Out of bound:** Rewriting goto ranking. TypeScript. Making `/pages.json` a Datastar signal.

**Tests:** `goto.js` does not contain `location.assign`. Live page still loads `goto.js`. Navigation tests still pass.

**Exit:** `cargo test -p okmate --no-default-features` and `cargo fmt --all -- --check`.

**Owner:** `assets/goto.js`.

## Status

Exploratory; no phase started. Research:
[client-side JavaScript for the Datastar okmate viewer](/research/okmate/datastar-client-js.md).[^research]

[^research]: Keep vanilla ES modules; no TS toolchain; enhance chrome outside the morph; import `actions` from pinned Datastar.
[^leptos]: Replacing Askama/Datastar with hydrate or islands is a product rewrite.
[^extract]: Server-owned state; no client domain store; one-shot morph; Cmd-K may be a small script that prefers Datastar.
[^peek-plan]: Strip outside `#okmate-main`; product tab gestures already specified.
[^overview]: Application crate owns HTML/JS; `okf/` UI-neutral.
[^readme]: Pinned `assets/datastar.js`; native CLI tests.
[^base]: Module Datastar plus classic defer chrome scripts; Askama tabs when `tabs.len() > 1`.
[^main-frag]: Patch template is main plus toc.
[^pages]: Datastar GET `PatchElements` of main and toc.
[^prefs]: Prefs POST 204.
[^site]: `include_str!` asset emit.
[^session]: Session tab list is durable state.
[^nav-js]: Hidden button `@get` and `MutationObserver` fan-out.
[^tabs-js]: `replaceChildren` rebuild from a JS `tabs` array.
[^goto-js]: `window.location.assign(route)`.
[^datastar-js]: v1.0.2 module exports `actions`.
[^nav-test]: String assert on `requestDocument`.
[^prefs-test]: Fragment excludes `#okmate-tabs`.
[^ds-actions]: `@get` and `datastar-fetch` finished.
