//! Harman Becker Geographic Routing Database (GDB) format parser (§5, RQ-010).

use crate::adapter::{FormatAdapter, FormatCapabilities};
use mmi_core::CoreError;
use serde::{Deserialize, Serialize};

pub const GDB_MAGIC: &[u8; 4] = &[0xde, 0xad, 0xbe, 0xef];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HbGdbHeader {
    pub magic: [u8; 4],
    pub version: u32,
    pub flags: u32,
}

#[derive(Debug, Clone)]
pub struct HbGdb {
    pub header: HbGdbHeader,
    pub total_size: usize,
}

impl HbGdb {
    pub fn parse(data: &[u8]) -> Result<Self, CoreError> {
        if data.len() < 12 {
            return Err(CoreError::ImmutabilityViolation(
                "GDB file too small for 12-byte header".into(),
            ));
        }

        if &data[0..4] != GDB_MAGIC {
            return Err(CoreError::ImmutabilityViolation(
                "Invalid GDB magic signature: expected 0xDEADBEEF".into(),
            ));
        }

        let version = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
        let flags = u32::from_be_bytes([data[8], data[9], data[10], data[11]]);

        Ok(Self {
            header: HbGdbHeader {
                magic: *GDB_MAGIC,
                version,
                flags,
            },
            total_size: data.len(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct HbGdbAdapter {
    capabilities: FormatCapabilities,
}

impl Default for HbGdbAdapter {
    fn default() -> Self {
        Self {
            capabilities: FormatCapabilities::read_only(),
        }
    }
}

impl FormatAdapter for HbGdbAdapter {
    fn format_name(&self) -> &'static str {
        "hb_gdb"
    }

    fn capabilities(&self) -> &FormatCapabilities {
        &self.capabilities
    }

    fn detect(&self, data: &[u8]) -> bool {
        if data.len() < 4 {
            return false;
        }
        &data[0..4] == GDB_MAGIC
    }

    fn coverage_ratio(&self, data: &[u8]) -> f32 {
        if self.detect(data) {
            1.0
        } else {
            0.0
        }
    }
}
