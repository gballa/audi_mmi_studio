//! Day/Night Palette Simulation and Color Space Transformations.

use serde::{Deserialize, Serialize};

/// Display simulation mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisplayMode {
    /// Daytime high-contrast mode (standard sunlight readability).
    Day,
    /// Nighttime low-glare mode (darkened background, subdued amber/red/cyan accents).
    Night,
    /// Low-color quantisation preview (simulating hardware bit depth limits).
    ReducedColorQuantized,
}

pub struct PaletteSimulator;

impl PaletteSimulator {
    /// Transforms an RGBA pixel buffer based on the requested display mode.
    pub fn apply_simulation(rgba_pixels: &mut [u8], mode: DisplayMode) {
        match mode {
            DisplayMode::Day => {
                // Day mode is 1:1 original color reproduction
            }
            DisplayMode::Night => {
                // Night mode dimming / amber shift
                for chunk in rgba_pixels.chunks_exact_mut(4) {
                    let r = chunk[0] as f32;
                    let g = chunk[1] as f32;
                    let b = chunk[2] as f32;

                    // Apply night attenuation: damp blue/green light, preserve amber/red
                    chunk[0] = (r * 0.85).min(255.0) as u8;
                    chunk[1] = (g * 0.65).min(255.0) as u8;
                    chunk[2] = (b * 0.45).min(255.0) as u8;
                }
            }
            DisplayMode::ReducedColorQuantized => {
                // Quantize to RGB565 simulation
                for chunk in rgba_pixels.chunks_exact_mut(4) {
                    chunk[0] = (chunk[0] & 0xF8) | (chunk[0] >> 5);
                    chunk[1] = (chunk[1] & 0xFC) | (chunk[1] >> 6);
                    chunk[2] = (chunk[2] & 0xF8) | (chunk[2] >> 5);
                }
            }
        }
    }
}
