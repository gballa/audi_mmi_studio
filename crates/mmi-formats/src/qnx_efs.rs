//! QNX 6 Embedded Flash FileSystem (F3S / ETFS) parser (§5, RQ-006).

use crate::adapter::{FormatAdapter, FormatCapabilities};
use mmi_core::CoreError;
use serde::{Deserialize, Serialize};

pub const QNX_F3S_MAGIC: &[u8; 8] = b"QSSL_F3S";
pub const EFS_HEADER_MIN_SIZE: usize = 72;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QnxEfsHeader {
    pub magic: [u8; 8],
    pub mount_point: String,
}

#[derive(Debug, Clone)]
pub struct QnxEfs {
    pub header: QnxEfsHeader,
    pub total_size: usize,
}

impl QnxEfs {
    pub fn parse(data: &[u8]) -> Result<Self, CoreError> {
        if data.len() < EFS_HEADER_MIN_SIZE {
            return Err(CoreError::ImmutabilityViolation(
                "QNX EFS file too small for 72-byte header".into(),
            ));
        }

        if &data[0x2C..0x34] != QNX_F3S_MAGIC {
            return Err(CoreError::ImmutabilityViolation(
                "Invalid QNX EFS magic signature: expected QSSL_F3S".into(),
            ));
        }

        // Mount point string at offset 0x48
        let mount_slice = &data[0x48..];
        let null_pos = mount_slice.iter().position(|&b| b == 0).unwrap_or(mount_slice.len());
        let mount_point = String::from_utf8_lossy(&mount_slice[..null_pos]).to_string();

        let mut magic = [0u8; 8];
        magic.copy_from_slice(QNX_F3S_MAGIC);

        Ok(Self {
            header: QnxEfsHeader {
                magic,
                mount_point,
            },
            total_size: data.len(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct QnxEfsAdapter {
    capabilities: FormatCapabilities,
}

impl Default for QnxEfsAdapter {
    fn default() -> Self {
        Self {
            capabilities: FormatCapabilities::read_only(),
        }
    }
}

impl FormatAdapter for QnxEfsAdapter {
    fn format_name(&self) -> &'static str {
        "qnx_efs"
    }

    fn capabilities(&self) -> &FormatCapabilities {
        &self.capabilities
    }

    fn detect(&self, data: &[u8]) -> bool {
        if data.len() < 0x34 {
            return false;
        }
        &data[0x2C..0x34] == QNX_F3S_MAGIC
    }

    fn coverage_ratio(&self, data: &[u8]) -> f32 {
        if self.detect(data) {
            1.0
        } else {
            0.0
        }
    }
}
