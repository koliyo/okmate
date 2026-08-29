---
type: Research Report
title: OKMate versus serradura/okf
description: Pairwise comparison of two OKF v0.2 consumers, plus which okf-gem architecture contracts OKMate should adopt (three lenses, skeleton-first retrieval, skill as judgment) and which surfaces it should not copy.
tags: [domain/okmate, domain/okf, concern/architecture, concern/agents, concern/review, concern/tooling]
status: draft
generated: { by: process:cursor, at: 2026-08-29T08:55:00Z }
stale_after: 2026-11-29
authority: exploratory
owners: [human:nils]
sources:
  - id: overview
    resource: ../../architecture/system-overview.md
    title: OKMate system overview
    author: process:cursor
    last_modified: 2026-08-27
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
  - id: validate-rs
    resource: ../../../okf/src/validate.rs
    title: Base and strict profile validation
    author: process:git
    last_modified: 2026-08-28
  - id: ast-rs
    resource: ../../../okf/src/ast.rs
    title: Profile and trust-tier types
    author: process:git
    last_modified: 2026-08-28
  - id: cli
    resource: ../../../src/cli.rs
    title: okmate CLI surface
    author: process:git
    last_modified: 2026-08-28
  - id: review-http
    resource: ../../../src/http/review.rs
    title: Live Verify and Promote writes
    author: process:git
    last_modified: 2026-08-28
  - id: skill
    resource: ../../../.agents/skills/manage-okmate-knowledge/SKILL.md
    title: In-repo manage-okmate-knowledge skill
    author: process:git
    last_modified: 2026-08-28
  - id: cargo
    resource: ../../../Cargo.toml
    title: Workspace version and crate names
    author: process:git
    last_modified: 2026-08-28
  - id: license
    resource: ../../../LICENSE
    title: Apache-2.0 license
    author: process:git
    last_modified: 2026-08-26
  - id: gaps
    resource: okf-tool-gaps.md
    title: OKMate feature gaps versus the OKF tool ecosystem
    author: process:cursor
    last_modified: 2026-08-28
  - id: landscape
    resource: agent-dev-landscape.md
    title: OKMate and the agent knowledge-management landscape
    author: process:cursor
    last_modified: 2026-08-28
  - id: extended
    resource: extended-multi-bundle.md
    title: Extended multi-bundle viewer
    author: process:cursor
    last_modified: 2026-08-27
  - id: website
    resource: website.md
    title: OKMate product website
    author: process:cursor
    last_modified: 2026-08-28
  - id: workflows
    resource: ../okf/okf-tools-and-workflows.md
    title: State-of-the-art OKF tools and workflows
    author: process:codex
    last_modified: 2026-08-28
  - id: spec
    resource: https://github.com/GoogleCloudPlatform/open-knowledge-format/blob/main/SPEC.md
    title: Open Knowledge Format v0.2 specification
    author: organization:google-cloud
  - id: serradura-readme
    resource: https://github.com/serradura/okf/blob/7b23571f0a3d5f5021d2e981bb4699f59b8de008/README.md
    title: serradura/okf repository README
    author: human:rodrigo-serradura
  - id: serradura-gem
    resource: https://github.com/serradura/okf/blob/7b23571f0a3d5f5021d2e981bb4699f59b8de008/gems/okf/README.md
    title: okf gem README
    author: human:rodrigo-serradura
  - id: serradura-mcp
    resource: https://github.com/serradura/okf/blob/7b23571f0a3d5f5021d2e981bb4699f59b8de008/gems/okf-mcp/README.md
    title: okf-mcp gem README
    author: human:rodrigo-serradura
  - id: serradura-tui
    resource: https://github.com/serradura/okf/blob/7b23571f0a3d5f5021d2e981bb4699f59b8de008/gems/okf-tui/README.md
    title: okf-tui gem README
    author: human:rodrigo-serradura
  - id: serradura-pro
    resource: https://github.com/serradura/okf/blob/7b23571f0a3d5f5021d2e981bb4699f59b8de008/gems/okf-pro/README.md
    title: okf-pro gem README
    author: human:rodrigo-serradura
  - id: serradura-registry
    resource: https://github.com/serradura/okf/blob/7b23571f0a3d5f5021d2e981bb4699f59b8de008/.okf.json
    title: Project-local bundle registry
    author: human:rodrigo-serradura
  - id: serradura-gemspec
    resource: https://github.com/serradura/okf/blob/7b23571f0a3d5f5021d2e981bb4699f59b8de008/gems/okf/okf.gemspec
    title: okf gemspec
    author: human:rodrigo-serradura
  - id: serradura-version
    resource: https://github.com/serradura/okf/blob/7b23571f0a3d5f5021d2e981bb4699f59b8de008/gems/okf/lib/okf/version.rb
    title: okf gem version constant
    author: human:rodrigo-serradura
  - id: serradura-repo
    resource: https://github.com/serradura/okf
    title: serradura/okf GitHub repository
    author: human:rodrigo-serradura
  - id: okmate-repo
    resource: https://github.com/koliyo/okmate
    title: koliyo/okmate GitHub repository
    author: human:nils
  - id: okf-lib
    resource: ../../../okf/src/lib.rs
    title: Portable engine load, check, inspect, search, build
    author: process:git
    last_modified: 2026-08-28
  - id: gem-core-shell
    resource: https://github.com/serradura/okf/blob/26ce927090dbbe33c58d60453f30c217271596e1/gems/okf/.okf/design/core-shell-split.md
    title: okf-gem core/shell split
    author: human:rodrigo-serradura
    last_modified: 2026-08-28
  - id: gem-search-engines
    resource: https://github.com/serradura/okf/blob/26ce927090dbbe33c58d60453f30c217271596e1/gems/okf/.okf/design/search-engines.md
    title: okf-gem search facade and engines
    author: human:rodrigo-serradura
    last_modified: 2026-08-28
  - id: gem-validator
    resource: https://github.com/serradura/okf/blob/26ce927090dbbe33c58d60453f30c217271596e1/gems/okf/.okf/capabilities/validator.md
    title: okf-gem §11 validator
    author: human:rodrigo-serradura
    last_modified: 2026-08-28
  - id: gem-linter
    resource: https://github.com/serradura/okf/blob/26ce927090dbbe33c58d60453f30c217271596e1/gems/okf/.okf/capabilities/linter.md
    title: okf-gem advisory linter
    author: human:rodrigo-serradura
    last_modified: 2026-08-28
  - id: gem-read-views
    resource: https://github.com/serradura/okf/blob/26ce927090dbbe33c58d60453f30c217271596e1/gems/okf/.okf/capabilities/read-views.md
    title: okf-gem read views
    author: human:rodrigo-serradura
    last_modified: 2026-08-28
  - id: gem-skeleton
    resource: https://github.com/serradura/okf/blob/26ce927090dbbe33c58d60453f30c217271596e1/gems/okf/.okf/model/skeleton.md
    title: okf-gem directory skeleton
    author: human:rodrigo-serradura
    last_modified: 2026-08-28
  - id: gem-overview
    resource: https://github.com/serradura/okf/blob/26ce927090dbbe33c58d60453f30c217271596e1/gems/okf/.okf/overview.md
    title: okf gem overview
    author: human:rodrigo-serradura
    last_modified: 2026-08-13
  - id: gem-skill
    resource: https://github.com/serradura/okf/blob/26ce927090dbbe33c58d60453f30c217271596e1/skills/okf/SKILL.md
    title: okf-gem installable skill
    author: human:rodrigo-serradura
    last_modified: 2026-08-28
  - id: gem-authoring
    resource: https://github.com/serradura/okf/blob/26ce927090dbbe33c58d60453f30c217271596e1/skills/okf/reference/authoring.md
    title: okf-gem authoring craft
    author: human:rodrigo-serradura
    last_modified: 2026-08-28
  - id: gem-extension
    resource: https://github.com/serradura/okf/blob/26ce927090dbbe33c58d60453f30c217271596e1/.okf/design/extension-points.md
    title: okf-gem plugin discovery
    author: human:rodrigo-serradura
    last_modified: 2026-08-28
  - id: gem-where
    resource: https://github.com/serradura/okf/blob/26ce927090dbbe33c58d60453f30c217271596e1/.okf/design/where-knowledge-lives.md
    title: README, AGENTS.md, and bundle as three readers
    author: human:rodrigo-serradura
    last_modified: 2026-08-28
  - id: gem-agents
    resource: https://github.com/serradura/okf/blob/26ce927090dbbe33c58d60453f30c217271596e1/AGENTS.md
    title: okf-gem repository agent guide
    author: human:rodrigo-serradura
    last_modified: 2026-08-28
