---
type: Research Report
title: Client-side JavaScript for the Datastar okmate viewer
description: Keep pinned vanilla JS as ES modules beside Askama and Datastar; do not add TypeScript or a Node toolchain; use Datastar’s public actions and fetch events instead of hidden buttons and MutationObserver; mutate the DOM only to enhance chrome and overlays that sit outside the morph.
tags: [domain/okmate, concern/architecture, concern/rendering, concern/tooling, integration/datastar]
status: draft
generated: { by: process:cursor, at: 2026-09-11T08:31:00Z }
stale_after: 2026-12-11
authority: exploratory
owners: [human:nils]
sources:
  - id: plan
    resource: ../../plans/okmate/datastar-client-js.md
    title: Datastar client-JS improvement plan
    author: process:cursor
    last_modified: 2026-09-11
  - id: overview
    resource: ../../architecture/system-overview.md
    title: OKMate system overview
    author: process:cursor
    last_modified: 2026-08-30
  - id: extract
    resource: ../../plans/okf/okmate.md
    title: Okmate extractable Rust OKF mate
    author: process:cursor
    last_modified: 2026-09-11
  - id: leptos
    resource: leptos.md
    title: Leptos instead of Askama and Datastar
    author: process:cursor
    last_modified: 2026-09-11
  - id: peek-plan
    resource: ../../plans/okmate/peek-and-tabs.md
    title: Viewer peek previews and document tabs
    author: process:cursor
    last_modified: 2026-09-11
  - id: readme
    resource: ../../../README.md
    title: Published OKMate stack
    author: process:git
    last_modified: 2026-09-10
  - id: cargo
    resource: ../../../Cargo.toml
    title: okmate package deps with no JS toolchain
    author: process:git
    last_modified: 2026-08-28
  - id: ci
    resource: ../../../.github/workflows/ci.yml
    title: Native cargo test without Node or wasm
    author: process:git
    last_modified: 2026-08-27
  - id: base
    resource: ../../../templates/base.html
    title: Shell landmarks and staged scripts
    author: process:git
    last_modified: 2026-09-11
  - id: nav-macros
    resource: ../../../templates/nav_macros.html
    title: Sidebar links issue Datastar @get
    author: process:git
    last_modified: 2026-09-11
  - id: main-frag
    resource: ../../../templates/fragments/main.html
    title: Datastar patch is main plus toc
    author: process:git
    last_modified: 2026-09-11
  - id: pages
    resource: ../../../src/http/pages.rs
    title: Datastar GET PatchElements of main and toc
    author: process:git
    last_modified: 2026-09-11
  - id: prefs
    resource: ../../../src/http/prefs.rs
    title: Prefs POST persists JSON and returns 204
    author: process:git
    last_modified: 2026-09-11
  - id: site
    resource: ../../../src/site.rs
    title: include_str asset emit including tabs.js
    author: process:git
    last_modified: 2026-09-11
  - id: session
    resource: ../../../src/session.rs
    title: Session tabs, open_path, open_hash
    author: process:git
    last_modified: 2026-09-11
  - id: views
    resource: ../../../src/views/mod.rs
    title: Document tabs and render_main_fragment
    author: process:git
    last_modified: 2026-09-11
  - id: tabs-js
    resource: ../../../assets/tabs.js
    title: Client tab strip rebuild
    author: process:git
    last_modified: 2026-09-11
  - id: nav-js
    resource: ../../../assets/nav.js
    title: Keep-nav, hidden-button @get, MutationObserver hub
    author: process:git
    last_modified: 2026-09-11
  - id: peek-js
    resource: ../../../assets/peek.js
    title: Hover peek JSON overlay
    author: process:git
    last_modified: 2026-09-11
  - id: goto-js
    resource: ../../../assets/goto.js
    title: Cmd-K palette; location.assign navigation
    author: process:git
    last_modified: 2026-09-03
  - id: reading-js
    resource: ../../../assets/reading.js
    title: Reading prefs persist via fetch JSON
    author: process:git
    last_modified: 2026-08-29
  - id: tables-js
    resource: ../../../assets/tables.js
    title: Table column-resize handles
    author: process:git
    last_modified: 2026-08-29
  - id: review-js
    resource: ../../../assets/review.js
    title: Review window fetch-and-replaceWith
    author: process:git
    last_modified: 2026-08-27
  - id: datastar-js
    resource: ../../../assets/datastar.js
    title: Pinned Datastar 1.0.2 module with public actions export
    author: process:git
    last_modified: 2026-08-26
  - id: prefs-test
    resource: ../../../tests/prefs.rs
    title: Two-tab session strip stays out of the Datastar patch
    author: process:git
    last_modified: 2026-09-11
  - id: nav-test
    resource: ../../../tests/navigation.rs
    title: nav.js string asserts for requestDocument
    author: process:git
    last_modified: 2026-09-11
  - id: ds-js
    resource: https://data-star.dev/guide/using_javascript
    title: Datastar guide Using JavaScript
    author: organization:starfederation
    last_modified: 2026-09-11
  - id: ds-actions
    resource: https://data-star.dev/reference/actions
    title: Datastar actions reference including datastar-fetch
    author: organization:starfederation
    last_modified: 2026-09-11
  - id: ds-plugin
    resource: https://data-star.dev/examples/custom_plugin
    title: Datastar custom plugin example
    author: organization:starfederation
    last_modified: 2026-09-11
  - id: ds-docs
    resource: https://data-star.dev/docs.md
    title: Datastar concatenated docs (response types)
    author: organization:starfederation
    last_modified: 2026-09-11
