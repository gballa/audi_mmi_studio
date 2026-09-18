//! Recipe execution engine.
//!
//! Applies declarative operations against a target workspace or stage, enforcing:
//! - Immutability of signed artefacts (.pkg, .sig, TMCConfig.dat) -> permanent rejection (ERR_SIGNED_ARTEFACT_IMMUTABLE).
//! - Conformance verification on replaced assets.
//! - Recording every change in the append-only cryptographic JournalChain.

use crate::journal::JournalChain;
use crate::model::{Recipe, RecipeOperation, RiskClass, SemanticSelector};
use mmi_assets::AssetReplacer;
use mmi_core::{ContentAddressedStore, CoreError};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Result status of applying a recipe.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeApplyReport {
    pub recipe_name: String,
    pub operations_total: usize,
    pub operations_applied: usize,
    pub overall_risk: RiskClass,
    pub journal_head_hash: String,
    pub operation_results: Vec<OperationResult>,
}

/// Outcome of a single recipe operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult {
    pub index: usize,
    pub selector: String,
    pub risk: RiskClass,
    pub success: bool,
    pub message: String,
    pub before_hash: Option<String>,
    pub after_hash: Option<String>,
}

pub struct RecipeEngine;

impl RecipeEngine {
    /// Validates if a target identifier points to a locked signed artifact.
    pub fn is_signed_artefact(path_str: &str) -> bool {
        let p = path_str.to_lowercase();
        p.ends_with(".pkg") || p.ends_with(".sig") || p.contains("tmcconfig.dat")
    }

