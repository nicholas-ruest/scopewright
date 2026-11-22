use scopewright_authorization::authorize_radius;
use scopewright_kernel::FactoryRunRequest;
#[test]
fn rejects_domain_boundary() {
    let r = FactoryRunRequest {
        service: "checkout".into(),
        trace_summary: "failover".into(),
        scope_pressure: 90,
        token_budget: 30,
        operator: "sre@example.com".into(),
    };
    assert!(authorize_radius(&r).is_err());
}
