use scopewright_api_schema::{CreateFactoryRun, API_VERSION};
#[test]
fn maps_typed_contract() {
    let d = CreateFactoryRun {
        service: "checkout".into(),
        trace_summary: "failover".into(),
        scope_pressure: 20,
        token_budget: 30,
    }
    .into_domain("sre@example.com".into());
    assert_eq!(d.service, "checkout");
    assert_eq!(API_VERSION, "v1");
}
