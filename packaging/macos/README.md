# macOS packaging

`uv run --no-dev okmate-ops package desktop` sets Okmate identity and runs
sibling [`h35-desktop`](https://github.com/koliyo/h35-desktop) `h35-ops package`.
That assembles `dist/Okmate.app` with Sparkle 2.
`okmate-ops package sign` wraps the host signer: Developer ID, notarytool,
staple. Missing signing secrets fail closed; the release workflow will not
attach an unsigned production archive.

The tag-triggered `.github/workflows/release.yml` workflow signs, zips that
bundle, and runs `okmate-ops package appcast`. The job uses the GitHub
Actions environment `release`, so repository secrets scoped to that
environment are the ones `package sign` and `package appcast` see.
Operators still create
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

## Obtain the secrets

Do this on a Mac you control. The release job only *uses* the values; it
does not create them.

`okmate-ops` already embeds a production Sparkle public key as
`DEFAULT_SU_PUBLIC_ED_KEY`. That string must stay the mate of
`SPARKLE_EDDSA_PRIVATE_KEY`. If a pair already exists for that public
key, export the private key. Do not generate a second pair unless you
also change the public key in source.

### Sparkle tools and EdDSA key

There is no `h35-ops sparkle` verb. `h35-ops package` (macOS) and
`h35-ops appcast` call `fetch_sparkle()`, which downloads Sparkle 2.8.1
into the sibling `h35-desktop` tree and extracts `Sparkle.framework`
plus `bin/`. From this repository:

```sh
uv run --no-dev okmate-ops package desktop
```

That leaves:

```
../h35-desktop/target/sparkle/2.8.1/bin/generate_keys
../h35-desktop/target/sparkle/2.8.1/bin/generate_appcast
```

First pair (only if the shipped public key is not already yours):

```sh
../h35-desktop/target/sparkle/2.8.1/bin/generate_keys
```

Sparkle stores the private key in the login keychain and prints the
public key (`SUPublicEDKey`). Print the existing private key for
`generate_appcast --ed-key-file -`:

```sh
../h35-desktop/target/sparkle/2.8.1/bin/generate_keys -p
```

That printed value is `SPARKLE_EDDSA_PRIVATE_KEY`. Never pass `-s` to
`generate_appcast`.

### Developer ID Application

Paid Apple Developer Program membership. Only the Account Holder can
create Developer ID certificates.

1. Keychain Access → Certificate Assistant → Request a Certificate From
   a Certificate Authority… → save a `.certSigningRequest` to disk.
2. [Certificates, Identifiers & Profiles](https://developer.apple.com/account/resources/certificates/list)
   → **+** → **Developer ID Application** (not Apple Development, not
   Developer ID Installer). Upload the CSR, download the `.cer`.
3. Double-click the `.cer` so it lands in the login keychain under My
   Certificates, on the same private key as the CSR.

```sh
security find-identity -p codesigning -v
```

The line `Developer ID Application: Your Name (TEAMID)` is
`APPLE_DEVELOPER_ID_APPLICATION`. `package sign` passes that string to
`codesign --sign`; the cert and key must already be in that machine’s
keychain.

### App Store Connect API key (notarytool)

1. [App Store Connect](https://appstoreconnect.apple.com) → Users and
   Access → Integrations → App Store Connect API.
2. **Issuer ID** at the top of the team keys list is `APPLE_API_ISSUER`
   (UUID). Create a **team** key; this project always sends `--issuer`.
3. Generate a team key (Developer role is enough). **Key ID** is
   `APPLE_API_KEY_ID`.
4. Download `AuthKey_<KEYID>.p8` once. Apple will not show it again.

`APPLE_API_KEY` is the entire PEM file, including
`-----BEGIN PRIVATE KEY-----` / `-----END PRIVATE KEY-----`.

```sh
xcrun notarytool history \
  --key AuthKey_XXXXXXXXXX.p8 \
  --key-id YOUR_KEY_ID \
  --issuer YOUR_ISSUER_UUID
```

A `401` usually means an individual key used with `--issuer`, or a
mistyped id.

Check that the five values are visible to the signer (does not claim
notarization):

```sh
SIGN_DRY_RUN=1 uv run --no-dev okmate-ops package sign dist/Okmate.app
```
