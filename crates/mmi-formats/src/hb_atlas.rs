use std::io::{self, Read, Seek, SeekFrom, Write};
use crate::adapter::{FormatAdapter, FormatCapabilities};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use mmi_core::CoreError;
use serde::{Deserialize, Serialize};

pub const ATLAS_MAGIC_TAG: &[u8; 7] = b"\x06HEADER";
pub const ATLAS_HEADER_MIN_SIZE: usize = 64;
pub const ATLAS_DEFAULT_BLOCK_SIZE: u32 = 4096;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AtlasTileIndexEntry {
    pub morton_key: u64,
    pub file_offset: u32,
    pub compressed_size: u32,
    pub uncompressed_size: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtlasTile {
    pub morton_key: u64,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HbAtlasHeader {
    pub tile_block_size: u32,
    pub version_major: u16,
    pub version_minor: u16,
    pub index_offset: u32,
    pub index_size: u32,
    pub project_name: String,
    pub container_type: String,
}

#[derive(Debug, Clone)]
pub struct HbAtlas {
    pub header: HbAtlasHeader,
    pub total_size: usize,
}

impl HbAtlas {
    pub fn parse(data: &[u8]) -> Result<Self, CoreError> {
        if data.len() < ATLAS_HEADER_MIN_SIZE {
            return Err(CoreError::ImmutabilityViolation(
                "HbAtlas file too small for 64-byte header".into(),
            ));
        }

        if &data[0..7] != ATLAS_MAGIC_TAG {
            return Err(CoreError::ImmutabilityViolation(
                "Invalid HbAtlas magic signature: expected \\x06HEADER".into(),
            ));
        }

        let tile_block_size = u32::from_le_bytes([data[0x10], data[0x11], data[0x12], data[0x13]]);
        let version_major = u16::from_le_bytes([data[0x14], data[0x15]]);
        let version_minor = u16::from_le_bytes([data[0x16], data[0x17]]);
        let index_offset = u32::from_le_bytes([data[0x18], data[0x19], data[0x1a], data[0x1b]]);
        let index_size = u32::from_le_bytes([data[0x1c], data[0x1d], data[0x1e], data[0x1f]]);

        // Pascal strings at 0x20 and 0x30
        let orion_len = data[0x20] as usize;
        let project_name = if 0x21 + orion_len <= data.len() {
            String::from_utf8_lossy(&data[0x21..0x21 + orion_len]).to_string()
        } else {
            "Unknown".to_string()
        };

        let atlas_len = data[0x30] as usize;
        let container_type = if 0x31 + atlas_len <= data.len() {
            String::from_utf8_lossy(&data[0x31..0x31 + atlas_len]).to_string()
        } else {
            "Unknown".to_string()
        };

        Ok(Self {
            header: HbAtlasHeader {
                tile_block_size,
                version_major,
                version_minor,
                index_offset,
                index_size,
                project_name,
                container_type,
            },
            total_size: data.len(),
        })
    }

    /// Reads all tile index entries from the container data slice.
    pub fn parse_index(&self, data: &[u8]) -> Result<Vec<AtlasTileIndexEntry>, CoreError> {
        let offset = self.header.index_offset as usize;
        let size = self.header.index_size as usize;
        if offset + size > data.len() {
            return Err(CoreError::ImmutabilityViolation("Atlas index out of bounds".into()));
        }
        let count = size / 20;
        let mut entries = Vec::with_capacity(count);
        for i in 0..count {
            let start = offset + i * 20;
            let chunk = &data[start..start + 20];
            let morton_key = u64::from_le_bytes(chunk[0..8].try_into().unwrap());
            let file_offset = u32::from_le_bytes(chunk[8..12].try_into().unwrap());
            let compressed_size = u32::from_le_bytes(chunk[12..16].try_into().unwrap());
            let uncompressed_size = u32::from_le_bytes(chunk[16..20].try_into().unwrap());
            entries.push(AtlasTileIndexEntry {
                morton_key,
                file_offset,
                compressed_size,
                uncompressed_size,
            });
        }
        Ok(entries)
    }

    /// Reads and decompresses the raw tile payload for a given index entry.
    pub fn read_tile(&self, data: &[u8], entry: &AtlasTileIndexEntry) -> Result<Vec<u8>, CoreError> {
        let start = entry.file_offset as usize;
        let end = start + entry.compressed_size as usize;
        if end > data.len() {
            return Err(CoreError::ImmutabilityViolation("Atlas tile data out of bounds".into()));
        }
        let mut decoder = ZlibDecoder::new(&data[start..end]);
        let mut decompressed = Vec::with_capacity(entry.uncompressed_size as usize);
        decoder.read_to_end(&mut decompressed).map_err(CoreError::Io)?;
        Ok(decompressed)
    }
}

/// Serializer for Orion Atlas spatial tile containers.
pub struct AtlasWriter<W: Write + Seek> {
    writer: W,
    tile_block_size: u32,
    project_name: String,
    container_type: String,
    tiles: Vec<AtlasTileIndexEntry>,
}

impl<W: Write + Seek> AtlasWriter<W> {
    pub fn new(mut writer: W, tile_block_size: u32, project: &str, container_type: &str) -> io::Result<Self> {
        // Reserve 64 bytes for header
        let dummy_header = vec![0u8; ATLAS_HEADER_MIN_SIZE];
        writer.write_all(&dummy_header)?;

        Ok(Self {
            writer,
            tile_block_size,
            project_name: project.to_string(),
            container_type: container_type.to_string(),
            tiles: Vec::new(),
        })
    }

    pub fn write_tile(&mut self, morton_key: u64, raw_payload: &[u8]) -> io::Result<()> {
        let current_offset = self.writer.stream_position()? as u32;

        // Compress tile payload with standard zlib
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(raw_payload)?;
        let compressed = encoder.finish()?;

        // Write compressed data padded to tile_block_size
        self.writer.write_all(&compressed)?;
        let pad_len = (self.tile_block_size as usize - (compressed.len() % self.tile_block_size as usize)) % self.tile_block_size as usize;
        if pad_len > 0 {
            self.writer.write_all(&vec![0u8; pad_len])?;
        }

        self.tiles.push(AtlasTileIndexEntry {
            morton_key,
            file_offset: current_offset,
            compressed_size: compressed.len() as u32,
            uncompressed_size: raw_payload.len() as u32,
        });

        Ok(())
    }

    pub fn finalize(mut self) -> io::Result<usize> {
        let index_offset = self.writer.stream_position()? as u32;

        // Write index table (20 bytes per entry)
        for tile in &self.tiles {
            self.writer.write_all(&tile.morton_key.to_le_bytes())?;
            self.writer.write_all(&tile.file_offset.to_le_bytes())?;
            self.writer.write_all(&tile.compressed_size.to_le_bytes())?;
            self.writer.write_all(&tile.uncompressed_size.to_le_bytes())?;
        }
        let index_size = (self.tiles.len() * 20) as u32;

        // Rewind and write real 64-byte header
        self.writer.seek(SeekFrom::Start(0))?;
        let mut header = vec![0xCCu8; ATLAS_HEADER_MIN_SIZE];
        header[0..7].copy_from_slice(ATLAS_MAGIC_TAG);
        header[0x10..0x14].copy_from_slice(&self.tile_block_size.to_le_bytes());
        header[0x14..0x16].copy_from_slice(&1u16.to_le_bytes()); // major = 1
        header[0x16..0x18].copy_from_slice(&0u16.to_le_bytes()); // minor = 0
        header[0x18..0x1C].copy_from_slice(&index_offset.to_le_bytes());
        header[0x1C..0x20].copy_from_slice(&index_size.to_le_bytes());

        // Pascal string: project_name (at 0x20)
        let proj_bytes = self.project_name.as_bytes();
        let proj_len = proj_bytes.len().min(15);
        header[0x20] = proj_len as u8;
        header[0x21..0x21 + proj_len].copy_from_slice(&proj_bytes[..proj_len]);

        // Pascal string: container_type (at 0x30)
        let type_bytes = self.container_type.as_bytes();
        let type_len = type_bytes.len().min(15);
        header[0x30] = type_len as u8;
        header[0x31..0x31 + type_len].copy_from_slice(&type_bytes[..type_len]);

        self.writer.write_all(&header)?;
        self.writer.flush()?;
        Ok(self.tiles.len())
    }
}

#[derive(Debug, Clone)]
pub struct HbAtlasAdapter {
    capabilities: FormatCapabilities,
}

impl Default for HbAtlasAdapter {
    fn default() -> Self {
        Self {
            capabilities: FormatCapabilities::read_only(),
        }
    }
}

impl FormatAdapter for HbAtlasAdapter {
    fn format_name(&self) -> &'static str {
        "hb_atlas"
    }

    fn capabilities(&self) -> &FormatCapabilities {
        &self.capabilities
    }

    fn detect(&self, data: &[u8]) -> bool {
        if data.len() < ATLAS_HEADER_MIN_SIZE {
            return false;
        }
        &data[0..7] == ATLAS_MAGIC_TAG
            && data.len() > 0x35
            && &data[0x21..0x26] == b"Orion"
            && &data[0x31..0x36] == b"Atlas"
    }

    fn coverage_ratio(&self, data: &[u8]) -> f32 {
        if self.detect(data) {
            1.0
        } else {
            0.0
        }
    }
}
