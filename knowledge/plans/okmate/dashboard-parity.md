---
type: Implementation Plan
title: Okmate dashboard parity with rocci-okf
description: Restore the old rocci-okf review dashboard content in Askama and match rocci-cli preview port assignment.
tags: [domain/okmate, concern/rendering, concern/tooling]
status: draft
generated: { by: process:cursor, at: 2026-08-26T20:40:00Z }
stale_after: 2026-11-26
authority: exploratory
owners: [human:nils]
sources:
  - id: research
    resource: ../../research/okf/okf-viewer-rust-vs-rocci.md
    title: OKF viewer Rust HTML versus finished Rocci shell
    author: process:cursor
    last_modified: 2026-08-26
  - id: extract
    resource: ../okf/okmate.md
    title: Okmate extractable Rust OKF mate
    author: process:cursor
    last_modified: 2026-08-26
  - id: site
    resource: ../../../src/site.rs
    title: Askama HTML routes and nav
    author: process:git
    last_modified: 2026-08-26
  - id: views
    resource: ../../../src/views/mod.rs
    title: Document and review row views
    author: process:git
    last_modified: 2026-08-26
  - id: cli
    resource: ../../../src/cli.rs
    title: Okmate clap CLI
    author: process:git
    last_modified: 2026-08-26
  - id: preview
    resource: ../../../src/preview.rs
    title: Live preview bind
    author: process:git
    last_modified: 2026-08-26
  - id: review-engine
    resource: ../../../okf/src/review.rs
    title: classify_concept_action
    author: process:git
    last_modified: 2026-08-26
  - id: shell-audit
    resource: ../../audits/okmate/viewer-shell-parity.md
    title: Viewer shell versus last rocci-okf
    author: process:cursor
    last_modified: 2026-08-26
  - id: shell-plan
    resource: viewer-shell-parity.md
    title: Viewer shell parity implementation plan
    author: process:cursor
    last_modified: 2026-08-26
---

# Okmate dashboard parity with rocci-okf

## Goal

Make `okmate view` home, review, and concept pages carry the same governance content as the last rocci-okf Rust shell, and use the same `--port` defaults as rocci-cli.[^research][^extract]

## Out of bound

Graph or search HTML. Review write/approve. Restoring `#okf-*` IDs, `/__rocci_okf/`, or hardcoded `PRIORITY_1_RECORDS`. Settings edge-matrix. Changing `okf` action classification. `ROC_BASIC_WEBSERVER_PORT`. Three-pane resize, outline spy, and keep-nav sidebar belong to [viewer-shell-parity](viewer-shell-parity.md).[^extract][^review-engine][^shell-audit][^shell-plan]

## Constraints that do not move

- HTML stays in this crate. `okf` stays UI-neutral.[^extract]
- Landmarks stay `#okmate-*`. CSS prefix `okmate-`.
- Recent documents use `generated.at` and exclude collection indexes, matching the old leaf list.[^research]
- Needs-action table is derived from `classify_concept_action`, not Rocci concept IDs.[^review-engine]
- Do not mix unrelated working-tree changes into phase commits.

## Phases

### Phase 0 — Preview port assignment

**Bound:** `PortArg` (`auto` / exact). Window default `auto`; `--no-window` default `8000`. Exact occupied ports fail before bind. README documents the defaults.[^cli][^preview]

**Out of bound:** Dashboard HTML. Bind-host changes.

**Tests:** Clap defaults, parse, occupied exact, auto bindable port.

**Exit:** `cargo test -p okmate --no-default-features` and `cargo fmt --all -- --check`.

**Owner:** `src/port.rs`, `src/cli.rs`, `README.md`.

### Phase 1 — View models and CSS

**Bound:** `StatCard`, richer review/recent/diagnostic/meta view types, `governance_stats`, `recent_leaf_documents` helpers, `.okmate-*` styles. Templates may grow unused fields.[^views]

**Out of bound:** Changing home/review/concept markup behavior.

**Tests:** Stats and recent mapping unit tests.

**Exit:** `cargo test -p okmate --no-default-features` and `cargo fmt --all -- --check`.

**Owner:** `src/views/mod.rs`, `assets/app.css`.

### Phase 2 — Home governance

**Bound:** `/` prepends recents, review CTA, stat grid, Knowledge Collections, then `index.md`. Recent list matches the old ten-leaf contract.[^site]

**Out of bound:** Review table and concept meta chrome.

**Tests:** Port `dashboard_lists_ten_recent_leaf_documents`. Build asserts recents, CTA, Total, Knowledge Collections.

**Exit:** `cargo test -p okmate --no-default-features` and `cargo fmt --all -- --check`.

**Owner:** `templates/home.html`, `src/site.rs`.

### Phase 3 — Review queue

**Bound:** `/review/` stat grid, needs-action table, five-column all-concepts table, filter bar, diagnostics, `review.js` after Datastar morph.[^site]

**Out of bound:** Concept meta. Queue writes.

**Tests:** `#okmate-queue`, column headers, `#okmate-search-input`, diagnostics heading.

**Exit:** `cargo test -p okmate --no-default-features` and `cargo fmt --all -- --check`.

**Owner:** `templates/fragments/queue.html`, `assets/review.js`.

### Phase 4 — Concept metadata

**Bound:** Concept pages: badges, trust, stale, action alert, description, provenance, `OKF4006` drift. Not collection indexes.[^views]

**Out of bound:** Home and review chrome.

**Tests:** Fixture with draft/stale/verification asserts badges and alert.

**Exit:** `cargo test -p okmate --no-default-features` and `cargo fmt --all -- --check`.

**Owner:** `templates/fragments/article.html`, `src/site.rs`.

[^research]: Dual-path rocci-okf viewer: Rust governance on home, review, and concept meta.
[^extract]: Okmate is the Askama successor; IDs and prefixes are `#okmate-*` / `/__okmate/`.
[^site]: Current routes are `/`, `/review/`, `/settings/`, and concept/collection pages.
[^views]: Review rows today are title, status, and action only.
[^cli]: `view --port` currently defaults to 8000 for every mode.
[^preview]: Preview binds the requested port and errors if it is taken.
[^review-engine]: Action labels and `is_action_required` come from `okf::classify_concept_action`.
[^shell-audit]: Chrome findings: panes, outline spy, heading IDs, keep-nav.
[^shell-plan]: Implementation phases for shell parity; not this plan.
