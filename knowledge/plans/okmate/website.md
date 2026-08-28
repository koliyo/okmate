---
type: Implementation Plan
title: OKMate product website
description: Ship a static Rocdown site for OKMate as an open-source OKF tool with git-owned multi-bundle knowledge, a human docs lane, and a first-class /agents/ plus /llms.txt entry for visiting agents.
tags: [domain/okmate, domain/okf, concern/publication, concern/agents, concern/developer-experience]
status: draft
generated: { by: process:cursor, at: 2026-08-28T12:15:00Z }
stale_after: 2026-11-28
authority: exploratory
owners: [human:nils]
sources:
  - id: research
    resource: ../../research/okmate/website.md
    title: OKMate product website research
    author: process:cursor
    last_modified: 2026-08-28
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
    resource: ../../research/okmate/agent-dev-landscape.md
    title: OKMate and the agent knowledge-management landscape
    author: process:cursor
    last_modified: 2026-08-28
  - id: multi-roots
    resource: ../okf/multi-knowledge-roots.md
    title: Multiple knowledge roots
    author: process:cursor
    last_modified: 2026-08-25
  - id: artifact
    resource: ../../../okf/src/artifact.rs
    title: Bundle llms.txt and JSON artifacts
    author: process:git
    last_modified: 2026-08-26
  - id: cli
    resource: ../../../src/cli.rs
    title: okmate CLI including roots and sync
    author: process:git
    last_modified: 2026-08-27
  - id: agent-plan
    resource: agent-knowledge.md
    title: Bootstrap okmate agents
    author: process:cursor
    last_modified: 2026-08-26
  - id: extract
    resource: ../okf/okmate.md
    title: Okmate extractable Rust OKF mate
    author: process:cursor
    last_modified: 2026-08-26
  - id: rocdown-cli
    resource: https://github.com/koliyo/rocci/blob/main/crates/rocci-rocdown-cli/README.md
    title: rocdown CLI
    author: process:git
    last_modified: 2026-08-26
  - id: rocdown-arch
    resource: https://github.com/koliyo/rocci/blob/main/knowledge/architecture/rocdown-documentation-compiler.md
    title: Rocdown documentation generator
    author: process:cursor
    last_modified: 2026-08-26
  - id: rocdown-llms
    resource: https://github.com/koliyo/rocci/blob/main/crates/rocci-rocdown/src/plan.rs
    title: Rocdown /llms.txt generation
    author: process:git
    last_modified: 2026-08-26
  - id: site-lane
    resource: https://github.com/koliyo/rocci/blob/main/knowledge/plans/site/okf-viewer-site-lane.md
    title: OKF HTML as a site lane, not Rocdown
    author: process:cursor
    last_modified: 2026-08-26
---

# OKMate product website

## Purpose and authority

This plan executes the [website research](/research/okmate/website.md). It adds a
public Rocdown site for the OKMate tool. It does not start a phase by being
written.[^research]

The record is exploratory.

## Goal

Give visitors a static Rocdown website that presents OKMate as an open-source
OKF knowledge tool: humans and agents share git-owned Markdown bundles, several
roots stay coordinated without merging catalogs, and an agent who fetches the
host can install the CLI and skills without scraping the human docs
lane.[^research][^readme][^landscape]

## Out of bound

- Converting `knowledge/**/*.md` into Rocdown, or using `okmate view` HTML as
  the product site.[^overview][^site-lane]
- Adding any `rocci-*` crate dependency to this workspace. `rocdown` stays an
  external compiler.[^overview][^extract]
- Hybrid islands, a VPS, or copying rocci.dev's live-counter home.[^rocdown-cli]
- MCP, hosted query, review approve/comment, or a merged cross-root
  catalog — including copy that implies they ship.[^landscape][^extract]
- Minting an approved Decision or a public hostname (`okmate.dev` vs GitHub
  Pages) as decided. Hostname is an operator input to Phase 6.[^research]
- Publishing the full maintainer knowledge tree under `/knowledge/` (Phase 7
  is a later, explicit choice of a demo or filtered mount).[^site-lane]
- Rewriting README install stanzas as a second source of truth without
  includes or links.[^readme]
- Executing later phases while working one phase. Pushing remotes.

## Constraints that do not move

- Canonical project knowledge stays inert OKF Markdown. Site pages are
  Rocdown. `rocdown` refuses OKF; `okmate` does not compile
  Rocdown.[^overview][^rocdown-cli]
- `check`, `inspect`, `search`, and `build` stay single-root. Site copy and
  agent instructions teach `okmate roots --format paths` then one
  path.[^cli][^skill][^multi-roots]
