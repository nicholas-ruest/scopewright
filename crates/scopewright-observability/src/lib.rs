use scopewright_kernel::{DomainError, FactoryRunRequest, StageResult};

pub fn instrument(request: &FactoryRunRequest) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    if request.service.len() > 80 {
        return Err(DomainError::Denied("service label is unbounded".into()));
    }
    let score = ((request.service.len() + request.trace_summary.len()) as f64).clamp(0.0, 100.0);
    Ok(StageResult {
        stage: "observability".into(),
        score,
        message: "opened correlated observation span".into(),
        evidence: vec![format!(
            "observability:{}:{:.2}",
            request.stable_key(),
            score
        )],
    })
}
