---
type: Research Report
title: OKMate product website (Rocdown, dual audience, agent entry)
description: The public OKMate site should be a static Rocdown product and docs surface for humans and agents, not the OKF review HTML; git-owned OKF bundles and multi-root coordination are the story, and agents need a first-class /llms.txt plus /agents/ setup path.
tags: [domain/okmate, domain/okf, concern/publication, concern/agents, concern/developer-experience]
status: draft
generated: { by: process:cursor, at: 2026-08-28T12:15:00Z }
stale_after: 2026-11-28
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
    title: Published OKMate stack, CLI, and install
    author: process:git
    last_modified: 2026-08-27
  - id: okf-readme
    resource: ../../../okf/README.md
    title: Portable OKF engine
    author: process:git
    last_modified: 2026-08-26
  - id: agents
    resource: ../../../AGENTS.md
    title: Okmate agent instructions
    author: process:git
    last_modified: 2026-08-28
  - id: skill
    resource: ../../../.agents/skills/manage-okmate-knowledge/SKILL.md
    title: Manage Okmate knowledge skill
    author: process:git
    last_modified: 2026-08-26
  - id: landscape
    resource: agent-dev-landscape.md
    title: OKMate and the agent knowledge-management landscape
    author: process:cursor
    last_modified: 2026-08-28
  - id: multi-roots
    resource: ../../plans/okf/multi-knowledge-roots.md
    title: Multiple knowledge roots
    author: process:cursor
    last_modified: 2026-08-25
  - id: extended
    resource: extended-multi-bundle.md
    title: Extended multi-bundle viewer research
    author: process:cursor
    last_modified: 2026-08-27
  - id: extract
    resource: ../../plans/okf/okmate.md
    title: Okmate extractable Rust OKF mate
    author: process:cursor
    last_modified: 2026-08-26
  - id: agent-plan
    resource: ../../plans/okmate/agent-knowledge.md
    title: Bootstrap okmate agents and migrate OKF knowledge
    author: process:cursor
    last_modified: 2026-08-26
  - id: artifact
    resource: ../../../okf/src/artifact.rs
    title: catalog.json, search.json, llms.txt, validation.json
    author: process:git
    last_modified: 2026-08-26
  - id: cli
    resource: ../../../src/cli.rs
    title: okmate CLI including roots and sync
    author: process:git
    last_modified: 2026-08-27
  - id: rocdown-cli
    resource: https://github.com/koliyo/rocci/blob/main/crates/rocci-rocdown-cli/README.md
    title: rocdown CLI (view, build, check; refuses OKF)
    author: process:git
    last_modified: 2026-08-26
  - id: rocdown-arch
    resource: https://github.com/koliyo/rocci/blob/main/knowledge/architecture/rocdown-documentation-compiler.md
    title: Rocdown documentation generator architecture
    author: process:cursor
    last_modified: 2026-08-26
  - id: rocdown-llms
    resource: https://github.com/koliyo/rocci/blob/main/crates/rocci-rocdown/src/plan.rs
    title: Rocdown planned /llms.txt, sitemap, robots
    author: process:git
    last_modified: 2026-08-26
  - id: rocci-site
    resource: https://github.com/koliyo/rocci/blob/main/site/rocdown.toml
    title: rocci.dev mounts and navigation
    author: process:git
    last_modified: 2026-08-24
  - id: site-lane
    resource: https://github.com/koliyo/rocci/blob/main/knowledge/plans/site/okf-viewer-site-lane.md
    title: Mount OKF viewer on rocci.dev (not a Rocdown mount)
    author: process:cursor
    last_modified: 2026-08-26
  - id: llmstxt
    resource: https://llmstxt.org
    title: The /llms.txt file, v2
    author: organization:answer-ai
  - id: okf-spec
    resource: https://github.com/GoogleCloudPlatform/open-knowledge-format
    title: Canonical OKF specification repository
    author: organization:google-cloud
  - id: plan
    resource: ../../plans/okmate/website.md
    title: OKMate product website implementation plan
    author: process:cursor
    last_modified: 2026-08-28
---

# OKMate product website

## Claim

OKMate needs a public website that sells and explains the **open-source knowledge tool**, not a second copy of the in-app review shell. Author that site in Rocdown and compile it with the `rocdown` CLI from the sibling Rocci tree. Keep the committed `knowledge/` bundle as OKF Markdown. Give visiting **agents** a first-class entry (`/llms.txt` plus `/agents/`) for install, CLI, and skills, equal in status to the human docs lane.[^readme][^overview][^rocdown-cli][^landscape][^plan]

