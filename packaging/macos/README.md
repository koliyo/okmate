# macOS packaging

`uv run --no-dev okmate-ops package desktop` sets OKMate identity and runs
sibling [`h35-desktop`](https://github.com/koliyo/h35-desktop) `h35-ops package`.
That assembles `dist/OKMate.app` with Sparkle 2.
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
cask installs the same `OKMate.zip` Sparkle serves. The movable `dev` tag
also runs this workflow and attaches a GitHub **prerelease**; it is not
`releases/latest` and not Sparkle’s feed.

## Secrets

| Name | Used by | Purpose |
| --- | --- | --- |
| `SPARKLE_EDDSA_PRIVATE_KEY` | `release.yml` → `okmate-ops package appcast` | Sparkle EdDSA private key, passed on stdin via `--ed-key-file -`. Never pass `-s` and never echo this value. |
| `APPLE_DEVELOPER_ID_APPLICATION` | `okmate-ops package sign` | Developer ID Application identity passed to `codesign --sign`. |
| `APPLE_CERTIFICATE_P12` | `okmate-ops package sign` on GitHub Actions | Base64 of a PKCS#12 that contains that Developer ID cert and private key. Hosted runners have an empty keychain; the identity string alone is not enough. |
| `APPLE_CERTIFICATE_PASSWORD` | `okmate-ops package sign` on GitHub Actions | Passphrase for that `.p12`. |
| `APPLE_API_KEY_ID` | `okmate-ops package sign` | App Store Connect API key id for `notarytool`. |
| `APPLE_API_ISSUER` | `okmate-ops package sign` | App Store Connect API issuer UUID. |
| `APPLE_API_KEY` | `okmate-ops package sign` | App Store Connect API `.p8` contents. Written to a temp file only. |

`SUPublicEDKey` in `Info.plist` is the production Sparkle public key and must
match `SPARKLE_EDDSA_PRIVATE_KEY`. Generate a pair with Sparkle
`bin/generate_keys`.

`SIGN_DRY_RUN=1 uv run --no-dev okmate-ops package sign dist/OKMate.app`
prints the intended steps without codesign or notarytool. It does not report
a notarization success.

## Verify a stapled build

```sh
codesign --verify --deep --strict --verbose=2 dist/OKMate.app
xcrun stapler validate dist/OKMate.app
spctl -a -vv dist/OKMate.app
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
`APPLE_DEVELOPER_ID_APPLICATION`. Local `package sign` passes that
string to `codesign --sign` and uses the cert already in this
machine’s login keychain.

GitHub-hosted `macos-latest` has no login keychain identities. Export
only the Developer ID (do not run `security export -t identities`; that
walks every identity, including VPN and browser certs):

1. Keychain Access → login → My Certificates.
2. Select `Developer ID Application: … (TEAMID)` — the same string as
   `APPLE_DEVELOPER_ID_APPLICATION`.
3. File → Export Items… → Personal Information Exchange (`.p12`).
4. Choose an export passphrase (`APPLE_CERTIFICATE_PASSWORD`).

```sh
base64 < developer-id.p12 | pbcopy
```

`APPLE_CERTIFICATE_P12` is that base64 blob. `package sign` imports the
`.p12` into a temporary keychain, signs, then deletes the keychain.

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

Check that the Apple and Sparkle values are visible to the signer
(does not claim notarization). On a laptop the `.p12` secrets are
optional if the Developer ID is already in the login keychain:

```sh
SIGN_DRY_RUN=1 uv run --no-dev okmate-ops package sign dist/OKMate.app
```
