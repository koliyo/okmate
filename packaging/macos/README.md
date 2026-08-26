# macOS packaging

`package.sh` release-builds `okmate`, embeds Sparkle 2, and writes `dist/Okmate.app`.
`sign.sh` signs the bundle and Sparkle helpers with Developer ID Application, submits
to notarytool, and staples. Missing signing secrets fail closed; the release
workflow will not attach an unsigned production archive.

The tag-triggered `.github/workflows/release.yml` workflow signs, zips that
bundle, and runs `generate-appcast.sh`. Operators still create immutable `v*`
tags only with `okmate-ops promote tag vX.Y.Z`. The movable `dev` tag is not
an update channel.

## Secrets

| Name | Used by | Purpose |
| --- | --- | --- |
| `SPARKLE_EDDSA_PRIVATE_KEY` | `release.yml` → `generate-appcast.sh` | Sparkle EdDSA private key, passed on stdin via `--ed-key-file -`. Never pass `-s` and never echo this value. |
| `APPLE_DEVELOPER_ID_APPLICATION` | `sign.sh` | Developer ID Application identity passed to `codesign --sign`. |
| `APPLE_API_KEY_ID` | `sign.sh` | App Store Connect API key id for `notarytool`. |
| `APPLE_API_ISSUER` | `sign.sh` | App Store Connect API issuer UUID. |
| `APPLE_API_KEY` | `sign.sh` | App Store Connect API `.p8` contents. Written to a temp file only. |

`SUPublicEDKey` in `Info.plist` is the production Sparkle public key and must
match `SPARKLE_EDDSA_PRIVATE_KEY`. Generate a pair with Sparkle
`bin/generate_keys`.

`SIGN_DRY_RUN=1 packaging/macos/sign.sh dist/Okmate.app` prints the intended
steps without codesign or notarytool. It does not report a notarization
success.

## Verify a stapled build

```sh
codesign --verify --deep --strict --verbose=2 dist/Okmate.app
xcrun stapler validate dist/Okmate.app
spctl -a -vv dist/Okmate.app
```

## Key rotation

Sparkle can rotate the EdDSA key *or* the Apple Developer ID certificate
across a regular app update, not both in the same update. Ship one rotation,
let clients install it, then rotate the other.
