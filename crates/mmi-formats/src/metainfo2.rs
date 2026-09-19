//! Parser and serializer for MMI package manifest files (metainfo2.txt).

use std::collections::HashMap;
use mmi_core::CoreError;
use serde::{Deserialize, Serialize};

use crate::adapter::{FormatAdapter, FormatCapabilities};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetaInfo2 {
    pub release: Option<String>,
    pub vendor: Option<String>,
    pub source_version: Option<String>,
    pub sections: HashMap<String, HashMap<String, String>>,
}

impl MetaInfo2 {
    /// Parses a `metainfo2.txt` string into structured key-value sections.
    pub fn parse(content: &str) -> Result<Self, CoreError> {
        let mut sections: HashMap<String, HashMap<String, String>> = HashMap::new();
        let mut current_section = "root".to_string();
        sections.insert(current_section.clone(), HashMap::new());

        let mut release = None;
        let mut vendor = None;
        let mut source_version = None;

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
                continue;
            }

            // Section header: [section_name]
            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                current_section = trimmed[1..trimmed.len() - 1].trim().to_string();
                sections.entry(current_section.clone()).or_default();
                continue;
            }

            // Key = Value
            if let Some((k, v)) = trimmed.split_once('=') {
                let key = k.trim().to_string();
                let mut val = v.trim().to_string();

                // Strip quotes if present
                if (val.starts_with('"') && val.ends_with('"'))
                    || (val.starts_with('\'') && val.ends_with('\''))
                {
                    val = val[1..val.len() - 1].to_string();
                }

                if current_section == "root" || current_section == "common" {
                    match key.as_str() {
                        "release" | "Release" => release = Some(val.clone()),
                        "vendor" | "Vendor" => vendor = Some(val.clone()),
                        "sourceVersion" | "SourceVersion" => source_version = Some(val.clone()),
                        _ => {}
                    }
                }

                sections
                    .entry(current_section.clone())
                    .or_default()
                    .insert(key, val);
            }
        }

        Ok(Self {
            release,
            vendor,
            source_version,
            sections,
        })
    }
}

pub struct MetaInfo2Adapter {
    capabilities: FormatCapabilities,
}

impl Default for MetaInfo2Adapter {
    fn default() -> Self {
        Self {
            capabilities: FormatCapabilities::full_support(),
        }
    }
}

impl FormatAdapter for MetaInfo2Adapter {
    fn format_name(&self) -> &'static str {
        "metainfo2"
    }

    fn capabilities(&self) -> &FormatCapabilities {
        &self.capabilities
    }

    fn detect(&self, data: &[u8]) -> bool {
        if let Ok(text) = std::str::from_utf8(data) {
            (text.contains("release") || text.contains("Release"))
                && (text.contains("vendor") || text.contains("Vendor") || text.contains('['))
        } else {
            false
        }
    }

    fn coverage_ratio(&self, data: &[u8]) -> f32 {
        if self.detect(data) {
            1.0
        } else {
            0.0
        }
    }
}

pub const CRC32_BLOCK_SIZE: usize = 524_288; // 512 KiB per SWDL specification

/// Standard IEEE 802.3 CRC32 calculation matching QNX SWDL.
pub fn crc32_ieee(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            if (crc & 1) != 0 {
                crc = (crc >> 1) ^ 0xEDB8_8320;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

/// Generates per-512KB CRC32 entries for a given file byte buffer.
pub fn generate_block_crcs(data: &[u8]) -> Vec<u32> {
    let mut crcs = Vec::new();
    let mut offset = 0;
    while offset < data.len() {
        let end = (offset + CRC32_BLOCK_SIZE).min(data.len());
        crcs.push(crc32_ieee(&data[offset..end]));
        offset = end;
    }
    crcs
}

/// Builder for constructing valid Harman/Becker metainfo2.txt manifests.
#[derive(Debug, Clone, Default)]
pub struct MetaInfo2Builder {
    pub release: String,
    pub vendor: String,
    pub compatible_trains: String,
    pub variant: String,
    pub sections: Vec<(String, Vec<(String, String)>)>,
}

impl MetaInfo2Builder {
    pub fn new(release: &str, train: &str) -> Self {
        Self {
            release: release.to_string(),
            vendor: "Harman/Becker".to_string(),
            compatible_trains: train.to_string(),
            variant: "9411".to_string(),
            sections: Vec::new(),
        }
    }

    pub fn add_section(&mut self, name: &str, entries: Vec<(&str, &str)>) {
        let converted: Vec<(String, String)> = entries
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        self.sections.push((name.to_string(), converted));
    }

    pub fn add_binary_with_blocks(&mut self, section_name: &str, path: &str, data: &[u8]) {
        let mut entries = vec![
            ("path".to_string(), path.to_string()),
            ("fileSize".to_string(), data.len().to_string()),
        ];

        let block_crcs = generate_block_crcs(data);
        for (i, crc) in block_crcs.iter().enumerate() {
            entries.push((format!("CheckSum.{}", i + 1), format!("0x{:08X}", crc)));
        }

        self.sections.push((section_name.to_string(), entries));
    }

    pub fn build(&self) -> String {
        let mut out = String::new();
        out.push_str("[common]\n");
        out.push_str(&format!("release = \"{}\"\n", self.release));
        out.push_str(&format!("vendor = \"{}\"\n", self.vendor));
        out.push_str("sourceVersion = \"K0942_4\"\n");
        out.push_str(&format!("compatibleTrains = \"{}\"\n", self.compatible_trains));
        out.push_str(&format!("variant = \"{}\"\n\n", self.variant));

        for (sec_name, entries) in &self.sections {
            out.push_str(&format!("[{}]\n", sec_name));
            for (k, v) in entries {
                out.push_str(&format!("{} = \"{}\"\n", k, v));
            }
            out.push('\n');
        }

        out
    }
}

