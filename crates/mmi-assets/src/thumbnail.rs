//! Thumbnail Generator: Generates fixed-dimension preview PNGs and saves them into CAS.

use image::imageops::FilterType;
use image::{DynamicImage, ImageBuffer, Rgba};
use mmi_core::{ContentAddressedStore, CoreError};

use crate::decoder::DecodedBitmap;

pub struct ThumbnailGenerator;

impl ThumbnailGenerator {
    /// Resizes a decoded bitmap to a square bounding box (e.g. 64x64) preserving aspect ratio,
    /// encodes it as a PNG, and stores it in CAS. Returns the CAS blob ID (BLAKE3 hex).
    pub fn create_thumbnail(
        bitmap: &DecodedBitmap,
        max_dim: u32,
        cas: &ContentAddressedStore,
    ) -> Result<String, CoreError> {
        let img_buf: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(
            bitmap.width,
            bitmap.height,
            bitmap.rgba_pixels.clone(),
        )
        .ok_or_else(|| {
            CoreError::ImmutabilityViolation("Failed to construct image buffer for thumbnail".to_string())
        })?;

        let dyn_img = DynamicImage::ImageRgba8(img_buf);
        let thumbnail = dyn_img.resize(max_dim, max_dim, FilterType::Triangle);

        let mut png_bytes = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut png_bytes);
        thumbnail
            .write_to(&mut cursor, image::ImageFormat::Png)
            .map_err(|e| CoreError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        let (blake3_hex, _) = cas.put_bytes(&png_bytes)?;
        Ok(blake3_hex)
    }
}
