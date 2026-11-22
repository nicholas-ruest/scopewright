# Nick's Dream Machine build standard

Authority: Nick's explicit instructions, October 11, 16 and 22, and November 7, 2026.
Scope: every Dream Machine generated project. This is build guidance, not merely rejection criteria.
Supersedes previous four-crate floors and older automatic multi-night continuation wording.


## November 7, 2026 — mandatory Rust majority and source hygiene

Every generated product must contain **at least 50% genuine Rust source code**.
This supplements the meaningful-crate, full-stack, dependency and evidence gates;
20 nominal crates alone cannot satisfy the Rust requirement.

Measure committed source-code **bytes**, with Rust bytes divided by all recognized
tracked source bytes. Include product code, frontend, integration adapters, tests,
shell scripts and verification/evidence tooling in the denominator. Exclude prose,
lockfiles, data, images and binary artifacts. The deterministic extension list is
in [the Rust-share checker](../../scripts/dream-machine-rust-share.mjs).
This source-byte audit is separate from GitHub's language badge.

Use the checker on an exact clean commit before publication and again at the final
CI head. Retain its complete JSON, file inventory and source SHA in external
source-bound evidence. A nonzero exit status blocks publication and BUILT:

```bash
node scripts/dream-machine-rust-share.mjs --self-test
node scripts/dream-machine-rust-share.mjs . > "$DREAM_MACHINE_EVIDENCE_DIR/rust-share.json"
```

Install this small additive checker from an immutable core-memory revision. Run it
as a separate CI step before the existing canonical full-product verifier; preserve
the existing gate bundle bytes and hashes. The checker rejects dirty source,
source symlinks, tracked dependency/build trees, TypeScript build caches and agent
session/authentication state. Vendored or generated Rust contributes no Rust
credit; its source bytes remain in the denominator.

Do not pad Rust files with comments, duplicated tests, unused wrappers, copied
upstream code or uncalled functions to change the percentage. Do not relabel or
hide JavaScript through .gitattributes, language overrides, vendored/generated
labels, extensions or exclusion flags. Independent semantic review must confirm
substantive Rust domain/backend behavior and inspect large generated/copied files.
Keep the functional TypeScript frontend and required upstream execution. Build
enough useful Rust or port appropriate behavior with regression tests; do not
remove product functionality to meet the percentage.

Report RUST_SOURCE_SHARE, RUST_BYTES, TOTAL_SOURCE_BYTES and SOURCE_HYGIENE alongside
the existing product verdicts. A passing percentage alone never establishes a
complete or high-quality product.

## Required outcome
Build a real functional high-fidelity prototype: a coherent Rust backend composing the
crates, working TypeScript frontend, middleware, authorized intelligence/persistence and
reproducible full-stack startup with executed browser acceptance evidence. Maintain a
MINIMUM OF 20 meaningful implemented Rust
workspace crates per project, approximately 25 substantive ADRs and 12 substantive DDD
documents. Small two-file CLIs are unacceptable. A CLI can be one interface to the system.
Twenty crates is a floor, not permission to split trivial functions into ceremonial packages.

## October 26 regression correction — executable enforcement

Apply [the product evidence gate](dream-machine-product-gate.md) in every generated
project and its CI. Plan the complete design before implementation, then build by
browser-to-Rust-to-upstream/persistence vertical slices. The first runnable milestone
must display a real persisted result in the TypeScript frontend. Expand complete user
workflows before polishing counts or prose. Never switch to a smaller CLI when time,
an executor, or an upstream is constrained; preserve the actual project and report
its exact missing gates. The 20 meaningful-crate and substantive ADR/DDD requirements
remain mandatory.

Copy the source-pinned recorder/checker into the project, record actual suite execution,
and run the evidence check on a clean immutable commit. Keep generated acceptance
evidence outside the checkout as a durable CI artifact bound to that source SHA.
Do not mutate or rebind old evidence to a later commit. `CI_REBIND_REQUIRED`, missing
raw logs, zero/skipped journeys and contradictory semantic reviews forbid BUILT.
`EVIDENCE_CHECKS_PASS` is necessary but never replaces independent product/source/UI
review. Read back exact-run artifacts, workflow head SHA and final repository state.

