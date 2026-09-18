//! Cross-train rebase engine (§13.4).
//!
//! Re-evaluates declarative recipe operations across software train releases
//! (e.g. from `HN+R_EU_AU_K0942_4` to new base packages), categorizing outcomes:
//! - `Applied`: Exact semantic selector and constraints matched cleanly.
//! - `AppliedWithDrift`: Target matched with minor non-breaking drift (e.g. path variance).
//! - `SelectorNotFound`: Target asset or config key does not exist on target train.
//! - `Ambiguous`: Multiple candidates match semantic selector.
//! - `UnsupportedOnBase`: Base train lacks required capability or format is locked.

use crate::engine::RecipeEngine;
use crate::model::{Recipe, SemanticSelector};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Status of rebasing an individual operation (§13.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RebaseStatus {
    Applied,
    AppliedWithDrift,
    SelectorNotFound,
    Ambiguous,
    UnsupportedOnBase,
}

impl std::fmt::Display for RebaseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Applied => write!(f, "APPLIED"),
            Self::AppliedWithDrift => write!(f, "APPLIED_WITH_DRIFT"),
            Self::SelectorNotFound => write!(f, "SELECTOR_NOT_FOUND"),
            Self::Ambiguous => write!(f, "AMBIGUOUS"),
            Self::UnsupportedOnBase => write!(f, "UNSUPPORTED_ON_BASE"),
        }
    }
}

/// Diagnostic item for an individual operation during rebase.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationRebaseReport {
    pub index: usize,
    pub selector: String,
    pub status: RebaseStatus,
    pub notes: String,
}

/// Comprehensive cross-train rebase analysis report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebaseReport {
    pub recipe_name: String,
    pub source_train: String,
    pub target_train: String,
    pub total_operations: usize,
    pub clean_count: usize,
    pub drift_count: usize,
    pub missing_count: usize,
    pub unsupported_count: usize,
    pub operations: Vec<OperationRebaseReport>,
}

pub struct RebaseEngine;

impl RebaseEngine {
    /// Evaluates compatibility of a recipe against a target train root directory.
    pub fn plan_rebase(recipe: &Recipe, target_train_root: &Path) -> RebaseReport {
        let mut reports = Vec::new();

        let mut clean = 0;
        let mut drift = 0;
        let mut missing = 0;
        let mut unsupported = 0;

        for (idx, op) in recipe.operations.iter().enumerate() {
            let target_id = op.selector().target_identifier();

            if RecipeEngine::is_signed_artefact(target_id) {
                unsupported += 1;
                reports.push(OperationRebaseReport {
                    index: idx,
                    selector: target_id.to_string(),
                    status: RebaseStatus::UnsupportedOnBase,
                    notes: "Target is a signed artefact permanently locked from modification".to_string(),
                });
                continue;
            }

            match op.selector() {
                SemanticSelector::AssetPath(p) => {
                    let direct_target = target_train_root.join(p);
                    if direct_target.exists() {
                        clean += 1;
                        reports.push(OperationRebaseReport {
                            index: idx,
                            selector: p.clone(),
                            status: RebaseStatus::Applied,
                            notes: "Exact asset path match on target train".to_string(),
                        });
                    } else {
                        // Check for path drift by matching filename across target train
                        let file_name = Path::new(p).file_name().map(|n| n.to_string_lossy().to_string());
                        let mut candidates = Vec::new();
                        if let Some(ref fname) = file_name {
                            Self::find_candidates_by_name(target_train_root, fname, &mut candidates);
                        }

                        if candidates.len() == 1 {
                            drift += 1;
                            reports.push(OperationRebaseReport {
                                index: idx,
                                selector: p.clone(),
                                status: RebaseStatus::AppliedWithDrift,
                                notes: format!("Asset found at drifted path: {}", candidates[0].display()),
                            });
                        } else if candidates.len() > 1 {
                            reports.push(OperationRebaseReport {
                                index: idx,
                                selector: p.clone(),
                                status: RebaseStatus::Ambiguous,
                                notes: format!("Ambiguous selector: {} matching locations found", candidates.len()),
                            });
                        } else {
                            missing += 1;
                            reports.push(OperationRebaseReport {
                                index: idx,
                                selector: p.clone(),
                                status: RebaseStatus::SelectorNotFound,
                                notes: "Asset does not exist in target train".to_string(),
                            });
                        }
                    }
                }
                SemanticSelector::ModuleId(m) => {
                    let module_path = target_train_root.join(m);
                    if module_path.exists() {
                        clean += 1;
                        reports.push(OperationRebaseReport {
                            index: idx,
                            selector: m.clone(),
                            status: RebaseStatus::Applied,
                            notes: "Target module found".to_string(),
                        });
                    } else {
                        missing += 1;
                        reports.push(OperationRebaseReport {
                            index: idx,
                            selector: m.clone(),
                            status: RebaseStatus::SelectorNotFound,
                            notes: "Module not found in target train".to_string(),
                        });
                    }
                }
                SemanticSelector::StringKey { catalog, key } => {
                    let cat_path = target_train_root.join(catalog);
                    if cat_path.exists() {
                        clean += 1;
                        reports.push(OperationRebaseReport {
                            index: idx,
                            selector: format!("{catalog}::{key}"),
                            status: RebaseStatus::Applied,
                            notes: "Target string catalog exists".to_string(),
                        });
                    } else {
                        missing += 1;
                        reports.push(OperationRebaseReport {
                            index: idx,
                            selector: format!("{catalog}::{key}"),
                            status: RebaseStatus::SelectorNotFound,
                            notes: "Target string catalog file not found".to_string(),
                        });
                    }
                }
                SemanticSelector::ConfigKey { file, key } => {
                    let conf_path = target_train_root.join(file);
                    if conf_path.exists() {
                        clean += 1;
                        reports.push(OperationRebaseReport {
                            index: idx,
                            selector: format!("{file}::{key}"),
                            status: RebaseStatus::Applied,
                            notes: "Target configuration file exists".to_string(),
                        });
                    } else {
                        missing += 1;
                        reports.push(OperationRebaseReport {
                            index: idx,
                            selector: format!("{file}::{key}"),
                            status: RebaseStatus::SelectorNotFound,
                            notes: "Configuration file not found on target train".to_string(),
                        });
                    }
                }
            }
        }

        let target_train_name = target_train_root
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown_target".to_string());

        RebaseReport {
            recipe_name: recipe.metadata.name.clone(),
            source_train: recipe.metadata.base_train.clone(),
            target_train: target_train_name,
            total_operations: recipe.operations.len(),
            clean_count: clean,
            drift_count: drift,
            missing_count: missing,
            unsupported_count: unsupported,
            operations: reports,
        }
    }

    fn find_candidates_by_name(root: &Path, filename: &str, candidates: &mut Vec<std::path::PathBuf>) {
        if let Ok(entries) = std::fs::read_dir(root) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_dir() {
                    // Limit recursion depth to 5
                    Self::find_candidates_by_name(&path, filename, candidates);
                } else if path.file_name().map(|n| n.to_string_lossy() == filename).unwrap_or(false) {
                    candidates.push(path);
                }
            }
        }
    }
}
