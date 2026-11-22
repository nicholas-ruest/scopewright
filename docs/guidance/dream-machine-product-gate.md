# Executable Dream Machine product evidence gate

## November 1 integrated-backend and design enforcement

The baseline is at least **25 substantive ADRs, 12 DDD documents and 20 meaningful
Rust crates composed into a working backend, middleware and TypeScript frontend**.
Improvements are additive. The composition report now requires `schema_version: 2`;
the external artifact manifest remains schema 2 and selection remains schema 1.
Old composition reports cannot bypass these checks. Copy all three gate modules from
one immutable revision: product, selection and `dream-machine-backend-gate.mjs`.

Record the real Cargo resolved graph on the same clean candidate, after dependencies
have been installed through authorized paths:

```bash
node scripts/dream-machine-product-gate.mjs record . "$task_evidence_dir" backend -- cargo metadata --locked --offline --format-version 1
```

Add `backend_metadata` to the external manifest. Its `path` is this recorder receipt's
`log`, and `sha256` is its `log_sha256`; the log contains Cargo's actual JSON stdout.
The recorder preserves stderr separately with its own hash so warnings cannot corrupt
the JSON. Keep both raw streams and the receipt. Do not handwrite metadata or receipts.
`--no-deps` cannot prove reachability. The graph describes the default production feature
configuration; the launched product must exercise that configuration. Rust quality and
integration suites still execute the service and tests; metadata alone is not behavior.

The backend checker verifies the entrypoint is a Cargo workspace binary, each counted
crate is an actual workspace member reachable through production dependency edges, its
implementation belongs to its package, and its named caller has a production edge.
Development/build-only packages and disconnected workspace members do not count.

Composition schema 2 adds these mandatory fields to the earlier composition schema:

| Field | Required content |
| --- | --- |
| `backend.crates[]` | Existing fields plus distinct useful `responsibility`, `public_api`, `failure_test_path`, nonempty `adr_paths` and `ddd_paths`. Source paths must belong to the actual immutable project. |
| `design.adrs`, `design.ddds` | At least 25 and 12 unique Markdown document records respectively. Each record has `path`, actual content `sha256`, `title`, `crate_manifest_paths`, `implementation_paths`, `test_paths`. Empty bodies, duplicate paths and exactly duplicate prose fail. Independently reject templated/filler designs; differing wording is not proof of substance. |
| `design.adrs_index_path`, `design.ddds_index_path` | Actual source index files. |
| Crate/design mappings | Every counted crate has ADR and DDD references; the referenced document reciprocally maps that crate. Each document maps implemented source and tests. |
| `api` | `schema_path`, TypeScript `client_path`, `contract_test_path`; inspect actual typed/versioned API and drift assertions. |
| `middleware.registration_path`, `middleware.order` | Actual source registration and unique ordered control names. Independently inspect registration and effect ordering. |
| `middleware.controls[]` | Unique `name`, actual `implementation_path`, `test_path`. Must cover `authn`, `authz`, `policy`, `validation`, `errors`, `correlation`, `timeouts`, `observability`. Preserve global denial/runtime evidence fields below. |

An actual Cargo edge is necessary, not proof that a function executes. Review real
call sites, per-crate behavior and failure tests, middleware ordering, frontend requests
and correlated raw traces. The gate must not be presented as proof of useful code or
of a finished product. Run the entire checker in `verify-prototype.sh` and CI. Missing
fields or failed checks forbid BUILT; within the authorized build run keep implementing
the full product instead of producing only an audit, handoff or smaller substitute.

This closes the October 26 regression: a small Rust CLI must not become the daily
deliverable, and `CI_REBIND_REQUIRED` or a passing unit-test count cannot establish
`BUILT`. The canonical build standard and independent product review still apply.

RuVector/RVF is optional. Choose integrations and intelligence/persistence for the
product; omission of RuVector is never itself a gate failure. Durable workflows,
authorized data access and the five-Ruvnet/two-enterprise integration floors still
apply. The selection gate validates topology and canonical dependency categories,
not a mandatory RuVector identity.

## Build by complete user workflows

1. Before creating a repository, describe the users, at least three substantial
   workflows, full deployment topology and real upstream composition. Reject a utility
   idea before creation. Plan all required crates and design decisions.
