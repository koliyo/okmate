---
type: Research Report
title: Restructure alternatives for registered OKF bundles
description: For the five registered roots, prefer navigation and new citing records over concept moves; Okmate and Rocci stay type-first, developer-knowledge gets a question front door, H35 and rocci-spotify wait for content.
tags: [domain/okmate, domain/okf, concern/authoring, concern/architecture]
status: draft
generated: { by: process:cursor, at: 2026-09-12T11:50:00Z }
stale_after: 2026-12-12
authority: exploratory
owners: [human:nils]
sources:
  - id: inventory
    resource: heterogeneous-bundle-authoring.md
    title: Registry inventory, public examples, and implementation findings
  - id: plan
    resource: ../../plans/okmate/heterogeneous-bundle-authoring.md
    title: Support heterogeneous OKF bundle authoring
  - id: exec
    resource: ../../plans/okmate/bundle-restructure-alternatives.md
    title: Apply preferred restructure alternatives on registered bundles
  - id: prototype
    resource: ../../status/heterogeneous-bundle-curation.md
    title: Fixture handbook questions and retrieval
  - id: authoring
    resource: ../../../docs/authoring.md
    title: Checked-in authoring guide
  - id: handbook
    resource: ../../../docs/examples/engineering-handbook/index.md
    title: Question-oriented handbook example
  - id: archive
    resource: ../../../docs/examples/software-archive/index.md
    title: Type-first software-archive example
  - id: operations
    resource: ../../../docs/examples/operations/index.md
    title: Service and runbook operations example
  - id: nested
    resource: ../../decisions/nested-okf-collections.md
    title: Nest collections under okf, okmate, and ops
  - id: modelling
    resource: bundle-modelling.md
    title: Type-first work archive versus domain-first product map
  - id: overview
    resource: ../../architecture/system-overview.md
    title: OKMate system overview
  - id: catalog
    resource: Local okmate inspect --profile strict catalog on each registered root, 2026-09-12, this checkout
    title: Re-counted concept IDs and types
    author: process:cursor
  - id: developer-index
    resource: ../../../../developer-knowledge/knowledge/index.md
    title: Developer-knowledge root index
  - id: developer-arch
    resource: ../../../../developer-knowledge/knowledge/architecture/developer-knowledge.md
    title: Developer-knowledge bundle contract
  - id: developer-sources
    resource: ../../../../developer-knowledge/knowledge/research/agentic/agentic-development-sources.md
    title: Agentic source register record
  - id: developer-brew
    resource: ../../../../developer-knowledge/knowledge/research/tools/homebrew-timemachine.md
    title: Homebrew and Time Machine research
  - id: h35-index
    resource: ../../../../h35-internal/knowledge/index.md
    title: H35 internal root index
  - id: h35-arch
    resource: ../../../../h35-internal/knowledge/architecture/h35-internal.md
    title: H35 internal repository contract
  - id: rocci-index
    resource: ../../../../rocci/knowledge/index.md
    title: Rocci knowledge root index
  - id: rocci-arch
    resource: ../../../../rocci/knowledge/architecture/index.md
    title: Rocci architecture collection
  - id: spotify-index
    resource: ../../../../rocci-spotify/knowledge/index.md
    title: rocci-spotify root index
---

# Restructure alternatives for registered OKF bundles

## Claim

Do not apply one folder template to the five registered roots. Prefer
**keep IDs and change the front door** over moving files. Physical
restructures are later alternatives with identity costs, not the default
next step. This report does not authorize migrations or edit sibling
working trees.[^inventory][^plan][^catalog] An
[implementation plan](/plans/okmate/bundle-restructure-alternatives.md)
follows the preferred sequence; writing it does not start a phase.[^exec]

## Method and counts

2026-09-12, this Okmate checkout (`9a3fa42`). Re-inspected each enabled
local directory from `okmate roots --format json --no-sync` with
`okmate inspect --profile strict catalog`. Sibling files were read for
navigation and metadata only. Product bodies were not copied here.
Rocci had nine dirty status lines and rocci-spotify one; those trees
were not cleaned to make the catalog neater.[^catalog][^inventory]

| Registry ID | Concepts (this inspect) | Types (this inspect) | Sibling HEAD |
| --- | --- | --- | --- |
| developer-knowledge | 11 | 9 Research Report, 1 Architecture, 1 Audit | `d327fca` (clean) |
| h35-internal | 6 | 3 Research Report, 2 Implementation Plan, 1 Architecture | `2138665` (clean) |
| okmate | 47 | 23 Implementation Plan, 17 Research Report, 2 Audit, 2 Decision, 2 Status, 1 Architecture | `9a3fa42` (this repo) |
| rocci | 192 | 85 Implementation Plan, 68 Research Report, 14 Audit, 10 Decision, 5 Architecture, 5 Status, 2 Design Standard, 2 Reference, 1 Case Study | `d0be666` (9 dirty lines) |
| rocci-spotify | 2 | 1 Implementation Plan, 1 Research Report | `5b049a6` (1 dirty line) |

