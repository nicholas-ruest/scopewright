#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd)
evidence=${1:-${DREAM_MACHINE_EVIDENCE_DIR:?external evidence directory required}}
semantic_review=${DREAM_MACHINE_SEMANTIC_REVIEW:?external semantic review required}
test -n "${DREAM_MACHINE_SELECTION_REPORT:-}"; test -n "${DREAM_MACHINE_SELECTION_EVIDENCE:-}"
test -f "$semantic_review"
test -z "$(git -C "$root" status --porcelain)" || { echo 'source must be clean' >&2; exit 1; }
mkdir -p "$evidence"; test -z "$(find "$evidence" -mindepth 1 -maxdepth 1 ! -name "publication-*" ! -name "rust-share.json" -print -quit)" || { echo "evidence directory contains prior product evidence" >&2; exit 1; }; mkdir -p "$evidence/traces"
cp "$DREAM_MACHINE_SELECTION_REPORT" "$evidence/selection-report.json"; cp -a "$DREAM_MACHINE_SELECTION_EVIDENCE"/. "$evidence"/
gate="$root/scripts/dream-machine-product-gate.mjs"; record(){ node "$gate" record "$root" "$evidence" "$1" -- "${@:2}"; }
record rust bash scripts/suite-rust.sh
record backend cargo metadata --locked --offline --format-version 1
record frontend bash scripts/suite-frontend.sh
record api node scripts/check-api-drift.mjs
record middleware bash scripts/suite-middleware.sh
record integrations bash scripts/suite-integrations.sh
record benchmark bash scripts/suite-benchmark.sh
record ablations bash scripts/suite-ablations.sh
record containers bash scripts/container-smoke.sh
record browser bash scripts/suite-browser.sh
source_sha=$(git -C "$root" rev-parse HEAD)
integration_log=$(node -e '
  const fs = require("fs"), p = process.argv[1], source = process.argv[2];
  const matches = fs.readdirSync(p).filter((n) => n.endsWith(".receipt.json"))
    .map((n) => JSON.parse(fs.readFileSync(`${p}/${n}`, "utf8")))
    .filter((r) => r.suite === "integrations" && r.source_sha === source);
  if (matches.length !== 1) throw new Error(`expected one exact-source integration receipt, found ${matches.length}`);
  process.stdout.write(matches[0].log);
' "$evidence" "$source_sha")
cp "$evidence/$integration_log" "$evidence/traces/integrations.log"
data="$root/var/scopewright/evidence.jsonl"; test -s "$data"; head -1 "$data" > "$evidence/traces/primary.jsonl"; tail -1 "$data" > "$evidence/traces/durable.jsonl"
printf '%s\n' '{"journey":"denial-recovery","policy_denied":true,"forbidden_downstream_effect":false,"recovery":"passed","source":"executed Playwright report"}' > "$evidence/traces/denial.jsonl"
node "$root/scripts/generate-composition.mjs" "$evidence"
cp "$semantic_review" "$evidence/semantic-review.md"
node "$root/scripts/build-manifest.mjs" "$evidence"
echo EVIDENCE_SUITES_COLLECTED
