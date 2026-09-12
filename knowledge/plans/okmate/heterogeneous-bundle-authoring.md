---
type: Implementation Plan
title: Support heterogeneous OKF bundle authoring
description: Separate portable reading, evidence checks, and local authoring style; make init minimal and path-correct, add optional conventions and templates, and improve discovery and documentation without migrating existing bundles by default.
tags: [domain/okmate, domain/okf, concern/authoring, concern/architecture, concern/tooling]
status: draft
generated: { by: process:cursor, at: 2026-09-12T10:20:00Z }
stale_after: 2026-12-12
authority: exploratory
owners: [human:nils]
sources:
  - id: research
    resource: ../../research/okmate/heterogeneous-bundle-authoring.md
    title: Registry inventory, public examples, and implementation findings
  - id: authoring-guide
    resource: ../../../docs/authoring.md
    title: Checked-in authoring guide
  - id: compatibility
    resource: ../../../docs/compatibility.md
    title: Current reader compatibility inventory
  - id: examples
    resource: ../../../docs/examples/README.md
    title: Contrasting worked OKF examples
  - id: init
    resource: ../../../src/init.rs
    title: Current create-only scaffold and agent integration
  - id: cli
    resource: ../../../src/cli.rs
    title: Current command contracts
  - id: init-tests
    resource: ../../../tests/init.rs
    title: Existing initialization tests
  - id: validation
    resource: ../../../okf/src/validate.rs
    title: Current base and Strict metadata policy
  - id: load
    resource: ../../../okf/src/load.rs
    title: Bundle discovery and reserved-file handling
  - id: preview
    resource: ../../../okf/src/preview.rs
    title: Explicit bundle directories and enclosing file roots
  - id: config
    resource: ../../../src/config.rs
    title: Directory and git root registry configuration
  - id: nav
    resource: ../../../src/nav.rs
    title: Current path-based merged navigation
  - id: skill
    resource: ../../../.agents/skills/manage-okmate-knowledge/SKILL.md
    title: Current local authoring and validation procedure
  - id: initial-plan
    resource: init-bundle.md
    title: Original init implementation plan
  - id: discovery-draft
    resource: ../../decisions/git-repository-bundles.md
    title: Approved authoring host and discovery contract
  - id: website
    resource: website.md
    title: Planned public documentation surface
  - id: readme
    resource: ../../../README.md
    title: Published current usage and validation commands
---

# Support heterogeneous OKF bundle authoring

## Disposition

Exploratory follow-up to the implemented `init` command. Phase 1 published
the authoring guide, contrasting examples, and a fixture-backed
compatibility inventory in this revision.[^authoring-guide][^examples][^compatibility]
Phase 2 made `init` default to a minimal scaffold, restored the previous
six collections as `--template software-project`, and writes actual
bundle paths after a staged check.[^init][^cli]
Phase 3 split portable format reading, an opt-in Evidence profile, and
application-side `okmate.toml` style findings; `check`/`view` still
default to Strict.[^validation][^load]
Phase 4 added dry-run concept creation and nearest-index updates.
The [authoring-host decision](/decisions/git-repository-bundles.md) is
approved: Git writes stay; discovery lists versioned roots without
preferring `knowledge/`.[^discovery-draft]
Phase 5 implements that discovery contract in this revision. Phase 6 has
not started.
Hosted CI is not claimed. The [research report](/research/okmate/heterogeneous-bundle-authoring.md) records
all five registered roots, public examples, exact local revisions, compatibility
findings, and the proposed developer-knowledge model. The original
[init plan](/plans/okmate/init-bundle.md) remains the history of the initial
scaffold; this plan proposes changing its universal collection assumption.[^research][^initial-plan][^init]

## Goal

An author can start a software map, engineering handbook, operations corpus,
research library, or data catalog without inheriting unrelated folders or
misleading warnings. A bundle works at an explicit `knowledge/`, `docs/`,
`.okf/`, or other path. Creation, agent routing, documentation, discovery, and
validation use that same scope. Existing bundles keep their paths, IDs, and
established policies until a maintainer chooses a migration.

