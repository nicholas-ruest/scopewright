use scopewright_kernel::FactoryRunRequest;
use scopewright_quality::score_risk;
#[test]
fn rejects_domain_boundary() {
    let r = FactoryRunRequest {
        service: "checkout".into(),
        trace_summary: "failover".into(),
        scope_pressure: 0,
        token_budget: 18_000,
        operator: "sre@example.com".into(),
    };
    assert!(score_risk(&r).is_err());
}
