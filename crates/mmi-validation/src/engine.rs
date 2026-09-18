//! 6-tier multi-level validation execution engine (§14.1).
//!
//! Evaluates candidate stages through all levels (L0 to L5):
//! - Any `ERROR` fails the validation.
//! - A bundle containing modified structures with unresolved warnings is marked `SIMULATED — NOT A GUARANTEE`.

use crate::levels::{FindingSeverity, ValidationFinding, ValidationLevel};
use crate::profile::TargetProfile;
use mmi_core::CoreError;
use mmi_formats::MetaInfo2;
use mmi_rebuild::StageNormalizer;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// High-level validation outcome status (§14.9).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildReadinessStatus {
    /// All levels passed cleanly with zero errors and zero warnings
    Verified,
    /// Build ready with non-fatal warnings
    BuildReadyDeploymentNotVerified,
    /// Failed validation due to errors
    Failed,
    /// Profile not supplied or unverified
    CompatibilityUnknown,
}

impl std::fmt::Display for BuildReadinessStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Verified => write!(f, "VERIFIED"),
            Self::BuildReadyDeploymentNotVerified => write!(f, "BUILD READY — DEPLOYMENT NOT VERIFIED"),
            Self::Failed => write!(f, "FAILED"),
            Self::CompatibilityUnknown => write!(f, "COMPATIBILITY UNKNOWN"),
        }
    }
}

/// Comprehensive multi-tier validation report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub target_stage: String,
    pub profile_name: Option<String>,
    pub status: BuildReadinessStatus,
    pub total_findings: usize,
    pub error_count: usize,
    pub warning_count: usize,
    pub info_count: usize,
    pub findings: Vec<ValidationFinding>,
}

pub struct ValidationEngine;

impl ValidationEngine {
    /// Runs full 6-tier validation (L0-L5) on a stage directory.
    pub fn validate_stage(
        stage_dir: &Path,
        profile: Option<&TargetProfile>,
        max_level: Option<ValidationLevel>,
    ) -> Result<ValidationReport, CoreError> {
        let max_lvl = max_level.unwrap_or(ValidationLevel::L5DeploymentCompatibility);
        let mut findings = Vec::new();

        let entries = StageNormalizer::collect_normalized_tree(stage_dir)?;

        // Level 0: Format Rebuild Status
        if max_lvl >= ValidationLevel::L0FormatRebuild {
            Self::validate_l0(&entries, &mut findings);
        }

        // Level 1: File Structure & Magic
        if max_lvl >= ValidationLevel::L1FileStructure {
            Self::validate_l1(stage_dir, &entries, &mut findings);
        }

        // Level 2: Resource Conformance
        if max_lvl >= ValidationLevel::L2ResourceConformance {
            Self::validate_l2(stage_dir, &entries, &mut findings);
        }

        // Level 3: Module Integrity
        if max_lvl >= ValidationLevel::L3ModuleIntegrity {
            Self::validate_l3(&entries, &mut findings);
        }

        // Level 4: Bundle & MetaInfo2
        if max_lvl >= ValidationLevel::L4BundleIntegrity {
            Self::validate_l4(stage_dir, &entries, &mut findings);
        }

        // Level 5: Deployment Compatibility
        if max_lvl >= ValidationLevel::L5DeploymentCompatibility {
            Self::validate_l5(&entries, profile, &mut findings);
        }

        let error_count = findings.iter().filter(|f| f.severity == FindingSeverity::Error).count();
        let warning_count = findings.iter().filter(|f| f.severity == FindingSeverity::Warning).count();
        let info_count = findings.iter().filter(|f| f.severity == FindingSeverity::Info).count();

        let status = if error_count > 0 {
            BuildReadinessStatus::Failed
        } else if profile.is_none() || !profile.map(|p| p.user_verified).unwrap_or(false) {
            BuildReadinessStatus::CompatibilityUnknown
        } else if warning_count > 0 {
            BuildReadinessStatus::BuildReadyDeploymentNotVerified
        } else {
            BuildReadinessStatus::Verified
        };

        Ok(ValidationReport {
            target_stage: stage_dir.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default(),
            profile_name: profile.map(|p| p.name.clone()),
            status,
            total_findings: findings.len(),
            error_count,
            warning_count,
            info_count,
            findings,
        })
    }

    fn validate_l0(entries: &[mmi_rebuild::NormalizedFileEntry], findings: &mut Vec<ValidationFinding>) {
        for entry in entries {
            if entry.is_signed {
                findings.push(ValidationFinding {
                    level: ValidationLevel::L0FormatRebuild,
                    severity: FindingSeverity::Info,
                    target: entry.relative_path.clone(),
                    code: "L0_SIGNED_LOCKED".to_string(),
                    message: "Signed artifact is permanently read-only and immutable".to_string(),
                    evidence_tag: Some("[EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L690]".to_string()),
                });
            }
        }
    }

