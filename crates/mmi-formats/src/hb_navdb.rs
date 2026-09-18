//! Harman Becker Navigation Fast Lookup Database (FLDB) parser (§5, RQ-001).

use crate::adapter::{FormatAdapter, FormatCapabilities};
use mmi_core::CoreError;
use serde::{Deserialize, Serialize};

pub const FLDB_MAGIC: &[u8; 4] = b"FLDB";
pub const NAVDB_HEADER_SIZE: usize = 36;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HbNavDbHeader {
    pub page_size: u32,
    pub root_page: u32,
    pub timestamp: u32,
    pub version: u32,
    pub header_size: u32,
    pub magic: [u8; 4],
}

#[derive(Debug, Clone)]
pub struct HbNavDb {
    pub header: HbNavDbHeader,
    pub total_size: usize,
}

impl HbNavDb {
    pub fn parse(data: &[u8]) -> Result<Self, CoreError> {
        if data.len() < NAVDB_HEADER_SIZE {
            return Err(CoreError::ImmutabilityViolation(
                "HbNavDb file too small for 36-byte header".into(),
            ));
        }

        let page_size = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        let root_page = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        let timestamp = u32::from_le_bytes([data[8], data[9], data[10], data[11]]);
        let version = u32::from_le_bytes([data[12], data[13], data[14], data[15]]);
        let header_size = u32::from_le_bytes([data[16], data[17], data[18], data[19]]);

        let mut magic = [0u8; 4];
        magic.copy_from_slice(&data[20..24]);

        if &magic != FLDB_MAGIC {
            return Err(CoreError::ImmutabilityViolation(format!(
                "Invalid HbNavDb magic signature: expected FLDB, found {:?}",
                String::from_utf8_lossy(&magic)
            )));
        }

        Ok(Self {
            header: HbNavDbHeader {
                page_size,
                root_page,
                timestamp,
                version,
                header_size,
                magic,
            },
            total_size: data.len(),
        })
    }

    /// Estimated page count based on file size and page size.
    pub fn page_count(&self) -> usize {
        if self.header.page_size == 0 {
            0
        } else {
            self.total_size / (self.header.page_size as usize)
        }
    }
}

#[derive(Debug, Clone)]
pub struct HbNavDbAdapter {
    capabilities: FormatCapabilities,
}

impl Default for HbNavDbAdapter {
    fn default() -> Self {
        Self {
            capabilities: FormatCapabilities::read_only(),
        }
    }
}

impl FormatAdapter for HbNavDbAdapter {
    fn format_name(&self) -> &'static str {
        "hb_navdb"
    }

    fn capabilities(&self) -> &FormatCapabilities {
        &self.capabilities
    }

    fn detect(&self, data: &[u8]) -> bool {
        if data.len() < NAVDB_HEADER_SIZE {
            return false;
        }
        &data[20..24] == FLDB_MAGIC
    }

    fn coverage_ratio(&self, data: &[u8]) -> f32 {
        if self.detect(data) {
            1.0
        } else {
            0.0
        }
    }
}
