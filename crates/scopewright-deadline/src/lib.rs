use scopewright_kernel::{DomainError, FactoryRunRequest, StageResult};

pub fn budget(request: &FactoryRunRequest) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    if request.scope_pressure > 95 {
        return Err(DomainError::Denied(
            "execution pressure cannot meet the 120 second deadline".into(),
        ));
    }
    let score = (100.0 - request.scope_pressure as f64).clamp(0.0, 100.0);
    Ok(StageResult {
        stage: "deadline".into(),
        score,
        message: "reserved 120 second execution deadline".into(),
        evidence: vec![format!(
            "deadline:{}:120s:{:.2}",
            request.stable_key(),
            score
        )],
    })
}
