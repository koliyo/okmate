# Okmate agent instructions

## Start with repository evidence

- Inspect `git status --short` before editing. Preserve unrelated tracked and
  untracked work; do not clean or rewrite it to simplify a task.
- Read the root `README.md` and `okf/README.md` before changing a public
  contract.
- For architecture, decisions, implementation status, and known limitations,
  start at `knowledge/index.md` and the relevant knowledge record when that
  bundle exists.
- Verify mutable implementation claims against the code, tests, or published
  documentation cited by the record.
- Keep implemented, approved, exploratory, and historical claims distinct.

## Work in the owning layer

| Change | Primary owner |
| --- | --- |
| Portable OKF parse, validate, search, artifacts | `okf/` |
| CLI, live preview, Askama HTML, desktop window | this crate (`src/`, `templates/`) |
| Canonical knowledge bundle | `knowledge/` |
| Maintainer CI, promote, PR checkout | `okmate-ops` |

- Keep `knowledge/**/*.md` inert Markdown with OKF YAML. Do not add executable
  declarations.

## Validate proportionally

- `cargo fmt --all -- --check`
- `cargo test -p okf`
- `cargo test -p okmate --no-default-features`
- After knowledge edits: `okmate check knowledge --profile strict`
- When `okmate-ops` is present: `uv run okmate-ops ci`.

## Rolling `dev` tag

A later `git pull` may report `! [rejected] dev -> dev (would clobber existing
tag)` after `uv run okmate-ops promote tag dev`. That is expected for the
movable prerelease tag. Configure this clone to force-update **only** `dev`:

```sh
git config --local --add remote.origin.fetch '+refs/tags/dev:refs/tags/dev'
```

Do not force-fetch all tags; `v*` releases stay immutable. One-shot without
config: `git fetch origin tag dev --force`. Until `dev` exists on the remote,
a bare `git fetch origin` fails with `couldn't find remote ref refs/tags/dev`;
`okmate-ops promote tag` fetches only the target branch. Do not treat a
rejected `dev` fetch as a repository error.

## Use specialized workflows when available

- Repository-scoped skills live under `.agents/skills`.
- Durable plans, reports, audits, and status belong in `knowledge/`. Follow
  `$manage-okmate-knowledge`. Cursor may inject `write-knowledge` for
  destination only.
- Inspect or fix hosted CI with `$okmate-devops`.
- Land a worktree on `main` only when invoked via `$merge-worktree-to-main`.
