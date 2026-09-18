//! Identity-Rebuild Gate (Keystone Protocol): Automated extract -> normalise -> rebuild verification.

use mmi_core::CoreError;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::adapter::FormatAdapter;
use crate::precomp::{PrecompAdapter, PrecompImage};

/// Cryptographic attestation and verdict emitted by the Identity-Rebuild Gate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GateVerdict {
    /// Exact bit-for-bit parity achieved (identical BLAKE3 and SHA-256 digests).
    PassedBitForBit,
    /// Passed canonical verification where pixel payload is verified 100% identical.
    PassedCanonical {
        rationale: String,
        pixel_bytes_matched: usize,
    },
    /// Rebuilt byte sequence differs from original.
    FailedMismatch {
        orig_bytes: usize,
        rebuilt_bytes: usize,
        orig_blake3: String,
        rebuilt_blake3: String,
    },
    /// Cryptographically signed or licensed asset (permanently locked from modification).
    LockedSignedArtefact { policy: String },
    /// Format does not yet have a verified rebuild encoder.
    UnsupportedFormat { format_name: String },
}

/// Detailed certification report emitted for each evaluated binary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GateReport {
    pub file_path: String,
    pub format_name: String,
    pub can_rebuild: bool,
    pub verdict: GateVerdict,
    pub blake3_hex: String,
    pub sha256_hex: String,
}

pub struct IdentityRebuildGate;

impl IdentityRebuildGate {
    /// Evaluates candidate binary data through the Identity-Rebuild Gate.
    pub fn evaluate(file_path: &str, data: &[u8]) -> Result<GateReport, CoreError> {
        let orig_blake3 = blake3::hash(data).to_hex().to_string();
        let orig_sha256 = hex::encode(Sha256::digest(data));

        // 1. Check Signed Artefact Gate (§1.4)
        if file_path.ends_with(".pkg")
            || file_path.ends_with(".sig")
            || file_path.ends_with("TMCConfig.dat")
        {
            return Ok(GateReport {
                file_path: file_path.to_string(),
                format_name: "Signed Payload".to_string(),
                can_rebuild: false,
                verdict: GateVerdict::LockedSignedArtefact {
                    policy: "ERR_SIGNED_ARTEFACT_IMMUTABLE: Detached signature exists. Rebuild permanently locked.".to_string(),
                },
                blake3_hex: orig_blake3,
                sha256_hex: orig_sha256,
            });
        }

        // 2. Evaluate Precomp
        let precomp_adapter = PrecompAdapter::default();
        if precomp_adapter.detect(data) {
            let decoded = PrecompImage::decode(data)?;
            let rebuilt = decoded.encode()?;

            let rebuilt_blake3 = blake3::hash(&rebuilt).to_hex().to_string();
            let _rebuilt_sha256 = hex::encode(Sha256::digest(&rebuilt));

            let verdict = if rebuilt == data {
                GateVerdict::PassedBitForBit
            } else {
                // Canonical verification: decompress and check if dimensions & pixels are identical bit-for-bit
                let re_decoded = PrecompImage::decode(&rebuilt)?;
                if decoded.width == re_decoded.width
                    && decoded.height == re_decoded.height
                    && decoded.pixels == re_decoded.pixels
                {
                    GateVerdict::PassedCanonical {
                        rationale: "ADR-010: Zlib Huffman table variations across library implementations canonicalized; uncompressed 32-bit pixel matrix verified 100% bit-for-bit identical".to_string(),
                        pixel_bytes_matched: decoded.pixels.len(),
                    }
                } else {
                    GateVerdict::FailedMismatch {
                        orig_bytes: data.len(),
                        rebuilt_bytes: rebuilt.len(),
                        orig_blake3: orig_blake3.clone(),
                        rebuilt_blake3,
                    }
                }
            };

            let can_rebuild = matches!(
                verdict,
                GateVerdict::PassedBitForBit | GateVerdict::PassedCanonical { .. }
            );

            return Ok(GateReport {
                file_path: file_path.to_string(),
                format_name: "precomp".to_string(),
                can_rebuild,
                verdict,
                blake3_hex: orig_blake3,
                sha256_hex: orig_sha256,
            });
        }

        // 3. Fallback for unhandled / read-only formats
        Ok(GateReport {
            file_path: file_path.to_string(),
            format_name: "unknown_or_readonly".to_string(),
            can_rebuild: false,
            verdict: GateVerdict::UnsupportedFormat {
                format_name: "unknown".to_string(),
            },
            blake3_hex: orig_blake3,
            sha256_hex: orig_sha256,
        })
    }
}
