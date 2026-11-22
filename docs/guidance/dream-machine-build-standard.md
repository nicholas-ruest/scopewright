# Nick's Dream Machine build standard

Authority: Nick's explicit instructions, October 11, 16, 22, 29 and November 1, 2026.
Scope: every Dream Machine generated project. This is build guidance, not merely rejection criteria.
Supersedes previous four-crate floors and older automatic multi-night continuation wording.

## November 5 completion execution and Core Memory pairing

ControlSurface stopped at a four-crate slice. Its CI verified checker bytes but did
not execute the product verifier. Copying gates, reporting PARTIAL_FAILURE and passing
slice diagnostics did not produce the required full product. The saved report gives
missing implementation checkpoints but no concrete external blocker for ending there.

### Execute the completion gate from the first implementation commit

Copy `scripts/dream-machine-verify-product.sh` to the generated product as
`scripts/verify-prototype.sh`, retaining the canonical three-module bundle and its
manifest/verifier. Keep the preliminary smoke checks in a separately named
`scripts/verify-slice.sh`. CI must run an unconditional **Full product gate** job on
the exact clean PR head; a green slice job cannot stand in for it. Do not mark the full
job optional, continue-on-error, or skipped merely because implementation is partial.
The canonical modules and their hashes are unchanged by this correction.

Implement the product-specific `scripts/collect-product-evidence.sh`. It must launch
the real stack, execute all ten recorder suites (rust, backend, frontend, api,
middleware, integrations, benchmark, ablations, containers, browser), preserve the
original selection/probe receipts and raw logs, and assemble the real external
schema-2 evidence manifest. Use the exact Cargo metadata command already specified.
Provide screenshots, Playwright JSON/trace, correlated runtime logs, composition schema
2 and independent substantive semantic review. Missing collection or evidence is a
nonzero product failure. Do not fill gaps with synthetic artifacts or placeholder scripts.

The wrapper runs the collector, then always executes the canonical product verifier,
even after collection fails. It refuses dirty source, altered gate bytes and evidence
inside the source checkout. Keep generated evidence in the supplied external directory
and retain it as a durable exact-source CI artifact, including on failure.

A partial slice is an intermediate milestone. During the authorized run, continue
implementing actual upstream composition, complete domain workflows, all meaningful
crates, decision-specific ADRs/DDDs and whole-system tests. Do not end simply because
the slice or its CI is green. If execution must end, give the specific failing command,
exit/error, unavailable capability or actual execution-limit evidence, the last
milestone and exact remaining work. A status label or missing implementation list
alone does not explain the blocker. Never invent runtime budgets or continuation
capabilities. Preserve one new project per date and no automatic next-night continuation.

### Pair every build with Core Memory and its examples

Before selection, call the live Core Memory MCP task-start operation in namespace
`dream-machine` with the concrete product, stack and constraints. Read this guidance,
AGENTS.md, CLAUDE.md, applicable architecture/ADR/DDD examples, learning-loop.md,
ADR-0026, the feedback schema and current learning policy at recorded revisions.
Use Mandate Mesh as the accepted product-shape example; inspect actual immutable
backend, frontend, middleware, persistence, tests and launch files. Its shape is an
example, not proof that it satisfies today's stronger integration/evidence floors.

Create `docs/core-memory-guidance-map.md` and `LESSON_APPLICATION_MAP` with exact
guidance/example sources and SHAs, relevant decisions, actual code/test paths and
measured evidence. Revisit these at the primary-integrated and final-product milestones.
An empty/unscreened retrieval is recorded honestly; it neither replaces canonical
guidance nor proves live memory quality. Raw/proposed/held feedback remains evidence;
only verified approved lessons may become recalled learned guidance.

After every success or failure, call the existing MCP outcome recorder with the actual
source-bound PR/commit/issue. Submit or reuse one deduplicated structured
`memory-feedback` issue in core-memory, using the existing issue schema and trusted
source scope. Preserve the route: outcome -> RuVector journal -> GitHub feedback ->
bounded proposal/evaluation -> human review -> verified recall -> subsequent measured
build outcome. Record actual responses, issue URL, lesson IDs and proposal/held status.
A memory ACK, workflow success, journal counter or proposed lesson never proves
optimization or product completion. Do not change promotion, review, storage or
authorization boundaries. Core Memory's own RuVector backing is separate from whether
RuVector is a useful runtime dependency of each generated product.

