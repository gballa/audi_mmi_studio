//! Error types for mmi-core.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Immutability violation: {0}")]
    ImmutabilityViolation(String),

    #[error("Path traversal / escape attempt: {0}")]
    PathEscape(String),

    #[error("Item not found: {0}")]
    NotFound(String),

    #[error("Corrupt blob: expected hash {expected}, got {actual}")]
    CorruptBlob {
        expected: String,
        actual: String,
    },

    #[error("Reflink / cloning failed: {0}")]
    ReflinkFailed(String),

    #[error("Signed artefact is immutable (ERR_SIGNED_ARTEFACT_IMMUTABLE): {0}")]
    SignedArtefactImmutable(String),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
}
