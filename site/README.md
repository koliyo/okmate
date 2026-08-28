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
