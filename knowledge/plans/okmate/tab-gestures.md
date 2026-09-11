---
type: Implementation Plan
title: Idiomatic document tab gestures
description: Ordinary navigation retargets the current document tab; Cmd/Ctrl+click opens a background tab; desktop Cmd+T focuses or opens home and Cmd+W closes a tab rather than the window; tab labels use document titles and type-color dots.
tags: [domain/okmate, concern/rendering, concern/tooling, concern/developer-experience]
status: draft
generated: { by: process:cursor, at: 2026-09-11T09:09:00Z }
stale_after: 2026-12-11
authority: exploratory
owners: [human:nils]
sources:
  - id: peek
    resource: peek-and-tabs.md
    title: Viewer peek previews and document tabs
    author: process:cursor
    last_modified: 2026-09-11
  - id: overview
    resource: ../../architecture/system-overview.md
    title: OKMate system overview
    author: process:cursor
    last_modified: 2026-08-30
  - id: session
    resource: ../../../src/session.rs
    title: Session tabs and persist_open_path_to
    author: process:git
    last_modified: 2026-09-11
  - id: tabs-js
    resource: ../../../assets/tabs.js
    title: Client tab strip
    author: process:git
    last_modified: 2026-09-11
  - id: nav-js
    resource: ../../../assets/nav.js
    title: In-app click and Cmd/Ctrl+click
    author: process:git
    last_modified: 2026-09-11
  - id: pages
    resource: ../../../src/http/pages.rs
    title: Datastar GET persist_open_path
    author: process:git
    last_modified: 2026-09-11
  - id: desktop
    resource: ../../../src/desktop.rs
    title: Okmate h35-desktop HostOptions
    author: process:git
    last_modified: 2026-08-28
  - id: prefs-test
    resource: ../../../tests/prefs.rs
    title: Two-tab session HTML
    author: process:git
    last_modified: 2026-09-11
  - id: h35-menu
    resource: https://github.com/koliyo/h35-desktop/blob/main/src/menu.rs
    title: Native File Close Window Cmd+W
    author: organization:koliyo
    last_modified: 2026-09-01
  - id: h35-preview
    resource: https://github.com/koliyo/h35-desktop/blob/main/src/preview.rs
    title: KeyboardInput close window
    author: organization:koliyo
    last_modified: 2026-09-01
  - id: h35-history
    resource: https://github.com/koliyo/h35-desktop/blob/main/src/history.rs
    title: Host IPC message parse
    author: organization:koliyo
    last_modified: 2026-09-01
  - id: peek-rs
    resource: ../../../src/peek.rs
    title: Peek JSON document_title and type
    author: process:git
    last_modified: 2026-09-11
  - id: type-color
    resource: ../../../src/views/governance.rs
    title: type_color palette
    author: process:git
    last_modified: 2026-09-11
  - id: base
    resource: ../../../templates/base.html
    title: Tab strip Askama markup
    author: process:git
    last_modified: 2026-09-11
---

# Idiomatic document tab gestures

## Goal

Stop treating every in-app navigation as a new tab. Ordinary clicks, sidebar
`@get`, and Cmd-K retarget the **current** document tab. Cmd/Ctrl+click and
middle-click keep opening a background tab. Desktop Cmd+T opens or focuses
home; Cmd+W closes the current tab, not the window.[^peek][^tabs-js][^session]

## Out of bound

Empty tabs. Duplicate routes (identity stays the document path). Per-tab
back/forward stacks. Drag-reorder. OS window tear-off. Rebuilding the strip
as Askama-owned HTML (`replaceChildren` stays until a later client-JS plan).
Changing Rocci’s default Cmd+W = close window. Intercepting Cmd+W or Cmd+T in
an ordinary browser tab (`window.ipc` absent).[^peek][^desktop]

## Constraints that do not move

- Tab identity is the document route. Hash is restore/scroll state, not a
  second tab. `normalize_tabs` uniqueness stays.[^session][^peek]
- Cmd/Ctrl+click, Cmd/Ctrl+Shift+click, and middle-click on in-app links
  already call `openHref`. Do not regress them.[^nav-js]
- The tab strip stays outside the Datastar main/toc patch. Strip hidden with
  fewer than two tabs.[^peek][^pages]
- Native File → Close Window is Cmd+W in h35-desktop. A page `keydown`
  cannot win on macOS. Tab close must be an **opt-in** host flag so Rocci
  stays unchanged.[^h35-menu][^h35-preview][^overview]
- In a normal browser, Cmd+W and Cmd+T belong to the browser. Only the
  desktop webview (`window.ipc`) intercepts them.[^desktop]

## Current behavior

`afterPatch` always `upsert`s the mounted route, which inserts when the path
is new. Live GET `persist_open_path_to` pushes a `SessionTab` whenever
`open_path` is not already listed. Sidebar navigation therefore accumulates
tabs. Cmd+T is unset. Desktop Cmd+W closes the window.[^tabs-js][^session][^pages][^h35-menu]

## Product contract

- Ordinary click / sidebar `@get` / Cmd-K: if the route is already a tab,
  activate it and restore its hash; otherwise retarget the active tab’s
  path, title, and hash.
- Cmd/Ctrl+click: background tab. Cmd/Ctrl+Shift+click: open and activate.
  Middle-click link: background tab. × and middle-click on a tab: close.
