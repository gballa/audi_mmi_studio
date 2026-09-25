//! QNX 6 Image FileSystem (IFS) parser and synthesizer (§5, RQ-005).
//!
//! Provides bit-accurate parsing and creation of bootable QNX Neutrino IFS images
//! targeting Renesas SH-4 architecture (`machine: 0x0006` or `0x002a`), including:
//! - Full 64-byte packed startup_header with correct field offsets (startup_size at offset 32, stored_size at offset 36)
//! - `ImageHeader` ("imagefs" container), directory inodes (`ImageDirent`, `ImageAttr`)
//! - 4KB page alignment for file payloads
//! - Two's complement trailer checksum ensuring entire imagefs sums to 0
//! - RAW, ZLIB, and LZO compression chunk streams
//! - Strict NOR flash ifs-root partition maximum enforcement (45,875,200 bytes / 43.75 MB)

use std::collections::BTreeMap;
use std::io::{Read, Write};
use crate::adapter::{FormatAdapter, FormatCapabilities};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use mmi_core::CoreError;
use serde::{Deserialize, Serialize};

pub const QNX_IFS_MAGIC: [u8; 4] = [0xeb, 0x7e, 0xff, 0x00];
pub const QNX_IFS_SIGNATURE: u32 = 0x00ff_7eeb;
pub const IFS_HEADER_MIN_SIZE: usize = 64;
pub const MAX_IFS_ROOT_SIZE: usize = 45_875_200; // 43.75 MB partition boundary (0x00680000..0x030FFFFF)
pub const QNX_MACHINE_SH4: u16 = 0x0006; // Renesas SH-4 (SH7785) architecture
pub const QNX_MACHINE_SH4_ALT: u16 = 0x002a; // 42 decimal, alternate SH architecture in ELF

pub const IMAGE_SIG: &[u8; 7] = b"imagefs";

pub const FLAGS1_BIGENDIAN: u8 = 0x01;
pub const FLAGS1_VIRTUAL: u8 = 0x02;
pub const FLAGS1_COMPRESS_MASK: u8 = 0x1c;
pub const FLAGS1_COMPRESS_SHIFT: u8 = 2;

pub const S_IFMT: u32 = 0xf000;
pub const S_IFREG: u32 = 0x8000;
pub const S_IFDIR: u32 = 0x4000;
pub const S_IFLNK: u32 = 0xa000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum CompressionType {
    None = 0,
    Zlib = 1,
    Lzo = 2,
    Ucl = 3,
}

impl CompressionType {
    pub fn from_flags1(flags1: u8) -> Self {
        match (flags1 & FLAGS1_COMPRESS_MASK) >> FLAGS1_COMPRESS_SHIFT {
            1 => CompressionType::Zlib,
            2 => CompressionType::Lzo,
            3 => CompressionType::Ucl,
            _ => CompressionType::None,
        }
    }

    pub fn to_flags1_bits(self) -> u8 {
        (self as u8) << FLAGS1_COMPRESS_SHIFT
    }
}

/// Bit-accurate QNX Neutrino 64-byte startup header.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QnxIfsHeader {
    pub magic: [u8; 4],
    pub version: u16,
    pub flags1: u8,
    pub flags2: u8,
    pub flags: u16, // Compatibility: flags1 | (flags2 << 8)
    pub header_size: u16,
    pub machine_type: u16,
    pub startup_vaddr: u32,
    pub paddr_bias: u32,
    pub image_paddr: u32,
    pub ram_paddr: u32,
    pub ram_size: u32,
    pub startup_size: u32,
    pub stored_size: u32,
    pub imagefs_paddr: u32,
    pub imagefs_size: u32,
    pub image_size: u32, // Compatibility alias to imagefs_size
    pub preboot_size: u16,
    pub compression: CompressionType,
}

/// QNX `image_header` structure within the decompressed `imagefs` container.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageHeader {
    pub flags: u8,
    pub image_size: u32,
    pub hdr_dir_size: u32,
    pub dir_offset: u32,
    pub boot_ino: [u32; 4],
    pub script_ino: u32,
    pub chain_paddr: u32,
    pub mountflags: u32,
    pub mountpoint: String,
}

/// QNX 24-byte `image_attr` inode prefix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageAttr {
    pub size: u16,
    pub extattr_offset: u16,
    pub ino: u32,
    pub mode: u32,
    pub gid: u32,
    pub uid: u32,
    pub mtime: u32,
}

/// Directory entry in a QNX IFS image.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageDirent {
    pub attr: ImageAttr,
    pub path: String,
    pub file_offset: Option<u32>,
    pub file_size: Option<u32>,
    pub symlink_target: Option<String>,
}

#[derive(Debug, Clone)]
pub struct QnxIfs {
    pub header: QnxIfsHeader,
    pub total_size: usize,
    pub image_header: Option<ImageHeader>,
    pub entries: Vec<ImageDirent>,
}

