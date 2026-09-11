---
type: Implementation Plan
title: Viewer peek previews and document tabs
description: Add live-preview hover peeks that honor heading anchors and keyed footnotes, then a document tab strip that stays outside the Datastar patch.
tags: [domain/okmate, concern/rendering, concern/tooling, concern/evidence]
status: draft
generated: { by: process:cursor, at: 2026-09-11T08:07:00Z }
stale_after: 2026-12-11
authority: exploratory
owners: [human:nils]
sources:
  - id: cite
    resource: ../../research/okf/reference-authoring-style.md
    title: Reference authoring style for OKF
    author: process:cursor
    last_modified: 2026-09-11
  - id: census
    resource: ../../research/okf/knowledge-systems-built-on-okf.md
    title: Knowledge systems built on Open Knowledge Format
    author: process:cursor
    last_modified: 2026-08-30
  - id: studio
    resource: https://github.com/saschb2b/okf-studio
    title: OKF Studio repository
    author: human:saschb2b
  - id: overview
    resource: ../../architecture/system-overview.md
    title: OKMate system overview
    author: process:cursor
    last_modified: 2026-08-30
  - id: shell
    resource: viewer-shell-parity.md
    title: Okmate viewer shell parity
    author: process:cursor
    last_modified: 2026-08-26
  - id: multi
    resource: extended-multi-bundle.md
    title: Extended multi-bundle viewer
    author: process:cursor
    last_modified: 2026-08-27
  - id: ast
    resource: ../../../okf/src/ast.rs
    title: HeadingSection and Concept footnote_ids
    author: process:git
    last_modified: 2026-08-26
  - id: graph
    resource: ../../../okf/src/graph.rs
    title: published_href fragment preservation
    author: process:git
    last_modified: 2026-08-26
  - id: md
    resource: ../../../okf/src/markdown.rs
    title: Heading IDs, footnote labels, article HTML
    author: process:git
    last_modified: 2026-08-28
  - id: load
    resource: ../../../okf/src/load.rs
    title: Footnote and sources[].id matching
    author: process:git
    last_modified: 2026-08-29
  - id: session
    resource: ../../../src/session.rs
    title: Session open_path and open_hash
    author: process:git
    last_modified: 2026-08-29
  - id: pages
    resource: ../../../src/http/pages.rs
    title: Datastar GET main and toc patch
    author: process:git
    last_modified: 2026-08-29
  - id: http
    resource: ../../../src/http/mod.rs
    title: Preview router
    author: process:git
    last_modified: 2026-09-11
  - id: site
    resource: ../../../src/site.rs
    title: Asset emit and page render
    author: process:git
    last_modified: 2026-09-11
  - id: html-util
    resource: ../../../src/html_util.rs
    title: first_prose_paragraph
    author: process:git
    last_modified: 2026-08-29
  - id: nav-js
    resource: ../../../assets/nav.js
    title: Keep-nav, hash restore, in-page links
    author: process:git
    last_modified: 2026-08-29
  - id: base
    resource: ../../../templates/base.html
    title: Okmate document chrome
    author: process:git
    last_modified: 2026-09-11
  - id: article
    resource: ../../../templates/fragments/article.html
    title: Concept meta and raw article HTML
    author: process:git
    last_modified: 2026-08-29
  - id: nav-test
    resource: ../../../tests/navigation.rs
    title: Datastar fragment keeps nav out of the patch
    author: process:git
    last_modified: 2026-08-29
---

# Viewer peek previews and document tabs

## Goal

Give `okmate view` Wikipedia-style hover peeks that preview the **targeted
heading or keyed footnote**, not only the document lead, and a browser-style
document tab strip in the live preview shell.[^studio][^cite][^pages][^shell]

## Out of bound

Static `okmate build` HTML (no peek endpoint, no tabs). OS window tear-off,
empty tabs, Cmd+T, per-tab back/fwd stacks, drag-reorder. Replacing collection
nav blurbs. Graph or search as tab types. Changing `published_href` unless
a real fragment bug is found. Interactive peek cards (`pointer-events` stay
none). A shared-bibliography resolver; join only this concept's `sources[].id`
to the hovered footnote label. Adding `heading_sections` on `Index` unless
Phase 1 cannot excerpt collection hashes another way. Peek helpers as `okf`
UI. Editing developer-knowledge; that citation restructure is evidence, not
a record this plan owns.[^cite][^graph][^multi][^overview]

