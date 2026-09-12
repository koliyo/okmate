---
type: Implementation Plan
title: Apply preferred restructure alternatives on registered bundles
description: Execute the preferred keep-IDs sequence from the restructure-alternatives research—question front doors, semantic retags, distilled guides, and current-state maps—without concept moves, root renames, or copying product knowledge.
tags: [domain/okmate, domain/okf, concern/authoring, concern/architecture]
status: draft
generated: { by: process:cursor, at: 2026-09-12T11:50:00Z }
stale_after: 2026-12-12
authority: exploratory
owners: [human:nils]
sources:
  - id: alts
    resource: ../../research/okmate/bundle-restructure-alternatives.md
    title: Per-root restructure alternatives and identity costs
  - id: inventory
    resource: ../../research/okmate/heterogeneous-bundle-authoring.md
    title: Registry inventory and developer-knowledge model
  - id: tooling
    resource: heterogeneous-bundle-authoring.md
    title: Tooling and fixture-curation plan; sibling moves were out of bound
  - id: prototype
    resource: ../../status/heterogeneous-bundle-curation.md
    title: Fixture handbook questions and retrieval
  - id: authoring
    resource: ../../../docs/authoring.md
    title: Checked-in authoring guide
  - id: handbook
    resource: ../../../docs/examples/engineering-handbook/index.md
    title: Question-oriented handbook analog
  - id: nested
    resource: ../../decisions/nested-okf-collections.md
    title: Unapproved Okmate-local area nesting
  - id: overview
    resource: ../../architecture/system-overview.md
    title: OKMate system overview
  - id: skill
    resource: ../../../.agents/skills/manage-okmate-knowledge/SKILL.md
    title: Author in the named bundle; do not invent verification
---

# Apply preferred restructure alternatives on registered bundles

## Purpose and authority

This plan executes the **preferred** alternatives in
[restructure alternatives for registered OKF bundles](/research/okmate/bundle-restructure-alternatives.md):
keep concept IDs, change the front door, retag in place, and distill new
citing records. It does not start a phase by being written. Class E
concept moves, whole-root renames, and bundle splits stay behind a review
gate.[^alts][^inventory][^tooling]

The [heterogeneous authoring](/plans/okmate/heterogeneous-bundle-authoring.md)
plan owns tooling and the fixture prototype. This plan owns live-bundle
curation in each registered git tree.[^tooling][^prototype]

## Goal

A reader or agent can answer the frozen questions for each corpus from
that bundle's root index, without renaming existing concepts or copying
product contracts into a handbook. Okmate and Rocci remain type-first
work archives. developer-knowledge becomes a handbook front door over
its eleven IDs. H35 and rocci-spotify stay as they are until real new
content exists.[^alts][^authoring]

## Out of bound

- Executing this plan merely because it has been written.
- Class E concept moves, class F root renames, or class G split/merge
  except Phase 7 after a recorded navigation failure and a per-ID path
  map in that bundle's git.[^alts]
- Copying Okmate or Rocci product facts into developer-knowledge.
- One universal folder set (old init six collections, or Okmate
  `okf`/`okmate`/`ops` areas) as a migration target.[^nested][^authoring]
- Filling empty decisions/plans/status collections, empty `services/`,
  or a software-project scaffold on H35 or rocci-spotify.
- Redirect stub concepts, invented `human:` verification, or
  restructuring to silence provenance warnings.
- Flattening the five registered roots into one corpus.
- Folding rocci-spotify into Rocci.
- Editing a sibling working tree from the Okmate checkout.

## Constraints that do not move

- Concept identity remains the path without `.md`. Preserve the eleven
  developer-knowledge IDs listed in the alternatives report until Phase
  7 explicitly maps them.[^alts]
- Author only in the bundle named by the phase, in that repository.
  Okmate phases edit `okmate/knowledge/` only.[^skill]
- Distill: new guides cite dated research or audits; do not retitle
  those records as how-tos.[^authoring][^handbook]
- Product contracts stay in their owning corpus. The handbook names
  that boundary; it does not duplicate architecture bodies.[^alts]
- Separated navigation remains the mixed-scope view; equal collection
  paths are not shared meaning.[^tooling]
- One phase per commit on the owning repo's named branch. Do not mix
  unrelated dirty files (Rocci and rocci-spotify had dirty lines at
  survey time).[^alts]
- Do not log a phase complete until that repo's required hosted CI (if
  any) succeeds; otherwise record local Exit evidence only.[^skill]

## Current behavior

developer-knowledge's root still lists type collections; three are
empty. The source register is a Research Report. Tools research uses
`domain/agentic` for backup material. Okmate already has
`architecture/system-overview` and type+area nesting. Rocci already has
five architecture records and status snapshots; the root does not lead
with “what is current.” H35 has six ops records and empty product
indexes. rocci-spotify has one plan and one research record. Fixture
A–C work lives only under `docs/examples/`.[^alts][^prototype][^overview]

## Phases

### Phase 1 — Freeze questions and measure developer-knowledge

**Bound:** In the developer-knowledge git tree, write a small frozen
question list (steering, evaluation, restore, citations / source
register, audit vs advice, owning product contract as *out of scope*).
Record intended existing concept IDs, lexical retrieval (`okmate
benchmark` or `search`) before any index rewrite, and manual hops from
today's type-first root. Do not edit concept files.

**Out of bound:** Frontmatter changes, new guides, path moves, Okmate
knowledge edits.

**Tests:** The frozen list names the eleven existing IDs it may target;
product-contract is not an in-bundle architecture copy.

**Exit:** Baseline retrieval and navigation notes exist in that repo
(research or status). All eleven IDs still present.

