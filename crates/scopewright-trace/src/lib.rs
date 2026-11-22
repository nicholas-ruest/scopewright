use scopewright_kernel::{DomainError, FactoryRunRequest, StageResult};

pub mod topology;

pub fn compile_plan(request: &FactoryRunRequest) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    if request.trace_summary.len() < 6 {
        return Err(DomainError::Denied("trace_summary is too vague".into()));
    }
    let analysis = topology::analyze_request(request)?;
    let vocabulary = request.trace_summary.split_whitespace().count() as f64 * 4.0;
    let topology_exposure = analysis.maximum_weighted_impact * 0.45;
    let score = (vocabulary + topology_exposure).clamp(0.0, 100.0);
    Ok(StageResult {
        stage: "trace_summary".into(),
        score,
        message: format!(
            "compiled recovery trace_summary across {} affected components",
            analysis.affected_components
        ),
        evidence: vec![
            format!("trace_summary:{}:{:.2}", request.stable_key(), score),
            format!(
                "topology:depth={}:burden={}",
                analysis.longest_path, analysis.recovery_burden_minutes
            ),
        ],
    })
}
