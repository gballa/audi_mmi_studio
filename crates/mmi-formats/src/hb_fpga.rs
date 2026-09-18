//! Harman Becker System FPGA Bitstream container parser (§5, RQ-008).

use crate::adapter::{FormatAdapter, FormatCapabilities};
use mmi_core::CoreError;
use serde::{Deserialize, Serialize};

pub const FPGA_HDG_TAG: &[u8; 4] = b".HDG";
pub const FPGA_XILINX_SYNC: &[u8; 4] = &[0x55, 0x99, 0xaa, 0x66];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HbFpgaChunkInfo {
    pub tag: String,
    pub length: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HbFpgaHeader {
    pub hardware_info: String,
    pub user_info: String,
    pub bitstream_len: u32,
}

#[derive(Debug, Clone)]
pub struct HbFpga {
    pub header: HbFpgaHeader,
    pub chunks: Vec<HbFpgaChunkInfo>,
    pub total_size: usize,
}

impl HbFpga {
    pub fn parse(data: &[u8]) -> Result<Self, CoreError> {
        if data.len() < 16 {
            return Err(CoreError::ImmutabilityViolation(
                "HBBIN file too small for FPGA chunk header".into(),
            ));
        }

        if &data[0..4] != FPGA_HDG_TAG {
            return Err(CoreError::ImmutabilityViolation(
                "Invalid FPGA bitstream magic: expected .HDG".into(),
            ));
        }

        let mut offset = 0;
        let mut chunks = Vec::new();
        let mut hardware_info = String::new();
        let mut user_info = String::new();
        let mut bitstream_len = 0u32;

        while offset + 8 <= data.len() {
            let tag_bytes = &data[offset..offset + 4];
            let tag_str = String::from_utf8_lossy(tag_bytes).to_string();
            let chunk_len = u32::from_le_bytes([
                data[offset + 4],
                data[offset + 5],
                data[offset + 6],
                data[offset + 7],
            ]);

            if chunk_len < 8 || offset + (chunk_len as usize) > data.len() {
                break;
            }

            let payload = &data[offset + 8..offset + (chunk_len as usize)];
            if tag_bytes == b".HDH" {
                hardware_info = String::from_utf8_lossy(payload)
                    .trim_matches('\0')
                    .trim()
                    .to_string();
            } else if tag_bytes == b".HGU" {
                user_info = String::from_utf8_lossy(payload)
                    .trim_matches('\0')
                    .trim()
                    .to_string();
            } else if tag_bytes == b".FDL" {
                bitstream_len = chunk_len - 8;
            }

            chunks.push(HbFpgaChunkInfo {
                tag: tag_str,
                length: chunk_len,
            });

            offset += chunk_len as usize;
        }

        Ok(Self {
            header: HbFpgaHeader {
                hardware_info,
                user_info,
                bitstream_len,
            },
            chunks,
            total_size: data.len(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct HbFpgaAdapter {
    capabilities: FormatCapabilities,
}

impl Default for HbFpgaAdapter {
    fn default() -> Self {
        Self {
            capabilities: FormatCapabilities::read_only(),
        }
    }
}

impl FormatAdapter for HbFpgaAdapter {
    fn format_name(&self) -> &'static str {
        "hb_fpga"
    }

    fn capabilities(&self) -> &FormatCapabilities {
        &self.capabilities
    }

    fn detect(&self, data: &[u8]) -> bool {
        if data.len() < 4 {
            return false;
        }
        &data[0..4] == FPGA_HDG_TAG
    }

    fn coverage_ratio(&self, data: &[u8]) -> f32 {
        if self.detect(data) {
            1.0
        } else {
            0.0
        }
    }
}
