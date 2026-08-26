# macOS packaging

`package.sh` release-builds `okmate`, embeds Sparkle 2, and writes `dist/Okmate.app`.

The tag-triggered `.github/workflows/release.yml` workflow zips that bundle and runs `generate-appcast.sh`. Operators still create immutable `v*` tags only with `okmate-ops promote tag vX.Y.Z`. The movable `dev` tag is not an update channel.

## Secrets

| Name | Used by | Purpose |
| --- | --- | --- |
| `SPARKLE_EDDSA_PRIVATE_KEY` | `release.yml` → `generate-appcast.sh` | Sparkle EdDSA private key, passed on stdin via `--ed-key-file -`. Never pass `-s` and never echo this value. |

The matching public key is `SUPublicEDKey` in `Info.plist`. Generate a pair with Sparkle `bin/generate_keys`.
