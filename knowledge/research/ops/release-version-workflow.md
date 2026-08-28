---
type: Research Report
title: Release version workflow
description: promote tag is an environment-promotion leftover; the idiomatic operator surface is release with explicit patch/minor/major (or a pin), not commit-message auto-tag.
tags: [domain/ops, domain/okmate, concern/ci, concern/tooling]
status: draft
generated: { by: process:cursor, at: 2026-08-28T10:26:09Z }
stale_after: 2026-11-28
authority: exploratory
owners: [human:nils]
sources:
  - id: promote
    resource: ../../../okmate-ops/src/okmate_ops/release.py
    title: Version commit, wait Test, tag, tap
    author: process:git
    last_modified: 2026-08-27
  - id: version
    resource: ../../../okmate-ops/src/okmate_ops/version.py
    title: Shared crate version rewrite
    author: process:git
    last_modified: 2026-08-27
  - id: ops-cli
    resource: ../../../okmate-ops/src/okmate_ops/cli.py
    title: okmate-ops command list
    author: process:git
    last_modified: 2026-08-27
  - id: ops-plan
    resource: ../../plans/ops/okmate-ops.md
    title: okmate-ops uv toolkit
    author: process:cursor
    last_modified: 2026-08-27
  - id: readme
    resource: ../../../README.md
    title: OKMate README publish steps
    author: process:git
    last_modified: 2026-08-27
  - id: release-yml
    resource: ../../../.github/workflows/release.yml
    title: Tag-triggered Release workflow
    author: process:git
    last_modified: 2026-08-27
  - id: self-update
    resource: ../../plans/okmate/standalone-self-update.md
    title: Standalone Okmate self-update
    author: process:cursor
    last_modified: 2026-08-27
  - id: cargo
    resource: ../../../Cargo.toml
    title: Workspace and okmate package version
    author: process:git
    last_modified: 2026-08-27
  - id: agents
    resource: ../../../AGENTS.md
    title: Rolling dev tag and promote fetch
    author: process:git
    last_modified: 2026-08-27
  - id: devops
    resource: ../../../.agents/skills/okmate-devops/SKILL.md
    title: DevOps skill promote tag notes
    author: process:git
    last_modified: 2026-08-27
  - id: plan
    resource: ../../plans/ops/release-version-workflow.md
    title: Implementation plan for release CLI
    author: process:cursor
    last_modified: 2026-08-28
  - id: cargo-release
    resource: https://github.com/crate-ci/cargo-release/blob/HEAD/docs/reference.md
    title: cargo-release LEVEL or VERSION
    author: organization:crate-ci
  - id: release-please
    resource: https://github.com/googleapis/release-please
    title: release-please release PR
    author: organization:googleapis
  - id: semantic-release
    resource: https://github.com/semantic-release/semantic-release
    title: semantic-release auto publish
    author: organization:semantic-release
  - id: release-plz
    resource: https://release-plz.dev/docs/why
    title: release-plz compared to cargo-release
    author: organization:release-plz
  - id: conventional
    resource: https://www.conventionalcommits.org/
    title: Conventional Commits 1.0.0
    author: organization:conventionalcommits
  - id: semver
    resource: https://semver.org/
    title: Semantic Versioning 2.0.0
    author: organization:semver
---

# Release version workflow

## Current operator path

`okmate-ops` exposes `promote` with a single child `tag`. The operator
must already know the next version:[^ops-cli][^ops-plan]

```text
uv run okmate-ops promote tag v0.1.2
uv run okmate-ops promote tag v0.1.2 --from BRANCH
uv run okmate-ops promote tag v0.1.2 --force
uv run okmate-ops promote tag dev
```

For a `vX.Y.Z` name the command writes that version into `Cargo.toml`,
`okf/Cargo.toml`, and the `okf`/`okmate` rows in `Cargo.lock`, pushes
`chore(release): set version X.Y.Z` to the target branch (default
`main`), waits for hosted `Test` on that SHA, pushes an annotated
immutable tag, then bumps `Casks/okmate.rb` on
`koliyo/homebrew-okmate`. `dev` skips the version rewrite and
force-moves the rolling prerelease tag after the same CI wait.[^promote][^version][^readme]

