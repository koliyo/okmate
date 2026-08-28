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

## CI pin

GitHub Actions builds and deploys this tree (`.github/workflows/site.yml`).
That job installs `rocdown` by compiling `rocci-rocdown-cli` from
`koliyo/rocci` at revision `05a4179665c2b55d7d41450cd3e52c36859822f2`, with
Roc nightly `2026-08-23` / `fb208ba` (same pin as Rocci's `docker/install-roc.sh`).
`base_url` is `https://koliyo.github.io/okmate` for when a host is chosen.
GitHub Pages deploy is off; CI only checks and builds.

## Agent index

`site/llms.txt` is the authored starting point for visiting agents. `rocdown
build` also emits a generated `llms.txt` from page titles; copy the authored
file over it:

```sh
rocdown build site --output dist/site
cp site/llms.txt dist/site/llms.txt
```
