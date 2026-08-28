---
type: Research Report
title: Review-queue authoring, prompt query, and colocated code
description: Agents do all Markdown authoring from prompts; query can stay bundle-only, while authoring and research need the colocated git repo that provenance already walks up to from knowledge/.
tags: [domain/okmate, domain/okf, concern/review, concern/agents, concern/authoring, concern/retrieval, concern/developer-experience]
status: draft
generated: { by: process:cursor, at: 2026-08-28T17:50:00Z }
stale_after: 2026-11-28
authority: exploratory
owners: [human:nils]
sources:
  - id: review-engine
    resource: ../../../okf/src/review.rs
    title: classify_concept_action and ActionKind
    author: process:git
    last_modified: 2026-08-28
  - id: validate
    resource: ../../../okf/src/validate.rs
    title: Lifecycle, sources, and provenance diagnostics
    author: process:git
    last_modified: 2026-08-28
  - id: diagnostic
    resource: ../../../okf/src/diagnostic.rs
    title: Diagnostic codes
    author: process:git
    last_modified: 2026-08-28
  - id: queue
    resource: ../../../templates/fragments/queue.html
    title: Review queue Askama fragment
    author: process:git
    last_modified: 2026-08-28
  - id: article
    resource: ../../../templates/fragments/article.html
    title: Concept meta and review alert
    author: process:git
    last_modified: 2026-08-28
  - id: views
    resource: ../../../src/views/mod.rs
    title: Review rows and action ranking
    author: process:git
    last_modified: 2026-08-28
  - id: governance
    resource: ../../../src/views/governance.rs
    title: Concept meta alerts from classifier
    author: process:git
    last_modified: 2026-08-28
  - id: settings-http
    resource: ../../../src/http/settings.rs
    title: Loopback settings POST and Datastar morph
    author: process:git
    last_modified: 2026-08-28
  - id: http
    resource: ../../../src/http/mod.rs
    title: Live preview routes; settings is the only POST mutation
    author: process:git
    last_modified: 2026-08-28
  - id: search-rs
    resource: ../../../okf/src/search.rs
    title: Lexical metadata and heading search
    author: process:git
    last_modified: 2026-08-28
  - id: cli
    resource: ../../../src/cli.rs
    title: okmate check, inspect, search stay single-root
    author: process:git
    last_modified: 2026-08-28
  - id: goto
    resource: ../../../assets/goto.js
    title: Cmd-K page palette over pages.json
    author: process:git
    last_modified: 2026-08-28
  - id: agents-md
    resource: ../../../AGENTS.md
    title: Okmate agent instructions at repository root
    author: process:git
    last_modified: 2026-08-28
  - id: skill
    resource: ../../../.agents/skills/manage-okmate-knowledge/SKILL.md
    title: Manage okmate knowledge skill
    author: process:git
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
  - id: rust-vs-rocci
    resource: ../okf/okf-viewer-rust-vs-rocci.md
    title: OKF viewer Rust HTML versus finished Rocci shell
    author: process:cursor
    last_modified: 2026-08-26
  - id: dashboard
    resource: ../../plans/okmate/dashboard-parity.md
    title: Dashboard parity; queue writes out of bound
    author: process:cursor
    last_modified: 2026-08-26
  - id: verify-plan
    resource: ../../plans/okmate/verify-promote.md
    title: Verify and promote from the review UI
    author: process:cursor
    last_modified: 2026-08-28
  - id: git-host
    resource: ../../decisions/git-repository-bundles.md
    title: Git working trees as the v1 authoring host
    author: process:cursor
    last_modified: 2026-08-28
  - id: gaps
    resource: okf-tool-gaps.md
    title: OKMate feature gaps versus the OKF tool ecosystem
    author: process:cursor
    last_modified: 2026-08-28
  - id: tools
    resource: ../okf/okf-tools-and-workflows.md
    title: State-of-the-art OKF tools and workflows
    author: process:codex
    last_modified: 2026-08-28
  - id: overview
    resource: ../../architecture/system-overview.md
    title: OKMate system overview
    author: process:cursor
    last_modified: 2026-08-27
  - id: multi-roots
    resource: ../../plans/okf/multi-knowledge-roots.md
    title: Multiple knowledge roots
    author: process:cursor
    last_modified: 2026-08-25
  - id: cursor-cli
    resource: https://cursor.com/docs/cli/headless
    title: Cursor Agent CLI headless print mode
    author: organization:cursor
  - id: cursor-sdk
    resource: https://cursor.com/docs/sdk/typescript
    title: Cursor TypeScript SDK Agent.create
    author: organization:cursor
  - id: claude-cli
    resource: https://docs.anthropic.com/en/docs/claude-code/headless
    title: Claude Code programmatic print mode
    author: organization:anthropic
  - id: claude-sub
    resource: https://support.claude.com/en/articles/11145838-use-claude-code-with-your-pro-or-max-plan
    title: Claude Code with Pro or Max subscription
    author: organization:anthropic
  - id: codex-cli
    resource: https://developers.openai.com/codex/cli/reference
    title: Codex CLI including exec
    author: organization:openai
  - id: pi
    resource: https://pi.dev/
    title: Pi coding agent
    author: human:mario-zechner
