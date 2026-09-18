//! Extractor: Non-destructive ingest and normalization of MMI packages into CAS and StageStore.

use std::path::Path;
use walkdir::WalkDir;

use crate::cas::ContentAddressedStore;
use crate::error::CoreError;
use crate::project::{
    ConfidenceLevel, MMIProject, ModuleRecord, PlatformVariant, SourceRef, TaggedValue,
};
use crate::source_store::SourceStore;
use crate::stage::{StageEntry, StageStore};

/// Non-destructive extractor pipeline.
pub struct PackageExtractor<'a> {
    source_store: &'a SourceStore,
    cas: &'a ContentAddressedStore,
}

impl<'a> PackageExtractor<'a> {
    pub fn new(source_store: &'a SourceStore, cas: &'a ContentAddressedStore) -> Self {
        Self { source_store, cas }
    }

    /// Recursively ingests a folder or single file from `SourceStore` into `cas` and records
    /// it into the `StageStore` and `MMIProject`.
    pub fn extract_package(
        &self,
        relative_source_path: impl AsRef<Path>,
        stage_store: &mut StageStore,
        project: &mut MMIProject,
    ) -> Result<(), CoreError> {
        let root = self.source_store.resolve(&relative_source_path)?;

        if root.is_file() {
            let rel_path = relative_source_path.as_ref().to_string_lossy().to_string();
            self.ingest_file(&rel_path, stage_store, project)?;
            return Ok(());
        }

        // Walk source directory
        for entry in WalkDir::new(&root).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let entry_path = entry.path();
                // Compute path relative to source_store root
                let rel = entry_path
                    .strip_prefix(self.source_store.root().as_path())
                    .map_err(|_| {
                        CoreError::PathEscape(format!(
                            "Failed to strip prefix from {}",
                            entry_path.display()
                        ))
                    })?;
                let rel_str = rel.to_string_lossy().to_string();
                self.ingest_file(&rel_str, stage_store, project)?;
            }
        }

        self.normalize_project_metadata(project, stage_store)?;

        Ok(())
    }

    fn ingest_file(
        &self,
        rel_path: &str,
        stage_store: &mut StageStore,
        project: &mut MMIProject,
    ) -> Result<(), CoreError> {
        let abs_path = self.source_store.resolve(rel_path)?;
        let (blake3_hex, sha256_hex) = self.cas.put_file(abs_path.as_path())?;
        let data = self.cas.read_bytes(&blake3_hex)?;
        let byte_size = data.len() as u64;
        let is_signed = MMIProject::is_signed_payload(rel_path);

        // Derive module name if path is structured as <train>/<module>/...
        // Ignore top-level files like metainfo2.txt which have no subdirectory module
        let parts: Vec<&str> = rel_path.split('/').collect();
        let module_name = if parts.len() > 2 {
            Some(parts[1].to_string())
        } else {
            None
        };

        let stage_entry = StageEntry {
            logical_path: rel_path.to_string(),
            blob_id: blake3_hex.clone(),
            sha256_hex: sha256_hex.clone(),
            byte_size,
            is_signed,
            module_name: module_name.clone(),
        };

        stage_store.put_entry(&stage_entry)?;

        project.source_refs.push(SourceRef {
            logical_path: rel_path.to_string(),
            blake3_hex: blake3_hex.clone(),
            sha256_hex: sha256_hex.clone(),
            byte_size,
            is_signed,
        });

        if is_signed {
            project.register_signed_artefact(
                rel_path,
                if rel_path.ends_with(".pkg") {
                    Some(format!("{}.sig", rel_path))
                } else {
                    None
                },
                blake3_hex,
                sha256_hex,
            );
        }

        Ok(())
    }

    /// Normalizes top-level project metadata from ingested stage entries.
    fn normalize_project_metadata(
        &self,
        project: &mut MMIProject,
        stage_store: &StageStore,
    ) -> Result<(), CoreError> {
        let entries = stage_store.list_entries()?;

        // Look for metainfo2.txt
        for entry in &entries {
            if entry.logical_path.ends_with("metainfo2.txt") {
                let bytes = self.cas.read_bytes(&entry.blob_id)?;
                if let Ok(text) = std::str::from_utf8(&bytes) {
                    self.parse_metainfo2_into_project(text, project, &entry.logical_path);
                }
            }
        }

        // Aggregate modules
        let mut modules_map: std::collections::HashMap<String, (usize, u64, bool)> =
            std::collections::HashMap::new();

        for entry in &entries {
            if let Some(ref m) = entry.module_name {
                let entry_stat = modules_map.entry(m.clone()).or_insert((0, 0, false));
                entry_stat.0 += 1;
                entry_stat.1 += entry.byte_size;
                if entry.is_signed {
                    entry_stat.2 = true;
                }
            }
        }

        for (mod_name, (count, size, signed)) in modules_map {
            project.modules.push(ModuleRecord {
                name: mod_name.clone(),
                source_folder: mod_name,
                version: None,
                file_count: count,
                total_bytes: size,
                is_signed: signed,
            });
        }

        Ok(())
    }

    fn parse_metainfo2_into_project(&self, text: &str, project: &mut MMIProject, manifest_path: &str) {
        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("release") || trimmed.starts_with("Release") {
                if let Some((_, val)) = trimmed.split_once('=') {
                    let cleaned = val.trim().trim_matches('"').trim_matches('\'').to_string();
                    let platform = if cleaned.starts_with("HN+R") {
                        PlatformVariant::HnPlusR
                    } else if cleaned.starts_with("HN+") {
                        PlatformVariant::HnPlus
                    } else {
                        PlatformVariant::Unknown(cleaned.clone())
                    };

                    project.detected_platform = TaggedValue {
                        value: platform,
                        confidence: ConfidenceLevel::Known,
                        evidence: format!("[EV:manifest:{}]", manifest_path),
                    };

                    project.software_train = Some(TaggedValue {
                        value: cleaned,
                        confidence: ConfidenceLevel::Known,
                        evidence: format!("[EV:manifest:{}]", manifest_path),
                    });
                }
            }
        }
    }
}