2. Write the initial specification, architecture, contracts and actual design decisions.
   Record the required ADR/DDD plan. Do not generate duplicate prose to fill counts.
3. Make the first runnable milestone a browser action reaching the Rust application,
   real upstream execution, authorized project-selected persistence and a displayed result.
   If prerequisites fail, fix them or report the exact blocker; do not substitute a CLI.
4. Expand that slice into all three domain workflows, meaningful domain behavior,
   failure/recovery, responsive frontend, middleware, typed API and durable retrieval.
   Complete the substantive ADRs/DDDs alongside the design and implementation.
5. Verify the complete product and independently inspect it before publication.
   The 20 meaningful Rust-crate floor remains; counts cannot replace product behavior.

Use `PARTIAL_FAILURE`, `INTEGRATION_BLOCKED` or `NONCOMPLIANT_BUILD` for missing
implementation or evidence. Preserve work; do not mark a different smaller project
successful to finish the run. Do not automatically continue it the next night.

## Capture execution on an immutable candidate

Copy `scripts/dream-machine-product-gate.mjs` from a recorded core-memory revision
into the generated project's scripts directory. Record its SHA-256 and provenance.
Wire the recorder and final check into that project's existing `verify-prototype.sh`
and GitHub Actions; a file-existence checker is insufficient.

Commit the candidate first. Generate evidence **outside the source checkout** in a
fresh directory for each complete verification attempt. Do not rewrite a committed
manifest to claim a new SHA. Evidence is a durable CI artifact bound to the source
commit; the repository can link to it without attempting a circular self-hash.

The recorder accepts explicit argv, executes it, captures stdout/stderr and its actual
exit status, and verifies the source stayed clean at the same commit. Commands must
execute the real suites. An `echo`, file counter, mocked backend or claimed boolean
cannot stand in for a suite. Inspect the recorded commands and source of called scripts.

```bash
task_evidence_dir="$(mktemp -d)"
node scripts/dream-machine-product-gate.mjs record . "$task_evidence_dir" rust -- bash scripts/check-rust.sh
# Record actual commands for each remaining suite in the same way:
# backend (exact metadata command above), frontend, api, middleware, integrations,
# benchmark, ablations, containers, browser.
node scripts/dream-machine-product-gate.mjs verify . "$task_evidence_dir" "$(git rev-parse HEAD)"
```

The shown suite script is illustrative: author and inspect each project's real
commands. Keep test data in authorized isolated namespaces, with deterministic seeding
and cleanup; do not publish secrets. Pass `DREAM_MACHINE_EVIDENCE_DIR` to reporters.
`DREAM_MACHINE_SOURCE_SHA` identifies the immutable source being tested.

## Evidence manifest (schema version 2)

Write `manifest.json` in the external evidence directory after execution:

```json
{
  "schema_version": 2,
  "source_sha": "<actual 40-character tested commit>",
  "artifacts": [
    {"kind": "desktop_screenshot", "path": "desktop.png", "sha256": "<actual SHA-256>"},
    {"kind": "mobile_screenshot", "path": "mobile.png", "sha256": "<actual SHA-256>"},
    {"kind": "browser_report", "path": "playwright.json", "sha256": "<actual SHA-256>"},
    {"kind": "browser_trace", "path": "trace.zip", "sha256": "<actual SHA-256>"},
    {"kind": "correlation_trace", "path": "correlation.jsonl", "sha256": "<actual SHA-256>"},
    {"kind": "semantic_review", "path": "semantic-review.md", "sha256": "<actual SHA-256>"},
    {"kind": "composition_report", "path": "composition.json", "sha256": "<actual SHA-256>"}
  ]
}
```

These placeholders document the schema; they never pass verification. Retain dependency
versions, run ID, timestamp, launch mode, test namespace, exact commands and all previous
canonical evidence fields as additional manifest metadata. Recorder receipts are
generated by execution, never handwritten. Preserve raw report and trace artifacts.
Use Playwright JSON output from real browser tests with these title markers:

- `[journey:primary]`: substantive main outcome through the real system.
- `[journey:denial-recovery]`: forbidden effect absent, then compliant recovery succeeds.
- `[journey:durable-retrieval]`: saved result/audit survives refresh and backend restart.

