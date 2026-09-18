//! Mandatory status vocabulary and safety phrase linter (§14.9).
//!
//! Enforces exact verbatim terminology across the workstation:
//! VERIFIED · SUPPORTED · PARTIALLY SUPPORTED · EXPERIMENTAL · UNSUPPORTED ·
//! UNKNOWN · RESEARCH REQUIRED · BUILD READY — DEPLOYMENT NOT VERIFIED ·
//! SIMULATED — NOT A GUARANTEE · PROTECTED / OUT OF SCOPE — DOCUMENT ONLY ·
//! HIGH RISK — NO VERIFIED RECOVERY PATH
//!
//! SAFETY RULE: The phrase "SAFE TO INSTALL" must NEVER be rendered under any condition.

use mmi_core::CoreError;
use serde::{Deserialize, Serialize};

/// Canonical verbatim safety status vocabulary (§14.9).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusVocabulary {
    Verified,
    Supported,
    PartiallySupported,
    Experimental,
    Unsupported,
    Unknown,
    ResearchRequired,
    BuildReadyDeploymentNotVerified,
    SimulatedNotAGuarantee,
    ProtectedOutOfScopeDocumentOnly,
    HighRiskNoVerifiedRecoveryPath,
}

impl Serialize for StatusVocabulary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for StatusVocabulary {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "VERIFIED" => Ok(Self::Verified),
            "SUPPORTED" => Ok(Self::Supported),
            "PARTIALLY SUPPORTED" => Ok(Self::PartiallySupported),
            "EXPERIMENTAL" => Ok(Self::Experimental),
            "UNSUPPORTED" => Ok(Self::Unsupported),
            "UNKNOWN" => Ok(Self::Unknown),
            "RESEARCH REQUIRED" => Ok(Self::ResearchRequired),
            "BUILD READY — DEPLOYMENT NOT VERIFIED" => Ok(Self::BuildReadyDeploymentNotVerified),
            "SIMULATED — NOT A GUARANTEE" => Ok(Self::SimulatedNotAGuarantee),
            "PROTECTED / OUT OF SCOPE — DOCUMENT ONLY" => Ok(Self::ProtectedOutOfScopeDocumentOnly),
            "HIGH RISK — NO VERIFIED RECOVERY PATH" => Ok(Self::HighRiskNoVerifiedRecoveryPath),
            other => Err(serde::de::Error::custom(format!("Unknown status vocabulary term: {other}"))),
        }
    }
}

impl std::fmt::Display for StatusVocabulary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Verified => write!(f, "VERIFIED"),
            Self::Supported => write!(f, "SUPPORTED"),
            Self::PartiallySupported => write!(f, "PARTIALLY SUPPORTED"),
            Self::Experimental => write!(f, "EXPERIMENTAL"),
            Self::Unsupported => write!(f, "UNSUPPORTED"),
            Self::Unknown => write!(f, "UNKNOWN"),
            Self::ResearchRequired => write!(f, "RESEARCH REQUIRED"),
            Self::BuildReadyDeploymentNotVerified => write!(f, "BUILD READY — DEPLOYMENT NOT VERIFIED"),
            Self::SimulatedNotAGuarantee => write!(f, "SIMULATED — NOT A GUARANTEE"),
            Self::ProtectedOutOfScopeDocumentOnly => write!(f, "PROTECTED / OUT OF SCOPE — DOCUMENT ONLY"),
            Self::HighRiskNoVerifiedRecoveryPath => write!(f, "HIGH RISK — NO VERIFIED RECOVERY PATH"),
        }
    }
}

pub struct SafetyLinter;

impl SafetyLinter {
    pub const FORBIDDEN_PHRASE: &'static str = "SAFE TO INSTALL";

    /// Scans arbitrary text output and returns an error if the forbidden phrase is present (§14.9).
    pub fn assert_no_forbidden_phrase(text: &str) -> Result<(), CoreError> {
        let normalized = text.to_uppercase();
        if normalized.contains(Self::FORBIDDEN_PHRASE) {
            Err(CoreError::ImmutabilityViolation(format!(
                "SAFETY VIOLATION: The phrase '{}' is permanently forbidden from all workstation outputs (§14.9)",
                Self::FORBIDDEN_PHRASE
            )))
        } else {
            Ok(())
        }
    }
}
