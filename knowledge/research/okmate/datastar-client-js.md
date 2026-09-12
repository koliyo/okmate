---
type: Research Report
title: Datastar fit and client-shell architecture for Okmate
description: Datastar suits document delivery; growing shell state warrants comparing explicit vanilla ownership with a component shell while retaining the Rust engine and static export.
tags: [domain/okmate, concern/architecture, concern/rendering, concern/tooling, integration/datastar]
status: draft
generated: { by: process:cursor, at: 2026-09-12T12:29:06Z }
stale_after: 2026-12-12
authority: exploratory
owners: [human:nils]
sources:
  - id: overview
    resource: "../../architecture/system-overview.md"
    title: "Current application and engine boundaries"
    author: process:git
  - id: plan
    resource: "../../plans/okmate/datastar-client-js.md"
    title: "Earlier bounded Datastar cleanup plan"
    author: process:git
  - id: extract
    resource: "../../plans/okf/okmate.md"
    title: "Exploratory extraction design"
    author: process:git
  - id: leptos
    resource: "leptos.md"
    title: "Earlier Leptos comparison and revised qualifications"
    author: process:git
  - id: readme
    resource: "../../../README.md"
    title: "Application stack and CLI"
    author: process:git
  - id: okf
    resource: "../../../okf/README.md"
    title: "Portable engine and article HTML"
    author: process:git
  - id: core
    resource: "../../../assets/core.js"
    title: "Datastar action adapter"
    author: process:git
  - id: nav
    resource: "../../../assets/nav.js"
    title: "Navigation and document completion coordination"
    author: process:git
  - id: tabs
    resource: "../../../assets/tabs.js"
    title: "Tab state and incremental DOM reconciliation"
    author: process:git
  - id: goto
    resource: "../../../assets/goto.js"
    title: "Command palette and navigation"
    author: process:git
  - id: reading
    resource: "../../../assets/reading.js"
    title: "Reading state and shared persistence timer"
    author: process:git
  - id: peek
    resource: "../../../assets/peek.js"
    title: "JSON preview and overlay positioning"
    author: process:git
  - id: review
    resource: "../../../assets/review.js"
    title: "Review window fetch and replacement"
    author: process:git
  - id: log-js
    resource: "../../../assets/log.js"
    title: "Log window fetch and replacement"
    author: process:git
  - id: base
    resource: "../../../templates/base.html"
    title: "Initial shell markup and script loading"
    author: process:git
  - id: main
    resource: "../../../templates/fragments/main.html"
    title: "Main and table-of-contents patch boundary"
    author: process:git
  - id: pages
    resource: "../../../src/http/pages.rs"
    title: "Full-page, fragment, and JSON HTTP responses"
    author: process:git
  - id: prefs
    resource: "../../../src/http/prefs.rs"
    title: "Loopback session persistence endpoint"
    author: process:git
  - id: site
    resource: "../../../src/site.rs"
    title: "Embedded assets, static pages, and preview shell"
    author: process:git
  - id: desktop
    resource: "../../../src/desktop.rs"
    title: "Native host and IPC"
    author: process:git
  - id: nav-tests
    resource: "../../../tests/navigation.rs"
    title: "HTTP navigation and JavaScript source assertions"
    author: process:git
  - id: prefs-tests
    resource: "../../../tests/prefs.rs"
    title: "Session and fragment boundary tests"
    author: process:git
  - id: ci
    resource: "../../../okmate-ops/src/okmate_ops/ci.py"
    title: "Maintainer validation commands"
    author: process:git
  - id: datastar
    resource: "https://data-star.dev/guide/datastar_expressions"
    title: "Datastar expressions, external scripts, and web components"
    author: process:cursor
  - id: svelte
    resource: "https://svelte.dev/docs/svelte/overview"
    title: "Svelte compiler and standalone components"
    author: process:cursor
  - id: svelte-mount
    resource: "https://svelte.dev/docs/svelte/imperative-component-api"
    title: "Svelte mounting and hydration"
    author: process:cursor
  - id: svelte-state
    resource: "https://svelte.dev/docs/svelte/$state"
    title: "Reactive state and sharing between modules"
    author: process:cursor
  - id: vue
    resource: "https://vuejs.org/guide/extras/ways-of-using-vue.html"
    title: "Vue standalone, embedded, and application modes"
    author: process:cursor
  - id: leptos-islands
    resource: "https://book.leptos.dev/islands.html"
    title: "Leptos interactive islands and server-only children"
    author: process:cursor
  - id: leptos-build
    resource: "https://book.leptos.dev/ssr/21_cargo_leptos.html"
    title: "Native and WASM compilation"
    author: process:cursor
  - id: typescript
    resource: "https://www.typescriptlang.org/docs/handbook/intro.html"
    title: "Static type checking for JavaScript programs"
    author: process:cursor