All three must execute and pass. Skips, expected failures, zero tests, failed retries,
missing suites, altered logs/artifacts, dirty source, stale receipts and placeholder
SHAs fail the gate. Use real desktop (width >=1024) and mobile (width 320-600)
screenshots, both height >=480. Prefer fixed local test ports and explicit readiness.

Upload the entire evidence directory with GitHub Actions under a name containing the
tested SHA and retain a durable run/artifact link. Read back actual workflow run
`head_sha`, conclusion, gate logs and artifact contents through the authorized GitHub
connection. Re-run verification on the landing commit after merges. Never relabel
old evidence by changing its source_sha or replacing text in a semantic review.

## October 29 executable composition report

The hashed `composition_report` artifact is mandatory for new builds. Manifest schema
remains version 2; the composition report has its own `schema_version: 2` and exact
tested `source_sha`. Produce it from source inspection and actual execution evidence.
It cannot establish runtime truth by self-attestation; independent review remains mandatory.

Required report fields:

| Field | Contract |
| --- | --- |
| `dependencies` | At least five distinct ruvnet and two enterprise OSS canonical upstreams, with the per-dependency fields below. |
| `history_status` | `VERIFIED`; missing evidence is `SELECTION_HISTORY_UNVERIFIED`, never verified diversity. |
| `history` | Last ten attributable completed builds, newest first, or all verified available history. Each includes `date`, immutable `source_sha`, a real GitHub `evidence_url`, and `dependencies` containing canonical `repo` and `category`. Verify order, completeness and provenance independently. |
| `backend.entrypoint` | Existing Rust service source path relative to the tested checkout. |
| `backend.crates` | At least twenty unique crate records with `manifest_path`, `implementation_path`, `caller_path`, `test_path`. Each manifest is an actual Cargo.toml. Independently verify workspace membership, substantive code and real callers. |
| `middleware.source_paths` | Nonempty list of implemented middleware/registration source paths. |
| `middleware.denial_test_path` | Executed denial/failure test source path. |
| `middleware.runtime_evidence_path` | Hashed runtime trace artifact path. |
| `journeys` | Records covering `primary`, `denial-recovery`, `durable-retrieval`, with the per-journey fields below. Multiple primary records may cover distinct domain workflows. |

Every dependency record includes:

- `repo`: canonical `owner/repository`, without a URL, package suffix or `.git`.
- `category`: `ruvnet` (owner ruvnet) or `enterprise` (different owner).
- `revision`: exact forty-character upstream commit; `license`: verified OSS license.
- `responsibility`: actual distinct product responsibility.
- `manifest_path`, `call_site_path`, `test_path`: existing relative source files.
- `runtime_evidence_path`: actual hashed trace evidence of upstream execution.

Every journey includes `name`, `frontend_path`, `api_path`, `middleware_path`,
`backend_path`, an observed `correlation_id`, `runtime_evidence_path`,
`crate_manifest_paths` and `dependency_repos`. Source paths must exist within the
immutable checkout. All counted crates must occur in executed product workflows; the
primary workflow records collectively include every counted upstream. Denial may list
no downstream dependencies when policy prevents calls.

Every referenced runtime evidence file must be listed and hashed in the manifest with
kind `integration_execution` or `correlation_trace`. Retain actual raw traces; trace
references, source files and a matching checksum do not alone prove execution.

The checker rejects sub-floor or duplicate upstreams, stale reports, missing source/trace
references, missing backend/middleware evidence, disconnected crate mappings and unmet
rotation. At least two ruvnet and one enterprise project must differ from the previous
completed build; the full ingredient set cannot repeat within verified history.
Independent review must additionally resolve forks/aliases, verify upstream ownership
and licenses, inspect actual crate linkage and middleware order, reconcile report
correlation IDs with raw traces, and inspect real browser assertions and failure effects.

The selection receipt must also show the ten-ruvnet/five-enterprise research shortlist,
usage frequency and choice rationale required by the build standard. Do not fabricate
history or pad dependencies to pass the checker. Apply new floors prospectively.

## November 1 pre-repository selection and readiness artifact

Copy scripts/dream-machine-product-gate.mjs,
scripts/dream-machine-selection-gate.mjs and scripts/dream-machine-backend-gate.mjs
from the same recorded revision.
Before remote repository creation, execute:

```bash
node scripts/dream-machine-selection-gate.mjs "$task_selection_json" "$task_probe_evidence_dir"
```

