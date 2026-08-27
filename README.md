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
- **Desktop:** [`h35-desktop`](https://github.com/koliyo/h35-desktop) (`src/desktop.rs` calls `preview`)

The `okf` engine lives in [`okf/`](okf/). This crate depends on that engine
only. Check, inspect, search, build, and view a bundle with this binary
(`okmate check knowledge --profile strict`). CLI-only builds omit the
webview: `cargo test -p okmate --no-default-features` (or
`cargo run --no-default-features`). `view` without `--no-window` needs the
default `desktop` feature.

Settings live under `~/.okmate/` (`OKMATE_CONFIG`, `OKMATE_CACHE`,
`OKMATE_STATE`).

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
okmate check knowledge --profile strict --format json
okmate inspect catalog knowledge
okmate inspect concept architecture/system-overview knowledge
okmate inspect graph knowledge
okmate search "system overview" knowledge --profile strict
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
interface). With a window, `--port` defaults to `auto` (a free local port).
`--no-window` defaults to `8000`. An explicit port fails if it is already in
use. Settings POST is `/__okmate/settings` and loopback-only. In the
desktop window, **Choose folder…** uses `rfd` via wry IPC (`pick-folder`), not
an HTTP pick-folder route. Without a window, paste the folder path.

Git cache is `OKMATE_CACHE` (default `~/.okmate/cache`).

## Development

```sh
cargo fmt --all -- --check
cargo test -p okf
cargo test -p okmate --no-default-features
```

Release-build the `okmate` binary with `uv run --no-dev okmate-ops build`.
To install it into `~/.local/bin`, run `uv run --no-dev okmate-ops install cli`.
On macOS, assemble `Okmate.app` with
`uv run --no-dev okmate-ops package desktop`.

## Install Okmate.app (macOS)

Download `Okmate.zip` from the latest
[GitHub Release](https://github.com/koliyo/okmate/releases/latest), unzip it,
and drag `Okmate.app` to `/Applications`. Or install the same archive with
Homebrew (this repository is a tap):

```sh
brew tap koliyo/okmate
brew install --cask okmate
```

The cask installs `Okmate.app` from that release `Okmate.zip` and uses
Homebrew’s `binary` stanza so `$(brew --prefix)/bin/okmate` (on Apple
Silicon, `/opt/homebrew/bin/okmate`) is a symlink to
`Okmate.app/Contents/MacOS/okmate`. That is the same crate binary: Finder
/ no-args launch opens the window, `okmate check` and the other
subcommands stay CLI, and Sparkle still runs only for the desktop window.
**Check for Updates…** and `brew upgrade --cask okmate` both follow
GitHub `releases/latest` (the same channel as `appcast.xml`). The cask is
marked `auto_updates` so Homebrew does not fight Sparkle. Neither path
rewrites `~/.okmate/` config, cache, or session.

Double-click opens the knowledge window (`view`) and restores
`~/.okmate/state` when present. After the second launch, Sparkle may ask
to check automatically. An update shows release notes and Install /
Remind Me Later / Skip.

A `cargo install` or copied `okmate` binary on `PATH` does not self-update.
Use a new install, or the `.app`, for updates.

## Publish a `v*` app release

The update plan is
[`knowledge/plans/okmate/standalone-self-update.md`](knowledge/plans/okmate/standalone-self-update.md).
Do not treat this README as an architecture decision.

1. Run `uv run okmate-ops promote tag vX.Y.Z` (or `--from BRANCH`). That is
   the only operator path that creates an immutable `v*` tag. For a versioned
   tag it writes `X.Y.Z` to `Cargo.toml`, `okf/Cargo.toml`, `Cargo.lock`, and
   `Casks/okmate.rb`, pushes that commit to the target branch, waits for
   hosted **Test** on the version commit, then pushes the tag. Set
   `BUNDLE_VERSION` only when Sparkle's compare version must move separately
   from Cargo (every `v*` must increase it).
2. Wait for the **Release** workflow on that tag (signing secrets must be
   present; the job fails closed instead of attaching an unsigned archive).
   How to mint those values is at the bottom of
   [`packaging/macos/README.md`](packaging/macos/README.md).
3. Confirm
   `https://github.com/koliyo/okmate/releases/latest/download/appcast.xml`
   serves the new item.

Replay local validation with `uv run --no-dev okmate-ops ci`. To publish a
GitHub release tag from `origin/main`, run
`uv run okmate-ops promote tag vX.Y.Z` (or `--from BRANCH`).
The `dev` tag is not version-bumped.
`uv run okmate-ops promote tag dev` force-moves the rolling `dev` prerelease
tag. `dev` is not `SUFeedURL`. A later `git pull` then reports
`! [rejected] dev -> dev (would clobber existing tag)` unless this repo
force-updates that tag on fetch:

```sh
git config --local --add remote.origin.fetch '+refs/tags/dev:refs/tags/dev'
```

Do not force-fetch all tags; `v*` releases stay immutable. Until `dev` exists
on the remote, a bare `git fetch origin` fails looking up that tag;
`okmate-ops promote tag` fetches only the target branch. To replace local
`dev` once without changing config, run `git fetch origin tag dev --force`.