---

# Client-side JavaScript for the Datastar okmate viewer

## Claim

Okmate’s live viewer is a **hypermedia shell**: Askama owns HTML, Datastar morphs `#okmate-main` and `#okmate-toc`, and small staged scripts enhance chrome that the morph must not remount.[^readme][^pages][^main-frag][^overview] That split is still the right architecture. The problem is not “too much JavaScript” or “not TypeScript.” It is that the chrome scripts **reimplement Datastar** (hidden buttons, MutationObserver-as-SSE, client HTML rebuilds) instead of using Datastar’s public module API and treating Askama markup as the HTML owner.[^nav-js][^tabs-js][^datastar-js][^ds-js]

Do **not** migrate `assets/*.js` to TypeScript. Do **not** put document or tab state in Datastar signals. Do use **vanilla ES modules** that `import { actions }` from the pinned `datastar.js`, listen for `datastar-fetch`, and mutate the DOM only where the morph does not own the node.[^ds-actions][^extract][^leptos]

This record is exploratory. It does not mint a Decision. Implementation sequence: [datastar-client-js plan](/plans/okmate/datastar-client-js.md).[^plan]

## Current setup

Published stack: Askama 0.16, Axum 0.8, official Datastar Rust SDK 0.4 plus pinned `assets/datastar.js` v1.0.2 (`type="module"`), optional `h35-desktop`. There is no `package.json`, no TypeScript, no bundler. CI is native `cargo test` and `okmate check`. `src/site.rs` embeds every chrome file with `include_str!` and writes `/__okmate/*.js`.[^readme][^cargo][^ci][^site][^datastar-js][^base]

First paint is a full document. A Datastar GET returns an SSE `PatchElements` of main plus toc and **does not** include `#okmate-nav` or `#okmate-tabs`. Session `tabs` seed the strip on full load when there are two or more; prefs POST is JSON and **204** (no morph).[^pages][^prefs][^prefs-test][^session][^views]

Uncompressed chrome JS excluding Datastar is about 82 KiB across twelve IIFEs. `nav.js` (17 KiB) and `goto.js` (14 KiB) dominate. `goto.js` already uses `const`/`let`; most others are `var` IIFEs hanging `window.__okmate*` globals.[^nav-js][^goto-js][^tabs-js]

## What Datastar actually wants from browser code

Datastar is the **browser transport**, not a client framework. Reactive signals and `@get` / `@post` only run inside `data-*` expressions. Logic that cannot live there belongs in **external scripts** or **web components**, with **props down, events up**. If a Datastar expression is doing too much, it is overcomplicated.[^ds-js][^extract]

Shipped response types the pinned client understands:[^ds-docs][^datastar-js]

| Server sends | Datastar does |
| --- | --- |
| `text/event-stream` | patch-elements / patch-signals events |
| `text/html` | Morph top-level elements by `id` |
| `application/json` | **Patch signals**, not the DOM |
| `text/javascript` | Execute script |
| `204` | Success, no morph |

`application/json` from a Datastar action therefore cannot fill a peek card or a tab strip. JSON that is meant to become HTML must be ordinary `fetch`, or the endpoint must return HTML/SSE instead.[^ds-docs][^peek-js]

The pinned module **exports** a public API (`action`, `actions`, `attribute`, `watcher`, `root`, …). `@get` is `actions.get`. Fetch lifecycle is the document event `datastar-fetch` with `detail.type` `started` / `finished` / `error`. Custom `action({ name, apply })` plugins are the documented extension point. None of this requires TypeScript at the call site; Datastar’s own sources are TS, the shipped asset is JS.[^datastar-js][^ds-plugin][^ds-actions]

## Classification of today’s scripts

