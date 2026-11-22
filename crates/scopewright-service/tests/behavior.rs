use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use scopewright_service::app::{router, AppState};
use tower::ServiceExt;

fn configure() {
    for (name, marker) in [
        ("SCOPEWRIGHT_RUFLO_CMD", "route_recommendation"),
        ("SCOPEWRIGHT_METAHARNESS_CMD", "harness_fit"),
        ("SCOPEWRIGHT_AGENTDB_CMD", "episode_match"),
        ("SCOPEWRIGHT_SITE_HARNESS_CMD", "adapter_inventory"),
        ("SCOPEWRIGHT_SUBLINEAR_CMD", "evidence_allocation"),
        ("SCOPEWRIGHT_TOKENECONOMIZER_CMD", "loopguard_intervention"),
        ("SCOPEWRIGHT_OTEL_CMD", "otel_exported"),
    ] {
        std::env::set_var(name, format!("printf {marker}"));
    }
}

#[tokio::test]
async fn browser_to_durable_result_slice() {
    configure();
    let path =
        std::env::temp_dir().join(format!("scopewright-service-{}.jsonl", std::process::id()));
    let app = router(AppState::open(&path).unwrap());
    let body = r#"{"service":"checkout","trace_summary":"regional failover","scope_pressure":25,"token_budget":18000}"#;
    let response = app
        .oneshot(
            Request::post("/api/v1/factory_runs")
                .header("authorization", "Bearer scopewright-operator")
                .header("x-role", "factory_run-operator")
                .header("x-operator", "sre@example.com")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(
        AppState::open(&path).unwrap().store.load().unwrap().len(),
        1
    );
    let _ = std::fs::remove_file(path);
}
