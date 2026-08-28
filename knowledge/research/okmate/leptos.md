---
type: Research Report
title: Leptos as an okmate viewer instead of Askama and Datastar
description: Leptos 0.8 can emit the same HTML strings Askama does, but the product people mean by leptos.dev is a WASM hydrate or islands app that fights okmate’s single native binary, static `/{id}/` tree, and server-owned hypermedia contract.
tags: [domain/okmate, concern/architecture, concern/rendering, concern/tooling]
status: draft
generated: { by: process:cursor, at: 2026-08-28T11:00:00Z }
stale_after: 2026-11-28
authority: exploratory
owners: [human:nils]
sources:
  - id: leptos-site
    resource: https://leptos.dev
    title: Leptos product site
    author: organization:leptos-rs
  - id: leptos-crate
    resource: https://docs.rs/leptos/0.8.20/leptos/
    title: leptos 0.8.20 crate docs (SSR, hydrate, islands, CSR)
    author: organization:leptos-rs
  - id: leptos-islands
    resource: https://book.leptos.dev/islands.html
    title: Leptos book islands guide
    author: organization:leptos-rs
  - id: cargo-leptos
    resource: https://book.leptos.dev/ssr/21_cargo_leptos.html
    title: cargo-leptos dual native and wasm32 build
    author: organization:leptos-rs
  - id: cargo-leptos-readme
    resource: https://github.com/leptos-rs/cargo-leptos/blob/main/README.md
    title: cargo-leptos README (bin ssr, lib hydrate)
    author: organization:leptos-rs
  - id: leptos-axum
    resource: https://docs.rs/leptos_axum/latest/leptos_axum/
    title: leptos_axum Axum integration
    author: organization:leptos-rs
  - id: overview
    resource: ../../architecture/system-overview.md
    title: OKMate system overview
    author: process:cursor
    last_modified: 2026-08-27
  - id: readme
    resource: ../../../README.md
    title: Published OKMate stack and CLI
    author: process:git
    last_modified: 2026-08-28
  - id: extract
    resource: ../../plans/okf/okmate.md
    title: Okmate extractable Rust OKF mate
    author: process:cursor
    last_modified: 2026-08-26
  - id: rust-vs-rocci
    resource: ../okf/okf-viewer-rust-vs-rocci.md
    title: OKF viewer Rust HTML versus finished Rocci shell
    author: process:cursor
    last_modified: 2026-08-26
  - id: cargo
    resource: ../../../Cargo.toml
    title: okmate package deps Askama Axum Datastar h35-desktop
    author: process:git
    last_modified: 2026-08-28
  - id: pages
    resource: ../../../src/http/pages.rs
    title: Datastar GET fragment and live in-memory document
    author: process:git
    last_modified: 2026-08-28
  - id: http
    resource: ../../../src/http/mod.rs
    title: Axum router ServeDir plus Datastar middleware
    author: process:git
    last_modified: 2026-08-28
  - id: site
    resource: ../../../src/site.rs
    title: build writes HTML pages; view writes preview shell
    author: process:git
    last_modified: 2026-08-28
  - id: views
    resource: ../../../src/views/mod.rs
    title: Askama Document templates and fragment renders
    author: process:git
    last_modified: 2026-08-28
  - id: desktop
    resource: ../../../src/desktop.rs
    title: h35-desktop preview host for localhost origin
    author: process:git
    last_modified: 2026-08-27
  - id: base
    resource: ../../../templates/base.html
    title: Shell landmarks and staged JS
    author: process:git
    last_modified: 2026-08-28
  - id: nav-items
    resource: ../../../templates/nav_items.html
    title: Sidebar links issue Datastar @get
    author: process:git
    last_modified: 2026-08-28
  - id: article
    resource: ../../../templates/fragments/article.html
    title: Concept meta plus safe-spliced article_html
    author: process:git
    last_modified: 2026-08-27
  - id: nav-js
    resource: ../../../assets/nav.js
    title: Keep-nav after Datastar patches main
    author: process:git
    last_modified: 2026-08-27
  - id: okf-readme
    resource: ../../../okf/README.md
    title: Portable UI-neutral OKF engine
    author: process:git
    last_modified: 2026-08-26
  - id: ci
    resource: ../../../.github/workflows/ci.yml
    title: Native cargo test without wasm target
    author: process:git
    last_modified: 2026-08-27
  - id: build-test
    resource: ../../../tests/build.rs
    title: build writes catalog.json and per-route index.html
    author: process:git
    last_modified: 2026-08-28
  - id: resp-plan
    resource: ../../plans/okmate/viewer-responsiveness.md
    title: Viewer responsiveness plan
    author: process:cursor
    last_modified: 2026-08-27
  - id: resp-research
    resource: viewer-responsiveness.md
    title: Viewer click-path latency and large chrome pages
    author: process:cursor
    last_modified: 2026-08-27
  - id: h35
    resource: https://github.com/koliyo/h35-desktop
    title: h35-desktop hypermedia webview host
    author: organization:koliyo
