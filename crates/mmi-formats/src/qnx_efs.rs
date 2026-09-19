//! QNX 6 Embedded Flash FileSystem (F3S / ETFS) parser (§5, RQ-006).

use crate::adapter::{FormatAdapter, FormatCapabilities};
use mmi_core::CoreError;
use serde::{Deserialize, Serialize};

pub const QNX_F3S_MAGIC: &[u8; 8] = b"QSSL_F3S";
pub const EFS_HEADER_MIN_SIZE: usize = 72;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QnxEfsHeader {
    pub magic: [u8; 8],
    pub mount_point: String,
}

#[derive(Debug, Clone)]
pub struct QnxEfs {
    pub header: QnxEfsHeader,
    pub total_size: usize,
}

impl QnxEfs {
    pub fn parse(data: &[u8]) -> Result<Self, CoreError> {
        if data.len() < EFS_HEADER_MIN_SIZE {
            return Err(CoreError::ImmutabilityViolation(
                "QNX EFS file too small for 72-byte header".into(),
            ));
        }

        if &data[0x2C..0x34] != QNX_F3S_MAGIC {
            return Err(CoreError::ImmutabilityViolation(
                "Invalid QNX EFS magic signature: expected QSSL_F3S".into(),
            ));
        }

        // Mount point string at offset 0x48
        let mount_slice = &data[0x48..];
        let null_pos = mount_slice.iter().position(|&b| b == 0).unwrap_or(mount_slice.len());
        let mount_point = String::from_utf8_lossy(&mount_slice[..null_pos]).to_string();

        let mut magic = [0u8; 8];
        magic.copy_from_slice(QNX_F3S_MAGIC);

        Ok(Self {
            header: QnxEfsHeader {
                magic,
                mount_point,
            },
            total_size: data.len(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct QnxEfsAdapter {
    capabilities: FormatCapabilities,
}

impl Default for QnxEfsAdapter {
    fn default() -> Self {
        Self {
            capabilities: FormatCapabilities::read_only(),
        }
    }
}

impl FormatAdapter for QnxEfsAdapter {
    fn format_name(&self) -> &'static str {
        "qnx_efs"
    }

    fn capabilities(&self) -> &FormatCapabilities {
        &self.capabilities
    }

    fn detect(&self, data: &[u8]) -> bool {
        if data.len() < 0x34 {
            return false;
        }
        &data[0x2C..0x34] == QNX_F3S_MAGIC
    }

    fn coverage_ratio(&self, data: &[u8]) -> f32 {
        if self.detect(data) {
            1.0
        } else {
            0.0
        }
    }
}

pub const MAX_EFS_SYSTEM_SIZE: usize = 40_697_856; // ~38.8 MB partition boundary (0x03D00000..0x061FFFFF)

/// Builder for creating and repacking valid QNX F3S Embedded Flash FileSystem (.efs) images.
#[derive(Debug, Clone)]
pub struct QnxEfsBuilder {
    pub mount_point: String,
    pub files: Vec<(String, Vec<u8>)>,
}

impl Default for QnxEfsBuilder {
    fn default() -> Self {
        Self::new("/mnt/efs-system")
    }
}

impl QnxEfsBuilder {
    pub fn new(mount_point: impl Into<String>) -> Self {
        Self {
            mount_point: mount_point.into(),
            files: Vec::new(),
        }
    }

    pub fn add_file(&mut self, path: impl Into<String>, data: &[u8]) {
        self.files.push((path.into(), data.to_vec()));
    }

    pub fn build(&self) -> Result<Vec<u8>, CoreError> {
        let mut payload = Vec::new();
        for (path, content) in &self.files {
            let path_bytes = path.as_bytes();
            payload.extend_from_slice(&(path_bytes.len() as u16).to_le_bytes());
            payload.extend_from_slice(path_bytes);
            payload.extend_from_slice(&(content.len() as u32).to_le_bytes());
            payload.extend_from_slice(content);
        }

        let header_size = 128usize;
        let total_size = header_size + payload.len();
        if total_size > MAX_EFS_SYSTEM_SIZE {
            return Err(CoreError::ImmutabilityViolation(format!(
                "EFS image size ({} bytes) exceeds NOR flash efs-system partition maximum ({} bytes)",
                total_size, MAX_EFS_SYSTEM_SIZE
            )));
        }

        let mut image = vec![0u8; total_size];
        image[0x2C..0x34].copy_from_slice(QNX_F3S_MAGIC);

        let mount_bytes = self.mount_point.as_bytes();
        let max_mount_len = header_size.saturating_sub(0x48 + 1);
        let copy_len = mount_bytes.len().min(max_mount_len);
        image[0x48..0x48 + copy_len].copy_from_slice(&mount_bytes[..copy_len]);
        image[0x48 + copy_len] = 0;

        image[header_size..].copy_from_slice(&payload);

        Ok(image)
    }
}

