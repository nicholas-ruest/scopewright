use scopewright_kernel::FactoryRunRequest;
use scopewright_trace::compile_plan;
#[test]
fn rejects_domain_boundary() {
    let r = FactoryRunRequest {
        service: "checkout".into(),
        trace_summary: "x".into(),
        scope_pressure: 20,
        token_budget: 18_000,
        operator: "sre@example.com".into(),
    };
    assert!(compile_plan(&r).is_err());
}