## Constraints that do not move

- HTML and JS stay in this crate. `okf` stays UI-neutral. Extract footnote
  definition text from Comrak `article_html` in okmate; add engine footnote
  bodies only if those ids are too brittle for tests.[^overview][^md][^ast]
- Landmarks stay `#okmate-nav`, `#okmate-main`, `#okmate-toc`. Tab strip is a
  sibling **outside** `#okmate-main` so Datastar morph does not remount it.
  CSS prefix `okmate-`. Datastar `@get` still patches main and toc only.[^pages][^nav-test][^base]
- Hash resolution order: Comrak footnote (`fn-{label}` / `fnref-{label}` and
  numbered `fnref-{label}-n`) then `HeadingSection` id then document lead. Pin
  Comrak 0.54 ids in tests; do not assume GitHub `#footnote-1`.[^md][^ast]
- Same-document footnote refs scroll in page and do not create tabs. Hashed
  concept links inside a footnote navigate (and may open a tab).[^nav-js][^cite]
- `persist_open_path_to` currently clears `open_hash`. Tab persist must keep
  heading and footnote hashes (`#s21`, `#fn-s21`).[^session]

## Product contract

Reference UX is OKF Studio's peek card and tab strip: type badge, title,
description, excerpt, about 450ms dwell, non-interactive card, Cmd/Ctrl+click
new tab. Studio strips fragments and never peeks footnotes. This plan must
not copy that gap.[^studio][^census]

Motivating citation shape: a report cites with `Steinberger[^s21]` instead of
`[Steinberger](register.md#s21)`, and the footnote carries a publication URL plus
`[Research context](register.md#s21)`. Hovering either target previews **that**
footnote or heading.[^cite]

**Peek (live `okmate view` / desktop only)**

- Dwell on in-bundle links and footnote refs in the article (including the
  footnotes list), related lists, recents, breadcrumbs, and sidebar leaves.
  Skip external, assets, `/__okmate/`, broken links, and collection-folder
  summaries (keep existing blurbs).[^article][^multi]
- No hash: type chip, document title, description, lead excerpt
  (`first_prose_paragraph`, about 280 characters).[^html-util][^studio]
- Heading hash: type and document title as context; heading text as card
  title; excerpt from that `HeadingSection`. Same-document `#section` peeks
  that section.[^ast]
- Footnote hash: type and document title as context; card title is matching
  `sources[].title` (else `[^label]`); body is definition plaintext plus joined
  source author and resource when present. Because the card is
  non-interactive, also include the first in-bundle hashed link's section
  excerpt when the definition contains one. Unknown id falls back to the
  document lead.[^load][^cite]
- JSON `GET /__okmate/peek?path=/…/&hash=` from in-memory `Workspace`.[^http][^pages]

**Tabs**

- Identity is the document route. Hash is stored for restore and scroll; it
  does not create a second tab for the same path.[^session][^studio]
- Click and sidebar `@get` navigate the current tab and apply the hash. If
  that route is already open, activate it and scroll.[^pages]
- Cmd/Ctrl+click and middle-click open a background tab; Cmd/Ctrl+Shift+click
  activates. Close via × and middle-click on the tab. Strip hidden while
  fewer than two tabs. Chrome pages (home, review, log, settings) are
  tabbable.[^studio][^base]
- Capture-phase handler: article-body links use `requestDocument` / `@get`
  instead of a full reload. Same-doc `#fn-*` / `#fnref-*` stay in-page. Modifier
  clicks run before Datastar `data-on:click__prevent`.[^nav-js][^article]

### Phase 1 — Peek model and endpoint

**Bound:** `Peek` JSON from lead prose, `heading_sections`, and
footnote/source join. `GET /__okmate/peek` on the preview router. Application
crate owns assembly.[^ast][^html-util][^http][^load]

**Out of bound:** Peek card CSS/JS. Tabs. Changing Comrak options.

