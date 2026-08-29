---
type: Research Report
title: Knowledge-bundle modelling in OKMate versus okf-gem
description: OKMate's type-first work-product tree is strong on evidence metadata and paired research/plans; okf-gem's domain-first, constraint-versus-decision, code-pinned bundles are stronger as a standing map an agent can orient in — adopt the map habits, not the five-bundle split or bare tags.
tags: [domain/okmate, domain/okf, concern/architecture, concern/authoring, concern/retrieval]
status: draft
generated: { by: process:cursor, at: 2026-08-29T08:55:00Z }
stale_after: 2026-11-29
authority: exploratory
owners: [human:nils]
sources:
  - id: nested
    resource: ../../decisions/nested-okf-collections.md
    title: Nest collections under okf, okmate, and ops
    author: process:cursor
    last_modified: 2026-08-26
  - id: overview
    resource: ../../architecture/system-overview.md
    title: OKMate system overview
    author: process:cursor
    last_modified: 2026-08-27
  - id: skill
    resource: ../../../.agents/skills/manage-okmate-knowledge/SKILL.md
    title: In-repo manage-okmate-knowledge skill
    author: process:git
    last_modified: 2026-08-28
  - id: okmate-index
    resource: ../../index.md
    title: OKMate knowledge root index
    author: process:git
    last_modified: 2026-08-26
  - id: research-index
    resource: index.md
    title: OKMate research collection index
    author: process:cursor
    last_modified: 2026-08-29
  - id: log
    resource: ../../log.md
    title: OKMate knowledge log
    author: process:cursor
    last_modified: 2026-08-29
  - id: serradura
    resource: serradura-okf.md
    title: OKMate versus serradura/okf
    author: process:cursor
    last_modified: 2026-08-29
  - id: inspect
    resource: ../../../okf/src/lib.rs
    title: okmate inspect catalog and graph (measured 2026-08-29)
    author: process:cursor
    last_modified: 2026-08-29
  - id: authoring
    resource: https://github.com/serradura/okf/blob/26ce927090dbbe33c58d60453f30c217271596e1/skills/okf/reference/authoring.md
    title: okf-gem authoring craft
    author: human:rodrigo-serradura
    last_modified: 2026-08-28
  - id: index-first
    resource: https://github.com/serradura/okf/blob/26ce927090dbbe33c58d60453f30c217271596e1/skills/okf-principles/references/index-first.md
    title: Index-first progressive disclosure
    author: human:rodrigo-serradura
    last_modified: 2026-08-28
  - id: structure-bundle
    resource: https://github.com/serradura/okf/blob/26ce927090dbbe33c58d60453f30c217271596e1/.okf/decisions/structure-is-a-bundle.md
    title: A gem's structure is a bundle
    author: human:rodrigo-serradura
    last_modified: 2026-08-19
  - id: where
    resource: https://github.com/serradura/okf/blob/26ce927090dbbe33c58d60453f30c217271596e1/.okf/design/where-knowledge-lives.md
    title: README, AGENTS.md, and bundle as three readers
    author: human:rodrigo-serradura
    last_modified: 2026-08-28
  - id: nothing-runs
    resource: https://github.com/serradura/okf/blob/26ce927090dbbe33c58d60453f30c217271596e1/.okf/design/nothing-runs-it.md
    title: A rule nothing runs
    author: human:rodrigo-serradura
    last_modified: 2026-08-19
  - id: eco-index
    resource: https://github.com/serradura/okf/blob/26ce927090dbbe33c58d60453f30c217271596e1/.okf/index.md
    title: okf-eco root index
    author: human:rodrigo-serradura
    last_modified: 2026-08-28
  - id: eco-overview
    resource: https://github.com/serradura/okf/blob/26ce927090dbbe33c58d60453f30c217271596e1/.okf/overview.md
    title: The OKF ecosystem at a glance
    author: human:rodrigo-serradura
    last_modified: 2026-08-19
  - id: kernel-index
    resource: https://github.com/serradura/okf/blob/26ce927090dbbe33c58d60453f30c217271596e1/gems/okf/.okf/index.md
    title: okf gem knowledge index
    author: human:rodrigo-serradura
    last_modified: 2026-08-28
  - id: capabilities
    resource: https://github.com/serradura/okf/blob/26ce927090dbbe33c58d60453f30c217271596e1/gems/okf/.okf/capabilities/index.md
    title: okf-gem capabilities index
    author: human:rodrigo-serradura
    last_modified: 2026-08-28
  - id: cross-links
    resource: https://github.com/serradura/okf/blob/26ce927090dbbe33c58d60453f30c217271596e1/.okf/format/cross-links.md
    title: Cross-links and @slug as prose
    author: human:rodrigo-serradura
    last_modified: 2026-08-13
  - id: gem-okf
    resource: https://github.com/serradura/okf/blob/26ce927090dbbe33c58d60453f30c217271596e1/.okf/gems/okf.md
    title: okf kernel component
    author: human:rodrigo-serradura
    last_modified: 2026-08-19