---

# Datastar fit and client-shell architecture for Okmate

## Recommendation and authority

Datastar remains a reasonable fit for server-rendered documents. Okmate's
stateful shell deserves a separate architecture evaluation: tabs, history,
scroll restoration, the command palette, and pane preferences already have
client-side state and coordination. The maintenance question is who owns that
state and its DOM, not whether custom JavaScript exists.[^pages][^nav][^tabs][^reading]

Retain the Rust engine, Axum host, document rendering, and static export.
Compare a deliberately organized vanilla shell with a small Svelte + TypeScript
shell before committing to migration. Svelte is the preferred experiment here,
not a measured winner; Vue is a credible alternative. A bounded component shell
has a stronger rationale if interactive workspace features continue growing.
If the product remains mainly a document reader, improving the existing shell
may cost less overall.[^okf][^site][^svelte][^vue]

This is exploratory research, revised on 2026-09-12. It supersedes this record's
blanket prohibition on TypeScript and component frameworks. It does not approve
a migration or execute a prototype. The [earlier cleanup plan](/plans/okmate/datastar-client-js.md)
retains its original scope; its exclusions are not permanent product decisions.
The [extraction plan](/plans/okf/okmate.md) is also marked exploratory, while the
[system overview](/architecture/system-overview.md) describes the implemented
stack.[^plan][^extract][^overview]

## Evidence and limits

Read-only implementation review used checkout `1941e0f` on 2026-09-12. Existing
uncommitted work was in unrelated knowledge records. The source census was
`wc -l -c assets/*.js`: 13 custom files, 3,292 physical lines and 91,480 bytes
(89.3 KiB), excluding the vendored Datastar asset. These totals include comments
and whitespace, are not executable-line counts, and say nothing about runtime
performance. Four files dominate: nav 593 lines, tabs 615, goto 502, and reading
373 (2,083 lines combined). Sources and embedding were inspected directly.[^site][^nav][^tabs][^goto][^reading]

This review also inspected HTTP and source-assertion tests. Those demonstrate
fragment shape and implementation guardrails; they do not establish browser
correctness under overlapping navigation, focus changes, or session reload.
No browser benchmark or alternative-framework prototype was run for this
revision. Claimed savings below are hypotheses, not measured results.[^nav-tests][^prefs-tests]

## Current implementation, corrected from the earlier report