| Script | Role | Owns HTML? | Talks to Datastar? | Verdict |
| --- | --- | --- | --- | --- |
| `nav.js` | Keep-nav, history, in-app clicks, post-patch hub | Restyles nav that Askama already rendered | Hidden button with `data-on:click="@get(…)"`; `MutationObserver` on `#okmate-main` then fans out `enhance` | Wrong Datastar adapter; right place for keep-nav |
| `tabs.js` | Open-document strip | **Rebuilds** the strip with `createElement` / `replaceChildren` after reading a JS `tabs` array | Called from nav’s observer; persist via `reading.js` JSON | Dual HTML owner; see below |
| `peek.js` | Hover overlay | Creates `#okmate-peek` in JS | Ordinary `fetch` JSON, not `@get` | Correct for an overlay; JSON is required unless peek becomes HTML |
| `reading.js` | Font/width/pane prefs | Sets CSS variables / `data-*` on `<html>` | `fetch` POST `/__okmate/prefs` | Correct chrome island; 204 is the right prefs response[^reading-js] |
| `goto.js` | Cmd-K | Fills an Askama `<dialog>` | `fetch /pages.json`; **`location.assign`** | Palette is a fine island; navigation should join the Datastar path |
| `toc.js` / `resize.js` / `tables.js` / `meta.js` | Enhance patched or chrome DOM | Handles, spy, timestamps | Re-run after nav’s observer | Direct mutation of existing nodes is correct[^tables-js] |
| `review.js` / `log.js` | Windowed lists | `DOMParser` + `replaceWith` | Ordinary `fetch` of HTML, not Datastar morph | DIY morph; candidate to become `@get` of a fragment |
| `reload.js` | Workspace reload prompt | Fills Askama `<dialog>` | `EventSource` + POST | Fine as a small island |

The extract plan already said Cmd-K may be a small script and should prefer Datastar for navigation. `goto.js` still does a full document load.[^extract][^goto-js][^nav-macros]

## TypeScript versus raw JS

**Keep raw JS.** A TS migration would add a Node toolchain, an emit step, source maps, and a second language to a crate whose contract is `include_str!` plus native CI. That is the same class of dual-compile cost the Leptos research declined for WASM hydrate.[^leptos][^ci][^site][^cargo]

Types would not catch the real bugs here (morph identity, observer re-entry, module order, 204 vs fragment). Datastar plugins *can* be authored in TS with a bundler; okmate is not shipping a Datastar plugin registry, it is enhancing one pinned hypermedia app.[^ds-plugin]

Acceptable type-adjacent steps that stay inside Cargo:

- Convert chrome files to **ES modules** (`type="module"`) so they can import `./datastar.js` and a shared `core.js`.
- Deduplicate `normalizeRoute` / in-app href checks (copied in `nav.js`, `tabs.js`, `peek.js`).[^nav-js][^tabs-js][^peek-js]
- Optional `// @ts-check` JSDoc later; not a phase gate and not a `tsc` CI job.

Do not introduce `package.json`, esbuild, or checked-in compiled output next to the sources.

## Direct DOM mutation versus morph

Direct DOM mutation is the **right** tool when all of these hold:

1. The node sits **outside** the Datastar patch (`#okmate-nav`, `#okmate-tabs`, `#okmate-toolbar`, peek card, dialogs), **or** it is a transient handle on an already-morphed node (table resizers, outline spy).
2. Askama (or the last morph) already supplied the meaningful HTML; JS only restyles, measures, or binds.
3. Durable state still lands in `session.json` / workspace, not in a client store that other tabs must read.[^extract][^session][^prefs]

Direct DOM mutation is the **wrong** tool when JS becomes a second template engine for markup Askama already knows how to render.

`tabs.js` is the example. Full GET renders `#okmate-tabs` from session. Datastar GET correctly omits it so the strip survives. After every patch, JS upserts a client `tabs` array, `replaceChildren()`s the strip, and POSTs JSON. First paint is server HTML; every later navigation is a client clone of that HTML. `readDom()` then treats the clone as source. That is split-brain, not “Datastar architecture.”[^tabs-js][^base][^prefs-test][^peek-plan]

Keep the strip **outside** the main/toc morph (already tested). For open/close/activate:

- **Enhance, don’t rebuild.** Seed from Askama. After navigation, toggle `is-current`, update title/hash attributes, insert or remove one tab node (clone a `<template>` in `base.html` if a node must be created). Persist still JSON + 204; the strip is chrome latency, not a morph target.
- **Do not** PATCH `#okmate-tabs` from the same document GET that patches main (would remount the strip and fight keep-nav). A *separate* prefs-POST fragment is possible later; it is not required to stop `replaceChildren`.
- **Do not** put the tab list in `$signals`. Signals are ephemeral UI, and JSON Datastar responses patch signals rather than `#okmate-tabs`.[^ds-docs][^extract]