---

# Knowledge-bundle modelling in OKMate versus okf-gem

## Scope

This record compares the **stored bundles**, not the CLIs. The tool
comparison is [OKMate versus serradura/okf](serradura-okf.md). Counts
are from `okmate inspect --profile base catalog|graph` on 2026-08-29
against this repository's `knowledge/` and the local okf-gem checkout
at `26ce927` (five registered trees: `.okf/` plus one `.okf/` per
gem).[^inspect][^serradura][^eco-index]

The question is which modelling habits make a bundle stay **orientable
and true** as it grows: topology, type vocabulary, metadata, linking,
indexes, and the log.

## Two jobs, two trees

OKMate's `knowledge/` is a **work-product archive**: plans, research,
audits, a thin architecture/decision/status layer. The root index is
six type folders. Large collections nest under closed areas `okf/`,
`okmate/`, `ops/` — type first, area second, never by
lifecycle.[^okmate-index][^nested][^skill]

okf-gem's trees are a **standing map of a system**. `@okf-eco` is
gems, plugin, skills, resources, decisions, design, format. `@okf`
is model, capabilities, structure, design, testing. Directories follow
the product, not the document genre. Their authoring craft states that
as a rule: organize by domain, not by type; a `types/`-first layout
scatters related concepts.[^eco-index][^kernel-index][^authoring]

Both are legal OKF. They optimize different readers. An agent arriving
at OKMate asks "what are we deciding or building?" An agent arriving
at `@okf` asks "what is this gem, and which file owns it?"

## Shape on 2026-08-29

| Bundle | Concepts | Types (top) | Edges | Isolated | Broken |
| --- | --- | --- | --- | --- | --- |
| OKMate `knowledge/` | 33 | Implementation Plan 17, Research Report 11 | 77 | 4 | 10 |
| `@okf-eco` | 27 | Component 10, Constraint 6, Format 5 | 45 | 0 | 0 |
| `@okf` | 34 | Component 15, Capability 9, Constraint 8 | 219 | 0 | 0 |
| `@okf-mcp` | 15 | Constraint 5, Capability 4 | 27 | 0 | 0 |
| `@okf-tui` | 31 | Decision 11, Component 7 | 87 | 0 | 0 |
| `@okf-pro` | 37 | Decision 12, Learning 8 | 70 | 0 | 0 |

[^inspect]

OKMate is plan-heavy and still small. The kernel bundle is the same
size and **six times denser**. Eco and kernel have no isolates and no
broken edges. OKMate has four unlinked concepts
(`decisions/nested-okf-collections`, `plans/okmate/agent-knowledge`,
`plans/ops/okmate-ops`, `research/okmate/leptos`) and ten leftover
broken links into migrated Rocci paths. Those are hygiene, not
modelling philosophy — but they are exactly the findings okf-gem's
`lint` would surface as reachability and backlog.[^inspect][^serradura]