| Surface | Observed behavior | Architectural implication |
| --- | --- | --- |
| Documents | Askama full-page HTML; Datastar GET returns one SSE patch containing main and TOC | Server rendering remains useful; the document GET is not a long-lived application-state stream[^pages][^main] |
| Shell | Sidebar and tab strip remain outside the document patch | There is already a boundary where independent client ownership can be evaluated[^base][^main][^prefs-tests] |
| Navigation | `core.js` calls exported `actions.get`; `nav.js` handles `datastar-fetch` completion | Hidden-button requests and the main MutationObserver completion hub are historical, not current defects[^core][^nav] |
| Tabs | Client arrays, metadata requests, template clones, incremental node updates, and session persistence | Whole-strip `replaceChildren` rebuilding was removed; manual state-to-DOM reconciliation remains[^tabs][^base] |
| Palette | Fetches `/pages.json`, renders local results, navigates through `__okmateNav.openRoute` | Earlier claims that it uses `location.assign` are obsolete[^goto] |
| Preferences | Client reading state posts JSON; server returns 204 | Local interaction state and server persistence already coexist[^reading][^prefs] |
| Review/log | Fetch HTML, parse it, replace window nodes, and use local observers | A second update lifecycle remains alongside document morphing[^review][^log-js] |
| Peek | Fetches JSON and manages an overlay | Local rendering is a reasonable choice for a transient preview[^peek][^pages] |

Datastar itself includes reactivity as well as transport and morphing. Calling it
only a transport library was too narrow. Its documentation explicitly supports
external scripts and custom elements for behavior that does not belong in data
expressions. It recommends encapsulated inputs and events. This supports a
hybrid design; it does not require that every widget become a server request.[^datastar]

## Where the complexity comes from

Some browser work is inherent: measuring layout, positioning previews, handling
keyboard gestures, and restoring scroll. Changing frameworks will not remove
those product requirements. State-to-DOM bookkeeping and cross-module lifecycle
coordination are more plausible targets for reduction.[^peek][^tabs][^nav]

`afterDocumentPatch` coordinates route/history, nav expansion, scroll, title,
location persistence, and six module callbacks. Request classification examines
attributes to infer whether a completed Datastar request is document navigation.
This couples application lifecycle to both HTML conventions and transport
completion. The remaining `window.__okmate*` calls make dependencies implicit.
These are maintainability findings from source inspection, not proof of a
specific observed navigation failure.[^nav]

`tabs.js` maintains state and manually updates node attributes, titles, colors,
and membership. Its incremental updates are an improvement, but template cloning
does not eliminate the need to reconcile state with DOM. An explicit client owner
would make that responsibility easier to identify whether implemented with
vanilla modules or a component framework.[^tabs]

There is also a concrete persistence concern independent of Datastar:
`reading.persist(extra)` creates a payload per call and cancels a single shared
timer. A later call with different extra fields can replace an earlier pending
payload rather than merge it. Nav and tabs call this common function. The source
therefore permits an unsent partial update to be dropped; impact on actual saved
sessions needs a behavioral reproduction. A framework does not automatically fix
this queueing policy.[^reading][^nav][^tabs]

## Alternatives worth comparing

The following ranking is an architectural judgment for Okmate, not a general
framework ranking.

| Option | Expected benefit | Remaining cost | Assessment |
| --- | --- | --- | --- |
| Askama + Datastar + explicit vanilla modules | Preserves build workflow; clarifies state, events, persistence, and patch lifecycle | Manual list reconciliation and browser coordination remain | Baseline to beat |
| Datastar + small custom elements | Encapsulates isolated widgets with attributes/events | Cross-widget workspace state still needs an owner; custom elements alone do not supply a shared application model | Suitable for isolated controls[^datastar] |
| Svelte + TypeScript shell; Rust documents | Declarative components and reactive shell state can replace manual tab DOM updates | Frontend compilation, integration boundary, persistence and navigation design | Preferred bounded experiment[^svelte][^svelte-state] |
| Vue shell; Rust documents | Incremental component adoption, including standalone script use | Same ownership questions; a rich typed shell still needs deliberate tooling | Credible alternative if its authoring model is preferred[^vue] |
| Leptos client shell/islands | Rust component model and selective browser interactivity | WASM build, browser bindings, integration and packaging work | Viable when Rust UI authoring is a positive requirement[^leptos-islands][^leptos-build] |
| Full client application | One UI owner for a substantially richer workspace | Broad API, route, template, and export migration | More scope than the current evidence warrants |

