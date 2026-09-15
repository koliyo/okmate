# Authoring an OKF bundle

This guide is for starting or reshaping a knowledge bundle. It does not
require Okmate’s own `knowledge/` layout or the in-repo agent skill. Those
are local conventions for this product, not OKF rules.

Open Knowledge Format (OKF) v0.2 leaves taxonomy and directory names to the
producer. Concept identity is the path without `.md`. `type` is an open
vocabulary. Choose structure from the job the corpus has to do.

## Start from reader questions

Before creating folders, write down a few questions a reader or agent should
be able to answer, and the kind of record that answers each one.

| Reader job | Typical record | What the body must do |
| --- | --- | --- |
| Understand a mechanism | Explanation | Model, tradeoffs, limits, a worked example |
| Complete a repeatable goal | How-to Guide | Prerequisites, steps, success check, recovery |
| Look up a stable interface | Reference | Scope, fields or options, examples |
| Evaluate evidence | Research Report | Question, method, findings, uncertainty |
| Understand an observed state | Audit | Environment, method, findings, date |
| Recover an adopted choice | Decision | Choice, alternatives, consequences |
| Respond to an operational trigger | Runbook | Trigger, steps, rollback, owner |
| Coordinate recurring work | Workflow | Roles, stages, handoffs |
| Describe a data asset | Dataset, Table, Metric | Identity, grain, owners, related assets |

Add a type only when the distinction changes how someone uses the record. A
Workflow is not a How-to Guide with a different label. A dated audit is not
team policy.

## Scope before folders

Give the bundle one audience, one maintenance responsibility, and a boundary
for what it will not contain.

- Product contracts stay in the product’s corpus.
- Private host operations stay in an operations corpus.
- General practice can live in a handbook that *cites* product evidence
  instead of copying it.
- A generated projection (export, build output, compiled wiki) is not the
  canonical authored tree. Edit the source of truth.

If two groups cannot share owners, release cadence, or secrecy, they are two
bundles, not two folders in one root.

## Find a bundle in a repository

A versioned bundle root is a directory whose `index.md` declares
`okf_version`. Collection indexes without that marker are not bundles.
Directory names (`knowledge/`, `docs/`, `.okf/`) are not identity.

```sh
okmate discover .
okmate discover docs --format json
okmate view docs
```

`discover` never writes registry entries. Pass an explicit bundle path to
`check`, `inspect`, `search`, `build`, `concept`, and `index`. If a
container has several versioned roots, `view` lists them and requires a
choice; it does not flatten them into one corpus or pick `knowledge/`
because of the name. Mixed-scope preview keeps each root labeled
(separated navigation is the default).

## Collection, type, and authority

These axes are independent. Do not encode all three in the folder name.

| Axis | Question | Not the same as |
| --- | --- | --- |
| Collection / navigation | Where does a reader start? | The record’s type |
| Type | What kind of reusable thing is this? | Whether it is binding |
| Authority | Advice, observation, adopted rule, or history? | How recently it was edited |
| Tags | Which other subjects it intersects | Identity or location |
| Lifecycle / evidence | Is it current, and what supports it? | Folder name |

A Research Report can live under a topic folder. A How-to Guide can cite an
Audit without becoming that audit. `authority: exploratory` is not an
adopted Decision.

## Research versus reusable guidance

Keep dated investigations as research or audits. When a finding is still
useful, write a maintained guide that links to that evidence and states
when it applies. Do not retitle a 2024 model comparison as timeless
practice, and do not mark guidance verified merely because it was moved.
The [engineering handbook example](examples/engineering-handbook/) is a
checked-in prototype of that split, with frozen root questions and a
retrieval file.

## Resource versus source

- `resource` on a concept points at the thing the record *is* (a table, a
  repo, a service).
- `sources[]` are citations for claims in *this* document. Each `id` should
  match a keyed footnote (`[^id]`) in the body.
- Not every URL needs its own concept. A source register can be one
  Reference with anchors.

## Generated projection versus authored knowledge

Canonical knowledge is inert Markdown you review in git. A static site, a
catalog export, or an agent index may be generated from it. Editing the
projection is lost on rebuild. Keep generated trees out of the authored
root, or treat them as derived output.

## Directory names and bundle boundaries

The directory name is not the bundle’s identity. Pass the actual path to
`okmate check`, `inspect`, `search`, `build`, and `view`.

| Placement | When it fits | Costs |
| --- | --- | --- |
| `knowledge/` | Curated knowledge beside source, tools, or other docs | Familiar default; not required |
| `docs/` | The documentation tree *is* the typed corpus | Ordinary untyped Markdown in the same tree becomes a validation problem |
| `.okf/` | Explicit machine-discoverable knowledge beside code | Hidden in ordinary browsing; discovery must be explicit |
| Repository root | A dedicated knowledge-only repository | `README.md`, `CONTRIBUTING.md`, and `AGENTS.md` are not automatically outside the corpus |
| `bundles/<name>/` or component-local roots | Distinct scopes, audiences, or owners | Needs explicit resolution; do not flatten a container of bundles into one root |

A whole-root rename keeps relative concept IDs but changes registry paths and
external links. Moving a record inside a root *changes its ID* and requires
updating links and consumers.

