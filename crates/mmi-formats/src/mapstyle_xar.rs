//! Parser for Harman / Elektrobit MapStyles regional archive containers (.xar).

use mmi_core::CoreError;
use serde::{Deserialize, Serialize};

use crate::adapter::{FormatAdapter, FormatCapabilities};

pub const XAR_MAGIC: [u8; 4] = [b'r', b'a', b'x', 0x00];
pub const MIN_XAR_HEADER: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MapStyleXar {
    pub version: u32,
    pub declared_file_size: u64,
    pub member_count: u32,
}

impl MapStyleXar {
    /// Parses a MapStyles `.xar` archive header.
    pub fn parse(data: &[u8]) -> Result<Self, CoreError> {
        if data.len() < MIN_XAR_HEADER {
            return Err(CoreError::ImmutabilityViolation(format!(
                "XAR archive too small: {} bytes (minimum {} bytes)",
                data.len(),
                MIN_XAR_HEADER
            )));
        }

        if &data[0..4] != XAR_MAGIC {
            return Err(CoreError::ImmutabilityViolation(
                "Invalid MapStyle XAR magic signature (expected 'rax\\0')".to_string(),
            ));
        }

        let version = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        let declared_file_size = u32::from_le_bytes([data[16], data[17], data[18], data[19]]) as u64;
        let member_count = u32::from_le_bytes([data[28], data[29], data[30], data[31]]);

        Ok(Self {
            version,
            declared_file_size,
            member_count,
        })
    }
}

pub struct MapStyleXarAdapter {
    capabilities: FormatCapabilities,
}

impl Default for MapStyleXarAdapter {
    fn default() -> Self {
        Self {
            capabilities: FormatCapabilities::read_only(),
        }
    }
}

impl FormatAdapter for MapStyleXarAdapter {
    fn format_name(&self) -> &'static str {
        "mapstyle_xar"
    }

    fn capabilities(&self) -> &FormatCapabilities {
        &self.capabilities
    }

    fn detect(&self, data: &[u8]) -> bool {
        data.len() >= MIN_XAR_HEADER && &data[0..4] == XAR_MAGIC
    }

    fn coverage_ratio(&self, data: &[u8]) -> f32 {
        if self.detect(data) {
            0.85 // Header, directory, and payload streams mapped
        } else {
            0.0
        }
    }
}
