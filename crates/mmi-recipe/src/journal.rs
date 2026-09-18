//! Cryptographically linked, append-only modification journal (§13.6).
//!
//! Maintains an immutable hash chain of applied recipe operations, ensuring
//! complete provenance and auditability.

use crate::model::RecipeOperation;
use blake3::Hasher;
use mmi_core::CoreError;
use serde::{Deserialize, Serialize};

/// A single entry in the hash-chained journal.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JournalEntry {
    /// Zero-based sequential entry index
    pub sequence: u64,
    /// ISO 8601 timestamp
    pub timestamp: String,
    /// BLAKE3 digest of previous journal entry (or 64 zeroes for genesis)
    pub prev_hash: String,
    /// The recipe operation recorded in this entry
    pub operation: RecipeOperation,
    /// Target asset or entity digest before modification
    pub before_hash: Option<String>,
    /// Target asset or entity digest after modification
    pub after_hash: Option<String>,
    /// Current entry cryptographic digest (BLAKE3)
    pub entry_hash: String,
}

/// Append-only journal managing an unbroken hash chain of applied transformations.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JournalChain {
    pub entries: Vec<JournalEntry>,
}

impl JournalChain {
    pub const GENESIS_HASH: &'static str =
        "0000000000000000000000000000000000000000000000000000000000000000";

    /// Appends a new operation to the journal and computes its cryptographic hash.
    pub fn append(
        &mut self,
        operation: RecipeOperation,
        before_hash: Option<String>,
        after_hash: Option<String>,
    ) -> &JournalEntry {
        let sequence = self.entries.len() as u64;
        let prev_hash = if let Some(last) = self.entries.last() {
            last.entry_hash.clone()
        } else {
            Self::GENESIS_HASH.to_string()
        };

        let timestamp = "2026-09-18T17:20:00Z".to_string(); // Canonical determinism

        let mut hasher = Hasher::new();
        hasher.update(sequence.to_string().as_bytes());
        hasher.update(prev_hash.as_bytes());
        let op_json = serde_json::to_string(&operation).unwrap_or_default();
        hasher.update(op_json.as_bytes());
        if let Some(b) = &before_hash {
            hasher.update(b.as_bytes());
        }
        if let Some(a) = &after_hash {
            hasher.update(a.as_bytes());
        }
        let entry_hash = hasher.finalize().to_hex().to_string();

        let entry = JournalEntry {
            sequence,
            timestamp,
            prev_hash,
            operation,
            before_hash,
            after_hash,
            entry_hash,
        };

        self.entries.push(entry);
        self.entries.last().unwrap()
    }

    /// Verifies the cryptographic integrity of the entire journal chain.
    pub fn verify_integrity(&self) -> Result<(), CoreError> {
        let mut expected_prev = Self::GENESIS_HASH.to_string();

        for (idx, entry) in self.entries.iter().enumerate() {
            if entry.sequence != idx as u64 {
                return Err(CoreError::ImmutabilityViolation(format!(
                    "Journal sequence mismatch at {idx}: expected {idx}, got {}",
                    entry.sequence
                )));
            }

            if entry.prev_hash != expected_prev {
                return Err(CoreError::ImmutabilityViolation(format!(
                    "Journal chain broken at {idx}: prev_hash {} != expected {expected_prev}",
                    entry.prev_hash
                )));
            }

            // Recompute BLAKE3 hash
            let mut hasher = Hasher::new();
            hasher.update(entry.sequence.to_string().as_bytes());
            hasher.update(entry.prev_hash.as_bytes());
            let op_json = serde_json::to_string(&entry.operation).unwrap_or_default();
            hasher.update(op_json.as_bytes());
            if let Some(b) = &entry.before_hash {
                hasher.update(b.as_bytes());
            }
            if let Some(a) = &entry.after_hash {
                hasher.update(a.as_bytes());
            }
            let computed = hasher.finalize().to_hex().to_string();

            if computed != entry.entry_hash {
                return Err(CoreError::CorruptBlob {
                    expected: computed,
                    actual: entry.entry_hash.clone(),
                });
            }

            expected_prev = entry.entry_hash.clone();
        }

        Ok(())
    }
}
