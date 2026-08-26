# Okmate

Okmate (open knowledge mate) is a standalone knowledge application for Open
Knowledge Format (OKF) bundles. The binary is `okmate`.

Repository: [github.com/koliyo/okmate](https://github.com/koliyo/okmate)

## Stack

- **Engine:** the portable [`okf`](okf/) crate in this repository (UI-neutral; the only library crate)
- **HTML:** Askama 0.16
- **HTTP:** Axum 0.8
- **Morph / SSE:** official Datastar Rust SDK 0.4 (`axum` feature) plus a
  pinned `assets/datastar.js`
- **Desktop:** tao / wry / rfd, in this crate (`src/desktop.rs`)

This crate does not interpret `.rocci` templates. `cargo test` does not require
Roc.

## Depends on `okf` only

Okmate must not depend on any `rocci-*` crate. The `okf` engine lives in
[`okf/`](okf/). For a side-by-side Rocci checkout you only need the knowledge
bundle path (`../rocci/knowledge`).

Rocci keeps the inert `knowledge/` bundle. Check, inspect, search, build, and
view that bundle with this binary (`okmate check knowledge --profile rocci`).
CLI-only builds omit the webview: `cargo test -p okmate --no-default-features`
(or `cargo run --no-default-features`). `view` without `--no-window` needs the
default `desktop` feature.

Settings live under `~/.okmate/` (`OKMATE_CONFIG`, `OKMATE_CACHE`,
`OKMATE_STATE`). If `~/.okmate/config.toml` is missing, Okmate may import
`~/.rocci/okf.toml` once. Do not treat `~/.rocci/` as the long-term path.

## CLI

| Command | Purpose |
| --- | --- |
| `okmate check [root]` | Validate a bundle (`--format terminal\|json`, `--profile`) |
| `okmate inspect catalog\|concept\|graph` | Engine JSON inspect |
| `okmate search <query> [root]` | Metadata and heading search JSON |
| `okmate benchmark <toml> [root]` | Retrieval benchmark |
| `okmate build [root] -o <dir>` | Engine artifacts plus Askama HTML |
| `okmate view [path]` | Live preview; omit `--no-window` to open tao/wry |
| `okmate roots` | Print resolved root paths (`--format json\|paths`, `--sync` / `--no-sync`) |
| `okmate sync [id]` | Fetch configured git roots |

```sh
okmate check knowledge --profile rocci --format json
okmate inspect catalog knowledge
okmate inspect concept architecture/system-overview knowledge
okmate inspect graph knowledge
okmate search "system overview" knowledge --profile rocci
okmate benchmark knowledge/retrieval-benchmark.toml knowledge
okmate build knowledge -o dist/knowledge
okmate view knowledge --no-window
okmate roots --format json --no-sync
okmate sync
```

`check`, `inspect`, `search`, and `build` stay single-root. Agents list
resolved folders first:

```sh
okmate roots --format paths | while IFS= read -r root; do
  okmate inspect catalog "$root"
done
```

`--format json` emits `{ id, kind, path, revision, incoming, enabled, error }`
and never includes tokens or resolved secrets. If the config is missing or
`roots` is empty, `./knowledge` is printed when that directory exists.

`view` serves the live HTML tree on localhost (pass `--public` to bind every
interface). Settings POST is `/__okmate/settings` and loopback-only. In the
desktop window, **Choose folder…** uses `rfd` via wry IPC (`pick-folder`), not
an HTTP pick-folder route. Without a window, paste the folder path.

Git cache is `OKMATE_CACHE` (default `~/.okmate/cache`).

## Development

```sh
cargo fmt --all -- --check
cargo test -p okf
cargo test -p okmate --no-default-features
```

When `tools/okmate-ops` is present, replay those jobs with
`uv run --no-dev okmate-ops ci`. To publish a GitHub release from
`origin/main`, run `uv run okmate-ops promote tag vX.Y.Z` (or `--from BRANCH`).
That waits for hosted CI on the target SHA, then pushes the tag.
`uv run okmate-ops promote tag dev` force-moves the rolling `dev` prerelease
tag. A later `git pull` then reports `! [rejected] dev -> dev (would clobber
existing tag)` unless this repo force-updates that tag on fetch:

```sh
git config --local --add remote.origin.fetch '+refs/tags/dev:refs/tags/dev'
```

Do not force-fetch all tags; `v*` releases stay immutable. To replace local
`dev` once without changing config, run `git fetch origin tag dev --force`.
