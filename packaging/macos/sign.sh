#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: sign.sh <Okmate.app>" >&2
  exit 2
fi

app=$1
if [[ ! -d $app ]]; then
  echo "sign.sh: not an app bundle: $app" >&2
  exit 1
fi

required=(
  APPLE_DEVELOPER_ID_APPLICATION
  APPLE_API_KEY_ID
  APPLE_API_ISSUER
  APPLE_API_KEY
)

missing=()
for name in "${required[@]}"; do
  if [[ -z ${!name:-} ]]; then
    missing+=("$name")
  fi
done

if [[ ${SIGN_DRY_RUN:-} == 1 ]]; then
  echo "sign.sh: dry-run (not signing, not submitting to notarytool)"
  if [[ ${#missing[@]} -gt 0 ]]; then
    echo "sign.sh: would fail-closed without: ${missing[*]}"
  else
    echo "sign.sh: would codesign with hardened runtime using APPLE_DEVELOPER_ID_APPLICATION"
    echo "sign.sh: would notarytool submit and stapler staple"
  fi
  exit 0
fi

if [[ ${#missing[@]} -gt 0 ]]; then
  echo "sign.sh: missing signing secrets: ${missing[*]}" >&2
  echo "sign.sh: refusing to upload an unsigned production archive" >&2
  exit 1
fi

identity=$APPLE_DEVELOPER_ID_APPLICATION
codesign_nested() {
  local path=$1
  if [[ -e $path ]]; then
    codesign --force --options runtime --timestamp --sign "$identity" "$path"
  fi
}

framework=$app/Contents/Frameworks/Sparkle.framework
if [[ -d $framework ]]; then
  while IFS= read -r -d '' helper; do
    codesign_nested "$helper"
  done < <(find "$framework" \( -name Autoupdate -o -name Updater -o -name '*.xpc' \) -print0)
  codesign_nested "$framework"
fi

codesign --force --options runtime --timestamp --sign "$identity" "$app/Contents/MacOS/okmate"
codesign --force --options runtime --timestamp --sign "$identity" "$app"

scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
zip=$scratch/Okmate.zip
ditto -c -k --keepParent "$app" "$zip"

keyfile=$scratch/api.p8
printf '%s\n' "$APPLE_API_KEY" >"$keyfile"
xcrun notarytool submit "$zip" \
  --key "$keyfile" \
  --key-id "$APPLE_API_KEY_ID" \
  --issuer "$APPLE_API_ISSUER" \
  --wait
xcrun stapler staple "$app"
echo "sign.sh: stapled $app"
