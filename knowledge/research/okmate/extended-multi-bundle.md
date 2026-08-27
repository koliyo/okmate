---
type: Research Report
title: Extended multi-bundle viewer (nav modes, dashboard, log)
description: The roots registry already lists many OKF bundles; the Askama viewer still loads one tree, so path collisions, a merged dashboard, collection hover, and a merged log are application work, not an engine merge.
tags: [domain/okmate, concern/architecture, concern/rendering, concern/tooling]
status: draft
generated: { by: process:cursor, at: 2026-08-27T08:10:00Z }
stale_after: 2026-11-27
authority: exploratory
owners: [human:nils]
sources:
  - id: overview
    resource: ../../architecture/system-overview.md
    title: Okmate system overview
    author: process:cursor
    last_modified: 2026-08-26
  - id: readme
    resource: ../../../README.md
    title: Okmate README
    author: process:git
    last_modified: 2026-08-27
  - id: multi-roots
    resource: ../../plans/okf/multi-knowledge-roots.md
    title: Multiple knowledge roots (historical rocci-okf contract)
    author: process:cursor
    last_modified: 2026-08-25
  - id: extract
    resource: ../../plans/okf/okmate.md
    title: Okmate extractable Rust OKF mate
    author: process:cursor
    last_modified: 2026-08-26
  - id: dash-plan
    resource: ../../plans/okmate/dashboard-parity.md
    title: Okmate dashboard parity
    author: process:cursor
    last_modified: 2026-08-26
  - id: shell-plan
    resource: ../../plans/okmate/viewer-shell-parity.md
    title: Okmate viewer shell parity
    author: process:cursor
    last_modified: 2026-08-26
  - id: nested
    resource: ../../decisions/nested-okf-collections.md
    title: Nest collections under okf, okmate, and ops
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
    title: Datastar GET fragment (one okf::load)
    author: process:git
    last_modified: 2026-08-27
  - id: http
    resource: ../../../src/http/mod.rs
    title: AppState (one root, one output)
    author: process:git
    last_modified: 2026-08-27
  - id: gov
    resource: ../../../src/views/governance.rs
    title: Recents, stats, and collection prefix
    author: process:git
    last_modified: 2026-08-27
  - id: views
    resource: ../../../src/views/mod.rs
    title: Document and NavNode views
    author: process:git
    last_modified: 2026-08-27
  - id: roots
    resource: ../../../src/roots.rs
    title: Resolve configured directory and git roots
    author: process:git
    last_modified: 2026-08-27
  - id: config
    resource: ../../../src/config.rs
    title: ~/.okmate/config.toml registry
    author: process:git
    last_modified: 2026-08-27
  - id: home
    resource: ../../../templates/fragments/home.html
    title: Dashboard recents, stats, Knowledge Collections
    author: process:git
    last_modified: 2026-08-27
  - id: nav-items
    resource: ../../../templates/nav_items.html
    title: Three-level unrolled collection nav
    author: process:git
    last_modified: 2026-08-27
  - id: nav-js
    resource: ../../../assets/nav.js
    title: Keep-nav sync and section fold persist
    author: process:git
    last_modified: 2026-08-27
  - id: goto
    resource: ../../../assets/goto.js
    title: Cmd-K palette over pages.json
    author: process:git
    last_modified: 2026-08-27
  - id: ast
    resource: ../../../okf/src/ast.rs
    title: Bundle, Index, and Log types
    author: process:git
    last_modified: 2026-08-27
  - id: okf-lib
    resource: ../../../okf/src/lib.rs
    title: Single-root load and log.md parse
    author: process:git
    last_modified: 2026-08-27
  - id: okf-preview
    resource: ../../../okf/src/preview.rs
    title: Preview path resolution (log.md rejected)
    author: process:git
    last_modified: 2026-08-27
  - id: okf-readme
    resource: ../../../okf/README.md
    title: Portable engine boundary
    author: process:git
    last_modified: 2026-08-26
  - id: okf-engine-test
    resource: ../../../okf/tests/engine.rs
    title: "okf: hrefs are not bundle paths"
    author: process:git
    last_modified: 2026-08-27
  - id: build-test
    resource: ../../../tests/build.rs
    title: Built home asserts Knowledge Collections
    author: process:git
    last_modified: 2026-08-27
  - id: nav-test
    resource: ../../../tests/navigation.rs
    title: Datastar fragment keeps nav out of the patch
    author: process:git
    last_modified: 2026-08-27
  - id: log
    resource: ../../log.md
    title: This bundle's knowledge log
    author: process:cursor
    last_modified: 2026-08-27
  - id: pair
    resource: ../../plans/okmate/extended-multi-bundle.md
    title: Implementation plan paired with this report
    author: process:cursor
    last_modified: 2026-08-27
