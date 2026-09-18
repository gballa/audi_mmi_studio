//! Virtualized hex viewer and formatted row emitter.

use serde::{Deserialize, Serialize};

/// A single formatted row of 16 bytes in hex and ASCII.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HexRow {
    pub offset: usize,
    pub hex_bytes: Vec<String>,
    pub ascii: String,
}

/// Formats a byte slice into structured, virtualized hex view rows.
pub struct HexViewer;

impl HexViewer {
    /// Renders a window of bytes `[offset .. offset + length]` into `HexRow` structures.
    pub fn render_slice(data: &[u8], base_offset: usize, bytes_per_row: usize) -> Vec<HexRow> {
        let stride = if bytes_per_row == 0 { 16 } else { bytes_per_row };
        let mut rows = Vec::new();

        for (i, chunk) in data.chunks(stride).enumerate() {
            let row_offset = base_offset + (i * stride);
            let hex_bytes = chunk.iter().map(|b| format!("{:02X}", b)).collect();

            let ascii = chunk
                .iter()
                .map(|&b| {
                    if (0x20..=0x7E).contains(&b) {
                        b as char
                    } else {
                        '.'
                    }
                })
                .collect();

            rows.push(HexRow {
                offset: row_offset,
                hex_bytes,
                ascii,
            });
        }

        rows
    }

    /// Formats a slice into a standard debug string (similar to `hexdump -C`).
    pub fn format_hexdump(data: &[u8], base_offset: usize) -> String {
        let rows = Self::render_slice(data, base_offset, 16);
        let mut out = String::new();

        for row in rows {
            out.push_str(&format!("{:08X}  ", row.offset));

            // Left octet
            for j in 0..8 {
                if let Some(b) = row.hex_bytes.get(j) {
                    out.push_str(b);
                    out.push(' ');
                } else {
                    out.push_str("   ");
                }
            }
            out.push(' ');

            // Right octet
            for j in 8..16 {
                if let Some(b) = row.hex_bytes.get(j) {
                    out.push_str(b);
                    out.push(' ');
                } else {
                    out.push_str("   ");
                }
            }

            out.push_str(&format!(" |{}|\n", row.ascii));
        }

        out
    }
}
