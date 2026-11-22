use scopewright_kernel::{DomainError, FactoryRunRequest, StageResult};

pub fn ingest(request: &FactoryRunRequest) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    if request.trace_summary.contains("NaN") {
        return Err(DomainError::Denied(
            "telemetry contains invalid sample".into(),
        ));
    }
    let score = ((request.scope_pressure as f64).sqrt() * 10.0).clamp(0.0, 100.0);
    Ok(StageResult {
        stage: "telemetry".into(),
        score,
        message: "ingested normalized factory_run telemetry".into(),
        evidence: vec![format!("telemetry:{}:{:.2}", request.stable_key(), score)],
    })
}
