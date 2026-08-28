---
type: Research Report
title: OKMate and the agent knowledge-management landscape
description: Compiled, reviewable Markdown knowledge is a distinct job from Linear tickets, DeepWiki, Glean, and Mem0; OKMate's realistic peers are Karpathy-style wikis, Basic Memory, and Letta MemFS, and adoption stays niche without MCP and a promotion path from sessions.
tags: [domain/okmate, domain/okf, concern/agents, concern/retrieval, concern/review]
status: draft
generated: { by: process:cursor, at: 2026-08-28T12:00:00Z }
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
    title: Published OKMate stack and CLI
    author: process:git
    last_modified: 2026-08-28
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
  - id: okf-tools
    resource: ../okf/okf-tools-and-workflows.md
    title: State-of-the-art OKF tools and workflows
    author: process:codex
    last_modified: 2026-08-26
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
  - id: search-rs
    resource: ../../../okf/src/search.rs
    title: Lexical chunk search (term-contains)
    author: process:git
    last_modified: 2026-08-26
  - id: lib-rs
    resource: ../../../okf/src/lib.rs
    title: okf check, inspect, search, build
    author: process:git
    last_modified: 2026-08-26
  - id: artifact
    resource: ../../../okf/src/artifact.rs
    title: catalog.json, search.json, llms.txt, validation.json
    author: process:git
    last_modified: 2026-08-26
  - id: cli
    resource: ../../../src/cli.rs
    title: okmate CLI surface
    author: process:git
    last_modified: 2026-08-27
  - id: okf-blog
    resource: https://cloud.google.com/blog/products/data-analytics/how-the-open-knowledge-format-can-improve-data-sharing
    title: Introducing the Open Knowledge Format
    author: organization:google-cloud
  - id: okf-repo
    resource: https://github.com/GoogleCloudPlatform/open-knowledge-format
    title: Canonical OKF specification repository
    author: organization:google-cloud
  - id: karpathy
    resource: https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f
    title: LLM Wiki idea file
    author: human:andrej-karpathy
  - id: deepwiki
    resource: https://cognition.com/blog/deepwiki
    title: DeepWiki - AI docs for any repo
    author: organization:cognition
  - id: deepwiki-mcp
    resource: https://cognition.com/blog/deepwiki-mcp-server
    title: DeepWiki MCP server
    author: organization:cognition
  - id: llmstxt
    resource: https://llmstxt.org
    title: The /llms.txt file, v2
    author: organization:answer-ai
  - id: mintlify-llms
    resource: https://www.mintlify.com/docs/ai/llmstxt.md
    title: Mintlify llms.txt
    author: organization:mintlify
  - id: sourcegraph-mcp
    resource: https://sourcegraph.com/mcp
    title: Sourcegraph MCP
    author: organization:sourcegraph
  - id: sourcegraph-agentic
    resource: https://sourcegraph.com/blog/agentic-coding
    title: Agentic Coding in 2026
    author: organization:sourcegraph
  - id: unblocked-mcp
    resource: https://getunblocked.com/blog/why-mcp-isnt-enough-enhancing-agent-capabilities-with-a-context-engine/
    title: Why MCP isn't enough for coding agents
    author: organization:unblocked
  - id: unblocked-glean
    resource: https://getunblocked.com/blog/unblocked-vs-glean/
    title: Unblocked versus Glean
    author: organization:unblocked
  - id: rovo
    resource: https://developer.atlassian.com/cloud/rovo-mcp/
    title: Atlassian Rovo MCP
    author: organization:atlassian
  - id: notion-mcp
    resource: https://developers.notion.com/guides/mcp/overview
    title: Notion MCP
    author: organization:notion
  - id: pkm-mcp
    resource: https://chatforest.com/guides/mcp-personal-knowledge-management-pkm/
    title: MCP and personal knowledge management
    author: organization:chatforest
  - id: basic-memory
    resource: https://github.com/basicmachines-co/basic-memory/
    title: Basic Memory
    author: organization:basic-machines
  - id: basic-compare
    resource: https://basicmemory.com/blog/basic-memory-vs-mem0-vs-letta
    title: Basic Memory versus Mem0 versus Letta
    author: organization:basic-machines
  - id: letta-memfs
    resource: https://docs.letta.com/concepts/memfs/index.md
    title: Letta MemFS context repositories
    author: organization:letta
  - id: cognee
    resource: https://www.cognee.ai/
    title: Cognee agent memory platform
    author: organization:cognee
  - id: supermemory
    resource: https://github.com/supermemoryai/supermemory
    title: Supermemory
    author: organization:supermemory
  - id: memnexus-forget
    resource: https://memnexus.ai/blog/2026-02-20-ai-coding-assistant-memory
    title: How AI coding assistants forget everything
    author: organization:memnexus
  - id: linear-site
    resource: https://linear.app
    title: Linear product site
    author: organization:linear
  - id: linear-now
    resource: https://linear.app/now/coding-sessions-for-linear-agent
    title: Now Linear writes the code, too
    author: organization:linear
  - id: linear-coding
    resource: https://linear.app/docs/coding-sessions
    title: Linear coding sessions
    author: organization:linear
  - id: linear-mcp
    resource: https://linear.app/docs/mcp
    title: Linear MCP server
    author: organization:linear
  - id: linear-docs
    resource: https://linear.app/docs/documents
    title: Linear documents
    author: organization:linear
  - id: linear-loops
    resource: https://linear.app/now/introducing-loops
    title: Introducing Loops
    author: organization:linear
  - id: linear-agent
    resource: https://linear.app/docs/linear-agent
    title: Linear Agent
    author: organization:linear
  - id: zed-delta
    resource: https://zed.dev/blog/introducing-delta
    title: Introducing Delta
    author: organization:zed
  - id: zed-deltadb
    resource: https://zed.dev/blog/introducing-deltadb
    title: Software Is Made Between Commits (DeltaDB)
    author: organization:zed
  - id: agents-md
    resource: https://agents.md
    title: AGENTS.md
    author: organization:agentic-ai-foundation
  - id: claude-memory
    resource: https://code.claude.com/docs/en/memory
    title: How Claude remembers your project
    author: organization:anthropic
  - id: beads
    resource: https://github.com/steveyegge/beads
    title: Beads distributed graph issue tracker
    author: human:steve-yegge
  - id: fountain
    resource: https://fountaincity.tech/resources/blog/agent-memory-knowledge-systems-compared/
    title: Agent Memory - 8 knowledge systems compared
    author: organization:fountain-city
  - id: graphiti
    resource: https://github.com/getzep/graphiti
    title: Graphiti temporal knowledge graphs
    author: organization:zep
  - id: okfcli
    resource: https://github.com/okfcli/okf
    title: Agent-native OKF Go CLI
    author: organization:okfcli
