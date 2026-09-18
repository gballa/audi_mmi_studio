//! Stable serialization ABI and message protocol for third-party format plugins (§17.1).
//!
//! Enables adding support for new MMI hardware generations (MIB1, MIB2, MIB3)
//! or proprietary container formats without modifying the core codebase.

use serde::{Deserialize, Serialize};

/// Current ABI specification version.
pub const ABI_VERSION: u32 = 1;

/// Declared capabilities of a third-party format plugin.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginCapabilities {
    pub can_detect: bool,
    pub can_parse: bool,
    pub can_extract: bool,
    pub can_rebuild: bool,
}

impl Default for PluginCapabilities {
    fn default() -> Self {
        Self {
            can_detect: true,
            can_parse: true,
            can_extract: false,
            can_rebuild: false,
        }
    }
}

/// Metadata and registration manifest for a format plugin.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginManifest {
    pub abi_version: u32,
    pub plugin_id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub target_generation: String, // e.g. "MIB2_HIGH", "MIB3", "MMI_3G"
    pub supported_extensions: Vec<String>,
    pub capabilities: PluginCapabilities,
}

impl PluginManifest {
    pub fn is_compatible(&self) -> bool {
        self.abi_version == ABI_VERSION
    }
}

/// Request sent to plugin to detect if binary data matches its format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectRequest {
    pub file_name: String,
    pub header_bytes: Vec<u8>,
    pub total_byte_size: u64,
}

/// Response returned by plugin after format detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectResponse {
    pub matches: bool,
    pub confidence_score: f32, // 0.0 to 1.0
    pub detected_format: String,
    pub reason: Option<String>,
}

/// Request sent to plugin to parse structural entities from binary data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseRequest {
    pub file_name: String,
    pub payload: Vec<u8>,
}

/// Discovered named field in container structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedField {
    pub name: String,
    pub offset: u64,
    pub length: u64,
    pub value_repr: String,
}

/// Response returned by plugin with structural parsing details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseResponse {
    pub success: bool,
    pub fields: Vec<ParsedField>,
    pub total_bytes_covered: u64,
    pub error: Option<String>,
}

/// Request sent to plugin to list and extract container members.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractRequest {
    pub file_name: String,
    pub payload: Vec<u8>,
}

/// Extracted member item descriptor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedMember {
    pub relative_path: String,
    pub offset: u64,
    pub byte_size: u64,
    pub data: Vec<u8>,
}

/// Response returned by plugin with extracted members.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractResponse {
    pub success: bool,
    pub members: Vec<ExtractedMember>,
    pub error: Option<String>,
}
