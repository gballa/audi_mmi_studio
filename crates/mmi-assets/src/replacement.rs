//! Manual Asset Replacement Engine with cryptographic provenance tracking.

use mmi_core::{ContentAddressedStore, CoreError};
use serde::{Deserialize, Serialize};
use sha2::Digest;

use crate::conformance::{AssetConstraints, ConformancePipeline, ConformanceVerdict};
use crate::decoder::AssetDecoder;

/// Record of an admitted asset modification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplacementRecord {
    pub target_asset: String,
    pub original_blob_id: String,
    pub original_sha256: String,
    pub replacement_blob_id: String,
    pub replacement_sha256: String,
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub encoded_bytes: usize,
}

pub struct AssetReplacer;

impl AssetReplacer {
    /// Evaluates candidate replacement bytes against target asset file, conform it,
    /// and emits a `ReplacementRecord`.
    pub fn replace(
        target_path: &str,
        target_original_bytes: &[u8],
        replacement_image_bytes: &[u8],
        cas: &ContentAddressedStore,
    ) -> Result<ReplacementRecord, CoreError> {
        // Measure target original
        let decoded_orig = AssetDecoder::decode(target_original_bytes)?;
        let orig_b3 = blake3::hash(target_original_bytes).to_hex().to_string();
        let orig_sha = hex::encode(sha2::Sha256::digest(target_original_bytes));

        let constraints = AssetConstraints {
            target_width: decoded_orig.width,
            target_height: decoded_orig.height,
            target_format: decoded_orig.source_format.clone(),
            max_encoded_bytes: Some((target_original_bytes.len() as f32 * 1.5) as usize), // 50% headroom
            require_alpha: decoded_orig.has_alpha,
        };

        match ConformancePipeline::process(replacement_image_bytes, &constraints, cas) {
            ConformanceVerdict::Admitted {
                blob_id,
                sha256_hex,
                encoded_bytes,
                width,
                height,
                format,
            } => Ok(ReplacementRecord {
                target_asset: target_path.to_string(),
                original_blob_id: orig_b3,
                original_sha256: orig_sha,
                replacement_blob_id: blob_id,
                replacement_sha256: sha256_hex,
                width,
                height,
                format,
                encoded_bytes,
            }),
            ConformanceVerdict::Rejected {
                stage,
                reason,
                details,
            } => Err(CoreError::ImmutabilityViolation(format!(
                "Conformance rejected at stage [{}]: {} ({})",
                stage, reason, details
            ))),
        }
    }
}
