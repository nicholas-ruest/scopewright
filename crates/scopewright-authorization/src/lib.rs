use scopewright_kernel::{DomainError, FactoryRunRequest, StageResult};

pub fn authorize_radius(request: &FactoryRunRequest) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    if request.scope_pressure > 80 {
        return Err(DomainError::Denied("blast radius exceeds policy".into()));
    }
    let score = (100.0 - request.scope_pressure as f64).clamp(0.0, 100.0);
    Ok(StageResult {
        stage: "authorization".into(),
        score,
        message: "authorized bounded blast radius".into(),
        evidence: vec![format!(
            "authorization:{}:{:.2}",
            request.stable_key(),
            score
        )],
    })
}
