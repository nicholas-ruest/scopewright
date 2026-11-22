use scopewright_evidence::{seal, verify, EvidenceRecord};
use scopewright_kernel::DomainError;

#[derive(Debug, Default)]
pub struct AuditChain {
    records: Vec<EvidenceRecord>,
}

impl AuditChain {
    pub fn append(
        &mut self,
        id: &str,
        correlation: &str,
        payload: &str,
    ) -> Result<&EvidenceRecord, DomainError> {
        let previous = self
            .records
            .last()
            .map(|r| r.hash.as_str())
            .unwrap_or("genesis");
        let record = seal(id, correlation, payload, previous)?;
        self.records.push(record);
        Ok(self.records.last().expect("record was pushed"))
    }
    pub fn verify_all(&self) -> Result<(), DomainError> {
        let mut previous = "genesis";
        for record in &self.records {
            if record.previous_hash != previous {
                return Err(DomainError::Integrity("audit chain discontinuity".into()));
            }
            verify(record)?;
            previous = &record.hash;
        }
        Ok(())
    }
    pub fn records(&self) -> &[EvidenceRecord] {
        &self.records
    }
}
