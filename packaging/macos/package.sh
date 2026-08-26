#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "$0")/../.." && pwd)
h35=${H35_DESKTOP:-$root/../h35-desktop}

export APP_NAME=Okmate
export BUNDLE_ID=${BUNDLE_ID:-com.koliyo.okmate}
export EXECUTABLE=okmate
export CRATE=okmate
export PRODUCT_ROOT=$root
export SU_FEED_URL=${SU_FEED_URL:-https://github.com/koliyo/okmate/releases/latest/download/appcast.xml}
export SU_PUBLIC_ED_KEY=${SU_PUBLIC_ED_KEY:-0cxKUYv/b7Z7qSI2l2lEzV0IcV/rb59l6lFnRD5vs2U=}

exec "$h35/packaging/macos/package.sh"