- Directory roots are writable; git roots are read-only snapshots; tokens
  never appear in `roots` JSON.[^multi-roots][^readme]
- Skills remain `.agents/skills/**/SKILL.md`. The site includes or links them;
  it does not become a drifting fork.[^skill][^agent-plan]
- Do not claim production-ready behavior the binary does not have. Keep
  humans-and-agents, OKF, git ownership, and multi-bundle coordination as the
  lead story, not Askama or Sparkle.[^landscape][^readme]
- `base_url` in `rocdown.toml` must match the published host so `/llms.txt` and
  the sitemap are honest.[^rocdown-llms]

## Current behavior

This repository has no `site/` tree. Public explanation lives in `README.md`,
`okf/README.md`, `AGENTS.md`, and the knowledge bundle. Agents in this repo
follow `$manage-okmate-knowledge`. There is no product `/llms.txt` on a public
host; `okmate build` only emits a bundle-local `llms.txt`.[^readme][^agents][^skill][^artifact]

The sibling Rocci checkout provides `rocdown` (`rocci-rocdown-cli`): `check`,
`view`, `build`, `package`, generated `/llms.txt` from page titles and
descriptions. Per-page Markdown artifacts are still backlog.[^rocdown-cli][^rocdown-arch][^rocdown-llms]

## Information architecture (target)

Home (layout `home`, no sidebar). Header lanes: **Docs**, **Agents**,
**Project**. GitHub as an action.

| Lane | Pages (stems) | Job |
| --- | --- | --- |
| Home | `index` | Four claims: OSS, OKF, git ownership, multi-bundle; cards to install, agents, docs |
| Docs | `docs/index`, `docs/install`, `docs/first-bundle`, `docs/git-ownership`, `docs/multiple-bundles`, `docs/cli`, `docs/desktop`, `docs/okf` | Human manual |
| Agents | `agents/index`, `agents/install`, `agents/skills`, `agents/cli` | Entrypoint for visiting agents and humans wiring agents |
| Project | `project/index`, `project/status`, `project/contributing` | Experimental boundary, license, how to work in this repo |

Every page sets `meta.title` and `meta.description` so Rocdown's generated
`/llms.txt` is a usable agent index. The Agents hub description must name
install, CLI, and skills.[^rocdown-llms][^research]

## Toolchain (all phases)

Documented invocation, in this order:

```sh
# Prefer a rocdown on PATH (future install or CI pin).
rocdown check site
rocdown view site --no-window
rocdown build site --output dist/site
```

```sh
# Maintainer fallback: sibling Rocci checkout.
cargo run -q -p rocci-rocdown-cli --manifest-path ../rocci/Cargo.toml -- check site
```

Do not add Rocci to this repo's `Cargo.toml`. Phase 6 pins how CI gets the
binary. Until then, Exit commands may use the sibling path when `rocdown` is
missing.[^rocdown-cli][^overview][^research]

## Phases

### Phase 1 — Skeleton and toolchain

**Bound:** `site/rocdown.toml` (`[site]`, `[build]`, empty or stub `[[nav]]`),
`site/index.rocdown` placeholder, `site/assets` stub if the theme requires it,
README or `site/README.md` with the two invocations above. No custom Rocci
theme unless `rocdown check` cannot run without one. No product copy yet
beyond a one-line title.

**Out of bound:** Docs/Agents pages, CI publish, domain choice.

**Exit:**

- `rocdown check site` (or the sibling cargo invocation) succeeds.
- `okmate check knowledge --profile strict --format terminal`

### Phase 2 — Home and chrome

**Bound:** Home copy for the four claims (open source, OKF compatible, git
ownership, coordinating multiple bundles). Humans-and-agents sentence on the
first screen. Card links to Docs, Agents, Install, GitHub. Favicon/OG
placeholders. Optional small `site/theme` only if default chrome cannot
express lanes. Footer may note experimental software.

**Out of bound:** Full docs corpus; live islands.

**Exit:**

- `rocdown check site`
- `rocdown view site --no-window` serves `/` without catalog errors.
- `okmate check knowledge --profile strict --format terminal`

### Phase 3 — Human docs

**Bound:** Docs pages listed in the IA table. Install page includes or links
README regions for brew/cask/`cargo`/app zip — do not hand-duplicate command
blocks that already live in `README.md`. Multiple-bundles page states
single-root commands and the `roots` then inspect loop. OKF page distinguishes
format, `okf` crate, and `okmate` app.[^readme][^okf-readme][^multi-roots][^cli]

