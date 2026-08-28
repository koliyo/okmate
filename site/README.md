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
`koliyo/rocci` at revision `c1a28910fac72cfc14a84d60cfb2d4bbc3e6243b`, with
Roc nightly `2026-08-23` / `fb208ba` (same pin as Rocci's `docker/install-roc.sh`).
`base_url` is `https://koliyo.github.io/okmate`. A custom domain is an operator
choice and is not set here.
