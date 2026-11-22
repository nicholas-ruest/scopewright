# Architecture
The React console uses a checked v1 client. Axum applies correlation, authentication, authorization, timeout, error and tracing middleware before constructing a domain request. Governance composes trace, identity, policy, validation, allocation, quality and telemetry contexts. Seven adapters then execute Ruflo, MetaHarness, AgentDB, site-harness, Sublinear Time Solver, TokenEconomizer and OpenTelemetry. Only a complete result is hash chained and appended to mounted JSONL storage. Retrieval verifies the chain on every load and rebuilds a correlation after restart.

Rootless containers use no privileged device. The service is stateless; durable evidence crosses the service boundary through a configured volume, satisfying the no service-owned database rule.
