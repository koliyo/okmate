---
type: Research Report
title: Memanto integration setup and OKF usage
description: Memanto is a derived semantic memory layer over Moorcheh; keep authoring this bundle in OKF, import with migrate, and do not treat memory sync --okf as a live watch on knowledge/.
tags: [domain/okmate, domain/okf, concern/agents, concern/retrieval]
status: draft
generated: { by: process:cursor, at: 2026-08-30T10:20:00Z }
stale_after: 2026-11-30
authority: exploratory
owners: [human:nils]
sources:
  - id: okf-docs
    resource: https://docs.memanto.ai/integrations/okf
    title: Memanto OKF export, import, and sync
    author: organization:moorcheh
  - id: migrate-docs
    resource: https://docs.memanto.ai/cli/migrate/migrate.md
    title: memanto migrate including OKF
    author: organization:moorcheh
  - id: sync-docs
    resource: https://docs.memanto.ai/cli/data/sync.md
    title: memanto memory sync
    author: organization:moorcheh
  - id: onprem
    resource: https://docs.memanto.ai/on-prem/quickstart.md
    title: Memanto on-prem quickstart
    author: organization:moorcheh
  - id: census
    resource: ../okf/knowledge-systems-built-on-okf.md
    title: Knowledge systems built on OKF
    author: process:cursor
    last_modified: 2026-08-29
  - id: landscape
    resource: agent-dev-landscape.md
    title: Agent knowledge-management landscape
    author: process:cursor
    last_modified: 2026-08-28
  - id: skill
    resource: ../../../.agents/skills/manage-okmate-knowledge/SKILL.md
    title: Manage Okmate knowledge skill
    author: process:git
    last_modified: 2026-08-26
---

# Memanto integration setup and OKF usage

Memanto is a memory agent (remember, recall, answer, expiry, conflicts) whose
retrieval engine is Moorcheh. OKF is documented as an **at-rest interchange**,
not the live store: Moorcheh answers search; markdown on disk is how memories
are exported, imported, and copied into a project wiki.[^okf-docs][^census]

This repository keeps **authoring in `knowledge/`**. Import into Memanto is a
repeatable command, not a file watcher. That split matches the census
classification: Memanto is a Class B projection (OKF in and out around a
semantic engine), with an essay claim of OKF-canonical storage that the
operational docs do not yet prove.[^census][^skill]

## Roles

| Layer | Job | Must stay canonical? |
| --- | --- | --- |
| `knowledge/` + `okmate` | Typed records, graph, check, review | Yes — write here |
| Moorcheh (`:8080`) | Embeddings, namespaces, RAG | No — derived index |
| Memanto CLI / `memanto ui` (`:8000`) | Agents, sessions, answer | No — operator surface |
| Ollama | Embed + chat models | Runtime only |

`okmate search` remains lexical and structural. `memanto recall` / `answer`
are semantic over whatever was last imported or remembered. They are
complementary, not substitutes.[^landscape]

## Setup (on-prem)

Documented first-run path: install the CLI (`pip` or `uv`), run `memanto` with
no subcommand, choose **Moorcheh On-Prem**, pick embedding and LLM providers,
then let the wizard write `~/.moorcheh/config.json`, start Docker, wait for
`http://localhost:8080/health`, and save `~/.memanto/on-prem/state.json`.[^onprem]

| Component | Typical location | Started by |
| --- | --- | --- |
| Memanto CLI / UI | Python env (`uv run memanto`) | `memanto`, `memanto ui`, `memanto serve` |
| Moorcheh server | Docker `moorcheh-onprem-server` | `moorcheh up` |
| Ollama | Docker profile `bundled-ollama`, **or** host `brew` / Ollama.app | Wizard or operator |
| Models | Container volume, **or** `~/.ollama` on the host | `ollama pull` |
| Provider config | `~/.moorcheh/config.json` | Wizard / `moorcheh configure` |
| Agent state | `~/.memanto/on-prem/state.json` | Wizard |

Verify with `memanto status`, then `agent create`, `remember`, `recall`, and
`answer`. The UI is `memanto ui` (default port 8000). The REST API is
`memanto serve`.[^onprem]

### Host Ollama on Apple Silicon

The wizard's default is a **Linux Ollama container**. Docker Desktop on macOS
does not expose Metal; the container is CPU-only and RAM-capped by the VM.
A 7B Q4 chat model can fail to load there while embeddings still succeed.

