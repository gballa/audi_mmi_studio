//! Multi-encoding string catalog parser and editor for Audi MMI systems.
//!
//! Handles diverse text encodings encountered across the MMI 3G High / Plus corpus:
//! - ASCII
//! - UTF-8
//! - UTF-16LE / UTF-16BE (with or without BOM)
//! - ISO-8859-1 (Latin-1)

use mmi_core::CoreError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Detected or specified text encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncodingType {
    Ascii,
    Utf8,
    Utf16Le,
    Utf16Be,
    Latin1,
}

impl std::fmt::Display for EncodingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ascii => write!(f, "ASCII"),
            Self::Utf8 => write!(f, "UTF-8"),
            Self::Utf16Le => write!(f, "UTF-16LE"),
            Self::Utf16Be => write!(f, "UTF-16BE"),
            Self::Latin1 => write!(f, "ISO-8859-1 (Latin-1)"),
        }
    }
}

/// A parsed entry from a string catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StringEntry {
    /// Zero-based line or sequential index.
    pub index: usize,
    /// Optional key or label (e.g. from key=value or tab-delimited records).
    pub key: Option<String>,
    /// The decoded text content.
    pub value: String,
    /// Raw unparsed line or segment.
    pub raw: String,
}

/// A string catalog holding localized text entries and encoding provenance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringCatalog {
    pub encoding: EncodingType,
    pub entries: Vec<StringEntry>,
    pub metadata: HashMap<String, String>,
}

impl StringCatalog {
    /// Detects encoding and parses raw bytes into a `StringCatalog`.
    pub fn parse(bytes: &[u8]) -> Result<Self, CoreError> {
        let encoding = Self::detect_encoding(bytes);
        let decoded = Self::decode_bytes(bytes, encoding)?;
        let mut entries = Vec::new();

        for (idx, line) in decoded.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Check for key-value pair separated by '=' or tab '\t'
            let (key, value) = if let Some(pos) = line.find('=') {
                let k = line[..pos].trim().to_string();
                let v = line[pos + 1..].trim().to_string();
                (Some(k), v)
            } else if let Some(pos) = line.find('\t') {
                let k = line[..pos].trim().to_string();
                let v = line[pos + 1..].trim().to_string();
                (Some(k), v)
            } else {
                (None, line.to_string())
            };

            entries.push(StringEntry {
                index: idx,
                key,
                value,
                raw: line.to_string(),
            });
        }

        Ok(Self {
            encoding,
            entries,
            metadata: HashMap::new(),
        })
    }

    /// Detects the probable encoding of a byte slice.
    pub fn detect_encoding(bytes: &[u8]) -> EncodingType {
        if bytes.len() >= 2 {
            if bytes[0] == 0xFF && bytes[1] == 0xFE {
                return EncodingType::Utf16Le;
            }
            if bytes[0] == 0xFE && bytes[1] == 0xFF {
                return EncodingType::Utf16Be;
            }
        }

        if bytes.len() >= 3 && bytes[0] == 0xEF && bytes[1] == 0xBB && bytes[2] == 0xBF {
            return EncodingType::Utf8;
        }

        // Check for ASCII null alternating pattern typical of UTF-16LE without BOM
        if bytes.len() >= 4 {
            let mut nulls_even = 0;
            let mut nulls_odd = 0;
            let check_len = bytes.len().min(128);
            for i in 0..check_len {
                if bytes[i] == 0x00 {
                    if i % 2 == 0 {
                        nulls_even += 1;
                    } else {
                        nulls_odd += 1;
                    }
                }
            }
            if nulls_odd > 10 && nulls_even == 0 {
                return EncodingType::Utf16Le;
            }
            if nulls_even > 10 && nulls_odd == 0 {
                return EncodingType::Utf16Be;
            }
        }

        // Try validating as strict UTF-8
        if std::str::from_utf8(bytes).is_ok() {
            // Check if purely ASCII
            if bytes.iter().all(|&b| b < 0x80) {
                return EncodingType::Ascii;
            }
            return EncodingType::Utf8;
        }

        // Fallback: ISO-8859-1 (Latin-1) which maps every single byte 0x00-0xFF
        EncodingType::Latin1
    }

    /// Decodes bytes according to the given encoding.
    pub fn decode_bytes(bytes: &[u8], encoding: EncodingType) -> Result<String, CoreError> {
        match encoding {
            EncodingType::Ascii => {
                let s = bytes
                    .iter()
                    .map(|&b| if b < 0x80 { b as char } else { '?' })
                    .collect::<String>();
                Ok(s)
            }
            EncodingType::Utf8 => {
                let slice = if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
                    &bytes[3..]
                } else {
                    bytes
                };
                String::from_utf8(slice.to_vec())
                    .map_err(|e| CoreError::NotFound(format!("Invalid UTF-8: {e}")))
            }
            EncodingType::Utf16Le => {
                let slice = if bytes.starts_with(&[0xFF, 0xFE]) {
                    &bytes[2..]
                } else {
                    bytes
                };
                let u16s: Vec<u16> = slice
                    .chunks_exact(2)
                    .map(|c| u16::from_le_bytes([c[0], c[1]]))
                    .collect();
                Ok(String::from_utf16_lossy(&u16s))
            }
            EncodingType::Utf16Be => {
                let slice = if bytes.starts_with(&[0xFE, 0xFF]) {
                    &bytes[2..]
                } else {
                    bytes
                };
                let u16s: Vec<u16> = slice
                    .chunks_exact(2)
                    .map(|c| u16::from_be_bytes([c[0], c[1]]))
                    .collect();
                Ok(String::from_utf16_lossy(&u16s))
            }
            EncodingType::Latin1 => {
                // ISO-8859-1 directly maps 1:1 to Unicode scalar values U+0000 to U+00FF
                let s = bytes.iter().map(|&b| b as char).collect::<String>();
                Ok(s)
            }
        }
    }

    /// Serializes catalog back to bytes using its declared encoding.
    pub fn serialize(&self) -> Result<Vec<u8>, CoreError> {
        let mut full_text = String::new();
        for entry in &self.entries {
            if let Some(key) = &entry.key {
                full_text.push_str(key);
                full_text.push('=');
                full_text.push_str(&entry.value);
            } else {
                full_text.push_str(&entry.value);
            }
            full_text.push('\n');
        }

        match self.encoding {
            EncodingType::Ascii => Ok(full_text
                .chars()
                .map(|c| if (c as u32) < 128 { c as u8 } else { b'?' })
                .collect()),
            EncodingType::Utf8 => Ok(full_text.into_bytes()),
            EncodingType::Utf16Le => {
                let mut out = Vec::new();
                out.extend_from_slice(&[0xFF, 0xFE]); // BOM
                for c in full_text.encode_utf16() {
                    out.extend_from_slice(&c.to_le_bytes());
                }
                Ok(out)
            }
            EncodingType::Utf16Be => {
                let mut out = Vec::new();
                out.extend_from_slice(&[0xFE, 0xFF]); // BOM
                for c in full_text.encode_utf16() {
                    out.extend_from_slice(&c.to_be_bytes());
                }
                Ok(out)
            }
            EncodingType::Latin1 => {
                let mut out = Vec::new();
                for c in full_text.chars() {
                    let code = c as u32;
                    if code <= 0xFF {
                        out.push(code as u8);
                    } else {
                        out.push(b'?');
                    }
                }
                Ok(out)
            }
        }
    }
}