The Guard must audit this pairing as well as the full product. Check actual recall,
guidance/example mappings, outcome record and feedback readback; report feedback
processing failures separately from product failures. Do not alter the existing
self-improvement loop, enable autonomous code changes, or auto-merge lessons.

## Required outcome
Build a real functional high-fidelity prototype: a coherent Rust backend composing the
crates, working TypeScript frontend, middleware, authorized intelligence/persistence and
reproducible full-stack startup with executed browser acceptance evidence. Maintain a
MINIMUM OF 20 meaningful implemented Rust
workspace crates per project, at least 25 substantive ADRs and 12 substantive DDD
documents. Small two-file CLIs are unacceptable. A CLI can be one interface to the system.
Twenty crates is a floor, not permission to split trivial functions into ceremonial packages.

## November 2 execution integrity repair

Before running the gates, fetch `config/dream-machine-gate-bundle.json` and all three
gate modules from the same immutable guidance revision. Execute
`node scripts/dream-machine-gate-bundle.mjs` in that checkout. The manifest records
the exact byte length, SHA-256 and Git blob identity of each module. Compare the
Git blob identities with the pinned tree as well; a self-authored manifest is not
independent provenance. Copy the verifier and manifest with the modules and verify
again in the product checkout. Never normalize whitespace or append a newline.
The former selection hash `cc643d01…939af25` described a local copy with an extra
newline. The repository bytes at `836ca9b7623b11c143223cd0a481e2263add4012`
have SHA-256 `27e99d82a90012110e097e9d94d0cfd2f9577c4ef12686fc589d68674e26ef49`.
The bundle check proves bytes only; it cannot establish a working product.

Check executable paths, writable capacity and actual runtime behavior in the chosen
authorized environment before declaring prerequisites absent. Record a compiled Rust
smoke test, real browser launch and required container capability. Repair routine
prerequisites within existing authority; missing capability never permits CLI scope.
Every execution/publication receipt must name the actual producer and run identifier,
effective guidance revision, bundle hashes and immutable product source. A repository's
creation time or commit author does not establish which automation created it.
Keep product completion separate from validator tests, memory ACKs and schedule state.

## November 1 full-system restoration

The restored baseline is **25 ADRs, 12 DDD documents, 20+ meaningful Rust crates,
a proper integrated Rust backend, executable middleware and a working TypeScript
frontend**. Improvements are additive to this baseline. This supersedes older
approximate document counts. Scope cannot fall back to a CLI, crate collection or
frontend shell. A CLI may be an additional interface to the complete product.

Implement the backend composition root and application services that invoke the domain
crates. Use typed ports/adapters for upstreams and authorized persistence. Wire the
TypeScript API client through registered authentication, authorization, policy,
validation, error, correlation, timeout and observability middleware into those services.
The domain determines the boundaries; do not create twenty trivial packages for a count.

Copy all THREE source-pinned gate modules: product, selection and backend. The backend
gate now requires actual Cargo metadata from the backend recorder suite, checks the
service binary target, workspace membership and production dependency reachability,
and rejects dev-only/disconnected crates. Composition schema 2 includes per-crate
responsibilities, APIs, caller/failure tests, reciprocal ADR/DDD mappings, a typed API
contract/client/drift test and explicit middleware registration/order/control tests.
See the evidence gate for the exact schema. These are necessary checks; independently
review actual function calls, substantial behavior, design quality and raw runtime proof.

After prerequisite probes and a concise architecture/coverage plan, first demonstrate
the real browser-to-backend workflow. Then implement and test complete domain workflows,
finish every substantive crate and all 25/12 design documents, and execute the entire
immutable-source verification before claiming BUILT. A failed final audit is not a
replacement for implementing the product; keep working within the authorized run.
If blocked, preserve the actual work and report the earliest failed checkpoint exactly.
No change to schedules, dependency diversity, publication/approval authority or the
prohibition on unrequested continuation is implied.

## November 1 dependency-choice clarification

