use scopewright_allocation::schedule;
use scopewright_kernel::FactoryRunRequest;
#[test]
fn rejects_domain_boundary() {
    let r = FactoryRunRequest {
        service: "checkout".into(),
        trace_summary: "failover".into(),
        scope_pressure: 20,
        token_budget: 1,
        operator: "sre@example.com".into(),
    };
    assert!(schedule(&r).is_err());
}
