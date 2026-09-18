//! Recipe data structures, semantic selectors, and risk classification.
//!
//! Conforms to §13.1 - §13.3 of AUDI_MMI_STUDIO_AGENT_PROMPT.md:
//! - Recipes are the unit of work: declarative, portable, reproducible.
//! - Strictly semantic selectors: module_id, asset_id, string_key, config_key — never byte offsets.
//! - Risk classes: COSMETIC, CONTENT, STRUCTURAL.

use serde::{Deserialize, Serialize};

/// Risk classification for a recipe operation (§13.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskClass {
    /// Image / icon / colour replacement; no size or structural layout shift.
    Cosmetic,
    /// String and localisation modifications within container constraints.
    Content,
    /// Configuration, module layout, or metadata mutations affecting integrity.
    /// Requires explicit user confirmation and evidence citation. Blocked permanently on signed files.
    Structural,
}

impl std::fmt::Display for RiskClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cosmetic => write!(f, "COSMETIC"),
            Self::Content => write!(f, "CONTENT"),
            Self::Structural => write!(f, "STRUCTURAL"),
        }
    }
}

/// Semantic selector designating target entities without hardcoded byte offsets (§13.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "target")]
pub enum SemanticSelector {
    /// Selects an asset by logical path or relative bundle path (e.g. "CombiStyles/IND/0/default/arr_e.precomp")
    AssetPath(String),
    /// Selects a module package entry (e.g. "GEMMI/nav/0/default")
    ModuleId(String),
    /// Selects a localized string by key or text identifier
    StringKey { catalog: String, key: String },
    /// Selects a configuration field (e.g. "MetaInfo2/Version" or "Common/Release")
    ConfigKey { file: String, key: String },
}

impl SemanticSelector {
    /// Returns the primary path or identifier targeted by this selector.
    pub fn target_identifier(&self) -> &str {
        match self {
            Self::AssetPath(p) => p.as_str(),
            Self::ModuleId(m) => m.as_str(),
            Self::StringKey { catalog, .. } => catalog.as_str(),
            Self::ConfigKey { file, .. } => file.as_str(),
        }
    }
}

/// A discrete, declarative operation within a Recipe (§13.2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op")]
pub enum RecipeOperation {
    /// Replace an asset with a candidate file/blob
    #[serde(rename = "replace_asset")]
    ReplaceAsset {
        selector: SemanticSelector,
        replacement_path: String,
        #[serde(default)]
        evidence_tag: Option<String>,
    },
    /// AI-generated asset replacement pinned to CAS with prompt provenance
    #[serde(rename = "generate_asset")]
    GenerateAsset {
        selector: SemanticSelector,
        prompt: String,
        pinned_blob_id: String,
        #[serde(default)]
        evidence_tag: Option<String>,
    },
    /// UI palette recolouring operation
    #[serde(rename = "recolour_palette")]
    RecolourPalette {
        selector: SemanticSelector,
        day_accent_hex: Option<String>,
        night_accent_hex: Option<String>,
        #[serde(default)]
        evidence_tag: Option<String>,
    },
    /// Localized string table mutation
    #[serde(rename = "set_string")]
    SetString {
        selector: SemanticSelector,
        new_value: String,
        #[serde(default)]
        evidence_tag: Option<String>,
    },
    /// MetaInfo2 or configuration property mutation
    #[serde(rename = "set_config")]
    SetConfig {
        selector: SemanticSelector,
        new_value: String,
        #[serde(default)]
        evidence_tag: Option<String>,
    },
}

impl RecipeOperation {
    /// Evaluates the intrinsic risk level of this operation.
    pub fn risk_class(&self) -> RiskClass {
        match self {
            Self::ReplaceAsset { .. } | Self::GenerateAsset { .. } | Self::RecolourPalette { .. } => {
                RiskClass::Cosmetic
            }
            Self::SetString { .. } => RiskClass::Content,
            Self::SetConfig { .. } => RiskClass::Structural,
        }
    }

    /// Gets the semantic selector targeted by this operation.
    pub fn selector(&self) -> &SemanticSelector {
        match self {
            Self::ReplaceAsset { selector, .. }
            | Self::GenerateAsset { selector, .. }
            | Self::RecolourPalette { selector, .. }
            | Self::SetString { selector, .. }
            | Self::SetConfig { selector, .. } => selector,
        }
    }

    /// Evidence tag associated with this operation, if any.
    pub fn evidence_tag(&self) -> Option<&str> {
        match self {
            Self::ReplaceAsset { evidence_tag, .. }
            | Self::GenerateAsset { evidence_tag, .. }
            | Self::RecolourPalette { evidence_tag, .. }
            | Self::SetString { evidence_tag, .. }
            | Self::SetConfig { evidence_tag, .. } => evidence_tag.as_deref(),
        }
    }
}

/// Metadata header for a Recipe document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeMetadata {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: Option<String>,
    pub base_train: String,
    #[serde(default)]
    pub target_trains: Vec<String>,
    pub created_at: String,
}

/// A complete declarative modification Recipe (§13.1).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recipe {
    pub api_version: String,
    pub metadata: RecipeMetadata,
    pub operations: Vec<RecipeOperation>,
}

impl Recipe {
    /// Parses a recipe from JSON text.
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    /// Serializes recipe to standard pretty JSON text.
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Calculates highest risk class among all included operations.
    pub fn overall_risk(&self) -> RiskClass {
        self.operations
            .iter()
            .map(|op| op.risk_class())
            .max()
            .unwrap_or(RiskClass::Cosmetic)
    }
}