Success means an author can explain the bundle's scope, choose meaningful
concepts, find a relevant record, and review proposed changes with clear
distinctions between format errors, evidence requirements, and style advice.
This addresses actual failures identified in the survey, including incorrect
agent destinations and type vocabulary warnings.[^research]

## Out of bound

- Executing this plan merely because it has been written.
- Moving or rewriting the five registered bundles in the tooling phases.
- Declaring a universal taxonomy, mandatory directory name, or new OKF standard.
- Replacing the current root registry, adding a database, vector service, MCP
  server, agent runtime, or template marketplace.
- Fetching or executing remote templates and author-supplied code.
- Automatically approving records, setting human verification, or weakening
  rendering isolation to accept active content.
- A full transactional move/split/merge authoring system; migration support
  initially produces an impact report and reviewable changes only.
- Rewriting every historical source link to the new upstream repository.

## Constraints that do not move

- The portable `okf/` layer owns parsing, identity, compatibility diagnostics,
  evidence primitives, and deterministic inspection. The application owns
  templates, CLI policy selection, configuration, and presentation. Repository
  style belongs in local guidance, not an engine-global list.[^readme][^validation]
- Keep records inert Markdown and preserve unknown metadata. Do not place
  authoring configuration in root `index.md` frontmatter; current parsing allows
  only `okf_version` there.[^load]
- Keep paths as concept IDs. Do not invent stable frontmatter IDs to avoid
  accounting for moves. No implicit cross-bundle links based on folder names.
- Preserve dry-run-first, explicit `--apply`, create-only writes, collision
  checks, and optional registration. Planning must show every destination,
  including extras outside the bundle.[^init]
- Optional local policy must never relabel style advice as a format error.
  Keep existing `--profile strict` semantics available during migration.
- Preserve authored indexes, sources, lifecycle, verification history, and
  unrelated working-tree edits. Generated recommendations remain drafts.[^skill]

## Proposed user contract

These flags and files are proposals, not commands available today.

```text
okmate init [path] --template minimal|software-project
okmate init [path] --collection <relative-directory> ...
okmate init [path] --template-file <local-template.toml>
```

Keep the path default `knowledge` for compatibility, but make `minimal` the
default content template in the release that introduces `--template`. Preserve
`--bare` as an alias for minimal; reject incompatible template combinations.
The named `software-project` template reproduces today's six collection indexes.
Custom collections and local templates prevent the template menu becoming
another closed taxonomy. Do not infer a product template solely because code
exists beside the corpus.[^init][^cli][^research]

Keep title, registry ID, path, and template independent. Optional `--register`
continues to require an unambiguous unique ID. Prefer a displayed repository or
bundle title-derived suggestion for generic basenames; an explicit `--id` wins,
and collisions remain errors. Preserve all existing registry IDs.[^config][^init]

For ongoing conventions, propose optional `<bundle>/okmate.toml`, a data-only
application sidecar distinct from `~/.okmate/config.toml`. Initial fields should
cover a version, a preferred type list, preferred tag vocabulary, type-to-path
suggestions, and a link to a human-readable authoring guide. Absence means no
local style checks. Templates seed content; conventions guide later authoring;
neither makes a type illegal to read. Final field names should follow Phase 3's
small fixture-backed design, not be standardized by this draft.

## Phases

### Phase 1 — Document the authoring choices and compatibility baseline

**Bound:** Publish a concise authoring guide and contrasting worked examples:
minimal standalone corpus, type-first software archive, topic-oriented engineering
handbook, operations/runbooks, and data catalog. Start with reader questions,
scope boundaries, a few useful types, evidence, and maintenance expectations.
Explain collection versus type versus authority; research versus reusable
guidance; resource versus source; and generated projection versus canonical
authored knowledge. Include a decision table for directory names and multi-bundle
boundaries. Use developer-knowledge's proposed navigation without moving it.

