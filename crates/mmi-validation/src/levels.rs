//! 6-tier validation levels, severity types, and findings (§14.1).

use serde::{Deserialize, Serialize};

/// 6-tier validation hierarchy levels (§14.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ValidationLevel {
    /// L0: Identity-rebuild status for every format in the bundle
    L0FormatRebuild,
    /// L1: File-level header, magic numbers, checksums, and size bounds
    L1FileStructure,
    /// L2: Resource-level constraints (dimensions, bit depth, font tables, text encoding)
    L2ResourceConformance,
    /// L3: Module-level internal directory structures and package declarations
    L3ModuleIntegrity,
    /// L4: Bundle-level metainfo2.txt, package dependencies, checksum manifests
    L4BundleIntegrity,
    /// L5: Deployment compatibility against target hardware profile
    L5DeploymentCompatibility,
}

impl std::fmt::Display for ValidationLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::L0FormatRebuild => write!(f, "L0: Format Rebuild Status"),
            Self::L1FileStructure => write!(f, "L1: File Structure & Magic"),
            Self::L2ResourceConformance => write!(f, "L2: Resource Conformance"),
            Self::L3ModuleIntegrity => write!(f, "L3: Module Integrity"),
            Self::L4BundleIntegrity => write!(f, "L4: Bundle & MetaInfo2"),
            Self::L5DeploymentCompatibility => write!(f, "L5: Deployment Compatibility"),
        }
    }
}

/// Finding severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FindingSeverity {
    Info,
    Warning,
    Error,
}

impl std::fmt::Display for FindingSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Info => write!(f, "INFO"),
            Self::Warning => write!(f, "WARNING"),
            Self::Error => write!(f, "ERROR"),
        }
    }
}

/// A specific finding emitted during validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFinding {
    pub level: ValidationLevel,
    pub severity: FindingSeverity,
    pub target: String,
    pub code: String,
    pub message: String,
    pub evidence_tag: Option<String>,
}
