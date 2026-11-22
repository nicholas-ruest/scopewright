use scopewright_evidence::{verify, EvidenceRecord};
use scopewright_kernel::DomainError;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct EvidenceStore {
    path: PathBuf,
}

impl EvidenceStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DomainError> {
        let path = path.as_ref().to_path_buf();
        if let Some(parent) = path.parent() {
            create_dir_all(parent).map_err(|e| DomainError::Storage(e.to_string()))?;
        }
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| DomainError::Storage(e.to_string()))?;
        Ok(Self { path })
    }
    pub fn append(&self, record: &EvidenceRecord) -> Result<(), DomainError> {
        verify(record)?;
        let mut file = OpenOptions::new()
            .append(true)
            .open(&self.path)
            .map_err(|e| DomainError::Storage(e.to_string()))?;
        let line =
            serde_json::to_string(record).map_err(|e| DomainError::Storage(e.to_string()))?;
        writeln!(file, "{line}").map_err(|e| DomainError::Storage(e.to_string()))?;
        file.sync_data()
            .map_err(|e| DomainError::Storage(e.to_string()))
    }
    pub fn load(&self) -> Result<Vec<EvidenceRecord>, DomainError> {
        let file = File::open(&self.path).map_err(|e| DomainError::Storage(e.to_string()))?;
        BufReader::new(file)
            .lines()
            .filter_map(|line| match line {
                Ok(text) if text.trim().is_empty() => None,
                Ok(text) => Some(
                    serde_json::from_str(&text).map_err(|e| DomainError::Storage(e.to_string())),
                ),
                Err(e) => Some(Err(DomainError::Storage(e.to_string()))),
            })
            .collect()
    }
    pub fn path(&self) -> &Path {
        &self.path
    }
}
