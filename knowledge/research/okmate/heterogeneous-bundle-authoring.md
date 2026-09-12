---
type: Research Report
title: Structuring heterogeneous OKF bundles
description: A survey of all five registered roots and public OKF examples supports purpose-specific navigation, open concept vocabularies, arbitrary bundle paths, and a minimal init scaffold with optional authoring templates.
tags: [domain/okmate, domain/okf, concern/authoring, concern/architecture, concern/tooling]
status: draft
generated: { by: process:cursor, at: 2026-09-12T09:00:44Z }
stale_after: 2026-12-12
authority: exploratory
owners: [human:nils]
sources:
  - id: survey
    resource: Local read-only registry, catalog, check, search, and temporary-fixture survey on 2026-09-12 using okmate 0.3.6; method and results recorded in this report
    title: Registered bundle inventory and behavior probes
    author: process:cursor
  - id: developer
    resource: ../../../../developer-knowledge/knowledge/index.md
    title: Developer knowledge scope and collection indexes
  - id: developer-readme
    resource: ../../../../developer-knowledge/README.md
    title: Personal knowledge, workflows, and small tools ownership
  - id: developer-tools
    resource: ../../../../developer-knowledge/knowledge/research/tools/index.md
    title: Developer tooling research
  - id: developer-agentic
    resource: ../../../../developer-knowledge/knowledge/research/agentic/index.md
    title: Agentic development research and source register
  - id: h35
    resource: ../../../../h35-internal/knowledge/index.md
    title: Private operations bundle scope
  - id: rocci
    resource: ../../../../rocci/knowledge/index.md
    title: Rocci collections and migration history
  - id: spotify
    resource: ../../../../rocci-spotify/knowledge/index.md
    title: App-local research and plan bundle
  - id: overview
    resource: ../../architecture/system-overview.md
    title: OKMate ownership boundaries
  - id: nested
    resource: ../../decisions/nested-okf-collections.md
    title: Exploratory Okmate-specific collection convention
  - id: modelling
    resource: bundle-modelling.md
    title: Earlier product-map versus work-archive research
  - id: census
    resource: ../okf/knowledge-systems-built-on-okf.md
    title: Earlier ecosystem census, used for discovery only
  - id: references
    resource: ../okf/reference-authoring-style.md
    title: Document-local claim citations and selective reference concepts
  - id: init
    resource: ../../../src/init.rs
    title: Init collections, agent text, registration, and apply ordering
  - id: cli
    resource: ../../../src/cli.rs
    title: Current CLI defaults and options
  - id: init-tests
    resource: ../../../tests/init.rs
    title: Init integration coverage
  - id: validation
    resource: ../../../okf/src/validate.rs
    title: Strict vocabulary, tag prefixes, and metadata validation
  - id: load
    resource: ../../../okf/src/load.rs
    title: Recursive discovery and root-index validation
  - id: markdown
    resource: ../../../okf/src/markdown.rs
    title: Raw HTML and comment diagnostics
  - id: preview
    resource: ../../../okf/src/preview.rs
    title: Explicit root handling and enclosing-root discovery
  - id: roots
    resource: ../../../src/roots.rs
    title: Registry resolution and knowledge-directory fallback
  - id: nav
    resource: ../../../src/nav.rs
    title: Collection navigation and merged path grouping
  - id: colors
    resource: ../../../src/views/governance.rs
    title: Arbitrary type color hashing
  - id: old-init
    resource: ../../plans/okmate/init-bundle.md
    title: Initial scaffold design
  - id: discovery-draft
    resource: ../../decisions/git-repository-bundles.md
    title: Unapproved repo discovery and authoring-host proposal
  - id: studio-index
    resource: ../../../../okf-studio/docs/index.md
    title: Studio product knowledge in docs
  - id: studio-create
    resource: ../../../../okf-studio/docs/features/create-bundle.md
    title: Studio's documented minimal creation workflow
  - id: studio-profile
    resource: ../../../../okf-studio/docs/features/profile-aware-authoring.md
    title: Studio's documented separation of conformance and advisory profiles
  - id: gem-index
    resource: ../../../../okf-gem/.okf/index.md
    title: okf-gem ecosystem and component bundle boundaries
  - id: gem-style
    resource: ../../../../okf-gem/gems/okf/lib/okf/skill/reference/authoring.md
    title: okf-gem authoring guidance, inspected as evidence
  - id: gem-personal
    resource: ../../../../okf-gem/gems/okf-pro/lib/okf/pro/template/seed/.okf/index.md
    title: okf-pro personal knowledge seed
  - id: upstream-move
    resource: https://github.com/GoogleCloudPlatform/knowledge-catalog/tree/main/okf
    title: Frozen reference directory points to the new canonical repository
    author: organization:google-cloud
  - id: spec
    resource: https://github.com/GoogleCloudPlatform/open-knowledge-format/blob/main/SPEC.md
    title: Current canonical OKF v0.2 specification, consulted 2026-09-12
    author: organization:google-cloud
  - id: upstream
    resource: https://github.com/GoogleCloudPlatform/open-knowledge-format/blob/main/README.md
    title: Official production and consumption setups
    author: organization:google-cloud
  - id: ga4
    resource: https://github.com/GoogleCloudPlatform/open-knowledge-format/tree/main/bundles/ga4
    title: Data catalog sample collections
  - id: retail
    resource: https://github.com/GoogleCloudPlatform/open-knowledge-format/tree/main/bundles/acme_retail
    title: Retail metrics, policies, tables, and computation sample
  - id: crm
    resource: https://github.com/jacquescorbytuech/crm-open-knowledge-wiki/blob/main/index.md
    title: Practitioner CRM knowledge organized around its subject
  - id: threats
    resource: https://github.com/Barnes70/TLCTC
    title: Cyber threat framework with a generated OKF projection
  - id: workbench
    resource: https://github.com/koizumikento/okf-workbench
    title: Minimal and domain templates with separate curation diagnostics
  - id: agent-knowledge
    resource: https://github.com/stjbrown/agent-knowledge/blob/main/skills/kb-init/SKILL.md
    title: Domain-sensitive initialization and explicit local conventions
  - id: diataxis
    resource: https://diataxis.fr/start-here/
    title: Reader needs and four documentation forms
    author: human:daniele-procida
  - id: para
    resource: https://fortelabs.com/blog/para/
    title: Actionability-based personal organization
    author: organization:forte-labs
  - id: plan
    resource: ../../plans/okmate/heterogeneous-bundle-authoring.md
    title: Documentation and tooling improvement plan