---

# Extended multi-bundle viewer

## Question

The user-level roots registry can already name many OKF bundles. What does
the Askama viewer actually do with more than one root, and what has to change
so the sidebar can switch between a path-union tree and VS Code-style folder
trees, the dashboard recents and log stay merged, and collection blurbs move
off the home page onto sidebar hover?[^overview][^readme][^pair]

This report is evidence against the current tree. It is not a Decision.

## Registry versus viewer

`~/.okmate/config.toml` stores directory and git roots. `okmate roots` /
`okmate sync` resolve them. `/settings/` edits the same file. Agents are
told to loop `okmate roots --format paths` and run `check` / `inspect` /
`search` once per path. Architecture and the README keep those verbs
single-root on purpose.[^config][^roots][^overview][^readme]

`okmate view` does not load that registry as a workspace. With a path it
calls `okf::resolve_preview_path`. With no path it restores
`~/.okmate/state/session.json`'s one `bundle` directory, else `./knowledge`.
`AppState` holds one `root`. `site::build` and Datastar `page_for_route`
call `okf::load` on that root. The file watcher watches that one
directory.[^preview][^http][^pages][^site]

The historical multi-root plan named this gap as an explicit non-goal:
do not merge N bundles into one review site, catalog, search index, or
concept-ID namespace. Cross-root authored links were spelled `okf:<id>/…`
and were not given published `/@id/` routes. The engine now classifies
`okf:` like `mailto:` (not OKF3001) but still emits the raw href in
`article_html`. There is no application rewrite onto in-app
routes.[^multi-roots][^okf-readme][^okf-engine-test]

Okmate already ported the registry, git cache, and settings UI from that
plan. The missing slice is the **viewer workspace**: one preview process,
many loaded `Bundle` values, application-owned HTML — not an `okf::load`
of a concatenated tree.[^extract][^okf-lib]

## Identity and URL

A concept ID is the bundle-relative path without `.md`. Two enabled roots
can both contain `plans/okmate/dashboard-parity.md`. Today's published
route is `/{id}/`, so those records collide if they share one HTTP
tree.[^nested][^site]

| Scheme | Fit |
| --- | --- |
| Keep `/{id}/` and last-write-wins | Silent data loss. Unfit. |
| Unprefixed when unique, `/@id/` only on collision | Unstable bookmarks when a second root is added. |
| Query `?root=` | Awkward for static files and Cmd-K. |
| Always `/@<root-id>/<id>/` in a workspace | Stable, matches `okf:<id>/…`, works in both nav modes. |

Single-root `okmate view knowledge` and `okmate build` can keep `/{id}/`
so existing tests and file trees stay valid. A workspace session (no
explicit path, two or more enabled roots) should prefix document routes.
Chrome stays unprefixed: `/`, `/review/`, `/settings/`.[^site][^build-test]

Intra-bundle Markdown is already rewritten to `/{id}/` inside one
`article_html`. A workspace renderer must prefix those hrefs with the
owning root. `okf:` hrefs should become `/@<id>/…` when that root is
loaded.[^okf-readme][^okf-engine-test]

## Sidebar today

The forest is one bundle's collection indexes. Each collection is a
`<details>` whose summary is a **span**, not a link; Overview is the first
child. Leaves are concepts. Site links (Dashboard, Review, Settings) sit
above the forest. `NavNode` is href, title, current, open, children,
`section_key`. There is no root id, source chip, or summary
field.[^site][^views][^nav-items][^shell-plan]

`nav_items.html` unrolls three nested `details` levels. A VS Code-style
root folder around nested collections (`okf` / `okmate` / `ops` under
`plans`) needs another level or a recursive include. Hard-coding a fourth
copy will rot.[^nav-items][^nested]

Datastar GET patches `#okmate-main` and `#okmate-toc` and **must not**
replace `#okmate-nav`. `nav.js` then paints `is-current` by exact
`a[href="…"]` match and treats `data-okmate-nav-section` as a path prefix
`/{key}/`. A workspace URL `/@okmate/plans/foo/` would open a section
keyed `okmate` if the key is only the first segment, or fail to open
`plans` if the script assumes the route starts at the collection.
Mode switch that rebuilds the tree cannot be an ordinary keep-nav GET: it
has to replace the nav node (full reload, or a dedicated nav patch that
this plan treats as the keep-nav exception).[^nav-js][^nav-test][^shell-plan]

