use crate::{verify, EvidenceRecord};
use scopewright_kernel::DomainError;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChainFindingKind {
    DuplicateId,
    DuplicateHash,
    BrokenPredecessor,
    InvalidDigest,
    EmptyCorrelation,
    PayloadShape,
    CorrelationDiscontinuity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChainFinding {
    pub index: usize,
    pub record_id: String,
    pub kind: ChainFindingKind,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CorrelationSummary {
    pub correlation_id: String,
    pub record_count: usize,
    pub first_index: usize,
    pub last_index: usize,
    pub payload_bytes: usize,
    pub terminal_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChainReport {
    pub valid: bool,
    pub record_count: usize,
    pub unique_correlations: usize,
    pub total_payload_bytes: usize,
    pub genesis_hash: String,
    pub terminal_hash: String,
    pub findings: Vec<ChainFinding>,
    pub correlations: Vec<CorrelationSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReplayEnvelope {
    pub correlation_id: String,
    pub records: Vec<EvidenceRecord>,
    pub complete_chain_prefix: bool,
    pub terminal_hash: String,
}

fn payload_is_json(payload: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(payload).is_ok()
}

fn finding(
    index: usize,
    record: &EvidenceRecord,
    kind: ChainFindingKind,
    detail: impl Into<String>,
) -> ChainFinding {
    ChainFinding {
        index,
        record_id: record.id.clone(),
        kind,
        detail: detail.into(),
    }
}

pub fn inspect_chain(records: &[EvidenceRecord]) -> ChainReport {
    let mut findings = Vec::new();
    let mut ids = BTreeSet::new();
    let mut hashes = BTreeSet::new();
    let mut summaries: BTreeMap<String, CorrelationSummary> = BTreeMap::new();
    let mut expected_previous = "genesis".to_string();
    let mut total_payload_bytes = 0_usize;
    let mut genesis_hash = String::new();

    for (index, record) in records.iter().enumerate() {
        if index == 0 {
            genesis_hash = record.hash.clone();
        }
        if !ids.insert(record.id.as_str()) {
            findings.push(finding(
                index,
                record,
                ChainFindingKind::DuplicateId,
                format!("record id {} appeared more than once", record.id),
            ));
        }
        if !hashes.insert(record.hash.as_str()) {
            findings.push(finding(
                index,
                record,
                ChainFindingKind::DuplicateHash,
                format!("record hash {} appeared more than once", record.hash),
            ));
        }
        if record.previous_hash != expected_previous {
            findings.push(finding(
                index,
                record,
                ChainFindingKind::BrokenPredecessor,
                format!(
                    "expected predecessor {}, observed {}",
                    expected_previous, record.previous_hash
                ),
            ));
        }
        if let Err(error) = verify(record) {
            findings.push(finding(
                index,
                record,
                ChainFindingKind::InvalidDigest,
                error.to_string(),
            ));
        }
        if record.correlation_id.trim().is_empty() {
            findings.push(finding(
                index,
                record,
                ChainFindingKind::EmptyCorrelation,
                "correlation identifier is empty",
            ));
        }
        if !payload_is_json(&record.payload) {
            findings.push(finding(
                index,
                record,
                ChainFindingKind::PayloadShape,
                "payload is not a JSON value",
            ));
        }
        total_payload_bytes += record.payload.len();
        let summary = summaries
            .entry(record.correlation_id.clone())
            .or_insert_with(|| CorrelationSummary {
                correlation_id: record.correlation_id.clone(),
                record_count: 0,
                first_index: index,
                last_index: index,
                payload_bytes: 0,
                terminal_hash: String::new(),
            });
        summary.record_count += 1;
        summary.last_index = index;
        summary.payload_bytes += record.payload.len();
        summary.terminal_hash = record.hash.clone();
        expected_previous = record.hash.clone();
    }

    for summary in summaries.values() {
        if summary.record_count > 1
            && summary.last_index - summary.first_index + 1 != summary.record_count
        {
            if let Some(record) = records.get(summary.last_index) {
                findings.push(finding(
                    summary.last_index,
                    record,
                    ChainFindingKind::CorrelationDiscontinuity,
                    format!(
                        "correlation {} is interleaved with other operations",
                        summary.correlation_id
                    ),
                ));
            }
        }
    }

    ChainReport {
        valid: findings.is_empty(),
        record_count: records.len(),
        unique_correlations: summaries.len(),
        total_payload_bytes,
        genesis_hash,
        terminal_hash: records
            .last()
            .map(|record| record.hash.clone())
            .unwrap_or_default(),
        findings,
        correlations: summaries.into_values().collect(),
    }
}

pub fn verify_chain(records: &[EvidenceRecord]) -> Result<ChainReport, DomainError> {
    let report = inspect_chain(records);
    if report.valid {
        Ok(report)
    } else {
        let details = report
            .findings
            .iter()
            .take(4)
            .map(|finding| format!("{}:{:?}", finding.record_id, finding.kind))
            .collect::<Vec<_>>()
            .join(",");
        Err(DomainError::Integrity(format!(
            "evidence chain failed verification: {details}"
        )))
    }
}

pub fn replay_correlation(
    records: &[EvidenceRecord],
    correlation_id: &str,
) -> Result<ReplayEnvelope, DomainError> {
    if correlation_id.trim().is_empty() {
        return Err(DomainError::Missing("correlation_id"));
    }
    verify_chain(records)?;
    let selected: Vec<_> = records
        .iter()
        .filter(|record| record.correlation_id == correlation_id)
        .cloned()
        .collect();
    if selected.is_empty() {
        return Err(DomainError::Missing("correlation evidence"));
    }
    let complete_chain_prefix = selected
        .first()
        .and_then(|first| records.iter().position(|record| record.hash == first.hash))
        .map(|index| {
            index == 0
                || records
                    .get(index - 1)
                    .map(|previous| selected[0].previous_hash == previous.hash)
                    .unwrap_or(false)
        })
        .unwrap_or(false);
    let terminal_hash = selected
        .last()
        .map(|record| record.hash.clone())
        .unwrap_or_default();
    Ok(ReplayEnvelope {
        correlation_id: correlation_id.to_string(),
        records: selected,
        complete_chain_prefix,
        terminal_hash,
    })
}

pub fn verify_append(
    existing: &[EvidenceRecord],
    candidate: &EvidenceRecord,
) -> Result<(), DomainError> {
    verify(candidate)?;
    if existing.iter().any(|record| record.id == candidate.id) {
        return Err(DomainError::Integrity(format!(
            "candidate record id {} already exists",
            candidate.id
        )));
    }
    let expected_previous = existing
        .last()
        .map(|record| record.hash.as_str())
        .unwrap_or("genesis");
    if candidate.previous_hash != expected_previous {
        return Err(DomainError::Integrity(format!(
            "candidate predecessor {} does not match terminal {}",
            candidate.previous_hash, expected_previous
        )));
    }
    let mut combined = existing.to_vec();
    combined.push(candidate.clone());
    verify_chain(&combined)?;
    Ok(())
}

pub fn correlation_index(
    records: &[EvidenceRecord],
) -> Result<BTreeMap<String, Vec<usize>>, DomainError> {
    verify_chain(records)?;
    let mut index = BTreeMap::new();
    for (position, record) in records.iter().enumerate() {
        index
            .entry(record.correlation_id.clone())
            .or_insert_with(Vec::new)
            .push(position);
    }
    Ok(index)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::seal;

    fn chain() -> Vec<EvidenceRecord> {
        let first = seal("one", "corr-a", r#"{"status":"started"}"#, "genesis").unwrap();
        let second = seal("two", "corr-a", r#"{"status":"completed"}"#, &first.hash).unwrap();
        let third = seal("three", "corr-b", r#"{"status":"audited"}"#, &second.hash).unwrap();
        vec![first, second, third]
    }

    #[test]
    fn complete_chain_reports_correlations_and_terminal_hash() {
        let records = chain();
        let report = verify_chain(&records).unwrap();
        assert!(report.valid);
        assert_eq!(report.record_count, 3);
        assert_eq!(report.unique_correlations, 2);
        assert_eq!(report.terminal_hash, records[2].hash);
    }

    #[test]
    fn tampered_payload_is_detected() {
        let mut records = chain();
        records[1].payload = r#"{"status":"forged"}"#.into();
        let report = inspect_chain(&records);
        assert!(!report.valid);
        assert!(report
            .findings
            .iter()
            .any(|finding| finding.kind == ChainFindingKind::InvalidDigest));
    }

    #[test]
    fn predecessor_break_is_detected_even_with_valid_digest() {
        let mut records = chain();
        records[1] = seal(
            "two",
            "corr-a",
            r#"{"status":"completed"}"#,
            "wrong-predecessor",
        )
        .unwrap();
        let report = inspect_chain(&records);
        assert!(report
            .findings
            .iter()
            .any(|finding| finding.kind == ChainFindingKind::BrokenPredecessor));
    }

    #[test]
    fn replay_selects_only_requested_correlation() {
        let records = chain();
        let replay = replay_correlation(&records, "corr-a").unwrap();
        assert_eq!(replay.records.len(), 2);
        assert_eq!(replay.terminal_hash, records[1].hash);
    }

    #[test]
    fn append_rejects_duplicate_id() {
        let records = chain();
        let candidate = seal(
            "two",
            "corr-c",
            r#"{"status":"duplicate"}"#,
            &records[2].hash,
        )
        .unwrap();
        assert!(
            matches!(verify_append(&records, &candidate), Err(DomainError::Integrity(message)) if message.contains("already exists"))
        );
    }

    #[test]
    fn correlation_index_is_stable() {
        let records = chain();
        let index = correlation_index(&records).unwrap();
        assert_eq!(index["corr-a"], vec![0, 1]);
        assert_eq!(index["corr-b"], vec![2]);
    }
}
