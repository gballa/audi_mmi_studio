//! Harman Becker Orion Atlas Spatial Tile Container parser (§5, RQ-002).

use crate::adapter::{FormatAdapter, FormatCapabilities};
use mmi_core::CoreError;
use serde::{Deserialize, Serialize};

pub const ATLAS_MAGIC_TAG: &[u8; 7] = b"\x06HEADER";
pub const ATLAS_HEADER_MIN_SIZE: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HbAtlasHeader {
    pub tile_block_size: u32,
    pub version_major: u16,
    pub version_minor: u16,
    pub index_offset: u32,
    pub index_size: u32,
    pub project_name: String,
    pub container_type: String,
}

#[derive(Debug, Clone)]
pub struct HbAtlas {
    pub header: HbAtlasHeader,
    pub total_size: usize,
}

impl HbAtlas {
    pub fn parse(data: &[u8]) -> Result<Self, CoreError> {
        if data.len() < ATLAS_HEADER_MIN_SIZE {
            return Err(CoreError::ImmutabilityViolation(
                "HbAtlas file too small for 64-byte header".into(),
            ));
        }

        if &data[0..7] != ATLAS_MAGIC_TAG {
            return Err(CoreError::ImmutabilityViolation(
                "Invalid HbAtlas magic signature: expected \\x06HEADER".into(),
            ));
        }

        let tile_block_size = u32::from_le_bytes([data[0x10], data[0x11], data[0x12], data[0x13]]);
        let version_major = u16::from_le_bytes([data[0x14], data[0x15]]);
        let version_minor = u16::from_le_bytes([data[0x16], data[0x17]]);
        let index_offset = u32::from_le_bytes([data[0x18], data[0x19], data[0x1a], data[0x1b]]);
        let index_size = u32::from_le_bytes([data[0x1c], data[0x1d], data[0x1e], data[0x1f]]);

        // Pascal strings at 0x20 and 0x30
        let orion_len = data[0x20] as usize;
        let project_name = if 0x21 + orion_len <= data.len() {
            String::from_utf8_lossy(&data[0x21..0x21 + orion_len]).to_string()
        } else {
            "Unknown".to_string()
        };

        let atlas_len = data[0x30] as usize;
        let container_type = if 0x31 + atlas_len <= data.len() {
            String::from_utf8_lossy(&data[0x31..0x31 + atlas_len]).to_string()
        } else {
            "Unknown".to_string()
        };

        Ok(Self {
            header: HbAtlasHeader {
                tile_block_size,
                version_major,
                version_minor,
                index_offset,
                index_size,
                project_name,
                container_type,
            },
            total_size: data.len(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct HbAtlasAdapter {
    capabilities: FormatCapabilities,
}

impl Default for HbAtlasAdapter {
    fn default() -> Self {
        Self {
            capabilities: FormatCapabilities::read_only(),
        }
    }
}

impl FormatAdapter for HbAtlasAdapter {
    fn format_name(&self) -> &'static str {
        "hb_atlas"
    }

    fn capabilities(&self) -> &FormatCapabilities {
        &self.capabilities
    }

    fn detect(&self, data: &[u8]) -> bool {
        if data.len() < ATLAS_HEADER_MIN_SIZE {
            return false;
        }
        &data[0..7] == ATLAS_MAGIC_TAG
            && data.len() > 0x35
            && &data[0x21..0x26] == b"Orion"
            && &data[0x31..0x36] == b"Atlas"
    }

    fn coverage_ratio(&self, data: &[u8]) -> f32 {
        if self.detect(data) {
            1.0
        } else {
            0.0
        }
    }
}
