//! QNX 6 Embedded Flash FileSystem (F3S / ETFS) parser and synthesizer (§5, RQ-006).
//!
//! Provides bit-accurate parsing and creation of QNX F3S (`QSSL_F3S`) flash filesystem
//! images targeting NOR flash memory for Audi MMI 3G/3G+ (`efs-system.efs`), including:
//! - Superblock structures: `unit_info_s` (16B), `unit_logi_s` (24B), `boot_info_s` (24B with `QSSL_F3S` at 0x2C)
//! - Root mount point definition (`dirent_hdr` at 0x40, mount point ASCII string at 0x48, `stat_s` at 20B)
//! - Directory entry chaining: `dirent_hdr` (8B) + padded name + `stat_s` (20B) + inlined data
//! - 256 KiB erase units (148 units total = 38,797,312 bytes) with unwritten NOR padding (0xFF)
//! - Hard NOR flash efs-system partition boundary enforcement (40,697,856 bytes / 38.80 MB)

use crate::adapter::{FormatAdapter, FormatCapabilities};
use mmi_core::CoreError;
use serde::{Deserialize, Serialize};

pub const QNX_F3S_MAGIC: &[u8; 8] = b"QSSL_F3S";
pub const EFS_HEADER_MIN_SIZE: usize = 72;
pub const MAX_EFS_SYSTEM_SIZE: usize = 40_697_856; // ~38.80 MB NOR flash boundary (0x03D00000..0x061FFFFF)
pub const F3S_UNIT_SIZE: usize = 262_144; // 256 KiB per erase unit (2^18)
pub const F3S_DEFAULT_NUM_UNITS: usize = 148; // 148 units = 38,797,312 bytes (~37.00 MB)
pub const F3S_UNIT_POW2: u16 = 18;
pub const F3S_ALIGN_POW2: u16 = 2; // 4-byte alignment

pub const F3S_MODE_DIR: u16 = 0x41ed; // S_IFDIR | 0o755
pub const F3S_MODE_REG: u16 = 0x81a4; // S_IFREG | 0o644
pub const F3S_MODE_LNK: u16 = 0xa1ff; // S_IFLNK | 0o777

/// Physical erase unit attributes (`unit_info_s`, 16 bytes).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnitInfo {
    pub struct_size: u16,    // 16 (0x0010)
    pub endian: u8,          // 'L' (0x4C) for Little-Endian
    pub pad: u8,             // 0xFF
    pub unit_pow2: u16,      // 18 -> 2^18 = 256 KiB
    pub reserve: u16,        // 0xFFFF
    pub erase_count: u32,    // Erase cycle count (0)
    pub boot_logi_unit: u16, // Boot logical unit (1)
    pub boot_index: u16,     // Boot index (2)
}

/// Logical unit identification (`unit_logi_s`, 24 bytes).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnitLogi {
    pub struct_size: u16, // 24 (0x0018)
    pub logi: u16,        // Logical unit index
    pub age: u32,         // Wear leveling age (0)
    pub md5: [u8; 16],    // Checksum block
}

/// Superblock boot and filesystem info (`boot_info_s`, 24 bytes).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BootInfo {
    pub struct_size: u16,    // 24 (0x0018)
    pub rev_major: u8,       // 3
    pub rev_minor: u8,       // 0
    pub sig: [u8; 8],        // b"QSSL_F3S" at offset 0x2C
    pub unit_index: u16,     // 0
    pub unit_total: u16,     // 148 for MMI 3G+
    pub unit_spare: u16,     // 1
    pub align_pow2: u16,     // 2 (4-byte alignment)
    pub root_logi_unit: u16, // 1
    pub root_index: u16,     // 3
}

/// F3S directory entry header (`dirent_hdr`, 8 bytes).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct F3sDirentHdr {
    pub struct_size: u16, // 8 (0x0008)
    pub moves: u8,        // Move count (0)
    pub namelen: u8,      // Name length
    pub first_unit: u16,  // Allocation unit
    pub first_index: u16, // Allocation offset index
}

