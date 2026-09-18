//! Media image builder and volume directory generator (§14.5).
//!
//! Prepares deployable FAT32 filesystem structures and outputs volume checksum manifests.

use crate::layout::{Fat32Constraints, VolumeSplitter};
use mmi_core::CoreError;
use mmi_rebuild::StageNormalizer;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Result of building a media image volume.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaBuildResult {
    pub volume_label: String,
    pub volume_directory: PathBuf,
    pub total_bytes: u64,
    pub file_count: usize,
    pub manifest_blake3: String,
}

pub struct MediaBuilder;

impl MediaBuilder {
    /// Builds deployment media folder(s) from a stage directory.
    pub fn build(
        stage_dir: &Path,
        output_root: &Path,
        volume_label: &str,
        constraints: &Fat32Constraints,
    ) -> Result<Vec<MediaBuildResult>, CoreError> {
        let entries = StageNormalizer::collect_normalized_tree(stage_dir)?;

        let mut file_pairs = Vec::new();
        for e in &entries {
            file_pairs.push((PathBuf::from(&e.relative_path), e.size_bytes));
        }

        let volumes = VolumeSplitter::partition_volumes(
            &file_pairs,
            constraints.max_volume_bytes,
            volume_label,
        );

        let mut results = Vec::new();

        for vol in volumes {
            let vol_dir = if vol.volume_index == 1 && vol.volume_label == volume_label {
                output_root.to_path_buf()
            } else {
                output_root.join(&vol.volume_label)
            };

            std::fs::create_dir_all(&vol_dir)?;

            let mut manifest_entries = Vec::new();

            for rel_path in &vol.files {
                let src = stage_dir.join(rel_path);
                let dst = vol_dir.join(rel_path);

                if let Some(parent) = dst.parent() {
                    std::fs::create_dir_all(parent)?;
                }

                std::fs::copy(&src, &dst)?;

                let bytes = std::fs::read(&dst)?;
                let file_hash = blake3::hash(&bytes).to_hex().to_string();
                manifest_entries.push(serde_json::json!({
                    "path": rel_path.to_string_lossy(),
                    "size": bytes.len(),
                    "blake3": file_hash,
                }));
            }

            // Write media volume manifest
            let manifest_json = serde_json::to_string_pretty(&manifest_entries).unwrap();
            let manifest_path = vol_dir.join("media_manifest.json");
            std::fs::write(&manifest_path, manifest_json.as_bytes())?;

            let manifest_hash = blake3::hash(manifest_json.as_bytes()).to_hex().to_string();

            results.push(MediaBuildResult {
                volume_label: vol.volume_label,
                volume_directory: vol_dir,
                total_bytes: vol.total_bytes,
                file_count: vol.file_count,
                manifest_blake3: manifest_hash,
            });
        }

        Ok(results)
    }
}