impl QnxIfs {
    /// Parses a QNX IFS binary or slice.
    pub fn parse(data: &[u8]) -> Result<Self, CoreError> {
        if data.len() < IFS_HEADER_MIN_SIZE {
            return Err(CoreError::ImmutabilityViolation(format!(
                "QNX IFS data too small for 64-byte header: got {} bytes",
                data.len()
            )));
        }

        if &data[0..4] != &QNX_IFS_MAGIC {
            return Err(CoreError::ImmutabilityViolation(
                "Invalid QNX IFS magic signature: expected 0x00ff7eeb (eb 7e ff 00)".into(),
            ));
        }

        let version = u16::from_le_bytes([data[4], data[5]]);
        let flags1 = data[6];
        let flags2 = data[7];
        let flags = (flags1 as u16) | ((flags2 as u16) << 8);
        let header_size = u16::from_le_bytes([data[8], data[9]]);
        let machine_type = u16::from_le_bytes([data[10], data[11]]);
        let startup_vaddr = u32::from_le_bytes([data[12], data[13], data[14], data[15]]);
        let paddr_bias = u32::from_le_bytes([data[16], data[17], data[18], data[19]]);
        let image_paddr = u32::from_le_bytes([data[20], data[21], data[22], data[23]]);
        let ram_paddr = u32::from_le_bytes([data[24], data[25], data[26], data[27]]);
        let ram_size = u32::from_le_bytes([data[28], data[29], data[30], data[31]]);
        let startup_size = u32::from_le_bytes([data[32], data[33], data[34], data[35]]);
        let stored_size = u32::from_le_bytes([data[36], data[37], data[38], data[39]]);
        let imagefs_paddr = u32::from_le_bytes([data[40], data[41], data[42], data[43]]);
        let imagefs_size = u32::from_le_bytes([data[44], data[45], data[46], data[47]]);
        let preboot_size = u16::from_le_bytes([data[48], data[49]]);
        let compression = CompressionType::from_flags1(flags1);

        let header = QnxIfsHeader {
            magic: QNX_IFS_MAGIC,
            version,
            flags1,
            flags2,
            flags,
            header_size,
            machine_type,
            startup_vaddr,
            paddr_bias,
            image_paddr,
            ram_paddr,
            ram_size,
            startup_size,
            stored_size,
            imagefs_paddr,
            imagefs_size,
            image_size: imagefs_size,
            preboot_size,
            compression,
        };

        let mut image_header = None;
        let mut entries = Vec::new();

        // If sufficient data exists beyond startup_size, attempt to parse imagefs
        if data.len() > startup_size as usize {
            if let Ok(decompressed) = Self::decompress_imagefs(data, &header) {
                if let Ok((ihdr, dirents)) = parse_imagefs(&decompressed) {
                    image_header = Some(ihdr);
                    entries = dirents;
                }
            }
        }

        Ok(Self {
            header,
            total_size: data.len(),
            image_header,
            entries,
        })
    }

    /// Decompresses the `imagefs` portion from the raw IFS byte slice.
    pub fn decompress_imagefs(data: &[u8], header: &QnxIfsHeader) -> Result<Vec<u8>, CoreError> {
        let start = header.startup_size as usize;
        if data.len() < start {
            return Err(CoreError::ImmutabilityViolation(
                "IFS data truncated before startup_size boundary".into(),
            ));
        }

        match header.compression {
            CompressionType::None => {
                let end = if header.stored_size as usize > start && header.stored_size as usize <= data.len() {
                    header.stored_size as usize
                } else {
                    data.len()
                };
                Ok(data[start..end].to_vec())
            }
            CompressionType::Zlib => {
                let mut decompressed = Vec::new();
                let mut pos = start;
                while pos + 2 <= data.len() {
                    let chunk_len = u16::from_be_bytes([data[pos], data[pos + 1]]) as usize;
                    pos += 2;
                    if chunk_len == 0 {
                        break; // EOF marker
                    }
                    if pos + chunk_len > data.len() {
                        return Err(CoreError::ImmutabilityViolation(
                            "ZLIB chunk length overflows IFS buffer".into(),
                        ));
                    }
                    let chunk_slice = &data[pos..pos + chunk_len];
                    pos += chunk_len;

                    let mut decoder = ZlibDecoder::new(chunk_slice);
                    let mut chunk_out = Vec::new();
                    decoder.read_to_end(&mut chunk_out).map_err(|e| {
                        CoreError::ImmutabilityViolation(format!("Failed to decompress ZLIB chunk: {e}"))
                    })?;
                    decompressed.extend_from_slice(&chunk_out);
                }
                Ok(decompressed)
            }
            CompressionType::Lzo => {
                let mut decompressed = Vec::new();
                let mut pos = start;
                while pos + 2 <= data.len() {
                    let chunk_len = u16::from_be_bytes([data[pos], data[pos + 1]]) as usize;
                    pos += 2;
                    if chunk_len == 0 {
                        break;
                    }
                    if pos + chunk_len > data.len() {
                        return Err(CoreError::ImmutabilityViolation(
                            "LZO chunk length overflows IFS buffer".into(),
                        ));
                    }
                    let chunk_slice = &data[pos..pos + chunk_len];
                    pos += chunk_len;

                    let chunk_out = lzo1x_decompress(chunk_slice, 65536)?;
                    decompressed.extend_from_slice(&chunk_out);
                }
                Ok(decompressed)
            }
            CompressionType::Ucl => {
                Err(CoreError::ImmutabilityViolation(
                    "UCL compression not supported in this runtime".into(),
                ))
            }
        }
    }
}

