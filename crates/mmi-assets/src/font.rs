//! TrueType / OpenType font inspection and metadata extraction.

use mmi_core::CoreError;
use serde::{Deserialize, Serialize};

/// Extracted TrueType font metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FontInfo {
    pub family_name: Option<String>,
    pub sub_family_name: Option<String>,
    pub full_name: Option<String>,
    pub glyph_count: u16,
    pub tables: Vec<String>,
    pub has_arabic_coverage: bool,
}

pub struct FontInspector;

impl FontInspector {
    /// Inspects a TrueType font file buffer without full parsing or rasterization.
    pub fn inspect(data: &[u8]) -> Result<FontInfo, CoreError> {
        if data.len() < 12 {
            return Err(CoreError::NotFound("Font file too short".to_string()));
        }

        // SFNT Header
        let num_tables = u16::from_be_bytes([data[4], data[5]]) as usize;
        let mut tables = Vec::new();

        let mut name_offset = 0;
        let mut name_length = 0;
        let mut maxp_offset = 0;

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

            if tag == "name" {
                name_offset = tbl_offset;
                name_length = tbl_length;
            } else if tag == "maxp" {
                maxp_offset = tbl_offset;
            }

            tables.push(tag);
        }

        // Read glyph count from 'maxp'
        let mut glyph_count = 0;
        if maxp_offset + 6 <= data.len() && maxp_offset > 0 {
            glyph_count = u16::from_be_bytes([data[maxp_offset + 4], data[maxp_offset + 5]]);
        }

        // Read names from 'name' table
        let mut family_name = None;
        let mut sub_family_name = None;
        let mut full_name = None;

        if name_offset > 0 && name_offset + name_length <= data.len() && name_length >= 6 {
            let count = u16::from_be_bytes([data[name_offset + 2], data[name_offset + 3]]) as usize;
            let string_storage = name_offset + u16::from_be_bytes([data[name_offset + 4], data[name_offset + 5]]) as usize;

            for i in 0..count {
                let rec = name_offset + 6 + i * 12;
                if rec + 12 > data.len() {
                    break;
                }

                let platform_id = u16::from_be_bytes([data[rec], data[rec + 1]]);
                let name_id = u16::from_be_bytes([data[rec + 6], data[rec + 7]]);
                let length = u16::from_be_bytes([data[rec + 8], data[rec + 9]]) as usize;
                let str_offset = string_storage + u16::from_be_bytes([data[rec + 10], data[rec + 11]]) as usize;

                if str_offset + length <= data.len() {
                    let str_slice = &data[str_offset..str_offset + length];
                    // Handle UTF-16BE (Platform ID 0 or 3) or Latin/ASCII (Platform ID 1)
                    let text = if platform_id == 0 || platform_id == 3 {
                        let u16s: Vec<u16> = str_slice
                            .chunks_exact(2)
                            .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
                            .collect();
                        String::from_utf16_lossy(&u16s)
                    } else {
                        String::from_utf8_lossy(str_slice).to_string()
                    };

                    match name_id {
                        1 if family_name.is_none() => family_name = Some(text),
                        2 if sub_family_name.is_none() => sub_family_name = Some(text),
                        4 if full_name.is_none() => full_name = Some(text),
                        _ => {}
                    }
                }
            }
        }

        let has_arabic_coverage = tables.iter().any(|t| t == "GSUB" || t == "GPOS")
            || family_name.as_deref().unwrap_or("").to_lowercase().contains("arabic")
            || full_name.as_deref().unwrap_or("").to_lowercase().contains("arabic");

        Ok(FontInfo {
            family_name,
            sub_family_name,
            full_name,
            glyph_count,
            tables,
            has_arabic_coverage,
        })
    }
}
