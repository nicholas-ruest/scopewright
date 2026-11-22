pub const API_VERSION: &str = "v1";
use scopewright_kernel::FactoryRunRequest;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateFactoryRun {
    pub service: String,
    pub trace_summary: String,
    pub scope_pressure: u8,
    pub token_budget: u32,
}
impl CreateFactoryRun {
    pub fn into_domain(self, operator: String) -> FactoryRunRequest {
        FactoryRunRequest {
            service: self.service,
            trace_summary: self.trace_summary,
            scope_pressure: self.scope_pressure,
            token_budget: self.token_budget,
            operator,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FactoryRunResponse {
    pub id: String,
    pub correlation_id: String,
    pub status: String,
    pub decision: String,
    pub risk_score: f64,
    pub stages: Vec<String>,
    pub evidence_hash: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub correlation_id: String,
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn operator_is_middleware_owned() {
        let d = CreateFactoryRun {
            service: "x".into(),
            trace_summary: "a sufficiently complete trace".into(),
            scope_pressure: 3,
            token_budget: 1000,
        }
        .into_domain("principal".into());
        assert_eq!(d.operator, "principal");
    }
}
