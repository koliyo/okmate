#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 2 ]]; then
  echo "usage: generate-appcast.sh <inbox-dir> <download-url-prefix>" >&2
  exit 2
fi

inbox=$1
prefix=$2

if [[ -z ${SPARKLE_EDDSA_PRIVATE_KEY:-} ]]; then
  echo "generate-appcast.sh: SPARKLE_EDDSA_PRIVATE_KEY is required" >&2
  exit 1
fi

if [[ ! -d $inbox ]]; then
  echo "generate-appcast.sh: inbox is not a directory: $inbox" >&2
  exit 1
fi

shopt -s nullglob
nested=("$inbox"/*/)
if [[ ${#nested[@]} -gt 0 ]]; then
  echo "generate-appcast.sh: inbox must be flat (no subdirectories)" >&2
  exit 1
fi

tool=${GENERATE_APPCAST:-}
if [[ -z $tool ]]; then
  echo "generate-appcast.sh: GENERATE_APPCAST is required" >&2
  exit 1
fi

printf '%s' "$SPARKLE_EDDSA_PRIVATE_KEY" | "$tool" \
  --maximum-deltas 0 \
  --download-url-prefix "$prefix" \
  --ed-key-file - \
  -o "$inbox/appcast.xml" \
  "$inbox"