Update README pointers and current init guidance. Coordinate publication with
the [website plan](/plans/okmate/website.md), but keep a checked-in guide usable
without the website. Store durable rationale here; put published tutorials and
reference examples on the documentation surface, with one canonical home per
fact. Refresh active upstream references to the new canonical repository and
label historical frozen references.[^research][^website][^readme]

Inventory current parser/policy differences as small synthetic fixtures. Split
genuine malformed YAML, citation defects, peer extensions, and Okmate-specific
restrictions; do not treat whole third-party repositories as golden-valid test
fixtures. Document the current base reader's limits honestly.[^compatibility]

**Owner:** Documentation and `knowledge/`; fixtures under `okf/tests/` where
they establish intended format behavior.

**Exit:** Each example has a stated purpose, navigation path, and expected
validation profile; example checks give their documented results.[^examples]
A reader can choose a type and location without reading Okmate's
repository-specific skill.[^authoring-guide] The compatibility inventory
names rule ownership and expected diagnostic level.[^compatibility]

### Phase 2 — Fix init placement and make the scaffold selectable

**Bound:** Refactor fixed `COLLECTIONS` into declarative built-in template data;
add minimal/default and software-project behavior, custom collection paths, and
bounded local template input. Render actual repo-relative bundle paths in all
agent files, examples, validation commands, and `.gitattributes` hints. Quote
paths with spaces correctly. Do not emit the Git union-driver claim unless the
app established or observed the matching configuration.[^init][^init-tests]

Preflight the entire resulting corpus before writes, including existing ordinary
Markdown. For a repository-root bundle, reject `--agents` before mutation when
it would insert untyped `AGENTS.md` into the corpus. Initially print the explicit
alternative of a child bundle or external instructions; do not silently turn
AGENTS into a knowledge concept or hide arbitrary files from validation. Honor
valid root-only knowledge bundles that do not request incompatible extras.

Plan output must show resolved bundle root and each extra file's actual target;
JSON remains deterministic and backward-compatible where possible. Validate
the staged bundle before publishing it and before registration. Preflight all
collisions; report partial I/O failures accurately. Do not claim arbitrary
filesystem/config updates are atomic; if new files survive an unexpected write
failure, identify them and leave registration unchanged.

**Tests:** Minimal/default and legacy template contents; `--bare` compatibility;
arbitrary custom collections; reserved/escaping paths; template collisions;
`docs`, `.okf`, `docs/kb`, and space-containing roots; explicit and colliding
registry IDs; existing extras preserved; no-Git behavior; root `--agents`
rejection before any writes; validation failure does not change registry.
Assert generated text points at the actual path, not only that files exist.

**Owner:** `src/init.rs`, `src/cli.rs`, `tests/init.rs`; config helpers only as
needed. Templates stay outside the portable engine.

**Exit:** All placement cases produce the intended checkable content or fail
before mutation. README release notes explicitly describe the default scaffold
change and `--template software-project` compatibility command.

### Phase 3 — Separate format, evidence, and local style

**Bound:** Use the Phase 1 fixtures to correct portable reading behavior, then
add an opt-in domain-neutral evidence policy and application-side convention
checks. Keep legacy Strict selectable; do not silently remove its requirements
for existing users. Evidence requirements may retain title, description,
generation, owners, and authority, but should not force product type names or
`domain/` tag spelling. Select policy explicitly in newly generated instructions.
Changing existing check/view defaults is a separately documented rollout choice.

Prioritize tolerant version/optional-metadata reading, `verified` mapping
normalization, freshness timestamps, and raw-comment handling. Preserve unsafe
HTML exclusion from rendered output while deciding separately whether a comment
is an authoring error. Peer root-index extensions need an explicit supported
extension policy or a compatibility warning, not unexamined blanket acceptance.
Keep malformed YAML as a real parsing failure.[^research][^validation][^load]

