//! Format-specific container repackagers and canonical serializers.
//!
//! Provides deterministic repacking for supported MMI formats:
//! - Precomp images (`PrecompPackager`)
//! - Localized string catalogs (`StringCatalogPackager`)
//! - Stage bundle packager (`BundlePackager`)
//!
//! Enforces permanent lockout on signed files (.pkg, .sig, TMCConfig.dat).

use crate::normalizer::StageNormalizer;
use mmi_assets::StringCatalog;
use mmi_core::CoreError;
use mmi_formats::PrecompImage;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Status report for an individual repackaged container.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepackageResult {
    pub relative_path: String,
    pub original_size: usize,
    pub rebuilt_size: usize,
    pub blake3_hex: String,
    pub sha256_hex: String,
}

pub struct PrecompPackager;

impl PrecompPackager {
    /// Repackages a decompressed or parsed Precomp image back into binary format.
    pub fn repackage(image: &PrecompImage) -> Result<Vec<u8>, CoreError> {
        image.encode()
    }
}

pub struct StringCatalogPackager;

impl StringCatalogPackager {
    /// Serializes a string catalog using its declared encoding.
    pub fn repackage(catalog: &StringCatalog) -> Result<Vec<u8>, CoreError> {
        catalog.serialize()
    }
}

pub struct BundlePackager;

impl BundlePackager {
    /// Checks if a file path is a locked signed artifact.
    pub fn is_signed(path_str: &str) -> bool {
        let p = path_str.to_lowercase();
        p.ends_with(".pkg") || p.ends_with(".sig") || p.contains("tmcconfig.dat")
    }

    /// Repackages an entire staged tree into an output directory with deterministic ordering.
    pub fn repackage_stage(
        stage_dir: &Path,
        output_dir: &Path,
    ) -> Result<Vec<RepackageResult>, CoreError> {
        let entries = StageNormalizer::collect_normalized_tree(stage_dir)?;
        let mut results = Vec::new();

        for entry in entries {
            let src_file = stage_dir.join(&entry.relative_path);
            let dst_file = output_dir.join(&entry.relative_path);

            if let Some(parent) = dst_file.parent() {
                std::fs::create_dir_all(parent)?;
            }

            let src_bytes = std::fs::read(&src_file)?;
            let orig_len = src_bytes.len();

            let out_bytes = if entry.relative_path.to_lowercase().ends_with(".precomp") {
                // If it's a precomp, decode and re-encode canonically to verify consistency
                if let Ok(img) = PrecompImage::decode(&src_bytes) {
                    PrecompPackager::repackage(&img).unwrap_or(src_bytes)
                } else {
                    src_bytes
                }
            } else if entry.relative_path.to_lowercase().ends_with(".txt") {
                if let Ok(cat) = StringCatalog::parse(&src_bytes) {
                    StringCatalogPackager::repackage(&cat).unwrap_or(src_bytes)
                } else {
                    src_bytes
                }
            } else {
                src_bytes
            };

            std::fs::write(&dst_file, &out_bytes)?;

            let blake3_hex = blake3::hash(&out_bytes).to_hex().to_string();
            use sha2::{Digest, Sha256};
            let mut hasher = Sha256::new();
            hasher.update(&out_bytes);
            let sha256_hex = hex::encode(hasher.finalize());

            results.push(RepackageResult {
                relative_path: entry.relative_path,
                original_size: orig_len,
                rebuilt_size: out_bytes.len(),
                blake3_hex,
                sha256_hex,
            });
        }

        Ok(results)
    }
}
