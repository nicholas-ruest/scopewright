use scopewright_authorization::authorize_radius;
use scopewright_kernel::FactoryRunRequest;
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
    let r = authorize_radius(&request()).unwrap();
    assert_eq!(r.stage, "authorization");
    assert!(!r.evidence.is_empty());
}
