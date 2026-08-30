---
type: Architecture
title: OKMate system overview
description: OKMate is a standalone OKF application; the portable engine lives in okf/ and the okmate binary owns CLI, HTML, and desktop preview.
tags: [domain/okmate, domain/okf, concern/architecture]
status: draft
generated: { by: process:cursor, at: 2026-08-30T09:40:00Z }
stale_after: 2026-11-26
authority: descriptive
owners: [human:nils]
sources:
  - id: readme
    resource: ../../README.md
    title: OKMate README
    author: process:cursor
    last_modified: 2026-08-27
  - id: okf-readme
    resource: ../../okf/README.md
    title: Portable OKF engine
    author: process:git
    last_modified: 2026-08-26
---

# OKMate system overview

## Current contract

OKMate (open knowledge mate) is a standalone knowledge application for Open
Knowledge Format (OKF) bundles. The CLI binary is `okmate`. The portable engine
is the `okf/` crate in this repository: UI-neutral parse, validate, search,
and artifact generation.[^readme][^okf-readme]

The application crate owns Askama HTML, Axum HTTP, official Datastar morph/SSE,
and an optional tao/wry/rfd desktop window. It does not interpret `.rocci`
templates and must not depend on any `rocci-*` crate.[^readme]

Settings live under `~/.okmate/` (`OKMATE_CONFIG`, `OKMATE_CACHE`,
`OKMATE_STATE`). If `~/.okmate/config.toml` is missing, OKMate may import
`~/.rocci/okf.toml` once. Do not treat `~/.rocci/` as the long-term path.[^readme]

This repository's `knowledge/` bundle is the canonical OKMate discussion
database. Rocci keeps its own inert `knowledge/` and checks it with this
binary (`okmate check knowledge --profile base`). This repository uses
`--profile strict` for owners and evidence.[^readme]

## Boundaries

| Surface | Owner |
| --- | --- |
| Parse, graph, search, build artifacts | `okf/` |
| CLI, live preview, Askama site, desktop | this crate |
| Canonical product knowledge | `knowledge/` |
| Local CI replay, release, PR checkout | `okmate-ops` |

`check`, `inspect`, `search`, and `build` stay single-root. Agents list
resolved folders first with `okmate roots --format paths`.[^readme]

[^readme]: Current repository overview, CLI, and settings paths.
[^okf-readme]: Engine scope and profile names.