Earlier inventory counts (44 / 190 for Okmate / Rocci) are the morning
baseline before later records in those trees. Directional fit is
unchanged.[^inventory]

## Move classes (identity effects)

OKF concept identity is the path without `.md`. Alternatives below use
these classes. A whole-root rename and a concept move are not the same
operation.[^authoring][^plan]

| Class | What changes | Typical cost |
| --- | --- | --- |
| A. Root-index only | Headings and links on `index.md` | IDs stable |
| B. Extra indexes | New collection `index.md` files that link existing records | IDs stable |
| C. Distill | New concept cites old evidence; old record stays | New IDs only |
| D. Retype / retag | Frontmatter on the same path | ID stable; search and Strict vocabulary shift |
| E. Concept move | File path changes | Update inbound Markdown, `sources[].resource`, indexes, benchmarks, agents, registry subpaths, published links, preview state |
| F. Whole-root rename | Bundle directory name | Relative IDs may hold; registry and external paths change |
| G. Split / merge bundles | Audience or secrecy boundary | Two corpora, not two folders |

Do not add redirect stub concepts merely to clear broken-link findings.
Do not mark guidance verified because it was reorganized.[^plan][^authoring]

The engineering-handbook and software-archive examples are checked-in
analogs of classes A–C, not migrations of these five roots. A fixture
retrieval pass is not evidence that sibling search would score the
same.[^prototype][^handbook][^archive]

## developer-knowledge

Present layout is a type-first tree whose root lists Architecture,
Decisions, Status, Plans, Research, and Audits. Decisions, plans, and
status collections have no concepts. Research already nests `agentic/`
(six reports, including a source register typed Research Report) and
`tools/` (three reports tagged `domain/agentic` even for Homebrew / Time
Machine). The architecture record says product contracts stay in sibling
repos.[^developer-index][^developer-arch][^developer-sources][^developer-brew][^catalog]

| Alternative | Class | Do this when | Do not do this when |
| --- | --- | --- | --- |
| **A. Question front door, keep eleven IDs** (preferred) | A, maybe D | Readers cannot see steering, evaluation, restore, and citations from the root | You need a folder aesthetic |
| B. Topic indexes only | B | The root questions help but research/ is still a dump | You invent empty topic folders |
| C. Distill one or two guides | C | A dated report is still used as standing practice | You retitle the report as a how-to |
| D. Move into `agentic-development/` and `developer-environments/` | E | A and B still fail navigation after a frozen-question check | You lack an old/new path map for all eleven IDs |
| E. Copy Okmate or Rocci contracts into this handbook | G | Never for this corpus | Product facts belong in those bundles |

Preferred sequence: A, then D on the source register (Reference, not
Research Report) and on tools tags (`domain/developer-environments` or
similar instead of `domain/agentic` for backup material), then C for a
restore guide that cites the Time Machine audit, then B if needed. C’s
physical topic tree stays a last resort. The handbook example already
exercised A–C without renaming its original four IDs.[^inventory][^prototype][^handbook]

## Okmate `knowledge/`

Present layout is type-first with unapproved area nesting under plans,
research, and audits (`okf/`, `okmate/`, `ops/`). Architecture is one
current-contract record. That matches a product work archive, not a
practice handbook.[^overview][^nested][^modelling][^catalog]

| Alternative | Class | Do this when | Do not do this when |
| --- | --- | --- | --- |
| **A. Keep type-first areas; extend the current map** (preferred) | A / C | Implementers file plans and research by type and area | You want a consumer handbook |
| B. Question bullets on the root | A | Agents miss architecture, decisions, and status | Questions replace type collections |
| C. Topic folders (viewer, authoring, …) | E | Recurring retrieval demand by topic, with a path map | Aesthetics or matching developer-knowledge |
| D. Domain-first okf-gem map | E / G | Not recommended for this archive | Work-product pairing of plan and research would scatter |

Preferred: A, with B as a cheap extra. Do not export this bundle’s
`okf/okmate/ops` area vocabulary into every other root. Do not flatten
the five registered roots into one corpus because discovery can list
them.[^nested][^plan]

## Rocci