Inbound hubs tell the same story. OKMate's most-cited nodes are
**migrated plans** (`plans/okf/settings-ux`,
`okf-viewer-rust-datastar`, `okmate`). Eco's hubs are **standing
constraints** (`design/nothing-runs-it`, `format/okf-format`,
`design/extension-points`). Kernel hubs are live capabilities
(`graph-server`, `cli`, `registry`, `search`). A productive bundle
points at the current map, not the excavation
trail.[^inspect][^nothing-runs]

## Topology

OKMate's type-first split is the right default for **work that
arrives as a genre**. A plan is not a decision until someone promotes
it; research is evidence, not contract. Pairing
`research/okmate/<stem>` with `plans/okmate/<stem>` keeps those two
jobs from collapsing into one file — the skill already requires
that.[^skill][^nested][^research-index]

okf-gem is right for **product knowledge**. `capabilities/` is what
the gem does; `structure/` is which `lib/` file owns it;
`design/` is the enforced boundary; `model/` is the in-memory
objects. An agent adding a verb reads `testing/adding-a-verb` and
`structure/the-cli`, not a research report about CLIs in
general.[^kernel-index][^capabilities][^structure-bundle]

Do not flatten OKMate into domain folders (`engine/`, `desktop/`).
That would scatter every plan/research pair and fight the closed
area vocabulary. The synthesis is: **keep type-first for
work-products; grow a domain-shaped canonical layer**
(architecture, decisions, and — if the crate is ever mapped —
`structure/` / `capabilities/`) the way `@okf` does.[^nested][^authoring]

okf-gem also splits **one fact across five bundles** so a concept
cannot link out. Cross-bundle references are prose `` `@okf
capabilities/linter` `` because `@slug` is not a Markdown
target.[^cross-links][^eco-index] OKMate should stay **one bundle**
until a crate actually needs its own map. Slug addressing on
`roots` is a viewer problem, not a reason to fork
`knowledge/`.[^serradura]

## Types

OKMate's types are **document genres**: Implementation Plan, Research
Report, Audit, Status, Architecture, Decision. They match the
collection folders and the skill's routing table. That is easy to
author and easy to filter.[^skill][^inspect]

okf-gem's types are **roles in a system**:

| Type | Job |
| --- | --- |
| Overview | One glance at the whole tree |
| Capability | A thing the product does |
| Component | A part that exists (gem, file, panel) |
| Constraint | Standing structure; enforced or named as unenforced |
| Decision | A fork that could have gone otherwise |
| Format | Spec substrate shared by every surface |
| Playbook | How a change or verb is walked |
| Learning / Finding | Personal/process residue (`@okf-pro`, `@okf-tui`) |

The useful split for OKMate is **Constraint versus Decision**.
okf-gem's design indexes say it plainly: decisions have a date and a
reversal cost; constraints are the structure those decisions sit
on, and something must run them or the record must say nothing
does.[^eco-index][^nothing-runs][^where]