---

# Structuring heterogeneous OKF bundles

## Findings and recommendation

Keep bundle structure local to its purpose. Okmate and Rocci have useful
type-first work archives; developer-knowledge needs a software-engineering
handbook and research library; H35 needs an operations map; rocci-spotify is
small enough for its existing research/plan pair. Uniform folders would make
these different jobs harder to distinguish. The recommendations below are
exploratory, based on the inventory and source inspection, and do not authorize
migrations or implement the [paired plan](/plans/okmate/heterogeneous-bundle-authoring.md).[^survey][^plan]

Recommend a minimal default for `init`, an explicit software-project template
that preserves today's scaffold, and optional local conventions. Treat bundle
path, scope, navigation, concept type, authority, and freshness as independent
choices. Fix generated paths and validation assumptions before encouraging
authors to adopt new vocabulary. These are follow-up changes to an existing
command, not the initial implementation of `init`.[^init][^old-init]

## Scope, method, and limits

Survey date: 2026-09-12. The installed `/opt/homebrew/bin/okmate` reports 0.3.6,
matching the checkout's package version; binary/source identity was not otherwise
attested. Enumerated every entry returned by `okmate roots --format json
--no-sync`; all five were enabled local directory roots with no resolution
errors. Counts are the baseline before adding this report and its plan. For
each, inspected the strict catalog, ran strict check and an
`overview` search, read indexes, and reviewed representative records. No registry
sync or remote update was requested. Counts exclude reserved indexes and logs.
Temporary fixtures tested unknown types, registration IDs, and init placement.[^survey]