---

# Review-queue authoring operations and agent dispatch

## Purpose and authority

This report designs how the OKMate review UI could *resolve* the work it
already classifies, and how a prompt box can **ask** or **author** without
becoming a Markdown editor. It is exploratory. Queue writes, in-UI agent
jobs, and review approve/comment are explicitly later work in current plans;
they are not shipped.[^extract][^dashboard][^okf-app]

The intended product posture: the queue and a global prompt are a
**workbench dispatcher**. Typical bundle edits go to a coding agent that
already knows OKF (skills, `AGENTS.md`, `okmate check`). Query can stay on
the bundle. Authoring and research usually need the **git repository that
contains the bundle**, which provenance already discovers. A few trusted
human acts stay in the UI as finite mutations. The browser is not the domain
store.[^rust-vs-rocci][^skill][^validate]

## Current behavior

`okf::classify_concept_action` maps each concept plus bundle diagnostics to an
`ActionKind`, a label, a detail string, and `is_action_required`. The live
`/review/` page renders a Needs action table, a filterable all-concepts table,
and a diagnostics list. Concept pages show the same action as an alert banner
copied from the classifier detail.[^governance] There is no button that writes
a record. The only live POST that mutates state is `/__okmate/settings`
(loopback, re-read, morph `#okmate-settings`).[^review-engine][^queue][^article][^http][^settings-http]

Priority order today:[^views]

| Rank | `ActionKind` | Queue label | Required? |
| --- | --- | --- | --- |
| 0 | `FixErrors` | Fix Errors | yes |
| 1 | `ReverifySources` | Re-verify | yes |
| 2 | `ReverifyRegenerated` | Re-verify | yes |
| 3 | `RefreshStale` | Refresh Stale | yes |
| 4 | `UncommittedChanges` | Commit Evidence | yes |
| 5 | `UntrackedSources` | Track Evidence | yes |
| 6 | `InitialVerification` | Verify | yes |
| 7 | `PendingPromotion` | Verify | yes |
| 8 | `Exploratory` | Exploratory | no |
| 9 | `Clean` | Stable / status | no |

That classifier is the operation catalog. Do not invent a second "issue type"
in the UI. Bind each row's primary control to `ActionKind`.[^review-engine]

## What "resolved" means

A row leaves Needs action when a subsequent `okf` load classifies it as
`Exploratory` or `Clean`. The UI should not mark a row done from local JS.
After any mutation or agent job: reload the workspace, re-run classification,
morph `#okmate-queue` (and the concept meta region if that page is open). That
is the settings POST shape applied to the queue.[^settings-http][^rust-vs-rocci]

Git-root snapshots stay read-only. Authoring only targets **directory roots**
that resolve to a writable checkout. Tokens never appear in `roots`
JSON.[^multi-roots][^overview]

