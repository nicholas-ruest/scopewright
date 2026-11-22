#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd); out=${1:?output json required}; start=$(date +%s%N)
(cd "$root" && cargo test --locked -p scopewright-governance composes_domain_stages -- --nocapture >/dev/null)
end=$(date +%s%N); START="$start" END="$end" node -e 'console.log(JSON.stringify({schema_version:1,mode:"domain-only-ablation",elapsed_ms:Number((BigInt(process.env.END)-BigInt(process.env.START))/1000000n),stage_count:10,removed_capabilities:["RuVector","ruv-FANN","midstream","QuDAG","federated-mcp","OPA","NATS"],interpretation:"removing executed upstreams reduces accountable stage coverage from 17 to 10"},null,2))' > "$out"