- Cmd+T: open `/` if it is not already a tab; otherwise activate home.
- Cmd+W (desktop): close the current tab. Last tab: close the window.
- Cmd+Shift+W (desktop): close the window with several tabs open.
- Also: Cmd+Shift+T reopen last closed tab; Ctrl+Tab / Ctrl+Shift+Tab cycle;
  Cmd+1…8 and Cmd+9 (last) in the desktop webview. Skip when a text field or
  `#okmate-goto` is focused.

`openHref` remains the only insert path (modifier click, middle-click, Cmd+T
when home is not open).[^tabs-js][^nav-js]

### Phase 1 — Retarget persist and JS gestures

**Bound:** `persist_open_path_to` renames the tab matching the previous
`open_path` instead of pushing when the new route is absent. `afterPatch`
activates an existing tab or retargets the active one. Cmd+T home-or-focus;
closed-tab stack for Cmd+Shift+T; Ctrl+Tab cycle; desktop-only Cmd+W /
Cmd+1–9; `__h35NewTab` / `__h35CloseTab` for the later host menu. Last tab
in a browser no-ops; with `window.ipc`, last Cmd+W posts `close-window`.[^session][^tabs-js]

**Out of bound:** h35-desktop menu changes. Bumping the git dep. Empty tabs.

**Tests:** One-tab hello → review stays one tab. Two-tab session navigating
to a third route retargets the active tab only. Existing two-tab restore
and Datastar fragment tests still pass.[^prefs-test]

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`.

**Owner:** `src/session.rs`, `assets/tabs.js`, `tests/prefs.rs`.

### Phase 2 — h35-desktop tab chrome

**Bound:** `HostOptions.tab_shortcuts` default false. When true: File New
Tab (Cmd+T) evaluates `__h35NewTab`; Close Tab (Cmd+W) evaluates
`__h35CloseTab` or IPC `close-window`; Close Window (Cmd+Shift+W) always
exits. `IpcMessage::CloseWindow`. KeyboardInput Cmd+W uses the same
close-tab path when the flag is on.[^h35-menu][^h35-preview][^h35-history]

**Out of bound:** Okmate `HostOptions` wiring. Changing Rocci defaults.

**Tests:** IPC parse for `close-window`. Close-shortcut tests still use
Cmd/Ctrl+W as the close-tab key; window close is Cmd+Shift+W when
`tab_shortcuts` is on.

**Exit:** `cargo test` and `cargo fmt --all -- --check` in h35-desktop.

**Owner:** `h35-desktop` `src/preview.rs`, `src/menu.rs`, `src/history.rs`,
`src/events.rs`.

### Phase 3 — Wire okmate desktop

**Bound:** `src/desktop.rs` sets `tab_shortcuts: true`. Bump the
`h35-desktop` git rev in `Cargo.lock` after Phase 2 lands.[^desktop]

**Out of bound:** Rocci host options. Tab HTML ownership rewrite.

**Tests:** Existing desktop unit tests. Smoke Cmd+T, Cmd+W, and last-tab
window close in `okmate view`.

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`.

**Owner:** `src/desktop.rs`, `Cargo.lock`.

### Phase 4 — Tab titles and type dots (implemented)

**Bound:** Tab labels are document titles, never path slugs. Cmd-click seeds
title and type-color from the trigger or nav leaf, then
`GET /__okmate/peek?path=` fills `document_title` and `type_color`. Peek
JSON gains `type_color` (concepts via `type_color(type)`, indexes
`Index`, chrome omitted). Session and Askama persist `type_color`. Tab
button is a type dot plus `.okmate-tab-label`. Chrome routes have no
dot.[^peek-rs][^type-color][^tabs-js][^base]

**Out of bound:** Empty tabs, duplicate routes, Datastar strip rewrite.

**Tests:** Peek JSON for a typed concept includes `type_color` matching
`okmate::views::type_color`; chrome peek omits it. Two-tab HTML includes
`.okmate-type-dot` and `.okmate-tab-label`. Session prefs round-trip
`type_color`. `tabs.js` fetches `/__okmate/peek` and uses
`document_title`.

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`.

**Owner:** `src/peek.rs`, `src/session.rs`, `src/site.rs`, `src/views/mod.rs`,
`templates/base.html`, `assets/tabs.js`, `assets/nav.js`, `assets/app.css`.

## Status

Phases 1–4 implemented on `tab-gestures`. Extends
[peek previews and document tabs](/plans/okmate/peek-and-tabs.md).[^peek]

[^peek]: Shipped tab strip already required current-tab navigation and Cmd-click; Cmd+T was out of bound there.
[^overview]: Application crate owns Askama, Axum, Datastar, and optional desktop preview.
[^session]: `persist_open_path_to` pushes a tab when the route is new; `normalize_tabs` dedupes by path.
[^tabs-js]: `afterPatch` upsert-inserts every mounted route.
[^nav-js]: Capture-phase Cmd/Ctrl+click already calls `openHref`.
[^pages]: Datastar GET persists `open_path` on every document navigation.
[^desktop]: Okmate `preview()` HostOptions; no tab shortcut flag today.
[^prefs-test]: Two-tab full HTML includes `#okmate-tabs`; fragment does not.
[^h35-menu]: File Close Window accelerator is Cmd/Ctrl+W.
[^h35-preview]: `is_close_key_event` exits the preview loop.
[^h35-history]: Host IPC enum has no close-window variant.
[^peek-rs]: Peek already returns `type` and `document_title`; tab strip used the path when those were missing.
[^type-color]: Same hash palette as nav leaves and article meta.
[^base]: Tab strip Askama; title currently a bare button string.