RuVector/RVF is **optional for Dream Machine builds**. It will likely serve a useful
role in many projects, but select it only when its capabilities fit the actual
product. No fixed upstream is mandatory across every build. When omitted, document
the appropriate authorized intelligence and persistence architecture and prove the
same durable product workflows against that implementation. Do not reject a build
solely because RuVector is absent, and do not add it as a decorative dependency.

The minimum five distinct executed Ruvnet and two enterprise OSS upstreams, rotation,
real integration evidence, full application requirements and existing authorization,
audit and shared-storage policies still apply. This clarification supersedes older
Dream Machine wording that mandates RuVector/RVF or names it as the only allowed
intelligence/persistence implementation. It does not change unrelated projects.

## November 1 regression repair — readiness before creation and attempted-build rotation

The October 29 rules remain mandatory. The repeated failures show that end-of-run
rejection alone does not produce a working application. Apply these checkpoints
before expanding the project; never manufacture a token implementation to finish.

1. **Check prerequisites before creating a remote project.** In a disposable local
   probe workspace, pin and install each of the >=5 Ruvnet and >=2 enterprise upstreams.
   Execute an actual intended operation AND a failure-path assertion for each.
   Use the product recorder's integrations suite to capture separate receipts/raw logs
   for each boundary and test mode on a clean immutable probe commit. A failure-path
   test exits zero only when the expected failure behavior is asserted. Preserve a
   hashed archive of the actual probe source. Version/help/package metadata, imports,
   availability checks, README examples and mocks cannot satisfy these probes.
   Inspect the actual scripts, input/output, upstream call paths and authorization.
2. **Check variety against attempts, including failures.** Read the last ten
   attributable attempted projects, newest first, or all independently verified
   available history. Include selected and executed upstream sets plus outcome and
   immutable evidence for PARTIAL_FAILURE, NONCOMPLIANT_BUILD and blank/scaffold
   projects. Do not skip them to compare only against an older successful build.
   Select >=2 Ruvnet and >=1 enterprise upstreams absent from the UNION of the last
   three attempts. No identical complete ingredient set within ten attempts.
   RuVector may be selected when useful; recurring coordination tools
   cannot count as new product integrations. Retain the older completed-build
   comparison as an additional gate.
3. **Execute the selection gate.** Produce the selection_report defined in
   dream-machine-product-gate.md, with the ten/five researched shortlist,
   complementary responsibilities, real probes, full topology and three workflow
   plans. Execute scripts/dream-machine-selection-gate.mjs before remote repository
   creation, then independently inspect its evidence. If a useful compatible set
   cannot pass, examine up to three candidates within the same authorized run.
   Correct prerequisites or select a different product before repository creation;
   otherwise report the exact PRECHECK_BLOCKED/INTEGRATION_BLOCKED evidence.
   Do not create an empty remote project merely to satisfy a daily activity count.
4. **Demonstrate the whole product early.** After a concise system design and ADR/DDD
   coverage plan, implement the primary browser -> typed API -> middleware -> Rust
   orchestration -> real upstreams -> authorized project-selected persistence write/read -> rendered
   result. Prove all seven counted boundaries in primary workflows before generating
   the remaining crate structure or bulk documentation. Build meaningful domain
   behavior and failure/recovery next. Reserve most of the implementation window
   for working software and whole-system verification; do not confuse a plan,
   scaffold, shell handoff or coordination receipt with implementation.
5. **Review substance at checkpoints.** Review the runnable primary workflow before
   expanding, and review full workflows before final publication. Every counted
   crate needs a useful domain responsibility, callers, implemented behavior and
   failure tests. Reject artificial splits and repeated ADR/DDD prose. Preserve
   >=20 meaningful Rust crates, at least 25 substantive ADRs and at least 12 DDDs; completing these
   requirements remains necessary before BUILT. No arbitrary line-count threshold
   replaces semantic review.
6. **Keep selection bound to execution.** Copy all three pinned gate modules into the
   generated project. Include the hashed selection_report, probe source archive,
   recorder receipts and raw logs in final durable CI evidence. The final checker
   rejects missing selection evidence or a different executed ingredient set.
   If any selected upstream/revision changes, redo readiness/selection and execute
   the full final verification on the new source; do not silently substitute it.

These checkpoints change implementation order and selection enforcement, not schedules,
scope, spend, approval authority or the prohibition on automatic next-night continuation.
A gate pass is necessary evidence, never a guarantee of product quality or permission
to merge/deploy. Report the exact earliest failed checkpoint and preserve actual work.

