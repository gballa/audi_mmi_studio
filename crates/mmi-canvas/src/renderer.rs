//! Canvas Renderer: Composites layers into an uncompressed RGBA frame buffer and emits PNGs.

use image::{ImageBuffer, Rgba};
use mmi_core::{ContentAddressedStore, CoreError};

use crate::composition::ScreenComposition;
use crate::palette::{DisplayMode, PaletteSimulator};

pub struct CanvasRenderer;

impl CanvasRenderer {
    /// Composites all layers of a `ScreenComposition` into an RGBA image buffer.
    pub fn render(
        composition: &ScreenComposition,
        mode: DisplayMode,
        cas: &ContentAddressedStore,
    ) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, CoreError> {
        let width = composition.target_width;
        let height = composition.target_height;

        // Base canvas initialized with dark OEM background
        let mut canvas: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_pixel(width, height, Rgba([18, 20, 24, 255]));

        for layer in &composition.layers {
            if !layer.visible {
                continue;
            }

            if let Some(ref blob_id) = layer.asset_blob_id {
                if let Ok(bytes) = cas.read_bytes(blob_id) {
                    if let Ok(dyn_img) = image::load_from_memory(&bytes) {
                        let mut rgba_layer = dyn_img.to_rgba8();
                        PaletteSimulator::apply_simulation(rgba_layer.as_mut(), mode);

                        image::imageops::overlay(
                            &mut canvas,
                            &rgba_layer,
                            layer.x as i64,
                            layer.y as i64,
                        );
                    }
                }
            }
        }

        Ok(canvas)
    }

    /// Renders composition and encodes it directly as standard PNG bytes.
    pub fn render_to_png_bytes(
        composition: &ScreenComposition,
        mode: DisplayMode,
        cas: &ContentAddressedStore,
    ) -> Result<Vec<u8>, CoreError> {
        let canvas = Self::render(composition, mode, cas)?;
        let mut png_bytes = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut png_bytes);
        canvas
            .write_to(&mut cursor, image::ImageFormat::Png)
            .map_err(|e| CoreError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        Ok(png_bytes)
    }
}