Also read the requested sibling checkouts and checked their authored bundles
with the base profile. Online discovery used broad OKF/bundle/knowledge queries,
domain examples, official documentation, and repositories named in the earlier
[ecosystem census](/research/okf/knowledge-systems-built-on-okf.md). Peer claims
below describe inspected documentation or files, not independently tested product
features. This is a purposive survey of contrasting setups, not a popularity
census; it cannot establish a dominant industry layout.[^census][^survey]

### Complete registered inventory

Paths below are relative to the common local `Projects/` directory. Revisions
identify repository HEAD, not all working-tree content. Okmate was clean before
this task; Rocci had seven changed/untracked status entries and rocci-spotify
one. Those changes were preserved.[^survey]

| Registry ID / path | Concepts / indexes | Observed composition | Fit and recommended direction |
| --- | --- | --- | --- |
| `developer-knowledge` / `developer-knowledge/knowledge` | 11 / 10 | 9 Research Report, 1 Audit, 1 Architecture; decisions, plans, status empty | Good evidence archive, weak handbook front door. Add topic/question navigation and selectively distill reusable practice. |
| `h35-internal` / `h35-internal/knowledge` | 6 / 9 | 3 Research Report, 2 Implementation Plan, 1 Architecture | Sensible private ownership boundary. Add service/host orientation and runbooks as operations mature; no need for empty product collections. |
| `okmate` / `okmate/knowledge` | 44 / 14 | 22 Implementation Plan, 16 Research Report, 2 Audit, 2 Decision, 1 Architecture, 1 Status | Type-first work archive makes sense; add a short current capability map and fix stale status prose before moving records. |
| `rocci` / `rocci/knowledge` | 190 / 27 | 84 Implementation Plan, 67 Research Report, 14 Audit, 10 Decision, 5 Architecture, 5 Status, 2 Reference, 2 Design Standard, 1 Case Study | Product-area nesting is justified at this scale. Improve current-state entry points and historical filtering; avoid broad ID churn. |
| `rocci-spotify` / `rocci-spotify/knowledge` | 2 / 3 | 1 Research Report and 1 Implementation Plan | Appropriate minimal app-local scope. Add collections only when real records require them. |

Scope evidence comes from the roots' own indexes and ownership descriptions;
the fit judgments are this report's interpretation.[^developer][^developer-readme][^h35][^overview][^rocci][^spotify]

| Repository | Inspected HEAD |
| --- | --- |
| developer-knowledge | `d327fcaba8a4f8859dc6059cddb489e73e65c031` |
| h35-internal | `213866587bc2b66c52c1da899de19cfc2deb60cb` |
| okmate | `05fdb637a2536893e1b364a2574efb13978cb3a7` |
| rocci | `39d490a5713ab8ec16727597c6d41bd775c15643` |
| rocci-spotify | `5b049a65951838b0321788ecc5127d14a503f0b7` |
| okf-studio | `47deb810baf0f0b456e9b324f44dfb99a795e8b0` |
| okf-gem | `7b23571f0a3d5f5021d2e981bb4699f59b8de008` |

Studio was clean; okf-gem had a modified `.okf.json`, so its registry was not
treated as a pristine published snapshot. These two checkouts are comparisons,
not additional Okmate-registered roots.[^survey]

### Validation is not a structure score

All five registered roots passed strict check with zero errors. Four had zero
warnings. Rocci had 101 warnings: 10 `OKF4005` (verification predates generation),
54 `OKF4006` (source changes after verification), 34 `OKF4002` (unused source
entries), and 3 `OKF3002` (unresolved links). These are baseline lifecycle,
provenance, citation, and link findings, not evidence that its folders are wrong.
The working tree was not normalized to make the survey cleaner.[^survey][^validation][^load]

## What is format, and what is local style?

The current canonical specification is still v0.2. It explicitly leaves taxonomy
and directory organization to producers. Concept identity is its relative path
without `.md`; `type` is an open vocabulary. Indexes and logs are optional, and
Git is a recommended distribution choice rather than a runtime requirement.
Optional metadata and unknown types do not justify rejecting a bundle. Root
index frontmatter has the special `okf_version` allowance; this is not a general
bundle-configuration channel.[^spec]

Google's old `knowledge-catalog/okf` directory now says that it is frozen and
points to `GoogleCloudPlatform/open-knowledge-format`. Refresh active guidance
and source links deliberately; preserve old citations when they identify a
historical implementation or snapshot.[^upstream-move]

