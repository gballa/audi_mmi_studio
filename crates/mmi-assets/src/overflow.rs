//! Font metrics calculation and UI text overflow prediction.
//!
//! Parses SFNT `head`, `hhea`, `maxp`, `cmap`, and `hmtx` tables from TrueType fonts
//! to compute glyph advance widths and determine whether localized strings will fit within
//! target UI bounding boxes.

use mmi_core::CoreError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Parsed horizontal metrics from TrueType tables.
#[derive(Debug, Clone)]
pub struct FontMetricsEngine {
    pub units_per_em: u16,
    pub ascent: i16,
    pub descent: i16,
    pub line_gap: i16,
    pub glyph_advances: Vec<u16>,
    pub char_to_glyph: HashMap<char, u16>,
}

impl FontMetricsEngine {
    /// Parses font metrics from a TrueType font byte slice.
    pub fn parse(data: &[u8]) -> Result<Self, CoreError> {
        if data.len() < 12 {
            return Err(CoreError::NotFound("Font file too short".to_string()));
        }

        let num_tables = u16::from_be_bytes([data[4], data[5]]) as usize;
        let mut tables: HashMap<String, (usize, usize)> = HashMap::new();

        let table_dir_offset = 12;
        for i in 0..num_tables {
            let offset = table_dir_offset + i * 16;
            if offset + 16 > data.len() {
                break;
            }

            let tag = std::str::from_utf8(&data[offset..offset + 4]).unwrap_or("????").to_string();
            let tbl_offset = u32::from_be_bytes([
                data[offset + 8],
                data[offset + 9],
                data[offset + 10],
                data[offset + 11],
            ]) as usize;
            let tbl_length = u32::from_be_bytes([
                data[offset + 12],
                data[offset + 13],
                data[offset + 14],
                data[offset + 15],
            ]) as usize;

            tables.insert(tag, (tbl_offset, tbl_length));
        }

        // 1. Read 'head' table for units_per_em
        let head = tables.get("head").ok_or_else(|| CoreError::NotFound("Missing 'head' table".to_string()))?;
        if head.0 + 38 > data.len() {
            return Err(CoreError::NotFound("Invalid 'head' table length".to_string()));
        }
        let units_per_em = u16::from_be_bytes([data[head.0 + 18], data[head.0 + 19]]);

        // 2. Read 'hhea' table for ascent, descent, line_gap, and number_of_hmetrics
        let hhea = tables.get("hhea").ok_or_else(|| CoreError::NotFound("Missing 'hhea' table".to_string()))?;
        if hhea.0 + 36 > data.len() {
            return Err(CoreError::NotFound("Invalid 'hhea' table length".to_string()));
        }
        let ascent = i16::from_be_bytes([data[hhea.0 + 4], data[hhea.0 + 5]]);
        let descent = i16::from_be_bytes([data[hhea.0 + 6], data[hhea.0 + 7]]);
        let line_gap = i16::from_be_bytes([data[hhea.0 + 8], data[hhea.0 + 9]]);
        let num_hmetrics = u16::from_be_bytes([data[hhea.0 + 34], data[hhea.0 + 35]]) as usize;

        // 3. Read 'hmtx' table for glyph horizontal advances
        let hmtx = tables.get("hmtx").ok_or_else(|| CoreError::NotFound("Missing 'hmtx' table".to_string()))?;
        let mut glyph_advances = Vec::with_capacity(num_hmetrics);
        for i in 0..num_hmetrics {
            let offset = hmtx.0 + i * 4;
            if offset + 2 <= data.len() {
                let adv = u16::from_be_bytes([data[offset], data[offset + 1]]);
                glyph_advances.push(adv);
            } else {
                glyph_advances.push(0);
            }
        }

        // 4. Read 'cmap' table to map char -> glyph ID
        let mut char_to_glyph = HashMap::new();
        if let Some(cmap) = tables.get("cmap") {
            Self::parse_cmap(data, cmap.0, &mut char_to_glyph);
        }

        Ok(Self {
            units_per_em: if units_per_em == 0 { 1000 } else { units_per_em },
            ascent,
            descent,
            line_gap,
            glyph_advances,
            char_to_glyph,
        })
    }