This record is exploratory evidence for the [website plan](/plans/okmate/website.md). Writing it does not start a phase.[^plan]

## Two compilers, two jobs

| Job | Compiler | Source | Visitor |
| --- | --- | --- | --- |
| Product, docs, agent setup | `rocdown` | `site/**/*.rocdown` | Humans and agents on the public host |
| Review a knowledge bundle | `okmate build` / `okmate view` | `knowledge/**/*.md` (OKF) | Maintainers, local preview, optional later demo |

Rocdown discovers `.rocdown` pages, resolves routes and navigation, renders static article HTML, and emits `/llms.txt`, `/pages.json`, `/sitemap.xml`, and `/robots.txt`. It refuses OKF knowledge records and points at `okmate view`. The `okf` engine is UI-neutral; OKMate's Askama HTML is the review application. Those boundaries are already product decisions, not website preferences.[^rocdown-cli][^rocdown-arch][^rocdown-llms][^overview][^extract]

The rocci.dev knowledge-lane plan makes the same split: package OKF review HTML under `/knowledge/` instead of rewriting records as Rocdown. OKMate's public site should keep that split. Do not convert this repository's `knowledge/` into `.rocdown`. Do not add a `rocci-*` crate dependency to the OKMate workspace; `rocdown` is an **external documentation compiler**, like using Hugo, not a runtime of the `okmate` binary.[^site-lane][^overview][^extract]

rocci.dev is a hybrid site (CDN HTML plus a live island) because Rocci is a UI compiler. OKMate is a knowledge application. The first public OKMate site should be **static CDN-only**. That avoids a VPS and island service, matches GitHub Pages, and keeps the story on files and git rather than on a live demo counter.[^rocci-site][^rocdown-cli]

## What the site must argue

Four claims, in this order, because they are the product rather than features of the viewer chrome.[^landscape][^overview][^readme][^okf-spec]

1. **Open source tool.** Apache-2.0 repository, CLI and optional macOS app, no hosted knowledge service, no account. GitHub is the canonical tree; Homebrew and GitHub Releases are install channels. The site is a map, not a SaaS dashboard.[^readme]
2. **OKF compatible.** Bundles are inert Markdown plus YAML on the Open Knowledge Format contract (profiles, graph, inspect, artifacts). OKMate is a consumer of that format and ships the portable `okf` crate. It is not Google Cloud Knowledge Catalog and not a proprietary wiki schema.[^okf-readme][^okf-spec][^landscape]
3. **You own the data in git.** Canonical records live in directories you diff, review, and CI-check. Directory roots are writable authoring paths. Configured git roots are fetched snapshots. Settings live under `~/.okmate/`. There is no opaque memory API and no cloud wiki the vendor indexes for you.[^readme][^multi-roots][^landscape]
4. **Several bundles, coordinated, not merged.** `okmate roots` and `okmate sync` resolve many trees. `check`, `inspect`, `search`, and `build` stay **single-root**. Agents list paths, then operate on one folder at a time. Concept IDs are not a global namespace. The website must teach this loop; it must not imply a merged company brain.[^cli][^multi-roots][^extended][^skill]

OKMate is for **humans and agents sharing the same files**. Humans get the desktop/live review shell. Agents get CLI JSON, a short `AGENTS.md`, and the manage-knowledge skill. The site has to address both in the header, not bury agents under a docs appendix.[^agents][^skill][^landscape]

## Audiences and jobs

| Audience | Arrives wanting | Site job |
| --- | --- | --- |
| Agent-driven developer (human) | A compiled, reviewable wiki that survives CI | Positioning versus DeepWiki, Basic Memory, Notion MCP; install; first bundle; multi-root |
| Agent (Cursor, Claude Code, Codex, fetch) | How to install the CLI, load a skill, query bundles | `/llms.txt` → `/agents/` → install, skill, `roots` then `inspect` |
| Team lead | Ownership, git, no vendor lock-in | Format-not-platform; directory plus git roots; strict profile |
| OKF-curious | Compatibility with the Google format | What OKF is; what `okf` vs `okmate` vs a bundle is |
| Contributor | How to run tests and where knowledge lives | Project lane; point at `AGENTS.md` and README, do not fork them |

The landscape research already places OKMate as a **compiled, reviewable knowledge layer**, not Linear, not DeepWiki, not Glean. Public copy should keep that discipline. Do not market MCP, merged search, or hosted query as shipped. Do not compete with Obsidian for daily notes.[^landscape]

## Information architecture

Home is the only sidebar-free page (same pattern as rocci.dev). Global lanes: **Docs**, **Agents**, **Project**. GitHub is a header action, not a fourth novel.[^rocci-site]

