//! FAT32 SD-card layout constraints, volume geometry, and multi-volume partitioner (§14.5).

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Constraints for FAT32 SD media compatibility in Audi MMI head units.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fat32Constraints {
    pub max_volume_bytes: u64,
    pub cluster_size_bytes: u32,
    pub volume_label_max_chars: usize,
    pub max_path_depth: usize,
    pub max_path_length: usize,
}

impl Default for Fat32Constraints {
    fn default() -> Self {
        Self {
            // Standard 32 GiB maximum for standard FAT32 SDHC compatibility
            max_volume_bytes: 32 * 1024 * 1024 * 1024,
            // 32 KiB cluster size optimal for SDHC cards
            cluster_size_bytes: 32 * 1024,
            volume_label_max_chars: 11,
            max_path_depth: 8,
            max_path_length: 255,
        }
    }
}

/// An individual media volume (e.g. SD1, SD2) within a deployment set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaVolume {
    pub volume_index: usize,
    pub volume_label: String,
    pub total_bytes: u64,
    pub file_count: usize,
    pub files: Vec<PathBuf>,
}

pub struct VolumeSplitter;

impl VolumeSplitter {
    /// Partitions a list of files into one or more volumes respecting size ceilings.
    pub fn partition_volumes(
        entries: &[(PathBuf, u64)],
        max_bytes_per_volume: u64,
        label_prefix: &str,
    ) -> Vec<MediaVolume> {
        let mut volumes = Vec::new();
        let mut current_files = Vec::new();
        let mut current_bytes: u64 = 0;

        for (path, size) in entries {
            // If adding this file exceeds the volume limit and current volume is non-empty, roll over
            if current_bytes + size > max_bytes_per_volume && !current_files.is_empty() {
                let vol_idx = volumes.len() + 1;
                volumes.push(MediaVolume {
                    volume_index: vol_idx,
                    volume_label: format!("{}_{}", label_prefix, vol_idx),
                    total_bytes: current_bytes,
                    file_count: current_files.len(),
                    files: current_files,
                });
                current_files = Vec::new();
                current_bytes = 0;
            }

            current_files.push(path.clone());
            current_bytes += size;
        }

        if !current_files.is_empty() {
            let vol_idx = volumes.len() + 1;
            volumes.push(MediaVolume {
                volume_index: vol_idx,
                volume_label: if volumes.is_empty() {
                    label_prefix.to_string()
                } else {
                    format!("{}_{}", label_prefix, vol_idx)
                },
                total_bytes: current_bytes,
                file_count: current_files.len(),
                files: current_files,
            });
        }

        volumes
    }
}
