use scopewright_kernel::{DomainError, FactoryRunRequest, StageResult};

pub fn budget(request: &FactoryRunRequest) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    if request.token_budget > 720 {
        return Err(DomainError::Denied(
            "deadline consumes operational day".into(),
        ));
    }
    let score = ((1440 - request.token_budget) as f64 / 14.4).clamp(0.0, 100.0);
    Ok(StageResult {
        stage: "deadline".into(),
        score,
        message: "reserved execution deadline".into(),
        evidence: vec![format!("deadline:{}:{:.2}", request.stable_key(), score)],
    })
}