**Out of bound:** Agent lane pages (stubs/links from home are allowed).

**Exit:**

- `rocdown check site` (links and includes resolve).
- `okmate check knowledge --profile strict --format terminal`

### Phase 4 — Agent entrypoint

**Bound:** Agents lane pages. Hub at `/agents/` with the fetch sequence: read
site `/llms.txt`, install `okmate`, install or point at
`manage-okmate-knowledge`, `okmate roots --format paths`, then
`check` / `inspect` / `search` on one root, author Markdown, re-check. Skills
page `:include`s or quotes the in-repo `SKILL.md` and links the GitHub blob/raw
URL as canonical. CLI page lists JSON-oriented commands. Confirm a local
`rocdown build` writes `llms.txt` containing the Agents hub. Do not add
MCP.[^skill][^agents][^cli][^landscape][^rocdown-llms]

**Out of bound:** Bundle `okmate build` HTML; per-page `.md` mirrors (Rocdown
backlog).

**Exit:**

- `rocdown check site`
- `rocdown build site --output dist/site` and `dist/site/llms.txt` exists and
  names the Agents hub.
- `okmate check knowledge --profile strict --format terminal`

### Phase 5 — Project lane

**Bound:** Status (shipped versus experimental, no planned-as-shipped),
contributing (point at `AGENTS.md`, skills, `okmate check`, `cargo test -p
okf` / `okmate --no-default-features`), Apache-2.0. Cross-links to
github.com/koliyo/okmate.

**Exit:**

- `rocdown check site`
- `okmate check knowledge --profile strict --format terminal`

### Phase 6 — Static publish

**Bound:** CI job that builds `site/` with a **pinned** `rocdown` (rocci
revision, released binary, or documented fetch — pick one and record it in
the workflow). Deploy static files to GitHub Pages or another static host the
operator names. Set `base_url`. Do not introduce Docker islands or a VPS.
Optional `okmate-ops` helper only if it wraps the same `rocdown` invocation.

**Out of bound:** Custom domain purchase (operator). Changing release
signing.

**Exit:**

- Hosted workflow on the website revision succeeds (cite the run in
  `knowledge/log.md` when logging complete).
- Public `/llms.txt` and `/agents/` return 200 on the chosen host.
- `okmate check knowledge --profile strict --format terminal`

### Phase 7 — Optional public knowledge demo

**Bound:** Either (a) keep GitHub `knowledge/` links only, and close this
phase as documentation, or (b) mount `okmate build` HTML for a **dedicated
public demo bundle** or a filtered export under `/knowledge/`, not a Rocdown
rewrite of maintainer drafts. Keep site `/llms.txt` distinct from
`/knowledge/llms.txt`.[^artifact][^site-lane][^research]

**Exit:**

- Written decision in the site status page plus `rocdown check site`.
- If mounting: `okmate build <demo-root> -o dist/knowledge-demo` is part of
  CI and the lane serves `llms.txt`.
- `okmate check knowledge --profile strict --format terminal`

## Copy rules (every authoring phase)

- Lead with the tool, not the stack.
- Same artifacts for humans and agents; git is the store; OKF is the contract;
  many bundles, one-at-a-time commands.
- Name competitors only to draw a boundary (DeepWiki, Basic Memory, hosted
  MCP) as in the landscape research — no fake feature parity.[^landscape]
- Experimental limits stay on Project/status, not hidden.

## Validation (bundle, after knowledge edits)

```sh
okmate check knowledge --profile strict --format terminal
```

Site Exit commands above are in addition to that check whenever `site/`
exists.

[^research]: IA, dual compiler split, agent fetch sequence, static hosting.
[^overview]: No Rocci crates in OKMate; knowledge stays OKF.
[^readme]: Install channels and CLI table remain canonical.
[^okf-readme]: Engine and profiles.
[^agents]: Short AGENTS.md plus skills.
[^skill]: Multi-root retrieve loop; strict check after authoring.
[^landscape]: Positioning and no MCP.
[^multi-roots]: Registry semantics and single-root commands.
[^artifact]: Bundle llms.txt is not the product-site index.
[^cli]: `roots` / `sync`.
[^agent-plan]: Skills live in-repo.
[^extract]: Application boundary.
[^rocdown-cli]: External `rocdown` binary.
[^rocdown-arch]: Static docs compiler; Markdown mirrors backlog.
[^rocdown-llms]: Generated llms.txt from descriptions.
[^site-lane]: OKF HTML may be mounted later; not converted.
