use scopewright_identity::verify_operator;
use scopewright_kernel::FactoryRunRequest;
#[test]
fn rejects_domain_boundary() {
    let r = FactoryRunRequest {
        service: "checkout".into(),
        trace_summary: "failover".into(),
        scope_pressure: 20,
        token_budget: 18_000,
        operator: "invalid".into(),
    };
    assert!(verify_operator(&r).is_err());
}
