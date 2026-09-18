//! MMIProject: Document model representing normalized MMI firmware and media projects.

use serde::{Deserialize, Serialize};

/// Confidence grading for all project model inferences and extractions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConfidenceLevel {
    Known,
    Inferred,
    Experimental,
    Unknown,
}

/// A value paired with an explicit confidence level and evidence basis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaggedValue<T> {
    pub value: T,
    pub confidence: ConfidenceLevel,
    pub evidence: String,
}

impl<T> TaggedValue<T> {
    pub fn known(value: T, evidence: impl Into<String>) -> Self {
        Self {
            value,
            confidence: ConfidenceLevel::Known,
            evidence: evidence.into(),
        }
    }

    pub fn inferred(value: T, evidence: impl Into<String>) -> Self {
        Self {
            value,
            confidence: ConfidenceLevel::Inferred,
            evidence: evidence.into(),
        }
    }

    pub fn experimental(value: T, evidence: impl Into<String>) -> Self {
        Self {
            value,
            confidence: ConfidenceLevel::Experimental,
            evidence: evidence.into(),
        }
    }

    pub fn unknown(value: T, evidence: impl Into<String>) -> Self {
        Self {
            value,
            confidence: ConfidenceLevel::Unknown,
            evidence: evidence.into(),
        }
    }
}

/// Discovered MMI hardware and software platform variant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlatformVariant {
    HnPlusR,
    HnPlus,
    Hn,
    Unknown(String),
}

/// Project metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub version: String,
    pub source_evidence: String,
}

/// Reference to a verified source artifact in `originals/`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceRef {
    pub logical_path: String,
    pub blake3_hex: String,
    pub sha256_hex: String,
    pub byte_size: u64,
    pub is_signed: bool,
}

/// Target hardware/firmware execution profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetProfile {
    pub profile_id: String,
    pub display_resolution: (u32, u32),
    pub max_asset_bytes: u64,
    pub supported_codecs: Vec<String>,
}

impl Default for TargetProfile {
    fn default() -> Self {
        Self {
            profile_id: "AUDI_HN_PLUS_R_EU_C7".to_string(),
            display_resolution: (800, 480),
            max_asset_bytes: 4 * 1024 * 1024,
            supported_codecs: vec!["precomp".to_string(), "png".to_string()],
        }
    }
}

/// Discovered module record (e.g. GEMMI, RSU, DU, ARU).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleRecord {
    pub name: String,
    pub source_folder: String,
    pub version: Option<String>,
    pub file_count: usize,
    pub total_bytes: u64,
    pub is_signed: bool,
}

/// Asset metadata descriptor for visual and font elements.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetDescriptor {
    pub asset_id: String,
    pub logical_path: String,
    pub module: String,
    pub blob_id: String,
    pub format: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub byte_size: u64,
    pub can_edit: bool,
    pub can_rebuild: bool,
}

/// Record of an immutable, cryptographically signed payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedArtefactRecord {
    pub path: String,
    pub signature_path: Option<String>,
    pub signature_type: String,
    pub blake3_hex: String,
    pub sha256_hex: String,
    pub can_edit: bool,
    pub can_rebuild: bool,
    pub policy: String,
}

/// Opaque span reference representing unparsed byte regions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpaqueSpanRef {
    pub container_blob_id: String,
    pub offset: u64,
    pub length: u64,
    pub rationale: String,
}

/// Top-level normalized document model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MMIProject {
    pub metadata: ProjectMetadata,
    pub source_refs: Vec<SourceRef>,
    pub target_profile: TargetProfile,
    pub detected_platform: TaggedValue<PlatformVariant>,
    pub software_train: Option<TaggedValue<String>>,
    pub map_version: Option<TaggedValue<String>>,
    pub modules: Vec<ModuleRecord>,
    pub assets: Vec<AssetDescriptor>,
    pub opaque_spans: Vec<OpaqueSpanRef>,
    pub signed_artefacts: Vec<SignedArtefactRecord>,
    pub validation_issues: Vec<String>,
}

impl MMIProject {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            metadata: ProjectMetadata {
                id: id.into(),
                name: name.into(),
                created_at: "2026-09-18T16:31:00Z".to_string(),
                version: "0.1.0".to_string(),
                source_evidence: "Pass A & Pass B Corpus Audit".to_string(),
            },
            source_refs: Vec::new(),
            target_profile: TargetProfile::default(),
            detected_platform: TaggedValue::unknown(
                PlatformVariant::Unknown("Undetected".to_string()),
                "[UNK RQ-001]",
            ),
            software_train: None,
            map_version: None,
            modules: Vec::new(),
            assets: Vec::new(),
            opaque_spans: Vec::new(),
            signed_artefacts: Vec::new(),
            validation_issues: Vec::new(),
        }
    }

    /// Checks if a file path belongs to a signed payload.
    pub fn is_signed_payload(path: &str) -> bool {
        path.ends_with(".pkg") || path.ends_with(".sig") || path.ends_with("TMCConfig.dat")
    }

    /// Registers a signed payload into the project, enforcing immutable analysis-only status.
    pub fn register_signed_artefact(
        &mut self,
        path: impl Into<String>,
        sig_path: Option<String>,
        blake3_hex: impl Into<String>,
        sha256_hex: impl Into<String>,
    ) {
        let p = path.into();
        self.signed_artefacts.push(SignedArtefactRecord {
            path: p,
            signature_path: sig_path,
            signature_type: "RSA/Detached".to_string(),
            blake3_hex: blake3_hex.into(),
            sha256_hex: sha256_hex.into(),
            can_edit: false,
            can_rebuild: false,
            policy: "ANALYSIS-ONLY: Signed artefact is permanently immutable".to_string(),
        });
    }
}