Use the following authoring model as local guidance:

| Axis | Question it answers | Example for developer-knowledge |
| --- | --- | --- |
| Bundle boundary | Which knowledge has one scope, audience, and maintenance responsibility? | General engineering practice, separate from a product's contracts and private host operations |
| Collection / navigation | Where does a reader start? | Agentic development; developer environments; testing and evaluation |
| Concept type | What kind of reusable thing is this? | Explanation, How-to Guide, Reference, Research Report, Audit |
| Tags | Which other subjects or concerns does it intersect? | `domain/developer-environments`, `concern/reproducibility` |
| Authority | Is this advice, observed behavior, a binding rule, or history? | An exploratory recommendation is not an adopted team policy |
| Lifecycle / evidence | Is it current, and what supports it? | A dated audit remains an observation of that machine at that time |

The existing [nested collections](/decisions/nested-okf-collections.md) record
is an unapproved local convention used in Okmate. Its `okf/okmate/ops` vocabulary
does not belong in every bundle. Earlier [bundle modelling research](/research/okmate/bundle-modelling.md)
rejected some personal-knowledge types for this product's map; that judgment
should not become a validator-wide restriction.[^nested][^modelling]

## A suitable model for developer-knowledge

Treat this as a growing engineering knowledge library. The repository also owns
small `koliyo-ops` tools, but that does not make its entire corpus a product
architecture archive. Keep tool contracts subordinate to that broader purpose.
Product implementation facts remain in their owning repositories; private
deployment details remain in H35.[^developer-readme][^developer][^h35]

### Useful concepts

Start with a small vocabulary, adding types when the distinction changes how a
reader uses a record. The following are proposed choices, not newly required OKF
types or instructions to retype all existing documents.

| Type | Reader job and suitable body | Concrete candidate |
| --- | --- | --- |
| Explanation | Understand a mechanism; model, tradeoffs, limits, worked example | How autonomous task steering works |
| How-to Guide | Complete a repeatable goal; prerequisites, steps, success check, recovery | Restore a Homebrew environment from its saved manifest |
| Reference | Look up a stable definition or interface; scope, fields/options, examples | Source register or a bounded tool capability reference |
| Research Report | Evaluate evidence; question, method, findings, uncertainty, implications | Existing model/harness and practitioner-evidence reports |
| Audit | Understand an observed state; environment, method, findings, date | Existing Time Machine snapshot audit |
| Decision | Recover an actual adopted choice and its alternatives | A future personal backup policy, only after it is adopted |

`Pattern`, `Workflow`, `Runbook`, `Experiment`, and `Glossary Term` are reasonable
later additions if repeatedly useful. A `Workflow` coordinates recurring roles
and stages; a `Runbook` responds to an operational trigger. Do not distinguish
these from a guide just to add labels. An abstract concept need not denote a
software component. A source register can remain one reference record with
anchors; not every URL needs a separate concept.[^references]

### Navigation before migration

First revise the root index into a short scope statement and question-oriented
entry points linking existing records. Preserve all 11 concept IDs. For example:

- How should I delegate and steer development work? → agentic synthesis and steering.
- How should I evaluate agent results? → evaluation and practitioner evidence.
- How do I maintain and restore my development environment? → tooling research and audit.
- Where did these conclusions come from? → the source register and local keyed citations.

Those topics reflect the present content, rather than an imagined complete
software-engineering encyclopedia. The source register is currently typed
Research Report, and the tools group uses `domain/agentic` even for backup
material; both deserve semantic review, not automatic bulk replacement.[^developer-agentic][^developer-tools][^survey]

Then distill useful current guidance from the research without discarding the
evidence record. A dated model comparison remains dated research; a maintained
guide cites it and states its applicability. A machine audit is evidence for a
general backup guide, not universal backup policy. Do not declare guidance
verified merely because it was reorganized.

If navigation remains poor after this, a possible future physical layout is:

