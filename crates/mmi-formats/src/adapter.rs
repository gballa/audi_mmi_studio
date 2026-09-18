//! FormatAdapter trait and dynamic capability model.

use serde::{Deserialize, Serialize};

/// Dynamic capabilities derived strictly from verification tests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormatCapabilities {
    pub can_analyse: bool,
    pub can_extract: bool,
    pub can_normalise: bool,
    pub can_edit: bool,
    pub can_rebuild: bool,
    pub can_validate: bool,
    pub can_package: bool,
}

impl FormatCapabilities {
    /// Returns read-only capabilities (analysis, extraction, normalization).
    pub fn read_only() -> Self {
        Self {
            can_analyse: true,
            can_extract: true,
            can_normalise: true,
            can_edit: false,
            can_rebuild: false,
            can_validate: true,
            can_package: false,
        }
    }

    /// Returns full capabilities once identity-rebuild gate has passed.
    pub fn full_support() -> Self {
        Self {
            can_analyse: true,
            can_extract: true,
            can_normalise: true,
            can_edit: true,
            can_rebuild: true,
            can_validate: true,
            can_package: true,
        }
    }
}

/// Trait implemented by all binary format adapters in Audi MMI Studio.
pub trait FormatAdapter: Send + Sync {
    /// Identifier of the format (e.g. "metainfo2", "precomp", "mapstyle_xar").
    fn format_name(&self) -> &'static str;

    /// Returns the verified capabilities of this adapter.
    fn capabilities(&self) -> &FormatCapabilities;

    /// Checks whether the supplied data conforms to this format's signature.
    fn detect(&self, data: &[u8]) -> bool;

    /// Calculates the named bytes to total bytes ratio (coverage ratio).
    fn coverage_ratio(&self, data: &[u8]) -> f32;
}
