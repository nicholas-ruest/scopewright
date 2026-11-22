use scopewright_kernel::{DomainError, FactoryRunRequest, StageResult};

pub mod plan;

pub fn schedule(request: &FactoryRunRequest) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    if request.token_budget < 5 {
        return Err(DomainError::Denied(
            "factory_run window is unschedulable".into(),
        ));
    }
    let recovery = plan::plan_request(request)?;
    let score = if recovery.slack_minutes >= 0 {
        (100.0 - recovery.critical_path_minutes as f64).clamp(0.0, 100.0)
    } else {
        (50.0 + recovery.slack_minutes as f64).clamp(0.0, 100.0)
    };
    Ok(StageResult {
        stage: "scheduler".into(),
        score,
        message: format!(
            "scheduled {} recovery tasks with {}-way parallelism",
            recovery.tasks.len(),
            recovery.maximum_parallelism
        ),
        evidence: vec![
            format!("scheduler:{}:{:.2}", request.stable_key(), score),
            format!(
                "recovery:critical={}:objective={}:slack={}",
                recovery.critical_path_minutes, recovery.objective_minutes, recovery.slack_minutes
            ),
        ],
    })
}
