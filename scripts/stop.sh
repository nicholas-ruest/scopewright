#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd); runtime="$root/.runtime"
for name in frontend backend; do if test -f "$runtime/$name.pid"; then pid=$(cat "$runtime/$name.pid"); kill -TERM -- "-$pid" 2>/dev/null || kill "$pid" 2>/dev/null || true; wait "$pid" 2>/dev/null || true; fi; done
rm -f "$runtime/frontend.pid" "$runtime/backend.pid"