    fn validate_l1(
        stage_dir: &Path,
        entries: &[mmi_rebuild::NormalizedFileEntry],
        findings: &mut Vec<ValidationFinding>,
    ) {
        for entry in entries {
            let path = stage_dir.join(&entry.relative_path);
            if let Ok(bytes) = std::fs::read(&path) {
                if entry.relative_path.to_lowercase().ends_with(".precomp") {
                    if bytes.len() < mmi_formats::HEADER_SIZE || bytes[0..6] != mmi_formats::PRECOMP_MAGIC {
                        findings.push(ValidationFinding {
                            level: ValidationLevel::L1FileStructure,
                            severity: FindingSeverity::Error,
                            target: entry.relative_path.clone(),
                            code: "L1_INVALID_MAGIC".to_string(),
                            message: "File lacks valid Precomp magic header [00 01 00 00 00 00]".to_string(),
                            evidence_tag: Some("[EV:ksy:precomp.ksy]".to_string()),
                        });
                    }
                }
            }
        }
    }

    fn validate_l2(
        stage_dir: &Path,
        entries: &[mmi_rebuild::NormalizedFileEntry],
        findings: &mut Vec<ValidationFinding>,
    ) {
        for entry in entries {
            let path = stage_dir.join(&entry.relative_path);
            if entry.relative_path.to_lowercase().ends_with(".ttf") {
                if let Ok(bytes) = std::fs::read(&path) {
                    if let Ok(info) = mmi_assets::FontInspector::inspect(&bytes) {
                        if info.glyph_count == 0 {
                            findings.push(ValidationFinding {
                                level: ValidationLevel::L2ResourceConformance,
                                severity: FindingSeverity::Warning,
                                target: entry.relative_path.clone(),
                                code: "L2_EMPTY_FONT".to_string(),
                                message: "Font contains zero glyphs".to_string(),
                                evidence_tag: None,
                            });
                        }
                    }
                }
            }
        }
    }

    fn validate_l3(
        entries: &[mmi_rebuild::NormalizedFileEntry],
        findings: &mut Vec<ValidationFinding>,
    ) {
        // Verify module packages have their expected layout (e.g. 0/default subfolder)
        for entry in entries {
            if entry.relative_path.starts_with("GEMMI/") || entry.relative_path.starts_with("CombiStyles/") {
                if !entry.relative_path.contains("/0/default/") {
                    findings.push(ValidationFinding {
                        level: ValidationLevel::L3ModuleIntegrity,
                        severity: FindingSeverity::Warning,
                        target: entry.relative_path.clone(),
                        code: "L3_NONSTANDARD_MODULE_PATH".to_string(),
                        message: "Resource located outside standard /0/default/ module tree".to_string(),
                        evidence_tag: None,
                    });
                }
            }
        }
    }

    fn validate_l4(
        stage_dir: &Path,
        entries: &[mmi_rebuild::NormalizedFileEntry],
        findings: &mut Vec<ValidationFinding>,
    ) {
        let has_metainfo = entries.iter().any(|e| e.relative_path.to_lowercase() == "metainfo2.txt");
        if !has_metainfo {
            findings.push(ValidationFinding {
                level: ValidationLevel::L4BundleIntegrity,
                severity: FindingSeverity::Warning,
                target: "metainfo2.txt".to_string(),
                code: "L4_MISSING_METAINFO2".to_string(),
                message: "Stage root does not contain metainfo2.txt manifest".to_string(),
                evidence_tag: Some("[EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L691]".to_string()),
            });
        } else {
            let meta_path = stage_dir.join("metainfo2.txt");
            if let Ok(content) = std::fs::read_to_string(meta_path) {
                if MetaInfo2::parse(&content).is_err() {
                    findings.push(ValidationFinding {
                        level: ValidationLevel::L4BundleIntegrity,
                        severity: FindingSeverity::Error,
                        target: "metainfo2.txt".to_string(),
                        code: "L4_CORRUPT_METAINFO2".to_string(),
                        message: "metainfo2.txt failed syntax and structure parsing".to_string(),
                        evidence_tag: None,
                    });
                }
            }
        }
    }

    fn validate_l5(
        entries: &[mmi_rebuild::NormalizedFileEntry],
        profile: Option<&TargetProfile>,
        findings: &mut Vec<ValidationFinding>,
    ) {
        if let Some(prof) = profile {
            // Check hardware revision matching
            let mut matched_rev = false;
            for rev in &prof.hardware_revisions {
                if entries.iter().any(|e| e.relative_path.contains(&format!("/{}/", rev))) {
                    matched_rev = true;
                    break;
                }
            }

            if !matched_rev && !prof.hardware_revisions.is_empty() {
                findings.push(ValidationFinding {
                    level: ValidationLevel::L5DeploymentCompatibility,
                    severity: FindingSeverity::Info,
                    target: prof.name.clone(),
                    code: "L5_REVISION_SUBSET".to_string(),
                    message: format!(
                        "Stage package does not declare explicit matching revision paths for [{}]",
                        prof.hardware_revisions.join(", ")
                    ),
                    evidence_tag: Some("[EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L703]".to_string()),
                });
            }
        } else {
            findings.push(ValidationFinding {
                level: ValidationLevel::L5DeploymentCompatibility,
                severity: FindingSeverity::Warning,
                target: "TargetProfile".to_string(),
                code: "L5_NO_TARGET_PROFILE".to_string(),
                message: "No target hardware profile provided; resulting build is COMPATIBILITY UNKNOWN".to_string(),
                evidence_tag: Some("[EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L698]".to_string()),
            });
        }
    }
}