`pages.json` / Cmd-K concatenate title, route, path, and collection. They
do not mention a root id. Duplicate titles in a workspace would be
indistinguishable.[^goto][^site]

## Two explorer modes

**Separated** matches VS Code multi-root workspaces: each enabled root is
a top-level folder; the existing collection forest hangs under it;
documents never share a nav path because the folder prefix differs. This
is the unambiguous tree.

**Merged** is not something VS Code's explorer does. It is a **path
union**: collection folders named `plans` from every bundle collapse to
one `plans` section; two records with the same relative path appear as
sibling leaves (or a split row) with the bundle id shown. Shared path is
the point of the mode, not an error.

Recommendation: default **separated** when two or more roots are loaded
(familiar, no collision UX). Persist the choice on the server
(`session.json`), not only `localStorage`, so the first HTML paint matches
the tree. Hide the toggle when fewer than two roots are loaded. Ordinary
in-app navigation stays keep-nav; toggling mode reloads or patches
`#okmate-nav`.[^preview][^nav-js]

## Dashboard today

Home prepends recents (ten leaf concepts by `generated.at`, collection
badge, no indexes), a review CTA, the stat grid, an `<h2>Knowledge
Collections</h2>`, then the bundle root `index.md` as `article_html`. That
root index **is** the collection list (this bundle: Architecture,
Decisions, Status, Plans, Research, Audits). Removing the heading but
keeping `article_html` still dumps the list. A workspace home also cannot
honestly pick one root's `index.md`.[^home][^gov][^dash-plan][^site]

`tests/build.rs` and the dashboard unit test assert the string `Knowledge
Collections`. Those tests are the contract to replace, not a reason to
keep the heading.[^build-test][^views]

Recents hrefs are `/{id}/` with no source. A merged recents list (required
in both nav modes) needs a root badge whenever more than one bundle
contributes, and workspace hrefs `/@<id>/{concept}/`. Sort stays
`generated.at` descending, then id. Limit stays ten across the union, not
ten per root.[^gov]

Governance stats and `/review/` are still one `Bundle`. This report does
not treat a redesigned review queue as in-scope, but a workspace that
leaves `/review/` bound to the first root would contradict the sidebar.
The paired plan concatenates review rows with a source column so the
existing chrome link stays true, without new filters.[^gov][^dash-plan]

## Collection hover

Collection indexes are not concepts. They have `article_html`, headings,
and links. Typical body: H1, a one-line blurb, then a Markdown list of
children (see `plans/okmate/index.md`: “Application, desktop preview, and
agent setup.”). That blurb is the hover target. Dumping the full
`article_html` (the child list) would duplicate the sidebar.[^ast][^site]

Native `title` is the wrong control (plain text, slow, no markup). A CSS
popover or tooltip on the collection `summary`, with keyboard focus, can
show the first non-heading paragraph (or the italic lead). `aria-describedby`
on the summary keeps it out of the Datastar patch path. Merged mode: one
section, possibly several blurbs keyed by root id if the same collection
path exists in more than one bundle.

## Bundle log

The engine discovers any `log.md`, forbids frontmatter (OKF1021), and
requires `## YYYY-MM-DD` headings (OKF1022). `Bundle.logs` is `Vec<Log>`
with only `path` and `body_span` — no `article_html`, no parsed
entries.[^okf-lib][^ast]

`okmate view path/to/log.md` is rejected: a knowledge log is not a
concept.[^okf-preview]

This bundle's log is Git `merge=union`: independent bullets under the same
day heading combine. The viewer should **not** re-merge Git; it should
parse each root's `log.md` and **union by date** in the application, tagging
each bullet with the root id. Newest day first. Missing `log.md` is skip,
not error.[^log]

Parsing belongs in okmate (read `{root}/log.md` using `Log.path`), not in
a new `okf` HTML type. Extending `Log` with structured entries can wait.

## Watch, session, and entry points

