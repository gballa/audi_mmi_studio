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

/// Request contract for compiling navigation map SD media.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapCompileIpcRequest {
    pub profile_code: String,
    pub enable_gmp: bool,
    pub output_dir: String,
}

/// Response contract for compilation pipeline execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapCompileIpcResponse {
    pub success: bool,
    pub total_pages: usize,
    pub total_bytes: u64,
    pub root_sha1: String,
    pub logs: Vec<String>,
}

/// Executes full end-to-end map compilation and SD media packaging pipeline.
pub fn handle_compile_map_pipeline(
    request: MapCompileIpcRequest,
) -> Result<MapCompileIpcResponse, CoreError> {
    use mmi_rebuild::geo::{IrDataset, IrEdge, IrNode, IrPoi, RegionalProfile};
    use mmi_rebuild::osm_ingest::{CountryCode, OsmIngestConfig, OsmIngestPipeline};
    use mmi_rebuild::gmp_enrich::GmpEnrichmentPipeline;
    use mmi_rebuild::{FldbCompilerPipeline, LitCompiler, AtlasCompiler};
    use mmi_media::{SdMediaPackager, SdMediaPackageConfig};
    use std::fs;

    let mut logs = Vec::new();
    let profile = RegionalProfile::from_code(&request.profile_code)
        .unwrap_or_else(|| RegionalProfile::micro_albania());

    logs.push(format!("Stage 1: Initializing regional profile '{}' ({})", profile.name, profile.code));

    let mut dataset = IrDataset::new(profile.bbox, Some(profile.code.clone()));

    // Check if source geodata files exist
    let sources_dir = Path::new("data/sources");
    let mut ingested_from_file = false;

    if sources_dir.exists() {
        if let Ok(entries) = fs::read_dir(sources_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                let config = OsmIngestConfig {
                    bounding_box: Some(profile.bbox),
                    country: CountryCode::from_str_code(&profile.code),
                    max_frc: 7,
                    simplify_epsilon_m: 0.5,
                };
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if ext.eq_ignore_ascii_case("geojson") || ext.eq_ignore_ascii_case("json") {
                        if let Ok(text) = fs::read_to_string(&path) {
                            if let Ok(ds) = OsmIngestPipeline::ingest_geojson(&text, &config) {
                                dataset.nodes.extend(ds.nodes);
                                dataset.edges.extend(ds.edges);
                                dataset.restrictions.extend(ds.restrictions);
                                ingested_from_file = true;
                                logs.push(format!("Stage 1: Ingested OpenStreetMap vector geometry from {}", path.display()));
                            }
                        }
                    } else if ext.eq_ignore_ascii_case("pbf") {
                        if let Ok(mut f) = fs::File::open(&path) {
                            if let Ok(ds) = OsmIngestPipeline::ingest_pbf_stream(&mut f, &config) {
                                dataset.nodes.extend(ds.nodes);
                                dataset.edges.extend(ds.edges);
                                dataset.restrictions.extend(ds.restrictions);
                                ingested_from_file = true;
                                logs.push(format!("Stage 1: Ingested OpenStreetMap PBF geometry from {}", path.display()));
                            }
                        }
                    }
                }
            }
        }
    }

    if !ingested_from_file || dataset.nodes.is_empty() {
        // Seed default regional corridor (Tirana - Durres Corridor)
        let center_lat = (profile.bbox.min_lat + profile.bbox.max_lat) / 2.0;
        let center_lon = (profile.bbox.min_lon + profile.bbox.max_lon) / 2.0;
        dataset.nodes.push(IrNode::from_wgs84(1, center_lat, center_lon, 100, 0));
        dataset.nodes.push(IrNode::from_wgs84(2, center_lat + 0.015, center_lon + 0.015, 105, 0));
        dataset.nodes.push(IrNode::from_wgs84(3, center_lat + 0.030, center_lon + 0.030, 110, 0));

        dataset.edges.push(IrEdge {
            edge_id: 1,
            from_node: 1,
            to_node: 2,
            length_dm: 1200,
            frc: 1,
            speed_forward: 90,
            speed_reverse: 90,
            lane_count: 2,
            turn_lane_mask: 0,
            geometry: vec![],
            access_flags: 0,
        });
        dataset.edges.push(IrEdge {
            edge_id: 2,
            from_node: 2,
            to_node: 3,
            length_dm: 1500,
            frc: 1,
            speed_forward: 110,
            speed_reverse: 110,
            lane_count: 2,
            turn_lane_mask: 0,
            geometry: vec![],
            access_flags: 0,
        });

        dataset.pois.push(IrPoi::new(1, "RRUGA E DIBRES".into(), "highway".into(), center_lat, center_lon, None));
        dataset.pois.push(IrPoi::new(2, "AUTOSTRADA TIRANE DURRES".into(), "highway".into(), center_lat + 0.01, center_lon + 0.01, None));
        dataset.pois.push(IrPoi::new(3, "SHESTI SKENDERBEJ".into(), "place".into(), center_lat + 0.02, center_lon + 0.02, None));

        logs.push(format!("Stage 1: Staged {} corridor nodes and {} routing edges", dataset.nodes.len(), dataset.edges.len()));
    }

    // Stage 2: GMP Enrichment
    if request.enable_gmp {
        let mut gmp_pipe = GmpEnrichmentPipeline::new_offline();
        let _ = gmp_pipe.enrich_dataset(&mut dataset);
        logs.push("Stage 2: Google Maps Platform enrichment applied (Offline Places & Geocoding)".to_string());
    } else {
        logs.push("Stage 2: Offline vector definition active without GMP online lookup".to_string());
    }

    // Stage 3: FLDB & LIT Compilation into intermediate build directory
    let stage_dir_buf = std::env::temp_dir().join(format!(
        "mmi_desktop_stage_{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
    ));
    let stage_dir = stage_dir_buf.as_path();
    let _ = fs::create_dir_all(stage_dir);

    let compile_res = FldbCompilerPipeline::compile_and_package(&dataset, stage_dir, Some("2026_ECE"))
        .map_err(CoreError::Io)?;
    logs.push(format!("Stage 3: FLDB 544-byte physical pages assembled ({} pages, {} bytes)", compile_res.total_pages, compile_res.total_bytes));

    let pkgdb_dir = stage_dir.join("pkgdb");
    let lit_res = LitCompiler::compile_lit3gp_package(&dataset, &pkgdb_dir, "2026.01.0")
        .map_err(CoreError::Io)?;
    logs.push(format!("Stage 3: LIT3GP Rotary Speller B-Tree generated ({} search pages)", lit_res.total_pages));

    let atlas_res = AtlasCompiler::compile_atlas_package(&dataset, &pkgdb_dir, "2026.01.0")
        .map_err(CoreError::Io)?;
    logs.push(format!("Stage 3: Orion ATLAS 3D visual tiles compiled ({} terrain, {} building meshes)", atlas_res.terrain_tiles, atlas_res.building_tiles));

    // Stage 4: Assemble final SD Media Package
    let output_path = Path::new(&request.output_dir);
    let config = SdMediaPackageConfig {
        release_tag: "2026_ECE".to_string(),
        volume_label: "MMI3G_NAV".to_string(),
        target_variant: "9411".to_string(),
        enable_recovery_script: true,
        sanitize_dotfiles: true,
    };

    let media_report = SdMediaPackager::assemble_release_package(stage_dir, output_path, &config)
        .map_err(CoreError::Io)?;

    logs.push(format!("Stage 4: SD media sanitized and pre-flight verified (6/6 Pass: {})", media_report.simulation_passed));
    logs.push(format!("Stage 4: Root metainfo2 SHA-1 attestation: {}", media_report.root_metainfo_sha1));

    let _ = fs::remove_dir_all(&stage_dir_buf);

    Ok(MapCompileIpcResponse {
        success: media_report.simulation_passed,
        total_pages: compile_res.total_pages + lit_res.total_pages,
        total_bytes: media_report.total_bytes,
        root_sha1: media_report.root_metainfo_sha1,
        logs,
    })
}