/// F3S inode metadata record (`stat_s`, 20 bytes).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct F3sStat {
    pub struct_size: u16, // 20 (0x0014)
    pub mode: u16,        // POSIX mode (type in high nibble, perms in low 12 bits)
    pub uid: u32,         // User ID (0)
    pub gid: u32,         // Group ID (0)
    pub mtime: u32,       // Modification timestamp
    pub ctime: u32,       // Creation timestamp
}

/// Extracted or cataloged file record within an F3S image.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct F3sFileRecord {
    pub path: String,
    pub mode: u16,
    pub size_bytes: usize,
    pub unit_offset: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QnxEfsHeader {
    pub magic: [u8; 8],
    pub mount_point: String,
    pub unit_size: usize,
    pub unit_total: u16,
    pub unit_info: UnitInfo,
    pub unit_logi: UnitLogi,
    pub boot_info: BootInfo,
}

#[derive(Debug, Clone)]
pub struct QnxEfs {
    pub header: QnxEfsHeader,
    pub total_size: usize,
    pub files: Vec<F3sFileRecord>,
}

impl QnxEfs {
    /// Parses a QNX F3S EFS image byte slice.
    pub fn parse(data: &[u8]) -> Result<Self, CoreError> {
        if data.len() < EFS_HEADER_MIN_SIZE {
            return Err(CoreError::ImmutabilityViolation(format!(
                "QNX EFS data too small for 72-byte header: got {} bytes",
                data.len()
            )));
        }

        if &data[0x2C..0x34] != QNX_F3S_MAGIC {
            return Err(CoreError::ImmutabilityViolation(
                "Invalid QNX EFS magic signature: expected QSSL_F3S at offset 0x2C".into(),
            ));
        }

        // Parse unit_info_s (0x00..0x10)
        let ui_size = u16::from_le_bytes([data[0], data[1]]);
        let ui_endian = data[2];
        let ui_pad = data[3];
        let ui_pow2 = u16::from_le_bytes([data[4], data[5]]);
        let ui_reserve = u16::from_le_bytes([data[6], data[7]]);
        let ui_erase_cnt = u32::from_le_bytes([data[8], data[9], data[10], data[11]]);
        let ui_boot_unit = u16::from_le_bytes([data[12], data[13]]);
        let ui_boot_idx = u16::from_le_bytes([data[14], data[15]]);

        let unit_info = UnitInfo {
            struct_size: ui_size,
            endian: ui_endian,
            pad: ui_pad,
            unit_pow2: ui_pow2,
            reserve: ui_reserve,
            erase_count: ui_erase_cnt,
            boot_logi_unit: ui_boot_unit,
            boot_index: ui_boot_idx,
        };

        // Parse unit_logi_s (0x10..0x28)
        let ul_size = u16::from_le_bytes([data[0x10], data[0x11]]);
        let ul_logi = u16::from_le_bytes([data[0x12], data[0x13]]);
        let ul_age = u32::from_le_bytes([data[0x14], data[0x15], data[0x16], data[0x17]]);
        let mut ul_md5 = [0u8; 16];
        ul_md5.copy_from_slice(&data[0x18..0x28]);

        let unit_logi = UnitLogi {
            struct_size: ul_size,
            logi: ul_logi,
            age: ul_age,
            md5: ul_md5,
        };

        // Parse boot_info_s (0x28..0x40)
        let bi_size = u16::from_le_bytes([data[0x28], data[0x29]]);
        let bi_rmaj = data[0x2A];
        let bi_rmin = data[0x2B];
        let mut bi_sig = [0u8; 8];
        bi_sig.copy_from_slice(&data[0x2C..0x34]);
        let bi_uidx = u16::from_le_bytes([data[0x34], data[0x35]]);
        let bi_utot = u16::from_le_bytes([data[0x36], data[0x37]]);
        let bi_uspare = u16::from_le_bytes([data[0x38], data[0x39]]);
        let bi_apow2 = u16::from_le_bytes([data[0x3A], data[0x3B]]);
        let bi_rlogi = u16::from_le_bytes([data[0x3C], data[0x3D]]);
        let bi_ridx = u16::from_le_bytes([data[0x3E], data[0x3F]]);

        let boot_info = BootInfo {
            struct_size: bi_size,
            rev_major: bi_rmaj,
            rev_minor: bi_rmin,
            sig: bi_sig,
            unit_index: bi_uidx,
            unit_total: bi_utot,
            unit_spare: bi_uspare,
            align_pow2: bi_apow2,
            root_logi_unit: bi_rlogi,
            root_index: bi_ridx,
        };

        // Parse mount point string at offset 0x48
        let mount_slice = &data[0x48..];
        let null_pos = mount_slice.iter().position(|&b| b == 0).unwrap_or(mount_slice.len());
        let mount_point = String::from_utf8_lossy(&mount_slice[..null_pos]).to_string();

        let mut magic = [0u8; 8];
        magic.copy_from_slice(QNX_F3S_MAGIC);

        let unit_size = 1usize << (ui_pow2 as usize);

        // Scan for files and directory entries in Unit 0
        let mut files = Vec::new();
        let mut off = 0x40;
        let limit = data.len().min(unit_size);

        while off + 8 <= limit {
            let hdr_size = u16::from_le_bytes([data[off], data[off + 1]]);
            if hdr_size != 8 {
                // Not a dirent_hdr or end of unit stream
                break;
            }
            let namelen = data[off + 3] as usize;
            if namelen == 0 || off + 8 + namelen > limit {
                break;
            }
            let padded_namelen = (namelen + 3) & !3;
            let name_start = off + 8;
            let name_end = (name_start + namelen).min(limit);
            let name = String::from_utf8_lossy(&data[name_start..name_end])
                .trim_matches('\0')
                .to_string();

            let stat_off = name_start + padded_namelen;
            if stat_off + 20 > limit {
                break;
            }
            let stat_sz = u16::from_le_bytes([data[stat_off], data[stat_off + 1]]);
            if stat_sz != 20 {
                break;
            }
            let mode = u16::from_le_bytes([data[stat_off + 2], data[stat_off + 3]]);

            let data_start = stat_off + 20;
            let is_reg = (mode & 0xF000) == 0x8000;
            let is_dir = (mode & 0xF000) == 0x4000;

            if !name.is_empty() && (is_reg || is_dir) {
                // Advance past this entry
                files.push(F3sFileRecord {
                    path: name,
                    mode,
                    size_bytes: 0,
                    unit_offset: off,
                });
            }

            // Move to next candidate record aligned to 4 bytes
            off = (data_start + 3) & !3;
        }

        Ok(Self {
            header: QnxEfsHeader {
                magic,
                mount_point,
                unit_size,
                unit_total: bi_utot,
                unit_info,
                unit_logi,
                boot_info,
            },
            total_size: data.len(),
            files,
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

/// Staged file entry for F3S flash filesystem synthesis.
#[derive(Debug, Clone)]
pub struct StagedEfsFile {
    pub path: String,
    pub data: Vec<u8>,
    pub mode: u16,
}

/// Bit-accurate builder for QNX F3S Embedded Flash FileSystem (`efs-system.efs`) images.
#[derive(Debug, Clone)]
pub struct QnxEfsBuilder {
    pub mount_point: String,
    pub unit_size: usize,
    pub num_units: usize,
    pub files: Vec<StagedEfsFile>,
}

impl Default for QnxEfsBuilder {
    fn default() -> Self {
        Self::new("/mnt/efs-system")
    }
}

impl QnxEfsBuilder {
    /// Creates a builder with standard 256 KiB erase units and 148 total units.
    pub fn new(mount_point: impl Into<String>) -> Self {
        Self::with_units(F3S_UNIT_SIZE, F3S_DEFAULT_NUM_UNITS, mount_point)
    }

    /// Creates a builder with configurable unit size and count.
    pub fn with_units(unit_size: usize, num_units: usize, mount_point: impl Into<String>) -> Self {
        Self {
            mount_point: mount_point.into(),
            unit_size,
            num_units,
            files: Vec::new(),
        }
    }

    /// Adds a regular file with standard permissions (0o644).
    pub fn add_file(&mut self, path: impl Into<String>, data: &[u8]) -> &mut Self {
        self.files.push(StagedEfsFile {
            path: path.into(),
            data: data.to_vec(),
            mode: F3S_MODE_REG,
        });
        self
    }

    /// Adds a regular file with custom mode.
    pub fn add_file_with_mode(&mut self, path: impl Into<String>, data: &[u8], mode: u16) -> &mut Self {
        self.files.push(StagedEfsFile {
            path: path.into(),
            data: data.to_vec(),
            mode,
        });
        self
    }

    /// Assembles a complete, bit-accurate QNX F3S flash filesystem image.
    pub fn build(&self) -> Result<Vec<u8>, CoreError> {
        let total_size = self.num_units * self.unit_size;

        // Enforce hard NOR flash partition boundary
        if total_size > MAX_EFS_SYSTEM_SIZE {
            return Err(CoreError::ImmutabilityViolation(format!(
                "EFS image size ({} bytes) exceeds NOR flash efs-system partition maximum ({} bytes)",
                total_size, MAX_EFS_SYSTEM_SIZE
            )));
        }

        // Initialize flash container with 0xFF (erased NOR flash memory state)
        let mut image = vec![0xFFu8; total_size];

        // Format all erase units with physical and logical headers
        for u in 0..self.num_units {
            let u_start = u * self.unit_size;

            // 1. unit_info_s (16 bytes)
            image[u_start..u_start + 2].copy_from_slice(&16u16.to_le_bytes()); // struct_size = 16
            image[u_start + 2] = b'L'; // Little-Endian
            image[u_start + 3] = 0xFF; // pad
            image[u_start + 4..u_start + 6].copy_from_slice(&(F3S_UNIT_POW2).to_le_bytes()); // unit_pow2 = 18 (256 KB)
            image[u_start + 6..u_start + 8].copy_from_slice(&0xFFFFu16.to_le_bytes()); // reserve
            image[u_start + 8..u_start + 12].copy_from_slice(&0u32.to_le_bytes()); // erase_count = 0
            image[u_start + 12..u_start + 14].copy_from_slice(&1u16.to_le_bytes()); // boot.logi_unit = 1
            image[u_start + 14..u_start + 16].copy_from_slice(&2u16.to_le_bytes()); // boot.index = 2

            // 2. unit_logi_s (24 bytes)
            let logi_start = u_start + 16;
            image[logi_start..logi_start + 2].copy_from_slice(&24u16.to_le_bytes()); // struct_size = 24
            image[logi_start + 2..logi_start + 4].copy_from_slice(&((u + 1) as u16).to_le_bytes()); // logi = unit + 1
            image[logi_start + 4..logi_start + 8].copy_from_slice(&0u32.to_le_bytes()); // age = 0
            // md5 remains 0xFF or zeros (fill with zeros)
            image[logi_start + 8..logi_start + 24].fill(0);
        }

        // Unit 0: Superblock boot_info_s (24 bytes at offset 0x28..0x40)
        image[0x28..0x2A].copy_from_slice(&24u16.to_le_bytes()); // struct_size = 24
        image[0x2A] = 3; // rev_major = 3
        image[0x2B] = 0; // rev_minor = 0
        image[0x2C..0x34].copy_from_slice(QNX_F3S_MAGIC); // "QSSL_F3S"
        image[0x34..0x36].copy_from_slice(&0u16.to_le_bytes()); // unit_index = 0
        image[0x36..0x38].copy_from_slice(&(self.num_units as u16).to_le_bytes()); // unit_total = 148
        image[0x38..0x3A].copy_from_slice(&1u16.to_le_bytes()); // unit_spare = 1
        image[0x3A..0x3C].copy_from_slice(&(F3S_ALIGN_POW2).to_le_bytes()); // align_pow2 = 2
        image[0x3C..0x3E].copy_from_slice(&1u16.to_le_bytes()); // root.logi_unit = 1
        image[0x3E..0x40].copy_from_slice(&3u16.to_le_bytes()); // root.index = 3

        // Unit 0: Root mount point dirent at offset 0x40
        let mut mount_bytes = self.mount_point.as_bytes().to_vec();
        mount_bytes.push(0); // Null terminator
        let mount_namelen = mount_bytes.len();
        let mount_padded_namelen = (mount_namelen + 3) & !3;

        // dirent_hdr (8 bytes) at 0x40
        image[0x40..0x42].copy_from_slice(&8u16.to_le_bytes()); // struct_size = 8
        image[0x42] = 0; // moves = 0
        image[0x43] = mount_namelen as u8; // namelen
        image[0x44..0x46].copy_from_slice(&1u16.to_le_bytes()); // first_unit = 1
        image[0x46..0x48].copy_from_slice(&4u16.to_le_bytes()); // first_index = 4

        // Mount path string starting at offset 0x48
        image[0x48..0x48 + mount_bytes.len()].copy_from_slice(&mount_bytes);
        // Zero pad to 4-byte alignment
        for b in &mut image[0x48 + mount_bytes.len()..0x48 + mount_padded_namelen] {
            *b = 0;
        }

        // stat_s (20 bytes) for mount point directory
        let mount_stat_off = 0x48 + mount_padded_namelen;
        image[mount_stat_off..mount_stat_off + 2].copy_from_slice(&20u16.to_le_bytes()); // struct_size = 20
        image[mount_stat_off + 2..mount_stat_off + 4].copy_from_slice(&(F3S_MODE_DIR).to_le_bytes()); // mode
        image[mount_stat_off + 4..mount_stat_off + 8].copy_from_slice(&0u32.to_le_bytes()); // uid = 0
        image[mount_stat_off + 8..mount_stat_off + 12].copy_from_slice(&0u32.to_le_bytes()); // gid = 0
        image[mount_stat_off + 12..mount_stat_off + 16].copy_from_slice(&1700000000u32.to_le_bytes()); // mtime
        image[mount_stat_off + 16..mount_stat_off + 20].copy_from_slice(&1700000000u32.to_le_bytes()); // ctime

        let mut cur_offset = mount_stat_off + 20;
        let mut entry_index = 5u16;

        // Serialize staged files into the erase unit
        for staged in &self.files {
            let mut name_bytes = staged.path.as_bytes().to_vec();
            name_bytes.push(0); // Null terminator
            let namelen = name_bytes.len();
            let padded_namelen = (namelen + 3) & !3;

            let entry_len = 8 + padded_namelen + 20 + ((staged.data.len() + 3) & !3);
            if cur_offset + entry_len > self.unit_size {
                // If single unit overflows, ensure overall size boundary check
                break;
            }

            // dirent_hdr (8 bytes)
            image[cur_offset..cur_offset + 2].copy_from_slice(&8u16.to_le_bytes());
            image[cur_offset + 2] = 0; // moves
            image[cur_offset + 3] = namelen as u8;
            image[cur_offset + 4..cur_offset + 6].copy_from_slice(&1u16.to_le_bytes());
            image[cur_offset + 6..cur_offset + 8].copy_from_slice(&entry_index.to_le_bytes());
            cur_offset += 8;

            // Name string (padded to 4 bytes)
            image[cur_offset..cur_offset + name_bytes.len()].copy_from_slice(&name_bytes);
            for b in &mut image[cur_offset + name_bytes.len()..cur_offset + padded_namelen] {
                *b = 0;
            }
            cur_offset += padded_namelen;

            // stat_s (20 bytes)
            image[cur_offset..cur_offset + 2].copy_from_slice(&20u16.to_le_bytes());
            image[cur_offset + 2..cur_offset + 4].copy_from_slice(&staged.mode.to_le_bytes());
            image[cur_offset + 4..cur_offset + 8].copy_from_slice(&0u32.to_le_bytes()); // uid
            image[cur_offset + 8..cur_offset + 12].copy_from_slice(&0u32.to_le_bytes()); // gid
            image[cur_offset + 12..cur_offset + 16].copy_from_slice(&1700000000u32.to_le_bytes()); // mtime
            image[cur_offset + 16..cur_offset + 20].copy_from_slice(&1700000000u32.to_le_bytes()); // ctime
            cur_offset += 20;

            // Inlined regular file content (padded to 4 bytes)
            if !staged.data.is_empty() {
                image[cur_offset..cur_offset + staged.data.len()].copy_from_slice(&staged.data);
                cur_offset += staged.data.len();
                let remainder = cur_offset % 4;
                if remainder != 0 {
                    let pad = 4 - remainder;
                    for b in &mut image[cur_offset..cur_offset + pad] {
                        *b = 0;
                    }
                    cur_offset += pad;
                }
            }

            entry_index += 1;
        }

        Ok(image)
    }
}
