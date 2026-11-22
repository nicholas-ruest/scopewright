# Observability bounded context

## Ubiquitous language
`CorrelatedSpan` is the aggregate root and `TraceContext` is the value carried across its boundary. A factory run is not complete merely because a command exited successfully; it becomes a saved governance decision only after every required responsibility returns source-bound evidence.

## Responsibility
This context must join browser, middleware, Rust and upstream work. Its public Rust API returns typed results and typed failures. The service composition root is the production caller; browser journeys exercise the same path.

## Invariants
1. Empty or uncorrelated evidence is invalid.
2. A denial cannot create a durable record or invoke a downstream integration.
3. A successful record remains retrievable after process restart.
4. Upstream identity is pinned independently of the local adapter.

## Collaboration and tests
The context publishes only the minimum facts needed by its neighbours, keeping identity, authorization, policy, persistence and rendering separate. Unit tests cover its behavior and failure edge; integration and browser suites prove the caller path. The traceability matrix binds the code path to exact-source external evidence.