---

# OKMate and the agent knowledge-management landscape

## Scope

This report maps **agent knowledge management** as developers actually practice it in 2026: compiled Markdown wikis, generated code wikis, enterprise context engines, docs-for-agents, extraction memory APIs, PKM vaults with MCP, harness-local instruction files, work queues, and execution threads. Linear, Zed Delta, Beads, and OKF CLIs remain in scope as adjacent jobs, not as the whole field.[^karpathy][^deepwiki][^fountain][^linear-site][^zed-delta]

OKF-only tooling is surveyed in [State-of-the-art OKF tools and workflows](../okf/okf-tools-and-workflows.md). This record cites that survey instead of repeating it.[^okf-tools]

Claims about OKMate behavior are descriptive of this repository. Positioning and adoption odds are exploratory inference. Vendor numbers and competitive claims (Glean ARR, DeepWiki index size, memory-benchmark scores) are those vendors' published figures, not independently verified here.

## What OKMate is

OKMate is a standalone knowledge application for Open Knowledge Format bundles. The portable `okf` crate owns parse, validate, graph, lexical search, retrieval benchmarks, and machine artifacts. The `okmate` binary owns CLI, Askama HTML, Axum preview, and an optional desktop window. Records stay inert Markdown with YAML frontmatter.[^overview][^readme][^okf-readme]

The agent-facing contract that is actually shipped:

