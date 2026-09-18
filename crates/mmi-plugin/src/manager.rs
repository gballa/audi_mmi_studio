//! Plugin discovery, lifecycle management, and format adapter bridge (§17.1).

use crate::abi::*;
use crate::sandbox::{PluginBackend, PluginSandbox, SandboxError};
use mmi_core::CoreError;
use mmi_formats::{FormatAdapter, FormatCapabilities};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use walkdir::WalkDir;

/// Represents an active, verified plugin registered in the workstation.
pub struct LoadedPlugin {
    pub plugin_dir: PathBuf,
    pub manifest: PluginManifest,
    pub sandbox: PluginSandbox,
}

/// Discovers, loads, and manages third-party format plugins.
pub struct PluginManager {
    plugins: HashMap<String, Arc<LoadedPlugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    /// Registers an instantiated plugin into the manager.
    pub fn register(&mut self, plugin_dir: PathBuf, backend: Box<dyn PluginBackend>) -> Result<(), CoreError> {
        let manifest = backend.manifest().clone();
        let plugin_id = manifest.plugin_id.clone();
        let sandbox = PluginSandbox::new(backend).map_err(|e| CoreError::ImmutabilityViolation(e.to_string()))?;

        let loaded = LoadedPlugin {
            plugin_dir,
            manifest,
            sandbox,
        };

        self.plugins.insert(plugin_id, Arc::new(loaded));
        Ok(())
    }

    /// Returns list of all registered plugins.
    pub fn list_plugins(&self) -> Vec<PluginManifest> {
        self.plugins.values().map(|p| p.manifest.clone()).collect()
    }

    /// Gets a loaded plugin by its unique plugin ID.
    pub fn get_plugin(&self, plugin_id: &str) -> Option<Arc<LoadedPlugin>> {
        self.plugins.get(plugin_id).cloned()
    }

    /// Scans a directory for plugin manifests (`plugin.json`) and loads mock/declarative plugins.
    pub fn discover_from_dir<P: AsRef<Path>>(&mut self, root: P) -> Result<usize, CoreError> {
        let root = root.as_ref();
        if !root.exists() {
            return Ok(0);
        }

        let mut loaded_count = 0;
        for entry in WalkDir::new(root).max_depth(3).into_iter().filter_map(|e| e.ok()) {
            if entry.file_name() == "plugin.json" && entry.path().is_file() {
                let manifest_bytes = std::fs::read(entry.path())
                    .map_err(|e| CoreError::ImmutabilityViolation(format!("Failed to read plugin manifest: {e}")))?;
                let manifest: PluginManifest = serde_json::from_slice(&manifest_bytes)
                    .map_err(|e| CoreError::ImmutabilityViolation(format!("Invalid plugin manifest at {:?}: {e}", entry.path())))?;

                let plugin_dir = entry.path().parent().unwrap_or(root).to_path_buf();
                let backend = Box::new(DeclarativePluginBackend::new(manifest));
                self.register(plugin_dir, backend)?;
                loaded_count += 1;
            }
        }

        Ok(loaded_count)
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Declarative plugin backend for configuration-driven format plugins (e.g. signature & header mapping).
pub struct DeclarativePluginBackend {
    manifest: PluginManifest,
}

impl DeclarativePluginBackend {
    pub fn new(manifest: PluginManifest) -> Self {
        Self { manifest }
    }
}

impl PluginBackend for DeclarativePluginBackend {
    fn manifest(&self) -> &PluginManifest {
        &self.manifest
    }

    fn handle_detect(&self, request: DetectRequest) -> Result<DetectResponse, SandboxError> {
        let ext = Path::new(&request.file_name)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        let ext_matches = self.manifest.supported_extensions.iter().any(|e| e.to_lowercase() == ext);

        if ext_matches {
            Ok(DetectResponse {
                matches: true,
                confidence_score: 0.9,
                detected_format: self.manifest.target_generation.clone(),
                reason: Some(format!("Extension .{} recognized by plugin {}", ext, self.manifest.plugin_id)),
            })
        } else {
            Ok(DetectResponse {
                matches: false,
                confidence_score: 0.0,
                detected_format: "UNKNOWN".to_string(),
                reason: None,
            })
        }
    }

    fn handle_parse(&self, request: ParseRequest) -> Result<ParseResponse, SandboxError> {
        Ok(ParseResponse {
            success: true,
            fields: vec![
                ParsedField {
                    name: "PayloadSize".to_string(),
                    offset: 0,
                    length: request.payload.len() as u64,
                    value_repr: format!("{} bytes", request.payload.len()),
                }
            ],
            total_bytes_covered: request.payload.len() as u64,
            error: None,
        })
    }

    fn handle_extract(&self, request: ExtractRequest) -> Result<ExtractResponse, SandboxError> {
        Ok(ExtractResponse {
            success: true,
            members: vec![
                ExtractedMember {
                    relative_path: "extracted_payload.bin".to_string(),
                    offset: 0,
                    byte_size: request.payload.len() as u64,
                    data: request.payload,
                }
            ],
            error: None,
        })
    }
}

/// Bridge turning any registered PluginSandbox into a standard FormatAdapter.
pub struct PluginFormatAdapterBridge {
    plugin: Arc<LoadedPlugin>,
    capabilities: FormatCapabilities,
}

impl PluginFormatAdapterBridge {
    pub fn new(plugin: Arc<LoadedPlugin>) -> Self {
        let caps = &plugin.manifest.capabilities;
        let capabilities = FormatCapabilities {
            can_analyse: caps.can_detect,
            can_extract: caps.can_extract,
            can_normalise: caps.can_parse,
            can_edit: false, // Untrusted plugins cannot edit payloads directly (§18)
            can_rebuild: caps.can_rebuild,
            can_validate: true,
            can_package: false,
        };

        Self {
            plugin,
            capabilities,
        }
    }
}

impl FormatAdapter for PluginFormatAdapterBridge {
    fn format_name(&self) -> &'static str {
        Box::leak(self.plugin.manifest.plugin_id.clone().into_boxed_str())
    }

    fn capabilities(&self) -> &FormatCapabilities {
        &self.capabilities
    }

    fn detect(&self, data: &[u8]) -> bool {
        let req = DetectRequest {
            file_name: "unknown".to_string(),
            header_bytes: data[..data.len().min(512)].to_vec(),
            total_byte_size: data.len() as u64,
        };

        self.plugin.sandbox.detect(req).map(|res| res.matches).unwrap_or(false)
    }

    fn coverage_ratio(&self, data: &[u8]) -> f32 {
        if self.detect(data) {
            1.0
        } else {
            0.0
        }
    }
}
