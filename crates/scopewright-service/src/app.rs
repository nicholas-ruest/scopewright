use crate::middleware::{authn, authz_policy, correlation, timeouts, CorrelationId, Principal};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use scopewright_api_schema::{CreateFactoryRun, ErrorResponse, FactoryRunResponse};
use scopewright_audit::AuditChain;
use scopewright_kernel::{DomainError, FactoryRunRequest, StageResult};
use scopewright_persistence::EvidenceStore;
use std::sync::Arc;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub store: Arc<EvidenceStore>,
}
impl AppState {
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, DomainError> {
        Ok(Self {
            store: Arc::new(EvidenceStore::open(path)?),
        })
    }
}
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(health))
        .route(
            "/api/v1/factory_runs",
            post(create_factory_run).get(list_factory_runs),
        )
        .route("/api/v1/factory_runs/{correlation}", get(get_factory_run))
        .route("/api/v1/audit", get(audit))
        .layer(middleware::from_fn(timeouts))
        .layer(middleware::from_fn(authz_policy))
        .layer(middleware::from_fn(authn))
        .layer(middleware::from_fn(correlation))
        .fallback_service(
            ServeDir::new("frontend/dist")
                .not_found_service(ServeFile::new("frontend/dist/index.html")),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
async fn health() -> impl IntoResponse {
    Json(serde_json::json!({"status":"ready","api":"v1","service":"scopewright"}))
}
fn error(e: DomainError, c: &str) -> (StatusCode, Json<ErrorResponse>) {
    let s = match e {
        DomainError::Denied(_) => StatusCode::FORBIDDEN,
        DomainError::Missing(_) | DomainError::Range(_) => StatusCode::UNPROCESSABLE_ENTITY,
        DomainError::Upstream(_) => StatusCode::BAD_GATEWAY,
        DomainError::Integrity(_) | DomainError::Storage(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };
    (
        s,
        Json(ErrorResponse {
            code: s.as_u16().to_string(),
            message: e.to_string(),
            correlation_id: c.into(),
        }),
    )
}
fn cfg(name: &str, marker: &str) -> Result<scopewright_upstream_otel::AdapterConfig, DomainError> {
    let script = std::env::var(name)
        .map_err(|_| DomainError::Upstream(format!("missing runtime binding {name}")))?;
    Ok(scopewright_upstream_otel::AdapterConfig {
        command: "/bin/sh".into(),
        args: vec!["-c".into(), script],
        required_marker: marker.into(),
    })
}
fn run_upstreams(r: &FactoryRunRequest) -> Result<Vec<StageResult>, DomainError> {
    let raw = cfg("SCOPEWRIGHT_RUFLO_CMD", "route_recommendation")?;
    let a = scopewright_upstream_ruflo::AdapterConfig {
        command: raw.command,
        args: raw.args,
        required_marker: raw.required_marker,
    };
    let raw = cfg("SCOPEWRIGHT_METAHARNESS_CMD", "harness_fit")?;
    let b = scopewright_upstream_metaharness::AdapterConfig {
        command: raw.command,
        args: raw.args,
        required_marker: raw.required_marker,
    };
    let raw = cfg("SCOPEWRIGHT_AGENTDB_CMD", "episode_match")?;
    let c = scopewright_upstream_agentdb::AdapterConfig {
        command: raw.command,
        args: raw.args,
        required_marker: raw.required_marker,
    };
    let raw = cfg("SCOPEWRIGHT_SITE_HARNESS_CMD", "adapter_inventory")?;
    let d = scopewright_upstream_site_harness::AdapterConfig {
        command: raw.command,
        args: raw.args,
        required_marker: raw.required_marker,
    };
    let raw = cfg("SCOPEWRIGHT_SUBLINEAR_CMD", "evidence_allocation")?;
    let e = scopewright_upstream_sublinear::AdapterConfig {
        command: raw.command,
        args: raw.args,
        required_marker: raw.required_marker,
    };
    let raw = cfg("SCOPEWRIGHT_TOKENECONOMIZER_CMD", "loopguard_intervention")?;
    let f = scopewright_upstream_tokeneconomizer::AdapterConfig {
        command: raw.command,
        args: raw.args,
        required_marker: raw.required_marker,
    };
    let g = cfg("SCOPEWRIGHT_OTEL_CMD", "otel_exported")?;
    Ok(vec![
        scopewright_upstream_ruflo::route_factory_trace(&a, r)?,
        scopewright_upstream_metaharness::score_checkout(&b, r)?,
        scopewright_upstream_agentdb::retrieve_episode(&c, r)?,
        scopewright_upstream_site_harness::inventory_adapters(&d, r)?,
        scopewright_upstream_sublinear::allocate_evidence(&e, r)?,
        scopewright_upstream_tokeneconomizer::recommend_intervention(&f, r)?,
        scopewright_upstream_otel::export_trace(&g, r)?,
    ])
}
async fn create_factory_run(
    State(state): State<AppState>,
    Extension(principal): Extension<Principal>,
    Extension(correlation): Extension<CorrelationId>,
    Json(input): Json<CreateFactoryRun>,
) -> Result<(StatusCode, Json<FactoryRunResponse>), (StatusCode, Json<ErrorResponse>)> {
    let request = input.into_domain(principal.0);
    request
        .validate_identity()
        .map_err(|e| error(e, &correlation.0))?;
    if request.protected_prune_requested() || request.scope_pressure > 80 {
        return Err(error(DomainError::Denied("protected full-product capability would be removed; no downstream integration executed".into()),&correlation.0));
    }
    let mut stages =
        scopewright_governance::run_domain(&request).map_err(|e| error(e, &correlation.0))?;
    let cloned = request.clone();
    let upstream = tokio::task::spawn_blocking(move || run_upstreams(&cloned))
        .await
        .map_err(|e| {
            error(
                DomainError::Upstream(format!("worker failed: {e}")),
                &correlation.0,
            )
        })?
        .map_err(|e| error(e, &correlation.0))?;
    stages.extend(upstream);
    let score =
        scopewright_governance::aggregate_score(&stages).map_err(|e| error(e, &correlation.0))?;
    let decision = if score >= 85.0 {
        "continue"
    } else {
        "intervene"
    };
    let id = Uuid::new_v4().to_string();
    let payload=serde_json::json!({"id":id,"service":request.service,"trace_summary":request.trace_summary,"risk_score":score,"decision":decision,"stages":stages.iter().map(|s|s.stage.clone()).collect::<Vec<_>>()}).to_string();
    let persisted = state.store.load().map_err(|e| error(e, &correlation.0))?;
    scopewright_evidence::ledger::verify_chain(&persisted).map_err(|e| error(e, &correlation.0))?;
    let mut chain = AuditChain::default();
    for item in &persisted {
        let rebuilt = chain
            .append(&item.id, &item.correlation_id, &item.payload)
            .map_err(|e| error(e, &correlation.0))?;
        if rebuilt.hash != item.hash {
            return Err(error(
                DomainError::Integrity("persisted audit hash mismatch".into()),
                &correlation.0,
            ));
        }
    }
    let record = chain
        .append(&id, &correlation.0, &payload)
        .map_err(|e| error(e, &correlation.0))?
        .clone();
    state
        .store
        .append(&record)
        .map_err(|e| error(e, &correlation.0))?;
    Ok((
        StatusCode::CREATED,
        Json(FactoryRunResponse {
            id,
            correlation_id: correlation.0,
            status: "saved".into(),
            decision: decision.into(),
            risk_score: score,
            stages: stages.into_iter().map(|s| s.stage).collect(),
            evidence_hash: record.hash,
        }),
    ))
}
async fn list_factory_runs(
    State(s): State<AppState>,
) -> Result<Json<Vec<scopewright_evidence::EvidenceRecord>>, (StatusCode, Json<ErrorResponse>)> {
    s.store.load().map(Json).map_err(|e| error(e, "list"))
}
async fn get_factory_run(
    State(s): State<AppState>,
    Path(c): Path<String>,
) -> Result<Json<scopewright_retrieval::ReplaySummary>, (StatusCode, Json<ErrorResponse>)> {
    scopewright_retrieval::replay(&s.store, &c)
        .map(Json)
        .map_err(|e| error(e, &c))
}
async fn audit(
    State(s): State<AppState>,
) -> Result<Json<Vec<scopewright_evidence::EvidenceRecord>>, (StatusCode, Json<ErrorResponse>)> {
    let rows = s.store.load().map_err(|e| error(e, "audit"))?;
    scopewright_evidence::ledger::verify_chain(&rows).map_err(|e| error(e, "audit"))?;
    Ok(Json(rows))
}
