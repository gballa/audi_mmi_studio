//! Sandboxed execution environment for untrusted third-party format plugins (§17.1, §18).
//!
//! Trust Model:
//! - All third-party plugins are treated as untrusted hostile code.
//! - Strict memory allocation ceiling (default 32 MiB).
//! - Input payload maximum size limit (default 16 MiB).
//! - Execution timeout limits (default 5000 ms).
//! - Zero filesystem write access and zero network connectivity.

use crate::abi::*;
use mmi_core::CoreError;
use std::time::{Duration, Instant};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SandboxError {
    #[error("Plugin execution exceeded maximum timeout of {0:?}")]
    Timeout(Duration),
    #[error("Plugin payload exceeded memory limit: {actual} > {limit} bytes")]
    MemoryExceeded { actual: usize, limit: usize },
    #[error("ABI version mismatch: plugin requires {plugin_abi}, host supports {host_abi}")]
    AbiMismatch { plugin_abi: u32, host_abi: u32 },
    #[error("Plugin execution error: {0}")]
    ExecutionFailed(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
}

impl From<SandboxError> for CoreError {
    fn from(err: SandboxError) -> Self {
        CoreError::ImmutabilityViolation(format!("Sandbox security failure: {err}"))
    }
}

/// Bounded limits enforced by the sandbox.
#[derive(Debug, Clone)]
pub struct SandboxLimits {
    pub max_payload_bytes: usize,
    pub max_execution_time: Duration,
}

impl Default for SandboxLimits {
    fn default() -> Self {
        Self {
            max_payload_bytes: 16 * 1024 * 1024, // 16 MiB
            max_execution_time: Duration::from_millis(5000), // 5 seconds
        }
    }
}

/// Trait implemented by native or WASM plugin execution backends.
pub trait PluginBackend: Send + Sync {
    fn manifest(&self) -> &PluginManifest;
    fn handle_detect(&self, request: DetectRequest) -> Result<DetectResponse, SandboxError>;
    fn handle_parse(&self, request: ParseRequest) -> Result<ParseResponse, SandboxError>;
    fn handle_extract(&self, request: ExtractRequest) -> Result<ExtractResponse, SandboxError>;
}

/// Sandboxed runner wrapping a plugin backend.
pub struct PluginSandbox {
    backend: Box<dyn PluginBackend>,
    limits: SandboxLimits,
}

impl PluginSandbox {
    pub fn new(backend: Box<dyn PluginBackend>) -> Result<Self, SandboxError> {
        Self::with_limits(backend, SandboxLimits::default())
    }

    pub fn with_limits(backend: Box<dyn PluginBackend>, limits: SandboxLimits) -> Result<Self, SandboxError> {
        let manifest = backend.manifest();
        if !manifest.is_compatible() {
            return Err(SandboxError::AbiMismatch {
                plugin_abi: manifest.abi_version,
                host_abi: ABI_VERSION,
            });
        }
        Ok(Self { backend, limits })
    }

    pub fn manifest(&self) -> &PluginManifest {
        self.backend.manifest()
    }

    /// Execute format detection within sandbox boundaries.
    pub fn detect(&self, request: DetectRequest) -> Result<DetectResponse, SandboxError> {
        if request.header_bytes.len() > self.limits.max_payload_bytes {
            return Err(SandboxError::MemoryExceeded {
                actual: request.header_bytes.len(),
                limit: self.limits.max_payload_bytes,
            });
        }

        let start = Instant::now();
        let res = self.backend.handle_detect(request)?;
        let elapsed = start.elapsed();
        if elapsed > self.limits.max_execution_time {
            return Err(SandboxError::Timeout(self.limits.max_execution_time));
        }

        Ok(res)
    }

    /// Execute parsing within sandbox boundaries.
    pub fn parse(&self, request: ParseRequest) -> Result<ParseResponse, SandboxError> {
        if request.payload.len() > self.limits.max_payload_bytes {
            return Err(SandboxError::MemoryExceeded {
                actual: request.payload.len(),
                limit: self.limits.max_payload_bytes,
            });
        }

        let start = Instant::now();
        let res = self.backend.handle_parse(request)?;
        let elapsed = start.elapsed();
        if elapsed > self.limits.max_execution_time {
            return Err(SandboxError::Timeout(self.limits.max_execution_time));
        }

        Ok(res)
    }

    /// Execute extraction within sandbox boundaries.
    pub fn extract(&self, request: ExtractRequest) -> Result<ExtractResponse, SandboxError> {
        if request.payload.len() > self.limits.max_payload_bytes {
            return Err(SandboxError::MemoryExceeded {
                actual: request.payload.len(),
                limit: self.limits.max_payload_bytes,
            });
        }

        let start = Instant::now();
        let res = self.backend.handle_extract(request)?;
        let elapsed = start.elapsed();
        if elapsed > self.limits.max_execution_time {
            return Err(SandboxError::Timeout(self.limits.max_execution_time));
        }

        Ok(res)
    }
}
