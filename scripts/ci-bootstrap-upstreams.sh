#!/usr/bin/env bash
set -euo pipefail
root=${1:?upstream checkout directory required}; mkdir -p "$root"
clone_pin(){
  local name=$1 url=$2 sha=$3 dir="$root/$1"
  test ! -e "$dir"; git init -q "$dir"
  git -C "$dir" remote add origin "$url"
  git -C "$dir" fetch -q --depth 1 origin "$sha"
  git -C "$dir" checkout -q --detach FETCH_HEAD
  test "$(git -C "$dir" rev-parse HEAD)" = "$sha"
}
clone_pin ruvector https://github.com/ruvnet/RuVector.git 979c31acc57b970c1a7e0771ed527c1703c137e8
clone_pin ruv-fann https://github.com/ruvnet/ruv-FANN.git 02431fd3b3d5639f38feee8f964ec9a8bcc27fc5
clone_pin midstream https://github.com/ruvnet/midstream.git 0934fd3b2f92b4abf93d0d56e6d9085a8d4ad92e
clone_pin qudag https://github.com/ruvnet/QuDAG.git b53466814554737c93f916d458783f7dff6c8acd
clone_pin federated-mcp https://github.com/ruvnet/federated-mcp.git 1caf7ace9e8c6977e1c0d77c89989d9ae6a35f71
clone_pin opa https://github.com/open-policy-agent/opa.git ba39415db4b3c90cd68942a74b3cc2223ec2b23c
clone_pin nats-server https://github.com/nats-io/nats-server.git 2a9066eb57ff4aa50d8e6a979bdfbd6a8b9db251
npm --prefix "$root/federated-mcp/modern" ci --no-audit --no-fund
(
  cd "$root/opa"
  go build -trimpath -o opa .
)
(
  cd "$root/nats-server"
  go build -trimpath -o nats-server .
)
test -x "$root/opa/opa"
test -x "$root/nats-server/nats-server"
printf 'CI_UPSTREAMS_READY=%s\n' "$root"