Simply substituting another HTML-request library would leave the tab state,
scroll, persistence, and lifecycle responsibilities largely intact. The proposed
comparison should change ownership and rendering responsibilities, not just
request syntax.[^nav][^tabs][^reading]

TypeScript is an independent choice from Datastar. Static checking could expose
inconsistent tab/session payload shapes and module interfaces; it cannot prove
DOM identity, event ordering, or correct scroll restoration. The earlier claim
that types would not catch relevant bugs was too absolute. Adding a compiler is
a build cost, not a reason to reject useful checking without evaluation.[^typescript][^tabs][^reading]

## Proposed ownership for a component-shell experiment

| Responsibility | Proposed owner |
| --- | --- |
| Bundle parsing, validation, search, and canonical content | Existing native `okf` engine |
| Document body rendering and static HTML export | Existing Rust application and Askama |
| Active document, tab list, navigation intent, palette, pane state | One client shell state model |
| Browser history, focus, and scroll application | Shell navigation controller with explicit document-completion events |
| Session persistence | Client snapshot/queue, server storage; distinguish pending changes from acknowledged state |
| Document transport and patching during an experiment | One adapter using existing Datastar document patches |
| Desktop menu and folder integration | Existing native host and IPC, adapted to the shell boundary |

These are proposed responsibilities, not implemented changes. They preserve
existing engine and host boundaries while making session-state ownership more
explicit.[^okf][^site][^desktop][^prefs]

Initially mount components into dedicated shell roots outside `#okmate-main` and
`#okmate-toc`. Svelte supports mounting to a target element; do not assume its
hydration API can adopt arbitrary Askama markup. Seed state through a defined
bootstrap payload, then give the mounted component exclusive ownership of its
root. Retire the corresponding legacy listeners and DOM updates for each migrated
root. Keep document link handling behind one navigation adapter.[^svelte-mount][^base][^main][^nav]

Datastar may remain the document adapter if the boundary stays simple. A later
whole-shell component may instead contain an explicitly opaque document host;
that integration must preserve the host and prevent either renderer from
reconciling the other's children. Wrapping the existing patch targets in ordinary
reactive component markup without an ownership contract would reproduce the
current coordination problem. No renderer replacement is required to test tabs.

Do not send every scroll or tab-selection event to the server to make the system
appear server-owned. Keep transient interaction state local and persist a merged
snapshot or ordered updates. Request cancellation, stale-response rejection,
save ordering, and failure behavior remain explicit application responsibilities.
The prototype should test them rather than assume component reactivity supplies
them.[^nav][^reading]

## Packaging, static export, and Leptos qualifications

A frontend build does not inherently require a Node runtime in the shipped
application. Svelte compiles components to JavaScript; generated assets could be
embedded and emitted by the existing native asset pipeline. This is a feasible
integration direction inferred from Svelte's compiler and Okmate's `include_str!`
pipeline, not a packaging path validated here. Reproducible compilation, asset
inclusion, source maps, clean-checkout builds, and release CI would need a defined
policy.[^svelte][^site][^ci]

`okmate build` and live `view` already have different rendering paths: static
per-route HTML versus a preview shell and in-memory HTTP rendering. A component
shell need not eliminate static exports. Retaining Askama exports does mean
maintaining a second presentation path, so shared style/data contracts and export
checks become part of the tradeoff. Test exports on a plain file server; do not
assume live session, settings, and workspace endpoints exist there.[^site][^pages][^readme]

The [earlier Leptos analysis](/research/okmate/leptos.md) overstates several
constraints. Client components do not force `okf` into WASM if only native
endpoints load bundles. WASM/JS assets do not inherently force a new desktop host
or multiple distributed files; embedding remains possible. Leptos islands can
accept server-rendered children. Its genuine additional cost is the native/WASM
build and integration work. A preserved Askama export is also possible, although
it introduces two presentation paths. Neither this revision nor the earlier
report benchmarks an Okmate Leptos implementation.[^leptos][^leptos-islands][^leptos-build][^site][^desktop]

