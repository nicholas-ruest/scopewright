use scopewright_kernel::{DomainError, FactoryRunRequest, StageResult};
use std::process::Command;
#[derive(Debug, Clone)]
pub struct AdapterConfig {
    pub command: String,
    pub args: Vec<String>,
    pub required_marker: String,
}
impl AdapterConfig {
    pub fn pinned(command: impl Into<String>, args: Vec<String>) -> Self {
        Self {
            command: command.into(),
            args,
            required_marker: "route_recommendation".into(),
        }
    }
}
pub fn route_factory_trace(
    config: &AdapterConfig,
    request: &FactoryRunRequest,
) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    let output = Command::new(&config.command)
        .args(&config.args)
        .env("SCOPEWRIGHT_SERVICE", &request.service)
        .env("SCOPEWRIGHT_TRACE", &request.trace_summary)
        .output()
        .map_err(|e| DomainError::Upstream(format!("ruflo: {e}")))?;
    if !output.status.success() {
        return Err(DomainError::Upstream(format!(
            "ruflo exited {}",
            output.status
        )));
    }
    let text = String::from_utf8_lossy(&output.stdout);
    if !text.contains(&config.required_marker) {
        return Err(DomainError::Upstream(format!(
            "ruflo missing marker {}",
            config.required_marker
        )));
    }
    Ok(StageResult {
        stage: "upstream-ruflo".into(),
        score: 100.0,
        message: "Ruflo routes specialist analysis across the factory trace".into(),
        evidence: vec![text.trim().into()],
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    fn r() -> FactoryRunRequest {
        FactoryRunRequest {
            service: "factory".into(),
            trace_summary: "complete full stack factory trace".into(),
            scope_pressure: 30,
            token_budget: 9000,
            operator: "test".into(),
        }
    }
    #[test]
    fn success_is_evidenced() {
        let c = AdapterConfig::pinned(
            "/bin/sh",
            vec!["-c".into(), "echo route_recommendation".into()],
        );
        assert_eq!(
            route_factory_trace(&c, &r()).unwrap().stage,
            "upstream-ruflo"
        );
    }
    #[test]
    fn failure_is_typed() {
        let c = AdapterConfig::pinned("/bin/sh", vec!["-c".into(), "exit 17".into()]);
        assert!(matches!(
            route_factory_trace(&c, &r()),
            Err(DomainError::Upstream(_))
        ));
    }
}