/// Helper function to parse an uncompressed `imagefs` container.
pub fn parse_imagefs(imagefs: &[u8]) -> Result<(ImageHeader, Vec<ImageDirent>), CoreError> {
    if imagefs.len() < 88 {
        return Err(CoreError::ImmutabilityViolation(
            "imagefs container too small for image_header".into(),
        ));
    }

    if &imagefs[0..7] != IMAGE_SIG {
        return Err(CoreError::ImmutabilityViolation(
            "Missing 'imagefs' signature in container".into(),
        ));
    }

    let flags = imagefs[7];
    let image_size = u32::from_le_bytes(imagefs[8..12].try_into().unwrap());
    let hdr_dir_size = u32::from_le_bytes(imagefs[12..16].try_into().unwrap());
    let dir_offset = u32::from_le_bytes(imagefs[16..20].try_into().unwrap());

    let mut boot_ino = [0u32; 4];
    for (i, slot) in boot_ino.iter_mut().enumerate() {
        let off = 20 + i * 4;
        *slot = u32::from_le_bytes(imagefs[off..off + 4].try_into().unwrap());
    }

    let script_ino = u32::from_le_bytes(imagefs[36..40].try_into().unwrap());
    let chain_paddr = u32::from_le_bytes(imagefs[40..44].try_into().unwrap());
    let mountflags = u32::from_le_bytes(imagefs[84..88].try_into().unwrap());

    let mp_slice = &imagefs[88..];
    let null_idx = mp_slice.iter().position(|&b| b == 0).unwrap_or(mp_slice.len());
    let mountpoint = String::from_utf8_lossy(&mp_slice[..null_idx]).to_string();

    let header = ImageHeader {
        flags,
        image_size,
        hdr_dir_size,
        dir_offset,
        boot_ino,
        script_ino,
        chain_paddr,
        mountflags,
        mountpoint,
    };

    let mut entries = Vec::new();
    let mut off = dir_offset as usize;
    let end = (hdr_dir_size as usize).min(imagefs.len());

    while off + 24 <= end {
        let size = u16::from_le_bytes([imagefs[off], imagefs[off + 1]]) as usize;
        if size == 0 || off + size > end {
            break;
        }

        let extattr_offset = u16::from_le_bytes([imagefs[off + 2], imagefs[off + 3]]);
        let ino = u32::from_le_bytes(imagefs[off + 4..off + 8].try_into().unwrap());
        let mode = u32::from_le_bytes(imagefs[off + 8..off + 12].try_into().unwrap());
        let gid = u32::from_le_bytes(imagefs[off + 12..off + 16].try_into().unwrap());
        let uid = u32::from_le_bytes(imagefs[off + 16..off + 20].try_into().unwrap());
        let mtime = u32::from_le_bytes(imagefs[off + 20..off + 24].try_into().unwrap());

        let attr = ImageAttr {
            size: size as u16,
            extattr_offset,
            ino,
            mode,
            gid,
            uid,
            mtime,
        };

        let ftype = mode & S_IFMT;
        let mut file_offset = None;
        let mut file_size = None;
        let mut symlink_target = None;
        let path_start;

        if ftype == S_IFREG {
            if off + 32 <= off + size {
                file_offset = Some(u32::from_le_bytes(imagefs[off + 24..off + 28].try_into().unwrap()));
                file_size = Some(u32::from_le_bytes(imagefs[off + 28..off + 32].try_into().unwrap()));
                path_start = off + 32;
            } else {
                path_start = off + 24;
            }
        } else if ftype == S_IFDIR {
            path_start = off + 24;
        } else if ftype == S_IFLNK {
            if off + 28 <= off + size {
                let sym_off = u16::from_le_bytes([imagefs[off + 24], imagefs[off + 25]]) as usize;
                let sym_sz = u16::from_le_bytes([imagefs[off + 26], imagefs[off + 27]]) as usize;
                path_start = off + 28;
                let sym_abs = off + sym_off;
                if sym_abs + sym_sz <= off + size {
                    symlink_target = Some(
                        String::from_utf8_lossy(&imagefs[sym_abs..sym_abs + sym_sz]).to_string(),
                    );
                }
            } else {
                path_start = off + 24;
            }
        } else {
            path_start = off + 24;
        }

        let slice = &imagefs[path_start..off + size];
        let p_end = slice.iter().position(|&b| b == 0).unwrap_or(slice.len());
        let path = String::from_utf8_lossy(&slice[..p_end]).to_string();

        entries.push(ImageDirent {
            attr,
            path,
            file_offset,
            file_size,
            symlink_target,
        });

        off += size;
    }

    Ok((header, entries))
}

/// Computes the 32-bit two's complement checksum for the QNX IFS imagefs container.
pub fn compute_trailer_checksum(imagefs: &[u8]) -> u32 {
    let mut sum: u32 = 0;
    let word_count = (imagefs.len().saturating_sub(4)) / 4;
    for i in 0..word_count {
        let off = i * 4;
        let word = u32::from_le_bytes([
            imagefs[off],
            imagefs[off + 1],
            imagefs[off + 2],
            imagefs[off + 3],
        ]);
        sum = sum.wrapping_add(word);
    }
    (!sum).wrapping_add(1)
}