| Surface | Role |
| --- | --- |
| `okmate check` | Conformance and profile diagnostics (strict owners-and-evidence) |
| `okmate inspect catalog\|concept\|graph` | Normalized JSON for concepts and links |
| `okmate search` | Metadata and heading chunks as JSON |
| `okmate build` | `catalog.json`, `search.json`, `validation.json`, `llms.txt`, plus HTML |
| `okmate roots` / `sync` | Multi-root listing; git fetch of configured roots |
| `okmate view` | Local human review shell |

There is no MCP server, no authoring or write CLI, and no issue-tracker query. Agents are expected to edit Markdown in git and re-run check/inspect/search.[^cli][^lib-rs][^artifact][^skill][^extract]

Search is lexical: heading and metadata chunks, AND of substring terms, with filters for type, status, authority, tags, trust, and staleness. It is not embeddings and not a ranked BM25 scorer in the current engine.[^search-rs][^okf-readme]

This repository already uses that loop: `AGENTS.md` points durable plans and reports at `knowledge/`, and `$manage-okmate-knowledge` tells agents to search, inspect, author, and `okmate check knowledge --profile strict`.[^agents][^skill]

The extract plan left review decisions, hosted query, and MCP on a later application horizon. The intended workflow is still "agents edit Markdown; deterministic checks judge; humans review."[^extract][^okf-app]

## Jobs in agent knowledge management

Agent-driven development is several knowledge jobs sharing one person. Tools win a job, then try to annex the others. OKMate should be scored as a **compiled, reviewable knowledge layer**, not as "another Linear" or "another Mem0."

