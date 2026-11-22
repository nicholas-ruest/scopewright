use scopewright_kernel::{DomainError, FactoryRunRequest, StageResult};

pub mod model;

pub fn score_risk(request: &FactoryRunRequest) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    if request.scope_pressure == 0 {
        return Err(DomainError::Denied(
            "risk model needs nonzero exercise".into(),
        ));
    }
    let assessment = model::assess_request(request)?;
    let sensitivity = model::sensitivity(
        &model::factors_for_request(request)?,
        &model::standard_controls(request),
    )?;
    let strongest = sensitivity
        .first()
        .map(|point| point.control_name.as_str())
        .unwrap_or("none");
    Ok(StageResult {
        stage: "risk".into(),
        score: assessment.residual_score,
        message: format!(
            "calculated {:?} residual risk; strongest control {}",
            assessment.band, strongest
        ),
        evidence: vec![
            format!(
                "risk:{}:inherent={:.2}:residual={:.2}",
                request.stable_key(),
                assessment.inherent_score,
                assessment.residual_score
            ),
            format!(
                "risk:dominant={:?}:uncertainty={:.2}",
                assessment.dominant_factor, assessment.uncertainty
            ),
        ],
    })
}