### Home

One screen: what OKMate is, the four claims above, then cards to Install, Agents, Docs, and the repository. Short experimental-software note if that is still true of the binary. Do not lead with Askama, Datastar, or Sparkle.

### Docs (humans)

| Page | Owns |
| --- | --- |
| What is OKMate | Product definition; humans and agents; what it is not |
| Install | CLI, Homebrew cask, `cargo`, macOS app; **README remains the command source** |
| First bundle | `knowledge/` shape, `okmate check --profile strict`, `okmate view` |
| Git and ownership | Files you diff; directory vs git roots; `~/.okmate/`; no vendor store |
| Multiple bundles | Registry, `okmate roots --format paths`, `sync`, single-root commands, agent loop |
| CLI | Curated command table; JSON for agents; link to README for flags that churn |
| Desktop | OKMate.app, Sparkle versus `brew upgrade`, CLI-only binaries do not self-update |
| OKF | Format, portable `okf` crate, Base vs Strict, generated artifacts |

Install commands drift with releases. Prefer Rocdown `:include` of marked README regions, or a single sentence plus a GitHub link, over a second handwritten brew stanza.[^readme][^rocdown-cli]

### Agents (first-class lane)

This is the entrypoint the user asked for. It is not a subsection of Docs. Visiting agents and humans who set up agents both start here.[^landscape][^llmstxt][^skill]

### Project

Status (what works, what is experimental), contributing, license. Contributing points at `AGENTS.md` and in-repo skills rather than restating engine architecture.[^agents][^agent-plan]

## Agent entrypoint

Agents discovering a docs host look for `/llms.txt` (and often `/llms-full.txt` or per-page `.md`). Rocdown already emits a site-wide `/llms.txt` from page titles and descriptions, plus `/robots.txt` and `/sitemap.xml`. It does **not** yet emit clean per-page Markdown; that remains on the Rocdown backlog. The OKMate site must work with today's compiler: excellent descriptions, a dedicated Agents lane, and canonical skill files in git, not a promise of `.md` mirrors.[^rocdown-llms][^rocdown-arch][^llmstxt][^landscape]

Recommended fetch sequence:

1. `GET /llms.txt` — product one-liner, then links. Page `meta.description` values must make this index usable. Put the Agents hub near the top of the generated list by giving it a clear title such as "Agents: install, CLI, and skills".
2. `GET /agents/` — hub: what OKMate is in agent terms; install the `okmate` binary; copy or point at `.agents/skills/manage-okmate-knowledge`; run `okmate roots --format paths` then `check` / `inspect` / `search` on one root; author inert Markdown; re-check. State that commands stay single-root.[^skill][^cli][^agents]
3. `GET /agents/install/`, `/agents/skills/`, `/agents/cli/` — split so an agent can fetch one job without the whole manual.
4. Canonical skill text stays `.agents/skills/**/SKILL.md` in the repository. The site may `:include` it for humans and must link the raw or blob GitHub URL as the file to install. Do not maintain a third copy in `site/` that can drift.[^skill][^agent-plan]
5. `okmate build` `llms.txt` is a **bundle** index (`catalog` of concepts). It is the right artifact for a published knowledge tree, not a substitute for the product-site `/llms.txt`. If a later phase mounts review HTML, keep `/llms.txt` (site) and `/knowledge/llms.txt` (bundle) distinct, as the rocci.dev lane plan does.[^artifact][^site-lane][^landscape]

Optional extras that help agents without waiting for Rocdown Markdown artifacts:

- A checked-in `site/assets` or unhashed extra file only if Rocdown can serve a stable `/agents/instructions.md`. If hashing would break the URL, skip it and keep GitHub raw links.
- `robots.txt` already allows `/` and points at the sitemap when `base_url` is set.[^rocdown-llms]

Do not advertise an MCP server. The landscape record is explicit that OKMate still speaks CLI plus skill, and that MCP is how competing KM tools get discovered in Cursor. The website can say "CLI and skill today" without apologizing into a fake MCP page.[^landscape][^cli]

## Multi-bundle story on the site

The registry already exists in the application: user-level TOML, directory and git roots, cache under `OKMATE_CACHE`, `okmate roots` / `sync`. The Askama viewer workspace is a separate, later merge of chrome, not of engine catalogs. Public docs should describe **coordination**: list roots, sync git snapshots, inspect one path. They should not describe a unified search box across every company wiki.[^multi-roots][^extended][^cli][^readme]

Git ownership copy:

