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
