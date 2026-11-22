use scopewright_kernel::{DomainError, FactoryRunRequest, StageResult};

pub fn correlate(request: &FactoryRunRequest) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    if request.stable_key().len() < 8 {
        return Err(DomainError::Denied("correlation key is too short".into()));
    }
    let score = (request.stable_key().bytes().map(|b| b as u64).sum::<u64>() as f64 % 100.0)
        .clamp(0.0, 100.0);
    Ok(StageResult {
        stage: "correlation".into(),
        score,
        message: "derived deterministic correlation key".into(),
        evidence: vec![format!("correlation:{}:{:.2}", request.stable_key(), score)],
    })
}
