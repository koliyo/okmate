#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "$0")/../.." && pwd)
cd "$root"

version=$(sed -n 's/^version = "\([^"]*\)"/\1/p' Cargo.toml | head -n 1)
if [[ -z $version ]]; then
  echo "package.sh: could not read package.version from Cargo.toml" >&2
  exit 1
fi

cargo build --release -p okmate

"$root/packaging/macos/assemble.sh" \
  "$root/target/release/okmate" \
  "$root/dist/Okmate.app" \
  "$version" \
  "$version"

if [[ $(uname -s) == Darwin ]]; then
  codesign --force --deep --sign - "$root/dist/Okmate.app"
fi

echo "$root/dist/Okmate.app"
