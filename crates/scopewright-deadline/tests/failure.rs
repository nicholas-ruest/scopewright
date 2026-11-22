use scopewright_deadline::budget;
use scopewright_kernel::FactoryRunRequest;
#[test]
fn rejects_domain_boundary() {
    let r = FactoryRunRequest {
        service: "checkout".into(),
        trace_summary: "failover".into(),
        scope_pressure: 99,
        token_budget: 18_000,
        operator: "sre@example.com".into(),
    };
    assert!(budget(&r).is_err());
}
