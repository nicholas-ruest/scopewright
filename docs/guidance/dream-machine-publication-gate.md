# Dream Machine publication entrypoint

Every generated product must pass the complete product contract and the Rust
source floor before its first remote publication. A Rust-only utility can be
100% Rust and still fail the product contract.

Use `scripts/dream-machine-publication-gate.mjs` as the execution entrypoint
around the already-authorized publisher. Copy it from a pinned core-memory
revision and independently verify its digest before use. Keep this controller
outside the candidate checkout or commit it before final verification.

The candidate must contain the unchanged canonical product, selection and
backend modules, bundle helper, full-product wrapper installed as
`scripts/verify-prototype.sh`, and the pinned Rust-share checker. The controller
checks their immutable digests, runs the bundle check, runs the Rust floor,
executes the real collector and canonical full-product verifier through the
wrapper, and rechecks source cleanliness, SHA and verifier integrity.

```bash
export DREAM_MACHINE_PRODUCER_RUN_ID='<actual parent execution identifier>'
node /path/to/controller/dream-machine-publication-gate.mjs --self-test
node /path/to/controller/dream-machine-publication-gate.mjs \
  --publish /path/to/candidate /path/to/external-evidence \
  -- bash scripts/publish-via-pr.sh
```

`publish-via-pr.sh` is the product's existing governed publishing procedure.
It must preserve branch/PR review authority, use the tested source and report
the remote refs it actually wrote. This example does not authorize a merge,
deployment, spend, credential change or additional repository. Do not replace
the publisher with a generic repository-creation fallback.

Use `--check CANDIDATE EXTERNAL_EVIDENCE` in CI; it performs verification without
running a publisher. Keep the separately named Rust floor CI step required by
the current build standard. Retain `publication-gate.json`, `rust-share.json`,
execution logs and the canonical source-bound evidence. A previous receipt must
not be reused for another run or source. The producer binding is an opaque hash
of the supplied execution identifier, not a signature or independent proof of
the caller's identity.

Every failed phase prevents the supplied publishing command from being called.
The inline self-test injects failures at each of six boundaries and checks zero
publisher calls; a clean control permits exactly one call. The public Ageglass
source snapshot is a regression example that must be denied before publication.
These tests establish execution order, not successful completion of a new full
product. Independent semantic review and post-publication readback remain required.

## Preserve the build request through delegation

Before any coding worker starts, save its complete resolved build request in the
isolated workspace and read it back. Pass that file and the pinned guidance to
the worker explicitly. Inspect the actual request for all mandatory criteria:
20 meaningful Rust product crates; 25 substantive ADRs; 12 DDD documents; real
TypeScript frontend, versioned API/client and middleware; 5 distinct executed
Ruvnet and 2 enterprise OSS projects; 3 browser journeys; restart retrieval;
benchmarks/ablations; source-bound verification; and at least 50% genuine Rust.
Do not shorten the request to 'build a Rust project' or 'make a useful CLI'.
Capture the handoff's digest, actual runtime and actual publishing command in
private run evidence. Report truncation or missing authority as a routing error.

## Limits

This entrypoint protects only publication routed through it. It does not revoke
credentials or prevent another session from directly invoking GitHub CLI, Git
or an API. The operator must route the actual builder's publication through the
entrypoint and verify that routing; changing an unrelated scheduled prompt is
not evidence that another publisher has been repaired. No global PATH shim,
security-policy change or credential restriction is installed by this code.