Introduce the smallest useful `okmate.toml` convention model. Unknown types
remain readable; undeclared or inconsistent types produce optional style
findings naming the local rule. No conventions file means no vocabulary noise.
Do not replace ten global preferred types with fifty. Governance, style, and
format reports must be distinguishable in terminal/JSON and review UI; style
findings do not enter the format error count. Existing diagnostic consumers get
a documented compatibility path.

**Tests:** A type-only custom concept; properly evidenced Workflow/Runbook and
domain records; known and unknown extensions; declared versus missing style;
legacy Strict compatibility; arbitrary tags under the new evidence policy;
safe rendering; unknown metadata round-trip; v0.1/v0.2 reader fixtures; advisory
root extension handling without hiding malformed reserved files.

**Owner:** Portable compatibility and reusable evidence rules in `okf/`;
convention loading and policy selection in `src/`; reporting in views/templates.

**Exit:** Identical valid content can be read without installing a style; an
author can opt into evidence checks without product vocabulary warnings.
Diagnostics name their source contract. The documented examples pass the
appropriate profiles without weakening historical verification semantics.

### Phase 4 — Support ongoing authoring and navigation

**Bound:** Add explicit concept creation from small type/body templates, using
the bundle's existing vocabulary and optional conventions. A minimal record
requires no invented actor, owner, citations, or human approval. When an evidence
profile requires missing fields, request explicit values or fail with named
missing inputs. Keep concept content as a proposal until `--apply`.

Add index maintenance as a reviewable diff: suggest links/descriptions and
identify unlisted or unresolved records while preserving authored grouping,
ordering, prose, and intentional links. Never replace a question-oriented index
with an alphabetic dump automatically. Add optional style diagnostics for empty
scaffold directories, vocabulary inconsistency, duplicated overview roles, and
unsupported local conventions; distinguish measured facts from heuristic advice.

Provide reusable templates for Explanation, How-to Guide, Reference, Research
Report, Audit, and optional operations/data examples. Types and folders remain
independent. Link to retained evidence instead of converting every research
report into timeless guidance.[^research][^skill]

**Owner:** CLI authoring module, application template/convention layer, and
public authoring guidance. Reuse the engine's parsed catalog and link data.

**Exit:** A handbook record can be created, indexed, checked, and found without
editing generic product rules. Authored index structure and citations survive
an index-update preview. Focused tests exercise user changes, collisions,
missing evidence values, and idempotent proposals rather than template internals.

### Phase 5 — Make root discovery explicit and preserve heterogeneous views

**Bound:** Add read-only root discovery for a selected repository/container.
Explicit bundle paths and registry entries retain priority. Discover versioned
root indexes independently of basename, including hidden `.okf/` and nested
component locations, with bounded depth, ignore rules, and reported scan limits.
Never flatten a container of bundles into one root. Markerless trees may be
reported as candidates with their evidence; do not automatically claim a
collection index proves a separate bundle.[^preview][^load][^config]

When multiple candidates remain, list them and require explicit selection;
do not privilege `knowledge/`. Follow the approved
[authoring-host decision](/decisions/git-repository-bundles.md): keep the
Git write boundary; do not revive one-level `knowledge/` inference.[^discovery-draft]

In mixed-scope workspaces, preserve root labels and offer separated navigation
as the predictable view. Merged paths are a display choice, not semantic
equivalence. Do not equate same-named types, collections, or concepts across
bundles without an explicit convention.[^nav]

**Tests:** Direct hidden root; nested component roots; zero/one/multiple candidates;
container versus collection indexes; scan limits; no unintended hidden dependency
walk; explicit overrides; and two bundles with conflicting `reference/` meanings.

**Owner:** Application discovery/CLI/settings and workspace navigation; reuse
portable root-marker parsing without making the engine own registry policy.

**Exit:** Studio-like `docs/` and gem-like component `.okf/` layouts are discoverable
and selectable without renaming; ambiguity is visible and no corpus is silently
merged or registered.

