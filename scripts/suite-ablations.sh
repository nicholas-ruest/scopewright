#!/usr/bin/env bash
set -euo pipefail
evidence=${DREAM_MACHINE_EVIDENCE_DIR:?}; scripts/ablation.sh "$evidence/ablation.json"; cat "$evidence/ablation.json"
