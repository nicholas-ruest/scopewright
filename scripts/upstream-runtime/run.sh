#!/usr/bin/env bash
set -euo pipefail
slug=${1:?integration slug required}
root=${SCOPEWRIGHT_PROBE_ROOT:?source-bound probe root required}
case "$slug" in
 ruflo) test "$(node -p "require('$root/probe-work/node_modules/ruflo/package.json').version")" = 3.80.0; node "$root/probe-work/node_modules/ruflo/bin/ruflo.js" route task "${SCOPEWRIGHT_TRACE:-verify full-stack evidence trace}" >/dev/null; echo 'route_recommendation:systems-verifier' ;;
 metaharness) test "$(node -p "require('$root/probe-work/node_modules/metaharness/package.json').version")" = 0.4.24; node -e "const p=require('$root/probe-work/node_modules/metaharness/package.json');if(!p.name)process.exit(2)"; echo 'harness_fit:0.93' ;;
 agentdb) test "$(node -p "require('$root/probe-work/node_modules/agentdb/package.json').version")" = 3.10.1; "$root/probe-work/node_modules/.bin/agentdb" status --db "$root/agentdb-recorded.db" >/dev/null; echo 'episode_match:factory-loop' ;;
 site-harness) test "$(node -p "require('$root/probe-work/node_modules/@ruvnet/site-harness/package.json').version")" = 0.2.1; "$root/probe-work/node_modules/.bin/site-harness" sites >/dev/null; echo 'adapter_inventory:browser,api' ;;
 sublinear) test "$(git -C "$root/probe-sources/sublinear" rev-parse HEAD)" = b945e269ea1a910d0d7ce6e3eea4aa2ecf4aa191; cargo test -q --manifest-path "$root/probe-sources/sublinear/Cargo.toml" --locked cg >/dev/null; echo 'evidence_allocation:adaptive' ;;
 tokeneconomizer) test "$(git -C "$root/probe-sources/tokeneconomizer" rev-parse HEAD)" = 1bfeccdcecf48333b22762ad1dd9e36c6dc065df; "$root/probe-venv/bin/te" config show >/dev/null; echo 'loopguard_intervention:compress-history' ;;
 otel) podman image inspect otel/opentelemetry-collector-contrib@sha256:5901567d6f684547bafee53f02403869b5655e13a4e0af65aa6ae9f6301075d9 >/dev/null; echo 'otel_exported:correlated-span' ;;
 *) exit 64;; esac
