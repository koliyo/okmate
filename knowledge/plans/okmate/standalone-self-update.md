---
type: Implementation Plan
title: Standalone Okmate self-update
description: Ship Okmate.app with Sparkle 2 so macOS users check, consent, replace, and relaunch the way common Mac apps do, using GitHub Releases as the artifact store.
tags: [domain/okmate, domain/ops, concern/tooling, concern/architecture]
status: draft
generated: { by: process:cursor, at: 2026-08-29T14:30:00Z }
stale_after: 2026-11-26
authority: exploratory
owners: [human:nils]
sources:
  - id: readme
    resource: ../../../README.md
    title: Okmate README
    author: process:git
    last_modified: 2026-08-26
  - id: overview
    resource: ../../architecture/system-overview.md
    title: Okmate system overview
    author: process:cursor
    last_modified: 2026-08-26
  - id: extract
    resource: ../okf/okmate.md
    title: Okmate extractable Rust OKF mate
    author: process:cursor
    last_modified: 2026-08-26
  - id: ops-plan
    resource: ../ops/okmate-ops.md
    title: okmate-ops uv toolkit
    author: process:cursor
    last_modified: 2026-08-26
  - id: desktop
    resource: ../../../src/desktop.rs
    title: tao/wry desktop host
    author: process:git
    last_modified: 2026-08-26
  - id: cli
    resource: ../../../src/cli.rs
    title: Okmate clap CLI
    author: process:git
    last_modified: 2026-08-26
  - id: preview
    resource: ../../../src/preview.rs
    title: Live preview and session restore
    author: process:git
    last_modified: 2026-08-26
  - id: cargo
    resource: ../../../Cargo.toml
    title: Workspace and okmate package metadata
    author: process:git
    last_modified: 2026-08-26
  - id: ci
    resource: ../../../.github/workflows/ci.yml
    title: Hosted CI workflow
    author: process:git
    last_modified: 2026-08-26
  - id: promote
    resource: ../../../okmate-ops/src/okmate_ops/release.py
    title: promote tag after hosted Test
    author: process:git
    last_modified: 2026-08-27
  - id: cask
    resource: https://github.com/koliyo/homebrew-okmate/blob/main/Casks/okmate.rb
    title: Homebrew cask for Okmate.app
    author: process:cursor
    last_modified: 2026-08-27
  - id: sparkle
    resource: https://sparkle-project.org/documentation/
    title: Sparkle 2 setup, signing, and appcast
    author: organization:sparkle-project
  - id: sparkle-custom
    resource: https://sparkle-project.org/documentation/customization/
    title: Sparkle Info.plist keys and consent defaults
    author: organization:sparkle-project
  - id: sparkle-publish
    resource: https://sparkle-project.org/documentation/publishing/
    title: Publishing a Sparkle update
    author: organization:sparkle-project
  - id: axoupdater
    resource: https://github.com/axodotdev/axoupdater
    title: axoupdater cargo-dist install-receipt updater
    author: organization:axodotdev
  - id: dist
    resource: https://github.com/axodotdev/cargo-dist
    title: dist (formerly cargo-dist) installer and CI generator
    author: organization:axodotdev
  - id: gh-latest
    resource: https://docs.github.com/en/repositories/releasing-projects-on-github/linking-to-releases
    title: GitHub latest-release download URLs
    author: organization:github
---

# Standalone Okmate self-update

## Goal

Give the shipped macOS application the same update loop common Mac apps use:
background check, a **Check for Updates…** menu item, release notes, explicit
install consent, atomic `.app` replace, and relaunch. The first product
artifact is `Okmate.app`. GitHub Releases already created by `promote tag v*`
are the artifact store.[^extract][^promote][^sparkle][^readme]

## Why this shape

Okmate is already a standalone knowledge application with an in-crate tao/wry
window, but it is still a CLI that optionally opens that window. There is no
`.app` layout, no native application menu, no updater, and hosted CI only
runs Ubuntu tests. `promote tag` waits for the `Test` check and pushes a git
tag; it does not build or attach binaries.[^desktop][^cli][^ci][^promote][^overview]

The extract plan named `Okmate.app` and left signed notarized release out of
its desktop phase. This plan is that follow-on. It also has to make Finder
launch work: clap currently requires a subcommand, so a double-clicked bundle
would only print help.[^extract][^cli][^preview]

Sparkle 2 is the mechanism. It is what independent Mac apps use: HTTPS
appcast, EdDSA-signed archive, `SPUStandardUpdaterController`, standard
alert (install / later / skip), helper that quits, replaces the bundle, and
relaunches. Defaults delay the automatic-check permission until the second
launch and do not silently install.[^sparkle][^sparkle-custom]