| Job | What the agent needs | Typical 2026 winners | OKMate today |
| --- | --- | --- | --- |
| Always-on instructions | Short rules loaded every turn | [AGENTS.md](https://agents.md), CLAUDE.md, Cursor/Continue/Copilot rules | Thin skill that *points at* the bundle |
| Compiled wiki | Interlinked Markdown that compounds | Karpathy LLM wiki, OKF bundles, Basic Memory | **Primary fit** |
| Generated code wiki | Architecture synthesized from the repo | DeepWiki / Devin Wiki | Out of scope (derived, not curated) |
| Code intelligence | Exact symbols, refs, history across repos | Sourcegraph MCP, Graphify-style local graphs | Out of scope |
| Enterprise context | Synthesized answers across Slack, PRs, tickets, docs | Glean, Unblocked, Atlassian Rovo | Out of scope |
| Public/API docs for agents | Discoverable Markdown indexes | [llms.txt](https://llmstxt.org), Mintlify, Fern | Partial (`okmate build` emits `llms.txt`) |
| Work queue | Ready work, claims, dependencies | Linear, Beads, Jira | Plans as Markdown, not `ready` |
| Execution / pairing | Code + conversation while work happens | Cursor, Claude Code, Linear coding sessions, Zed Delta | Out of scope |
| Extraction memory | Facts pulled from chat into a store | Mem0, Graphiti/Zep, Cognee, SuperMemory | Explicit records only |
| Per-agent memory FS | Git Markdown that travels with the agent | Letta MemFS | Similar files, team-owned not agent-owned |
| Human PKM | Notes a person likes to write | Obsidian, Logseq, Notion (+ MCP) | Same Markdown shape, stricter contract |

Google's OKF announcement is explicit that the missing piece is a **format**, not another knowledge service, and that Obsidian, Notion, Hugo, AGENTS.md, and "LLM wiki" folders already look similar but do not interoperate.[^okf-blog][^okf-repo]

## The compiled-wiki pattern

Andrej Karpathy's April 2026 [LLM Wiki](https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f) gist is the clearest statement of this job. RAG (NotebookLM, ChatGPT uploads, most vector stores) re-discovers fragments on every question. A wiki **compiles** sources into interlinked Markdown once, then keeps the synthesis current. Three layers: immutable raw sources, an LLM-maintained wiki (`index.md`, `log.md`, entity and concept pages), and a schema file (`AGENTS.md` / `CLAUDE.md`) that makes the agent a disciplined maintainer. Operations are ingest, query, and lint. Karpathy's own setup is an agent on one side and Obsidian on the other; the human sources and asks, the LLM does bookkeeping.[^karpathy]

That pattern is culturally winning among agent-driven developers. It is also underspecified: no required metadata, no source/footnote alignment, no profile that fails CI, no portable consumer besides "read the folder." Google's OKF is one attempt to freeze a small interoperability surface on top of the same Markdown directory. OKMate is a consumer of that surface with check, inspect, search, graph, and a review shell.[^okf-blog][^overview]

Important disagreement with Karpathy: he says the LLM should **own** the wiki and humans should rarely write it. OKMate's strict profile assumes the opposite for high-value claims: humans own verification, agents draft, sources and footnotes stay aligned, trust tiers distinguish generated from reviewed. That is closer to Karpathy's "business/team wiki with humans in the loop" aside than to the personal-wiki default.[^karpathy][^okf-app][^skill]

## Closest product peers: Basic Memory and Letta MemFS

**Basic Memory** is the nearest shipped product. Knowledge lives as Markdown on disk (Obsidian-compatible). Humans and agents read and write the same files. Wikilinks form a graph. An MCP server exposes search, recent activity, and graph navigation with behavior hints so agents pick tools without trial-and-error. The project's own comparison frames Mem0/SuperMemory as opaque APIs and Letta as an agent framework, while Basic Memory is "can you read what it stored?"[^basic-memory][^basic-compare]

OKMate versus Basic Memory: same philosophy (files you can diff), different contract. Basic Memory optimizes personal/MCP convenience and a looser note graph. OKMate optimizes bundle conformance, owners, keyed citations, staleness, retrieval benchmarks, a desktop review shell, and a JSON CLI without MCP. A developer who wants "Claude remembers my notes in Obsidian" will pick Basic Memory. A team that wants "architecture records fail CI if sources drift" will pick OKMate or an OKF CLI.[^basic-memory][^overview][^cli]

**Letta MemFS** (also called context repositories) projects a per-agent git repo of Markdown with YAML `description` frontmatter onto disk. Files under `system/` load every turn; the rest are discovered by path. Default search is ordinary file tools, not vectors. Commits are the save boundary; cloud agents sync the memory repo; subagents use git worktrees. This is OKMate-shaped storage in service of **one agent's identity**, not a team's curated bundle. Shared MemFS exists for multiple Letta agents, still inside Letta's runtime.[^letta-memfs]

OKMate should not become Letta. Letta is a stateful agent harness. OKMate is a knowledge application any harness can drive. The overlap (git Markdown, frontmatter, progressive disclosure) is evidence the compiled-wiki shape is spreading, not that the products are interchangeable.[^letta-memfs][^overview]

## Generated code wikis: DeepWiki

Cognition's DeepWiki is the public Devin Wiki: replace `github.com` with `deepwiki.com` on a repo URL and get generated architecture docs, diagrams, and chat grounded in the codebase. Cognition publishes that tens of thousands of public repos are pre-indexed; private repos go through a Devin account. A free MCP server (`ask_question`, `read_wiki_contents`, `read_wiki_structure`) lets Cursor and Claude Code query that wiki without auth for public repos.[^deepwiki][^deepwiki-mcp]

DeepWiki answers "what does this codebase look like?" by compiling **code**. OKMate answers "what did we decide, with evidence?" by compiling **authored claims**. Generated wikis go stale when the generator's ontology disagrees with the team's; they also invent structure the repo never wrote down. Complementary use: DeepWiki (or a local Graphify/Understand-Anything graph) for onboarding and code navigation; OKMate for decisions that must survive a rewrite of the wiki generator.

A team that thinks OKMate will replace DeepWiki is solving the wrong problem. A team that lets DeepWiki be the only architecture record will lose provenance the first time the generator changes its mind.

## Code intelligence is not knowledge management

Sourcegraph MCP gives agents keyword and semantic search, go-to-definition, find-references, commit history, and Deep Search across indexed repos. Sourcegraph's 2026 agentic-coding guide places this **under** Claude Code, Codex, Gemini CLI, and Amp: agents write code; enterprises need visibility into all existing code. Stripe is quoted using MCP to gather internal docs, tickets, build status, and Sourcegraph search.[^sourcegraph-mcp][^sourcegraph-agentic]

Local "code knowledge graph" skills (Graphify, Understand-Anything, and similar) try to give a single-repo subset of that: AST graphs, fewer tokens, often local. They are maps of **what the code is**, not records of **why we accepted a tradeoff**. OKMate should not grow a SCIP index. It should remain the place a Sourcegraph-informed agent files the decision after it understands the call graph.

## Enterprise context engines

Once an org has more than a git repo, agents drown in MCP servers. Unblocked's published argument: MCP gives access, not judgment; a context engine should sit behind one MCP tool, fan out to PRs, Slack, Jira, Notion, Confluence, and code, then return one cited, permission-checked answer. Unblocked contrasts itself with Glean as engineering-depth synthesis versus company-wide enterprise search (Glean's published strength is sales, HR, and support content). Treat that contrast as vendor positioning.[^unblocked-mcp][^unblocked-glean]

Atlassian Rovo MCP is the incumbent suite's version: OAuth-gated tools over Jira, Confluence, Compass, JSM, Bitbucket, plus natural-language search across the Teamwork Graph. Notion's hosted MCP (`https://mcp.notion.com/mcp`) lets Cursor, Claude Code, and Codex search, read, and update the workspace, including custom agent sessions.[^rovo][^notion-mcp]

These products win **organizational recall**. They do not produce a portable, profile-validated Markdown bundle. Conflict resolution ("the ADR says X, Slack said Y six months later") is their claimed value and also their opacity: the synthesis lives in the engine, not in a file a reviewer diffs. OKMate's opposite bet is that some facts must be promoted into an explicit record. Context engines make that promotion optional; they do not make it unnecessary for architecture and compliance-shaped knowledge.[^unblocked-mcp][^okf-blog]

## Documentation for agents: llms.txt

[llms.txt](https://llmstxt.org) (v2) is a small Markdown index at a site path so agents discover LLM-friendly pages instead of scraping HTML. Anthropic, OpenAI, and Gemini publish one for their own docs. Mintlify, Fern, GitBook, and others auto-generate `/llms.txt`, often `/llms-full.txt`, and per-page `.md`. Chrome Lighthouse audits for the file as part of agentic browsing checks.[^llmstxt][^mintlify-llms]

`okmate build` already emits `llms.txt` for a bundle. That is the right artifact for **published** knowledge (a docs site, a public architecture handbook). It is not a substitute for inspect/search inside a working tree, and it does not carry OKF trust tiers or source/footnote checks. Mintlify/Fern compete with OKMate only if the team's entire knowledge strategy is "host docs and let agents fetch Markdown." Most agent-driven product teams need both an external docs index and an internal decision graph.[^artifact][^llmstxt]

## Extraction memory APIs

Mem0, SuperMemory, Cognee, Zep/Graphiti, and LangMem extract facts from conversations (and sometimes docs, tickets, Drive) into vectors, graphs, or both. MCP is now table stakes for these products. SuperMemory markets memory *and* RAG plus connectors (Drive, Gmail, Notion, GitHub). Cognee markets ontologies and a "company brain" with GitHub/Slack/Linear connectors. Graphiti remains the temporal-graph engine (validity windows). A mid-2026 comparison still treats **markdown vault plus search** as the path with the strongest human-agent merge.[^supermemory][^cognee][^graphiti][^fountain][^basic-compare]

These systems optimize recall for chat agents. Humans typically cannot diff the store. Automatic extraction without a review gate fights OKMate's strict profile. The useful composition is: extraction memory for preferences and "what we tried yesterday"; OKMate for claims that need owners, sources, and `stale_after`.[^okf-app][^fountain]

Claude Code auto memory (`MEMORY.md` under `~/.claude/projects/`) and Continue/Windsurf/Copilot instruction files are harness-local. Cursor shipped Memories in 2025 and, according to later round-ups, removed the built-in feature in 2.1 in favor of rules files. None of these are a team database. Cross-tool memory is a cottage industry of MCP servers precisely because the harnesses do not share state.[^claude-memory][^memnexus-forget]

## PKM vaults and MCP

Obsidian remains the IDE Karpathy assumes for LLM wikis. As of mid-2026 it has no official MCP server; community servers typically wrap the Local REST API plugin. Notion has an official hosted MCP with OAuth and admin controls. Logseq, Capacities, Tana, Heptabase, and similar tools appear in MCP registries with uneven quality. The PKM+MCP pattern is: local agent reads Markdown on disk; cloud agent reads Notion/Confluence through a hosted server; sync plugins try to keep one source of truth.[^karpathy][^pkm-mcp][^notion-mcp]

OKMate is stricter PKM: YAML types, concept IDs, bundle-root links, evidence. It will feel hostile as a daily notes app and appropriate as an architecture register. Competing with Obsidian for journal capture is a strategy error; offering a checked-in subset of an Obsidian vault as an OKF bundle is a plausible import path, not a current feature.[^okf-blog][^overview]

## Linear, Beads, and Zed Delta (adjacent, not KM)

Linear (Agent, documents, [coding sessions](https://linear.app/now/coding-sessions-for-linear-agent), [Loops](https://linear.app/now/introducing-loops), [hosted MCP](https://linear.app/docs/mcp)) is the product-development system of record: tickets, PRDs, team workflows, cloud agents that open PRs. Knowledge is workspace history, not a portable bundle. Complementary shape: Linear owns "what we are building this cycle"; OKMate owns "what is true about this system, with sources."[^linear-site][^linear-now][^linear-docs][^linear-loops][^linear-mcp][^linear-agent][^linear-coding]

Beads (`bd`) is a git-backed task graph (`bd ready`, typed dependencies). OKMate plans are knowledge *about* work, not a live queue.[^beads]

Zed Delta / DeltaDB records every edit between commits and glues the agent conversation to the worktree. That answers "why did the agent write this line?" OKMate answers "what did we decide, with sources, and is it still current?" Session transcript as memory makes a **promotion path** into a reviewed record more necessary, not less.[^zed-delta][^zed-deltadb]

## OKF ecosystem

OKF is still an emerging format. Google published it as a vendor-neutral Markdown+YAML contract; the reference agent and one-file visualizer are proofs of concept. Independent CLIs (`okfcli`, `okf-gem`, Open Knowledge) converge on JSON-first validation, graph, and search. OKMate is one of the more complete consumers (strict evidence profile, live preview, static HTML, retrieval benchmark, multi-root config). It is not Google Cloud Knowledge Catalog.[^okf-blog][^okf-repo][^okf-tools][^okfcli][^overview]

## AGENTS.md and editor rules

[AGENTS.md](https://agents.md) is the cross-agent instruction file (Codex, Cursor, Copilot, Gemini CLI, Zed, and others; Claude Code still loads `CLAUDE.md` and documents an `@AGENTS.md` import). Continue, Windsurf, and Copilot keep parallel rule directories. The file is plain Markdown, no schema, sized to load every turn. Overflowing it is a known failure. The healthy pattern, which this repository uses, is a short AGENTS.md plus a skill that retrieves from a larger store.[^agents-md][^claude-memory][^memnexus-forget][^agents][^skill]

## Adoption for agent-driven developers

**Yes, a bounded set, if OKMate stays the compiled-wiki layer and does not pretend to be Linear, DeepWiki, Glean, or Mem0.**

The 2026 KM market is loud. Most of it is not a rival: it is a different job. The actual rivals for OKMate's job are (1) an undisciplined Karpathy wiki in Obsidian, (2) Basic Memory, (3) Letta MemFS used as a team folder, and (4) "we put ADRs in Notion and let MCP search them."

### Who can adopt it now

1. **Teams already compiling git Markdown for agents** (plans, ADRs, research) who have outgrown AGENTS.md and do not want an opaque memory API. This repository is the existence proof.[^agents][^skill][^readme]
2. **People who need CI-shaped knowledge.** Basic Memory and Karpathy wikis do not fail a build when a footnote and a source id diverge. OKMate's strict profile does.[^okf-readme][^skill]
3. **Teams that already split jobs:** Linear/Jira/Beads for work, Sourcegraph/DeepWiki for code maps, Cursor/Claude for execution, OKMate for promoted facts.[^linear-now][^sourcegraph-mcp][^deepwiki]

### Who will not

- Orgs whose knowledge strategy is Glean, Rovo, Notion MCP, or Unblocked. They already have "ask the company brain."[^rovo][^notion-mcp][^unblocked-mcp]
- Developers whose wiki is DeepWiki. They wanted generated onboarding, not curated evidence.[^deepwiki]
- Personal PKM users who want Obsidian + Basic Memory + MCP this afternoon.[^basic-memory][^pkm-mcp]
- Anyone who wants install-MCP-and-go. OKMate still has no MCP, and search is term-contains.[^cli][^search-rs]
- Product orgs standardized on Linear Agent for documents and coding sessions.[^linear-site]

### What is actually strong

- **Same artifact for humans and agents**, with a checkable contract (owners, sources, footnotes, staleness, trust). Karpathy and Basic Memory share the files; OKMate adds judgment that can live in CI.[^okf-blog][^basic-compare][^okf-readme]
- **Progressive disclosure** via indexes, inspect, and search, plus a build-time `llms.txt` for published trees.[^lib-rs][^artifact][^llmstxt]
- **Portable engine.** `okf` can be reused; OKMate is one consumer. Format-not-platform is how a small tool survives Linear-scale and Cognition-scale products.[^overview][^okf-blog]

### What blocks wider adoption

- **The Karpathy gist is free and good enough** for many individuals. OKF ceremony is the tax for teams; most individuals will not pay it.[^karpathy]
- **MCP is how KM products get discovered in Cursor.** DeepWiki, Notion, Linear, Atlassian, Basic Memory, Cognee, and SuperMemory all speak it. OKMate speaks CLI + skill.[^deepwiki-mcp][^notion-mcp][^linear-mcp][^basic-memory][^cli]
- **Context engines and generated wikis absorb the "just ask" habit.** Promotion into an explicit record has to be a deliberate practice.[^unblocked-mcp][^deepwiki]
- **Missing promotion UX** from thread → draft record → check → human verify. Delta, Linear Agent, and Letta make capture easy and proprietary; OKMate makes review honest and capture manual.[^zed-delta][^letta-memfs][^okf-app]
- **Horizon features still plans:** review approve/comment, authenticated query, MCP.[^extract][^okf-app]

### Verdict

Agent knowledge management in 2026 is not one market. It is at least compiled wikis, generated code wikis, enterprise search, docs indexes, extraction APIs, PKM vaults, instruction files, work queues, and execution threads. OKMate is a serious attempt at **compiled, reviewable, portable project knowledge**. That job is real. It is not the job most agent-driven developers will name first.

Realistic adoption is developers and small teams who already treat git Markdown as more than scratch paper, who have felt silent drift in a Karpathy wiki or a Notion dump, and who will tolerate a strict check the way they tolerate tests. Closest substitutes: Basic Memory (easier, less judgment) and an untyped LLM wiki (easier still). Closest non-substitutes that will steal attention anyway: DeepWiki, Linear documents, Notion MCP, and whatever context engine the company already bought.

The path that can work:

1. Keep AGENTS.md tiny; retrieve from the bundle on demand (already the in-repo pattern).[^agents][^skill]
2. Stay complementary: Linear/Beads for work, Sourcegraph/DeepWiki for code maps, Delta/Cursor for threads, extraction memory for chat trivia, OKMate for promoted facts.
3. If MCP or "draft a record from this session" appears, keep them adapters on inert Markdown, not a new system of record.[^okf-app][^okf-blog]
4. Dogfood until `okmate check` feels as ordinary as `cargo test`. Developers copy public-repo workflows, not specs.

If those hold, a slice of agent-driven developers will use OKMate the way they use a test suite. If it tries to become the company brain, the execution thread, or the generated wiki, Glean, Zed, and Cognition already occupy those grounds.

[^overview]: Engine versus application versus knowledge ownership; settings under `~/.okmate/`.
[^readme]: Published CLI, stack, desktop, and install paths.
[^okf-readme]: UI-neutral engine: profiles, graph, search, artifacts.
[^agents]: In-repo instruction to keep durable writing in `knowledge/` and validate with `okmate check`.
[^skill]: Retrieve, author, and strict-check procedure for this bundle.
[^okf-tools]: OKF CLI, viewer, MCP, and closed evidence-loop survey.
[^extract]: Okmate extract bound; review, MCP, and hosted query out of bound.
[^okf-app]: Target workflow: Markdown edits, deterministic checks, human review; no write MCP required.
[^search-rs]: Search chunks are metadata and headings; matching is AND of lowercase substring terms plus filters.
[^lib-rs]: `check`, `inspect` catalog/concept/graph, `search` public API.
[^artifact]: Build emits catalog, search index, `llms.txt`, and validation JSON.
[^cli]: Shipped subcommands; no MCP or authoring command.
[^okf-blog]: OKF as format not platform; producer/consumer split; familiarity with Obsidian, Notion, AGENTS.md, LLM wikis (Google Cloud, 2026-06-12).
[^okf-repo]: Canonical spec and reference implementations.
[^karpathy]: LLM Wiki gist: compile sources into interlinked Markdown; ingest/query/lint; index.md and log.md; LLM writes, human sources (2026-04-04).
[^deepwiki]: Public Devin Wiki for GitHub repos; generated architecture docs and chat.
[^deepwiki-mcp]: Unauthenticated public MCP tools over indexed DeepWiki repos.
[^llmstxt]: llms.txt v2: small Markdown index plus per-page `.md` for agents.
[^mintlify-llms]: Hosted docs platforms auto-generate llms.txt and related agent files.
[^sourcegraph-mcp]: Cross-repo code intelligence MCP: search, navigation, history, Deep Search.
[^sourcegraph-agentic]: Code intelligence sits under coding agents; does not replace them.
[^unblocked-mcp]: Vendor claim that MCP access is not synthesis; context engine behind one MCP tool.
[^unblocked-glean]: Unblocked's published contrast with Glean (engineering vs company-wide search).
[^rovo]: Atlassian hosted MCP over Jira, Confluence, Compass, Bitbucket, Teamwork Graph.
[^notion-mcp]: Hosted Notion MCP for search, read, and update from coding agents.
[^pkm-mcp]: Survey of PKM MCP servers; Obsidian community-only, Notion official hosted.
[^basic-memory]: Local-first Markdown knowledge graph with MCP tools for agents.
[^basic-compare]: Files-you-can-read versus opaque memory APIs and agent frameworks.
[^letta-memfs]: Per-agent git Markdown memory with YAML frontmatter; system/ always loaded.
[^cognee]: Graph/ontology memory platform with MCP and connectors including Linear and GitHub.
[^supermemory]: Hybrid RAG plus extracted memory, connectors, and MCP.
[^memnexus-forget]: Harness-local rules and memories; Cursor Memories reported removed in 2.1.
[^linear-site]: Linear as product-development system for teams and agents.
[^linear-now]: Coding sessions, shared issue context, published agent-loop claims (2026-06-11).
[^linear-coding]: Cloud Claude Code / Codex sessions from an issue.
[^linear-mcp]: Hosted Linear MCP for Cursor, Claude, Codex, Zed, and others.
[^linear-docs]: Documents as long-form context attached to work.
[^linear-loops]: Recurring team-level agent workflows (2026-07-20).
[^linear-agent]: Workspace-native agent over issues, projects, documents.
[^zed-delta]: Delta app, DeltaDB replication, Claude Code, private beta (2026-08-12).
[^zed-deltadb]: Operation-level identity between commits; conversation and worktree as one artifact.
[^agents-md]: Cross-agent instruction file; no schema; nearest file wins; AAIF stewardship.
[^claude-memory]: CLAUDE.md versus AGENTS.md import; auto memory under `~/.claude/projects/`.
[^beads]: Git-backed agent issue graph, `bd ready`, JSON-first, not a PRD/wiki.
[^fountain]: Memory-system comparison; markdown vault plus search as the strong human-agent merge path.
[^graphiti]: Temporal knowledge graphs with validity windows; extraction-first, infrastructure-heavy.
[^okfcli]: Agent-native OKF Go CLI as an independent consumer/producer.
