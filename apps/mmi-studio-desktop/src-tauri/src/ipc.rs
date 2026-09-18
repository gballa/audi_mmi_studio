//! Strongly typed Tauri IPC command handlers and response contracts (§15, ADR-002).
//!
//! Exposes native workstation capabilities directly to the React 19 frontend
//! without performing any binary processing in JavaScript.

use mmi_assets::decoder::DecodedBitmap;
use mmi_assets::thumbnail::ThumbnailGenerator;
use mmi_canvas::{CanvasRenderer, DisplayMode, LayoutProvenance, ScreenComposition};
use mmi_core::{ContentAddressedStore, CoreError, SourceStore};
use mmi_formats::{FormatAdapter, PrecompAdapter, PrecompImage};
use mmi_re_lab::{EntropyCalculator, HexRow, HexViewer};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectResult {
    pub file_path: String,
    pub size_bytes: u64,
    pub blake3_hash: String,
    pub detected_format: String,
    pub thumbnail_blob_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HexDumpResult {
    pub offset: usize,
    pub length: usize,
    pub rows: Vec<HexRow>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntropyResult {
    pub average_entropy: f64,
    pub classification: String,
    pub segments: Vec<(usize, f64)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScreenRenderResult {
    pub width: u32,
    pub height: u32,
    pub active_mode: String,
    pub pixel_count: usize,
}

/// Inspects a file in originals/ and generates CAS-backed thumbnail if it is an image.
pub fn handle_inspect_file(rel_path: &str, cas_base: &Path) -> Result<InspectResult, CoreError> {
    let originals_root = Path::new("originals");
    let store = SourceStore::new(originals_root)?;
    let data = store.read_bytes(rel_path)?;

    let size_bytes = data.len() as u64;
    let b3 = blake3::hash(&data).to_hex().to_string();

    let mut detected_format = "Raw Binary".to_string();
    let mut thumbnail_blob_id = None;

    let precomp_adapter = PrecompAdapter::default();
    if precomp_adapter.detect(&data) {
        detected_format = "Precomp Graphic (.precomp)".to_string();
        if let Ok(img) = PrecompImage::decode(&data) {
            let bitmap = DecodedBitmap {
                width: img.width as u32,
                height: img.height as u32,
                rgba_pixels: img.pixels,
                source_format: "precomp".to_string(),
                has_alpha: true,
            };

            let cas = ContentAddressedStore::new(cas_base)?;
            if let Ok(thumb_id) = ThumbnailGenerator::create_thumbnail(&bitmap, 64, &cas) {
                thumbnail_blob_id = Some(thumb_id);
            }
        }
    }

    Ok(InspectResult {
        file_path: rel_path.to_string(),
        size_bytes,
        blake3_hash: b3,
        detected_format,
        thumbnail_blob_id,
    })
}

/// Generates a virtualized hex dump of a specified byte range.
pub fn handle_hexdump(rel_path: &str, offset: usize, length: usize) -> Result<HexDumpResult, CoreError> {
    let originals_root = Path::new("originals");
    let store = SourceStore::new(originals_root)?;
    let data = store.read_bytes(rel_path)?;

    let end = (offset + length).min(data.len());
    let slice = if offset < data.len() { &data[offset..end] } else { &[] };
    let rows = HexViewer::render_slice(slice, offset, 16);

    Ok(HexDumpResult {
        offset,
        length,
        rows,
    })
}

/// Computes sliding Shannon entropy for the file.
pub fn handle_entropy(rel_path: &str, window_size: usize) -> Result<EntropyResult, CoreError> {
    let originals_root = Path::new("originals");
    let store = SourceStore::new(originals_root)?;
    let data = store.read_bytes(rel_path)?;

    let avg = EntropyCalculator::shannon_entropy(&data);
    let class = mmi_re_lab::EntropyClass::from_entropy(avg);
    let segments = EntropyCalculator::compute_strip(&data, window_size)
        .into_iter()
        .map(|s| (s.offset, s.entropy))
        .collect();

    Ok(EntropyResult {
        average_entropy: avg,
        classification: format!("{:?}", class),
        segments,
    })
}

/// Renders virtual 800x480 screen composition.
pub fn handle_render_screen(mode: &str, cas_base: &Path) -> Result<ScreenRenderResult, CoreError> {
    let comp = ScreenComposition::new_main_display(
        "MAIN_SCREEN",
        "MMI Navigation",
        LayoutProvenance::Derived {
            evidence_tag: "[EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]]".to_string(),
        },
    );
    let m = match mode.to_lowercase().as_str() {
        "night" => DisplayMode::Night,
        _ => DisplayMode::Day,
    };

    let cas = ContentAddressedStore::new(cas_base)?;
    let img = CanvasRenderer::render(&comp, m, &cas)?;

    Ok(ScreenRenderResult {
        width: 800,
        height: 480,
        active_mode: format!("{:?}", m),
        pixel_count: (img.width() * img.height()) as usize,
    })
}