**Tests:** Unit plus HTTP. `/hello/#details` excerpt is the Details section,
not the intro. Citation-chain fixture: report with `sources[].id: s21`,
body `…[^s21]`, definition linking a publication and `register.md#s21`, plus
a register concept with `### S21`. `peek?path=/report/&hash=` plus the pinned
Comrak footnote id returns the footnote/source title, not the report lead.
`peek?path=/register/&hash=s21` returns the S21 section, not the register
lead.[^cite][^md]

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`. If engine footnote bodies were added:
`cargo test -p okf` as well.

**Owner:** new peek module under `src/`, `src/http/mod.rs`,
`src/http/pages.rs`.

### Phase 2 — Peek card

**Bound:** `assets/peek.js` and CSS. Dwell about 450ms; dismiss on leave,
click, scroll, Escape. Bind footnote-ref anchors as well as concept links.
Wire in `templates/base.html` and `src/site.rs` asset emit. Card is
`role="tooltip"`, non-interactive, placed below the trigger and flipped if
needed. Hint: `Click to open · ⌘/Ctrl+click: new tab`; omit the new-tab
clause for same-doc footnote refs.[^studio][^base][^site]

**Out of bound:** Tab strip. Changing Datastar patch targets.

**Tests:** Rendered live page includes `peek.js`. Fixture HTML contains
footnote-ref hrefs the script can classify. No native `title` tooltip race
on peeked concept links.

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`.

**Owner:** `assets/peek.js`, `assets/app.css`, `templates/base.html`,
`src/site.rs`.

### Phase 3 — In-app link navigation

**Bound:** Article-body (and modifier-aware chrome) links stay in the
Datastar shell and scroll to heading or footnote hashes after patch.
Same-doc footnote refs do not `@get`. Keep the existing “patch has no
`#okmate-nav`” contract.[^nav-js][^pages][^nav-test][^article]

**Out of bound:** Tab session persist. Peek JSON changes unless a hash
classifier bug is found.

**Tests:** Datastar GET still excludes `#okmate-nav`. A body link to
`/register/#s21` is handled as an in-app navigation target in the
classifier tests or HTTP fixture.

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`.

**Owner:** `assets/nav.js`, optional `assets/peek.js` click hand-off.

### Phase 4 — Document tabs

**Bound:** Session tab list and active id. Strip chrome above main, outside
the patch. Gestures from the product contract. Persist and restore route plus
heading or `fn-*` hash. Strip absent with one tab.[^session][^base][^studio]

**Out of bound:** Items listed under Out of bound above.

**Tests:** Two-tab session restores the active route and hash. Strip markup
absent with a single tab. Datastar patch still excludes `#okmate-nav` and the
tab strip id.

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`.

**Owner:** `src/session.rs`, `templates/base.html`, tab CSS/JS, `src/http/prefs.rs`.

## Status

Exploratory; no phase started. Citation research:
[reference authoring style](/research/okf/reference-authoring-style.md).[^cite]

[^cite]: Claim-level keyed footnotes versus register heading links; engine joins labels to `sources[].id` and does not resolve `register.md#s21` as a source id.
[^census]: OKF Studio as a dedicated reader with relationship panels.
[^studio]: Peek card and tab-strip UX; fragment stripping is the gap not to copy.
[^overview]: Application crate owns Askama, Axum, Datastar, and desktop preview.
[^shell]: Three-pane keep-nav shell this work extends.
[^multi]: Collection nav blurbs stay; they are not peek cards.
[^ast]: `HeadingSection` for heading peeks; `footnote_ids` without definition bodies.
[^graph]: `published_href` keeps path fragments and leaves a leading `#` alone.
[^md]: Comrak footnotes and heading ids on `article_html`.
[^load]: Missing footnote/source pairs are diagnostics, not a peek resolver.
[^session]: Single `open_path` / `open_hash`; persist path clears hash.
[^pages]: Datastar GET patches main and toc from in-memory workspace.
[^http]: `/__okmate/*` routes beside the ServeDir fallback.
[^site]: Preview asset emit from `include_str!`.
[^html-util]: Lead excerpt helper already used for collection summaries.
[^nav-js]: Same-document hash scroll and `afterPatch === "hash"`.
[^base]: Shell landmarks; no tab strip today.
[^article]: Related links use `@get`; body HTML does not.
[^nav-test]: Patch must not include `#okmate-nav`.