/// Verifies that summing all 32-bit little-endian words in the container equals 0.
pub fn verify_trailer_checksum(imagefs: &[u8]) -> bool {
    if imagefs.len() < 4 || imagefs.len() % 4 != 0 {
        return false;
    }
    let mut sum: u32 = 0;
    for chunk in imagefs.chunks_exact(4) {
        let word = u32::from_le_bytes(chunk.try_into().unwrap());
        sum = sum.wrapping_add(word);
    }
    sum == 0
}

#[derive(Debug, Clone)]
pub struct QnxIfsAdapter {
    capabilities: FormatCapabilities,
}

impl Default for QnxIfsAdapter {
    fn default() -> Self {
        Self {
            capabilities: FormatCapabilities::read_only(),
        }
    }
}

impl FormatAdapter for QnxIfsAdapter {
    fn format_name(&self) -> &'static str {
        "qnx_ifs"
    }

    fn capabilities(&self) -> &FormatCapabilities {
        &self.capabilities
    }

    fn detect(&self, data: &[u8]) -> bool {
        if data.len() < 4 {
            return false;
        }
        &data[0..4] == &QNX_IFS_MAGIC
    }

    fn coverage_ratio(&self, data: &[u8]) -> f32 {
        if self.detect(data) {
            1.0
        } else {
            0.0
        }
    }
}

/// File or symlink record in the IFS build staging tree.
#[derive(Debug, Clone)]
pub struct StagedFile {
    pub path: String,
    pub data: Vec<u8>,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub mtime: u32,
    pub symlink_target: Option<String>,
}

/// Builder for constructing valid, bootable QNX IFS images conforming to Renesas SH-4 specifications.
#[derive(Debug, Clone)]
pub struct QnxIfsBuilder {
    pub machine_type: u16,
    pub compression: CompressionType,
    pub mountpoint: String,
    pub startup_code: Option<Vec<u8>>,
    pub files: Vec<StagedFile>,
}

impl Default for QnxIfsBuilder {
    fn default() -> Self {
        Self::new(QNX_MACHINE_SH4)
    }
}

impl QnxIfsBuilder {
    /// Creates a new builder for the specified machine architecture.
    pub fn new(machine_type: u16) -> Self {
        Self {
            machine_type,
            compression: CompressionType::None,
            mountpoint: "/".to_string(),
            startup_code: None,
            files: Vec::new(),
        }
    }

    /// Configures the compression format for the imagefs stream.
    pub fn with_compression(&mut self, compression: CompressionType) -> &mut Self {
        self.compression = compression;
        self
    }

    /// Sets the target mount point for this IFS container.
    pub fn with_mountpoint(&mut self, mountpoint: impl Into<String>) -> &mut Self {
        self.mountpoint = mountpoint.into();
        self
    }

    /// Provides custom or reference SH-4 startup machine code.
    pub fn with_startup_code(&mut self, code: &[u8]) -> &mut Self {
        self.startup_code = Some(code.to_vec());
        self
    }

    /// Adds a regular file with standard permissions (0o755).
    pub fn add_file(&mut self, path: impl Into<String>, data: &[u8]) -> &mut Self {
        self.files.push(StagedFile {
            path: path.into(),
            data: data.to_vec(),
            mode: S_IFREG | 0o755,
            uid: 0,
            gid: 0,
            mtime: 1700000000,
            symlink_target: None,
        });
        self
    }

    /// Adds a regular file with explicit POSIX mode flags.
    pub fn add_file_with_mode(&mut self, path: impl Into<String>, data: &[u8], mode: u32) -> &mut Self {
        self.files.push(StagedFile {
            path: path.into(),
            data: data.to_vec(),
            mode: S_IFREG | (mode & 0o7777),
            uid: 0,
            gid: 0,
            mtime: 1700000000,
            symlink_target: None,
        });
        self
    }

    /// Adds an entry with full parameters conforming to project specifications.
    pub fn add_entry(&mut self, path: &str, data: &[u8], mode: u32, compression: CompressionType) -> &mut Self {
        self.compression = compression;
        self.files.push(StagedFile {
            path: path.to_string(),
            data: data.to_vec(),
            mode,
            uid: 0,
            gid: 0,
            mtime: 1700000000,
            symlink_target: None,
        });
        self
    }

    /// Adds a directory inode with POSIX mode.
    pub fn add_dir(&mut self, path: impl Into<String>, mode: u32) -> &mut Self {
        self.files.push(StagedFile {
            path: path.into(),
            data: Vec::new(),
            mode: S_IFDIR | (mode & 0o7777),
            uid: 0,
            gid: 0,
            mtime: 1700000000,
            symlink_target: None,
        });
        self
    }

    /// Adds a symbolic link inode.
    pub fn add_symlink(&mut self, path: impl Into<String>, target: impl Into<String>) -> &mut Self {
        self.files.push(StagedFile {
            path: path.into(),
            data: Vec::new(),
            mode: S_IFLNK | 0o777,
            uid: 0,
            gid: 0,
            mtime: 1700000000,
            symlink_target: Some(target.into()),
        });
        self
    }

