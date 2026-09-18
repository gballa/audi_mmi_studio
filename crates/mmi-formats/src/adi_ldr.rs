//! Analog Devices Blackfin DSP Loader (LDR) executable parser (§5, RQ-012).

use crate::adapter::{FormatAdapter, FormatCapabilities};
use mmi_core::CoreError;
use serde::{Deserialize, Serialize};

pub const ADI_LDR_MAGIC: &[u8; 4] = &[0xe2, 0xd3, 0xc6, 0xb8];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdiLdrHeader {
    pub magic: [u8; 4],
    pub header_size: u32,
    pub target_processor: u32,
}

#[derive(Debug, Clone)]
pub struct AdiLdr {
    pub header: AdiLdrHeader,
    pub total_size: usize,
}

impl AdiLdr {
    pub fn parse(data: &[u8]) -> Result<Self, CoreError> {
        if data.len() < 12 {
            return Err(CoreError::ImmutabilityViolation(
                "LDR file too small for 12-byte header".into(),
            ));
        }

        if &data[0..4] != ADI_LDR_MAGIC {
            return Err(CoreError::ImmutabilityViolation(
                "Invalid Blackfin LDR magic: expected 0xB8C6D3E2".into(),
            ));
        }

        let header_size = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        let target_processor = u32::from_le_bytes([data[8], data[9], data[10], data[11]]);

        Ok(Self {
            header: AdiLdrHeader {
                magic: *ADI_LDR_MAGIC,
                header_size,
                target_processor,
            },
            total_size: data.len(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct AdiLdrAdapter {
    capabilities: FormatCapabilities,
}

impl Default for AdiLdrAdapter {
    fn default() -> Self {
        Self {
            capabilities: FormatCapabilities::read_only(),
        }
    }
}

impl FormatAdapter for AdiLdrAdapter {
    fn format_name(&self) -> &'static str {
        "adi_ldr"
    }

    fn capabilities(&self) -> &FormatCapabilities {
        &self.capabilities
    }

    fn detect(&self, data: &[u8]) -> bool {
        if data.len() < 4 {
            return false;
        }
        &data[0..4] == ADI_LDR_MAGIC
    }

    fn coverage_ratio(&self, data: &[u8]) -> f32 {
        if self.detect(data) {
            1.0
        } else {
            0.0
        }
    }
}
