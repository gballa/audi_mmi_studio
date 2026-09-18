//! QNX 6 Image FileSystem (IFS) parser (§5, RQ-005).

use crate::adapter::{FormatAdapter, FormatCapabilities};
use mmi_core::CoreError;
use serde::{Deserialize, Serialize};

pub const QNX_IFS_MAGIC: [u8; 4] = [0xeb, 0x7e, 0xff, 0x00];
pub const IFS_HEADER_MIN_SIZE: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QnxIfsHeader {
    pub magic: [u8; 4],
    pub version: u16,
    pub flags: u16,
    pub header_size: u16,
    pub machine_type: u16,
    pub startup_size: u32,
    pub stored_size: u32,
    pub image_size: u32,
    pub ram_size: u32,
}

#[derive(Debug, Clone)]
pub struct QnxIfs {
    pub header: QnxIfsHeader,
    pub total_size: usize,
}

impl QnxIfs {
    pub fn parse(data: &[u8]) -> Result<Self, CoreError> {
        if data.len() < IFS_HEADER_MIN_SIZE {
            return Err(CoreError::ImmutabilityViolation(
                "QNX IFS file too small for 32-byte header".into(),
            ));
        }

        if &data[0..4] != &QNX_IFS_MAGIC {
            return Err(CoreError::ImmutabilityViolation(
                "Invalid QNX IFS magic signature".into(),
            ));
        }

        let version = u16::from_le_bytes([data[4], data[5]]);
        let flags = u16::from_le_bytes([data[6], data[7]]);
        let header_size = u16::from_le_bytes([data[8], data[9]]);
        let machine_type = u16::from_le_bytes([data[10], data[11]]);
        let startup_size = u32::from_le_bytes([data[12], data[13], data[14], data[15]]);
        let stored_size = u32::from_le_bytes([data[16], data[17], data[18], data[19]]);
        let image_size = u32::from_le_bytes([data[20], data[21], data[22], data[23]]);
        let ram_size = u32::from_le_bytes([data[24], data[25], data[26], data[27]]);

        Ok(Self {
            header: QnxIfsHeader {
                magic: QNX_IFS_MAGIC,
                version,
                flags,
                header_size,
                machine_type,
                startup_size,
                stored_size,
                image_size,
                ram_size,
            },
            total_size: data.len(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct QnxIfsAdapter {
    capabilities: FormatCapabilities,
}

impl Default for QnxIfsAdapter {
    fn default() -> Self {
        Self {
            capabilities: FormatCapabilities::read_only(),
        }
    }
}

impl FormatAdapter for QnxIfsAdapter {
    fn format_name(&self) -> &'static str {
        "qnx_ifs"
    }

    fn capabilities(&self) -> &FormatCapabilities {
        &self.capabilities
    }

    fn detect(&self, data: &[u8]) -> bool {
        if data.len() < 4 {
            return false;
        }
        &data[0..4] == &QNX_IFS_MAGIC
    }

    fn coverage_ratio(&self, data: &[u8]) -> f32 {
        if self.detect(data) {
            1.0
        } else {
            0.0
        }
    }
}