    /// Assembles the complete, bit-accurate QNX IFS binary image.
    pub fn build(&self) -> Result<Vec<u8>, CoreError> {
        // Collect directories and files
        let mut dirs = BTreeMap::new();
        let mut regular_files = Vec::new();
        let mut symlinks = Vec::new();

        for file in &self.files {
            let normalized_path = file.path.trim_start_matches('/').to_string();
            if normalized_path.is_empty() {
                continue;
            }

            // Ensure all intermediate directories exist
            let parts = normalized_path.split('/').collect::<Vec<_>>();
            if parts.len() > 1 {
                let mut accumulated = String::new();
                for i in 0..parts.len() - 1 {
                    if !accumulated.is_empty() {
                        accumulated.push('/');
                    }
                    accumulated.push_str(parts[i]);
                    dirs.entry(accumulated.clone()).or_insert(S_IFDIR | 0o755);
                }
            }

            let ftype = file.mode & S_IFMT;
            if ftype == S_IFDIR {
                dirs.insert(normalized_path, file.mode);
            } else if ftype == S_IFLNK {
                symlinks.push((normalized_path, file.symlink_target.clone().unwrap_or_default(), file.mode, file.mtime));
            } else {
                regular_files.push((normalized_path, &file.data, file.mode, file.mtime));
            }
        }

        // Build image_header
        let mp_bytes = self.mountpoint.as_bytes();
        let header_base_size = 88 + mp_bytes.len() + 1;
        let dir_offset = (header_base_size + 3) & !3;

        let mut ihdr_bytes = vec![0u8; dir_offset];
        ihdr_bytes[0..7].copy_from_slice(IMAGE_SIG);
        ihdr_bytes[7] = 0; // Flags
        ihdr_bytes[16..20].copy_from_slice(&(dir_offset as u32).to_le_bytes());
        ihdr_bytes[20..24].copy_from_slice(&1u32.to_le_bytes()); // boot_ino[0]
        ihdr_bytes[84..88].copy_from_slice(&0u32.to_le_bytes()); // mountflags
        ihdr_bytes[88..88 + mp_bytes.len()].copy_from_slice(mp_bytes);
        ihdr_bytes[88 + mp_bytes.len()] = 0; // Null terminator

        // Build dirent records
        let mut dirent_blob = Vec::new();
        let mut ino_counter = 2u32;

        // 1. Directory dirents
        for (dir_path, dir_mode) in &dirs {
            let path_bytes = dir_path.as_bytes();
            let raw_size = 24 + path_bytes.len() + 1;
            let rec_size = (raw_size + 3) & !3;
            let mut rec = vec![0u8; rec_size];
            rec[0..2].copy_from_slice(&(rec_size as u16).to_le_bytes());
            rec[2..4].copy_from_slice(&0u16.to_le_bytes());
            rec[4..8].copy_from_slice(&ino_counter.to_le_bytes());
            rec[8..12].copy_from_slice(&dir_mode.to_le_bytes());
            rec[12..16].copy_from_slice(&0u32.to_le_bytes()); // gid
            rec[16..20].copy_from_slice(&0u32.to_le_bytes()); // uid
            rec[20..24].copy_from_slice(&1700000000u32.to_le_bytes()); // mtime
            rec[24..24 + path_bytes.len()].copy_from_slice(path_bytes);
            rec[24 + path_bytes.len()] = 0;

            dirent_blob.extend_from_slice(&rec);
            ino_counter += 1;
        }

        // 2. Regular file dirents (with placeholder offsets to be patched)
        let mut file_rec_offsets = Vec::new();
        for (file_path, file_data, file_mode, mtime) in &regular_files {
            let path_bytes = file_path.as_bytes();
            let raw_size = 32 + path_bytes.len() + 1;
            let rec_size = (raw_size + 3) & !3;
            let mut rec = vec![0u8; rec_size];
            rec[0..2].copy_from_slice(&(rec_size as u16).to_le_bytes());
            rec[2..4].copy_from_slice(&0u16.to_le_bytes());
            rec[4..8].copy_from_slice(&ino_counter.to_le_bytes());
            rec[8..12].copy_from_slice(&file_mode.to_le_bytes());
            rec[12..16].copy_from_slice(&0u32.to_le_bytes());
            rec[16..20].copy_from_slice(&0u32.to_le_bytes());
            rec[20..24].copy_from_slice(&mtime.to_le_bytes());
            rec[24..28].copy_from_slice(&0u32.to_le_bytes()); // Placeholder for file_offset
            rec[28..32].copy_from_slice(&(file_data.len() as u32).to_le_bytes());
            rec[32..32 + path_bytes.len()].copy_from_slice(path_bytes);
            rec[32 + path_bytes.len()] = 0;

            let dirent_blob_offset = dirent_blob.len();
            file_rec_offsets.push(dirent_blob_offset);
            dirent_blob.extend_from_slice(&rec);
            ino_counter += 1;
        }

        // 3. Symlink dirents
        for (link_path, target_path, link_mode, mtime) in &symlinks {
            let path_bytes = link_path.as_bytes();
            let target_bytes = target_path.as_bytes();
            let sym_off = (28 + path_bytes.len() + 1 + 3) & !3;
            let raw_size = sym_off + target_bytes.len() + 1;
            let rec_size = (raw_size + 3) & !3;
            let mut rec = vec![0u8; rec_size];
            rec[0..2].copy_from_slice(&(rec_size as u16).to_le_bytes());
            rec[2..4].copy_from_slice(&0u16.to_le_bytes());
            rec[4..8].copy_from_slice(&ino_counter.to_le_bytes());
            rec[8..12].copy_from_slice(&link_mode.to_le_bytes());
            rec[12..16].copy_from_slice(&0u32.to_le_bytes());
            rec[16..20].copy_from_slice(&0u32.to_le_bytes());
            rec[20..24].copy_from_slice(&mtime.to_le_bytes());
            rec[24..26].copy_from_slice(&(sym_off as u16).to_le_bytes());
            rec[26..28].copy_from_slice(&(target_bytes.len() as u16).to_le_bytes());
            rec[28..28 + path_bytes.len()].copy_from_slice(path_bytes);
            rec[28 + path_bytes.len()] = 0;
            rec[sym_off..sym_off + target_bytes.len()].copy_from_slice(target_bytes);
            rec[sym_off + target_bytes.len()] = 0;

            dirent_blob.extend_from_slice(&rec);
            ino_counter += 1;
        }

        // Zero terminator for dirent list (rec_size = 0)
        dirent_blob.extend_from_slice(&[0u8, 0u8, 0u8, 0u8]);

        let hdr_dir_size = dir_offset + dirent_blob.len();

        // 4KB page alignment for file payloads
        let file_data_start = (hdr_dir_size + 4095) & !4095;

        // Patch file_offset fields in dirent_blob
        let mut current_file_offset = file_data_start;
        for (i, (_, file_data, _, _)) in regular_files.iter().enumerate() {
            let dirent_rec_off = file_rec_offsets[i];
            dirent_blob[dirent_rec_off + 24..dirent_rec_off + 28]
                .copy_from_slice(&(current_file_offset as u32).to_le_bytes());
            current_file_offset += file_data.len();
            current_file_offset = (current_file_offset + 3) & !3; // 4-byte align each file
        }

        let total_data = current_file_offset;
        let total_aligned = (total_data + 4095) & !4095;
        let image_size = total_aligned + 4; // +4 for 32-bit trailer checksum

        // Patch image_header sizes
        ihdr_bytes[8..12].copy_from_slice(&(image_size as u32).to_le_bytes());
        ihdr_bytes[12..16].copy_from_slice(&(hdr_dir_size as u32).to_le_bytes());

        // Assemble imagefs
        let mut imagefs = vec![0u8; image_size];
        imagefs[0..dir_offset].copy_from_slice(&ihdr_bytes);
        imagefs[dir_offset..dir_offset + dirent_blob.len()].copy_from_slice(&dirent_blob);

        let mut write_cur = file_data_start;
        for (_, file_data, _, _) in &regular_files {
            imagefs[write_cur..write_cur + file_data.len()].copy_from_slice(file_data);
            write_cur += file_data.len();
            write_cur = (write_cur + 3) & !3;
        }

        // Compute two's complement trailer checksum
        let cksum = compute_trailer_checksum(&imagefs);
        imagefs[image_size - 4..image_size].copy_from_slice(&cksum.to_le_bytes());

        // Format compressed or raw chunk stream
        let mut stream_payload = Vec::new();
        let flags1_comp;

        match self.compression {
            CompressionType::None => {
                stream_payload = imagefs.clone();
                flags1_comp = 0;
            }
            CompressionType::Zlib => {
                flags1_comp = CompressionType::Zlib.to_flags1_bits();
                for chunk in imagefs.chunks(65536) {
                    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
                    encoder.write_all(chunk).map_err(|e| {
                        CoreError::ImmutabilityViolation(format!("ZLIB compression failed: {e}"))
                    })?;
                    let compressed = encoder.finish().map_err(|e| {
                        CoreError::ImmutabilityViolation(format!("ZLIB finalize failed: {e}"))
                    })?;
                    stream_payload.extend_from_slice(&(compressed.len() as u16).to_be_bytes());
                    stream_payload.extend_from_slice(&compressed);
                }
                // End-of-stream marker
                stream_payload.extend_from_slice(&0u16.to_be_bytes());
            }
            CompressionType::Lzo => {
                flags1_comp = CompressionType::Lzo.to_flags1_bits();
                for chunk in imagefs.chunks(65536) {
                    let compressed = lzo1x_compress(chunk)?;
                    stream_payload.extend_from_slice(&(compressed.len() as u16).to_be_bytes());
                    stream_payload.extend_from_slice(&compressed);
                }
                stream_payload.extend_from_slice(&0u16.to_be_bytes());
            }
            CompressionType::Ucl => {
                return Err(CoreError::ImmutabilityViolation(
                    "UCL compression not supported in this runtime".into(),
                ));
            }
        }

        // Generate startup section
        let startup_size = self
            .startup_code
            .as_ref()
            .map(|c| c.len().max(256))
            .unwrap_or(256);

        let mut startup_buf = vec![0u8; startup_size];
        if let Some(code) = &self.startup_code {
            let copy_len = code.len().min(startup_size);
            startup_buf[..copy_len].copy_from_slice(&code[..copy_len]);
        } else {
            // Embed SH-4 NOP instructions in bootstrap area
            for i in (64..startup_size).step_by(2) {
                startup_buf[i] = 0x09;
                startup_buf[i + 1] = 0x00; // nop in SH-4
            }
        }

        let stored_size = startup_size as u32 + stream_payload.len() as u32;
        let total_size = stored_size as usize;

        // Enforce hard NOR flash partition boundary
        if total_size > MAX_IFS_ROOT_SIZE {
            return Err(CoreError::ImmutabilityViolation(format!(
                "IFS image size ({} bytes) exceeds NOR flash ifs-root partition maximum ({} bytes)",
                total_size, MAX_IFS_ROOT_SIZE
            )));
        }

        // Populate 64-byte QNX startup_header
        startup_buf[0..4].copy_from_slice(&QNX_IFS_MAGIC);
        startup_buf[4..6].copy_from_slice(&1u16.to_le_bytes()); // version = 1
        startup_buf[6] = flags1_comp | if self.startup_code.is_some() { FLAGS1_VIRTUAL } else { 0 };
        startup_buf[7] = 0; // flags2
        startup_buf[8..10].copy_from_slice(&(startup_size as u16).to_le_bytes());
        startup_buf[10..12].copy_from_slice(&self.machine_type.to_le_bytes());
        startup_buf[12..16].copy_from_slice(&0x8801_3D80u32.to_le_bytes()); // startup_vaddr
        startup_buf[16..20].copy_from_slice(&0x8000_0000u32.to_le_bytes()); // paddr_bias
        startup_buf[20..24].copy_from_slice(&0x0801_0000u32.to_le_bytes()); // image_paddr
        startup_buf[24..28].copy_from_slice(&0x0801_0000u32.to_le_bytes()); // ram_paddr
        startup_buf[28..32].copy_from_slice(&((image_size as u32) + 0x0010_0000).to_le_bytes()); // ram_size
        startup_buf[32..36].copy_from_slice(&(startup_size as u32).to_le_bytes()); // startup_size at offset 32!
        startup_buf[36..40].copy_from_slice(&stored_size.to_le_bytes()); // stored_size at offset 36!
        startup_buf[40..44].copy_from_slice(&0u32.to_le_bytes()); // imagefs_paddr
        startup_buf[44..48].copy_from_slice(&(image_size as u32).to_le_bytes()); // imagefs_size at offset 44!
        startup_buf[48..50].copy_from_slice(&0u16.to_le_bytes()); // preboot_size
        startup_buf[50..52].copy_from_slice(&0u16.to_le_bytes()); // zero0

        let mut output = Vec::with_capacity(total_size);
        output.extend_from_slice(&startup_buf);
        output.extend_from_slice(&stream_payload);

        Ok(output)
    }
}

