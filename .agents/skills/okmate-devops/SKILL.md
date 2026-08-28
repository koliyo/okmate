---
name: okmate-devops
description: Inspect, monitor, trigger, diagnose, and fix GitHub Actions CI/CD workflows and repository automation for koliyo/okmate using the `gh` CLI and local validation. Use when checking CI status, triaging failing workflow runs, inspecting job logs, dispatching workflows, or reproducing CI failures. Do not use for ordinary non-CI source changes.
---

# Okmate DevOps

Inspect, triage, reproduce, fix, and verify GitHub Actions CI/CD for the
[okmate](https://github.com/koliyo/okmate) repository using `gh` and local
commands.

## Establish context

1. Work from the repository root and inspect `git status --short` before
   drawing CI provenance conclusions or editing workflows.
2. Hosted workflow: `.github/workflows/ci.yml`
   - Triggers: push to `main`, pull requests, `workflow_dispatch`
   - Job `Code Formatting & Lints` on `ubuntu-latest`: `okmate-ops ci lint`
     (`cargo fmt`, `cargo clippy --workspace --all-targets --no-default-features -D warnings`)
   - Job `Test` on `ubuntu-latest`: `cargo test -p okf && cargo test -p okmate --no-default-features`
3. When `okmate-ops` is present, replay the same jobs locally with
   `uv run okmate-ops ci`.
4. `gh` talks to `https://api.github.com`. In a sandbox, run `gh` unsandboxed.
5. Hosted release workflows:
   - `.github/workflows/cut-release.yml` — `workflow_dispatch` only.
     Runs `okmate-ops release` (same version commit → Test → tag → tap
     path as localhost). Versioned cuts need `HOMEBREW_TAP_TOKEN` on
     the `release` environment.
   - `.github/workflows/release.yml` — tag push (`v*`, `dev`) or
     `workflow_dispatch` with an existing tag (rebuild/notarize retry).
     Do not treat a branch dispatch as a release.

## Rolling `dev` tag

`uv run okmate-ops release dev` force-moves the rolling prerelease tag
after hosted CI on the target SHA. A later `git pull` may report
`! [rejected] dev -> dev (would clobber existing tag)`. That is expected.
Do not treat it as a repository or CI failure. This clone should force-update
**only** that tag:

```sh
git config --local --add remote.origin.fetch '+refs/tags/dev:refs/tags/dev'
```

Do not force-fetch all tags; `v*` releases stay immutable. One-shot:
`git fetch origin tag dev --force`. Until `dev` exists, a bare
`git fetch origin` fails on that refspec; `okmate-ops release` fetches the
target branch only.

## Inspect and monitor CI runs

```sh
gh run list --limit 5
gh run list --workflow ci.yml --limit 5
gh run list --branch main --limit 5
gh run list --commit $(git rev-parse HEAD)
gh run view RUN_ID
gh run view RUN_ID --log-failed
gh run watch RUN_ID
gh pr checks
gh run rerun RUN_ID --failed
gh workflow run cut-release.yml -f spec=patch -f from=main
gh workflow run release.yml -f tag=v0.1.3
```

## Reproduce locally

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --no-default-features -- -D warnings
cargo test -p okf
cargo test -p okmate --no-default-features
```

When the knowledge bundle exists:

```sh
cargo run -q --no-default-features -p okmate -- check knowledge --profile strict --format terminal
```

## Report results

- Name the workflow, run ID, and failing job.
- Separate hosted failures from local reproduction.
- Do not invent check names. Use the jobs defined in this repository.