## Three resolution classes

Keep these distinct in the UI. Mixing them into one "Fix" control is the
failure mode.

### 1. Direct mutation (okmate writes YAML)

Finite, reversible, no model. Same contract as settings: validate, write,
re-read, morph. Loopback-only.

Use this only where the human *is* the actor the metadata records. Agents
must not invent `human:` verification events or promote a draft to
`stable`.[^skill]

### 2. Git operation

`OKF4007` / `OKF4008` are repository state, not prose. Resolution is `git add`
or `git commit` on cited source paths. Prefer dispatching an agent (it already
runs git in a harness) over teaching okmate to commit. A dedicated "stage
these paths" button is optional later; it is not the first UX.[^validate]

### 3. Agent job (typical)

Anything that needs judgment, multi-file Markdown, source/footnote alignment,
or a skill-following authoring loop. Okmate builds a prompt from the row,
spawns an adapter, streams progress, then reloads the bundle. The agent
writes ordinary Markdown; `okmate check` judges.[^skill][^gaps][^tools]

Do **not** add an in-app frontmatter form or a WYSIWYG body editor for these.
Peers that write from MCP or `--fix` fight the "agents edit Markdown; the
tool judges" posture unless gated. The queue should gate, not compete.[^gaps]

## Prompt surfaces: ask versus author

The human-facing authoring tool is a **prompt**, not a record editor. Two
jobs share a text box and must stay distinguishable. Confusing them is how
a "chat in the wiki" becomes an accidental write, or how a research task
runs with no view of `src/`.

| Job | What the user wants | What it may write | What it must see |
| --- | --- | --- | --- |
| **Navigate** | Open a known page | nothing | `pages.json` titles/routes |
| **Ask** | A question about what the bundle already claims | nothing (answer + citations) | one or more OKF roots |
| **Author** | Change or create records | Markdown under the bundle | git repo that contains the bundle, plus the bundle |
| **Verify / Promote** | Sign a revision | YAML `verified` / `status` only | the concept; no model |

Cmd-K today is Navigate (`goto.js` over `pages.json`). Keep it. Do not
overload it with an LLM. Ask is a separate control: a field on home, a
panel, or "Ask about this concept" on the article. Author is the queue
primary/overflow plus an explicit **Author from prompt** (new record, or
"turn this answer into a draft").[^goto][^cli]

Default the empty box to **Ask**. Switching to Author should be a visible
mode, not inferred from wording. "What did we decide about ports?" is Ask.
"Write research on prompt query" is Author. A follow-up on an Ask result
can be **Save as draft** (Author job seeded with the question and answer).

### Ask is easier and bundle-only

Ask does not need the colocated code tree. `okmate search`, `inspect
catalog|concept|graph`, and `build`'s `search.json` already retrieve from
one root. The UI can run those deterministically, then optionally compose
an answer. Git snapshot roots are valid Ask targets. Writes are
forbidden.[^search-rs][^cli][^multi-roots]

Implementation ladder (ship in this order):

1. **Deterministic retrieve.** Prompt → `okmate search` (and maybe inspect
   of top hits) → list of concepts with snippets and links. No model. This
   is already the engine contract; the missing piece is an Ask UI that is
   not the review-table filter and not Cmd-K.
2. **Compose over retrieval.** Same hits, then a short LLM pass whose only
   context is those chunks plus a "cite concept ids" instruction. Lane A
   (API) is acceptable here: no repo tools, no `--force`. Read-only CLI
   (`claude -p` without Edit, Codex default read-only sandbox) also works.
3. **Read-only agent** only when the question needs graph walks, multiple
   `inspect` calls, or comparison across configured roots (`okmate roots
   --format paths` then one search per path). Still no writes.

Ask success is a cited answer. Remorph the queue only if the user then
starts an Author job. Default presentation is Tier 2 (the answer *is* the
product). Collapse tool traces.

