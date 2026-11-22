#!/usr/bin/env bash
set -euo pipefail
curl -fsS -X POST http://127.0.0.1:8787/api/v1/factory_runs -H 'content-type: application/json' -H 'authorization: Bearer scopewright-operator' -H 'x-operator: seed@example.com' -H 'x-role: factory_run-operator' --data '{"service":"inventory-api","trace_summary":"regional dependency loss","scope_pressure":32,"token_budget":14}'
