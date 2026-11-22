use scopewright_kernel::FactoryRunRequest;
use scopewright_validation::validate_window;
#[test]
fn rejects_domain_boundary() {
    let r = FactoryRunRequest {
        service: "checkout".into(),
        trace_summary: "failover".into(),
        scope_pressure: 20,
        token_budget: 128,
        operator: "sre@example.com".into(),
    };
    assert!(validate_window(&r).is_err());
}