## Evidence needed before choosing

A future evaluation should compare the same tab/navigation behavior in the
vanilla baseline and a bounded component implementation. This is an experiment
design, not a phased implementation plan or authorization to start one.

- Exercise foreground and background tab opening, close/reopen, palette
  navigation, same-document hashes, browser back/forward, title/color updates,
  session reload, and tab actions while document requests overlap.
- Verify that scroll and focus survive document changes appropriately, that
  one user action commits one navigation, and that failed or superseded requests
  cannot commit stale shell state.
- Exercise overlapping preference updates and persistence failure. Inspect the
  saved session after reload rather than checking only a 204 response.
- Compare explicit state owners, cross-module calls, lifecycle hooks, and the
  amount of manual DOM reconciliation removed. Count total maintained source,
  templates, build configuration, and tests, not just JavaScript line reduction.
- Measure first paint and tab-switch latency, asset size, and repeated-navigation
  behavior in the browser and the actual desktop webview. Check clean builds and
  plain-server static exports.

These scenarios target responsibilities found in the current source and gaps
left by the inspected HTTP/source assertions.[^tabs][^goto][^nav][^reading][^nav-tests][^prefs-tests]

Prefer the component direction only if it removes coordination responsibilities
while preserving behavior at an acceptable build and export cost. If it merely
wraps the existing globals or adds a second rendering lifecycle, keep Datastar
and improve vanilla ownership. No performance improvement, defect reduction, or
migration completion is claimed by this research.

[^overview]: Current application and engine boundaries; inspected for this revision.
[^plan]: Earlier bounded Datastar cleanup plan; inspected for this revision.
[^extract]: Exploratory extraction design; inspected for this revision.
[^leptos]: Earlier Leptos comparison and revised qualifications; inspected for this revision.
[^readme]: Application stack and CLI; inspected for this revision.
[^okf]: Portable engine and article HTML; inspected for this revision.
[^core]: Datastar action adapter; inspected for this revision.
[^nav]: Navigation and document completion coordination; inspected for this revision.
[^tabs]: Tab state and incremental DOM reconciliation; inspected for this revision.
[^goto]: Command palette and navigation; inspected for this revision.
[^reading]: Reading state and shared persistence timer; inspected for this revision.
[^peek]: JSON preview and overlay positioning; inspected for this revision.
[^review]: Review window fetch and replacement; inspected for this revision.
[^log-js]: Log window fetch and replacement; inspected for this revision.
[^base]: Initial shell markup and script loading; inspected for this revision.
[^main]: Main and table-of-contents patch boundary; inspected for this revision.
[^pages]: Full-page, fragment, and JSON HTTP responses; inspected for this revision.
[^prefs]: Loopback session persistence endpoint; inspected for this revision.
[^site]: Embedded assets, static pages, and preview shell; inspected for this revision.
[^desktop]: Native host and IPC; inspected for this revision.
[^nav-tests]: HTTP navigation and JavaScript source assertions; inspected for this revision.
[^prefs-tests]: Session and fragment boundary tests; inspected for this revision.
[^ci]: Maintainer validation commands; inspected for this revision.
[^datastar]: Datastar expressions, external scripts, and web components; inspected for this revision.
[^svelte]: Svelte compiler and standalone components; inspected for this revision.
[^svelte-mount]: Svelte mounting and hydration; inspected for this revision.
[^svelte-state]: Reactive state and sharing between modules; inspected for this revision.
[^vue]: Vue standalone, embedded, and application modes; inspected for this revision.
[^leptos-islands]: Leptos interactive islands and server-only children; inspected for this revision.
[^leptos-build]: Native and WASM compilation; inspected for this revision.
[^typescript]: Static type checking for JavaScript programs; inspected for this revision.
