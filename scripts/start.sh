#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd); runtime="$root/.runtime"; mkdir -p "$runtime" "$root/var/scopewright"
"$root/scripts/stop.sh" >/dev/null 2>&1 || true
export PATH="$HOME/.cargo/bin:/usr/bin:$PATH" PLAYWRIGHT_BROWSERS_PATH="${PLAYWRIGHT_BROWSERS_PATH:-${XDG_CACHE_HOME:-$HOME/.cache}/ms-playwright}"
export PORT=${SCOPEWRIGHT_PORT:-8787} SCOPEWRIGHT_DATA_PATH=${SCOPEWRIGHT_DATA_PATH:-$root/var/scopewright/evidence.jsonl}
export SCOPEWRIGHT_PROBE_ROOT=${SCOPEWRIGHT_PROBE_ROOT:-$root/.upstreams}
source "$root/scripts/upstream-runtime/env.sh"
setsid sh -c "cd '$root' && exec cargo run --locked -p scopewright-service" >"$runtime/backend.log" 2>&1 & echo $! >"$runtime/backend.pid"
setsid sh -c "cd '$root/frontend' && exec npm run dev -- --port 4173 --strictPort" >"$runtime/frontend.log" 2>&1 & echo $! >"$runtime/frontend.pid"
"$root/scripts/readiness.sh"
echo "Scopewright ready at http://127.0.0.1:4173"
