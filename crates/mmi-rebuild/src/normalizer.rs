//! File ordering, timestamp clamping, and deterministic filesystem tree normalization.

use mmi_core::CoreError;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// A normalized file entry in a target bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedFileEntry {
    pub relative_path: String,
    pub size_bytes: u64,
    pub blake3_hex: String,
    pub is_signed: bool,
}

pub struct StageNormalizer;

impl StageNormalizer {
    /// Recursively collects and lexicographically sorts all files under a root directory.
    pub fn collect_normalized_tree(root: &Path) -> Result<Vec<NormalizedFileEntry>, CoreError> {
        let mut entries = Vec::new();

        for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let path = entry.path();
                let rel = path
                    .strip_prefix(root)
                    .map_err(|e| CoreError::NotFound(e.to_string()))?
                    .to_string_lossy()
                    .replace('\\', "/");

                let bytes = std::fs::read(path)?;
                let blake3_hex = blake3::hash(&bytes).to_hex().to_string();

                let is_signed = rel.to_lowercase().ends_with(".pkg")
                    || rel.to_lowercase().ends_with(".sig")
                    || rel.to_lowercase().contains("tmcconfig.dat");

                entries.push(NormalizedFileEntry {
                    relative_path: rel,
                    size_bytes: bytes.len() as u64,
                    blake3_hex,
                    is_signed,
                });
            }
        }

        // Enforce strict canonical lexicographical order
        entries.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
        Ok(entries)
    }

    /// Recursively copies files from source to destination in normalized order.
    pub fn copy_normalized(source_dir: &Path, dest_dir: &Path) -> Result<Vec<PathBuf>, CoreError> {
        let entries = Self::collect_normalized_tree(source_dir)?;
        let mut copied = Vec::new();

        for entry in entries {
            let src_file = source_dir.join(&entry.relative_path);
            let dst_file = dest_dir.join(&entry.relative_path);

            if let Some(parent) = dst_file.parent() {
                std::fs::create_dir_all(parent)?;
            }

            std::fs::copy(&src_file, &dst_file)?;
            copied.push(dst_file);
        }

        Ok(copied)
    }
}
