use scopewright_kernel::DomainError;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub mod ledger;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EvidenceRecord {
    pub id: String,
    pub correlation_id: String,
    pub payload: String,
    pub previous_hash: String,
    pub hash: String,
}

fn digest(parts: &[&str]) -> String {
    let mut h = Sha256::new();
    for part in parts {
        h.update(part.as_bytes());
        h.update([0]);
    }
    format!("{:x}", h.finalize())
}

pub fn seal(
    id: &str,
    correlation_id: &str,
    payload: &str,
    previous_hash: &str,
) -> Result<EvidenceRecord, DomainError> {
    if id.trim().is_empty() || correlation_id.trim().is_empty() || payload.trim().is_empty() {
        return Err(DomainError::Integrity(
            "evidence fields must be non-empty".into(),
        ));
    }
    let hash = digest(&[id, correlation_id, payload, previous_hash]);
    Ok(EvidenceRecord {
        id: id.into(),
        correlation_id: correlation_id.into(),
        payload: payload.into(),
        previous_hash: previous_hash.into(),
        hash,
    })
}

pub fn verify(record: &EvidenceRecord) -> Result<(), DomainError> {
    let expected = digest(&[
        &record.id,
        &record.correlation_id,
        &record.payload,
        &record.previous_hash,
    ]);
    if expected != record.hash {
        return Err(DomainError::Integrity("hash mismatch".into()));
    }
    Ok(())
}
