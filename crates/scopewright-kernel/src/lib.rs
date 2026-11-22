use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const PROTECTED_CAPABILITIES: [&str; 5] = [
    "frontend",
    "api",
    "middleware",
    "rust-backend",
    "durable-audit",
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FactoryRunRequest {
    pub service: String,
    pub trace_summary: String,
    pub scope_pressure: u8,
    pub token_budget: u32,
    pub operator: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StageResult {
    pub stage: String,
    pub score: f64,
    pub message: String,
    pub evidence: Vec<String>,
}

#[derive(Debug, Error, Clone, PartialEq)]
pub enum DomainError {
    #[error("missing field: {0}")]
    Missing(&'static str),
    #[error("value out of range: {0}")]
    Range(&'static str),
    #[error("policy denied: {0}")]
    Denied(String),
    #[error("upstream unavailable: {0}")]
    Upstream(String),
    #[error("evidence integrity failure: {0}")]
    Integrity(String),
    #[error("storage failure: {0}")]
    Storage(String),
}

impl FactoryRunRequest {
    pub fn validate_identity(&self) -> Result<(), DomainError> {
        if self.service.trim().is_empty() {
            return Err(DomainError::Missing("service"));
        }
        if self.trace_summary.trim().is_empty() {
            return Err(DomainError::Missing("trace_summary"));
        }
        if self.operator.trim().is_empty() {
            return Err(DomainError::Missing("operator"));
        }
        if self.scope_pressure > 100 {
            return Err(DomainError::Range("scope_pressure"));
        }
        if self.token_budget == 0 || self.token_budget > 200_000 {
            return Err(DomainError::Range("token_budget"));
        }
        Ok(())
    }
    pub fn protected_prune_requested(&self) -> bool {
        let normalized = self.trace_summary.to_ascii_lowercase();
        PROTECTED_CAPABILITIES.iter().any(|c| {
            normalized.contains(&format!("remove {c}")) || normalized.contains(&format!("skip {c}"))
        })
    }
    pub fn stable_key(&self) -> String {
        format!(
            "{}:{}:{}",
            self.service.trim().to_lowercase(),
            self.trace_summary.trim().to_lowercase(),
            self.operator.trim().to_lowercase()
        )
    }
}

pub trait Stage {
    fn name(&self) -> &'static str;
    fn run(&self, request: &FactoryRunRequest) -> Result<StageResult, DomainError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    fn req(s: &str) -> FactoryRunRequest {
        FactoryRunRequest {
            service: "factory".into(),
            trace_summary: s.into(),
            scope_pressure: 41,
            token_budget: 12000,
            operator: "nick".into(),
        }
    }
    #[test]
    fn protects_product_shape() {
        assert!(req("please remove frontend before shipping").protected_prune_requested());
    }
    #[test]
    fn accepts_complete_trace() {
        assert!(
            req("complete browser api middleware rust persistence trace")
                .validate_identity()
                .is_ok()
        );
    }
    #[test]
    fn rejects_tiny_budget() {
        let mut r = req("complete trace with evidence");
        r.token_budget = 0;
        assert!(matches!(r.validate_identity(), Err(DomainError::Range(_))));
    }
}
