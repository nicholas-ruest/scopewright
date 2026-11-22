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
            required_marker: "adapter_inventory".into(),
        }
    }
}
pub fn inventory_adapters(
    config: &AdapterConfig,
    request: &FactoryRunRequest,
) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    let output = Command::new(&config.command)
        .args(&config.args)
        .env("SCOPEWRIGHT_SERVICE", &request.service)
        .env("SCOPEWRIGHT_TRACE", &request.trace_summary)
        .output()
        .map_err(|e| DomainError::Upstream(format!("site-harness: {e}")))?;
    if !output.status.success() {
        return Err(DomainError::Upstream(format!(
            "site-harness exited {}",
            output.status
        )));
    }
    let text = String::from_utf8_lossy(&output.stdout);
    if !text.contains(&config.required_marker) {
        return Err(DomainError::Upstream(format!(
            "site-harness missing marker {}",
            config.required_marker
        )));
    }
    Ok(StageResult {
        stage: "upstream-site-harness".into(),
        score: 100.0,
        message: "site-harness inventories executable tool adapters".into(),
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
            vec!["-c".into(), "echo adapter_inventory".into()],
        );
        assert_eq!(
            inventory_adapters(&c, &r()).unwrap().stage,
            "upstream-site-harness"
        );
    }
    #[test]
    fn failure_is_typed() {
        let c = AdapterConfig::pinned("/bin/sh", vec!["-c".into(), "exit 17".into()]);
        assert!(matches!(
            inventory_adapters(&c, &r()),
            Err(DomainError::Upstream(_))
        ));
    }
}
