# macOS packaging

`uv run --no-dev okmate-ops package desktop` sets Okmate identity and runs
sibling [`h35-desktop`](https://github.com/koliyo/h35-desktop) `h35-ops package`.
That assembles `dist/Okmate.app` with Sparkle 2.
`okmate-ops package sign` wraps the host signer: Developer ID, notarytool,
staple. Missing signing secrets fail closed; the release workflow will not
attach an unsigned production archive.

The tag-triggered `.github/workflows/release.yml` workflow signs, zips that
bundle, and runs `okmate-ops package appcast`. Operators still create
immutable `v*` tags only with `okmate-ops promote tag vX.Y.Z`, which writes
the crate and Homebrew cask versions, pushes that commit, then tags. The
cask installs the same `Okmate.zip` Sparkle serves. The movable `dev` tag
is not an update channel.

## Secrets

| Name | Used by | Purpose |
| --- | --- | --- |
| `SPARKLE_EDDSA_PRIVATE_KEY` | `release.yml` → `okmate-ops package appcast` | Sparkle EdDSA private key, passed on stdin via `--ed-key-file -`. Never pass `-s` and never echo this value. |
| `APPLE_DEVELOPER_ID_APPLICATION` | `okmate-ops package sign` | Developer ID Application identity passed to `codesign --sign`. |
| `APPLE_API_KEY_ID` | `okmate-ops package sign` | App Store Connect API key id for `notarytool`. |
| `APPLE_API_ISSUER` | `okmate-ops package sign` | App Store Connect API issuer UUID. |
| `APPLE_API_KEY` | `okmate-ops package sign` | App Store Connect API `.p8` contents. Written to a temp file only. |

`SUPublicEDKey` in `Info.plist` is the production Sparkle public key and must
match `SPARKLE_EDDSA_PRIVATE_KEY`. Generate a pair with Sparkle
`bin/generate_keys`.

`SIGN_DRY_RUN=1 uv run --no-dev okmate-ops package sign dist/Okmate.app`
prints the intended steps without codesign or notarytool. It does not report
a notarization success.

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
