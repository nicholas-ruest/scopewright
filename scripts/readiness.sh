#!/usr/bin/env bash
set -euo pipefail
for _ in $(seq 1 60); do if curl -fsS http://127.0.0.1:8787/healthz >/dev/null && curl -fsS http://127.0.0.1:4173/ >/dev/null; then exit 0; fi; sleep 1; done
echo "Scopewright readiness timed out" >&2; exit 1
