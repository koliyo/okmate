---
type: Research Report
title: OKMate feature gaps versus the OKF tool ecosystem
description: A August 2026 survey of OKF CLIs, viewers, MCP servers, and editor tools finds OKMate strong on review HTML, strict evidence, multi-root git, and retrieval benchmarks, and thin on MCP, scaffolding, lint-versus-validate, ranked search, SARIF, and graph navigation verbs.
tags: [domain/okmate, domain/okf, concern/agents, concern/review, concern/retrieval, concern/tooling]
status: draft
generated: { by: process:cursor, at: 2026-08-28T15:30:00Z }
stale_after: 2026-11-28
authority: exploratory
owners: [human:nils]
sources:
  - id: overview
    resource: ../../architecture/system-overview.md
    title: OKMate system overview
    author: process:cursor
    last_modified: 2026-08-28
  - id: readme
    resource: ../../../README.md
    title: Published OKMate CLI and stack
    author: process:git
    last_modified: 2026-08-28
  - id: okf-readme
    resource: ../../../okf/README.md
    title: Portable OKF engine
    author: process:git
    last_modified: 2026-08-28
  - id: search-rs
    resource: ../../../okf/src/search.rs
    title: Lexical chunk search (term-contains)
    author: process:git
    last_modified: 2026-08-28
  - id: cli
    resource: ../../../src/cli.rs
    title: okmate CLI surface
    author: process:git
    last_modified: 2026-08-28
  - id: okf-lib
    resource: ../../../okf/src/lib.rs
    title: okf check, inspect, search, build
    author: process:git
    last_modified: 2026-08-28
  - id: okf-tools
    resource: ../okf/okf-tools-and-workflows.md
    title: State-of-the-art OKF tools and workflows
    author: process:codex
    last_modified: 2026-08-26
  - id: landscape
    resource: agent-dev-landscape.md
    title: OKMate and the agent knowledge-management landscape
    author: process:cursor
    last_modified: 2026-08-28
  - id: extract
    resource: ../../plans/okf/okmate.md
    title: Okmate extractable Rust OKF mate
    author: process:cursor
    last_modified: 2026-08-26
  - id: okf-app
    resource: ../../plans/okf/rocci-okf-app.md
    title: Standalone Rocci OKF review and query application
    author: process:cursor
    last_modified: 2026-08-26
  - id: spec
    resource: https://github.com/GoogleCloudPlatform/open-knowledge-format/blob/main/SPEC.md
    title: Open Knowledge Format v0.2 specification
    author: organization:google-cloud
  - id: google-viz
    resource: https://github.com/GoogleCloudPlatform/open-knowledge-format
    title: Canonical OKF spec, reference agent, and visualizer
    author: organization:google-cloud
  - id: okfcli
    resource: https://github.com/okfcli/okf
    title: Agent-native OKF Go CLI
    author: organization:okfcli
  - id: useokf
    resource: https://useokf.com/
    title: okfcli product site
    author: organization:okfcli
  - id: okfgem-cli
    resource: https://okfgem.com/docs/cli/
    title: okf-gem CLI verbs
    author: human:rodrigo-serradura
  - id: okfgem-mcp
    resource: https://okfgem.com/docs/mcp/
    title: okf-gem MCP server
    author: human:rodrigo-serradura
  - id: openknowledge
    resource: https://github.com/openknowledge-sh/openknowledge
    title: Open Knowledge CLI
    author: organization:openknowledge-sh
  - id: okq
    resource: https://github.com/mikevalstar/okq
    title: okq deterministic OKF query CLI
    author: human:mike-valstar
  - id: galkleinman
    resource: https://github.com/galkleinman/okf-toolkit
    title: okft Rust validator, MCP, and GitHub Action
    author: human:gal-kleinman
  - id: workbench
    resource: https://github.com/koizumikento/okf-workbench
    title: OKF Workbench VS Code extension
    author: human:koizumi-kento
  - id: akdira
    resource: https://github.com/akdira/okf-toolkit
    title: Python okf-toolkit CLI
    author: organization:akdira
  - id: okf4net
    resource: https://github.com/jchable/okf4net
    title: OKF4net .NET library, CLI, and MCP
    author: human:julien-chable
  - id: hdean
    resource: https://github.com/hdean-ssp/okf-mcp
    title: okf-mcp hybrid search and CRUD
    author: human:hdean-ssp
  - id: okf-schema
    resource: https://github.com/gsemet/okf-schema
    title: okf-schema JSON Schema frontmatter validation
    author: human:gaetan-semet
  - id: w4g1
    resource: https://github.com/W4G1/okf
    title: crates.io okf Rust CLI, lint --fix, studio, refactor
    author: organization:w4g1
  - id: crates-okf
    resource: https://crates.io/crates/okf
    title: Published crates.io okf package
    author: organization:w4g1
  - id: okftool
    resource: https://github.com/ryansann/okftool
    title: okftool embeddable validator and 28-rule linter
    author: human:ryan-sann
  - id: abcubed3
    resource: https://github.com/abcubed3/okf
    title: OKF-go suite README
    author: human:abcubed3
  - id: iwe-okf
    resource: https://iwe.md/docs/agentic/okf/
    title: IWE Open Knowledge Format support
    author: organization:iwe
  - id: harness
    resource: https://github.com/pumblus/okf-harness
    title: OKF Harness LLM-wiki workspace
    author: organization:pumblus
  - id: xsavik
    resource: https://github.com/xSAVIKx/okf-skills
    title: OKF producer skills and generic MCP host
    author: human:xsavik