// ---------------------------------------------------------------------------
// Pure-Rust LZO1X Compression and Decompression
// ---------------------------------------------------------------------------

/// Compresses a slice using LZO1X-1 format compatible with QNX Neutrino and MMI 3G+.
pub fn lzo1x_compress(input: &[u8]) -> Result<Vec<u8>, CoreError> {
    let in_len = input.len();
    if in_len == 0 {
        return Ok(vec![0x11, 0x00, 0x00]);
    }

    let mut out = Vec::with_capacity(in_len + (in_len / 16) + 64);
    let mut ip = 0;
    let mut anchor = 0;

    // First literal byte run
    if in_len <= 3 {
        out.push(in_len as u8);
        out.extend_from_slice(&input[0..in_len]);
        out.extend_from_slice(&[0x11, 0x00, 0x00]); // EOF marker
        return Ok(out);
    }

    // Small hash table for 4-byte match detection
    const HASH_SIZE: usize = 16384;
    let mut hash_table = vec![usize::MAX; HASH_SIZE];

    let mut first_literal = true;

    while ip + 4 <= in_len {
        let h = ((u32::from_le_bytes([input[ip], input[ip + 1], input[ip + 2], input[ip + 3]])
            .wrapping_mul(0x18359b3d))
            >> 18) as usize
            % HASH_SIZE;
        let ref_pos = hash_table[h];
        hash_table[h] = ip;

        let mut match_len = 0;
        let offset = if ref_pos != usize::MAX && ip > ref_pos && (ip - ref_pos) <= 49151 {
            ip - ref_pos
        } else {
            0
        };

        if offset > 0 {
            while ip + match_len < in_len
                && input[ref_pos + match_len] == input[ip + match_len]
                && match_len < 2048
            {
                match_len += 1;
            }
        }

        if match_len >= 3 {
            // Flush literals before this match
            let lit_len = ip - anchor;
            if lit_len > 0 {
                write_lzo_literals(&mut out, &input[anchor..ip], first_literal);
                first_literal = false;
            }

            // Encode match
            write_lzo_match(&mut out, match_len, offset);
            ip += match_len;
            anchor = ip;
        } else {
            ip += 1;
        }
    }

    // Flush any remaining literals
    if anchor < in_len {
        write_lzo_literals(&mut out, &input[anchor..in_len], first_literal);
    }

    // End-of-stream marker: M4 match with offset = 0
    out.extend_from_slice(&[0x11, 0x00, 0x00]);

    Ok(out)
}

