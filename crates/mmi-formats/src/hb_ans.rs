//! Harman Becker Speech Prompts (ANS) audio container parser (§5, RQ-007).

use crate::adapter::{FormatAdapter, FormatCapabilities};
use mmi_core::CoreError;
use serde::{Deserialize, Serialize};

pub const ANS_MAGIC: &[u8; 4] = b"ANS\0";
pub const ANS_HEADER_MIN_SIZE: usize = 16;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HbAnsHeader {
    pub magic: [u8; 4],
    pub codec_id: [u8; 3],
}

#[derive(Debug, Clone)]
pub struct HbAns {
    pub header: HbAnsHeader,
    pub total_size: usize,
}

impl HbAns {
    pub fn parse(data: &[u8]) -> Result<Self, CoreError> {
        if data.len() < ANS_HEADER_MIN_SIZE {
            return Err(CoreError::ImmutabilityViolation(
                "ANS file too small for 16-byte header".into(),
            ));
        }

        if &data[0..4] != ANS_MAGIC {
            return Err(CoreError::ImmutabilityViolation(
                "Invalid ANS magic signature: expected ANS\\0".into(),
            ));
        }

        let mut codec_id = [0u8; 3];
        if data.len() >= 10 {
            codec_id.copy_from_slice(&data[7..10]);
        }

        Ok(Self {
            header: HbAnsHeader {
                magic: *ANS_MAGIC,
                codec_id,
            },
            total_size: data.len(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct HbAnsAdapter {
    capabilities: FormatCapabilities,
}

impl Default for HbAnsAdapter {
    fn default() -> Self {
        Self {
            capabilities: FormatCapabilities::read_only(),
        }
    }
}

impl FormatAdapter for HbAnsAdapter {
    fn format_name(&self) -> &'static str {
        "hb_ans"
    }

    fn capabilities(&self) -> &FormatCapabilities {
        &self.capabilities
    }

    fn detect(&self, data: &[u8]) -> bool {
        if data.len() < 4 {
            return false;
        }
        &data[0..4] == ANS_MAGIC
    }

    fn coverage_ratio(&self, data: &[u8]) -> f32 {
        if self.detect(data) {
            1.0
        } else {
            0.0
        }
    }
}
