use scopewright_kernel::FactoryRunRequest;
use scopewright_trace::compile_plan;
fn request() -> FactoryRunRequest {
    FactoryRunRequest {
        service: "checkout".into(),
        trace_summary: "regional failover exercise".into(),
        scope_pressure: 25,
        token_budget: 18_000,
        operator: "sre@example.com".into(),
    }
}
#[test]
fn produces_domain_evidence() {
    let r = compile_plan(&request()).unwrap();
    assert_eq!(r.stage, "trace_summary");
    assert!(!r.evidence.is_empty());
}
