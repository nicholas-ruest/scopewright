#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd); evidence=${1:?external evidence directory required}
mkdir -p "$evidence/screenshots" "$evidence/browser"
if [[ -L "$root/.evidence" ]]; then rm -f -- "$root/.evidence"; fi
if [[ -e "$root/.evidence" ]]; then
  echo "refusing to replace non-symlink evidence path: $root/.evidence" >&2
  exit 1
fi
ln -s "$evidence" "$root/.evidence"
cleanup(){
  "$root/scripts/stop.sh" >/dev/null 2>&1 || true
  if [[ -L "$root/.evidence" ]]; then rm -f -- "$root/.evidence"; fi
}
trap cleanup EXIT
"$root/scripts/reset.sh"; npm --prefix "$root/frontend" run build >/dev/null; "$root/scripts/start.sh"
(cd "$root/frontend" && npx playwright test)
cp -a "$root/frontend/test-results" "$evidence/browser/playwright-traces"
trace=$(find "$evidence/browser/playwright-traces" -name trace.zip -print -quit); test -n "$trace"; cp "$trace" "$evidence/browser/trace.zip"
echo "BROWSER_JOURNEYS_PASS restart_retrieval=true" | tee "$evidence/browser/receipt.txt"