```text
knowledge/
  index.md                  # Scope and question routes
  log.md
  about.md                  # Scope, ownership, and local conventions
  agentic-development/      # Explanations, workflows, evaluations together
  developer-environments/   # Setup, reproducibility, backup, troubleshooting
  research/                 # Dated investigations worth preserving separately
  audits/                   # Environment-specific observations
```

This is an illustrative destination, not a requested migration or a starter
that creates empty folders. Keep the existing type-first research archive while
adding topic indexes if it works well enough. At eleven records, a rewritten
front door may be sufficient. Grow topics such as testing or software design
only when there is content and recurring retrieval demand.

## Practices visible in the public ecosystem

The examples establish variety, not a universal best practice. The strongest
recurring pattern is a scoped corpus, short navigation, domain-appropriate
concepts, ordinary links, and a maintenance loop. The software-project archive
is one useful instance.

| Primary evidence | Observed setup | Implication for Okmate |
| --- | --- | --- |
| Google reference bundles | GA4 has datasets, tables, references; Acme Retail adds metrics, policies, computations and supporting resources. | Templates should reflect knowledge entities and reader tasks. Do not infer a project's document ontology from its implementation language.[^ga4][^retail] |
| CRM practitioner wiki | Repository-root knowledge organized into principles, foundations, channels, measurement, references. | A professional practice can be the bundle's subject without an accompanying application. Its inspected index is legacy-shaped; copy its information architecture, not its metadata verbatim.[^crm] |
| TLCTC threat framework | Generated `okf/` projection from canonical material; collections include clusters, rules, glossary, controls, and incident paths. | Distinguish authored sources from generated distribution bundles; editing a projection may be lost on rebuild.[^threats] |
| OKF Workbench | Documents Minimal, Software Project, Data & Analytics seeds; arbitrary nonempty types; conformance separate from curation; managed index regions. | Named starting points and optional advice are established implementation patterns.[^workbench] |
| Agent Knowledge `kb-init` | Inspects domain and recurring entities before adapting a seed; keeps local vocabulary and conventions in `spec/` records; allows alternate paths. | Reusable scaffolding can coexist with domain judgment. A schema need not be a hardcoded global enum.[^agent-knowledge] |
| okf-gem | An ecosystem `.okf/` and four component `.okf/` bundles; local vocabulary includes Component, Capability, Constraint, Overview. | Multi-bundle boundaries can match independently maintained components; `@slug` routing is tool-specific.[^gem-index][^gem-style] |
| okf-pro seed | Personal knowledge zones for reference/learnings/glossary, projects/areas, and board/journal/roadmap. | Even one tool family supports both software maps and personal knowledge. Personal types should not leak into every starter either.[^gem-personal] |
| OKF Studio | Product knowledge in `docs/`, with product, features, UX, architecture, reference, proposals; documents minimal creation and advisory profile context. | Folder names and a familiar documentation architecture can serve human readers without losing typed concepts.[^studio-index][^studio-create][^studio-profile] |

The official README describes manual, agent, and catalog-export production;
files can then be consumed by editors, static sites, agents, search indexes, or
graph viewers. These are alternative operating setups over portable content,
not obligations to install a database or agent runtime.[^upstream]

Two adjacent methods help choose a style. Diátaxis separates learning,
goal completion, lookup, and understanding through tutorials, how-to guides,
reference, and explanation. PARA organizes by projects, ongoing responsibilities,
resources, and archives. They answer different questions; neither is an OKF
specification. For developer-knowledge, use Diátaxis to shape reusable guidance
and a light topic map for navigation. Borrow PARA's distinction between ongoing
practice and finite work without moving stable concept paths whenever work
becomes inactive.[^diataxis][^para]

## Directory names and bundle boundaries

| Placement | When it fits | Costs to account for |
| --- | --- | --- |
| `knowledge/` | Curated knowledge beside source, tools, or other documentation | Current ergonomic default, but not the bundle's identity |
| `docs/` | The documentation tree itself is the typed knowledge corpus, as in Studio | Ordinary untyped Markdown mixed into the same tree becomes a validation concern |
| `.okf/` | Explicit machine-discoverable knowledge beside code, as in okf-gem | Hidden in ordinary browsing; scanners must discover the root explicitly |
| Repository root | A dedicated knowledge-only repository | README, CONTRIBUTING, and AGENTS Markdown are not automatically outside the corpus |
| `bundles/<name>/` or component-local roots | Distinct scopes with separate audiences, release cadence, or ownership | Requires explicit resolution, namespaced identity, and cross-bundle navigation |

