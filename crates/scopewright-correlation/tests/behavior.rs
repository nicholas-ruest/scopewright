use scopewright_correlation::correlate;
use scopewright_kernel::FactoryRunRequest;
fn request() -> FactoryRunRequest {
    FactoryRunRequest {
        service: "checkout".into(),
        trace_summary: "regional failover exercise".into(),
        scope_pressure: 25,
        token_budget: 18_000,
        operator: "sre@example.com".into(),
    }
}
#[test]
fn produces_domain_evidence() {
    let r = correlate(&request()).unwrap();
    assert_eq!(r.stage, "correlation");
    assert!(!r.evidence.is_empty());
}
