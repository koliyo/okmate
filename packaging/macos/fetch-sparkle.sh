#!/usr/bin/env bash
set -euo pipefail

SPARKLE_VERSION=${SPARKLE_VERSION:-2.8.1}
root=$(cd "$(dirname "$0")/../.." && pwd)
dest=${SPARKLE_CACHE:-$root/target/sparkle/$SPARKLE_VERSION}
framework=$dest/Sparkle.framework

if [[ ! -d $framework ]]; then
  mkdir -p "$dest"
  archive=$dest/Sparkle-$SPARKLE_VERSION.tar.xz
  url=https://github.com/sparkle-project/Sparkle/releases/download/$SPARKLE_VERSION/Sparkle-$SPARKLE_VERSION.tar.xz
  if [[ ! -f $archive ]]; then
    curl -fsSL -o "$archive" "$url"
  fi
  tar -xJf "$archive" -C "$dest" Sparkle.framework bin
fi

if [[ ! -d $framework ]]; then
  echo "fetch-sparkle.sh: Sparkle.framework missing under $dest" >&2
  exit 1
fi

printf '%s\n' "$framework"
