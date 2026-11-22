use scopewright_kernel::{DomainError, FactoryRunRequest, StageResult};

pub mod plan;

pub fn schedule(request: &FactoryRunRequest) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    if request.token_budget < 5 {
        return Err(DomainError::Denied(
            "factory_run window is unschedulable".into(),
        ));
    }
    let plan = plan::plan_request(request)?;
    let score = if plan.slack_minutes >= 0 {
        (100.0 - plan.critical_path_minutes as f64).clamp(0.0, 100.0)
    } else {
        (50.0 + plan.slack_minutes as f64).clamp(0.0, 100.0)
    };
    Ok(StageResult {
        stage: "scheduler".into(),
        score,
        message: format!(
            "scheduled {} evidence tasks with {}-way parallelism",
            plan.tasks.len(),
            plan.maximum_parallelism
        ),
        evidence: vec![
            format!("scheduler:{}:{:.2}", request.stable_key(), score),
            format!(
                "evidence-plan:critical={}:objective={}:slack={}",
                plan.critical_path_minutes, plan.objective_minutes, plan.slack_minutes
            ),
        ],
    })
}
