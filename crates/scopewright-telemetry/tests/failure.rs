use scopewright_kernel::FactoryRunRequest;
use scopewright_telemetry::ingest;
#[test]
fn rejects_domain_boundary() {
    let r = FactoryRunRequest {
        service: "checkout".into(),
        trace_summary: "NaN".into(),
        scope_pressure: 20,
        token_budget: 18_000,
        operator: "sre@example.com".into(),
    };
    assert!(ingest(&r).is_err());
}
