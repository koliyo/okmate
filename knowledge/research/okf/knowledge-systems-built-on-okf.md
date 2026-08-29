---
type: Research Report
title: Knowledge systems built on Open Knowledge Format
description: A 2026 ecosystem census distinguishes OKF-canonical knowledge systems from database-backed projections, bundle consumers, operational profiles, and supporting tools.
tags: [domain/okf, concern/agents, concern/ecosystem, concern/governance, concern/retrieval]
status: draft
generated: { by: process:codex, at: 2026-08-29T09:55:11Z }
stale_after: 2026-11-29
authority: exploratory
owners: [human:nils]
sources:
  - id: okf-spec
    resource: https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md
    title: Open Knowledge Format v0.2 specification
    author: organization:google-cloud
  - id: reference
    resource: https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/README.md
    title: Google Cloud OKF reference agent and visualizer
    author: organization:google-cloud
  - id: census-1
    resource: https://api.github.com/search/repositories?q=topic%3Aopen-knowledge-format&per_page=100&page=1
    title: GitHub repositories tagged open-knowledge-format, page 1
    author: organization:github
  - id: census-2
    resource: https://api.github.com/search/repositories?q=topic%3Aopen-knowledge-format&per_page=100&page=2
    title: GitHub repositories tagged open-knowledge-format, page 2
    author: organization:github
  - id: memanto
    resource: https://github.com/moorcheh-ai/memanto
    title: Memanto repository
    author: organization:moorcheh-ai
  - id: memanto-docs
    resource: https://docs.memanto.ai/integrations/okf
    title: Memanto Open Knowledge Format integration
    author: organization:moorcheh-ai
  - id: memanto-essay
    resource: https://medium.com/@majid.fekri/okf-isnt-replacing-the-vector-database-it-s-freeing-it-3cc64ab47fa3
    title: OKF Isn't Replacing the Vector Database. It's Freeing It.
    author: human:majid-fekri
  - id: remnic
    resource: https://github.com/joshuaswarren/remnic
    title: Remnic repository
    author: human:joshua-warren
  - id: grafito
    resource: https://github.com/jpmanson/GrafitoDB
    title: GrafitoDB repository
    author: human:jp-manson
  - id: turbomem
    resource: https://github.com/turbomem/turbomem
    title: Turbomem repository
    author: organization:turbomem
  - id: pi-wiki
    resource: https://github.com/zosmaai/pi-llm-wiki
    title: pi-llm-wiki repository
    author: organization:zosmaai
  - id: harness
    resource: https://github.com/pumblus/okf-harness
    title: OKF Harness repository
    author: human:pumblus
  - id: mnemo
    resource: https://github.com/jeromeetienne/mnemo_wiki
    title: mnemo_wiki repository
    author: human:jerome-etienne
  - id: citadel
    resource: https://github.com/MarkusNeusinger/cite-citadel
    title: Cite Citadel repository
    author: human:markus-neusinger
  - id: openknowledge
    resource: https://github.com/openknowledge-sh/openknowledge
    title: Open Knowledge repository
    author: organization:openknowledge-sh
  - id: data-wiki
    resource: https://github.com/aws-samples/sample-okf-llm-wiki
    title: AWS sample Data Wiki repository
    author: organization:aws
  - id: studio
    resource: https://github.com/saschb2b/okf-studio
    title: OKF Studio repository
    author: human:saschb2b
  - id: nodus
    resource: https://github.com/sorenwacker/nodus
    title: Nodus repository
    author: human:soren-wacker
  - id: kiso
    resource: https://github.com/oak-invest/kiso
    title: Kiso repository
    author: organization:oak-invest
  - id: living-docs
    resource: https://github.com/ejklock/living-docs-skill
    title: Living Docs repository
    author: human:ejklock
  - id: surface
    resource: https://github.com/Connorrmcd6/surface
    title: Surface repository
    author: human:connor-mcdonald
  - id: loremaster
    resource: https://github.com/PFreda-Lab/okf-loremaster
    title: OKF Loremaster repository
    author: organization:pfreda-lab
  - id: okmate
    resource: ../../../README.md
    title: OKMate README
    author: organization:koliyo
