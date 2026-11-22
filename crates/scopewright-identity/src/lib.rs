use scopewright_kernel::{DomainError, FactoryRunRequest, StageResult};

pub fn verify_operator(request: &FactoryRunRequest) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    if !request.operator.contains("@") {
        return Err(DomainError::Denied(
            "operator identity is not an email".into(),
        ));
    }
    let score = (request.operator.split("@").count() as f64 * 25.0).clamp(0.0, 100.0);
    Ok(StageResult {
        stage: "identity".into(),
        score,
        message: "verified operator identity".into(),
        evidence: vec![format!("identity:{}:{:.2}", request.stable_key(), score)],
    })
}
