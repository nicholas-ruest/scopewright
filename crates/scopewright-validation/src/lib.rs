use scopewright_kernel::{DomainError, FactoryRunRequest, StageResult};

pub fn validate_window(request: &FactoryRunRequest) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    if request.token_budget > 120 {
        return Err(DomainError::Denied(
            "recovery window exceeds two hours".into(),
        ));
    }
    let score = (120.0 - request.token_budget.min(120) as f64).clamp(0.0, 100.0);
    Ok(StageResult {
        stage: "validation".into(),
        score,
        message: "validated recovery objective".into(),
        evidence: vec![format!("validation:{}:{:.2}", request.stable_key(), score)],
    })
}