Keep developer-knowledge's existing `knowledge/`: it separates the corpus from
tools and machine manifests. Keep existing paths elsewhere unless a concrete
benefit pays for migration. Directory name alone does not determine authority,
visibility, or product scope. A whole-root rename preserves relative concept
IDs but changes registry paths and possibly external references. Moving records
inside a root changes their IDs and requires updating links and consumers.[^developer-readme][^load][^roots]

In current Okmate, an explicitly supplied `.okf/` is readable: hidden child
entries are skipped during discovery, not the supplied root itself. Conversely,
loading a repository directory does not discover its child bundles: directory
input is treated as the bundle. A container can therefore be misread as one
large corpus; nested versioned indexes are rejected as non-root frontmatter.
The proposed one-level repo discovery rule, including preferring `knowledge`,
is still an unapproved design, not the behavior of `resolve_preview_path`.[^load][^preview][^discovery-draft]

Recommend explicit root/registry identity first; when offering discovery, list
candidates and report ambiguity. Do not select `knowledge/` merely because its
name wins over an equally valid `docs/` or `.okf/` root.

## Concrete tooling findings

### Initialization

| Finding verified in source or a temporary fixture | Improvement |
| --- | --- |
| Default creates root index/log plus six fixed collection indexes; `--bare` creates only index/log. | Make the minimal form the default and preserve the old form as `software-project`.[^init][^cli] |
| `init docs --bare --agents --apply` succeeds but all generated authoring instructions route to `knowledge/`. | Render actual root, selected style, and validation command throughout generated instructions.[^survey][^init] |
| `init . --bare --agents --apply` in an empty temporary Git repo writes files, then fails strict check because the generated `AGENTS.md` has no concept frontmatter. | Preflight corpus boundaries and incompatible extras before writing; root-only bundles need a distinct instructions placement strategy.[^survey][^load] |
| Registry IDs derive from the target basename: `docs`, `okf`, or `knowledge`. | Propose a scope-aware ID and retain explicit `--id`; never silently suffix collisions.[^survey][^init] |
| Generated log claims a Git union driver exists even when `--agents` did not create attributes; registration precedes strict post-check. | Make log wording conditional and validate before committing registration.[^init] |
| Current agent integration test targets `knowledge/` and checks file existence and attributes. | Test contents and use nonstandard paths, root placement, spaces, and multiple bundles.[^init-tests] |

### Vocabulary, consumption, and interoperability

Strict has ten hardcoded preferred types. A valid temporary `Workflow` record
passed base without diagnostics but strict emitted `OKF2002`; strict also requires
`domain/` tags and rejects prefixes outside four built-in families. Arbitrary
types already receive colors in the viewer. The pressure toward product
documents therefore comes from authoring/check policy, not a renderer inability
to display other concepts.[^survey][^validation][^colors]

Base check on the sibling trees exposed separate interoperability problems:

- Studio `docs/`: 38 errors and 20 warnings, including extra index keys,
  YAML parse failures, escaping links, and source/footnote problems.
- Studio `design-system/`: 2 errors and 44 warnings; it declares an additional
  design-system version and is a distinct corpus from product documentation.
- okf-gem's five authored bundles: respectively 3, 14, 1, 35, and 39 errors
  for ecosystem, kernel, MCP, TUI, and pro. Findings include missing footnote
  definitions, raw HTML comments, and date-only generation timestamps.[^survey]

Do not label every rejection a broken producer or every extension a new
standard. YAML parse failures and missing cited definitions require source
inspection; Studio's profile keys are peer extensions; Okmate's blanket raw
HTML rejection also rejects inert comments.[^markdown] Some compatibility behavior needs
work: the implementation accepts only v0.2 declarations, requires a list for
`verified`, and requires date-only `stale_after`. Thus `base` is not yet a
complete tolerant interchange reader. Audit each rule against its owning
contract, keeping safe rendering separate from authoring policy.[^validation][^load][^studio-profile]