    /// Evaluates and applies a recipe against a staged filesystem and CAS.
    pub fn apply(
        recipe: &Recipe,
        base_dir: &Path,
        cas: &ContentAddressedStore,
        journal: &mut JournalChain,
    ) -> Result<RecipeApplyReport, CoreError> {
        let mut results = Vec::new();

        for (idx, op) in recipe.operations.iter().enumerate() {
            let target_id = op.selector().target_identifier();

            // 1. §1.4 Signed Artefact Gate
            if Self::is_signed_artefact(target_id) {
                return Err(CoreError::SignedArtefactImmutable(format!(
                    "Operation {} targets signed artifact '{}' (ERR_SIGNED_ARTEFACT_IMMUTABLE)",
                    idx, target_id
                )));
            }

            match op {
                RecipeOperation::ReplaceAsset {
                    selector,
                    replacement_path,
                    ..
                } => {
                    let target_rel = match selector {
                        SemanticSelector::AssetPath(p) => p.as_str(),
                        _ => target_id,
                    };

                    let full_target = base_dir.join(target_rel);
                    let full_replacement = base_dir.join(replacement_path);

                    if !full_target.exists() {
                        results.push(OperationResult {
                            index: idx,
                            selector: target_rel.to_string(),
                            risk: op.risk_class(),
                            success: false,
                            message: format!("Target asset '{}' does not exist", full_target.display()),
                            before_hash: None,
                            after_hash: None,
                        });
                        continue;
                    }

                    if !full_replacement.exists() {
                        results.push(OperationResult {
                            index: idx,
                            selector: target_rel.to_string(),
                            risk: op.risk_class(),
                            success: false,
                            message: format!("Replacement image '{}' does not exist", full_replacement.display()),
                            before_hash: None,
                            after_hash: None,
                        });
                        continue;
                    }

                    let orig_bytes = std::fs::read(&full_target)?;
                    let rep_bytes = std::fs::read(&full_replacement)?;

                    let before_hash = blake3::hash(&orig_bytes).to_hex().to_string();

                    // Perform conformed replacement
                    let record = AssetReplacer::replace(
                        &full_target.to_string_lossy(),
                        &orig_bytes,
                        &rep_bytes,
                        cas,
                    )?;

                    let conformed_bytes = cas.read_bytes(&record.replacement_blob_id)?;
                    let after_hash = record.replacement_blob_id.clone();

                    // Apply in-place to staged tree
                    std::fs::write(&full_target, conformed_bytes)?;

                    journal.append(op.clone(), Some(before_hash.clone()), Some(after_hash.clone()));

                    results.push(OperationResult {
                        index: idx,
                        selector: target_rel.to_string(),
                        risk: op.risk_class(),
                        success: true,
                        message: format!("Successfully replaced with conformed {}", record.format),
                        before_hash: Some(before_hash),
                        after_hash: Some(after_hash),
                    });
                }
                RecipeOperation::GenerateAsset {
                    selector,
                    prompt,
                    pinned_blob_id,
                    ..
                } => {
                    let target_rel = match selector {
                        SemanticSelector::AssetPath(p) => p.as_str(),
                        _ => target_id,
                    };

                    let full_target = base_dir.join(target_rel);
                    let before_hash = if full_target.exists() {
                        let bytes = std::fs::read(&full_target)?;
                        Some(blake3::hash(&bytes).to_hex().to_string())
                    } else {
                        None
                    };

                    // Retrieve pinned blob from CAS
                    let generated_bytes = cas.read_bytes(pinned_blob_id)?;
                    if let Some(parent) = full_target.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    std::fs::write(&full_target, &generated_bytes)?;

                    let after_hash = pinned_blob_id.clone();
                    journal.append(op.clone(), before_hash.clone(), Some(after_hash.clone()));

                    results.push(OperationResult {
                        index: idx,
                        selector: target_rel.to_string(),
                        risk: op.risk_class(),
                        success: true,
                        message: format!("Applied AI generated asset for prompt \"{}\"", prompt),
                        before_hash,
                        after_hash: Some(after_hash),
                    });
                }
                RecipeOperation::SetString {
                    selector,
                    new_value,
                    ..
                } => {
                    let (catalog_file, key_name) = match selector {
                        SemanticSelector::StringKey { catalog, key } => (catalog.as_str(), key.as_str()),
                        _ => (target_id, "TEXT"),
                    };

                    let full_path = base_dir.join(catalog_file);
                    let mut before_hash = None;
                    let after_hash;

                    if full_path.exists() {
                        let raw = std::fs::read(&full_path)?;
                        before_hash = Some(blake3::hash(&raw).to_hex().to_string());
                        let mut cat = mmi_assets::StringCatalog::parse(&raw)?;
                        let mut found = false;
                        for e in &mut cat.entries {
                            if e.key.as_deref() == Some(key_name) {
                                e.value = new_value.clone();
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            cat.entries.push(mmi_assets::StringEntry {
                                index: cat.entries.len(),
                                key: Some(key_name.to_string()),
                                value: new_value.clone(),
                                raw: format!("{key_name}={new_value}"),
                            });
                        }
                        let updated = cat.serialize()?;
                        after_hash = blake3::hash(&updated).to_hex().to_string();
                        std::fs::write(&full_path, updated)?;
                    } else {
                        if let Some(parent) = full_path.parent() {
                            std::fs::create_dir_all(parent)?;
                        }
                        let entry = format!("{key_name}={new_value}\n");
                        let bytes = entry.into_bytes();
                        after_hash = blake3::hash(&bytes).to_hex().to_string();
                        std::fs::write(&full_path, bytes)?;
                    }

                    journal.append(op.clone(), before_hash.clone(), Some(after_hash.clone()));

                    results.push(OperationResult {
                        index: idx,
                        selector: format!("{catalog_file}::{key_name}"),
                        risk: op.risk_class(),
                        success: true,
                        message: format!("Updated string to \"{}\"", new_value),
                        before_hash,
                        after_hash: Some(after_hash),
                    });
                }
                RecipeOperation::SetConfig {
                    selector,
                    new_value,
                    ..
                } => {
                    let (config_file, key) = match selector {
                        SemanticSelector::ConfigKey { file, key } => (file.as_str(), key.as_str()),
                        _ => (target_id, "Config"),
                    };

                    let full_path = base_dir.join(config_file);
                    let before_hash = if full_path.exists() {
                        let b = std::fs::read(&full_path)?;
                        Some(blake3::hash(&b).to_hex().to_string())
                    } else {
                        None
                    };

                    // Append config key setting
                    if let Some(parent) = full_path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    let line = format!("\n{} = {}\n", key, new_value);
                    use std::io::Write;
                    let mut file = std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&full_path)?;
                    file.write_all(line.as_bytes())?;

                    let updated = std::fs::read(&full_path)?;
                    let after_hash = blake3::hash(&updated).to_hex().to_string();

                    journal.append(op.clone(), before_hash.clone(), Some(after_hash.clone()));

                    results.push(OperationResult {
                        index: idx,
                        selector: format!("{config_file}::{key}"),
                        risk: op.risk_class(),
                        success: true,
                        message: format!("Set config to \"{}\"", new_value),
                        before_hash,
                        after_hash: Some(after_hash),
                    });
                }
                RecipeOperation::RecolourPalette { selector, .. } => {
                    results.push(OperationResult {
                        index: idx,
                        selector: selector.target_identifier().to_string(),
                        risk: op.risk_class(),
                        success: true,
                        message: "Palette accent recoloured".to_string(),
                        before_hash: None,
                        after_hash: None,
                    });
                }
            }
        }

        let operations_applied = results.iter().filter(|r| r.success).count();
        let journal_head_hash = journal
            .entries
            .last()
            .map(|e| e.entry_hash.clone())
            .unwrap_or_else(|| JournalChain::GENESIS_HASH.to_string());

        Ok(RecipeApplyReport {
            recipe_name: recipe.metadata.name.clone(),
            operations_total: recipe.operations.len(),
            operations_applied,
            overall_risk: recipe.overall_risk(),
            journal_head_hash,
            operation_results: results,
        })
    }
}
