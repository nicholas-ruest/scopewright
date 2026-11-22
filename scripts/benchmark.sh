#!/usr/bin/env bash
set -euo pipefail
out=${1:?output json required}; start=$(date +%s%N)
body=$(curl -fsS -X POST http://127.0.0.1:8787/api/v1/factory_runs -H 'content-type: application/json' -H 'authorization: Bearer scopewright-operator' -H 'x-operator: benchmark@example.com' -H 'x-role: factory_run-operator' --data '{"service":"checkout-edge","trace_summary":"regional message-bus isolation","scope_pressure":35,"token_budget":12}')
end=$(date +%s%N); BODY="$body" START="$start" END="$end" node -e 'const r=JSON.parse(process.env.BODY); console.log(JSON.stringify({schema_version:1,mode:"full",elapsed_ms:Number((BigInt(process.env.END)-BigInt(process.env.START))/1000000n),stage_count:r.stages.length,risk_score:r.risk_score,correlation_id:r.correlation_id,all_upstreams:r.stages.length===17},null,2))' > "$out"
node -e 'const r=require(process.argv[1]);if(r.stage_count!==17||!r.all_upstreams)process.exit(1)' "$out"
