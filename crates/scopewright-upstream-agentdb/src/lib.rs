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
            required_marker: "episode_match".into(),
        }
    }
}
pub fn retrieve_episode(
    config: &AdapterConfig,
    request: &FactoryRunRequest,
) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    let output = Command::new(&config.command)
        .args(&config.args)
        .env("SCOPEWRIGHT_SERVICE", &request.service)
        .env("SCOPEWRIGHT_TRACE", &request.trace_summary)
        .output()
        .map_err(|e| DomainError::Upstream(format!("agentdb: {e}")))?;
    if !output.status.success() {
        return Err(DomainError::Upstream(format!(
            "agentdb exited {}",
            output.status
        )));
    }
    let text = String::from_utf8_lossy(&output.stdout);
    if !text.contains(&config.required_marker) {
        return Err(DomainError::Upstream(format!(
            "agentdb missing marker {}",
            config.required_marker
        )));
    }
    Ok(StageResult {
        stage: "upstream-agentdb".into(),
        score: 100.0,
        message: "AgentDB retrieves analogous failure and recovery episodes".into(),
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
        let c = AdapterConfig::pinned("/bin/sh", vec!["-c".into(), "echo episode_match".into()]);
        assert_eq!(
            retrieve_episode(&c, &r()).unwrap().stage,
            "upstream-agentdb"
        );
    }
    #[test]
    fn failure_is_typed() {
        let c = AdapterConfig::pinned("/bin/sh", vec!["-c".into(), "exit 17".into()]);
        assert!(matches!(
            retrieve_episode(&c, &r()),
            Err(DomainError::Upstream(_))
        ));
    }
}
