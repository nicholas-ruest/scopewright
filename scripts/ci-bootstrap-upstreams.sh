#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd); up=${SCOPEWRIGHT_PROBE_ROOT:-$root/.upstreams}; mkdir -p "$up/probe-work" "$up/probe-sources"
cat > "$up/probe-work/package.json" <<'JSON'
{"private":true,"dependencies":{"ruflo":"3.80.0","metaharness":"0.4.24","agentdb":"3.10.1","@ruvnet/site-harness":"0.2.1"}}
JSON
npm --prefix "$up/probe-work" install --ignore-scripts --no-audit --no-fund
clone(){ n=$1; u=$2; s=$3; test -d "$up/probe-sources/$n/.git" || git clone -q "$u" "$up/probe-sources/$n"; git -C "$up/probe-sources/$n" fetch -q origin "$s"; git -C "$up/probe-sources/$n" checkout -q --detach "$s"; }
clone sublinear https://github.com/ruvnet/sublinear-time-solver.git b945e269ea1a910d0d7ce6e3eea4aa2ecf4aa191
clone tokeneconomizer https://github.com/microsoft/TokenEconomizer.git 1bfeccdcecf48333b22762ad1dd9e36c6dc065df
python3 -m venv "$up/probe-venv"; "$up/probe-venv/bin/pip" install -q "$up/probe-sources/tokeneconomizer"
"$up/probe-work/node_modules/.bin/agentdb" init "$up/agentdb-recorded.db" >/dev/null
export SCOPEWRIGHT_PROBE_ROOT="$up"; echo UPSTREAM_BOOTSTRAP_PASS
