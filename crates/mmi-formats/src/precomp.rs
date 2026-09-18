//! Decoder and round-trip encoder for Harman Precomp instrument cluster graphics (.precomp).

use std::io::{Read, Write};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use mmi_core::CoreError;
use serde::{Deserialize, Serialize};

use crate::adapter::{FormatAdapter, FormatCapabilities};

pub const PRECOMP_MAGIC: [u8; 6] = [0x00, 0x01, 0x00, 0x00, 0x00, 0x00];
pub const HEADER_SIZE: usize = 10;

/// Decoded representation of a Precomp graphic containing dimensions and raw 32-bit pixel data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrecompImage {
    pub width: u16,
    pub height: u16,
    /// Raw uncompressed 32-bit RGBA/ARGB pixel bytes (length == width * height * 4).
    pub pixels: Vec<u8>,
}

impl PrecompImage {
    /// Decodes a `.precomp` binary buffer into raw 32-bit pixels.
    pub fn decode(data: &[u8]) -> Result<Self, CoreError> {
        if data.len() < HEADER_SIZE {
            return Err(CoreError::ImmutabilityViolation(format!(
                "Precomp file too small: {} bytes (minimum header is {} bytes)",
                data.len(),
                HEADER_SIZE
            )));
        }

        if &data[0..6] != PRECOMP_MAGIC {
            return Err(CoreError::ImmutabilityViolation(
                "Invalid Precomp magic header signature".to_string(),
            ));
        }

        let width = u16::from_be_bytes([data[6], data[7]]);
        let height = u16::from_be_bytes([data[8], data[9]]);

        // Security ceiling (§18): Maximum dimension allowed is 4096 x 4096 (64 MiB uncompressed)
        if width > 4096 || height > 4096 {
            return Err(CoreError::ImmutabilityViolation(format!(
                "Precomp dimensions exceed maximum safety ceiling ({}x{} > 4096x4096)",
                width, height
            )));
        }

        let expected_pixel_bytes = (width as usize) * (height as usize) * 4;

        let mut decoder = ZlibDecoder::new(&data[HEADER_SIZE..]);
        let mut pixels = Vec::with_capacity(expected_pixel_bytes);
        decoder.read_to_end(&mut pixels)?;

        if pixels.len() != expected_pixel_bytes {
            return Err(CoreError::CorruptBlob {
                expected: format!("{} pixel bytes ({}x{} * 4)", expected_pixel_bytes, width, height),
                actual: format!("{} bytes decompressed", pixels.len()),
            });
        }

        Ok(Self {
            width,
            height,
            pixels,
        })
    }

    /// Encodes this `PrecompImage` back into the binary `.precomp` container format.
    pub fn encode(&self) -> Result<Vec<u8>, CoreError> {
        let expected_bytes = (self.width as usize) * (self.height as usize) * 4;
        if self.pixels.len() != expected_bytes {
            return Err(CoreError::ImmutabilityViolation(format!(
                "Pixel buffer length mismatch: expected {}, got {}",
                expected_bytes,
                self.pixels.len()
            )));
        }

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(&self.pixels)?;
        let compressed = encoder.finish()?;

        let mut output = Vec::with_capacity(HEADER_SIZE + compressed.len());
        output.extend_from_slice(&PRECOMP_MAGIC);
        output.extend_from_slice(&self.width.to_be_bytes());
        output.extend_from_slice(&self.height.to_be_bytes());
        output.extend_from_slice(&compressed);

        Ok(output)
    }
}

pub struct PrecompAdapter {
    capabilities: FormatCapabilities,
}

impl Default for PrecompAdapter {
    fn default() -> Self {
        Self {
            capabilities: FormatCapabilities::full_support(),
        }
    }
}

impl FormatAdapter for PrecompAdapter {
    fn format_name(&self) -> &'static str {
        "precomp"
    }

    fn capabilities(&self) -> &FormatCapabilities {
        &self.capabilities
    }

    fn detect(&self, data: &[u8]) -> bool {
        if data.len() >= HEADER_SIZE && &data[0..6] == PRECOMP_MAGIC {
            let w = u16::from_be_bytes([data[6], data[7]]);
            let h = u16::from_be_bytes([data[8], data[9]]);
            w > 0 && h > 0 && w < 4096 && h < 4096
        } else {
            false
        }
    }

    fn coverage_ratio(&self, data: &[u8]) -> f32 {
        if self.detect(data) {
            1.0 // 100% of bytes are accounted for: 10-byte header + zlib stream
        } else {
            0.0
        }
    }
}
