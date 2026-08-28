# Knowledge log

Git merges this file with the built-in `union` driver (see `.gitattributes`).
Independent bullets under the same `## YYYY-MM-DD` heading combine instead of
conflicting. Add a new list item; do not reword another session's bullet in
the same change.

## 2026-08-28

- Drafted paired research and plan for the OKMate product website (Rocdown static site, git/OKF/multi-bundle positioning, first-class `/agents/` and `/llms.txt`). Exploratory; do not log complete until hosted CI succeeds.
- Drafted research on using Leptos instead of Askama and Datastar for the okmate viewer (SSR-only vs islands vs hydrate). Exploratory; do not log complete until hosted CI succeeds.
- Drafted paired research and plan for the release version workflow (`promote tag` → `okmate-ops release` with patch/minor/major; no commit-message auto-tag). Exploratory; do not log complete until hosted CI succeeds.

## 2026-08-27

- Viewer-responsiveness Phases 1–5 and 7 passed hosted CI on `500fc80` (workflow run [33070204563](https://github.com/koliyo/okmate/actions/runs/33070204563); Phase 6 skipped).
- Recorded machine-local `okmate timings` baselines on the in-repo `knowledge/` root after viewer-responsiveness Phases 1–5 (Phase 6 skipped). Exploratory; do not log complete until hosted CI succeeds.
- Revised the viewer-responsiveness plan so Phase 1 is a durable `okmate timings` pipeline (JSON/terminal spans plus live `Server-Timing`) for this work and later investigations. Exploratory; do not log complete until hosted CI succeeds.
- Drafted paired research and plan for viewer responsiveness (in-memory Datastar clicks, preview without per-click `okf::load`, windowed review and log). Exploratory; do not log complete until hosted CI succeeds.
- Moved the Homebrew cask to `koliyo/homebrew-okmate` so `brew tap koliyo/okmate` works. `promote tag v*` now bumps that tap after the versioned tag. Exploratory; do not log complete until hosted CI succeeds.
- Renamed the macOS app and user-facing product to OKMate; the CLI binary, Homebrew cask token, and `~/.okmate/` paths stay `okmate`. Exploratory; do not log complete until hosted CI succeeds.
- Drafted paired research and plan for an extended multi-bundle viewer (separated and merged sidebar, merged recents and log, collection hover). Exploratory; do not log complete until hosted CI succeeds.
- Added in-repo Homebrew formula for the `okmate` crate and taught `promote tag v*` to write crate versions, push that commit to the target branch, then tag. Exploratory; do not log complete until hosted CI succeeds.
- Switched the Homebrew tap to a Sparkle-aligned cask (`Okmate.zip` from GitHub Releases, `auto_updates`, `github_latest` livecheck). Exploratory; do not log complete until hosted CI succeeds.
- Homebrew cask now links the bundle CLI so one install provides `Okmate.app` and `okmate` from the same release zip. Exploratory; do not log complete until hosted CI succeeds.

## 2026-08-26

- Drafted paired audit and plan for okmate viewer-shell parity with last rocci-okf (panes, outline spy, keep-nav). Exploratory; do not log complete until hosted CI succeeds.
- Hoisted `okmate-ops` from `tools/okmate-ops` to the repository root. Exploratory; do not log complete until hosted CI succeeds.
- Implemented standalone macOS self-update phases 1–6 on `standalone-self-update` (Sparkle client, tag release workflow, signing fail-closed, README). Exploratory; do not log complete until hosted CI succeeds.
- Drafted okmate dashboard parity with the last rocci-okf review shell (home recents, review queue, concept meta, rocci-cli port defaults). Exploratory; do not log complete until hosted CI succeeds.
- Bootstrapped this bundle: architecture overview, closed-area decision, and operator-toolkit note. Agent skills and `tools/okmate-ops` live in-repo. Exploratory; do not log complete until hosted CI succeeds.
- Migrated engine and okmate-app discussions from Rocci (`plans/okf`, selected research, load-performance status). Rocci keeps pointer stubs. Exploratory; do not log complete until hosted CI succeeds.
- Drafted standalone macOS self-update plan (Sparkle 2 + GitHub Releases). Exploratory; do not log complete until hosted CI succeeds.
