//! Harman Becker Binary Grammar resource parser (§5, RQ-011).

use crate::adapter::{FormatAdapter, FormatCapabilities};
use mmi_core::CoreError;
use serde::{Deserialize, Serialize};

pub const HBGR_SIG: &[u8; 4] = &[0xfe, 0xff, 0xff, 0xff];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HbGrammarHeader {
    pub signature: [u8; 4],
    pub header_length: u32,
    pub banner: String,
}

#[derive(Debug, Clone)]
pub struct HbGrammar {
    pub header: HbGrammarHeader,
    pub total_size: usize,
}

impl HbGrammar {
    pub fn parse(data: &[u8]) -> Result<Self, CoreError> {
        if data.len() < 8 {
            return Err(CoreError::ImmutabilityViolation(
                "HBGR file too small for 8-byte header".into(),
            ));
        }

        if &data[0..4] != HBGR_SIG {
            return Err(CoreError::ImmutabilityViolation(
                "Invalid HBGR signature: expected 0xFFFFFFFE".into(),
            ));
        }

        let header_length = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        if header_length < 8 || (header_length as usize) > data.len() {
            return Err(CoreError::ImmutabilityViolation(
                "Invalid HBGR header length field".into(),
            ));
        }

        let banner = String::from_utf8_lossy(&data[8..header_length as usize])
            .trim_matches('\0')
            .trim()
            .to_string();

        Ok(Self {
            header: HbGrammarHeader {
                signature: *HBGR_SIG,
                header_length,
                banner,
            },
            total_size: data.len(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct HbGrammarAdapter {
    capabilities: FormatCapabilities,
}

impl Default for HbGrammarAdapter {
    fn default() -> Self {
        Self {
            capabilities: FormatCapabilities::read_only(),
        }
    }
}

impl FormatAdapter for HbGrammarAdapter {
    fn format_name(&self) -> &'static str {
        "hb_grammar"
    }

    fn capabilities(&self) -> &FormatCapabilities {
        &self.capabilities
    }

    fn detect(&self, data: &[u8]) -> bool {
        if data.len() < 4 {
            return false;
        }
        &data[0..4] == HBGR_SIG
    }

    fn coverage_ratio(&self, data: &[u8]) -> f32 {
        if self.detect(data) {
            1.0
        } else {
            0.0
        }
    }
}
