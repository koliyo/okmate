#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 3 || $# -gt 4 ]]; then
  echo "usage: assemble.sh <okmate-binary> <dest-app> <version> [bundle-version]" >&2
  exit 2
fi

binary=$(cd "$(dirname "$1")" && pwd)/$(basename "$1")
dest=$2
version=$3
bundle_version=${4:-$version}

if [[ ! -f $binary ]]; then
  echo "assemble.sh: binary not found: $binary" >&2
  exit 1
fi

here=$(cd "$(dirname "$0")" && pwd)
template=$here/Info.plist
if [[ ! -f $template ]]; then
  echo "assemble.sh: missing $template" >&2
  exit 1
fi

rm -rf "$dest"
macos=$dest/Contents/MacOS
resources=$dest/Contents/Resources
mkdir -p "$macos" "$resources"

cp "$binary" "$macos/okmate"
chmod 755 "$macos/okmate"

sed -e "s/@VERSION@/${version}/g" \
    -e "s/@BUNDLE_VERSION@/${bundle_version}/g" \
    "$template" >"$dest/Contents/Info.plist"

printf 'APPL????' >"$dest/Contents/PkgInfo"

if [[ -f $here/AppIcon.icns ]]; then
  cp "$here/AppIcon.icns" "$resources/AppIcon.icns"
fi