---

# Knowledge systems built on Open Knowledge Format

## Executive summary

Open Knowledge Format has already become more than a data-catalog interchange
format. The visible ecosystem includes agent-maintained wikis, shared agent
memory, GraphRAG engines, desktop knowledge workspaces, data and biomedical
knowledge production, Git-native governance, publishing, MCP serving, and a
long tail of converters and profiles. A GitHub topic census on 29 August 2026
returned 128 public repositories, but topic membership mixes complete systems,
libraries, skills, example bundles, articles, and experiments.[^census-1]
[^census-2]

The strongest common architecture is not “OKF instead of retrieval.” It is:

1. human-readable OKF files as durable or portable knowledge;
2. deterministic parsing, validation, graph construction, and indexing;
3. optional lexical, vector, or graph retrieval as rebuildable acceleration;
4. an agent, desktop application, CLI, or MCP server as the working interface;
5. Git or an explicit review queue as the governance boundary.

That layering follows the specification's actual boundary: OKF defines a
portable representation and trust vocabulary, while deliberately not
prescribing storage, serving, query, or runtime infrastructure.[^okf-spec]

The phrase “built on OKF” is nevertheless ambiguous. Some products use the
bundle as their authoritative working state. Others expose OKF as a lossless
interchange projection over a database or memory service. Still others only
consume or publish bundles. Those designs have different portability,
round-trip, review, and failure properties and should not be compared as if
they were equivalent.

## Scope and method

This survey asks which **knowledge systems** use OKF as part of a continuing
production, maintenance, retrieval, review, or presentation loop. It does not
count a parser, validator, one-off converter, example bundle, or agent skill as
a full system unless that project also provides a sustained knowledge workflow.
The companion [state-of-the-art tools and workflows](okf-tools-and-workflows.md)
report covers the portable toolchain, while [OKMate feature gaps versus the OKF
tool ecosystem](../okmate/okf-tool-gaps.md) compares individual CLI, MCP, and
editor capabilities. This report concentrates on assembled knowledge systems
and their authoritative-state contracts.

Discovery combined:

- all 128 public repositories returned by GitHub's
  `topic:open-knowledge-format` search on 29 August 2026;
- the upstream specification and reference implementation;
- untagged systems found through repository and web search, including Memanto,
  Open Knowledge, OKMate, and the Google and AWS reference applications;
- direct inspection of project READMEs and documentation for the systems
  profiled below.

The census is comprehensive for the stated query at the snapshot date, not for
all public or private OKF use. Topic tags are self-selected; repositories can be
untagged, mislabeled, duplicated, renamed, or newly created after the snapshot.
Descriptions and feature claims are project-authored unless an independent
test is explicitly identified. Stars are omitted because they measure
attention, not architectural completeness or correctness.

## A taxonomy of “built on OKF”

| Class | Test | Portability consequence |
| --- | --- | --- |
| A. OKF-canonical | The files are the authoritative working state; derived databases and indexes can be discarded. | Another conforming consumer can recover the durable knowledge without the originating runtime. |
| B. OKF-native projection | The operational system has another authoritative model but imports, exports, or synchronizes OKF with documented round-trip behavior. | Exit is plausible, but fidelity depends on extension preservation, identity mapping, and conflict semantics. |
| C. OKF consumer or publisher | The system reads a bundle to browse, search, serve, validate, or publish it, without owning its authoring lifecycle. | Knowledge stays portable, but the system is a view or delivery layer rather than the source of truth. |
| D. OKF profile or operating model | The project adds a domain schema, governance rules, freshness, federation, signing, or agent procedures. | Interoperability depends on keeping additions optional and preserving base OKF tolerance. |

A single project can span classes. For example, a desktop workspace can treat
OKF as canonical while using a local database for canvas coordinates, and an
MCP runtime can be both a consumer and part of an authoring loop.

## Reference baseline

