use scopewright_kernel::FactoryRunRequest;
use scopewright_observability::instrument;
#[test]
fn rejects_domain_boundary() {
    let r = FactoryRunRequest {
        service:
            "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
                .into(),
        trace_summary: "failover".into(),
        scope_pressure: 20,
        token_budget: 30,
        operator: "sre@example.com".into(),
    };
    assert!(instrument(&r).is_err());
}