| Entry | Today | Workspace implication |
| --- | --- | --- |
| `okmate view knowledge` | That one bundle | Stay single-root; `/{id}/` |
| `okmate build` | One HTML tree | Stay single-root |
| `okmate view` (no path), desktop no-args | Last session bundle or `./knowledge` | If two or more **enabled** config roots exist, load all of them; otherwise keep today's fallback |
| Settings-only host | No bundle | Unchanged |

Session today stores one `bundle` path. A workspace session should remember
nav mode and that the window is a workspace, without inventing a second
registry. Config remains membership; session remains last UI
choice.[^preview][^config]

Live reload must watch every loaded directory root (and git checkouts on
disk). Git poll-while-viewing from the historical plan is still absent in
`preview.rs`; it is not required to ship nav modes.[^preview][^multi-roots]

## What should not move

- `okf::load` stays one filesystem bundle. The workspace is
  `Vec<(ResolvedRoot, Bundle)>` in okmate.[^okf-lib][^okf-readme]
- `check`, `inspect`, `search`, and `build` stay single-root.[^overview][^readme]
- Concept IDs stay bundle-relative paths.[^nested]
- HTML, CSS, JS stay in this crate. Landmarks stay `#okmate-*`. Ordinary
  GET still does not replace `#okmate-nav`.[^shell-plan][^nav-test]
- Tokens never appear in HTML, `pages.json`, or recents.[^roots]
- Canonical records stay inert Markdown. Nav mode is not a knowledge
  record.[^overview]

## Recommendations the plan adopts

1. Application workspace over N `okf::load` results; no engine merge.
2. Workspace document URLs `/@<root-id>/<id>/`; single-root URLs unchanged.
3. Sidebar default **separated**; optional **merged** path union with source
   labels; toggle hidden for one root; persist in `session.json`.
4. Dashboard recents always the union; drop Knowledge Collections chrome
   and stop rendering root `index.md` on `/`; show merged log under recents.
5. Collection summary = first prose paragraph of that collection's
   `index.md`, on sidebar hover.
6. Rewrite in-app `okf:` and intra-bundle hrefs when rendering a workspace.
7. Thin concatenated `/review/` with source labels so chrome stays honest;
   no queue redesign.

Paired plan: [extended-multi-bundle](/plans/okmate/extended-multi-bundle.md).[^pair]

[^overview]: Engine versus application; `check` / `inspect` stay single-root; agents list `okmate roots` first.
[^readme]: CLI table, `~/.okmate/` paths, and the agent `roots` loop.
[^multi-roots]: Historical non-goal: do not merge N bundles into one review site; `okf:` spelling without `/@id/` routes.
[^extract]: Okmate owns Askama HTML; roots registry ported without a unified site.
[^dash-plan]: Home recents, Knowledge Collections heading, ten-leaf contract.
[^shell-plan]: Keep-nav: Datastar GET must not replace `#okmate-nav`.
[^nested]: Concept ID is the bundle path without `.md`.
[^site]: One `Bundle` per build; `/{id}/` routes; collection forest; `pages.json` without root.
[^preview]: Session stores one `bundle` path; watcher watches one directory.
[^pages]: Datastar GET calls `okf::load` on `AppState.root`.
[^http]: `AppState` is one `root` plus one `output`.
[^gov]: Recents from one bundle, `generated.at`, collection badge, no source.
[^views]: `NavNode` is href/title/current/open/children/`section_key`.
[^roots]: `resolve_all` returns enabled directory and git paths; JSON omits tokens.
[^config]: `UserConfig.roots` is membership; not the preview workspace.
[^home]: Recents, CTA, stats, then Knowledge Collections plus root `index.md`.
[^nav-items]: Three nested `details` levels; collection summary is a span.
[^nav-js]: Exact href current-marking; section prefix `/{key}/`.
[^goto]: Cmd-K haystack is title, route, path, collection.
[^ast]: `Index` has `article_html`; `Log` is path plus `body_span` only.
[^okf-lib]: `okf::load` is one filesystem root; `log.md` parse is validation, not HTML.
[^okf-preview]: `resolve_preview_path` rejects `log.md`.
[^okf-readme]: `okf:` hrefs are not intra-bundle; `article_html` keeps the raw href.
[^okf-engine-test]: Engine test asserts `href="okf:notes/…"`.
[^build-test]: Built home must contain `Knowledge Collections`.
[^nav-test]: Concept Datastar patch omits `#okmate-nav`.
[^log]: Dated bullets; Git `merge=union` is an authoring rule, not a viewer merge.
[^pair]: Implementation phases for the viewer workspace.