`nav.js` already shows the good pattern: Askama rendered the sidebar; JS only marks current and restores `<details>`. The bad part is the adapter: creating a hidden `<button data-on:click="@get('…')">` because classic scripts cannot import `actions`, and watching `childList` on `#okmate-main` because the code never subscribed to `datastar-fetch`. Tests currently assert the string `requestDocument` exists.[^nav-js][^nav-test][^ds-actions]

## Recommended contract

1. **HTML owner:** Askama templates. Datastar morphs `#okmate-main` and `#okmate-toc` only. Chrome landmarks stay mounted.[^main-frag][^pages]
2. **Transport:** `@get` / `@post` from `data-*` or from `import { actions } from './datastar.js'`. No synthetic elements to parse expressions.[^datastar-js][^ds-plugin]
3. **After patch:** `datastar-fetch` `finished` (filter to document GETs) plus `DOMContentLoaded`. Not `MutationObserver` on main as a global bus. Table/peek observers may remain local if they only bind new article nodes.[^ds-actions][^nav-js]
4. **JSON:** prefs persist and peek overlays. JSON is not a DOM patch. Windowed review/log HTML should eventually be Datastar fragments; that is a follow-on, not a tabs fix.[^prefs][^peek-js][^review-js]
5. **Language:** ES modules in `assets/`, `include_str!`, no TypeScript toolchain.[^site][^ci]
6. **Globals:** shrink `window.__okmate*` to a documented module surface (`requestDocument`, `enhance`) as files convert; do not grow a client domain model behind them.

## Out of this record

Executing the plan, minting a Decision, converting peek to an HTML fragment, rewriting review/log windowing, or adopting web components / Datastar Rocket. Rocci island research is a sibling product; okmate must not depend on `rocci-*`.[^overview][^leptos]

[^plan]: Paired implementation sequence; writing it is not executing it.
[^overview]: Application crate owns Askama, Axum, Datastar, desktop; `okf/` stays UI-neutral.
[^extract]: Server-owned state; no client domain store; one-shot morph first; Cmd-K may be a small script that prefers Datastar.
[^leptos]: Keep Askama + Datastar; dual compile / WASM is a product rewrite.
[^peek-plan]: Tab strip is a sibling outside `#okmate-main`; Datastar still patches main and toc only.
[^readme]: Stack table: Askama, Axum, Datastar 0.4, pinned `assets/datastar.js`.
[^cargo]: No JS package; `datastar` crate with `axum` feature.
[^ci]: `cargo test` and knowledge check; no Node job.
[^base]: `datastar.js` is `type="module"`; other scripts are classic `defer`; Askama tab strip when `tabs.len() > 1`.
[^nav-macros]: Sidebar `data-on:click__prevent="@get('…')"`.
[^main-frag]: Fragment template is `<main id="okmate-main">` plus toc include.
[^pages]: Datastar GET `PatchElements` of `render_main_fragment`; persist open path.
[^prefs]: Loopback JSON POST; `204 No Content`.
[^site]: `include_str!` for every chrome JS file including `tabs.js`.
[^session]: Session tab list and `open_path` / `open_hash`.
[^views]: `Document.tabs` and `render_main_fragment`.
[^tabs-js]: Client `tabs` array, `ensureStrip`, `replaceChildren`, `readDom`, persist through `__okmateReading`.
[^nav-js]: Hidden probe button; `MutationObserver` `{ childList: true }` on `#okmate-main`; fan-out to toc/resize/reading/tables/meta/tabs.
[^peek-js]: `GET /__okmate/peek` JSON; card constructed in JS; `pointer-events` overlay.
[^goto-js]: `fetch("/pages.json")`; `window.location.assign(route)`.
[^reading-js]: Prefs fetch JSON; CSS variables on `documentElement`.
[^tables-js]: Appends resizer handles under existing table wrappers.
[^review-js]: `fetch` HTML, `DOMParser`, `replaceWith` on `#okmate-review-window`.
[^datastar-js]: Header `Datastar v1.0.2`; `export { … action, actions, watcher … }`.
[^prefs-test]: Two-tab full HTML includes `#okmate-tabs`; Datastar fragment does not.
[^nav-test]: `include_str` asserts `requestDocument` in `nav.js`.
[^ds-js]: Most logic in `data-*`; otherwise external scripts or web components; props down, events up.
[^ds-actions]: `@get`; `datastar-fetch` started/finished/error.
[^ds-plugin]: `action({ name, apply })` then `@name` in expressions.
[^ds-docs]: HTML morphs DOM; JSON patches signals; 204 is no morph.
