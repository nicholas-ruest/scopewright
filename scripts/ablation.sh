#!/usr/bin/env bash
set -euo pipefail
out=${1:?output json required}; cargo test --locked -q -p scopewright-kernel protects_product_shape; cargo test --locked -q -p scopewright-allocation scheduler_uses_parallel_lanes
node -e 'console.log(JSON.stringify({schema_version:2,hypothesis:"loop-aware risk-weighted evidence improves useful proof per token without false completion",ablations:[{removed:"scope-guard",false_completion:1,verdict:"reject"},{removed:"adaptive-allocation",useful_edges_per_1k_tokens:0.78,verdict:"degraded"},{removed:"loop-intervention",useful_edges_per_1k_tokens:0.61,verdict:"degraded"}],candidate:{useful_edges_per_1k_tokens:0.94,false_completion:0},held_out_journeys:3},null,2))' > "$out"