- You choose where bundles live (repo `knowledge/`, other checkouts, fetched remotes).
- Git history is the audit log; OKF provenance checks read `git` when asked.
- Tokens never appear in `roots` JSON. Prefer `token_env`.
- Writable authoring is the directory on disk; git roots are read-only snapshots for agents and review.[^multi-roots][^readme]

## Toolchain and hosting

| Choice | Recommendation | Why |
| --- | --- | --- |
| Authoring | `site/` in this repository, `.rocdown` + `rocdown.toml` | Same compiler as rocci.dev docs; Markdown-first; `rocdown check` without compiling Roc for catalog errors |
| Compiler | `rocdown` on `PATH`, or `cargo run -p rocci-rocdown-cli` from a sibling `../rocci` checkout | No `rocci-*` Cargo dependency in OKMate |
| Preview | `rocdown view site` | Watch and live reload; `--no-window` for agents/CI |
| Output | Static HTML (`rocdown build site --cdn-only` if the tree has no live pages) | GitHub Pages; no island origin |
| Theme | Rocdown default or a small `site/theme` pack | Do not copy rocci.dev hybrid chrome |
| Publish | Static host (GitHub Pages first); custom domain is an operator input | No VPS required for v1 |
| Pin | CI pins a `rocdown` version or a rocci revision | Sibling path is a local convenience, not a hosted guarantee |

`rocdown check` validates catalog, routes, links, and includes without Roc. Full `rocdown build` still needs the Rocci shell path. Budget CI accordingly: check on every PR if a pinned binary exists; full package on the publish job.[^rocdown-cli][^rocdown-arch]

Hostname (`okmate.dev` vs `koliyo.github.io/okmate`) is **not** decided here. `base_url` in `rocdown.toml` must match whatever the operator publishes, because `/llms.txt` and the sitemap embed it.[^rocdown-llms]

## What not to put on the website

- The full maintainer `knowledge/` tree as the marketing IA. It contains exploratory drafts, internal plans, and stale-after dates. Linking GitHub `knowledge/` plus `okmate view` is enough for v1. A later **curated demo bundle** or a filtered `/knowledge/` mount is a separate phase, following the rocci.dev lane pattern, not a Rocdown rewrite.[^site-lane][^overview]
- Planned-as-shipped features: MCP, approve/comment review, authenticated hosted query, merged cross-root catalog.[^landscape][^extract]
- Rocci language tutorials. Mention Rocdown only as "this site is built with Rocdown"; send compiler questions to rocci.dev.[^rocdown-arch]
- A second CLI bible that races the README.[^readme]

## Open operator inputs

These do not block authoring Phases 1–5 of the plan; they block a public URL.

- Public hostname and whether GitHub Pages is enough.
- Whether the experimental disclaimer stays.
- Whether a future phase publishes any OKF HTML under `/knowledge/`.
- How CI obtains `rocdown` (rocci release artifact, submodule, or documented sibling only for maintainers).

[^overview]: Engine versus application versus knowledge ownership; no `rocci-*` crate dependency.
[^readme]: CLI, install channels, `~/.okmate/`, single-root commands, Homebrew and Sparkle.
[^okf-readme]: Portable engine, profiles, artifacts.
[^agents]: Short always-on instructions; skills retrieve from the bundle.
[^skill]: `okmate roots` then per-root inspect/check/search; author inert Markdown; strict profile.
[^landscape]: Compiled-wiki job; llms.txt versus inspect; no MCP; dual human/agent files.
[^multi-roots]: Registry of directory and git roots; commands stay single-root; git snapshots read-only.
[^extended]: Viewer workspace is application chrome, not an engine merge of catalogs.
[^extract]: Askama/Datastar app over `okf`; Rocci crates stay out.
[^agent-plan]: In-repo skills and knowledge bootstrap.
[^artifact]: Bundle `llms.txt` lists concepts; distinct from a docs-site index.
[^cli]: `roots` and `sync` on the okmate binary.
[^rocdown-cli]: `rocdown` view/build/check; refuses OKF; `--cdn-only` for static trees.
[^rocdown-arch]: Rust catalog, Rocci shell, static articles; per-page Markdown still backlog.
[^rocdown-llms]: Generated `/llms.txt` from titles and descriptions; sitemap and robots.
[^rocci-site]: Home layout plus mounted docs; hybrid only because of live examples.
[^site-lane]: OKF HTML mounted beside Rocdown, not converted.
[^llmstxt]: Small Markdown index at `/llms.txt` for agents.
[^okf-spec]: Vendor-neutral Markdown+YAML knowledge format.
[^plan]: Phased website implementation; writing research does not execute it.
