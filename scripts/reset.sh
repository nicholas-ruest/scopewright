#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd); "$root/scripts/stop.sh" || true
target="$root/var/scopewright"; test "$target" = "$root/var/scopewright"; rm -rf "$target"; mkdir -p "$target"
