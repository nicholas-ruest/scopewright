use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use scopewright_service::app::{router, AppState};
use tower::ServiceExt;

#[tokio::test]
async fn policy_denial_has_no_downstream_write() {
    let path =
        std::env::temp_dir().join(format!("scopewright-denial-{}.jsonl", std::process::id()));
    let state = AppState::open(&path).unwrap();
    let app = router(state.clone());
    let body = r#"{"service":"checkout","trace_summary":"unsafe global failover","scope_pressure":95,"token_budget":30}"#;
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
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    assert!(state.store.load().unwrap().is_empty());
    let _ = std::fs::remove_file(path);
}