Multi-root Ask fans out the same way agents already must: do not merge
catalogs. Show which root each citation came from.[^cli][^overview]

Questions that sound like Ask but are really Author/research: "Does this
architecture record still match the code?" That needs the git tree. Put a
**Check against sources** control on the concept (Author, repo cwd), not
in the default Ask path.

### Author and research need the colocated git repo

OKMate's intended layout is **not** a standalone wiki folder. A knowledge
bundle lives inside a software repository:

```text
<repo>/                 # git toplevel (cwd for Author agents)
  AGENTS.md
  .agents/skills/…
  src/ … crates/ …
  knowledge/            # OKF root (okmate view / check path)
    index.md
    architecture/…
```

This is how this repository, Rocci, and typical `okmate view knowledge`
sessions already work. Provenance does not treat `knowledge/` as the git
root: `git_repository_root` runs `git rev-parse --show-toplevel` from the
bundle path and walks up. Cited `resource` paths are relative to the
record and routinely point at sibling source (`../../../src/review.rs`).
Drift (`OKF4006`) is "that source file in the **repository** changed after
verification," not "a file inside the bundle changed."[^validate][^skill][^agents-md]

Configured directory roots are therefore usually `…/knowledge`, not the
repo. Git roots already have a `bundle` subdirectory for the same
split.[^multi-roots]

If an Author agent is spawned with `cwd = knowledge/`:

- `AGENTS.md` and `.agents/skills` at the repo root may not load.
- The agent cannot read `src/` without `../` guesses.
- `cargo test` and repo hooks are the wrong working directory.
- Refresh-stale and "reconcile drift" cannot check the evidence the
  classifier named.

**Author cwd must be the git toplevel** when `git_repository_root(bundle)`
succeeds. Pass the bundle path in the prompt (`okmate check knowledge
--profile strict`, edit only under `knowledge/`). If there is no git
toplevel (orphan directory bundle), fall back to the bundle path and warn
that source citations to code will be blind.

CLI harnesses are the right Author adapter *because* they already assume a
project checkout: skills, ignore files, sandbox `workspace-write` on that
tree. A Messages API call with the bundle files concatenated is the wrong
Author shape; it can still be a fine Ask composer (step 2 above).[^cursor-cli][^claude-cli]

Research prompts ("investigate how X works, write a report") are Author
jobs with a research-shaped envelope, not Ask. They depend on code even
more than a queue Fix Errors. Same cwd rule.

Git snapshot roots (fetched URL cache): Ask later; Author no. v1 authoring
is local git working trees only; infer the bundle from `okf_version`
`index.md` at the repo root or one directory down, preferring a unique
`knowledge/` child.[^git-host]

## Operation catalog

Each row needs one **primary** control (the classifier label) and an overflow
**Ask agent…** that always exists. Concept pages duplicate the same primary
control under the alert banner so a reviewer who opened the record can act
without returning to the table.[^article]

### Fix Errors

**Trigger:** any error diagnostic on the concept (`OKF1xxx`/`OKF2xxx` parse
and schema, `OKF3001`–`OKF3005` graph, `OKF2007`/`OKF2009` Markdown).[^diagnostic][^review-engine]

**Primary:** Ask agent to fix. **Not** a YAML field picker.

**Required input:** concept id, root id, diagnostic codes and messages
(already in `action_detail`). Writable directory path.

