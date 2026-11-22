#!/usr/bin/env bash
set -euo pipefail
for slug in ruflo metaharness agentdb site-harness sublinear tokeneconomizer otel; do cargo test --locked -p "scopewright-upstream-$slug"; printf '%s=' "$slug"; scripts/upstream-runtime/run.sh "$slug"; done
echo INTEGRATION_EXECUTION_PASS ruvnet=5 enterprise=2 diversity=7