When more than one candidate exists, list them and choose explicitly. Do not
prefer `knowledge/` merely because of its name.

## Choose a starting layout

Worked examples live in [`examples/`](examples/). Each README states
purpose, navigation, and the validation profile that currently matches it.

| If the corpus is… | Start from | Profile to check today |
| --- | --- | --- |
| A few records, no taxonomy yet | [`examples/minimal`](examples/minimal/) | `base` |
| A software project’s work archive | [`examples/software-archive`](examples/software-archive/) | `strict` or `evidence` |
| An engineering handbook / practice library | [`examples/engineering-handbook`](examples/engineering-handbook/) | `base` |
| Operations and runbooks | [`examples/operations`](examples/operations/) | `base` |
| Datasets, tables, metrics | [`examples/data-catalog`](examples/data-catalog/) | `base` |

`base` is the portable reader. `evidence` asks for title, description,
generation, owners, and authority without a product type list.
`strict` is Okmate’s owners-and-evidence profile **plus** this
repository’s preferred types and `domain/` tags. Handbook, operations, and
data types are valid OKF; check them with `base` (or `evidence` once those
records actually carry evidence fields). See
[`compatibility.md`](compatibility.md).

Optional local style lives in `<bundle>/okmate.toml`, not in root
`index.md`. A missing file means no type/tag/path advice. Example:

```toml
version = 1
preferred_types = ["Explanation", "How-to Guide", "Runbook"]
preferred_tags = ["ops"]
authoring_guide = "README.md"

[type_paths]
Explanation = "explanations"
"How-to Guide" = "how-to"
Runbook = "runbooks"
```

Undeclared types (`OKMATE5001`) and path mismatches (`OKMATE5002`) are
style warnings. They never fail `check`. This file is distinct from
`~/.okmate/config.toml`.

## Current `okmate init`

`okmate init [path]` prints a create-only plan. `--apply` writes. The
default path is `knowledge`.

- Default scaffold: `index.md` and `log.md` (`--template minimal`). This
  changed from the previous six collection indexes.
- `--template software-project`: restore that previous layout
  (`architecture`, `decisions`, `status`, `plans`, `research`, `audits`).
- `--bare`: alias for `--template minimal`.
- `--collection DIR`: add a relative collection (repeatable). Cannot
  combine with `--bare`.
- `--template-file path.toml`: local `[[collections]]` data. Not fetched
  or executed. Cannot combine with `--template`.
- `--register` / `--id`: optional local registry entry. `--id` wins.
  Generic directory names such as `docs` prefer the repository or title.
- `--agents`: optional create-only agent routing files at the git
  toplevel, using the actual bundle path. Rejected for a repository-root
  bundle because `AGENTS.md` would sit inside the corpus.
- `--title`: heading for the root index.

```sh
okmate init
okmate init --apply
okmate init --template software-project --apply
okmate init . --bare --apply
okmate init docs --apply
okmate init --collection runbooks --apply
okmate init --apply --register --id my-bundle
okmate check knowledge --profile strict
okmate check path/to/bundle --profile evidence
okmate check path/to/bundle --profile base
```

## Ongoing records and indexes

`okmate concept` and `okmate index` are dry-run first. `--apply` writes.
They do not invent owners, citations, or human verification. Default
`--profile base` so a handbook type is legal without Okmate product
vocabulary. `okmate move` is the same plan-then-`--apply` model: it
relocates one concept file and rewrites path-based Markdown hrefs,
collection membership, and relative `sources[].resource` values. It
does not rewrite unlinked prose, change `type`, or append `log.md`.
`--to` ending in `/` keeps the filename stem.

```sh
okmate concept knowledge --type Explanation --id guides/onboarding
okmate concept knowledge --type Explanation --id guides/onboarding --title Onboarding --apply
okmate concept knowledge --type Runbook --id runbooks/restore --profile evidence \
  --title Restore --description "Recover a host." --authority descriptive \
  --owner human:nils --generated-by process:okmate --apply
okmate index knowledge
okmate index knowledge --apply
okmate move knowledge --from research/foo --to audits/
okmate move knowledge --from research/foo --to audits/renamed --apply
```

`--profile evidence` or `strict` fails with named missing flags instead of
filling them in. Index updates append missing nearest-directory links and
report unresolved `.md` hrefs. They do not reorder authored grouping or
replace a question-oriented index with an alphabetic dump.

Body templates exist for Explanation, How-to Guide, Reference, Research
Report, Audit, Runbook, Dataset/Table/Metric, and Workflow. Types and
folders stay independent.

## Evidence without copying this repository

A minimal record needs a `type` and a body. Title, description, owners,
citations, and human verification are evidence you add when the claim
requires them—not fields to invent so a template looks complete.

When you do cite a claim, keep the footnote id and `sources[].id` in sync.
Preserve unknown metadata; do not strip fields another tool stored.

## Maintenance

- Indexes are navigation, not an alphabetic dump. Preserve authored grouping
  and questions.
- Stale dates and verification events describe the record, not the folder.
- Validate the profile you actually mean: format errors, missing evidence,
  and local style advice are different kinds of finding. `base` is portable
  OKF, `evidence` adds owners without product vocabulary, and `strict`
  mixes evidence with Okmate types and `domain/` tags. Details:
  [`compatibility.md`](compatibility.md).
