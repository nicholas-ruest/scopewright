use scopewright_correlation::correlate;
use scopewright_kernel::FactoryRunRequest;
#[test]
fn rejects_domain_boundary() {
    let r = FactoryRunRequest {
        service: "".into(),
        trace_summary: "x".into(),
        scope_pressure: 20,
        token_budget: 30,
        operator: "x".into(),
    };
    assert!(correlate(&r).is_err());
}
