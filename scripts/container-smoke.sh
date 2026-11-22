#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd); image=localhost/scopewright:${SOURCE_SHA:-local}
podman build --network=host -f "$root/Containerfile" -t "$image" "$root"
cid=$(podman run -d --rm --network=host -e PORT=8877 -e SCOPEWRIGHT_DATA_PATH=/tmp/evidence.jsonl "$image"); trap 'podman stop "$cid" >/dev/null 2>&1 || true' EXIT
for _ in $(seq 1 40); do curl -fsS http://127.0.0.1:8877/healthz >/dev/null && curl -fsS http://127.0.0.1:8877/ >/dev/null && { echo CONTAINER_TOPOLOGY_PASS; exit 0; }; sleep 1; done
podman logs "$cid"; exit 1