### Phase 6 — Pilot curation and measure before migrating

**Bound:** When separately requested, pilot the research recommendation in
developer-knowledge: preserve IDs, improve its root/question navigation, review
the source register's type and tooling tags, and distill one or two reusable
guides from existing evidence. Keep repository tool contracts and private host
state in their proper scopes. Do not copy product knowledge into the handbook.

Assess each other bundle individually: retain Okmate/Rocci type-first archives;
add current-state maps where useful; add H35 runbook navigation as content
appears; retain rocci-spotify's two-record layout. These are curation proposals,
not migrations authorized by this plan.[^research]

For any later move, produce an old/new path map and identify inbound Markdown
links, `sources[].resource`, indexes, benchmarks, agent routing, registry and
Git subpaths, published links, and preview state. A whole-root rename and a
concept move have different identity effects. Preserve history and verification;
do not create redirect stub concepts merely to suppress broken-link findings.

**Evaluation:** Establish a small frozen set of real reader questions before
curation. Include steering a task, finding its evidence, restoring a development
environment, distinguishing audit observation from advice, and finding an owning
product contract. Record intended concept/heading targets, lexical retrieval
results using the existing benchmark support where applicable, manual navigation
steps, and wrong-scope results. Navigation-only changes need a navigation check;
search metrics alone cannot measure the improvement.

**Owner:** Each bundle's maintainer, with tooling support from Okmate. The first
prototype can use temporary fixtures before any sibling edits are requested.

**Exit:** All baseline links and expected records remain reachable, retrieval
does not regress, and readers reach the intended topic with fewer irrelevant
branches. No broad move is justified solely by aesthetics. Record actual results
and remaining uncertainty before recommending physical restructuring.

## Delivery order and validation

Phases 1–2 provide immediate documentation and init improvements. Phase 3 is
required before advertising warning-free heterogeneous governance. Phase 4
builds on that policy model. Phase 5 can be delivered independently after its
discovery contract is settled. Phase 6 requires a separate curation request;
the present task only writes research and this plan.

For code changes run the repository gates: formatting, workspace Clippy with no
default features, `cargo test -p okf`, `cargo test -p okmate --no-default-features`,
and `uv run okmate-ops ci` when the maintainer tool is present. Run strict
knowledge validation after documentation-record changes. Test changed public
examples against their declared policy. Keep conformance errors, advisory
findings, and pre-existing provenance warnings separate.[^readme][^skill]

Do not mark a phase complete without its exit evidence and required hosted CI
on the relevant revision. Record behavior changes and results in the knowledge
log, preserving previous sessions' entries.[^skill]

[^research]: Full inventory, source review, public comparisons, and reproducible behavior observations.
[^init]: Existing scaffold is implemented; defaults and extras need follow-up changes.
[^cli]: Preserve explicit command compatibility while introducing named templates.
[^init-tests]: Extend path and content coverage rather than only file-existence assertions.
[^validation]: Separate preferred vocabulary and tag conventions from metadata evidence rules.
[^load]: Discovery and current root-index parser restrictions.
[^preview]: Current directory input is explicit bundle input, not repo discovery.
[^config]: Existing root IDs and local/git paths remain authoritative.
[^nav]: Equal collection paths currently merge in the merged display mode.
[^skill]: Canonical authoring, provenance, validation, and completion-log rules.
[^initial-plan]: Initial implementation design, not current evidence that init is absent.
[^discovery-draft]: Approved: Git writes; bounded marker discovery; no `knowledge/` privilege.
[^website]: Public documentation can publish the guide without becoming a dependency for local usage.
[^readme]: Ownership and current repository validation commands.
[^authoring-guide]: Checked-in guide for scope, types, paths, and current init.
[^examples]: Five contrasting bundles with documented check profiles.
[^compatibility]: Current diagnostic ownership and severity, backed by synthetic fixtures.