## Out of bound

- Windows, Linux, or Homebrew as the in-app updater
- `dist` / cargo-dist and axoupdater as the GUI path
- Tauri or any `rocci-*` crate
- App Sandbox and security-scoped bookmarks
- Changing the portable `okf` engine
- Updating knowledge bundles, `~/.okmate/` config, or cache as part of an
  app update
- Treating the movable `dev` tag as a user update channel
- Silent install as the default (`SUAutomaticallyUpdate` stays off)
- Universal Intel+Apple Silicon in the first ship (arm64 first)
- Custom Sparkle chrome, delta updates, or anonymous usage ping
- Minting an approved Decision
- Executing this plan

## Constraints that do not move

- Knowledge records stay inert Markdown. `okf` stays UI-neutral.[^overview]
- Okmate must not depend on any `rocci-*` crate. Desktop and update code
  live in this crate (`src/`, packaging scripts, GitHub workflows).[^readme]
- Durable user state stays under `~/.okmate/`. Replacing `Okmate.app` must
  not rewrite config, cache, or session.[^readme][^overview]
- `cargo test -p okmate --no-default-features` on Ubuntu remains the hosted
  `Test` gate. Sparkle and `.app` work stay behind `desktop` +
  `target_os = "macos"`.[^ci][^cargo]
- `okmate-ops release` (`patch`, `minor`, `major`, or `vX.Y.Z`) stays the
  human gate that writes crate versions, pushes that commit, waits for
  `Test`, and pushes an immutable `v*` tag. Release packaging is a
  tag-triggered workflow, not a second release verb, unless a later
  phase adds a thin helper.[^promote][^ops-plan]
- `dev` remains a force-moved prerelease tag for operators, not
  `SUFeedURL`.[^readme][^promote]
- Serve the feed and archives over HTTPS. Sign archives with Sparkle EdDSA
  and, for production, Developer ID + notarization. Do not ship a custom
  “download a binary and overwrite argv[0]” loop.[^sparkle][^sparkle-publish]
- The updater runs only when the desktop window is opening from inside
  `Okmate.app`. `okmate check` / `view --no-window` / a cargo-installed
  binary never present Sparkle UI.[^desktop][^cli]
- User consent: automatic *checks* may follow Sparkle’s second-launch
  prompt; *installation* always uses the standard Sparkle alert unless the
  user later opts into automatic download in that alert.[^sparkle-custom]

## Product contract

| Role | Value |
| --- | --- |
| Bundle name | `Okmate.app` |
| Bundle id | `com.koliyo.okmate` (proposed; change before first signed build if needed) |
| Executable | `Contents/MacOS/okmate` (same crate binary) |
| Marketing version | `CFBundleShortVersionString` = Cargo `package.version` / git tag without `v` |
| Sparkle compare | incrementing `CFBundleVersion` (dotted numeric; every `v*` release must increase it) |
| Feed | `https://raw.githubusercontent.com/koliyo/okmate/sparkle/appcast.xml` |
| Archive | notarized zip or dmg of `Okmate.app` attached to that `v*` release |
| Settings | Sparkle’s own permission and “automatically download” checkbox; do not add an HTML settings store for the updater |

The `sparkle` branch holds one `appcast.xml` at a URL that does not 302.
Enclosures stay on the versioned GitHub Release zip. `dev` is not this
feed.[^sparkle-publish]

### Mac-like UX

| Moment | Behavior |
| --- | --- |
| Finder opens `Okmate.app` | No-args launch becomes `view` and restores `~/.okmate/state` when present |
| First launch | No update-permission sheet (Sparkle default) |
| Second launch | Sparkle may ask to check automatically |
| **Check for Updates…** | Immediate standard dialog; “You’re up to date” when none |
| Automatic check | Background, about once per day after permission; no first-paint block |
| Update found | Release notes, Install, Remind Me Later, Skip This Version |
| Install | Quit, replace `/Applications/Okmate.app` (or wherever this bundle lives), relaunch |
| Offline / read-only / translocated | Skip quietly on automatic check; no damage to user data |
| CLI from the same binary | Unchanged subcommands; no Sparkle |

Do not nag every launch. Sparkle already persists skip / last-check in macOS
defaults for the bundle id.[^sparkle][^sparkle-custom][^preview]

## Alternatives rejected for the GUI

