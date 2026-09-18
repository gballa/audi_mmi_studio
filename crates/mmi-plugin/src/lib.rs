//! mmi-plugin: Third-party format adapter SDK, versioned ABI, sandboxing, and discovery (§17.1).

pub mod abi;
pub mod sandbox;
pub mod manager;

pub use abi::{
    ABI_VERSION, DetectRequest, DetectResponse, ExtractRequest, ExtractResponse, ExtractedMember,
    ParseRequest, ParseResponse, ParsedField, PluginCapabilities, PluginManifest,
};
pub use sandbox::{PluginBackend, PluginSandbox, SandboxError, SandboxLimits};
pub use manager::{DeclarativePluginBackend, LoadedPlugin, PluginFormatAdapterBridge, PluginManager};