---

# Leptos as an okmate viewer instead of Askama and Datastar

## Claim

[Leptos](https://leptos.dev) is a full-stack Rust web framework whose default product is **server HTML plus a `wasm32-unknown-unknown` client**.[^leptos-site] Okmate is a **single native binary** that compiles Askama templates, serves them with Axum, morphs `#okmate-main` / `#okmate-toc` with Datastar, and optionally loads that origin in `h35-desktop`. Those are different answers to “who owns the next click.”[^leptos-crate][^readme][^cargo][^pages]

Using Leptos as an Askama substitute (`ssr` only, `render_to_string`, no WASM) is feasible and would not move the engine boundary. Using Leptos the way the book and `cargo-leptos` describe it (hydrate or islands, dual compile, client router or islands router) would replace Datastar, the staged `assets/*.js` contract, CI, and how `okmate build` writes `/{id}/index.html`. That is a product rewrite, not a crate swap.[^cargo-leptos][^extract][^build-test]

This record is exploratory. It does not mint a Decision and it is not an implementation plan.

## Current setup (what would be replaced)

The published stack is Askama 0.16, Axum 0.8, official Datastar 0.4, and optional `h35-desktop`. The only in-repo Rust library is `okf`. CLI-only tests omit the webview: `cargo test -p okmate --no-default-features`.[^readme][^cargo][^overview][^ci]

| Surface | Owner today |
| --- | --- |
| Parse, graph, `article_html`, artifacts | `okf/` (UI-neutral) |
| Full documents and fragments | Askama templates + `Document` view structs |
| Live HTTP | Axum; Datastar middleware on GET; `ServeDir` for assets |
| Keep-nav, outline spy, resize, reading prefs, goto | Small staged JS (`nav.js`, `toc.js`, …) |
| Desktop window | `h35-desktop` loads the localhost origin; folder pick is IPC |

First paint and `okmate build` are full HTML documents with `#okmate-nav`, `#okmate-main`, `#okmate-toc`, and `#okmate-toolbar`. A Datastar GET returns an SSE `PatchElements` of main plus toc and **does not** replace the sidebar. `nav.js` watches `#okmate-main` and restyles current links in the nav that stayed in the DOM.[^base][^pages][^nav-js][^nav-items][^views]

`okmate build` writes engine JSON plus one `index.html` per route (`/hello/` → `hello/index.html`). Live `view` no longer writes that concept tree; it writes a preview shell (`pages.json` and `__okmate/` assets) and renders from the in-memory workspace.[^site][^build-test][^resp-plan]

Markdown bodies are not a Leptos or Askama concern. The engine already emits `article_html`; the article fragment splices it with `|safe`.[^article][^okf-readme]

Chrome JS plus `datastar.js` is about 70 KiB uncompressed (`datastar.js` 34 KiB). The extract plan chose this hypermedia loop on purpose: server-owned state, no client domain store, one-shot morph first.[^extract][^readme]

## What Leptos 0.8 actually is

Crate `leptos` 0.8.20 documents three exclusive-per-target modes: `csr` (DOM in the browser), `ssr` (HTML string on the server), `hydrate` (attach reactivity to SSR HTML). `islands` inverts the default so only `#[island]` components ship as WASM. Axum is a first-party host via `leptos_axum`. The same `view!` RSX can theoretically run in all three modes.[^leptos-crate][^leptos-axum][^leptos-islands]

Interactive SSR is not `cargo build -p okmate`. `cargo-leptos` compiles the **library** to `wasm32-unknown-unknown` with `--features hydrate` and the **binary** natively with `--features ssr`. CI and local toolchains must have that target. A “Hello, world!” islands app is documented at 24 KiB WASM uncompressed with no islands, 166 KiB with a small island, 274–355 KiB fully hydrated including the client router.[^cargo-leptos][^cargo-leptos-readme][^leptos-islands]

Leptos can also run as **MPA HTML** (`<a>` / `<form>`, no hydrate) or as **SSR-only** `render_to_string` written to disk. Those modes drop the part of leptos.dev that looks like a SPA framework. They still use RSX (or `include_view!`) instead of `.html` files.[^leptos-crate]

## Three adoption shapes

Do not evaluate “use Leptos” as one change. The cost and the leftover Datastar/JS depend on which shape.

### A — SSR-only templating (Askama → `view!`)

Keep Axum, Datastar, staged JS, `h35-desktop`, `okmate build` as `/{id}/index.html`, and `--no-default-features` as native-only. Replace `#[derive(Template)]` with Leptos components and `ssr` `render_to_string` (or equivalent) for full pages and fragments.

**Fits:** one native binary; engine stays off WASM; fragments can still be Datastar patches; landmarks and tests can stay string asserts.[^views][^pages][^ci]

**Does not buy:** reactivity, typed client routing, deletion of `assets/*.js`, or a smaller click path. Viewer latency is workspace load and chrome size, not Askama. Fragment render of a small page was already ~1 ms once the bundle was in memory.[^resp-research]

**Authoring:** this is the Maud-class option the extract plan already declined as the default. Askama keeps HTML files with `{% extends %}` / `{% include %}`. `view!` lives in rust-analyzer and looks less like a page. `include_view!` can load RSX from a file; it is still RSX, not Jinja-like HTML.[^extract][^rust-vs-rocci][^leptos-crate]

**Runtime cost:** Leptos SSR still constructs a reactive view tree to emit a string. Askama is a typed HTML compiler. For okmate’s mostly-static documents that is extra machinery for the same bytes.

### B — Islands for chrome, server HTML for documents

Server-render the sea of pages (home, concept, review, log, settings). Opt into `#[island]` only for widgets that today are JS: reading prefs, pane resize, outline spy, windowed review/log, goto. Drop Datastar if the islands router or ordinary `<a>` navigation is enough; or keep Datastar and then you have two client runtimes.[^leptos-islands][^nav-js]

**Fits:** `okf` never compiles to WASM if islands do not call it. Binary size can stay in the “small island” class if islands stay tiny. Fine-grained list updates are the one Leptos feature that maps onto the windowed review/log problem.[^leptos-islands][^resp-plan]

**Breaks or strains:**

- Dual compile (`ssr` bin + `hydrate` wasm) and a `pkg/` (or `__okmate/`) WASM+JS payload next to `app.css`. CI today is `dtolnay/rust-toolchain@stable` plus native `cargo test`.[^ci][^cargo-leptos]
- `okmate build` static tree: Leptos static routes are not primarily “write `hello/index.html` for any file server.” Hydration without the matching router/pkg files is a known mismatch. Build would need an export step that today’s `write_html_pages` does not have.[^build-test][^site]
- Server-owned session (`session.json`, loopback settings POST) can stay, but island signals are a client store for chrome. That is already true of `sessionStorage` in `nav.js`; islands would grow it.[^extract][^nav-js]
- `h35-desktop` is a hypermedia host (HTML origin + IPC). WASM islands in WKWebView work in principle; they are not the contract that crate documents.[^h35][^desktop]

### C — Full hydrate (or CSR) app

One Leptos `App`, client router, server functions, resources. Datastar and most of `assets/*.js` go away. Desktop either embeds CSR WASM (Tauri-shaped) or hydrates against the existing localhost Axum.

**Does not fit without moving constraints:**

- Hydrating any component that calls `okf::load` pulls parse, YAML, comrak, and filesystem into WASM. The engine is specified UI-neutral and is not a wasm crate.[^okf-readme][^overview]
- A client domain store (signals, resources) is the opposite of “no client domain store; one-shot morph first.”[^extract]
- Release artifacts become native binary + wasm + JS glue. Homebrew/`OKMate.app` today ship a native binary whose window loads HTML.[^readme][^desktop]
- Tests that GET with `datastar-request: true` and assert fragment HTML would all change.[^pages][^views]

CSR-in-webview with `okf` on a sidecar server is a different product (Tauri + Leptos templates exist in the wild). It throws away the extract plan’s “Askama owns HTML, Datastar owns morph” split and the static `build` tree.[^extract][^leptos-crate]

## Constraint check

| Constraint that does not move (extract / overview / responsiveness) | Leptos A (SSR-only) | Leptos B (islands) | Leptos C (hydrate/CSR) |
| --- | --- | --- | --- |
| `okf` UI-neutral; check/inspect/search need no HTML | Holds | Holds if islands never call `okf` | Fails if the client loads bundles |
| One workspace Rust dep: `okf`; rest crates.io | Holds (`leptos` on crates.io) | Holds; adds `cargo-leptos` toolchain | Holds as deps; fails as product shape |
| Server-owned durable state; no client domain store | Holds (Datastar stays) | Chrome signals leak | Fails |
| Askama HTML files are the HTML owner; Maud is not default | Fails (RSX ≈ Maud) | Fails | Fails |
| Official Datastar is the protocol crate | Holds | Likely dropped | Dropped |
| `okmate build` writes a full static `/{id}/` tree | Holds if you keep `write_html_pages` | Extra export story | Extra export story |
| Landmarks `#okmate-*`; HTML/CSS/JS in this crate | Holds | JS shrinks; WASM appears | WASM appears |
| Replacing Askama or Datastar is out of bound for responsiveness work | Orthogonal; do not mix | Orthogonal | Orthogonal |
| Native CI / `--no-default-features` | Holds | Needs wasm target and dual features | Same |

The extract plan’s HTML-owner sentence is the real block for A, not technical impossibility. That plan preferred Askama *because* it is HTML files with typed context structs, not because a Rust HTML DSL was unavailable.[^extract][^rust-vs-rocci]

## What is not a reason to switch

Click stall was per-request `Workspace::reload` / `okf::load`, not template compilation. After in-memory clicks, remaining work is windowed chrome and large articles. The responsiveness plan names replacing Askama or Datastar as a non-goal.[^resp-research][^resp-plan][^pages]

Designer-readable markup: Askama templates are already HTML. Leptos RSX is closer to the `format!` / Maud side of the earlier Rust-vs-Rocci authoring table.[^rust-vs-rocci][^base]

Sharing one UI language with a future WASM-heavy app: okmate’s window is a localhost HTML origin behind `h35-desktop`, not a CSR bundle. Switching hosts to Tauri to “use Leptos properly” is a third rewrite.[^desktop][^h35]

## Recommendation

Keep Askama + Axum + Datastar + `h35-desktop`. Do not adopt Leptos B or C unless the product goal changes to “Rust-in-the-browser chrome” and the project accepts dual compile, a WASM payload in `OKMate.app`, and a new static-export story.

Treat Leptos A as a templating taste choice already decided against (Maud-class). Revisit only if Askama inheritance/`Document` duplication becomes the actual maintenance bottleneck—and compare Maud in the same evaluation, because A does not use the rest of leptos.dev.

If a later plan ever wants typed reactive chrome without abandoning hypermedia documents, islands (B) is the only Leptos architecture that can keep `okf` on the server. That plan would still have to replace or quarantine Datastar, teach CI `wasm32-unknown-unknown`, and define how `okmate build` HTML works without a Leptos server.

## Out of this record

An implementation plan, a Decision, a spike crate, or measurements of Leptos SSR vs Askama on this machine. Pair a plan only if the recommendation is reversed.

[^leptos-site]: Product front door for the framework this record evaluates.
[^leptos-crate]: 0.8.20: `csr` / `ssr` / `hydrate` exclusive per target; `islands`; `view!` RSX; `include_view!`; MPA and progressive-enhance SPA described as the same code.
[^leptos-islands]: Islands invert interactivity; documented WASM sizes 24 / 166 / 274–355 KiB uncompressed for the guide’s hello-world variants; `hydrate_islands` plus `HydrationScripts islands=true`.
[^cargo-leptos]: Dual build: native server and `wasm32-unknown-unknown` client; `rustup target add wasm32-unknown-unknown`.
[^cargo-leptos-readme]: Lib features `hydrate`, bin features `ssr`; `--no-default-features` on each half.
[^leptos-axum]: First-party Axum handlers for streaming SSR and static-route generation helpers.
[^overview]: Application crate owns Askama, Axum, Datastar, desktop preview; `okf/` stays UI-neutral.
[^readme]: Stack table; `build` Askama HTML; `view` localhost; desktop feature; CLI-only `--no-default-features`.
[^extract]: Askama owns HTML; Datastar owns morph; server-owned state; Maud not default; `okf` only workspace Rust dep.
[^rust-vs-rocci]: Askama-class HTML files vs Maud macros vs Rocci; unbound pick was Rust HTML + Datastar, which became okmate.
[^cargo]: `askama`, `axum`, `datastar` with `axum` feature, optional `h35-desktop`.
[^pages]: Datastar GET renders `render_main_fragment`; full GET renders `render_document`; live path reads `AppState` workspace.
[^http]: Router: settings/prefs/window routes, `ServeDir`, Datastar middleware, fallback files.
[^site]: `write_html_pages` for `build`; preview shell is assets + `pages.json`.
[^views]: One `Document` feeds page templates and fragment templates; tests assert landmarks and `data-on:click__prevent`.
[^desktop]: `h35_desktop::preview` on the bound origin; pick-folder IPC alias.
[^base]: Shell loads `datastar.js` and the small chrome scripts.
[^nav-items]: Sidebar anchors `data-on:click__prevent="@get('…')"`.
[^article]: `{{ article_html|safe }}` after concept meta.
[^nav-js]: MutationObserver on `#okmate-main`; Datastar must not replace `#okmate-nav`.
[^okf-readme]: Engine has no HTML/HTTP/desktop deps; `article_html` is produced at load.
[^ci]: `cargo test -p okf` and `cargo test -p okmate --no-default-features`; no wasm target.
[^build-test]: `okmate build` writes `catalog.json` and landmark HTML at `index.html` and `{id}/index.html`.
[^resp-plan]: Non-goal: replacing Askama or Datastar; live preview does not write every page; landmarks stay `#okmate-*`.
[^resp-research]: Click stall was reload/load, not Askama; small fragment render ~1 ms.
[^h35]: Host owns window chrome and IPC; page origin owns document chrome; Datastar lives in the origin if used.