Merged navigation also groups collections by path, so two `reference/`
directories can appear together despite different local meanings. Preserve
bundle identity and offer separate navigation as the predictable cross-domain
view. Do not infer shared semantics from equal path strings.[^nav]

## Improvement priorities

1. Document the independent authoring choices and publish contrasting examples.
2. Repair path-specific init output and preflight failures; expose minimal and
   explicit product seeds with customizable collections.
3. Separate interchange compatibility, evidence requirements, and local style
   diagnostics; allow vocabulary growth without warning fatigue.
4. Add reviewed concept/index authoring and optional bundle-local conventions.
5. Support explicit discovery and migration reports without mandatory renames.
6. Pilot the handbook navigation in developer-knowledge and assess retrieval
   before reorganizing physical paths.

The [implementation plan](/plans/okmate/heterogeneous-bundle-authoring.md) defines
owners, compatibility rules, tests, and bounded exits. No implementation phase,
bundle migration, registration, or human approval is claimed by this report.[^plan]

[^survey]: Read-only baseline and isolated fixture results at the stated date; exact root revisions and diagnostic totals are preserved above.
[^developer]: Root navigation includes six type collections, three currently empty.
[^developer-readme]: Repository purpose and small tooling scope; product facts remain elsewhere.
[^developer-tools]: Existing environment, backup, and cache research.
[^developer-agentic]: Existing research roles and the 43-source register.
[^h35]: Private operations scope rather than general engineering knowledge.
[^rocci]: Product knowledge collections and migration references.
[^spotify]: Two app-local collections without the six-folder scaffold.
[^overview]: Engine, application, and canonical knowledge ownership.
[^nested]: This bundle's applied but unapproved collection convention.
[^modelling]: Earlier findings concern this product's map and archive.
[^census]: Discovery input; August counts were not reused as current adoption evidence.
[^references]: Keep citations keyed and local; create source concepts selectively.
[^init]: `COLLECTIONS`, `agent_files`, `register_id`, `run`, and `log_markdown`.
[^cli]: Current `init` and single-root command defaults.
[^init-tests]: Existing agent extras coverage exercises a knowledge child.
[^validation]: `PROFILE_TYPES`, Strict requirements, metadata families, and provenance diagnostic meanings.
[^load]: Discovery, root index restrictions, and unused-source warnings.
[^markdown]: Both block and inline HTML produce OKF2009 independently of the selected profile.
[^preview]: Directory input versus file-based enclosing-root lookup.
[^roots]: Only knowledge-directory fallback; explicit configured roots have their own paths.
[^nav]: Merged collection identity uses the collection path.
[^colors]: Type colors use a string hash, not the Strict type list.
[^old-init]: Original design is historical where contradicted by the implemented CLI.
[^discovery-draft]: Proposed one-level discovery has not become this function's contract.
[^studio-index]: Inspected local product topology and extension declarations.
[^studio-create]: Peer-documented static minimal generator; not executed in this survey.
[^studio-profile]: Peer-documented advisory policy context separate from validation.
[^gem-index]: Ecosystem and four component roots, rather than a single global knowledge directory.
[^gem-style]: Local authoring recommendations are evidence, not instructions adopted by this task.
[^gem-personal]: Personal seed differs substantially from the same ecosystem's software maps.
[^upstream-move]: The old directory declares itself frozen and points to the new repository.
[^spec]: Canonical v0.2 §§1–4, 8, 11–12; the short format summary is separate from local recommendations.
[^upstream]: Official examples of producer and consumer arrangements.
[^ga4]: Observed datasets, references, and tables directories.
[^retail]: Observed metrics, policies, tables, and computation resources.
[^crm]: Domain-practice navigation, with legacy metadata visible in the inspected index.
[^threats]: Project describes the generated OKF tree and its canonical-source build.
[^workbench]: Project-authored feature list, consulted 2026-09-12.
[^agent-knowledge]: Domain questions precede scaffold and local schema records.
[^diataxis]: Reader-need framework; applied here as guidance, not OKF taxonomy.
[^para]: Personal organization by actionability; not a required lifecycle folder model.
[^plan]: Proposed follow-up; writing it does not begin implementation.
