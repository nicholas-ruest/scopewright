use scopewright_kernel::{DomainError, FactoryRunRequest, StageResult};

pub fn run_domain(request: &FactoryRunRequest) -> Result<Vec<StageResult>, DomainError> {
    let stages = vec![
        scopewright_trace::compile_plan(request)?,
        scopewright_identity::verify_operator(request)?,
        scopewright_authorization::authorize_radius(request)?,
        scopewright_validation::validate_window(request)?,
        scopewright_correlation::correlate(request)?,
        scopewright_deadline::budget(request)?,
        scopewright_observability::instrument(request)?,
        scopewright_quality::score_risk(request)?,
        scopewright_telemetry::ingest(request)?,
        scopewright_allocation::schedule(request)?,
    ];
    if stages.iter().any(|stage| stage.evidence.is_empty()) {
        return Err(DomainError::Integrity(
            "domain stage omitted evidence".into(),
        ));
    }
    Ok(stages)
}

pub fn aggregate_score(stages: &[StageResult]) -> Result<f64, DomainError> {
    if stages.is_empty() {
        return Err(DomainError::Missing("stages"));
    }
    Ok(stages.iter().map(|s| s.score).sum::<f64>() / stages.len() as f64)
}
