---
type: Status
title: Heterogeneous bundle curation prototype
description: A fixture handbook froze six reader questions, kept existing concept IDs, and met lexical retrieval; sibling registered roots were assessed as proposals only and were not migrated.
tags: [domain/okmate, domain/okf, concern/authoring, concern/validation]
status: draft
generated: { by: process:cursor, at: 2026-09-12T11:30:00Z }
stale_after: 2026-12-12
authority: descriptive
owners: [human:nils]
sources:
  - id: plan
    resource: ../plans/okmate/heterogeneous-bundle-authoring.md
    title: Support heterogeneous OKF bundle authoring
  - id: research
    resource: ../research/okmate/heterogeneous-bundle-authoring.md
    title: Registry inventory, public examples, and implementation findings
  - id: handbook
    resource: ../../docs/examples/engineering-handbook/index.md
    title: Fixture handbook root questions
  - id: handbook-bench
    resource: ../../docs/examples/engineering-handbook/retrieval.toml
    title: Frozen handbook retrieval questions
  - id: archive
    resource: ../../docs/examples/software-archive/architecture/system-overview.md
    title: Fixture product contract
  - id: archive-bench
    resource: ../../docs/examples/software-archive/retrieval.toml
    title: Product-contract retrieval question
  - id: tests
    resource: ../../tests/curation.rs
    title: Navigation, retrieval, and wrong-scope checks
---

# Heterogeneous bundle curation prototype

## Snapshot date

2026-09-12.

This is a fixture prototype, not a migration of the five registered
roots. Sibling checkouts were not edited.[^plan][^research]

## Frozen reader questions

Intended targets were written on the handbook root before adding the
evaluate-results guide and dated model comparison.[^handbook]

| Reader question | Navigation | Intended concept | Retrieval 2026-09-12 |
| --- | --- | --- | --- |
| How should I steer delegated work? | Root → Task steering | `agentic-development/task-steering` | hit rank 1, MRR 1.0 |
| How should I evaluate agent results? | Root → Evaluate agent results | `agentic-development/evaluate-results` | hit rank 1; dated `research/model-comparison` also returned |
| How do I restore a development environment? | Root → Restore from a manifest | `developer-environments/restore-from-manifest` | hit rank 1 |
| Where did these conclusions come from? | Root → Source register | `reference/source-register` | hit rank 1 |
| What was observed on this machine? | Root → Backup snapshot (not the restore guide) | `audits/backup-snapshot` | hit rank 1 |
| Where is the owning product contract? | Root states not this corpus → product archive Architecture | `architecture/system-overview` in `docs/examples/software-archive` | handbook search does not return that id; archive retrieval hit rank 1 |

Handbook retrieval: 5/5, hit rate 1.0, mean reciprocal rank 1.0, threshold
met. Archive retrieval: 1/1, hit rate 1.0. `okmate check` on both examples
stayed clean on their declared profiles. Existing handbook concept IDs
were preserved; two records were added rather than renamed.[^handbook-bench][^archive-bench][^tests]

Wrong-scope: a handbook lexical search for `portable engine parses OKF`
does not surface a copied product architecture record. That question
belongs in the software-archive fixture.[^archive][^tests]

The source register in the fixture is a Reference, not a Research Report.
The restore guide has no `domain/agentic` tag. The backup snapshot remains
an Audit.[^tests][^research]

## Per-bundle assessment (proposals only)

Fit judgments reuse the 2026-09-12 inventory; this phase did not re-count
sibling catalogs or move files there.[^research]

| Bundle | Assessment | Physical move in this phase |
| --- | --- | --- |
| Okmate `knowledge/` | Keep the type-first work archive. `architecture/system-overview` already states current contracts; a longer capability map is optional later, not a folder rename. | None |
| Rocci | Keep the type-first product archive and area nesting. Improve current-state entry points without broad ID churn. | None (sibling not opened) |
| developer-knowledge | The fixture shows a question-oriented front door, a Reference source register, and a guide distilled from dated research. At eleven records a rewritten root may be enough; the illustrative topic folders remain a later option. | None (sibling not opened). Preserve those eleven IDs if a later move is requested. |
| H35 | Keep the private operations boundary. Add runbook navigation when real runbooks exist; do not invent empty product collections. | None |
| rocci-spotify | Keep the two-record research/plan layout. Add collections only when records require them. | None |

No broad move is justified by aesthetics. A later whole-root rename and a
concept move have different identity effects: collect inbound Markdown
links, `sources[].resource`, indexes, benchmarks, agent routing, registry
and Git subpaths, published links, and preview state before touching
paths. Do not add redirect stub concepts merely to clear link
findings.[^plan]

## Remaining uncertainty

- Sibling retrieval was not re-run on developer-knowledge's live eleven
  records; fixture hit rates do not prove that corpus would score the
  same after a root-index edit.
- Evaluate-results also returns the dated comparison at rank 2. That is
  acceptable evidence adjacency, not a reason to merge the types.
- Hosted CI for this revision is not claimed here.

[^plan]: Phase 6 bound: fixture first; assessments are not migrations.
[^research]: Inventory fit table and developer-knowledge navigation recommendation.
[^handbook]: Frozen question list on the example root index.
[^handbook-bench]: Five lexical questions against the handbook example.
[^archive]: Product facts stay in the software-archive example.
[^archive-bench]: One lexical question against the software-archive example.
[^tests]: `tests/curation.rs` navigation, retrieval, and wrong-scope assertions.
