use scopewright_kernel::{DomainError, FactoryRunRequest, StageResult};

pub fn validate_window(request: &FactoryRunRequest) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    if request.token_budget < 512 {
        return Err(DomainError::Denied(
            "evidence budget is below the executable minimum".into(),
        ));
    }
    let score = (request.token_budget as f64 / 200.0).clamp(20.0, 100.0);
    Ok(StageResult {
        stage: "validation".into(),
        score,
        message: "validated executable evidence budget".into(),
        evidence: vec![format!("validation:{}:{:.2}", request.stable_key(), score)],
    })
}