**Owner:** developer-knowledge maintainer.

### Phase 2 — developer-knowledge question front door

**Bound:** Class A. Rewrite `knowledge/index.md` into a short scope
statement plus question links to existing records. Keep type-collection
links if they still help. Preserve all eleven concept IDs.

**Out of bound:** Retype, retag, new concepts, folder moves.

**Tests:** Every frozen in-bundle question is one hop from the root;
`okmate check knowledge --profile strict` has no new errors; IDs
unchanged.

**Exit:** Root questions match the Phase 1 list; navigation no longer
depends on guessing `research/agentic/` vs `research/tools/`.

**Owner:** developer-knowledge maintainer.

### Phase 3 — developer-knowledge semantics and distilled restore guide

**Bound:** Class D then C. Retype
`research/agentic/agentic-development-sources` to Reference if the body
is still a register. Retag Homebrew / Time Machine research off
`domain/agentic` onto environment/reproducibility tags used in that
bundle. Add at most two How-to Guides (restore; optionally evaluate)
that cite existing reports and the Time Machine audit. Do not retitle
the dated reports.

**Out of bound:** Class E moves; copying product architecture; marking
new guides `verified` because they were added.

**Tests:** Source register type is Reference; restore guide has no
`domain/agentic`; audit remains Audit; new IDs only for the guides;
strict check clean enough to separate pre-existing provenance from new
errors.

**Exit:** Frozen restore (and optional evaluate) questions have standing
practice records; evidence records remain research/audit.

**Owner:** developer-knowledge maintainer.

### Phase 4 — Okmate current map and optional root questions

**Bound:** Class A, optionally a short Class C note on
`architecture/system-overview` if the current contract is incomplete.
Add question bullets on this repo's `knowledge/index.md` that point at
architecture, decisions, status, and the authoring/discovery records
without replacing type collections. Do not export `okf`/`okmate`/`ops`
into other bundles.

**Out of bound:** Topic folders that rename IDs; handbook conversion;
sibling edits.

**Tests:** `okmate check knowledge --profile strict`; root still lists
type collections; new bullets resolve.

**Exit:** An implementer can find the current contract and the
heterogeneous-authoring / restructure records from the root.

**Owner:** this repository (`knowledge/`).

### Phase 5 — Rocci lead with architecture and status

**Bound:** Class A plus B in the Rocci git tree. Make the root (and
architecture/status indexes) lead with current contracts and dated
status. Filter or group historical plans in indexes without nesting
folders by lifecycle. Do not flatten product areas. Do not move the
~192 concept IDs. Do not “fix” folders to clear OKF4005/4006 warnings.

**Out of bound:** Class E/F/G; mixing unrelated dirty files into the
phase commit.

**Tests:** Architecture collection still lists the five current records;
root questions or lead bullets resolve; check errors do not increase
from this change.

**Exit:** “What is current?” is answerable from the root without a
folder rename.

**Owner:** Rocci maintainer.

### Phase 6 — H35 keep; rocci-spotify keep

**Bound:** Confirm H35 remains a six-record ops archive. Add a Runbook
only if the maintainer names a real operational trigger in that phase
request; then Class C in h35-internal, citing existing origin/logs
research. Do not create empty `services/` or product collections.
Confirm rocci-spotify stays two records; add a collection only if a
third record with a different job already exists.

**Out of bound:** Software-project scaffold; folding into Rocci;
invented runbooks.

**Tests:** Concept counts stay 6 and 2 unless a named new record is
in the Bound; check does not gain format errors from empty indexes
alone.

**Exit:** Written keep (or one justified runbook / third record) in that
repo's log. No empty folder stamp.

**Owner:** h35-internal and rocci-spotify maintainers.

### Phase 7 — Review gate for class E

**Bound:** Only if Phases 2–3 still fail the frozen-question navigation
check. Produce an old/new path map for every ID to move, plus inbound
Markdown, `sources[].resource`, indexes, benchmarks, agents, registry
subpaths, published links, and preview state. Execute at most the
developer-knowledge topic-tree alternative from the research. No
redirect stubs.

**Out of bound:** Okmate topic folders, Rocci flatten, H35/spotify
renames, or any class E without that map.

**Exit:** Map reviewed; if executed, all mapped links updated and
retrieval does not regress vs Phase 1.

**Owner:** developer-knowledge maintainer; not this plan's default path.

## Delivery order and validation

Phases 1–3 are sequential on developer-knowledge. Phase 4 may run in
parallel in Okmate. Phase 5 is independent in Rocci. Phase 6 is skippable
keep. Phase 7 is a gate, not scheduled work.

When a phase edits Okmate knowledge:

```sh
cargo fmt --all -- --check
okmate check knowledge --profile strict --format terminal
```

When a phase edits another bundle, run that tree's `okmate check
knowledge --profile strict` (or `--profile base` if that is the
bundle's published contract) and do not treat pre-existing provenance
warnings as a reason to move files.[^skill][^alts]

[^alts]: Preferred A–D sequence; E/F/G last; do not copy product contracts.
[^inventory]: Fit table and eleven developer-knowledge IDs.
[^tooling]: Fixture prototype and discovery; sibling curation was not in those phases.
[^prototype]: Analog of A–C; sibling hit rates were not measured.
[^authoring]: Path is identity; distill, do not retitle.
[^handbook]: Frozen questions; product contract stays out of the handbook.
[^nested]: Unapproved local area nesting; not a global migration target.
[^overview]: Okmate knowledge is the product discussion database.
[^skill]: Named-bundle authoring; no invented human verification.