Use task-specific paths resolved by the executor. Probe source is a clean, immutable
disposable local git workspace, not an empty published project. Use the existing
product recorder with suite integrations to execute each real upstream operation
and failure-path test separately. Preserve every actual receipt and raw log plus
a hashed archive of that probe source. The recorder captures actual argv/exit codes.
Inspect the code and upstream outputs independently: a zero-exit version probe,
echo or mock does not establish readiness. Do not probe outside authorized resources.

The final manifest additionally requires kind selection_report. Its JSON schema is:

| Field | Required value |
| --- | --- |
| schema_version | 1 |
| attempt_id, date, selected_at | Stable attempt identifier, Toronto date, actual ISO timestamp after probes complete. Selection must be within 72 hours of verification. |
| probe_source_sha | Exact clean probe-source commit. |
| probe_source_archive | Object with relative path and actual sha256 of archived probe source in evidence directory. |
| product | users, capability, at least three distinct bounded_contexts; topology with frontend/backend/middleware/persistence; workflows with name and user_outcome for primary, denial-recovery, durable-retrieval. |
| shortlist | At least ten unique Ruvnet and five enterprise upstreams; each has repo, category, immutable revision, license, capability, decision (selected/rejected), rationale and source_url of the exact inspected GitHub tree revision. |
| dependencies | At least five Ruvnet and two enterprise selected upstreams, matching selected shortlist entries. Each has repo/category/revision/license, responsibility, actual binding, test_path and probes. |
| dependencies[].probes | Separate success and failure objects, each path + sha256 of an actual recorder receipt. Receipt suite is integrations and source_sha matches probe_source_sha. Logs must exist and match recorded hashes; every boundary/mode needs its own receipt. Failure-path harness exits zero only after asserting correct failure behavior. |
| history_status | VERIFIED_ATTEMPTS; unavailable evidence is SELECTION_HISTORY_UNVERIFIED. |
| history | Last ten attributable attempted projects, newest first, or all independently verified available. Each has attempt_id, attempted_at, status, source_sha, immutable GitHub evidence_url containing that SHA, selected_dependencies and executed_dependencies (repo/category entries). Include failed, partial and scaffold projects. |

At least two selected Ruvnet projects and one enterprise project must be absent
from the UNION of selected ingredients in the latest THREE attempts; no complete
set can repeat across ten attempts. Compute usage frequencies from all attempts,
including failed selections, for review. The prior completed-build comparison is
retained in composition_report; passing it cannot override attempted-build rotation.
Do not omit failed projects to make novelty pass. Missing history must be resolved
or reported honestly, never invented. No automatic bootstrap exception is provided.

Carry the original selection JSON, probe archive, receipts and raw logs into the
final CI evidence directory without rewriting them. The product checker validates
selection evidence and requires the composed canonical upstream set AND revisions
to match the selection. New selections require new probes; final runtime evidence
still must prove all selected integrations on the exact final candidate.

The selection checker verifies byte integrity, required structure, chronology,
floors and rotation; it cannot authenticate self-authored reports, inspect an
archive's semantics, prove source provenance or confirm history completeness.
The executor and Guard must inspect actual probe source and log contents, licenses,
fork/alias identity and attributable attempt evidence. They must also independently
review the working primary workflow, real crate substance and final product.
Synthetic fixtures under scripts/fixtures are gate regression tests, never product
integration evidence. Do not copy their invented upstreams or receipts into builds.

## Limits and independent completion decision

`EVIDENCE_CHECKS_PASS` is necessary, and intentionally is **not** `BUILT`. The gate
checks binding, completeness and byte integrity. It does not establish that an author
is honest, screenshots represent the real product, every crate is meaningful, browser
assertions test real behavior, an upstream is really executed, or a workflow came from
trusted CI. Inspect those separately against source, UI, actual requests and raw traces.
Review crate responsibilities/callers and reject trivial splitting, duplicated domain
functions, decorative integrations, generic one-form UI and templated design prose.

The Guard must reconcile contradictory evidence, inspect the exact same source and
artifacts, and issue `NONCOMPLIANT_BUILD` instead of repeating an executor's success.
Only apply `BUILT` when this necessary check, substantive product review and every
other existing contract requirement pass. A real environment blocker is reported
honestly; the gate must never fabricate it or suppress useful implementation work.



