//! SMSC MOST INIC Firmware Programming File (IPF) container parser (§5, RQ-009).

use crate::adapter::{FormatAdapter, FormatCapabilities};
use mmi_core::CoreError;
use serde::{Deserialize, Serialize};

pub const SMSC_IPF_MAGIC: &[u8; 16] = &[
    0x01, 0x0f, 0xff, 0xff, 0xff, 0xff, 0x01, 0x01, 0x00, 0x00, 0x20, 0x00, 0x00, 0x01, 0xdc, 0x00,
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SmscIpfHeader {
    pub magic: [u8; 16],
    pub payload_size: usize,
}

#[derive(Debug, Clone)]
pub struct SmscIpf {
    pub header: SmscIpfHeader,
    pub total_size: usize,
}

impl SmscIpf {
    pub fn parse(data: &[u8]) -> Result<Self, CoreError> {
        if data.len() < 16 {
            return Err(CoreError::ImmutabilityViolation(
                "IPF file too small for 16-byte header".into(),
            ));
        }

        if &data[0..16] != SMSC_IPF_MAGIC {
            return Err(CoreError::ImmutabilityViolation(
                "Invalid SMSC IPF magic signature".into(),
            ));
        }

        Ok(Self {
            header: SmscIpfHeader {
                magic: *SMSC_IPF_MAGIC,
                payload_size: data.len() - 16,
            },
            total_size: data.len(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SmscIpfAdapter {
    capabilities: FormatCapabilities,
}

impl Default for SmscIpfAdapter {
    fn default() -> Self {
        Self {
            capabilities: FormatCapabilities::read_only(),
        }
    }
}

impl FormatAdapter for SmscIpfAdapter {
    fn format_name(&self) -> &'static str {
        "smsc_ipf"
    }

    fn capabilities(&self) -> &FormatCapabilities {
        &self.capabilities
    }

    fn detect(&self, data: &[u8]) -> bool {
        if data.len() < 16 {
            return false;
        }
        &data[0..16] == SMSC_IPF_MAGIC
    }

    fn coverage_ratio(&self, data: &[u8]) -> f32 {
        if self.detect(data) {
            1.0
        } else {
            0.0
        }
    }
}
