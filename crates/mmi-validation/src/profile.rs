//! Target hardware profiles and vehicle configuration declarations (§14.2 - §14.3).

use serde::{Deserialize, Serialize};

/// Supported Audi MMI hardware generations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MmiGeneration {
    Mmi3GHigh,
    Mmi3GPlus,
    Mib1,
    Mib2High,
}

impl std::fmt::Display for MmiGeneration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mmi3GHigh => write!(f, "MMI 3G High (HNav)"),
            Self::Mmi3GPlus => write!(f, "MMI 3G Plus (HN+)"),
            Self::Mib1 => write!(f, "MIB1 High"),
            Self::Mib2High => write!(f, "MIB2 High (MHI2)"),
        }
    }
}

/// Target vehicle head-unit deployment profile (§14.2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetProfile {
    pub name: String,
    pub generation: MmiGeneration,
    pub head_unit_part_number: String,
    pub current_software_train: String,
    pub target_software_train: String,
    pub hardware_revisions: Vec<String>,
    pub region: String,
    pub display_resolution: (u32, u32),
    pub user_verified: bool,
}

impl Default for TargetProfile {
    fn default() -> Self {
        Self {
            name: "Default Audi A6/A7 MMI 3G Plus (C7 EU)".to_string(),
            generation: MmiGeneration::Mmi3GPlus,
            head_unit_part_number: "4G0035670".to_string(),
            current_software_train: "HN+R_EU_AU_K0942_4".to_string(),
            target_software_train: "HN+R_EU_AU_K0942_4".to_string(),
            hardware_revisions: vec!["41".to_string(), "51".to_string(), "61".to_string(), "7".to_string()],
            region: "EU".to_string(),
            display_resolution: (800, 480),
            user_verified: true,
        }
    }
}

impl TargetProfile {
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}