OKF v0.2 requires only a `type` field on concept documents and uses standard
Markdown links. It adds optional provenance, verification, trust, lifecycle,
freshness, and attested-computation metadata while requiring consumers to
tolerate unknown types and extension fields. Indexes are progressive-
disclosure aids, and Git is recommended distribution rather than a mandatory
runtime.[^okf-spec]

The Google reference repository is a bounded data-knowledge producer plus a
self-contained visualizer. It demonstrates structured catalog harvesting,
optional source enrichment, generated bundles, graph and backlink navigation,
search, and static distribution. It is a proof of the producer/consumer
contract, not a general collaborative knowledge application.[^reference]

This baseline matters because ecosystem projects sometimes describe Obsidian
wikilinks, mandatory directory indexes, typed edges, embeddings, or a database
as if OKF itself required them. Those are useful profiles or implementations,
but not portable base-format requirements.

## System landscape

### Agent-maintained wikis and research knowledge

| System | Class | Working model | Distinctive contribution |
| --- | --- | --- | --- |
| [pi-llm-wiki](https://github.com/zosmaai/pi-llm-wiki) | A + D | Raw source packets, source pages, canonical wiki pages, and generated indexes/logs in an Obsidian-compatible OKF v0.2 vault. | A self-maintaining wiki integrated with Pi and exposed to other agents over MCP; it reads legacy and OKF pages without forced migration.[^pi-wiki] |
| [OKF Harness](https://github.com/pumblus/okf-harness) | A + D | Immutable sources and a manifest feed cited wiki pages; agents use deterministic evidence, read, and graph commands. | A terminal-native, multi-agent harness that separates source custody from agent-authored synthesis.[^harness] |
| [mnemo_wiki](https://github.com/jeromeetienne/mnemo_wiki) | A + D | `sources/`, agent-authored `wiki/`, and a `CLAUDE.md` operating manual; its CLI performs deterministic validation, indexing, BM25 search, link repair, moves, and logging. | A particularly clear “tool is the hands, external agent is the head” split; gaps become Question pages and retired pages are preserved.[^mnemo] |
| [Cite Citadel](https://github.com/MarkusNeusinger/cite-citadel) | A + D | Immutable `raw/`, an LLM-owned and fully cited OKF `wiki/`, and editable rules for schema, tasks, formats, and genres. | Citation-first personal research wiki with MCP retrieval and a rule layer that changes agent behavior without changing code.[^citadel] |
| [Living Docs](https://github.com/ejklock/living-docs-skill) | A + D | Repository documentation is the source of truth; a skill and deterministic CLI govern constitutions, ADRs, BDRs, PRDs, issues, research, diagrams, and public export. | “One home per fact” and “indexed or it does not exist” turn OKF into a documentation operating model rather than merely a file schema.[^living-docs] |

These systems converge on a three-layer pattern: immutable or separately
controlled sources, agent-maintained synthesis, and deterministic mechanics.
They differ mainly in agent host, schema richness, source manifest, citation
strictness, and whether retrieval lives in the CLI, MCP, or the agent itself.

### Agent memory and hybrid retrieval

| System | Class | Working model | Distinctive contribution |
| --- | --- | --- | --- |
| [Memanto](https://github.com/moorcheh-ai/memanto) | B, with an A claim | A memory agent performs extraction, consolidation, reconciliation, forgetting, briefing, and semantic recall; OKF supports import, export, sync, and migration. | Lossless extension preservation under `x_memanto`, explicit memory types, provenance, temporal recall, policies for expiry, and portable movement between agent frameworks.[^memanto][^memanto-docs] |
| [Remnic](https://github.com/joshuaswarren/remnic) | A/B hybrid | Plain Markdown memory files are the source of truth and the memory directory doubles as an OKF v0.1 bundle; hybrid indexes are rebuildable. Remnic's `category` remains authoritative and `type` is interoperability metadata. | Shared local memory across many agent hosts, automatic extraction and recall, provenance, temporal supersession, graph recall, correction, governance, and continuous external connectors.[^remnic] |
| [GrafitoDB](https://github.com/jpmanson/GrafitoDB) | B + C | OKF bundles import into an embedded SQLite property graph with text, vector, hybrid, graph expansion, Cypher, and token-budgeted context, then export back to OKF. | The most explicit GraphRAG bridge: concepts, links, citations, trust, freshness, review queues, MCP, and queryable graph storage behind an `OKFBundle` façade.[^grafito] |
| [Turbomem](https://github.com/turbomem/turbomem) | B, experimental | A TypeScript embedded memory runtime uses PGlite and embeddings; an experimental `@turbomem/okf` package bridges OKF and the memory model. | Framework adapters for Mastra and the Vercel AI SDK make OKF one interchange path into an application-embedded memory service.[^turbomem] |

Memanto exposes the most important classification caveat. Its essay argues for
OKF as canonical, Git-versioned substrate and semantic retrieval as a disposable
index. Its operational documentation, however, describes the bundle as an
export/import/sync representation around a semantic engine. Until the public
storage and conflict contract shows the files driving normal writes and recall,
the conservative classification is a lossless OKF-native projection, not a
proven OKF-canonical store.[^memanto-essay][^memanto-docs]

The essay's broader architectural claim is sound and visible elsewhere: a
format does not retrieve by itself. Structural traversal works when a question
maps to curated identity and taxonomy; lexical, vector, or graph search covers
fuzzy discovery and scale. The useful invariant is that any derived index can
be rebuilt and every result can resolve back to an inspectable concept, source,
and revision.[^memanto-essay]

### End-to-end operations, governance, and delivery

| System | Class | Working model | Distinctive contribution |
| --- | --- | --- | --- |
| [Open Knowledge](https://github.com/openknowledge-sh/openknowledge) | A + C + D | Markdown and Git pass through deterministic checks and evidence-backed pull requests into immutable, health-labeled MCP generations; rollback closes the loop. | The broadest production lifecycle: setup, validation, audit, claims/evidence, evaluation, quality gates, connectors, registry, HTML export, MCP, jobs, and deployment.[^openknowledge] |
| [AWS Data Wiki](https://github.com/aws-samples/sample-okf-llm-wiki) | A + C + D | Data catalogs and context documents are harvested into OKF bundles, reviewed and searched, incrementally refreshed, and served through authenticated MCP and a web console. | A vertical enterprise reference with scoped re-harvest, semantic index synchronization, OAuth2 machine credentials, graph browsing, and a reproducible text-to-SQL benchmark.[^data-wiki] |
| [Surface](https://github.com/Connorrmcd6/surface) | D | An OKF-conformant documentation hub records the code surfaces a document describes; deterministic checks fail when referenced code changes without corresponding documentation. | Source-bound freshness computed from Git rather than an author-supplied timestamp.[^surface] |
| [Kiso](https://github.com/oak-invest/kiso) | C + D | A CLI validates an OKF bundle and builds a static site containing HTML, original Markdown, `llms.txt`, and `sitemap.xml`, with profiles and GitHub Actions support. | A focused publishing boundary for simultaneous human and agent delivery.[^kiso] |
| [OKMate](https://github.com/koliyo/okmate) | C + D | A portable Rust engine parses, validates, graphs, searches, benchmarks, and builds artifacts; the application adds CLI, live preview, desktop, multi-root configuration, and strict evidence policy. | A local, deterministic inspection and review surface that keeps the engine UI-neutral and supports base versus organizational profiles.[^okmate] |

Open Knowledge and AWS Data Wiki go furthest toward a production control plane.
Surface isolates one missing operational primitive—freshness tied to the source
revision—while Kiso and OKMate deliberately stop at delivery and review. These
are complementary boundaries rather than a single feature ladder.

### Human-facing knowledge workspaces

| System | Class | Working model | Distinctive contribution |
| --- | --- | --- | --- |
| [OKF Studio](https://github.com/saschb2b/okf-studio) | A + C | A local folder opens read-only as graph, reader, search, filters, relationship panels, and composition views; agent edits go to a staged tree for explicit review and transactional application. | The strongest dedicated OKF desktop author-review model, with Git operations and visible permissions rather than autonomous direct writes.[^studio] |
| [Nodus](https://github.com/sorenwacker/nodus) | A + D | Editable OKF Markdown notes sit on a canvas; a local LibSQL database supports layout and application state. | Canvas-native knowledge, Typst math, PDF-to-graph import, citation verification, Zotero/BibTeX, storylines, local/cloud models, and MCP, while the notes remain portable files.[^nodus] |

The topic census also reveals adjacent human interfaces: Akasha provides a 3D
graph explorer, `okf-roam` brings backlinks and navigation to Emacs,
`okf-enforcer` validates Obsidian vaults, `obsidian-okf` authors and exports
vaults, `vibe-tent` manages coding-agent intent and handoff in Obsidian, and
`vscode-weave-context` adds repository-knowledge authoring in VS Code. These
projects show that OKF is becoming a substrate beneath existing interaction
styles rather than forcing a single application shell.[^census-1]

### Domain systems and published knowledge products

| System or bundle | Role | Evidence of a domain-specific layer |
| --- | --- | --- |
| [OKF Loremaster](https://github.com/PFreda-Lab/okf-loremaster) | Biomedical evidence-production system | Five agents plus deterministic checks search PubMed/PMC, extract and synthesize evidence into OKF v0.2, record source licenses and export safety, and optionally vectorize the corpus for RAG.[^loremaster] |
| [OpenDPP knowledge](https://github.com/OpenDPP/opendpp-knowledge) | Generated API knowledge product | Regenerates cross-linked OKF documentation for endpoints, schemas, and webhooks from a live OpenAPI specification.[^census-1] |
| [Public Procurement OKF](https://github.com/downoff/public-procurement-okf) | Public-data bundle | Packages tens of thousands of tenders from multiple countries and sources as a public-procurement knowledge bundle.[^census-1] |
| [ONS OKF](https://github.com/chris-page-gov/okf-ons) | Government-data discovery bundle | Organizes knowledge for discovering and selecting UK Office for National Statistics data.[^census-1] |
| [Dharma OKF](https://github.com/dharma-okf-foundation/dharma-okf) | Cultural-knowledge profile and bundle | Adds a `not` extension for mistranslations that agents should avoid, illustrating domain-negative knowledge.[^census-1] |
| [Open Craft Commons](https://github.com/itokri2024/open-craft-commons) | Stewarded cultural reference | Publishes verified, openly licensed, machine-readable knowledge about Indian craft.[^census-1] |
| [Accounting Knowledge Graph](https://github.com/Spark-Collective/accounting-knowledge-graph) | Professional-practice graph | Models accounting practice as agent-readable Markdown plus typed edges, initially for Belgium.[^census-2] |
| [OPFF](https://github.com/eidos-agi/prim.opff) | Personal-finance profile | Defines an additive OKF v0.2 profile for portable household-finance packs.[^census-2] |

Vertical use is still early, but it demonstrates two different products:
systems that **produce** governed knowledge from external evidence, and bundles
that are themselves the distributable knowledge product. OKF's tolerance for
domain types and extension fields enables both without a central registry.

## The supporting ecosystem

The 128-repository census is dominated numerically by supporting components,
not full knowledge systems. They matter because they make the systems above
composable.

### Authoring, validation, and agent operating layers

- General toolkits include `scaccogatto/okf-skills`, `serradura/okf`,
  `okfcli/okf`, `W4G1/okf`, `abcubed3/okf`, `skosovsky/okf`, `factile`,
  `okf-toolkit`, `okft`, and `okf-tools`.
- Agent authoring layers include `agent-knowledge`, `janet-agent`,
  `okf-knowledge`, `okf-author`, `okforge`, `okf-system`, `claude-okf`,
  `agent-smith`, `batcave`, and multiple reusable OKF skills.
- Profile and governance work includes `okf-federation`, `signed-okf`,
  `okf-grc`, `AIX`, `Surface`, `KAUT`, `spec-drift`, and the OKF operational
  layer.[^census-1][^census-2]

### Production, conversion, and synchronization

- Generic generation and crawling: OKFy, `okfgen`, `okf-kit`,
  `website_to_okf`, `okf-convert`, `video-to-okf`, and `sleepwalker`.
- Structured sources: `okfdump` for SQL databases, `okf-weaver` for SQL/dbt,
  `repoglyph-py` and `bran` for repositories, the Confluence producer,
  WordPress knowledge layer, and Feishu/Notion/Obsidian/GitHub conversion
  utilities collected in `awesome-okf`.
- Domain production: Data Wiki, OKF Loremaster, the ONS and procurement
  bundles, API knowledge generation, and job-sheet production for voice
  agents.[^census-1][^census-2]

### Retrieval, MCP, and presentation

- Retrieval and graph infrastructure includes `okq`, `okf-ingest`,
  `okf-agents`, `okf-tools`, GrafitoDB, Turbomem, `OpenContextScitool`, and
  several OKF MCP servers.
- Presentation includes OKF Studio, Nodus, Akasha, Kiso, OKF web servers,
  Obsidian and Emacs integrations, offline HTML viewers, and knowledge hubs.
- Many toolkits combine CLI, library, MCP, agent skill, static graph, and CI
  action in one repository. That packaging lowers adoption friction but also
  makes capability names poor evidence of a stable architectural boundary.[^census-1]

## Cross-system architecture patterns

### 1. The bundle as source, index as cache

The most portable systems make every retrieval structure disposable. Remnic
states this directly for its Markdown store; GrafitoDB imports into SQLite and
can export back; the Memanto essay argues for the same separation; OKMate keeps
search and graph artifacts derived. This design makes changing retrieval
engines an indexing migration rather than a knowledge migration.[^remnic]
[^grafito][^memanto-essay][^okmate]

### 2. Progressive disclosure before open-ended search

Indexes, metadata summaries, and graph neighborhoods provide cheap orientation
before full-page reads. Wiki systems add deterministic lexical search, while
memory and GraphRAG systems add semantic retrieval for fuzzy questions. The
systems that expose both treat traversal and search as complementary query
plans over the same concepts.[^pi-wiki][^mnemo][^grafito]

### 3. Separate judgment from mechanics

The agent decides what a source means, which concept changes, and whether two
claims conflict. Deterministic code owns parsing, schema checks, file movement,
index rebuilding, link integrity, caching, and evaluation. This division is
explicit in mnemo_wiki, OKF Harness, Living Docs, and the upstream reference
producer.[^mnemo][^harness][^living-docs][^reference]

### 4. Review the write, not only the answer

The strongest systems make proposed knowledge changes inspectable. OKF Studio
stages agent edits, Open Knowledge uses evidence-backed pull requests and
immutable served generations, and AWS Data Wiki provides review and operational
surfaces around harvested changes. This is a deeper control than attaching a
citation only after retrieval.[^studio][^openknowledge][^data-wiki]

### 5. Domain profiles remain additive

Useful systems need fields and rules beyond base OKF: memory confidence,
negative translations, typed claims, export safety, source binding, federation,
signatures, or application layout. Interoperability survives when these remain
extensions, base fields retain their portable meaning, and round trips preserve
unknown metadata.[^okf-spec][^memanto-docs][^loremaster]

## Gaps and risks

### Canonicality is often asserted, not demonstrated

An export button does not prove that OKF is the operational source of truth.
For Class B systems the material questions are:

- Does a normal write update the bundle first or a private database first?
- Can the index and database be deleted and rebuilt without semantic loss?
- Are stable identities, links, deletions, supersession, and conflicts preserved?
- Are unknown frontmatter and body structures retained across round trips?
- Can two writers merge through ordinary Git, or is there a hidden single-writer
  authority?

Memanto documents unknown-field preservation and an `x_memanto` extension,
which is stronger than a shallow export, but the canonical-storage claim still
needs an executable recovery and conflict test.[^memanto-docs]

### Version and profile fragmentation

The current field spans v0.1 and v0.2, standard Markdown links and Obsidian
wikilinks, optional versus required indexes, and different meanings for types,
status, confidence, and relationships. Base OKF intentionally permits much of
this variation, but a system claiming interoperability should publish its
target version, extensions, accepted link forms, and lossless round-trip tests.
The specification requires standard Markdown links and unknown-field
preservation; wikilinks and typed edges are profile additions.[^okf-spec]

### Security and authorization sit above the format

Trust metadata is not access control. Systems serving MCP or synchronizing
cloud sources need path containment, principal and namespace scoping,
authentication, publication filters, secret separation, and a write gate.
Open Knowledge, Data Wiki, OKF Studio, Memanto, and Remnic expose different
parts of this larger control plane; no ecosystem-wide authorization profile is
yet evident.[^okf-spec][^openknowledge][^data-wiki][^studio][^memanto][^remnic]

### Evaluation is the exception

AWS Data Wiki publishes a reproducible BIRD text-to-SQL benchmark and reports
74.0 execution accuracy using only the wiki over MCP. That is promising but
project-authored evidence. Most projects demonstrate features rather than
retrieval quality, authoring accuracy, freshness, round-trip fidelity, or
recovery from corruption. Cross-system evaluation should measure those
properties separately.[^data-wiki]

### The ecosystem is young and README-heavy

The census is active and diverse, but many repositories are small experiments,
templates, or newly published tools. Feature matrices should therefore record
evidence level—description, documentation, test, release, benchmark, or
independent deployment—instead of treating every README claim as implemented
production behavior.[^census-1][^census-2]

## A practical evaluation framework

For a knowledge system claiming to be built on OKF, test five contracts.

### Storage contract

1. Identify the authoritative state.
2. Delete derived indexes and rebuild them from the bundle.
3. Import, edit, export, and compare unknown metadata, body structure, links,
   lifecycle, provenance, and identity.
4. Test moves, deletions, conflicts, and concurrent writers.

### Retrieval contract

1. Separate index traversal, lexical search, semantic search, and graph
   expansion in traces.
2. Return concept IDs, headings, citations, lifecycle, trust, freshness, and
   bundle revision with every hit.
3. Benchmark retrieval before generative answer composition.
4. Rebuild the index under a new engine and compare results.

### Authoring and review contract

1. Preserve immutable source material or exact source snapshots.
2. Show the proposed semantic and file diff before promotion.
3. Run deterministic validation and graph checks on the candidate state.
4. Bind approval to the exact revision and invalidate it after later changes.

### Operations contract

1. Detect stale, conflicting, orphaned, and unsupported knowledge.
2. Expose health, provenance, and generation state to consumers.
3. Keep rollback and recovery independent of the agent or model provider.
4. Define publication and authorization boundaries separately from trust.

### Portability contract

1. Declare the OKF version and every extension/profile.
2. Accept unknown types and preserve unknown fields.
3. Use standard Markdown links for the portable graph.
4. Open the output with at least one independent consumer.

## Implications for OKMate

This survey supports OKMate's existing separation between a portable engine and
the application shell. It also points to the next competitive questions:

- **Canonicality proof:** make it trivial to show that search, graph, HTML, and
  caches rebuild from the files and that no hidden store is authoritative.
- **Interoperability harness:** add fixture-based import/inspect/round-trip tests
  against representative v0.1, v0.2, Obsidian-adjacent, memory-extension, and
  vertical-profile bundles.
- **Hybrid retrieval adapters:** keep lexical retrieval as the deterministic
  baseline, but define a replaceable semantic index contract whose hits resolve
  to concept IDs, headings, citations, trust, freshness, and revision.
- **Reviewable authoring:** prefer staged changes, evidence views, validation,
  and revision-bound promotion over autonomous file mutation.
- **Operational health:** surface source drift, staleness, broken links,
  conflicts, retrieval regressions, and degraded generations as distinct states.
- **Profile visibility:** show which requirements come from base OKF, Strict,
  or a producer-specific extension rather than presenting one policy as the
  format itself.

These are exploratory implications, not approved roadmap commitments.

## Conclusion

OKF is becoming a thin waist for several different knowledge-system families.
Its most credible role is a durable, inspectable knowledge substrate between
producers, agents, retrieval engines, review workflows, and human applications.
The ecosystem is not converging on one database, one graph model, one agent, or
one user interface—and the specification is designed not to require that.

The important divide is therefore not vector versus vectorless, or wiki versus
memory. It is **recoverable open state versus a proprietary authoritative
state**. Class A systems make the bundle the recoverable truth. Class B systems
can approach the same portability if they prove lossless round trips and
conflict behavior. Class C and D systems add valuable interfaces and policy
without needing to own the knowledge. A mature OKF ecosystem needs all four,
but it should name them honestly.

[^okf-spec]: OKF v0.2 scope, required fields, extension tolerance, standard Markdown links, trust and lifecycle metadata, progressive indexes, and runtime non-goals.
[^reference]: Upstream reference agent, bounded enrichment, generated example bundles, and self-contained visualizer.
[^census-1]: GitHub topic search snapshot, first 100 of 128 public repositories returned on 2026-08-29; repository descriptions are project-authored discovery evidence.
[^census-2]: GitHub topic search snapshot, remaining 28 of 128 public repositories returned on 2026-08-29; repository descriptions are project-authored discovery evidence.
[^memanto]: Memanto memory lifecycle, agent integrations, local/cloud backends, provenance, forgetting, briefing, and OKF migration claims.
[^memanto-docs]: OKF export/import/sync behavior, `x_memanto` extensions, field preservation, and exported bundle layout.
[^memanto-essay]: Author's argument for OKF as canonical substrate and semantic retrieval as a rebuildable, swappable index; this is an interested-party architectural claim.
[^remnic]: File-authoritative memory, rebuildable hybrid indexes, OKF v0.1 interoperability, agent integrations, provenance, correction, and connectors.
[^grafito]: SQLite property graph, OKF import/export and façade, hybrid retrieval, citations, trust/freshness filters, token-budgeted context, and MCP.
[^turbomem]: Embedded TypeScript memory architecture and experimental `@turbomem/okf` bridge.
[^pi-wiki]: OKF v0.2/legacy vault behavior, source/canonical/generated layers, Pi integration, deterministic indexes, and MCP access.
[^harness]: Source-manifest-to-cited-wiki pipeline and deterministic agent-facing evidence, read, and graph operations.
[^mnemo]: Source/wiki/instructions architecture, external-agent boundary, deterministic CLI, BM25 search, health checks, questions, and retirement behavior.
[^citadel]: Immutable raw sources, cited OKF wiki, rules layer, MCP, and local-model option.
[^openknowledge]: Git-native knowledge lifecycle, deterministic checks, evidence-backed PRs, immutable MCP generations, governance, evaluation, and publishing.
[^data-wiki]: Data harvesting, freshness, semantic indexing, authenticated MCP, web console, review loop, and project-authored BIRD benchmark.
[^studio]: Local desktop reader/graph, search and composition views, staged agent changes, transactional apply, and Git workflow.
[^nodus]: OKF file storage, canvas editing, local layout database, PDF/citation workflows, storylines, model tools, and MCP.
[^kiso]: Bundle validation and static publishing to HTML, source Markdown, `llms.txt`, and sitemap with profiles and CI integration.
[^living-docs]: Repository-first documentation, governance invariants, deterministic mechanics, document trail, and public export.
[^surface]: Source-bound documentation freshness and deterministic build failure when described code changes without its documentation.
[^loremaster]: Multi-agent biomedical evidence pipeline, OKF v0.2 output, deterministic checks, optional RAG indexing, licensing, and export-safety metadata.
[^okmate]: Current in-repository description of the portable `okf` engine and OKMate CLI, preview, desktop, Strict profile, multi-root, build, search, and benchmark surfaces.