fn write_lzo_literals(out: &mut Vec<u8>, literals: &[u8], is_first: bool) {
    let mut len = literals.len();
    let mut pos = 0;

    if is_first && len <= 238 {
        if len <= 3 {
            out.push(len as u8);
        } else {
            out.push((len - 3) as u8);
        }
        out.extend_from_slice(literals);
        return;
    }

    while len > 0 {
        let chunk_len = len.min(1024);
        if chunk_len <= 3 {
            out.push(chunk_len as u8);
        } else if chunk_len <= 18 {
            out.push((chunk_len - 3) as u8);
        } else {
            out.push(0);
            let mut rem = chunk_len - 18;
            while rem >= 255 {
                out.push(0);
                rem -= 255;
            }
            out.push(rem as u8);
        }
        out.extend_from_slice(&literals[pos..pos + chunk_len]);
        pos += chunk_len;
        len -= chunk_len;
    }
}

fn write_lzo_match(out: &mut Vec<u8>, len: usize, offset: usize) {
    let m_off = offset - 1;
    if len <= 8 && offset <= 2048 {
        // M2 match
        let t = ((len - 1) << 5) | ((m_off & 7) << 2);
        out.push(t as u8);
        out.push((m_off >> 3) as u8);
    } else if len <= 33 && offset <= 16384 {
        // M3 match
        let t = 32 | (len - 2);
        out.push(t as u8);
        out.push(((m_off & 3) << 6) as u8);
        out.push((m_off >> 2) as u8);
    } else {
        // M4 match
        let len_adj = len - 2;
        let mut t = 16;
        if len_adj <= 7 {
            t |= len_adj;
            out.push(t as u8);
        } else {
            out.push(t as u8);
            let mut rem = len_adj - 7;
            while rem >= 255 {
                out.push(0);
                rem -= 255;
            }
            out.push(rem as u8);
        }
        let off_val = offset.saturating_sub(0x4000);
        out.push(((off_val & 3) << 6) as u8);
        out.push((off_val >> 2) as u8);
    }
}

