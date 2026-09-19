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

/// Detailed summary of a media sanitization operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaSanitizationReport {
    pub target_directory: PathBuf,
    pub purged_items: Vec<String>,
    pub reclaimed_bytes: u64,
    pub is_dry_run: bool,
    pub compliant: bool,
}

/// Utility for scanning and sanitizing removable SD media for Audi MMI QNX compatibility.
///
/// Purges host OS junk files (.DS_Store, AppleDouble ._*, Thumbs.db, .Trashes)
/// that cause QNX SWDL and scriptlauncher parsing failures or "Medium unreadable" errors.
pub struct MediaSanitizer;

impl MediaSanitizer {
    /// Returns true if the file or directory name is host OS metadata or junk.
    pub fn is_junk_name(name: &str) -> bool {
        name == ".DS_Store"
            || name.starts_with("._")
            || name.eq_ignore_ascii_case("thumbs.db")
            || name.eq_ignore_ascii_case("desktop.ini")
            || name == ".Spotlight-V100"
            || name == ".Trashes"
            || name == ".fseventsd"
    }

    /// Recursively scans and purges junk files from target directory.
    pub fn sanitize(
        target_dir: &std::path::Path,
        dry_run: bool,
    ) -> Result<MediaSanitizationReport, std::io::Error> {
        let mut purged_items = Vec::new();
        let mut reclaimed_bytes: u64 = 0;

        if target_dir.exists() {
            let mut it = walkdir::WalkDir::new(target_dir).into_iter();
            while let Some(entry_res) = it.next() {
                let entry = match entry_res {
                    Ok(e) => e,
                    Err(_) => continue,
                };
                let file_name = entry.file_name().to_string_lossy();
                if Self::is_junk_name(&file_name) {
                    let path = entry.path().to_path_buf();
                    let is_dir = entry.file_type().is_dir();
                    let rel = path
                        .strip_prefix(target_dir)
                        .unwrap_or(&path)
                        .to_string_lossy()
                        .to_string();

                    let item_bytes = if is_dir {
                        walkdir::WalkDir::new(&path)
                            .into_iter()
                            .filter_map(|e| e.ok())
                            .filter_map(|e| e.metadata().ok())
                            .filter(|m| m.is_file())
                            .map(|m| m.len())
                            .sum()
                    } else {
                        entry.metadata().map(|m| m.len()).unwrap_or(0)
                    };

                    purged_items.push(rel);
                    reclaimed_bytes += item_bytes;

                    if !dry_run {
                        if is_dir {
                            let _ = std::fs::remove_dir_all(&path);
                        } else {
                            let _ = std::fs::remove_file(&path);
                        }
                    }

                    if is_dir {
                        // Don't descend into directory we just marked/purged
                        it.skip_current_dir();
                    }
                }
            }
        }

        Ok(MediaSanitizationReport {
            target_directory: target_dir.to_path_buf(),
            purged_items,
            reclaimed_bytes,
            is_dry_run: dry_run,
            compliant: true,
        })
    }
}