---

# OKMate versus serradura/okf

## Scope

This is a pairwise comparison of [koliyo/okmate](https://github.com/koliyo/okmate)
and [serradura/okf](https://github.com/serradura/okf). Both consume Open
Knowledge Format v0.2. Neither is the Google Cloud specification or Knowledge
Catalog.[^spec][^readme][^serradura-readme]

The first pass (2026-08-28) compared published surfaces. A second pass
(2026-08-29) read the local okf-gem checkout at `26ce927` (`OKF::VERSION`
still `2.2.0`) — its enforced core/shell split, search facade, §11
validator, advisory linter, read views, directory skeleton, and
installable skill — to separate **architecture contracts** OKMate should
adopt from product surfaces it should not
copy.[^gem-core-shell][^gem-skill][^gem-read-views][^serradura-version]
How the two projects *store* knowledge — topology, types, linking,
indexes — is [Knowledge-bundle modelling in OKMate versus
okf-gem](bundle-modelling.md).

serradura/okf is the Ruby toolkit published as the `okf` gem (v2.2.0 at
`7b23571` and still at `26ce927`), with sibling gems `okf-mcp`,
`okf-tui`, and `okf-pro`, a Docker image, and a Claude Code plugin. The
product site is [okfgem.com](https://okfgem.com). This record calls that
stack **okf-gem** so the format, the unpublished Rust crate in this
repository, and the Ruby binary are not collapsed into one
word.[^serradura-gem][^serradura-version][^serradura-readme]

OKMate is crate `0.2.5`: a portable `okf` engine plus the `okmate` binary
(Askama HTML, Axum, Datastar, optional tao/wry desktop). Settings live under
`~/.okmate/`.[^cargo][^overview][^readme]

A field-wide survey of other OKF CLIs lives in [OKMate feature gaps versus
the OKF tool ecosystem](okf-tool-gaps.md). This record goes deeper on one
peer because okf-gem is the closest complete product to OKMate: same format,
same "files in git, no hosted wiki" thesis, opposite primary UI and agent
surface.[^gaps][^workflows]

GitHub snapshot on this date: serradura/okf is public Ruby, created
2026-07-11, 139 stars, 10 forks, 392 commits, 2 open issues. koliyo/okmate
is public Rust, created 2026-08-26, 1 star, 0 forks. Both are
Apache-2.0.[^serradura-repo][^okmate-repo][^license][^serradura-gemspec]

## Shared contract

Both treat a bundle as a directory of Markdown files with YAML frontmatter.
One concept is one file. Ordinary Markdown links are the graph. `index.md`
is the progressive-disclosure map. `log.md` is dated change history. Unknown
types and extra frontmatter keys must be tolerated at base conformance.
Provenance (`generated`, `verified`), lifecycle (`status`, `stale_after`),
and keyed `sources` are the v0.2 trust families.[^spec][^serradura-readme][^okf-readme]

Both say the knowledge stays readable if the tool is uninstalled. Both refuse
a hosted knowledge service. Both ship their own bundle beside the code
(`knowledge/` here; `.okf/` in each okf-gem tree).[^readme][^serradura-readme]

The hard frontmatter floor is the same: a non-empty `type`. Everything else
is optional at base / `validate`. Organizational policy is where they
diverge.[^spec][^validate-rs][^serradura-gem]

## Product thesis

okf-gem's pitch is that agent reasoning dies in four places — never written,
rots, cannot be found, stuck in one tool — and one install answers all four:
a skill so knowledge gets written, `validate`/`lint` so drift fails CI,
ranked `search`/`index` so the corpus outgrows a context window, and a graph
plus MCP so the next host does not start over.[^serradura-readme]

OKMate's pitch is a **review application** over a UI-neutral engine. Humans
read article HTML (live preview or `build`). Agents get deterministic
`check` / `inspect` / `search` / `roots`. Canonical records stay inert
Markdown that a human diffs. The tool judges; it does not own the
files.[^overview][^readme][^website]

Those theses overlap on format and CI. They disagree on the default human
surface (graph vs review shell), on how policy is encoded (`lint` vs
`check --profile strict`), and on how an agent first meets the product
(installable skill + MCP vs in-repo skill + CLI).[^gaps][^serradura-gem]

## Architecture

| Layer | OKMate | okf-gem |
| --- | --- | --- |
| Engine | Unpublished Rust crate `okf/` — parse, validate, graph, search, artifacts. No HTML or HTTP. | Ruby `OKF::Bundle` — the only thing that touches disk. Pure in-memory objects plus on-disk `Folder` handles. |
| Application | `okmate` crate: CLI, Askama site, Axum + Datastar preview, optional desktop. | Same `okf` binary: CLI, Rack graph server, `okf render`. |
| Extensions | None. Desktop is a Cargo feature. | Plugin seam: a gem named `okf-*` with `okf/plugin.rb` adds a verb. MCP, TUI, and Pro arrive this way. |
| Config | `~/.okmate/config.toml` (roots, actor). Cache and state beside it. | `$OKF_HOME` default `~/.okf` (`registry.json`). Project-local `.okf.json` wins by walking up. |
| Language floor | Rust edition 2024, compiled binary. | Ruby >= 2.4, no native extension; `rack`, `webrick`, `minifts`. |

The engine/application split is shared. OKMate makes it a crate boundary.
okf-gem makes it a gem/plugin boundary and keeps one binary name.[^okf-readme][^overview][^serradura-gem][^serradura-readme]

okf-gem also **enforces** the split: `boundary_test.rb` fails if a core
file names a shell class or reaches for `File` / `Dir` / stdio. `OKF::Bundle`
is an in-memory object; `catalog`, `directory_index`, `hubs`, `stats`,
`tag_groups`, and `skeleton` are derived once and shared by CLI, server,
and MCP so `--dir` cannot mean two different sets. Search is a facade that
owns the result row; engines only retrieve. The default engine is a
raw-text scan because a one-shot CLI cannot amortize an index, and the
tokenizer would drop hits the scan finds.[^gem-overview][^gem-core-shell][^gem-read-views][^gem-search-engines][^gem-skeleton]

OKMate's `okf/` crate already refuses HTML and HTTP. `load` still takes a
`Path` and does I/O; inspect/search/build do not share a projected catalog
the way the review HTML does. That is a crate boundary, not the same
purity-and-projection contract.[^okf-lib][^okf-readme][^overview]

okf-gem's library is also a mountable Rack app (`OKF::Server::App` /
`Hub`), so a Rails host can serve the graph behind its own auth. OKMate's
HTTP is a localhost preview (loopback settings and review POSTs; `--public`
binds every interface). That is a different embed story.[^serradura-gem][^readme]

## Surfaces

| Surface | OKMate | okf-gem |
| --- | --- | --- |
| Validate / check | `okmate check` — `base` or `strict` (default strict) | `okf validate` (§11) and `okf lint` / `loose` (curation) |
| Inspect / catalog | `inspect catalog\|concept\|graph` JSON | `catalog`, `files`, `tags`, `types`, `stats`, `dirs`, `index`, `graph`, `references` |
| Search | `okmate search` — AND substring over metadata and heading chunks | `okf search` — ranked; optional BM25+fuzzy index; `@all` |
| Build / publish | `okmate build` — `catalog.json`, `search.json`, `validation.json`, `llms.txt`, Askama HTML | `okf render` — one self-contained graph HTML |
| Live UI | `okmate view` — review shell; desktop window by default | `okf server` — force-directed graph; hub over the registry |
| Multi-root | `roots` / `sync`; check/inspect/search/build stay single-root | `@slug`, groups, `@all`; hub and TUI span the set |
| Agent skill | In-repo `manage-okmate-knowledge`; no installer | `okf skill .claude\|.agents`; Claude plugin; `npx skills add` |
| MCP | None | `okf mcp` — 14 read tools, resources, two prompts |
| TUI | None | `okf tui` — six views |
| Personal process | Live Verify / Promote on the review page | `okf pro` — board, journal, three fail-closed doors |
| Eval | `okmate benchmark` (hit rate / MRR), `okmate timings` | None documented |
| Distribution | GitHub Releases, Homebrew cask, Sparkle `.app` | RubyGems, Docker / `docker.okfgem.com`, Claude marketplace |

[^cli][^readme][^serradura-gem][^serradura-mcp][^serradura-tui][^serradura-pro]

## Validation and policy

OKF §11 forbids rejecting a bundle for broken links, unknown types, or
missing optional fields. okf-gem implements that literally: `validate` is
the hard gate (exit 1 only when non-conformant; broken links are warnings).
`lint` asks whether the bundle is navigable, complete, and fresh
(reachability, backlog, provenance, attestation, migration, hygiene) and
exits 0 unless `--fail-on warn`. `loose` is the unlinked-file lens. A v0.1
bundle stays readable; two migration findings name leftover spellings and
do not fail the build.[^spec][^serradura-gem][^serradura-readme]

OKMate folds policy into **profiles**. `base` is the portable floor. `strict`
(CLI default) requires `title`, `description`, `status`, `generated`,
`authority`, non-empty `owners`, and tags with a `domain/*` value; unknown
`type` is a warning, not a rejection; unknown tag prefixes and bad
`authority` are errors. Stable diagnostic codes (`OKF1xxx` parse,
`OKF2xxx` metadata, `OKF3xxx` graph, `OKF4xxx` provenance). Git provenance
(`OKF4006`/`4007`/`4008`) is part of strict load and can be turned off for
preview.[^validate-rs][^cli][^okf-readme][^readme]

Strict is a coherent organizational profile. What it is not: a disableable
hygiene layer with `--deny` / `--allow`, a SARIF emitter, or a split that
lets CI gate §11 without gating taste. okf-gem's `--only` / `--fail-on` /
`--today` is the clearer CI story for a migration campaign. OKMate's
footnote-to-`sources` alignment and owner requirements are the stricter
evidence story; few peers fail a build on that drift.[^gaps][^validate-rs][^serradura-gem]

Trust-tier vocabularies differ. OKMate derives
`human-reviewed | generated | unverified`. okf-gem filters
`unverified | machine-confirmed | human-reviewed`. Both read the same
`generated` / `verified` families; the derived labels are not
interchangeable in CLI filters.[^ast-rs][^serradura-mcp]

OKMate also treats `authority` (`normative | descriptive | exploratory |
historical`) as a first-class strict field and refuses to Promote an
exploratory record. okf-gem does not use that vocabulary.[^validate-rs][^review-http]

## Search and progressive disclosure

okf-gem search is designed for agents that must not load the bundle. Default
is an exact raw-text scan. `engine: "index"` (minifts, the same engine as
the graph page) opts into BM25+ ranking; `fuzzy` implies the index;
`regexp` stays on the scan. `--fields` / `--except` project rows. `index`
and `dirs` take `--dir` and `--depth` so a 400-concept map can shrink from
hundreds of KB to a few KB. `@all` ranks across every registered
bundle.[^serradura-gem][^serradura-mcp]

OKMate search ANDs lowercase substring terms over title, description,
heading, chunk text, and tags. Chunks are metadata plus heading sections,
not a tokenized corpus. Filters exist (`type`, `status`, `authority`,
`tag`, trust tier, stale). There is no rank score, no fuzzy engine, no
`--fields` projection, and no multi-root query. The engine README still
says "BM25/lexical matching"; `search.rs` does not implement
BM25.[^search-rs][^okf-readme][^gaps]

`okmate inspect catalog|concept|graph` dumps normalized JSON. Peers expose
verbs agents actually call (`backlinks`, `orphans`, `neighbors`, bounded
`read_concept`). The graph data is already in the engine; the CLI does not
cut it into those verbs.[^cli][^gaps]

`okmate benchmark` is unique in this pair: a TOML of questions scored as
hit rate and MRR. okf-gem has no published retrieval eval. That is the
right place to measure a future BM25 change rather than adding embeddings
by default.[^okf-readme][^gaps]

## Multi-bundle

okf-gem treats many bundles as the normal case. A registry names them
`@slug`. Groups nest. `@all` is a search-only union. A project-local
`.okf.json` replaces `~/.okf` while you stand in the repo — this monorepo
addresses `@okf-eco`, `@okf`, `@okf-mcp`, `@okf-tui`, `@okf-pro` that
way. `registry link` composes another file read-only. The graph hub mounts
each bundle at `/b/<slug>/` and the command palette searches every one.
The TUI splits **active** bundle (browse/graph/health) from **scope**
(search).[^serradura-gem][^serradura-registry][^serradura-tui][^serradura-readme]

OKMate already lists many roots (`okmate roots` / `sync`, directory plus
git-fetch snapshots). `check`, `inspect`, `search`, and `build` stay
**single-root**. Agents list paths, then operate on one folder. Concept IDs
are not a global namespace. The Askama viewer still loads one tree; a
merged dashboard, collection hover, and merged log are planned application
work, not an engine merge. Git-backed roots plus a desktop folder picker
are rare among OKF tools; `@all` ranked search is what okf-gem has and
OKMate does not.[^readme][^overview][^extended][^gaps]

Do not collapse these into one registry design. okf-gem's `@slug` is a
local address book over directories you already have. OKMate's git roots
are fetched snapshots with revision metadata. A useful synthesis is
**addressing** (slugs, groups) on top of **acquisition** (sync), not a
merged company brain.[^website][^extended][^serradura-gem]

## Human interface

okf-gem's primary UI is a force-directed graph: live `okf server`, static
`okf render`, keyboard palette, cluster-by-directory, inspector panel,
trust tier as a visual channel. Bodies are sanitized before they reach the
DOM; the page still loads libraries from a CDN. The TUI is the terminal
counterpart: browse in reading order, follow links with `f`, health as
validate+lint+hubs.[^serradura-gem][^serradura-readme][^serradura-tui]

OKMate's primary UI is a documentation-like review shell: concept pages,
collections, review queue, log, settings, live reload, optional desktop
window with Sparkle updates. Static `okmate build` HTML is the same article
tree without Verify/Promote. That is uncommon among OKF tools; most peers
ship Cytoscape-style viz or a TUI.[^readme][^gaps][^website]

The complementary gap is honest on both sides. OKMate still lacks a
self-contained graph HTML or Mermaid dump for orientation and for sharing
a bundle without running the app. okf-gem still lacks an article/review
queue, outline, and human Verify/Promote on the page. Copying the other's
primary shell would be a second product, not a feature.[^gaps][^readme][^serradura-gem]

## Agent loop

okf-gem ships the authoring brain with the gem. `okf skill` writes
`SKILL.md` plus reference and templates into `.claude/skills/okf` or
`.agents/skills/okf`. Verbs: orient, `search`, `produce`, `migrate`,
`maintain`, `refine` (proposes, never applies), `consume`, `curate`
(validate+lint+loose), `doctor`, and any CLI verb. Claude Code adds
`/okf:gem` and a post-edit hook that runs `validate` + `lint` inside a
bundle (`OKF_CURATE_DISABLED=1` to silence). `npx skills add serradura/okf`
installs a generated copy without Ruby.[^serradura-gem][^serradura-readme]

`okf-mcp` is read-only on purpose: 14 tools (`list_bundles`, `dirs`,
`index`, `search`, `read_concept`, `catalog`, `log`, `validate`, `lint`,
`graph`, `references`, `tags`, `types`, `stats`), `okf://` resources,
prompts `okf-search` and `okf-consume`. Authoring stay with the skill and
the filesystem. Answers are bounded and dual-published as text plus
`structuredContent`. No authentication on `--http`.[^serradura-mcp]

OKMate's shipped agent contract is the CLI plus an in-repo skill that
teaches retrieve / author / `okmate check --profile strict`. There is no
`okmate skill` installer, no MCP, no produce/migrate playbook, no post-edit
hook. Agents already edit Markdown in git; the skill forbids treating the
tool as a write API.[^skill][^cli][^gaps]

okf-gem's MCP design is the closest fit if OKMate adds one: read-only,
result caps, resources plus tools, no writes. The skill-installer gap is
the other adoption cost: other repos cannot get the retrieve/author/check
loop without copying this tree.[^gaps][^serradura-mcp][^landscape]

## Authoring and trust

Both keep the agent from silently attesting. okf-pro holds any write that
carries `verified:` and routes it to the human; approval is the
attestation. Absent `verified:` on a generated concept is a true state,
not a defect. The rest of Pro is a PARA-shaped personal repository
(`reference/`, `learnings/`, `glossary/`, `projects/`, `areas/`) with a
five-item in-flight cap, a daily log snapshot, and three doors (Claude
hooks, pre-commit, CI) that fail closed. It is one owner, its own repo,
not `.okf/` inside an application. It is not a paid
tier.[^serradura-pro]

OKMate's live review writes Verify (append a `human:` event, leave
`status`) and Promote (`draft` → `stable` only after a human event,
never for `authority: exploratory`). Loopback only, git working tree
only, actor from Settings. The working-tree file changes; the human
commits. Static build HTML does not include these
controls.[^readme][^review-http]

These are cousins, not substitutes. Pro enforces a personal working
practice at the agent/commit/CI boundary. Verify/Promote is a maintainer
control on a project knowledge bundle. OKMate should not absorb Pro's
board or journal. okf-gem should not be treated as already having a
documentation review queue.[^serradura-pro][^readme]

okf-gem's skill will **produce** and **migrate** docs into a bundle.
OKMate has no `init` / `new` / index regenerator. Scaffolding is a
documented gap versus almost every OKF CLI; Workbench's plan-then-`--apply`
is the safety model that fits a tool that otherwise refuses to own the
files.[^gaps][^serradura-gem]

## Distribution and naming

okf-gem optimizes for "already have Ruby or Docker": `gem install okf`,
`ghcr.io/serradura/okf`, or a curl installer that wraps Docker so the
command still reads `okf`. The Claude marketplace is a two-command
install. The `okf` binary name is the product.[^serradura-gem][^serradura-readme]

OKMate optimizes for a compiled desktop: signed `OKMate.app`, Homebrew
cask, Sparkle, CLI symlink from the same zip. `cargo install` does not
self-update. The binary is `okmate`. The engine crate is also named `okf`,
unpublished; `cargo install okf` installs W4G1's toolkit, not this
engine.[^readme][^gaps][^cargo]

Both are Apache-2.0. okf-gem bundles Google's SPEC under Google's
Apache-2.0 notice. OKMate implements the format without shipping that
file in the skill.[^license][^serradura-readme][^skill]

Community scale is not a quality metric, but it is an adoption fact:
okf-gem is already the named Ruby implementation in the August 2026
survey; OKMate is two days old as a public repo and is still the more
complete **review** consumer.[^serradura-repo][^okmate-repo][^gaps]

## What each is stronger at

**okf-gem, today**

- Agent discovery: installable skill, Claude plugin, read-only MCP, Docker.
- Spec-faithful `validate` plus advisory `lint` with `--fail-on` / `--only` / `--today`.
- Ranked search, progressive `index`/`dirs`, `@all`, `--fields`.
- Graph as a shareable artifact (`render`) and a hub over many bundles.
- TUI and a documented plugin seam.
- Produce / migrate / maintain playbooks that write ordinary Markdown.

**OKMate, today**

- Evidence-strict CI: owners, domain tags, authority, source/footnote alignment, git provenance codes.
- Human review shell: article HTML, collections, review queue, desktop window.
- Verify / Promote as explicit maintainer writes, not agent attestation.
- Multi-root **git fetch** plus a folder picker, with single-root query kept honest.
- Retrieval benchmark and load/parse-cache timings.
- `llms.txt` and a full static review site from `build`.

**Neither should copy blindly**

The architecture cut below is the durable list. In short: no write MCP,
no unplanned `--fix`, no embeddings as default search, no Pro board as a
team model, no crates.io or RubyGems name `okf`.[^gaps][^serradura-mcp][^serradura-pro][^validate-rs][^serradura-gemspec]

## Architecture contracts to adopt

These are design contracts, not a product checklist. Adopt the idea; do
not port the Ruby, the graph shell, or the gem layout. Feature-level gaps
versus the wider field stay in [OKMate feature gaps versus the OKF tool
ecosystem](okf-tool-gaps.md).[^gaps]

### Three lenses: legal, good, true

okf-gem's skill names the load-bearing mistake as conflating three
questions:[^gem-skill][^gem-validator][^gem-linter]

| Lens | Question | Who answers |
| --- | --- | --- |
| Legal | Is it OKF §11? | `validate` — binary, tolerant |
| Good | Is it navigable, complete, fresh? | `lint` — advisory, structural |
| True | Is it consistent with reality? | The agent — semantic |

OKMate folds legal and good into `check --profile`. Strict is a coherent
**organizational** profile (owners, authority, footnote alignment). It is
not a substitute for the split. Growing Strict until it fails
§11-tolerated conditions (orphans, unknown types, broken links)
contradicts the spec.[^spec][^validate-rs][^gem-validator]

The linter never reads a wall clock: the caller injects `today:`, and
`--today` makes CI reproducible. Author `stale_after` (`expired`) is a
different mechanism from a reader age cutoff (`stale`). Severity is API:
`--fail-on warn` / `--only` / `--except` by check id, never a silent
severity promotion.[^gem-linter]

The third lens is the one OKMate's skill already practices but does not
name: `okmate check` cannot detect a record that parses and still no
longer matches the code.[^skill]

### Skeleton-first retrieval

This is the strongest architectural idea in the checkout, and the one
OKMate is furthest from.

The rule is explicit: `dirs` → `index --dir --depth` → ranked `search` →
bodies last. `--fields` / `--except` project rows so an agent never pays
for a dump. `index` exists because **grep cannot find a missing
listing**. `graph --minimal` and a directory **skeleton** (cohesion
versus coupling at directory grain) are the same idea one level
up.[^gem-skill][^gem-read-views][^gem-skeleton][^gem-authoring]

OKMate's contract is the opposite posture: `inspect catalog|concept|graph`
emits the whole normalized object. Agents grep or load files. The engine
already has the graph; it does not cut it into cheap orientation
verbs.[^cli][^okf-lib][^gaps]

Ranked search matters, but **progressive disclosure is the deeper
contract**. BM25 without `dirs` / `index --depth` still forces a dump.
okf-gem is also honest about search architecture: the default is a
raw-text **scan** (no tokenizer holes; a one-shot CLI cannot amortize an
index). BM25 is an opt-in engine behind a facade that **owns the result
row**. That is the right shape if OKMate adds ranking. The engine README
claiming BM25 while `search.rs` ANDs substrings is the opposite of that
honesty.[^gem-search-engines][^search-rs][^okf-readme]

`okmate benchmark` is the right place to measure a future ranked engine.
okf-gem has no published retrieval eval.[^okf-readme][^gaps]

### Skill is judgment; the binary is mechanics

okf-gem does not treat the CLI as the authoring brain. Playbooks
(`produce`, `maintain`, `consume`, `curate`, `refine`) own craft.
`refine` proposes and never applies. `curate` is structural only; content
drift is `maintain`; shape that underserves retrieval is `refine`.
Closeout is a finishing gate (indexes, log, `generated.at`, validate,
lint). `AGENTS.md` routes; the bundle argues; a test fails when `lib/`
has no concept.[^gem-skill][^gem-authoring][^gem-agents][^gem-where]

OKMate already has the right **posture** (agents edit Markdown; the tool
judges; no write API). What it lacks is a **portable** skill: installable
playbooks plus `okmate skill .agents`, not only
`manage-okmate-knowledge` in this tree. Do not copy produce/migrate
wholesale. Copy the division of labour and the install
path.[^skill][^gaps]

### Read-only MCP as a projection of the kernel

`okf-mcp` is not a second product. It is the same read views as MCP
tools, `okf://` resources, and two prompts that restate retrieval
doctrine. Bounds, dual text plus `structuredContent`, no writes.
Authoring stays with the skill and the filesystem. That is the correct
seam for OKMate. Write MCP would fight both products.[^serradura-mcp][^gaps][^landscape]

### Addressing versus acquisition

okf-gem's `@slug` / groups / project-local `.okf.json` is an **address
book** over directories you already have. OKMate's `roots` / `sync` is
**acquisition** (git-fetch snapshots, revision metadata). Those should
not be collapsed. The sound synthesis is slugs and groups **on top of**
sync. `@all` ranked search is a later choice. Merged concept IDs are
not.[^serradura-registry][^extended][^website]

### Already have a cousin

| okf-gem | OKMate | Action |
| --- | --- | --- |
| Core/shell, `Bundle` never touches disk | `okf/` vs `okmate` crate | Keep the crate boundary. Do not ban `Path` in Rust the way they ban `File` in Ruby.[^gem-core-shell][^okf-lib] |
| Emergent graph from Markdown links | Same | Keep. |
| Human attestation hold (`okf-pro`) | Verify / Promote on the review page | Cousins. Do not absorb Pro's PARA board or journal.[^serradura-pro][^review-http] |
| Best-effort unparseable collection | Diagnostics on load | Keep collecting; do not abort the graph on one bad file. |
| README / `AGENTS.md` / bundle as three readers | Same split in this repo | Keep routing, not restating.[^gem-where] |

Their **code-to-bundle pin** (a test fails when `lib/` has no concept) is
a process idea worth stealing for OKMate's own `knowledge/`, not a
user-facing feature.[^gem-agents]

### Do not adopt

- **Graph as the primary human shell.** A self-contained graph HTML or
  Mermaid dump as a *secondary* `build` artifact is the useful
  fragment.[^serradura-gem][^gaps]
- **Plugin discovery (`okf-*` gems).** That seam exists because they are
  a RubyGems ecosystem. A compiled binary with Cargo features is the
  right OKMate equivalent.[^gem-extension]
- **Frontmatter `id:` override.** They document it as a portability
  trade: identity views follow `id`, physical views follow path. OKMate
  should keep path-as-id.[^gem-authoring]
- **Write MCP, `lint --fix` that edits without a plan, embeddings as
  default search.** Both theses refuse these.[^gaps][^serradura-mcp]
- **Taking the name `okf`.** The gem is theirs; crates.io `okf` is
  W4G1's; this engine is unpublished.[^gaps][^serradura-gemspec]

## Implications for OKMate

These are research implications, not an implementation plan. Ranked by
architectural leverage, not by flash.

1. **Skeleton-first read views** (`dirs`, depth-bounded `index`, field
   projection, bounded concept read) on the engine OKMate already has.
   Progressive disclosure is the contract; ranked search is a later
   engine behind the same facade.[^gem-read-views][^cli][^okf-lib]
2. **Legal versus hygiene split** (or a disableable lint layer) so
   `check --profile base` stays §11-faithful. Inject `--today` for
   freshness. Do not grow Strict until it contradicts the spec's
   consumer rules.[^spec][^validate-rs][^gem-linter]
3. **Read-only MCP** as a projection of those views: caps, resources, no
   writes. Follow okf-mcp, not a second API.[^serradura-mcp][^gaps]
4. **Installable skill** (`okmate skill .agents`) that teaches the
   retrieve / author / check loop. Produce/migrate can stay out of
   scope.[^skill][^gem-skill]
5. **Ranked lexical search** behind a facade, defaulting to a honest
   scan, measured with `okmate benchmark`. Do not add embeddings by
   default.[^gem-search-engines][^search-rs][^okf-readme]
6. **Slug addressing** on top of `roots` / `sync`. `@all` search is a
   later choice; merged concept IDs are not.[^extended][^website]
7. **Graph HTML as a secondary `build` artifact** complements the review
   shell. Do not replace it.[^gaps][^serradura-gem]

okf-gem is the better **agent-and-orientation** architecture. OKMate is
the better **evidence-review** architecture. The sound move is to take
their retrieval doctrine and leave their primary UI, plugin ecosystem,
and personal-process layer alone.[^gaps][^landscape][^website]

[^overview]: Engine versus application; `~/.okmate/`; single-root check/inspect/search/build.
[^readme]: Published CLI, desktop, build, roots/sync, view, Verify/Promote, Homebrew/Sparkle.
[^okf-readme]: Profiles, graph, artifacts, timings, claimed BM25.
[^search-rs]: Heading/metadata chunks; AND of lowercase substring terms.
[^validate-rs]: Strict requires title, description, status, generated, authority, owners, domain tags.
[^ast-rs]: `Profile::{Base,Strict}`; trust tiers human-reviewed, generated, unverified.
[^cli]: Shipped subcommands; default profile strict; no MCP, lint, or skill install.
[^review-http]: Verify appends `human:`; Promote requires draft plus human event; exploratory refused.
[^skill]: Retrieve/author/check against this bundle; inert Markdown; no write API.
[^cargo]: Workspace version 0.2.5; crate names `okmate` and path `okf`.
[^license]: Apache License 2.0.
[^gaps]: Field survey; MCP/lint/BM25/init gaps; review-shell and strict-evidence strengths.
[^landscape]: MCP as discovery path; compiled reviewable knowledge layer.
[^extended]: Many roots already; viewer still one tree; do not merge IDs.
[^website]: Format-not-platform; several bundles coordinated, not merged.
[^workflows]: Earlier okf-gem note: validate/lint split and graph as orientation.
[^spec]: OKF v0.2 §11: tolerate unknown types, broken links, missing optionals.
[^serradura-readme]: Ecosystem pitch, four gems, Claude plugin, Docker, Apache-2.0, OKF v0.2.
[^serradura-gem]: CLI verbs, validate≠lint, registry, skill verbs, Rack graph, Ruby 2.4, minifts.
[^serradura-mcp]: Fourteen read tools, resources, prompts, BM25 opt-in, no writes, no auth on HTTP.
[^serradura-tui]: Six views; active vs scope; no bundle writes.
[^serradura-pro]: PARA zones, three doors, `verified:` hold, not a paid tier, one owner.
[^serradura-registry]: Five in-repo slugs: okf-eco, okf, okf-mcp, okf-tui, okf-pro.
[^serradura-gemspec]: Gem name `okf`, Apache-2.0, Rodrigo Serradura, RubyGems.
[^serradura-version]: `OKF::VERSION = "2.2.0"`.
[^serradura-repo]: Public repo metadata: stars, forks, created 2026-07-11, language Ruby.
[^okmate-repo]: Public repo metadata: created 2026-08-26, language Rust, Apache-2.0.
[^okf-lib]: Engine `load` takes a path; `check` / `inspect` / `search` / `build` are the public verbs.
[^gem-core-shell]: Pure core, I/O shell, `boundary_test.rb` fails the build on a leak.
[^gem-search-engines]: Facade owns the row; scan is default; BM25 is an opt-in engine.
[^gem-validator]: §11 three hard conditions; everything else a warning; never rejects for broken links.
[^gem-linter]: Advisory; pinned severities; injected clock; `--fail-on` / `--only`.
[^gem-read-views]: `dirs` / `index` / catalog / `--fields`; one directory set for `--dir`.
[^gem-skeleton]: Directory reduction; cohesion versus coupling; unthresholded arcs.
[^gem-overview]: Seven capabilities; dual audience; emergent graph; core/shell ethos.
[^gem-skill]: Three lenses; skeleton-first; playbooks; CLI is mechanics.
[^gem-authoring]: Modelling craft; closeout gate; `id:` override is a portability trade.
[^gem-extension]: `okf-*` plugin discovery; install is the whole installation.
[^gem-where]: README, `AGENTS.md`, and the bundle each answer one reader.
[^gem-agents]: Bundle ships with the code; `lib/` unnamed by a concept fails the suite.