Present layout is the same type-first idea at larger scale, with
product-area nesting (`rocci`, `rocdown`, `site`, `okf`, `ops`,
`shared`). The root already points at architecture, status, reference,
and a consolidation record. Five architecture concepts exist as a
current-state map if the root sent readers there first.[^rocci-index][^rocci-arch][^catalog]

| Alternative | Class | Do this when | Do not do this when |
| --- | --- | --- | --- |
| **A. Keep type-first areas; lead with architecture and status** (preferred) | A | The problem is “what is current,” not “wrong folders” | You treat 192 IDs as cheap to rename |
| B. Filter historical vs current in indexes | B | Old plans bury shipped contracts | You nest folders by lifecycle |
| C. Flatten product areas | E | Areas no longer match ownership | Scale still justifies nesting |
| D. Split a second bundle | G | A product line cannot share owners, cadence, or secrecy | Convenience of a smaller sidebar |

Preferred: A plus B. Lifecycle findings on this tree are not a structure
score; do not “fix” folders to silence provenance warnings.[^inventory][^plan]

## H35 internal

Present layout is type-first ops-area records plus empty decisions,
status, and audits indexes. Architecture states a private host-ops
boundary; product crates stay elsewhere.[^h35-index][^h35-arch][^catalog]

| Alternative | Class | Do this when | Do not do this when |
| --- | --- | --- | --- |
| **A. Keep the ops work archive** (preferred) | — | Six records still match origin/logs/monitoring work | You fill empty product collections |
| B. Service / host indexes | B | Several records share a host and readers start from the host | You create empty `services/` |
| C. Add runbooks | C | A real operational trigger exists | You copy the software-project six-folder scaffold |
| D. Mirror Okmate type+area depth | B / E | Volume actually reaches Rocci-like scale | Six records |

Preferred: A until a trigger exists, then C (and B if hosts multiply).
The operations example is the analog, not a template to stamp empty
directories.[^operations][^inventory]

## rocci-spotify

Two concepts, paired plan and research, root lists only those
collections. Rocci platform contracts stay in Rocci.[^spotify-index][^catalog]

| Alternative | Class | Do this when | Do not do this when |
| --- | --- | --- | --- |
| **A. Keep the two-record pair** (preferred) | — | The app-local question is still one topic | You want visual parity with Rocci |
| B. Add a collection | B / C | A third record has a different job | You pre-create architecture/decisions/status |
| C. Fold into Rocci | G | Ownership and secrecy actually merged | Platform vs app-local split still holds |

Preferred: A.

## What not to do across all five

- One universal folder set (the old init six collections, or Okmate’s
  area names) as a migration target.[^authoring][^plan]
- Merged-nav path equality as shared meaning (`reference/` in two
  bundles).[^plan]
- Handbook copies of product architecture.[^handbook][^archive][^developer-arch]
- Redirect stubs, invented verification, or sibling edits “to try” a
  layout.[^plan]

## Remaining uncertainty

Sibling lexical retrieval was not re-run against a rewritten
developer-knowledge root. Rocci’s dirty tree means today’s 192 is a
working-tree catalog, not a clean HEAD snapshot. Choosing class E still
needs a per-ID path map produced in that bundle’s own git, not in this
report.[^prototype][^catalog]

[^inventory]: Morning survey fit table and developer-knowledge model.
[^exec]: Preferred A–D on live bundles; class E behind a review gate.
[^plan]: Phase 6: proposals, not migrations; identity checklist for later moves.
[^prototype]: Fixture A–C prototype; sibling roots were not opened for edits.
[^authoring]: Path is identity; types and folders are independent; distill don’t retitle.
[^handbook]: Frozen questions; product contract stays out of the handbook.
[^archive]: Type-first product archive analog.
[^operations]: Service and runbook navigation analog.
[^nested]: Unapproved Okmate-local area nesting; not a global rule.
[^modelling]: Keep type-first for this product’s work archive.
[^overview]: Okmate knowledge is the product discussion database.
[^catalog]: 2026-09-12 strict catalogs on the five registered directory roots.
[^developer-index]: Type-first root with empty decisions/plans/status indexes.
[^developer-arch]: Personal notes and tools; product contracts stay in siblings.
[^developer-sources]: Source register is still `type: Research Report`.
[^developer-brew]: Tools research tagged `domain/agentic`.
[^h35-index]: Same six type collections as a software-project front door.
[^h35-arch]: Private origin operations; not rocci or okmate knowledge.
[^rocci-index]: Type collections plus migration and consolidation pointers.
[^rocci-arch]: Five current architecture records.
[^spotify-index]: Plans and research only; Rocci contracts stay in Rocci.