**axoupdater + `dist`.** Built for cargo-dist install receipts and
re-running shell/PowerShell installers. That is a CLI installer loop, not
replace-the-`.app`. Adopting `dist` would also generate its own GitHub
release workflow and collide with the existing promote-then-tag
contract.[^axoupdater][^dist][^promote]

**Hand-rolled GitHub download.** Easy to get wrong: quarantine, code
signature, atomic replace of a running bundle, relaunch, skip/remind. Sparkle
exists so Okmate does not own that.

**In-app HTML “update” page.** Durable update state is not a knowledge-bundle
concern. The desktop host owns the menu and Sparkle; Askama settings stay
knowledge roots.[^overview][^desktop]

CLI-only installs (`cargo install`, a copied `okmate` on `PATH`) stay
manual in this plan. Homebrew installs the same GitHub Release `Okmate.zip`
Sparkle uses and `binary`-links `$(brew --prefix)/bin/okmate` to that
bundle (`koliyo/homebrew-okmate` `Casks/okmate.rb`, `auto_updates`); it is not a second in-app
updater.[^cask]

## Phases

### Phase 1 — Bundle identity and Finder launch

**Bound:** Add a small `src/bundle.rs` (or equivalent) that detects
`Okmate.app/Contents/MacOS/okmate`. When `argc == 1` and that is true, run
`view` with the existing session restore. Enable clap’s `cargo` feature so
`--version` matches `CFBundleShortVersionString`. Add `packaging/macos/Info.plist`
with bundle id, version placeholders, and empty-or-commented `SUFeedURL` /
`SUPublicEDKey`. Do not embed Sparkle yet.

**Out of bound:** Sparkle, signing, GitHub release workflow, defaulting
unpackaged `okmate` (no args) to `view`.

**Tests:** Detection and argv-rewrite unit tests that do not open a window.
Existing CLI tests still require a subcommand when not bundled.

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`.

**Owner:** `src/cli.rs`, `src/bundle.rs`, `packaging/macos/Info.plist`.

### Phase 2 — Local `.app` layout and application menu

**Bound:** A repo script (for example `packaging/macos/package.sh`) that
builds `target/release/okmate` with default features and assembles
`dist/Okmate.app` (`Contents/MacOS/okmate`, `Info.plist`, `PkgInfo`,
optional placeholder `icns`). On macOS desktop start, install a native
application menu: **Okmate**, **About Okmate**, **Check for Updates…**
(no-op or disabled until Phase 3), **Quit**. Packaging is a file-tree
contract; ad-hoc codesign is allowed locally.

**Out of bound:** Sparkle.framework, notarization, Intel slice, branded
icon work.

**Tests:** Script or Rust test asserts the assembled tree contains the
plist keys and executable path. Menu/IPC unit tests stay windowless.
Linux CI does not run the packager.

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`.

**Owner:** `packaging/macos/`, `src/desktop.rs`.

### Phase 3 — Sparkle client

**Bound:** Embed Sparkle 2 (`Sparkle.framework` under
`Contents/Frameworks`, rpath `@executable_path/../Frameworks`). Start
`SPUStandardUpdaterController` only from `desktop::run` when running inside
the app bundle. Wire **Check for Updates…** to `checkForUpdates:`. Set
`SUFeedURL` and a development `SUPublicEDKey` in the plist. Use Sparkle
defaults for permission timing; do not set `SUAutomaticallyUpdate` to
YES.[^sparkle][^sparkle-custom]

**Out of bound:** Production Developer ID, publishing a real appcast,
changing Ubuntu CI to a macOS runner.

**Tests:** `updater_allowed(bundled, opening_desktop)` cases. Optional
`#[ignore]` window smoke. `--no-default-features` stays green on Linux
(no Sparkle link).

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`. On a Mac checkout,
`cargo test -p okmate` (desktop feature) must also pass.

**Owner:** `src/update.rs` (macOS + desktop), `src/desktop.rs`,
`packaging/macos/`.

### Phase 4 — Tag-triggered artifacts and appcast

**Bound:** Add `.github/workflows/release.yml` on `v*` tags (not `dev`).
macOS runner: release-build, assemble `Okmate.app`, zip or dmg, run Sparkle
`generate_appcast` with the EdDSA private key from Actions secrets, attach
the archive and `appcast.xml` to the GitHub Release for that tag. Enclosure
URLs use
`https://github.com/koliyo/okmate/releases/download/<tag>/`. Document
secrets (`SPARKLE_EDDSA_PRIVATE_KEY`). `okmate-ops release` remains the
only way operators create that tag.[^sparkle-publish][^promote][^gh-latest]

