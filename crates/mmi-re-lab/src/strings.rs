//! Multi-encoding string extraction with byte offset tracking.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StringEncoding {
    Ascii,
    Utf8,
    Utf16Le,
    Latin1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtractedString {
    pub offset: usize,
    pub length: usize,
    pub encoding: StringEncoding,
    pub value: String,
}

pub struct StringExtractor {
    pub min_length: usize,
}

impl Default for StringExtractor {
    fn default() -> Self {
        Self { min_length: 4 }
    }
}

impl StringExtractor {
    pub fn new(min_length: usize) -> Self {
        Self { min_length: if min_length == 0 { 4 } else { min_length } }
    }

    /// Extracts ASCII and UTF-8 strings from data with byte offset coordinates.
    pub fn extract_ascii_utf8(&self, data: &[u8]) -> Vec<ExtractedString> {
        let mut results = Vec::new();
        let mut start: Option<usize> = None;

        for (idx, &b) in data.iter().enumerate() {
            let is_printable = (0x20..=0x7E).contains(&b) || b == b'\t';
            if is_printable {
                if start.is_none() {
                    start = Some(idx);
                }
            } else if let Some(s) = start {
                let len = idx - s;
                if len >= self.min_length {
                    if let Ok(valid_str) = std::str::from_utf8(&data[s..idx]) {
                        results.push(ExtractedString {
                            offset: s,
                            length: len,
                            encoding: StringEncoding::Ascii,
                            value: valid_str.to_string(),
                        });
                    }
                }
                start = None;
            }
        }

        // Handle trailing string
        if let Some(s) = start {
            let len = data.len() - s;
            if len >= self.min_length {
                if let Ok(valid_str) = std::str::from_utf8(&data[s..]) {
                    results.push(ExtractedString {
                        offset: s,
                        length: len,
                        encoding: StringEncoding::Ascii,
                        value: valid_str.to_string(),
                    });
                }
            }
        }

        results
    }

    /// Extracts 16-bit Little-Endian Unicode strings (UTF-16LE).
    pub fn extract_utf16le(&self, data: &[u8]) -> Vec<ExtractedString> {
        let mut results = Vec::new();
        let mut current_chars = Vec::new();
        let mut start_offset: Option<usize> = None;

        for i in (0..data.len().saturating_sub(1)).step_by(2) {
            let code_unit = u16::from_le_bytes([data[i], data[i + 1]]);
            let is_printable_u16 = (0x20..=0x7E).contains(&code_unit) || code_unit == 0x09;

            if is_printable_u16 {
                if start_offset.is_none() {
                    start_offset = Some(i);
                }
                current_chars.push(code_unit);
            } else {
                if let Some(start) = start_offset {
                    if current_chars.len() >= self.min_length {
                        if let Ok(val) = String::from_utf16(&current_chars) {
                            results.push(ExtractedString {
                                offset: start,
                                length: current_chars.len() * 2,
                                encoding: StringEncoding::Utf16Le,
                                value: val,
                            });
                        }
                    }
                    start_offset = None;
                    current_chars.clear();
                }
            }
        }

        results
    }

    /// Combined extraction across all supported encodings.
    pub fn extract_all(&self, data: &[u8]) -> Vec<ExtractedString> {
        let mut all = self.extract_ascii_utf8(data);
        all.extend(self.extract_utf16le(data));
        all.sort_by_key(|s| s.offset);
        all
    }
}
