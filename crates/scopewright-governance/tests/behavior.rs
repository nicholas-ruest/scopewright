use scopewright_governance::{aggregate_score, run_domain};
use scopewright_kernel::FactoryRunRequest;
#[test]
fn composes_domain_stages() {
    let r = FactoryRunRequest {
        service: "checkout".into(),
        trace_summary: "regional failover".into(),
        scope_pressure: 25,
        token_budget: 18_000,
        operator: "sre@example.com".into(),
    };
    let s = run_domain(&r).unwrap();
    assert_eq!(s.len(), 10);
    assert!(aggregate_score(&s).unwrap() > 0.0);
}