This correction takes precedence over any older ordering that completes all prose
or counts implemented crates before the first functional slice. Design checkpoints
validate design coverage; implementation checks run only after implementation.

## Mandatory functional prototype contract — October 22, 2026

The unit of delivery is a usable, high-fidelity, end-to-end solution. Rust crates,
ADRs and DDD documents support that solution; their counts alone never establish completion.
Every generated project MUST include all of the following, in the same repository:

1. **Coherent Rust backend.** Run a real service that composes the domain crates into
   application use cases and exposes versioned, typed APIs. Include validated requests,
   explicit error contracts, health/readiness, configuration, structured tracing and
   cancellation/timeouts. Document the actual API contract (OpenAPI or an equally precise
   schema) and provide a typed TypeScript client with a contract-drift check.
2. **Working TypeScript frontend.** Default to React + TypeScript unless a documented
   technical reason selects another TypeScript framework. Build a polished, domain-specific
   application with coherent navigation, responsive layouts, accessible forms/tables,
   meaningful visualizations where useful, and loading, empty, validation, success and
   error states. All primary controls must perform real actions and display real backend
   results. A README, static landing page, generic dashboard, CLI, screenshots or a UI
   backed solely by hardcoded/mock data cannot satisfy this requirement.
3. **Integrated middleware and adapters.** Wire frontend requests through authentication
   and authorization/policy checks, request validation, consistent errors, correlation IDs
   and observability into the Rust application services. Apply bounded retries,
   idempotency, rate limits, job/status handling or streaming where the use case requires
   them. Middleware can live in the backend; add a separate TypeScript BFF only if needed.
   Browser code must never contain service credentials or directly bypass policy.
4. **Real data and intelligence.** Use RuVector/RVF through authorized interfaces as the
   single intelligence/persistence layer, backed by Cloud SQL Postgres where required by
   the governing architecture. Preserve append-only auditable, policy-enforced memory.
   No service-owned database. Provide deterministic, clearly labeled synthetic seed data
   entering through real application interfaces, with reset/isolation and retention
   instructions; keep test effects in an authorized test namespace. Seeded data is
   acceptable; fabricated live results are not. In-memory test doubles do not prove
   durable integration or justify a successful prototype verdict.
5. **Complete product workflows.** Before implementation define at least three
   domain-specific acceptance journeys: the main user outcome, a substantive
   validation/policy/dependency failure and recovery path, and saved-result/audit retrieval
   after refresh or service restart. Each must connect UI -> API/middleware -> actual
   Rust application/domain crates -> required upstream/persistence boundary -> visible
   user result. Document inputs, expected behavior and required evidence. Use the main
   journey to execute the required Ruvnet and enterprise OSS composition for real.
6. **Reproducible launch and delivery.** Include locked Rust/Node dependencies, pinned
   toolchains, a safe .env.example with no credentials, Linux bash bootstrap/start/stop
   commands, deterministic seed setup, container build(s), documented ports/URLs and
   health checks. Supply a single documented command that starts the complete prototype
   from a clean checkout once stated authorized prerequisites are present. Include GCP
   Cloud Run-compatible deployment configuration and rollback instructions for stateless,
   IAM-scoped services; function wrappers only where needed. Verify the containerized
   full stack locally/in CI. Cloud deployment, spend and production promotion retain
   their existing authorization requirements.
7. **Executed whole-system verification.** Actually run Rust quality checks, frontend
   typecheck/lint/unit tests/production build, API/contract tests, middleware
   authorization/validation/failure tests, and browser end-to-end tests (Playwright or
   equivalent) against the running frontend AND real backend. Do not intercept or mock
   first-party API routes in the acceptance suite; isolated mocks in unit tests are fine.
   Test the three journeys, actual result rendering, failed dependencies, policy denial
   with no forbidden effect, and refresh/restart retrieval. Preserve real upstream
   integration, benchmark and ablation gates as additional requirements.