Operator workaround used on this machine (2026-08-30): install Ollama with
Homebrew (`brew services start ollama`), pull models on the host, stop the
bundled container, and point Moorcheh at the Mac:

```text
~/.moorcheh/config.json  embedding.base_url / llm.base_url
  → http://host.docker.internal:11434
```

Then recreate only the server:

```sh
moorcheh down --bundled-ollama
moorcheh up --use-host-ollama --no-configure
```

`moorcheh up` without `--bundled-ollama` uses host Ollama when
`127.0.0.1:11434` already answers. The compose file ships
`extra_hosts: host.docker.internal:host-gateway`. Docker Desktop may forward
that hostname to host localhost even when `ollama serve` binds `127.0.0.1`.

The wizard's LLM name is `qwen2.5`. Host `ollama pull qwen2.5:7b` does **not**
register `qwen2.5` / `qwen2.5:latest`. `/api/chat` then 404s (`model not
found`) after embeddings succeed. Fix with `ollama cp qwen2.5:7b qwen2.5` and
align `llm.model` plus `~/.memanto/on-prem/state.json` `llm_model`.

Do not run brew Ollama and `moorcheh-ollama` on port 11434 at once.

## OKF workflow

Keep editing this bundle with the knowledge skill. Refresh Memanto when
semantic recall should see new or changed records — not on every commit.[^skill]

### OKF → Memanto (load the index)

```sh
memanto migrate okf ./knowledge --dry-run
memanto migrate okf ./knowledge --agent AGENT
```

`migrate okf` walks a bundle (or a single `.md`), maps nodes onto Memanto's
thirteen types, and batch-writes the active (or `--agent`) namespace. If the
tree has a `memories/` folder, only that folder is imported — this checkout
does not, so the whole `knowledge/` tree is in scope. Unmapped fields go into
a `[Supporting data]` footer. Domain types such as `Decision` or
`Implementation Plan` are auto-classified; the original `type` is kept in the
footer. v0.1 still imports.[^okf-docs][^migrate-docs]

This is **not** documented as upsert-by-concept-id. Re-running after edits
can duplicate unless identity is carried in `x_memanto` from a prior Memanto
export. Preview with `--dry-run` first.[^migrate-docs]

Langfuse's migrate path is a ledgered, re-run-safe sync. OKF is not.[^migrate-docs]

### Memanto → files (export a wiki)

```sh
memanto memory export --okf
memanto memory sync --okf --project-dir ./some-project
```

Export writes `~/.memanto/exports/_okf/`. `sync --okf` runs a **fresh export**
and copies it to `<project>/okf/` for browsing or git — it does **not** pull
this repository's `knowledge/` into Moorcheh. Without `--okf`, sync writes
`MEMORY.md`.[^okf-docs][^sync-docs]

Round-trip Memanto → OKF → Memanto preserves extras under `x_memanto`. A
foreign Okmate bundle will not be byte-identical after import: types differ,
and Memanto does not store inter-memory edges (body links stay as text).[^okf-docs]

## Day-to-day commands

After the stack is up:

```sh
memanto status
memanto agent list
uv run memanto ui

memanto remember "…" --type fact
memanto recall "…"
uv run memanto answer "…"
```

Use `answer` only when the chat model is loaded on host Ollama (or a cloud
LLM). Use `recall` to test embeddings without generation.

## What this does not do

- Watch `knowledge/` or git.
- Replace `okmate check` / inspect / graph.
- Keep OKF `type`, owners, sources, or typed links as first-class Memanto
  columns.
- Make Memanto the place to edit architecture. Write in the bundle; treat
  Memanto as a disposable semantic index.[^census][^skill]

[^okf-docs]: Official Memanto OKF guide: interchange role, export layout, migrate import, sync --okf, field mapping, and x_memanto round-trip.
[^migrate-docs]: `memanto migrate okf` is a local bulk import; Langfuse migrate is the ledgered re-run-safe sync.
[^sync-docs]: `memanto memory sync --okf` exports Memanto memories into a project `okf/` tree after a fresh export.
[^onprem]: On-prem wizard, Moorcheh Docker, default Ollama models, state paths, and verify commands.
[^census]: Class B projection versus asserted OKF-canonical store; Memanto row and caveats.
[^landscape]: Semantic memory tools versus OKMate's lexical review layer.
[^skill]: Author and check this repository's `knowledge/` bundle with okmate.
