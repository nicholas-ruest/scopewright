#!/usr/bin/env bash
set -euo pipefail
evidence=${DREAM_MACHINE_EVIDENCE_DIR:?}; trap 'scripts/stop.sh >/dev/null 2>&1 || true' EXIT
scripts/start.sh >/dev/null; scripts/benchmark.sh "$evidence/benchmark.json"; cat "$evidence/benchmark.json"
