use scopewright_kernel::FactoryRunRequest;
use scopewright_validation::validate_window;
fn request() -> FactoryRunRequest {
    FactoryRunRequest {
        service: "checkout".into(),
        trace_summary: "regional failover exercise".into(),
        scope_pressure: 25,
        token_budget: 30,
        operator: "sre@example.com".into(),
    }
}
#[test]
fn produces_domain_evidence() {
    let r = validate_window(&request()).unwrap();
    assert_eq!(r.stage, "validation");
    assert!(!r.evidence.is_empty());
}
