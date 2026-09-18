//! Opaque byte span passthrough for unknown binary regions.

use serde::{Deserialize, Serialize};

/// Represents an unparsed or proprietary region in a binary container that must
/// be passed through verbatim during repackaging without modification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpaqueByteSpan {
    pub offset: usize,
    pub length: usize,
    pub description: String,
    pub blob_id: Option<String>,
}

impl OpaqueByteSpan {
    pub fn new(offset: usize, length: usize, description: impl Into<String>) -> Self {
        Self {
            offset,
            length,
            description: description.into(),
            blob_id: None,
        }
    }

    /// Associates this opaque span with a CAS blob ID.
    pub fn with_blob_id(mut self, blob_id: impl Into<String>) -> Self {
        self.blob_id = Some(blob_id.into());
        self
    }
}
