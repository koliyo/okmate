#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/../.." && pwd)
h35=${H35_DESKTOP:-$root/../h35-desktop}
export EXECUTABLE=${EXECUTABLE:-okmate}
exec "$h35/packaging/macos/$1" "${@:2}"