The **Release** workflow is already tag-triggered (`v*` and `dev`). It
builds, signs, notarizes, writes the Sparkle appcast, and creates the
GitHub Release. Promote does not attach binaries.[^release-yml][^self-update]

Crate versions are shared today (`0.1.2` on workspace package and
`okmate`). There is no bump helper: `parse_release_version` only accepts
an exact `v` prefix plus `X.Y.Z` (optional prerelease suffix).[^cargo][^version]

The self-update plan treats `promote tag vX.Y.Z` as the only human gate
that may create an immutable `v*` tag. That gate, the Test-then-tag
order, immutable `v*`, and `dev` not being `SUFeedURL` are product
constraints, not naming accidents.[^self-update][^agents]

## What `promote tag` is naming

In gitops, *promote* usually means move an already-built artifact
across environments (`promote staging`, `promote production`). The
okmate-ops plan explicitly put `promote staging|production` out of
bound, then kept the parent verb anyway.[^ops-plan]

Here the work is *cut a release*: choose a version, rewrite crate
metadata, wait for tests, create the tag that starts packaging. The
child `tag` is leftover nesting for a family that does not exist. The
help line `promote       tag` advertises a tree with one leaf.
The devops skill still teaches that string.[^ops-cli][^devops]

`h35-ops` copies the same `promote tag` shape but only annotates and
pushes; it does not rewrite crate versions. Shared vocabulary is
convenient for the operator, not evidence that the name is right.

## Ecosystem patterns

Four common shapes:

| Shape | Typical command | Human gate | Version source |
| --- | --- | --- | --- |
| CLI level or pin | `cargo release patch` / `cargo release 1.2.3` | Yes, often dry-run first | Operator chooses level or exact version[^cargo-release] |
| Release PR | release-please, release-plz | Merge the PR | Commits (please requires Conventional Commits; plz can bump without them)[^release-please][^release-plz] |
| CI auto-publish | semantic-release on push to `main` | None by default | Conventional Commits[^semantic-release][^conventional] |
| Trailer override | `Release-As: 1.2.3` in a commit body | Weak (any merge can cut) | Explicit pin in history[^release-please] |

`cargo-release` is the closest Rust CLI idiom: one verb, positional
`LEVEL|VERSION` (`major`, `minor`, `patch`, prerelease channels, or an
exact semver). Steps exist (`version`, `tag`, `push`) but the happy
path is one command. Dry-run is the default.[^cargo-release]

release-please and release-plz optimize for *crates.io* and changelog
PRs. They do not wait for this repo's hosted `Test`, do not bump the
Homebrew tap after an immutable tag, and do not own Sparkle/notarize.
Adopting them as the publisher would replace the existing gate rather
than wrap it.[^release-plz][^self-update]

semantic-release publishes from CI when Conventional Commits land on
the release branch. That is the opposite of "only the operator creates
`v*`".[^semantic-release][^self-update]

## Commit-message bump

Conventional Commits map `fix` → patch, `feat` → minor, `BREAKING
CHANGE` / `type!` → major.[^conventional][^semver] That is a good
*changelog* convention. It is a poor *release trigger* for this
product:

- The repo already writes mixed messages (`chore(release): …`,
  knowledge log lines, plan phase commits). Auto-analysis would either
  no-op or surprise-bump.
- A mistaken `feat!:` on a docs PR would start notarize, Sparkle
  `releases/latest`, and a Homebrew tap push. Fail-closed signing does
  not make that cheap.[^release-yml]
- `0.y.z` treats breaking changes as minor in several tools; operators
  still decide when `1.0.0` happens. Commit parsers disagree.[^semver]