/// Permissive LZO1X decompressor matching QNX Neutrino and Harman specifications.
pub fn lzo1x_decompress(input: &[u8], _expected_len: usize) -> Result<Vec<u8>, CoreError> {
    let mut out = Vec::new();
    let mut ip = 0;
    let in_len = input.len();

    if in_len == 0 {
        return Ok(out);
    }

    let mut t: usize;
    if input[ip] > 17 {
        t = (input[ip] - 17) as usize;
        ip += 1;
        if ip + t > in_len {
            return Err(CoreError::ImmutabilityViolation("LZO overrun in initial literal".into()));
        }
        out.extend_from_slice(&input[ip..ip + t]);
        ip += t;
    }

    while ip < in_len {
        t = input[ip] as usize;
        ip += 1;

        if t < 16 {
            if t == 0 {
                while ip < in_len && input[ip] == 0 {
                    t += 255;
                    ip += 1;
                }
                if ip >= in_len {
                    return Err(CoreError::ImmutabilityViolation("LZO unexpected EOF".into()));
                }
                t += 15 + input[ip] as usize;
                ip += 1;
            }
            t += 3;
            if ip + t > in_len {
                return Err(CoreError::ImmutabilityViolation("LZO literal overrun".into()));
            }
            out.extend_from_slice(&input[ip..ip + t]);
            ip += t;
            if ip >= in_len {
                break;
            }
            t = input[ip] as usize;
            ip += 1;
        }

        let m_len: usize;
        let m_off: usize;

        if t >= 64 {
            m_len = ((t >> 5) - 1) + 2;
            let byte2 = if ip < in_len { input[ip] as usize } else { 0 };
            ip += 1;
            m_off = (((t & 0x18) >> 3) | (byte2 << 2)) + 1;
        } else if t >= 32 {
            t &= 31;
            if t == 0 {
                while ip < in_len && input[ip] == 0 {
                    t += 255;
                    ip += 1;
                }
                if ip >= in_len {
                    return Err(CoreError::ImmutabilityViolation("LZO unexpected EOF".into()));
                }
                t += 31 + input[ip] as usize;
                ip += 1;
            }
            m_len = t + 2;
            if ip + 2 > in_len {
                break;
            }
            m_off = ((input[ip] as usize >> 6) | ((input[ip + 1] as usize) << 2)) + 1;
            ip += 2;
        } else if t >= 16 {
            t &= 7;
            if t == 0 {
                while ip < in_len && input[ip] == 0 {
                    t += 255;
                    ip += 1;
                }
                if ip >= in_len {
                    return Err(CoreError::ImmutabilityViolation("LZO unexpected EOF".into()));
                }
                t += 7 + input[ip] as usize;
                ip += 1;
            }
            m_len = t + 2;
            if ip + 2 > in_len {
                break;
            }
            let off_raw = (input[ip] as usize >> 6) | ((input[ip + 1] as usize) << 2);
            ip += 2;
            if off_raw == 0 {
                // EOF marker
                break;
            }
            m_off = off_raw + 0x4000;
        } else {
            let byte2 = if ip < in_len { input[ip] as usize } else { 0 };
            ip += 1;
            m_off = 1 + (t >> 2) + (byte2 << 2);
            m_len = 2;
        }

        if m_off > out.len() {
            // Gracefully handle or break on edge
            break;
        }
        let copy_start = out.len() - m_off;
        for i in 0..m_len {
            let b = out[copy_start + i];
            out.push(b);
        }
    }

    Ok(out)
}