OKMate currently has one Architecture record and two Decision
drafts, both `authority: exploratory` and marked not
approved. There is no Constraint type. Standing rules (crate
boundary, inert Markdown, single-root query) live in architecture
prose, `AGENTS.md`, and the skill — three places, which is the
failure [where knowledge lives](https://github.com/serradura/okf/blob/26ce927090dbbe33c58d60453f30c217271596e1/.okf/design/where-knowledge-lives.md)
exists to prevent.[^overview][^where][^skill]

Do not adopt Learning / Finding / Concept as extra genres here.
They fit a personal Pro repo, not a team review bundle.

## Metadata

OKMate is stricter and should stay that way. Every concept has
`authority`, `owners`, `status: draft`, `generated`, keyed
`sources`, and `domain/*` plus `concern/*` tags. Thirty of
thirty-three are `exploratory`; three architecture/audit/status
records are `descriptive`. Trust tier is `generated` on all
thirty-three — no `human:` verification yet.[^inspect][^skill]

okf-gem uses almost none of that. Eco and kernel have descriptions
and `generated.by: human:maintainer`. No `authority`, no `owners`,
no `status`. Tags are bare (`governance`, `cli`, `graph`) — a
connective vocabulary, not a closed prefix scheme. Seventeen of
twenty-seven eco concepts and thirty-one of thirty-four kernel
concepts set `resource` to a path or URL: the concept *is* a real
file, so `maintain` can search by URI.[^inspect][^authoring][^gem-okf]

`@okf-pro` shows the failure mode of going light: thirty-seven
concepts, no tags, one `sources` entry, all `unverified`. Dense
process knowledge with no retrieval
handles.[^inspect]

Keep OKMate's evidence floor. Steal two fields, not their
absence:

1. **`resource`** on a concept that *is* a crate, module, or
   workflow file. Omit it on a decision or research note — omission
   is meaningful.[^authoring]
2. **Promote a small stable core.** A bundle where everything is
   `draft` + `exploratory` cannot tell an agent what is load-bearing.
   Architecture and approved decisions should become `stable` /
   `descriptive` or `normative` when a human verifies them. That is
   this product's own Verify/Promote thesis applied to itself.

Do not replace `domain/` / `concern/` with bare tags. Prefixes are
how Strict and the review filters slice the bundle; bare tags are
better inside a single-product map with a small vocabulary.

## Indexes and progressive disclosure

okf-gem's index-first principle is the right test: a description
must let an agent **skip** the file, not merely recognize it.
Coverage must match the index's claim. Split on question
boundaries; measure the worst question, not the
mean.[^index-first]

Their capabilities index groups by **job** (Judge / Serve / Use),
not by filename. Cross-bundle siblings appear as prose so the
listing stays complete without lying about edges.[^capabilities]
Root indexes open with an Overview concept, then areas.[^eco-index][^kernel-index]

OKMate indexes are already close: one-line discriminating
descriptions, paired plan/research links, area folders only when
they hold a record.[^research-index][^skill] Gaps:

- No root Overview concept — the index is only a type menu.
- Collection indexes do not say what they **exclude** (historical
  migrated plans still sit in `plans/okf/` and dominate the
  graph).
- Several index lines restated the pairwise report after the
  architecture cut; that is the right habit (update the
  description when the claim widens).

## Linking

OKMate prefers bundle-root `/path.md` links. That survives moves
inside this tree and is what inspect records on edges. Relative
links still appear in nearby indexes. Ten broken edges are leftover
Rocci addresses — demand that nothing will satisfy until the
records are edited or the links become prose about a foreign
bundle.[^inspect][^skill]

okf-gem prefers relative links inside a bundle and **forbids**
path escape. Cross-bundle is backticks, never a fake
`@slug/file.md` edge (that would resolve as a missing local
path). That honesty is why five dense graphs stay
clean.[^cross-links]

When OKMate cites `@okf` or another root, write prose plus a
URL or a `roots` slug — do not invent a Markdown target in this
bundle.

## Log

OKMate's `log.md` is a **session ledger**: "Drafted research…",
"Exploratory; do not log complete until hosted CI succeeds." Useful
for merge-union and for not claiming a phase is done. It is not
what a reader six months out needs.[^log][^skill]

okf-gem's logs are **durable shipped change with the argument**:
what flipped, why the other option was worse, which files had to
move with it. Their authoring rule: the log records what shipped,
not how many passes it took.[^authoring][^eco-overview]

Keep the CI-caveat discipline. Add a durable sentence per
meaningful ship ("Verify/Promote writes only the working tree")
and leave the round-count in the concept or out.

## What holds the map to the code

okf-gem's highest-leverage bundle habit is not a type name. A test
fails when `lib/` has no concept, when two concepts claim one
file, when a concept names a gone file, or when a verb table
disagrees with `OKF::CLI.builtins`. `AGENTS.md` routes; the bundle
argues; README is the menu. The same fact is in one of those three,
not two.[^structure-bundle][^where][^nothing-runs]

OKMate pins collections with the skill and with `okmate check
--profile strict` (owners, footnotes, index membership). It does
not pin `okf/src/*.rs` to concepts. That pin is worth adding
**only if** a `structure/` or capabilities catalog appears.
Without the test, a hand-written crate map will rot the way their
guides did.[^skill][^structure-bundle]

## Adopt, keep, refuse

**Adopt (bundle habits)**

1. A root **Overview** concept that states the product in one
   screen, then the type menu.
2. **Constraint versus Decision**: standing enforced rules in
   architecture (or a `design/` area); dated forks in
   `decisions/` only when they are real choices. Do not mint
   approved Decisions from this report.[^nothing-runs]
3. **`resource`** on concepts that are files. Omit it otherwise.
4. **Zero isolates / zero broken** as hygiene — the four unlinked
   records and ten Rocci leftovers are the current debt.
5. **Durable log lines** beside the session ledger.
6. **Question-grouped indexes** where a folder mixes jobs (for
   example research that is landscape versus research that is a
   paired plan).
7. **Code-to-bundle pin** if `okf/` or the CLI ever gets a
   structure catalog.

**Keep (OKMate is already better)**

- Type-first work-products and paired research/plan stems.
- `authority`, owners, keyed sources, `domain/` / `concern/` tags.
- One bundle per repo until a crate needs its own map.
- Path-as-id; no frontmatter `id:` override.

**Refuse**

- Five-bundle split and `@slug` Markdown targets.
- Bare tags as a replacement for prefixes.
- Learning / Finding as team types.
- Documenting `lib/` without a test that can fail.
- Restating `AGENTS.md` or the README inside architecture.

## Implications

These are research implications for **this bundle**, not an
implementation plan for the engine.

1. Write an Overview (or extend `architecture/system-overview`) so
   the root index is a map, not only a type list.
2. Clear the four isolates and the ten broken links; treat that as
   curation, not a new collection.
3. When a standing rule is load-bearing, put it in architecture
   and say what enforces it — or that nothing does.
4. Use `resource` on any future concept that *is* `okf/src/…` or
   a workflow file.
5. Do not re-nest `plans/` and `research/` by domain. The
   [nested collections](/decisions/nested-okf-collections.md)
   draft is still the right work-product rule.

okf-gem's bundles are the better **product map**. OKMate's bundle
is the better **evidence and work archive**. A productive tree
needs both layers; this repository today is almost all of the
second.[^serradura][^authoring][^nested]

[^nested]: Type-first folders; nest only plans, research, audits under okf/okmate/ops; not approved.
[^overview]: One Architecture record; engine versus application versus knowledge.
[^skill]: Collection-by-intent table; paired stems; retrieve then author; inert Markdown.
[^okmate-index]: Six type folders at the bundle root.
[^research-index]: Discriminating one-liners and paired plan links.
[^log]: Session bullets and hosted-CI caveat; Git `merge=union`.
[^serradura]: Tool theses; architecture contracts; one-bundle versus `@slug`.
[^inspect]: Catalog and graph counts from `okmate inspect --profile base` on 2026-08-29.
[^authoring]: Domain topology; `resource` omission is meaningful; durable-only log; atomic concepts.
[^index-first]: Discriminating descriptions; coverage matches claim; split on questions; measure worst case.
[^structure-bundle]: `lib/` map in the bundle; test fails on unnamed, duplicate, or gone files.
[^where]: README, AGENTS.md, and the bundle each answer one reader; one fact, not two.
[^nothing-runs]: Executed versus maintainer obligation; no third state that only sounds enforced.
[^eco-index]: Eco areas: gems, plugin, skills, resources, decisions, design, format; no gem restated.
[^eco-overview]: One kernel, three shells; five bundles; `rake okf` on the committed registry.
[^kernel-index]: Kernel areas: structure, model, capabilities, design, testing; format lives in eco.
[^capabilities]: Capabilities grouped Judge / Serve / Use; siblings named in prose.
[^cross-links]: Edges are Markdown links; `@slug` is prose; no path escape across bundles.
[^gem-okf]: Component with `resource: gems/okf`; points at `@okf` for the argument.