    fn parse_cmap(data: &[u8], cmap_offset: usize, char_to_glyph: &mut HashMap<char, u16>) {
        if cmap_offset + 4 > data.len() {
            return;
        }
        let num_subtables = u16::from_be_bytes([data[cmap_offset + 2], data[cmap_offset + 3]]) as usize;

        // Search for subtable format 4 (standard Unicode BMP)
        for i in 0..num_subtables {
            let entry_offset = cmap_offset + 4 + i * 8;
            if entry_offset + 8 > data.len() {
                break;
            }
            let sub_offset = cmap_offset + u32::from_be_bytes([
                data[entry_offset + 4],
                data[entry_offset + 5],
                data[entry_offset + 6],
                data[entry_offset + 7],
            ]) as usize;

            if sub_offset + 6 > data.len() {
                continue;
            }
            let format = u16::from_be_bytes([data[sub_offset], data[sub_offset + 1]]);

            if format == 4 {
                // Parse Format 4 subtable
                if sub_offset + 14 > data.len() {
                    continue;
                }
                let seg_count_x2 = u16::from_be_bytes([data[sub_offset + 6], data[sub_offset + 7]]) as usize;
                let seg_count = seg_count_x2 / 2;

                let end_code_offset = sub_offset + 14;
                let start_code_offset = end_code_offset + seg_count * 2 + 2;
                let id_delta_offset = start_code_offset + seg_count * 2;
                let id_range_offset = id_delta_offset + seg_count * 2;

                if id_range_offset + seg_count * 2 > data.len() {
                    continue;
                }

                for s in 0..seg_count {
                    let end = u16::from_be_bytes([
                        data[end_code_offset + s * 2],
                        data[end_code_offset + s * 2 + 1],
                    ]);
                    let start = u16::from_be_bytes([
                        data[start_code_offset + s * 2],
                        data[start_code_offset + s * 2 + 1],
                    ]);
                    let id_delta = i16::from_be_bytes([
                        data[id_delta_offset + s * 2],
                        data[id_delta_offset + s * 2 + 1],
                    ]);
                    let range_offset = u16::from_be_bytes([
                        data[id_range_offset + s * 2],
                        data[id_range_offset + s * 2 + 1],
                    ]) as usize;

                    if start == 0xFFFF {
                        break;
                    }

                    for code in start..=end {
                        let glyph_id = if range_offset == 0 {
                            ((code as i32 + id_delta as i32) & 0xFFFF) as u16
                        } else {
                            let glyph_offset = id_range_offset + s * 2 + range_offset + (code - start) as usize * 2;
                            if glyph_offset + 2 <= data.len() {
                                let id = u16::from_be_bytes([data[glyph_offset], data[glyph_offset + 1]]);
                                if id != 0 {
                                    ((id as i32 + id_delta as i32) & 0xFFFF) as u16
                                } else {
                                    0
                                }
                            } else {
                                0
                            }
                        };

                        if let Some(ch) = std::char::from_u32(code as u32) {
                            char_to_glyph.insert(ch, glyph_id);
                        }
                    }
                }
                break;
            }
        }
    }

    /// Gets horizontal advance width for a given character in font design units.
    pub fn get_char_advance_units(&self, ch: char) -> u16 {
        if let Some(&glyph_id) = self.char_to_glyph.get(&ch) {
            if (glyph_id as usize) < self.glyph_advances.len() {
                return self.glyph_advances[glyph_id as usize];
            } else if let Some(&last) = self.glyph_advances.last() {
                return last;
            }
        }
        // Fallback: average advance or fraction of units_per_em
        self.units_per_em / 2
    }

    /// Measures text dimensions in pixels for a given point size.
    pub fn measure_text(&self, text: &str, font_size_pt: f32) -> TextMetrics {
        let mut total_advance_units: u32 = 0;
        let mut char_count = 0;
        let mut is_bidi = false;

        for ch in text.chars() {
            char_count += 1;
            total_advance_units += self.get_char_advance_units(ch) as u32;
            // Detect Arabic or Hebrew range for BiDi flag
            let u = ch as u32;
            if (0x0590..=0x08FF).contains(&u) || (0xFB1D..=0xFEFC).contains(&u) {
                is_bidi = true;
            }
        }

        let scale = font_size_pt / self.units_per_em as f32;
        let pixel_width = (total_advance_units as f32) * scale;
        let pixel_height = ((self.ascent - self.descent) as f32) * scale;

        TextMetrics {
            width_px: pixel_width,
            height_px: pixel_height,
            char_count,
            is_bidi,
        }
    }
}

/// Computed dimensions and metadata for measured text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMetrics {
    pub width_px: f32,
    pub height_px: f32,
    pub char_count: usize,
    pub is_bidi: bool,
}

/// Target bounding box constraints for a UI text field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiBoundingBox {
    pub max_width_px: f32,
    pub max_height_px: f32,
    pub max_lines: usize,
    pub font_size_pt: f32,
}

/// Result of evaluating text against UI boundaries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverflowReport {
    pub fits: bool,
    pub text: String,
    pub metrics: TextMetrics,
    pub bounds: UiBoundingBox,
    pub overflow_width_px: f32,
    pub overflow_percentage: f32,
    pub requires_complex_shaping: bool,
}

pub struct OverflowPredictor;

impl OverflowPredictor {
    /// Evaluates whether a string fits in the specified UI bounding box.
    pub fn evaluate(engine: &FontMetricsEngine, text: &str, bounds: &UiBoundingBox) -> OverflowReport {
        let metrics = engine.measure_text(text, bounds.font_size_pt);
        let max_w = bounds.max_width_px * (bounds.max_lines.max(1) as f32);

        let fits = metrics.width_px <= max_w && metrics.height_px <= bounds.max_height_px;
        let overflow_width_px = (metrics.width_px - max_w).max(0.0);
        let overflow_percentage = if max_w > 0.0 && !fits {
            (overflow_width_px / max_w) * 100.0
        } else {
            0.0
        };

        OverflowReport {
            fits,
            text: text.to_string(),
            metrics: metrics.clone(),
            bounds: bounds.clone(),
            overflow_width_px,
            overflow_percentage,
            requires_complex_shaping: metrics.is_bidi,
        }
    }
}
