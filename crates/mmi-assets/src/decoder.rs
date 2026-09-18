//! Unified Image Decoder for MMI assets: Precomp, PNG, BMP, and raw bitmap buffers.

use image::{ImageBuffer, Rgba};
use mmi_core::CoreError;
use mmi_formats::{FormatAdapter, PrecompAdapter, PrecompImage};
use serde::{Deserialize, Serialize};

/// Supported decoded pixel representation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecodedBitmap {
    pub width: u32,
    pub height: u32,
    /// 32-bit RGBA pixels: length == width * height * 4
    pub rgba_pixels: Vec<u8>,
    pub source_format: String,
    pub has_alpha: bool,
}

impl DecodedBitmap {
    pub fn new(width: u32, height: u32, rgba_pixels: Vec<u8>, source_format: impl Into<String>) -> Result<Self, CoreError> {
        let expected_len = (width as usize) * (height as usize) * 4;
        if rgba_pixels.len() != expected_len {
            return Err(CoreError::ImmutabilityViolation(format!(
                "Decoded bitmap buffer size mismatch: expected {} bytes, got {}",
                expected_len,
                rgba_pixels.len()
            )));
        }

        let has_alpha = rgba_pixels.chunks_exact(4).any(|p| p[3] < 255);

        Ok(Self {
            width,
            height,
            rgba_pixels,
            source_format: source_format.into(),
            has_alpha,
        })
    }

    /// Converts this bitmap to a PNG-encoded byte vector.
    pub fn to_png_bytes(&self) -> Result<Vec<u8>, CoreError> {
        let img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(self.width, self.height, self.rgba_pixels.clone())
            .ok_or_else(|| CoreError::ImmutabilityViolation("Failed to construct image buffer from RGBA pixels".to_string()))?;

        let mut png_bytes = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut png_bytes);
        img.write_to(&mut cursor, image::ImageFormat::Png)
            .map_err(|e| CoreError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        Ok(png_bytes)
    }
}

/// Unified decoder dispatcher for visual assets.
pub struct AssetDecoder;

impl AssetDecoder {
    /// Attempts to decode input bytes into a normalized `DecodedBitmap`.
    pub fn decode(data: &[u8]) -> Result<DecodedBitmap, CoreError> {
        // 1. Try Precomp
        let precomp_adapter = PrecompAdapter::default();
        if precomp_adapter.detect(data) {
            let img = PrecompImage::decode(data)?;
            return DecodedBitmap::new(img.width as u32, img.height as u32, img.pixels, "precomp");
        }

        // 2. Try PNG / BMP via image crate
        if let Ok(dynamic_img) = image::load_from_memory(data) {
            let rgba = dynamic_img.to_rgba8();
            let (w, h) = rgba.dimensions();
            return DecodedBitmap::new(w, h, rgba.into_raw(), "standard_image");
        }

        Err(CoreError::NotFound("Unsupported or unrecognized image format".to_string()))
    }
}
