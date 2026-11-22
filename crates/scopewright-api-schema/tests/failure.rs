use scopewright_api_schema::CreateFactoryRun;
#[test]
fn domain_validation_rejects_empty_service() {
    let d = CreateFactoryRun {
        service: "".into(),
        trace_summary: "failover".into(),
        scope_pressure: 20,
        token_budget: 30,
    }
    .into_domain("sre@example.com".into());
    assert!(d.validate_identity().is_err());
}
