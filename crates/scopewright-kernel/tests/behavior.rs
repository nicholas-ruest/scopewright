use scopewright_kernel::FactoryRunRequest;
#[test]
fn accepts_well_formed_factory_run() {
    let r = FactoryRunRequest {
        service: "checkout".into(),
        trace_summary: "regional failover".into(),
        scope_pressure: 25,
        token_budget: 30,
        operator: "sre@example.com".into(),
    };
    assert!(r.validate_identity().is_ok());
    assert_eq!(r.stable_key(), "checkout:regional failover:sre@example.com");
}