**Optional input:** extra instruction ("leave custom keys", "do not touch
body"). Adapter override. Whether to run `okmate check --profile strict`
after (default on).

**Resolution:** agent edits the record (and maybe `index.md` / `log.md`).
Success = zero errors on that path after reload. Human verify is a *later*
row if the record is still draft.

### Refresh Stale (`OKF4004`)

**Primary:** Ask agent to refresh.

**Required:** concept id, root, `stale_after` date, title/description.

**Optional:** new `stale_after` hint (default: today + 90 days, matching
skill records); "refresh claims vs sources" vs "extend date only". Extending
the date without reading the body is a smell; default to a refresh job.

**Resolution:** agent revises body and sources, bumps `generated.at`, sets
`status: draft`, does **not** append `human:` verified. Row typically becomes
`InitialVerification` or `PendingPromotion`, which is correct.[^skill]

### Re-verify sources (`OKF4006`)

Two honest outcomes: the human still agrees (append verification), or the
record is wrong (agent updates claims/sources, then human verifies).

**Primary:** Ask agent to reconcile drift. Secondary: **Accept current
sources** (direct verify) after the human has opened the concept.

**Required for agent:** concept id, drifted source ids and git paths from the
diagnostic. **Required for accept:** configured `owners` actor (see Verify).

**Optional:** note ("reviewed path X, still accurate").

Do not silently treat drift as a status flip. The classifier exists because
git changed after `verified.at`.[^validate]

### Re-verify regenerated (`OKF4005`)

Generated content is newer than last human verification.

**Primary:** Open concept (already a click) plus **Verify** if the human
accepts, or **Ask agent** if they want a rewrite. Default the overflow to
agent when the human is not ready to sign.

**Required for verify:** owner identity. **For agent:** concept id.

### Track Evidence / Commit Evidence (`OKF4007` / `OKF4008`)

**Primary:** Ask agent to stage or commit the cited paths (and only those).

**Required:** source paths from diagnostics, repo root.

**Optional:** commit message; "stage only" vs "commit". Default stage-only
unless the user has enabled auto-commit in settings.

Okmate should not become a git GUI. If the adapter cannot run git (sandbox),
show the exact `git add` paths and fail clearly.

### Verify / Pending promotion

These look the same in the queue ("Verify") but they are not the same
write.[^review-engine]

| Kind | Meaning | Direct write | Agent? |
| --- | --- | --- | --- |
| `InitialVerification` | No `human:` event yet | Append `verified` event. Leave `status: draft` unless the user also chooses Promote. | Agent may improve the draft first; it must not write `human:`. |
| `PendingPromotion` | Already human-verified, still `draft` | Optional **Promote** (`status: stable`) after the human confirms the revision. | Agent should not promote. |

**Required for Verify:** actor string the product trusts, e.g. `human:nils`
from settings (not inferred from git `user.name` without confirmation).
Timestamp is server-now RFC 3339.

**Optional:** Promote in the same dialog (checkbox, default off). Comment
stored only if we later add a decision log; v1 can skip comments and keep
git history as the narrative.[^okf-app]

**Never:** overwrite the `verified` list; always append. Fail if the file
changed on disk since the page was rendered (compare-and-swap on
`generated.at` or file mtime).[^okf-app]

Exploratory drafts (`status: draft` + `authority: exploratory`) are
intentionally not required. Do not offer Promote there as a primary
control.[^review-engine]

### Clean / Exploratory

No primary mutation. Overflow Ask agent still useful ("tighten this
research") but that is authoring, not queue clearance.

## Inputs the UI must collect

Shared on every mutating control:

| Input | Required? | Source |
| --- | --- | --- |
| Concept id | yes | row |
| Root id / writable path | yes | workspace member; refuse git snapshots |
| `ActionKind` | yes | classifier |
| Adapter (for agent jobs) | yes if job | settings default, overridable per run |
| Extra instruction | no | textarea, empty default |
| Human actor | yes for Verify/Promote | settings `owners`-style id |
| Promote? | no | checkbox, default off |
| New `stale_after` | no | date field on Refresh |
| Commit vs stage | no | Track/Commit jobs |
| Show full transcript | no | sticky preference, see below |

Do not ask the user to retype diagnostic text. Pre-fill the agent prompt
from `action_detail`, codes, and `okmate inspect concept`.

A good prompt envelope (okmate-owned, not the adapter):

**Ask:** cwd unused or bundle; tools read-only; retrieve with `okmate search`
/ `inspect` first; answer with concept-id citations; do not write files.

**Author / queue fix / research:**

1. Working directory = `git_repository_root(bundle)` when it exists, else
   the bundle path.
2. Bundle path and concept id in the prompt. Instruction: follow
   `$manage-okmate-knowledge`; edit Markdown under the bundle only; do not
   mint `human:` verification; leave `status: draft`.
3. Diagnostics or the user's author prompt.
4. Success criterion: `okmate check <bundle> --profile strict` (Author) or
   the concept's errors gone (queue fix).
5. Optional user note.

## Agent dispatch: API versus CLI

Two lanes. The product needs both. Do not pick one as "the" integration.

### Lane A — Direct API / SDK + token

Examples: Cursor SDK (`Agent.create` / `Agent.prompt` with
`CURSOR_API_KEY`), Anthropic API, OpenAI API. Straightforward for a server
that already holds a secret: one HTTP or SDK call, structured `Run` objects,
cancel, resume.[^cursor-sdk]

**Fits:** CI, hosted okmate, users who already buy API credits.

**Fails for many customers:** Pro/Max/Cursor subscription users often have
**no** API key, or must not set `ANTHROPIC_API_KEY` because it *overrides*
the subscription and bills Console instead of the plan. Direct API is the
wrong default for "I already pay for Claude/Cursor/ChatGPT".[^claude-sub]

Rust embedding `@cursor/sdk` or the Python SDK also pulls a second runtime
into a native binary. Prefer spawning a process even for Lane A, or a thin
sidecar, rather than linking Node/Python into `okmate`.

### Lane B — CLI harness (preferred default on the desktop)

The same agents people already run in a terminal, with login-session auth,
skills, `AGENTS.md`, MCP, sandboxes, and permission prompts. Headless flags
are enough to drive from okmate:

| Adapter | Invoke (illustrative) | Auth | Writes | Output |
| --- | --- | --- | --- | --- |
| Cursor CLI | `agent -p --force --output-format stream-json "…"` | `agent login` or `CURSOR_API_KEY` | `--force` / `--yolo` required to apply | `text` / `json` / `stream-json`[^cursor-cli] |
| Claude Code | `claude -p "…" --allowedTools "Read,Edit,Bash" --output-format stream-json` | `claude login` (unset `ANTHROPIC_API_KEY` to stay on Pro/Max) | `--allowedTools` or a permission mode | `text` / `json` / `stream-json`[^claude-cli][^claude-sub] |
| Codex | `codex exec --sandbox workspace-write "…"` | ChatGPT login or `OPENAI_API_KEY` | sandbox flag; default exec is read-only | stderr progress, stdout final; `--json` JSONL[^codex-cli] |
| Pi | `pi -p "…"` / `--mode json` | `/login` or provider API keys | built-in read/write/edit/bash | print, JSON, RPC, SDK[^pi] |

**Why CLI is the better desktop DX:**

- Uses the plan the human already bought.
- Loads project skills (including `manage-okmate-knowledge`) and hooks the
  harness already has. A raw Messages API call does not.
- Permission and sandbox models exist (`--allowedTools`, Codex sandbox,
  Cursor `--force`). Okmate should not invent a second allowlist.
- Resume (`claude --continue`, `codex exec resume`, `agent resume`, `pi -c`)
  maps to "ask a follow-up on this queue job".

**CLI costs:** binary on `PATH`, version skew, TTY-vs-print auth bugs, and
unattended permission. Okmate must pass non-interactive allow flags or the
job hangs. Surface that in settings: "this adapter will write files without
asking" vs "this adapter will fail if it needs approval".

### Adapter interface (okmate-owned)

Do not special-case four SDKs in the queue template. One process adapter:

```text
spawn { argv, cwd, env, timeout }
→ stdout/stderr bytes (optionally NDJSON events)
→ exit code
→ cancel = kill process group
```

`cwd` is job-class specific: unused or bundle for Ask; git toplevel for
Author. Do not pass the bundle path as cwd "because that is what `view`
opened."

Settings store named adapters:

- `cursor-cli`, `claude`, `codex`, `pi`
- `custom` with argv template `{prompt}`, `{cwd}`

Do not add a Lane A SDK adapter. Desktop dispatch is CLI harnesses
only.[^git-host]

Detect which binaries exist; disable missing ones. Remember last successful
adapter per machine, not per bundle.

ACP (`agent acp` and similar) is a possible later unifier. Do not block v1
on it; argv print-mode is enough.

### Prompt vs tools

Okmate should **not** expose a write MCP from the viewer for queue jobs. The
CLI agent already has filesystem tools. The machine contract for *judgment*
stays `okmate check` / `inspect`. The UI is another client of those
commands, not a second write path.[^rust-vs-rocci][^tools]

## Presenting results

Match presentation to job class. Always re-morph the queue; the transcript
is extra.

### Tier 0 — Outcome chip (every job)

After reload: the row disappeared, moved, or still requires action. One
sentence: "Errors cleared" / "Still draft; Verify remains" / "Check failed:
OKF2003". Plus duration and adapter name. This is the Datastar morph people
feel first.[^rust-vs-rocci]

### Tier 1 — Summary (default for Verify, Promote, Track/Commit)

No model essay. Show: files touched (from git status or adapter json),
`okmate check` counts, maybe a three-line agent coda if present. Collapsed
"log" disclosure.

Direct mutations skip the agent log entirely: "Appended verification as
`human:nils`". That is the whole result.

### Tier 2 — Formatted harness output (default for Fix / Refresh / Reconcile)

Users compare this to running `claude -p` themselves. Render the adapter's
**final assistant text** as Markdown in a job panel (`#okmate-job`). Prefer
the harness's own formatting over a custom tool-call UI in v1.

If the adapter spoke `stream-json`, stream into that panel while running
(live SSE; this is the case that justifies a job, not a one-shot
POST).[^extract][^rust-vs-rocci] Allow cancel. On completion, keep the
transcript until the user dismisses it; do not navigate away.

### Tier 3 — Full trace (opt-in)

Tool calls, stderr, token usage. Behind "Show trace". Needed when a job
fails or the user is debugging an adapter. Do not make this the default;
it is noisy and looks like an IDE, which is the wrong comparison for a
review queue.

### Progressive rule

| Job | Default pane |
| --- | --- |
| Direct Verify / Promote | Tier 0 + 1, no panel |
| Ask (question) | Tier 2 answer + citation links; no queue morph |
| Git stage/commit via agent | Tier 1; Tier 2 if exit ≠ 0 |
| Fix Errors, Refresh, source reconcile, Author from prompt | Tier 2 streaming |
| User checked "verbose" | Tier 3 |

A **sticky per-user preference** ("Always show full reply") covers people
who want every job to look like the CLI. Default off.

Failed jobs: keep the transcript, do not claim the queue updated, offer
Retry and Open in terminal (copy the exact argv). Never auto-retry in a
loop.

## UX recommendations

**Ask box.** Separate from Cmd-K. Default mode Ask. Visible switch to
Author. Concept pages may pre-scope Ask to that record (`inspect` + body)
without changing cwd rules. Citations are ordinary in-app links.

**Queue row.** Primary pill is already colored. Turn the required-action
pill into a button (or put a button beside it). `Exploratory` / `Clean` stay
pills. Overflow `⋯` → Ask agent, Open concept, Copy inspect command.

**Do not batch-promote.** A "clear all drafts" control would violate
verification policy. Batch "Ask agent to fix all errors" can exist later
with a confirm that lists concept ids; v1 is one job, one concept.

**Settings first.** Before the first Ask agent: pick default adapter, human
actor, and "allow writes" (maps to `--force` / `--allowedTools` /
`workspace-write`). Missing actor blocks Verify, not agent jobs.

**Progress.** Settings-style one-shot POST is enough for Verify. Agent jobs
need a server-owned job id, SSE into `#okmate-job`, and a queue row spinner
keyed by concept id. Full page reload after "run" is the named failure
mode.[^rust-vs-rocci]

**Static `okmate build` HTML** must not grow these controls. Mutations belong
to live preview / desktop, same as settings. Built sites stay review
documents.[^extract]

**DX for agents outside the window.** The same fix a human triggered from
the queue must be expressible as a prompt + `okmate check`. Publish the
envelope in `/agents/` later; do not make the only path a hidden UI POST.

## Constraints that should not move

- Records stay inert Markdown. No Rocdown in `knowledge/`.[^overview]
- `okf` stays UI-neutral; classification stays in the engine.[^review-engine]
- Loopback-only writes; tokens never echoed.[^settings-http][^extract]
- Directory roots writable; git URL cache snapshots stay read-only.
  v1 writes require a local git working tree.[^multi-roots][^git-host]
- Ask is bundle-only and read-only; Author cwd is git toplevel when
  present.[^validate][^search-rs]
- Agent jobs, when built, use CLI harnesses, not vendor APIs.[^git-host]
- Append verification; do not invent `human:` from an agent.[^skill]
- One-shot morph first; live SSE only while a job runs.[^extract]
- Do not ship write-capable MCP as the queue's backend.[^gaps]

## Non-goals for a first implementation

- In-browser Markdown editing or frontmatter forms.
- Replacing Cmd-K goto with the Ask model.
- Merged cross-root search catalogs.
- GitHub PR approve/request-changes mapping (named in the older application
  plan; optional later adapter).[^okf-app]
- Hosted multi-user review with auth tokens.
- Auto-promotion after a green check.
- Embedding Node/Python SDKs inside the `okmate` binary.

## Suggested first slice

Verify/Promote on a git working tree is specified in
[verify and promote](/plans/okmate/verify-promote.md): infer bundle, actor
in settings, loopback POST, CAS, morph. Ask UI and CLI Author jobs come
after that plan. Do not add API-key SDKs.[^verify-plan][^git-host]

[^review-engine]: Action kinds, labels, required flag, diagnostic-driven details.
[^validate]: OKF4004–4008 stale, regenerated, drift, untracked, dirty sources; `git_repository_root` is `rev-parse --show-toplevel`.
[^diagnostic]: Stable diagnostic code list.
[^queue]: Read-only queue tables and filters.
[^article]: Concept alert is display-only.
[^views]: Ranked Needs action rows.
[^governance]: Meta alert copies `action.detail`.
[^settings-http]: Validate, write, re-read, PatchElements; loopback.
[^http]: No review POST today.
[^search-rs]: Metadata and heading chunk search; not BM25.
[^cli]: `check` / `inspect` / `search` stay single-root; `roots` lists paths.
[^goto]: Live Cmd-K is title/route filter over `pages.json`.
[^agents-md]: Repo-root agent instructions, not inside the bundle.
[^skill]: Agents author Markdown, strict check, never invent human verification.
[^extract]: Queue writes and in-UI agent jobs out of bound for the extract plan.
[^okf-app]: Revision-bound approve and CAS metadata commit as later phase.
[^rust-vs-rocci]: Finite queue mutations vs live agent jobs; morph vs SSE.
[^dashboard]: Queue writes out of bound for dashboard parity.
[^verify-plan]: First implementation: discover bundle, surgical YAML, live POST.
[^git-host]: Git working tree only; one-level okf_version discovery; CLI not API.
[^gaps]: Read-only authoring posture versus peer write MCP / lint --fix.
[^tools]: Closed evidence loop; humans review; agents submit Markdown.
[^overview]: Engine vs app vs knowledge ownership.
[^multi-roots]: Writable directory roots; git snapshots; no tokens in JSON.
[^cursor-cli]: `agent -p --force` and output formats.
[^cursor-sdk]: `CURSOR_API_KEY` local/cloud Agent runs.
[^claude-cli]: `claude -p` allowedTools and json/stream-json.
[^claude-sub]: Subscription vs ANTHROPIC_API_KEY billing override.
[^codex-cli]: `codex exec` sandbox and json.
[^pi]: `pi -p` / JSON / RPC harness.
