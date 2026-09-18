//! mmi-canvas: Screen reconstruction canvas, day/night palette simulation, and UI compositor.

pub mod composition;
pub mod palette;
pub mod renderer;

pub use composition::{LayoutProvenance, ScreenComposition, ScreenLayer};
pub use palette::{DisplayMode, PaletteSimulator};
pub use renderer::CanvasRenderer;