---

# OKMate feature gaps versus the OKF tool ecosystem

## Scope

This report compares **OKMate to other Open Knowledge Format tools**, not to
Linear, DeepWiki, or Mem0. The broader agent-KM map is
[OKMate and the agent knowledge-management landscape](agent-dev-landscape.md).
The earlier workflow synthesis is
[State-of-the-art OKF tools and workflows](../okf/okf-tools-and-workflows.md);
that record listed four independent consumers plus Google's reference and
AWS Data Wiki. This pass re-surveyed GitHub and product docs in August 2026
and found a much larger field.[^okf-tools][^landscape]

Claims about OKMate are descriptive of this repository. Peer features are
taken from published READMEs and docs, not from running every binary.
Marketing numbers (parse throughput, coverage gates) are those projects'
claims. Positioning of what OKMate *should* copy is exploratory.

## Discovery

The previous survey named Google's spec and visualizer, `okfcli/okf`,
`okf-gem`, Open Knowledge (`okn`), and the AWS Data Wiki sample.[^okf-tools]

This pass additionally found, by GitHub and web search for "Open Knowledge
Format" / OKF CLI / MCP / VS Code:

| Tool | Language | Role |
| --- | --- | --- |
| [W4G1/okf](https://github.com/W4G1/okf) (`cargo install okf`) | Rust | Full CLI: init/new, validate/lint `--fix`, trust, graph, semantic `mv`/`rm`/`split`/`merge`, `okf studio` TUI; **owns the crates.io name `okf`** |
| [galkleinman/okf-toolkit](https://github.com/galkleinman/okf-toolkit) (`okft` → binary `okf`) | Rust | Spec-faithful validate vs lint, MCP, local graph web, GitHub Action, SARIF |
| [mikevalstar/okq](https://github.com/mikevalstar/okq) | Rust | Query/navigation: BM25, `find`/`get`/`neighbors`/`path`/`orphans`/`deadlinks`, agent skills |
| [ryansann/okftool](https://github.com/ryansann/okftool) | Rust | Embeddable validator/linter (28 rules, profiles, wasm/npm), SARIF, `.tar.gz` pack |
| [koizumikento/okf-workbench](https://github.com/koizumikento/okf-workbench) | Rust + VS Code | Editor authoring loop, 3D graph, guarded writes, `init`/`new`/`migrate` |
| [jchable/okf4net](https://github.com/jchable/okf4net) | C# | Zero-dep library, Native AOT `okf`, MCP with optional writes, Agent Framework tools, winget |
| [gsemet/okf-schema](https://github.com/gsemet/okf-schema) | Python | Per-`type` JSON Schema in `_schema/`, `okfkb` query DSL, skills |
| [akdira/okf-toolkit](https://github.com/akdira/okf-toolkit) | Python | Small init/new/validate/index/search/graph/stats CLI |
| [hdean-ssp/okf-mcp](https://github.com/hdean-ssp/okf-mcp) | Python | Hybrid BM25+vector search, write MCP (`commit`/`update`/`delete`) |
| [abcubed3/okf](https://github.com/abcubed3/okf) | Go | README claims harvest, assemble, MCP, LSP, HTML portal, Notion/Confluence sync |
| [iwe-org/iwe](https://github.com/iwe-org/iwe) | Rust | Markdown LSP + MCP that *also* scaffolds and validates OKF |
| [pumblus/okf-harness](https://github.com/pumblus/okf-harness) | TypeScript | Karpathy-style ingest/evidence workspace on OKF files |
| [xSAVIKx/okf-skills](https://github.com/xSAVIKx/okf-skills) | Go | Producer connectors (`okf-sqlite`, viz) behind a generic MCP host |

Google's reference agent (produce/enrich + one-file Cytoscape visualizer)
remains the canonical producer/viewer pair, now in
`GoogleCloudPlatform/open-knowledge-format`.[^google-viz][^spec]

`okfcli` `serve`/`render` were still listed as planned, not shipped.[^okfcli]
`abcubed3/okf` advertises a very wide surface; treat harvest/sync/LSP as
README claims until independently verified.

A naming collision matters for distribution: `cargo install okf` installs
W4G1's toolkit, not this repository's unpublished `okf` crate.[^crates-okf][^w4g1]
galkleinman documents the same clash with `okft`.[^galkleinman]

## What OKMate already ships

OKMate is a standalone knowledge application: portable `okf` engine plus
`okmate` CLI, Askama HTML, Axum preview, optional desktop window. Records stay
inert Markdown. The shipped agent contract is check / inspect / search / build
/ roots / sync / view / benchmark / timings.[^overview][^readme][^cli][^okf-lib]

Relative to the field, these are **uncommon or stronger here**:

- **Human review shell**, not only a force-directed graph: live preview,
  desktop window, static HTML build, concept pages, collections, settings.
  Most peers ship a Cytoscape-style viz or a TUI; few ship a documentation-like
  review app.[^readme][^google-viz][^okfgem-cli][^w4g1]
- **Strict owners-and-evidence profile** (footnotes aligned to `sources`,
  owners, git provenance codes) as a CI gate. Peers validate spec §11 and
  optional hygiene; few fail a build on source/footnote drift.[^okf-readme][^galkleinman][^okftool]
- **Multi-root registry with git fetch** (`okmate roots` / `sync`,
  `~/.okmate/`). okf-gem has `@slug` registry; Open Knowledge has
  connect/registry; OKF4net has a catalog. Git-backed roots plus a desktop
  picker is still rare.[^readme][^okfgem-cli][^openknowledge][^okf4net]
- **Retrieval benchmark** (`okmate benchmark` TOML → hit rate / MRR). Almost
  no other OKF CLI publishes this.[^okf-readme][^cli]
- **Load timings and parse cache** for preview. Operational, not a user
  feature, but unique in this set.[^okf-readme]
- **`llms.txt` in `build` output**, shared with Open Knowledge HTML
  export.[^okf-readme][^openknowledge]
- **Read-only authoring posture**: agents edit Markdown in git; the tool
  judges. Several peers add write MCP or `--fix`, which fights that
  posture unless gated.[^extract][^okf-app][^hdean][^w4g1]

Search is **not** BM25. `okf/README.md` says "BM25/lexical matching";
`search.rs` ANDs lowercase substring terms over metadata and heading
chunks.[^okf-readme][^search-rs] okq and okf-gem document real BM25 (Tantivy
or an opt-in index).[^okq][^okfgem-cli]

There is no MCP, no `init`/`new`, no lint-apart-from-check, no SARIF, no
index generator, no dedicated backlinks/orphans/path verbs (the graph JSON
from `inspect graph` already contains edges), and no editor
extension.[^cli][^okf-lib][^landscape]

## Feature comparison

Legend: **yes** = documented as shipped; **partial** = data or a nearby
command exists; **no** = not in the published surface; **claim** = README
only.

### Agent discovery and CI

| Capability | OKMate | Who has it |
| --- | --- | --- |
| JSON inspect/search/check | yes | Almost everyone |
| Machine-readable CLI schema (`okf schema`) | no | okfcli (command metadata); okq (JSON Schema of `--json` output) |
| JSON-default stdout, structured error envelopes, mapped exit codes | partial (`--format json`) | okfcli (JSON by default, typed envelopes) |
| SARIF / GitHub PR annotations | no | okfcli, galkleinman, okftool |
| Dedicated GitHub Action | no (this repo has its own CI) | galkleinman, abcubed3 (claimed) |
| MCP read tools + `okf://` resources | no | okf-gem, galkleinman, Open Knowledge, OKF4net, IWE, hdean, abcubed3 (claimed) |
| MCP writes (create/update/delete) | no (intentionally) | hdean, OKF4net (optional), workbench (previewed editor writes) |
| Installable agent skills | skill in-repo, not a CLI install | okq (`okq skills install`), okf-gem (`okf skill`), okf-schema, workbench (`AGENTS.md` section), Open Knowledge setup |

MCP is how KM tools get discovered in Cursor. The landscape record already
called this an adoption blocker; the new survey confirms it is table stakes
among OKF tools, not only among Mem0-class products.[^landscape][^okfgem-mcp][^galkleinman]

okf-gem's MCP design is the closest fit: **read-only**, bounded result sizes,
resources plus tools, prompts that restate the retrieval doctrine, no write
surface (Claude Code uses the skill + CLI to author).[^okfgem-mcp]

### Conformance versus hygiene

OKF §11 forbids rejecting a bundle for broken links, unknown types, or missing
optional fields. Several tools therefore split **validate** (MUST) from
**lint** (SHOULD / opinion).[^spec][^galkleinman][^okftool][^okfgem-cli]

OKMate folds both into `check` plus **profiles** (`base` vs `strict`). That is
a coherent alternative: strict *is* an organizational policy. What is missing
is a **disableable hygiene layer** that the spec says must not be fatal:
orphans, loose files, missing titles, index drift, heading hierarchy — with
stable rule IDs, `--deny`/`--allow`, and SARIF.[^okf-readme][^okftool][^galkleinman]

okftool is the most complete lint product: 28 rules, `okf-recommended` /
`okf-strict` / `okf-minimal`, glob overrides, inline `okf-lint-disable`, wasm
for embedding.[^okftool] W4G1 adds `lint --fix` and `validate --fix` (v0.1
migration).[^w4g1] okf-gem has `loose` as a one-check lens on unlinked
files.[^okfgem-cli]

### Scaffolding and bundle maintenance

| Capability | OKMate | Typical peer |
| --- | --- | --- |
| `init` empty bundle | no | Nearly all CLIs |
| `new` concept from template | no | akdira, W4G1, workbench, okq, okf-schema |
| Regenerate `index.md` (managed region) | no (strict *checks* membership) | okfcli, W4G1, okq, workbench, akdira, okf-schema |
| Link-aware `mv` / `rm` / split / merge | no | W4G1 (preview + rewrite backlinks) |
| `fmt` / frontmatter normalize | no | W4G1, okf-schema (`ruamel` comment-preserving) |
| v0.1 → v0.2 migrate | no | workbench (previewed), W4G1 `--fix` |
| Ignore file (`.okqignore`) | no | okq |

Workbench is the cautionary tale worth copying **procedurally**: write
commands print a plan; `--apply` is required; collisions fail closed; index
and `AGENTS.md` use managed regions.[^workbench]

### Query and graph

OKMate can dump the graph as JSON and search chunks. Peers expose **verbs
agents actually call**:

- **okq**: BM25 `search`; `find` with `--where`/`--tag`/`--stale`; `get`
  one section or field; `neighbors`; `backlinks`; shortest `path`; `orphans`
  / `deadlinks --check`; unique path suffix; Obsidian `[[wikilinks]]` and
  aliases as a compatibility layer.[^okq]
- **okf-gem**: `dirs` / `index` / `catalog` / `files` / `tags` / `types` /
  `stats` / `graph`; search with optional BM25+`--fuzzy`; `@slug` and
  `@all` multi-bundle search; `--fields` projection.[^okfgem-cli]
- **okf-schema `okfkb query`**: filter DSL plus arrow traversal
  (`finding -> concept -> principle`).[^okf-schema]
- **abcubed3 `assemble`**: BFS from a concept with depth, direction, and a
  **character budget** (README).[^abcubed3]
- **W4G1**: `trust`, `info`, `links --broken --check`, Mermaid
  `graph`.[^w4g1]

Token-bounded "assemble this neighborhood" is the agent-facing counterpart of
OKMate's inspect. Without it, agents either grep or load whole files.

### Human visualization

Google's visualizer, okf-gem `server`/`render`, galkleinman `serve --web`,
and W4G1 `studio` all treat the **graph as the primary UI**. Workbench adds
a 3D graph inside the editor.[^google-viz][^okfgem-cli][^galkleinman][^w4g1][^workbench]

OKMate's primary UI is the **article/review shell**. That is the right
default for evidence review. A self-contained graph HTML (or a Mermaid dump
for PRs) is still a gap for orientation and for sharing a bundle without
running the app.

Open Knowledge exports a static viewer plus `llms.txt` and publication
filters (`okf_publish: false`).[^openknowledge]

### Authoring environments

- **Workbench**: VS Code activity bar, Problems-panel diagnostics, 3D graph,
  guarded create, agent-skill generation.[^workbench]
- **IWE**: LSP go-to-definition / rename-with-references for markdown graphs;
  `iwe init --okf` plus schema validate. IWE is a PKM+LSP product that
  *speaks* OKF, not an OKF-only tool.[^iwe-okf]
- **abcubed3**: claims `okf lsp`.[^abcubed3]

OKMate has no editor integration. Agents in this repo already use a skill +
CLI; humans use the desktop viewer. An LSP is optional; a Workbench-like
loop is a different product.

### Production and evaluation (leave to specialists)

- **Google reference agent**: bounded produce then enrich (seeds, allowed
  hosts, max pages).[^google-viz]
- **okf-harness**: ingest plans, citation-to-bytes checks, evidence
  briefs.[^harness]
- **xSAVIKx skills**: deterministic connectors that *produce* bundles
  (e.g. SQLite).[^xsavik]
- **Open Knowledge**: audit/claims/eval/quality, Knowledge CI, hosted MCP
  runtime with health.[^openknowledge]
- **hdean**: local embeddings (bge-small) + sqlite-vec.[^hdean]
- **okf-schema**: opinionated `_schema/` that **rejects unknown types**,
  which the spec tells consumers to tolerate. Useful as a *profile*, not as
  base conformance.[^okf-schema][^spec]
- **abcubed3 harvest**: DB/OpenAPI/proto/git/web → concepts (README).[^abcubed3]

These are producer or platform jobs. OKMate's extract plan left MCP and
hosted query on a later horizon and kept canonical records as git
Markdown.[^extract][^okf-app]

## What OKMate could benefit from

Ordered by fit to the existing product (compiled, reviewable, CI-shaped
knowledge) rather than by how flashy the peer is.

### High — close the agent and CI gap without changing the system of record

1. **Read-only MCP** wrapping `check`, `inspect`, `search`, `inspect graph`,
   and a bounded `read_concept`. Follow okf-gem: resources + tools, result
   caps, no writes. This is the single largest adoption gap versus other OKF
   tools.[^okfgem-mcp][^landscape][^galkleinman]
2. **`okmate schema` (or clap JSON)** so an agent learns flags and exit
   codes in one call, plus documented exit-code table. Copy okfcli's
   intent, not necessarily JSON-default stdout.[^okfcli][^useokf]
3. **SARIF (and optionally GitHub workflow commands)** from `check`, so
   findings annotate the PR diff. Several Rust/Go peers already do
   this.[^okfcli][^galkleinman][^okftool]
4. **Ranked lexical search (true BM25)** over the existing heading/metadata
   chunks, with today's filters kept. Align the engine README with
   behavior. Do not add embeddings by default.[^search-rs][^okq][^okfgem-cli]
5. **Graph navigation verbs** on top of `inspect graph`: `backlinks`,
   `orphans`, `deadlinks --check`, `neighbors`, optional shortest `path`.
   Data is already in the engine; okq shows the CLI shape agents
   use.[^okq][^okf-lib]
6. **Hygiene lint split or extra profile rules** with stable IDs:
   orphans, unlinked files, missing recommended fields, index-prose drift.
   Keep `check --profile base` spec-faithful. Expose `--deny`/`--allow`
   rather than growing Strict until it contradicts §11.[^spec][^galkleinman][^okftool]

### High — authoring convenience that still writes ordinary Markdown

7. **`init` / `new`** with templates and collision-safe creates. Workbench's
   plan-then-`--apply` is the right safety model for a tool that otherwise
   refuses to own the files.[^workbench][^akdira][^w4g1]
8. **Managed `index.md` regeneration** (fenced or marked region), matching
   what okq/W4G1/workbench do, so agents stop hand-editing listings that
   Strict already validates.[^okq][^w4g1][^workbench]
9. **Skill installer** (`okmate skills install` into `.agents` / `.claude`)
   so other repos get the retrieve/author/check loop without copying this
   tree. okq embeds skills in the binary.[^okq][^okfgem-cli]

### Medium — orientation and refactor

10. **Self-contained graph HTML or Mermaid export** as a *secondary* view
    (`build` artifact or `okmate graph --format mermaid`). Do not replace
    the review shell.[^google-viz][^w4g1][^okfgem-cli]
11. **Token-bounded neighborhood assemble** (depth + char/token budget) for
    agent context packing. Distinct from search.[^abcubed3][^okq]
12. **Link-aware rename/move** with `--dry-run`. High value once bundles
    grow; W4G1 already rewrites backlinks and anchors.[^w4g1]
13. **`get` one concept / section / field** so agents do not ingest whole
    `inspect concept` blobs. okq's suffix lookup is a nice extra.[^okq]
14. **Pinned `--today` for staleness** in CI (W4G1, galkleinman). Small,
    deterministic, and OKMate already computes stale.[^w4g1][^galkleinman]

### Lower or "not OKMate"

- **Write MCP / `commit_concept`**: contradicts git-as-review and the
  extract plan. If it appears, keep it an adapter that writes Markdown the
  human still diffs.[^okf-app][^hdean]
- **Hybrid vector search**: hdean's job; OKMate already has a retrieval
  benchmark to *measure* lexical quality first.[^hdean][^okf-readme]
- **JSON Schema type registry**: useful as an optional profile; fatal
  unknown-`type` violates the spec's consumer rule.[^okf-schema][^spec]
- **Harvest / Notion sync / attested-computation execution / hosted
  runtime**: producer and platform tools. Stay complementary.[^abcubed3][^openknowledge][^w4g1]
- **Obsidian wikilinks as the native link form**: okq degrades gracefully;
  OKMate should keep bundle-root Markdown links as the authored
  contract.[^okq][^overview]
- **TUI studio or VS Code 3D graph as the primary app**: OKMate already
  chose a desktop review window. Graph viz is an export, not a second
  shell.[^readme][^w4g1][^workbench]
- **Taking the crates.io name `okf`**: already taken. If the engine is
  published, it needs another package name (this repo already lives as
  `okmate` on GitHub).[^crates-okf]

## Synthesis

The OKF tool field in August 2026 is no longer "Google plus three CLIs."
It is a crowded layer of validators, query CLIs, MCP servers, a VS Code
workbench, a .NET port, schema-opinionated Python, an LLM-wiki harness, and
a published crates.io `okf` that is **not** this engine.[^w4g1][^okf-tools]

OKMate is still one of the few **review applications**. Peers beat it on
agent discovery (MCP, schema, skills install), CI packaging (SARIF,
Actions), query verbs, scaffolding, and graph hygiene. They do not beat it
on evidence-strict check, multi-root git preview, retrieval benchmarks, or
a documentation-like human shell.

The productive response is not to absorb harvest, embeddings, or write MCP.
It is to expose the engine OKMate already has through the interfaces the
rest of the ecosystem standardized on: **read-only MCP, ranked search,
lint-with-IDs, SARIF, init/index, and graph verbs** — while keeping Markdown
in git as the system of record.[^landscape][^okfgem-mcp][^extract]

[^overview]: Engine versus application; settings under `~/.okmate/`.
[^readme]: Published CLI, desktop, build, roots/sync, view.
[^okf-readme]: Profiles, graph, artifacts, timings, claimed BM25.
[^search-rs]: Heading/metadata chunks; AND of lowercase substring terms.
[^cli]: Shipped subcommands; no MCP, init, lint, or SARIF.
[^okf-lib]: `check`, `inspect` catalog/concept/graph, `search`, `build`.
[^okf-tools]: Earlier four-consumer survey and closed evidence-loop synthesis.
[^landscape]: MCP as discovery path; no MCP and term-contains search as adoption blockers.
[^extract]: Review, MCP, hosted query left on a later horizon.
[^okf-app]: Agents edit Markdown; deterministic checks; no write MCP required.
[^spec]: OKF v0.2 conformance: tolerate unknown types, broken links, missing optionals.
[^google-viz]: Reference produce/enrich agent and self-contained Cytoscape HTML.
[^okfcli]: `schema`, JSON-native CLI, SARIF, validate/lint/index/list/show/search/backlinks/graph; serve/render planned.
[^useokf]: Agent-first JSON, exit-code mapping, Homebrew tap.
[^okfgem-cli]: Judge/read/serve verbs, registry `@slug`, BM25 opt-in, render/server/skill.
[^okfgem-mcp]: Read-only MCP tools, resources, prompts, bounded answers, no writes.
[^openknowledge]: Setup, validate, search, view, MCP, export, registry, CI, eval.
[^okq]: BM25, find/get/neighbors/path/orphans/deadlinks, skills install, `.okqignore`.
[^galkleinman]: Validate≠lint, MCP tools including `trust`, `serve --web`, GitHub Action, SARIF, `okft` vs crates.io `okf`.
[^workbench]: VS Code authoring loop, 3D graph, plan-then-apply writes, migrate, managed index regions.
[^akdira]: Python init/new/validate/index/search/graph/stats.
[^okf4net]: .NET library, AOT CLI, MCP read/write, catalog, winget.
[^hdean]: Hybrid BM25+vector, write MCP, local embeddings.
[^okf-schema]: `_schema/` JSON Schema, okfkb query DSL; unknown types not tolerated.
[^w4g1]: Full Rust CLI, `lint --fix`, studio TUI, semantic mv/rm/split/merge, trust, Mermaid graph.
[^crates-okf]: crates.io package `okf` is W4G1's toolkit.
[^okftool]: 28 lint rules, profiles, wasm, SARIF, tar.gz pack.
[^abcubed3]: README: harvest, assemble with budget, MCP, LSP, HTML portal, sync.
[^iwe-okf]: `iwe init --okf`, schema validate, frontmatter find; LSP/MCP product.
[^harness]: Ingest/evidence/check/graph workspace for LLM wikis.
[^xsavik]: Connector skills exposed through a generic `okf-mcp` host.