8. **Reviewable proof.** Commit docs/prototype-acceptance.md mapping each journey to UI
   route/control -> API -> middleware -> application/crates -> upstream/persistence ->
   test/evidence. Commit a safe evidence/prototype/manifest.json binding tested source
   identity, dependency versions, commands, exit codes, timestamps, launch mode, test
   namespace and artifact paths/hashes. Include actual desktop/mobile browser screenshots,
   browser test traces/report and an observed UI-to-backend correlation trace. Retain
   large evidence as durable CI artifacts with verified links rather than committing
   unnecessary binaries. Distinguish synthetic fixtures, measured results and unavailable
   external services. Never publish credentials, sensitive records or internal endpoints.

### Build order and completion gate

Plan a bounded real product, its user journeys, UI flows and complete deployment topology
alongside the architecture/ADR/DDD work. Make the first runnable vertical slice connect a
browser action to real Rust behavior, storage and a displayed result; expand it into the
complete prototype before spending remaining time on polish and additional documentation.
Reserve implementation and validation time for frontend, backend wiring, middleware,
startup and browser QA. Do not finish the crate list first and leave product integration
as future work. TypeScript packages and UI screens are additional deliverables and do not
count toward the minimum 20 meaningful Rust crates. Do not inflate the crate count.

Implement scripts/verify-prototype.sh (or a documented equivalent) and wire it into GitHub
Actions to launch the full stack, wait for readiness, run contract and browser acceptance
checks, and return nonzero for failures. Checking file existence, counting artifacts or
trusting self-reported booleans is insufficient. Record the exact candidate tested and
revalidate after changes. Independently inspect the UI, live call paths and raw evidence.

BUILT is forbidden if any required layer, primary user action, real integration,
reproducible startup, browser journey, quality check or proof is missing. A polished UI
with disconnected APIs and a large crate workspace without a working UI both fail.
Report PARTIAL_FAILURE or NONCOMPLIANT_BUILD with the dated project's exact missing
acceptance criteria; also name dependency/environment blockers explicitly. Preserve
partial work without quietly substituting mocks or reducing the scope to a crate demo.

The final report must provide the runnable command, verified local URL and/or authorized
preview URL (label ephemeral URLs), implemented user journeys, backend/frontend/middleware
and storage status, actual test/CI results, screenshots/trace/evidence links, required
crate/ADR/DDD counts, upstream integration evidence and honest remaining limitations.
Do not claim a deployed preview, successful execution or production readiness from
configuration files alone.

## Start every run with guidance
Read this file and current AGENTS.md, CLAUDE.md, applicable architecture/ADR/DDD guidance
from nicholas-ruest/core-memory before selecting an idea or implementing.
Use the exposed Core-Memory task_start tool with namespace dream-machine and the concrete
situation. Record empty/unscreened retrieval honestly; repository guidance remains mandatory.
Core-memory specifies HOW Nick wants work built. It is not a decorative project dependency.
Create docs/core-memory-guidance-map.md: source + SHA -> instruction -> design decision ->
crate/document/test -> evidence. Revisit it at each checkpoint.

## Positive build sequence
1. Capture the original problem, users, realistic workload, success metrics and constraints.
2. Research technically complementary ingredients using primary sources: at least two
   Ruvnet components, one enterprise OSS component, and one frontier research mechanism.
   Pin revisions/licenses/interfaces and define real executed integration boundaries.
3. Produce specification and architecture, then approximately 25 decision-specific ADRs
   and 12 DDD documents before implementation. ADRs explain context, viable alternatives,
   decision, tradeoffs, consequences and validation. DDD covers bounded contexts, ubiquitous
   language, aggregates, entities, value objects, invariants, events, context map, ownership
   and ports/adapters. Make documents specific to this system.