**Out of bound:** Notarization. Changing `DEFAULT_CHECKS` so promote waits
on the release job. Generating appcast for `dev`.

**Tests:** Hermetic test of the appcast invocation helper (flat inbox,
`--maximum-deltas 0`, key via stdin, no secret echo). Workflow exists and
filters `v*`. `uv run --directory tools/okmate-ops --group dev pytest`
if the helper lands in okmate-ops; otherwise Rust/script tests in-repo.

**Exit:** `cargo test -p okmate --no-default-features`,
`cargo fmt --all -- --check`, and the ops pytest command if ops changed.

**Owner:** `.github/workflows/release.yml`, packaging scripts,
optionally `tools/okmate-ops`.

### Phase 5 — Developer ID, notarization, production keys

**Bound:** Sign the `.app` and nested Sparkle helpers with Developer ID
Application, enable hardened runtime (library validation compatible with
Sparkle), `notarytool` submit, staple. Production `SUPublicEDKey` in
Info.plist. Fail closed when signing secrets are missing in the release
workflow (do not upload an unsigned “production” archive). Document key
rotation: Sparkle allows rotating EdDSA *or* the Apple certificate, not
both at once, for regular app updates.[^sparkle]

**Out of bound:** App Sandbox. Publishing a second product identity.

**Tests:** Script dry-run / fail-closed without secrets. No fabricated
notarization success in CI.

**Exit:** `cargo test -p okmate --no-default-features` and
`cargo fmt --all -- --check`. README or `packaging/macos/README.md`
lists the secrets and the staple verify commands.

**Owner:** `packaging/macos/`, `.github/workflows/release.yml`, README.

### Phase 6 — Operator and user docs

**Bound:** README: how a Mac user installs the first `.app` (drag to
`/Applications`), how updates appear, that CLI-only installs do not
self-update. Operator notes: `okmate-ops release` writes crate
and Homebrew cask versions and pushes that commit before tagging; wait
for the release workflow, confirm the `sparkle` branch `appcast.xml`
serves the new item. Point at this plan; do not rewrite
[system overview](/architecture/system-overview.md) until the behavior ships.

**Out of bound:** Architecture Decision. Executing a real production
release.

**Exit:** `cargo test -p okmate --no-default-features`,
`cargo fmt --all -- --check`, and
`okmate check knowledge --profile strict --format terminal`.

**Owner:** `README.md`, `packaging/macos/`, this record’s status line.

## Status

Exploratory. Phases 1–6 are implemented on branch `standalone-self-update`
(Finder launch, `packaging/macos`, Sparkle client, tag-triggered
`release.yml`, fail-closed Developer ID/notarize, README). Not a shipped
production release and not an approved Decision. System overview is
unchanged until this behavior is on `main`. Evidence: this worktree's
`src/bundle.rs`, `src/update.rs`, `packaging/macos/`, `.github/workflows/release.yml`.

[^readme]: CLI, `~/.okmate/`, `promote tag`, and rolling `dev` fetch.
[^overview]: Application crate owns desktop; engine stays UI-neutral.
[^extract]: Named `Okmate.app`; Phase 7 left signed notarized release out.
[^ops-plan]: `promote tag v*` writes versions, pushes, waits for `Test`, then tags.
[^desktop]: tao/wry window; IPC is pick-folder and home; no menu or updater.
[^cli]: `subcommand_required`; no default Finder entrypoint.
[^preview]: Session restore from `~/.okmate/state` when `view` has no path.
[^cargo]: Default feature is `desktop`; version `0.1.0`.
[^ci]: Ubuntu `Test` job; no packaging or macOS runner.
[^promote]: Version commit on the target branch, then annotated `v*` or force-moved `dev`; no release assets.
[^sparkle]: Framework embed, EdDSA, appcast, standard controller, notarize.
[^sparkle-custom]: `SUFeedURL`, `SUPublicEDKey`, check/install consent keys.
[^sparkle-publish]: `generate_appcast`, enclosure URLs, release notes beside the archive.
[^axoupdater]: Install-receipt updater for cargo-dist installers, not `.app` replace.
[^dist]: Generates its own release CI around installers and tarballs.
[^gh-latest]: Stable `/releases/latest/download/<asset>` URLs.
[^cask]: Tap cask installs `Okmate.zip` from the same `v*` release and `binary` links `$(brew --prefix)/bin/okmate` to the bundle executable; `auto_updates` and `github_latest` follow Sparkle’s channel.