- The version commit is created *by* the release tool after the
  operator chooses a number. Inferring that number from the same
  history that includes the last bump is circular unless Conventional
  Commits are enforced on every merge first.

A footer such as `Release-As: 0.1.3` on a merge to `main` is still an
automatic tag after CI. It saves typing the version and loses the
explicit local command, dry-run, and `--from` branch. It is not
recommended while Release is a signed macOS job.

Keep Conventional Commits optional for humans and agents. Do not wire
them to tagging.

## Recommended DX

Keep one operator command that owns the existing pipeline. Rename the
verb to match the job. Accept a bump *level* or an exact tag, the same
way `cargo release` does:[^cargo-release][^promote]

```text
uv run okmate-ops release patch
uv run okmate-ops release minor
uv run okmate-ops release major
uv run okmate-ops release v0.2.0
uv run okmate-ops release dev
uv run okmate-ops release patch --from BRANCH --dry-run
```

Rules that keep the current contract:

- `patch` / `minor` / `major` compute the next `X.Y.Z` from the
  checked-in crate version on the target remote SHA (after fetch), not
  from a dirty worktree. Refuse if `okmate` and `okf` disagree.
- Exact `vX.Y.Z` stays for pins, skips, and `--force` recovery.
- `dev` stays a channel name, not a semver bump.
- `--dry-run` prints the resolved tag and the files that would change,
  then exits. Default for real cuts stays execute (this CLI already
  has a human at the keyboard); dry-run is the safety rail, not a
  second product.
- Do not add `release tag`. Release *is* the tag cut. Do not add a
  separate `bump` that leaves an untagged version commit on `main`.
- Keep `promote tag …` as a deprecated alias for one release so
  muscle memory and h35 docs do not break on day one.

Do not adopt cargo-release, release-please, or release-plz as the
publisher. Reuse their *argument grammar*, not their publish
pipeline.[^cargo-release][^release-plz]

A later optional improvement is `workflow_dispatch` with
`patch|minor|major` that runs the same Python entrypoint. That is still
an explicit human click, not a commit parser. It is not required for
DX if the local command prints the resolved version before waiting on
CI.

## What this is not

- Environment promotion (`staging` / `production`).
- Per-crate semver (okf and okmate stay one version).
- crates.io publish.
- Changelog generation (nice later; not required to stop typing
  `v0.1.3`).
- Changing Test-then-tag, Sparkle `SUFeedURL`, or immutable `v*`.

Paired implementation plan: [Release version workflow](../../plans/ops/release-version-workflow.md).[^plan]

[^promote]: Version commit on the target branch, wait for Test, annotated `v*` or force-moved `dev`, then tap bump for versioned tags.
[^version]: Shared `X.Y.Z` rewrite; `parse_release_version` requires `v` plus semver.
[^ops-cli]: Top-level commands; `promote` only routes `tag`.
[^ops-plan]: Documents `promote tag` and puts `promote staging|production` out of bound.
[^readme]: Operator publish steps and rolling `dev` fetch.
[^release-yml]: Tag push starts signed macOS package and `gh release create`.
[^self-update]: Human gate, Test-then-tag, `dev` is not the Sparkle feed.
[^cargo]: Workspace and package version `0.1.2` at research time.
[^agents]: `promote tag` fetches the target branch only; do not force-fetch all tags.
[^devops]: Skill still names `promote tag`.
[^plan]: Phased CLI rename, bump levels, dry-run, docs.
[^cargo-release]: Positional `major|minor|patch` or exact version; dry-run default in that tool.
[^release-please]: Maintains a release PR; merge tags; Conventional Commits; `Release-As` trailer.
[^semantic-release]: Analyzes commits and publishes from CI on the release branch.
[^release-plz]: Release PR for Rust; no Conventional Commits required; aimed at crates.io.
[^conventional]: `fix` / `feat` / breaking footer map to patch / minor / major.
[^semver]: MAJOR.MINOR.PATCH; 0.y.z is development, not a reason to auto-major.
