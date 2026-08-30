---
type: Implementation Plan
title: Bootstrap okmate agents and migrate OKF knowledge
description: Add agent setup, okmate-ops, a local OKF bundle, and move engine/app discussions from Rocci with pointer stubs.
tags: [domain/okmate, domain/okf, concern/agents, concern/tooling]
status: draft
generated: { by: process:cursor, at: 2026-08-26T16:30:00Z }
stale_after: 2026-11-26
authority: exploratory
owners: [human:nils]
sources:
  - id: agents
    resource: ../../../AGENTS.md
    title: Okmate agent instructions
    author: process:cursor
    last_modified: 2026-08-26
  - id: overview
    resource: ../../architecture/system-overview.md
    title: Okmate system overview
    author: process:cursor
    last_modified: 2026-08-26
---

# Bootstrap okmate agents and migrate OKF knowledge

## Goal

Give this repository its own agent surface and canonical knowledge bundle, and
move engine/app discussions out of Rocci with pointer stubs.[^agents][^overview]

## Out of bound

- Okmate feature work (settings, multi-roots, viewer)
- Renaming engine `Profile::Rocci`
- Moving Rocci compile/render or site-lane records
- Pushing remotes

## Constraints that do not move

- No `rocci-*` crate dependency
- Records stay inert Markdown
- `--profile strict` remains the strict owners-and-evidence profile name

## Phases

### Phase 1 — Agent setup

**Bound:** `AGENTS.md`, skills, Cursor rules, README `dev` tag docs.

**Exit:** Those files exist on `okmate-agent-knowledge`.

### Phase 2 — okmate-ops

**Bound:** `tools/okmate-ops` with `ci`, `pr-checkout`, and `promote tag`.

**Exit:** `uv run --directory tools/okmate-ops --group dev pytest`

### Phase 3 — Knowledge bundle

**Bound:** Minimal `knowledge/` plus system overview and area decision.

**Exit:** `okmate check knowledge --profile strict --format terminal`

### Phase 4 — Move discussions

**Bound:** Copy listed Rocci OKF/okmate records into this bundle and rewrite sources.

**Exit:** `okmate check knowledge --profile strict --format terminal`

### Phase 5 — Rocci stubs

**Bound:** Pointer stubs in Rocci at the same paths.

**Exit:** `okmate check knowledge --profile base --format terminal` from Rocci.

[^agents]: Repository agent instructions and owning-layer table.
[^overview]: Engine versus application versus knowledge ownership.
