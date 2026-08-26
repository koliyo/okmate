---
type: Decision
title: Nest large OKF collections under okf, okmate, and ops
description: Keep type-first top-level folders; nest plans, research, and audits into okf, okmate, and ops. Do not nest by lifecycle. Concept ID remains the path.
tags: [domain/okf, domain/okmate, concern/architecture, concern/authoring]
status: draft
generated: { by: process:cursor, at: 2026-08-26T16:30:00Z }
stale_after: 2026-11-26
authority: exploratory
owners: [human:nils]
sources:
  - id: okf-spec
    resource: https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md
    title: Open Knowledge Format v0.2 specification
    author: organization:google-cloud
  - id: overview
    resource: ../architecture/system-overview.md
    title: Okmate system overview
    author: process:cursor
    last_modified: 2026-08-26
---

# Nest large OKF collections under okf, okmate, and ops

## Context

OKF v0.2 treats a bundle as a recursive directory tree: concept ID is the path
without `.md`, and any directory may have `index.md` for progressive
disclosure.[^okf-spec] Rocci uses a larger closed vocabulary for its product
areas. This repository only needs areas that this product owns.[^overview]

## Decision

- Keep type-first top-level collections (`plans/`, `research/`, `audits/`, and
  the small flat sets).
- Nest only plans, research, and audits under the closed areas `okf/`,
  `okmate/`, and `ops/`.
- Create an area directory only when it has at least one record. Mirror area
  names across those three type collections.
- Prefer bundle-root links (`/plans/okf/okmate.md`). Filename stems stay unique
  under `knowledge/plans/`.
- Keep a single bundle-root `log.md`. Do not add redirect stub concepts.

This decision is not approved.

## Consequences

Inspect IDs and collection indexes follow these paths. Rocci keeps its own
area vocabulary for the Rocci bundle.

## Current disposition

Exploratory draft. Used for this bundle's initial layout.

[^okf-spec]: Bundle tree, concept ID, and per-directory `index.md` (SPEC §§2–3, §8).
[^overview]: Engine versus application versus knowledge ownership.
