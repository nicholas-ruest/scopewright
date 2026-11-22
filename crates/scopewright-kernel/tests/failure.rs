use scopewright_kernel::{DomainError, FactoryRunRequest};
#[test]
fn rejects_impossible_recovery_window() {
    let r = FactoryRunRequest {
        service: "checkout".into(),
        trace_summary: "failover".into(),
        scope_pressure: 25,
        token_budget: 0,
        operator: "sre".into(),
    };
    assert_eq!(
        r.validate_identity(),
        Err(DomainError::Range("token_budget"))
    );
}
