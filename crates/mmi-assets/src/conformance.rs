//! The Conformance Pipeline: 5-stage admission gate (RECEIVE -> MEASURE -> CONFORM -> VERIFY -> ADMIT/REJECT).

use image::imageops::FilterType;
use image::{ImageBuffer, Rgba};
use mmi_core::ContentAddressedStore;
use mmi_formats::PrecompImage;
use serde::{Deserialize, Serialize};

/// Concrete constraints that a target asset class enforces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetConstraints {
    pub target_width: u32,
    pub target_height: u32,
    pub target_format: String, // "precomp" | "png" | "bmp"
    pub max_encoded_bytes: Option<usize>,
    pub require_alpha: bool,
}

/// Verdict emitted by the Conformance Pipeline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConformanceVerdict {
    Admitted {
        blob_id: String,
        sha256_hex: String,
        encoded_bytes: usize,
        width: u32,
        height: u32,
        format: String,
    },
    Rejected {
        stage: String,
        reason: String,
        details: String,
    },
}

pub struct ConformancePipeline;

impl ConformancePipeline {
    /// Executes the 5-stage conformance protocol on candidate image data.
    pub fn process(
        candidate_bytes: &[u8],
        constraints: &AssetConstraints,
        cas: &ContentAddressedStore,
    ) -> ConformanceVerdict {
        // 1. RECEIVE: Decode candidate bytes
        let dynamic_img = match image::load_from_memory(candidate_bytes) {
            Ok(img) => img,
            Err(e) => {
                return ConformanceVerdict::Rejected {
                    stage: "RECEIVE".to_string(),
                    reason: "Failed to decode input image bytes".to_string(),
                    details: e.to_string(),
                };
            }
        };

        // 2. MEASURE: Extract candidate metrics
        let initial_rgba = dynamic_img.to_rgba8();
        let (meas_w, meas_h) = initial_rgba.dimensions();

        // 3. CONFORM: Resize to target dimensions & re-encode
        let conformed_img = if meas_w != constraints.target_width || meas_h != constraints.target_height {
            dynamic_img.resize_exact(
                constraints.target_width,
                constraints.target_height,
                FilterType::Lanczos3,
            )
        } else {
            dynamic_img
        };

        let rgba = conformed_img.to_rgba8();
        let pixels = rgba.into_raw();

        let encoded_bytes = match constraints.target_format.as_str() {
            "precomp" => {
                let precomp = PrecompImage {
                    width: constraints.target_width as u16,
                    height: constraints.target_height as u16,
                    pixels,
                };
                match precomp.encode() {
                    Ok(b) => b,
                    Err(e) => {
                        return ConformanceVerdict::Rejected {
                            stage: "CONFORM".to_string(),
                            reason: "Failed to encode into Harman Precomp format".to_string(),
                            details: e.to_string(),
                        };
                    }
                }
            }
            "png" | _ => {
                let img_buf: ImageBuffer<Rgba<u8>, _> = match ImageBuffer::from_raw(
                    constraints.target_width,
                    constraints.target_height,
                    pixels,
                ) {
                    Some(b) => b,
                    None => {
                        return ConformanceVerdict::Rejected {
                            stage: "CONFORM".to_string(),
                            reason: "Failed to construct ImageBuffer from pixels".to_string(),
                            details: "".to_string(),
                        };
                    }
                };

                let mut png_buf = Vec::new();
                let mut cursor = std::io::Cursor::new(&mut png_buf);
                if let Err(e) = img_buf.write_to(&mut cursor, image::ImageFormat::Png) {
                    return ConformanceVerdict::Rejected {
                        stage: "CONFORM".to_string(),
                        reason: "Failed to encode conforming PNG".to_string(),
                        details: e.to_string(),
                    };
                }
                png_buf
            }
        };

        // 4. VERIFY: Enforce encoded size ceiling
        if let Some(max_bytes) = constraints.max_encoded_bytes {
            if encoded_bytes.len() > max_bytes {
                return ConformanceVerdict::Rejected {
                    stage: "VERIFY".to_string(),
                    reason: "Encoded size exceeds maximum allocation ceiling".to_string(),
                    details: format!(
                        "Encoded size {} bytes > maximum ceiling {} bytes",
                        encoded_bytes.len(),
                        max_bytes
                    ),
                };
            }
        }

        // 5. ADMIT: Ingest into CAS and return Admitted verdict
        let (blob_id, sha256_hex) = match cas.put_bytes(&encoded_bytes) {
            Ok(pair) => pair,
            Err(e) => {
                return ConformanceVerdict::Rejected {
                    stage: "ADMIT".to_string(),
                    reason: "Failed to write conforming asset to CAS".to_string(),
                    details: e.to_string(),
                };
            }
        };

        ConformanceVerdict::Admitted {
            blob_id,
            sha256_hex,
            encoded_bytes: encoded_bytes.len(),
            width: constraints.target_width,
            height: constraints.target_height,
            format: constraints.target_format.clone(),
        }
    }
}
