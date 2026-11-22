use scopewright_evidence::{verify, EvidenceRecord};
use scopewright_kernel::DomainError;
use scopewright_persistence::EvidenceStore;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReplaySummary {
    pub correlation_id: String,
    pub events: usize,
    pub final_hash: String,
    pub payloads: Vec<String>,
}

pub fn replay(store: &EvidenceStore, correlation: &str) -> Result<ReplaySummary, DomainError> {
    if correlation.trim().is_empty() {
        return Err(DomainError::Missing("correlation_id"));
    }
    let records: Vec<EvidenceRecord> = store
        .load()?
        .into_iter()
        .filter(|r| r.correlation_id == correlation)
        .collect();
    if records.is_empty() {
        return Err(DomainError::Storage("factory_run not found".into()));
    }
    let mut previous = "genesis";
    for record in &records {
        verify(record)?;
        if record.previous_hash != previous {
            return Err(DomainError::Integrity("replay chain discontinuity".into()));
        }
        previous = &record.hash;
    }
    Ok(ReplaySummary {
        correlation_id: correlation.into(),
        events: records.len(),
        final_hash: previous.into(),
        payloads: records.into_iter().map(|r| r.payload).collect(),
    })
}
