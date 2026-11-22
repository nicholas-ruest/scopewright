use scopewright_governance::run_domain;
use scopewright_kernel::FactoryRunRequest;
#[test]
fn denial_stops_composition() {
    let r = FactoryRunRequest {
        service: "checkout".into(),
        trace_summary: "failover".into(),
        scope_pressure: 90,
        token_budget: 18_000,
        operator: "sre@example.com".into(),
    };
    assert!(run_domain(&r).is_err());
}
