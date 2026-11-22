#!/usr/bin/env bash
set -euo pipefail
for crate in ruvector fann midstream qudag federation opa nats; do cargo test --locked -p "scopewright-upstream-$crate"; done
for slug in ruvector fann midstream qudag federation opa nats; do printf '%s=' "$slug"; scripts/upstream-runtime/run.sh "$slug"; done
echo INTEGRATION_EXECUTION_PASS
