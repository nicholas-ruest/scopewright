#!/usr/bin/env bash
set -euo pipefail
runner="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/run.sh"
export SCOPEWRIGHT_RUFLO_CMD="$runner ruflo"
export SCOPEWRIGHT_METAHARNESS_CMD="$runner metaharness"
export SCOPEWRIGHT_AGENTDB_CMD="$runner agentdb"
export SCOPEWRIGHT_SITE_HARNESS_CMD="$runner site-harness"
export SCOPEWRIGHT_SUBLINEAR_CMD="$runner sublinear"
export SCOPEWRIGHT_TOKENECONOMIZER_CMD="$runner tokeneconomizer"
export SCOPEWRIGHT_OTEL_CMD="$runner otel"