4. Write the implementation contract and a minimum 20-crate workspace map. For EACH crate:
   responsibility, bounded context, typed public API, callers/dependency edges, upstream
   role where relevant, ADR/DDD references, implementation tasks, tests and acceptance criteria.
   Specify the full-stack topology, API contracts, UI flows and the three required browser
   acceptance journeys crossing the system; map crates into the backend's live call paths.
5. Implement vertical slices through the complete browser-to-backend workflow. Each increment adds real behavior,
   integration and failure-path tests. A crate counts only if it is a workspace member,
   has substantive implemented behavior, has real callers or a useful executable entrypoint,
   and participates in tested system behavior. Empty skeletons, unused adapters, stub APIs,
   mock-only integrations and arbitrary file splitting do not count.
6. Execute real upstream integrations, failure recovery, policy enforcement and evaluation.
   Run locked workspace tests, format, strict Clippy and dependency/license/security checks.
   Execute the full-stack launch, frontend, middleware, API/contract and browser acceptance
   checks required above. Benchmark reproducibly against applicable single-ingredient
   baselines; run ablations.
   Use actual available Ruflo coordination and MetaHarness/Darwin/Flywheel capabilities
   with source-bound execution evidence. Never confuse coordination with implementation.
7. Complete an enterprise-grade README: quickstart, realistic input/output, workspace and
   architecture maps, integration matrix, benchmark/ablation results, limitations, ADR/DDD
   links, professional licensed icons and two project-specific animated SVGs.
8. Independently read back source, docs, tests and raw evidence; run the structural checker.
   Publish/merge/Gist only through existing authorized workflows and passing required checks.
   A checker pass alone does not prove integration execution or substantive design.

## Example planning scaffold: adapt to the actual domain
These responsibilities illustrate system scale; they are not a required naming scheme.
Every selected responsibility must be needed and implemented.

| Layer | Candidate responsibilities |
| --- | --- |
| Domain | domain model, typed contracts, invariants, domain events |
| Application | orchestration, workflow state, scheduling, policy decisions |
| Intelligence | RuVector integration, memory retrieval, learning/evaluation integration |
| Upstream composition | Ruvnet component adapter, enterprise OSS adapter, research mechanism |
| Runtime | configuration, observability, cancellation/timeouts, failure recovery |
| Interfaces/evidence | service API, operator CLI, append-only evidence, end-to-end evaluation |

Choose at least 20 justified crates from the actual domain design; this example offers 22
responsibilities. Combine or replace roles appropriately while retaining the 20-crate minimum.
Ruvector is the intelligence/persistence layer; use authorized append-only, auditable,
policy-enforced paths. GCP services remain stateless and IAM-scoped, configured through
environment variables; no service-owned databases. Rust core, required TypeScript frontend and typed API integration,
Linux-first deterministic builds, GitHub Actions and explicit rollback.

## One new standalone project per daily run

Nick's October 16 instruction supersedes the older automatic multi-night continuation
rule. Select exactly one materially new standalone project per run date. Compare recent
outputs and record novelty in the daily-build ledger. JitterMap and ToolRecall are
quality benchmarks/prior art only; maintenance, repair, receipts, documentation changes,
renamed utilities and another increment to a previous project do not meet the daily goal.
Choose a bounded product that can meet the complete prototype and existing scale gates.
If no suitable candidate exists, report NO_PROJECT_WORTH_CREATING honestly.

If the new project cannot meet all gates in the available run, preserve its branch,
source state, design, implemented counts, runnable components, evidence and exact missing
acceptance criteria; report PARTIAL_FAILURE or NONCOMPLIANT_BUILD. Do not automatically
resume it the following night unless Nick explicitly requests that exact continuation.
Do not start overlapping duplicate runs or change schedules/publishing authority.

A successful report must include the functional prototype evidence above in addition to
meaningful Rust crate count, ADR/DDD counts, real integration execution,
benchmark/ablation evidence, checker/CI outcomes and publication receipts.