## October 29 composition, diversity and backend correction

Every build requires **at least five distinct ruvnet upstream projects and two distinct
enterprise open-source upstream projects**, integrated into the running product.
Count canonical upstream projects once: multiple crates/packages/features from RuVector,
RVF or another monorepo do not become multiple projects. Forks and renamed wrappers do
not add a new ingredient. An upstream cannot occupy both categories. Record each
immutable revision, OSS license, useful responsibility, dependency/service binding,
real call site, failure/integration test and source-bound runtime trace. Unused manifest
entries, README mentions, mocks and build-time coordination alone do not count.

Before selection, inspect current primary-source upstreams, Radar/Scout discoveries and
the previous ten attributable completed builds, or all verified available history.
Evaluate at least ten relevant ruvnet and five enterprise OSS candidates across useful
capabilities. Keep a dated usage/selection receipt with sources, usage frequency, fit,
alternatives and selected/rejected rationale. Reuse useful foundations while selecting
at least **two ruvnet projects and one enterprise project absent from the immediately
previous verified completed build**. Do not repeat a full ingredient set from the
verified last ten builds. State missing history; unverified diversity cannot pass.
Choose a different candidate if useful compatible integrations cannot satisfy the floors.
Do not add ornamental dependencies to satisfy counts.

The backend must actually compose every counted meaningful Rust crate through product
application use cases or attached background workflows. Development-only and disconnected
crates cannot contribute to the twenty-crate floor. Publish a crate-to-backend call-path
map with service entrypoints, application callers, domain behavior, middleware
registration/order, upstream boundaries and executed tests. Implement authn/authz/policy,
validation, errors, correlation, timeouts and observability; in-process Rust middleware
is valid. A separate BFF is optional, implemented middleware is mandatory.

Primary product workflows must collectively execute all counted upstream projects.
Prove browser -> typed API/client -> middleware -> Rust orchestration/domain crates ->
real upstream composition -> authorized project-selected persistence -> displayed result.
Execute denial/recovery with no forbidden downstream effect, and restart-persistent
retrieval. Report FRONTEND, BACKEND_COMPOSITION, MIDDLEWARE, RUVNET_EXECUTED n/5,
ENTERPRISE_EXECUTED n/2 and DEPENDENCY_DIVERSITY separately. Missing layers, thresholds,
rotation or proof forbid BUILT. Preserve the actual partial product and exact gaps.

Copy the current source-pinned product checker and emit its hashed `composition_report`
artifact described in [the evidence gate](dream-machine-product-gate.md). The checker
validates thresholds and evidence references; independent review must still establish
actual execution, meaningful code, canonical upstream identity and trustworthy provenance.
This correction applies prospectively and supersedes all earlier 2/1 integration floors.

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
4. **Real data and intelligence.** Choose intelligence and persistence capabilities for
   the product's actual needs; RuVector/RVF is optional. If selected, use its authorized
   interfaces. Document and validate an appropriate authorized alternative when omitted.
   Use Cloud SQL Postgres where required by the governing architecture. Preserve
   append-only auditable, policy-enforced memory and existing shared-storage boundaries.
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
2. Research technically complementary ingredients using primary sources: at least five distinct
   Ruvnet upstream projects, two distinct enterprise OSS projects, and one frontier research mechanism.
   Pin revisions/licenses/interfaces and define real executed integration boundaries.
3. Produce specification and architecture, then at least 25 decision-specific ADRs
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
| Intelligence | product-selected intelligence integration, memory retrieval, learning/evaluation integration |
| Upstream composition | Ruvnet component adapter, enterprise OSS adapter, research mechanism |
| Runtime | configuration, observability, cancellation/timeouts, failure recovery |
| Interfaces/evidence | service API, operator CLI, append-only evidence, end-to-end evaluation |

Choose at least 20 justified crates from the actual domain design; this example offers 22
responsibilities. Combine or replace roles appropriately while retaining the 20-crate minimum.
Use the product-selected intelligence/persistence implementation through authorized
append-only, auditable, policy-enforced paths. RuVector/RVF remains optional. GCP services remain stateless and IAM-scoped, configured through
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




