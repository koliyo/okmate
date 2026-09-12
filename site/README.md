# OKMate product website

Static Rocdown site. Canonical project knowledge stays in `knowledge/` (OKF).
This tree is Rocdown only.

## Check, preview, and build

Prefer a `rocdown` on `PATH` (future install or CI pin):

```sh
rocdown check site
rocdown view site --no-window
rocdown build site --output dist/site
```

Maintainer fallback: sibling Rocci checkout.

```sh
cargo run -q -p rocci-rocdown-cli --manifest-path ../rocci/Cargo.toml -- check site
```

Do not add Rocci crates to this repository's `Cargo.toml`.

## CI

GitHub Actions builds this tree (`.github/workflows/site.yml`). That job
installs `rocdown` from the latest `koliyo/rocci` GitHub Release Linux
archive (`rocci-*-x86_64-unknown-linux-gnu.tar.gz`), not by compiling
`rocci-rocdown-cli`. `rocdown build` still needs Roc on `PATH`; CI pins Roc
nightly `2026-09-03` / `62fcb65`. `base_url` is
`https://koliyo.github.io/okmate` for when a host is chosen. GitHub Pages
deploy is off; CI only checks and builds.

## Agent index

`site/llms.txt` is the authored starting point for visiting agents. `rocdown
build` also emits a generated `llms.txt` from page titles; copy the authored
file over it:

```sh
rocdown build site --output dist/site
cp site/llms.txt dist/site/llms.txt
```
