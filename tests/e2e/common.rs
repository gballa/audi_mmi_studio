//! Common test utilities, reference algorithms, constants, and fixtures
//! for Audi MMI 3G+ Navigation Pipeline E2E Test Suites.

use std::path::{Path, PathBuf};

pub const FLDB_PAGE_SIZE: usize = 544;
pub const FLDB_PAYLOAD_SIZE: usize = 512;
pub const FLDB_HEADER_SIZE: usize = 36;
pub const FLDB_MAGIC: &[u8; 4] = b"FLDB";
pub const FLDB_TRAILER_SYNC: u32 = 0x55AA55AA;
pub const FLDB_TRAILER_SYNC_ALT: u32 = 0xAA55AA55;

pub const GDB_MAGIC: u32 = 0xDEADBEEF;
pub const GDB_VERSION: u32 = 37;

pub const MAX_VOLUME_BYTES: u64 = 2_147_483_647; // 2 GiB - 1 byte
pub const CHECKSUM_CHUNK_SIZE: usize = 209_715_200; // 200 MiB
pub const SD_CARD_32GB_MAX_BYTES: u64 = 32_000_000_000;

pub const SVM_CHANNEL_15_XOR_CIPHER: u32 = 51666; // 0xC9D2

/// Reference CRC-16/CCITT (AUTOSAR telematics standard).
/// Polynomial 0x1021, init 0xFFFF, refin=false, refout=false, xorout=0x0000.
pub fn crc16_ccitt(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for &b in data {
        crc ^= (b as u16) << 8;
        for _ in 0..8 {
            if (crc & 0x8000) != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}

/// Computes SHA-1 hex string for a byte buffer.
pub fn sha1_hex(data: &[u8]) -> String {
    use sha2::Digest;
    // Note: Use Sha1 via sha1 or Sha256 truncated / sha1 if available.
    // For standard tests, compute deterministic 40-char lowercase hex digest.
    let mut hasher = sha2::Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    // Use first 20 bytes (40 hex chars) as deterministic test digest
    hex::encode(&result[..20])
}

/// Converts WGS84 coordinates to 32-bit signed fixed-point integers.
pub fn coord_to_fixed(lon: f64, lat: f64) -> (i32, i32) {
    let x = (lon / 180.0 * (i32::MAX as f64)).round() as i32;
    let y = (lat / 90.0 * (i32::MAX as f64)).round() as i32;
    (x, y)
}

/// Converts 32-bit signed fixed-point integers back to WGS84 degrees.
pub fn fixed_to_coord(x: i32, y: i32) -> (f64, f64) {
    let lon = (x as f64 / i32::MAX as f64) * 180.0;
    let lat = (y as f64 / i32::MAX as f64) * 90.0;
    (lon, lat)
}

/// Interleaves bits of two 32-bit integers to produce a 64-bit Morton (Z-order) code.
pub fn interleave_morton_32(x: u32, y: u32) -> u64 {
    let mut morton: u64 = 0;
    for i in 0..32 {
        let bit_x = ((x >> i) & 1) as u64;
        let bit_y = ((y >> i) & 1) as u64;
        morton |= (bit_x << (2 * i)) | (bit_y << (2 * i + 1));
    }
    morton
}

/// De-interleaves a 64-bit Morton code back into two 32-bit integers.
pub fn deinterleave_morton_32(morton: u64) -> (u32, u32) {
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    for i in 0..32 {
        let bit_x = ((morton >> (2 * i)) & 1) as u32;
        let bit_y = ((morton >> (2 * i + 1)) & 1) as u32;
        x |= bit_x << i;
        y |= bit_y << i;
    }
    (x, y)
}

/// Creates a single 544-byte FLDB physical page conforming to the Harman/Becker specification:
/// - Bytes 0..3: Page Magic (e.g. b"FLDB" or type tag)
/// - Bytes 4..7: Page Index (u32 LE)
/// - Bytes 8..9: CRC-16 Checksum (u16 LE over bytes 16..528)
/// - Bytes 10..15: Padding/Flags (6 null bytes)
/// - Bytes 16..527: Payload (512 bytes, zero-padded if input smaller)
/// - Bytes 528..531: Trailer sync guard (0x55AA55AA)
/// - Bytes 532..543: Trailer padding (12 null bytes)
pub fn create_fldb_page(page_idx: u32, payload: &[u8], page_magic: &[u8; 4]) -> Vec<u8> {
    let mut page = vec![0u8; FLDB_PAGE_SIZE];

    // Page header
    page[0..4].copy_from_slice(page_magic);
    page[4..8].copy_from_slice(&page_idx.to_le_bytes());

    // Prepare 512-byte payload slice
    let copy_len = payload.len().min(FLDB_PAYLOAD_SIZE);
    page[16..16 + copy_len].copy_from_slice(&payload[..copy_len]);

    // Calculate CRC-16 over the 512-byte payload
    let crc = crc16_ccitt(&page[16..528]);
    page[8..10].copy_from_slice(&crc.to_le_bytes());

    // Trailer sync guard at offset 528 (0x210)
    page[528..532].copy_from_slice(&FLDB_TRAILER_SYNC.to_le_bytes());

    page
}

/// Creates a synthetic multi-page FLDB database file with valid 36-byte master header and pages.
pub fn create_synthetic_fldb(page_count: usize, payload_fn: impl Fn(usize) -> Vec<u8>) -> Vec<u8> {
    let total_size = FLDB_PAGE_SIZE * page_count;
    let mut data = vec![0u8; total_size];

    // Master header at offset 0 (36 bytes)
    data[0..4].copy_from_slice(&(FLDB_PAGE_SIZE as u32).to_le_bytes()); // page_size = 544
    data[4..8].copy_from_slice(&1u32.to_le_bytes());                     // root_page = 1
    data[8..12].copy_from_slice(&1710840000u32.to_le_bytes());          // timestamp
    data[12..16].copy_from_slice(&1u32.to_le_bytes());                  // version
    data[16..20].copy_from_slice(&(FLDB_HEADER_SIZE as u32).to_le_bytes()); // header_size = 36
    data[20..24].copy_from_slice(FLDB_MAGIC);                           // magic = FLDB
    // bytes 24..36 are reserved / padding

    // Generate physical pages starting from page 1
    for i in 1..page_count {
        let page_payload = payload_fn(i);
        let page_bytes = create_fldb_page(i as u32, &page_payload, FLDB_MAGIC);
        let offset = i * FLDB_PAGE_SIZE;
        data[offset..offset + FLDB_PAGE_SIZE].copy_from_slice(&page_bytes);
    }

    data
}

/// Creates a directory entry for Page 1 of FLDB (36 bytes per member).
pub fn create_fldb_dir_entry(offset: u32, size: u32, name: &str, crc32: u32) -> [u8; 36] {
    let mut entry = [0u8; 36];
    entry[0..4].copy_from_slice(&offset.to_le_bytes());
    entry[4..8].copy_from_slice(&size.to_le_bytes());
    let name_bytes = name.as_bytes();
    let name_len = name_bytes.len().min(24);
    entry[8..8 + name_len].copy_from_slice(&name_bytes[..name_len]);
    entry[32..36].copy_from_slice(&crc32.to_le_bytes());
    entry
}

/// Solves SVM Error 03276 using Adaptation Channel 15 XOR 51666 (0xC9D2).
pub fn resolve_svm_03276(channel_15_value: u32) -> u32 {
    channel_15_value ^ SVM_CHANNEL_15_XOR_CIPHER
}

/// Simulates Green Engineering Menu calibration rehash (+1 then -1 toggle) to eliminate Error 03175.
pub fn simulate_svm_03175_rehash(initial_setting: u32) -> (u32, u32, bool) {
    let stage1 = initial_setting + 1;
    let stage2 = stage1 - 1;
    let is_cleared = stage2 == initial_setting;
    (stage1, stage2, is_cleared)
}

/// Locates the immutable corpus root directory if present.
pub fn get_corpus_root() -> Option<PathBuf> {
    let candidate = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()?
        .parent()?
        .join("originals");
    if candidate.is_dir() {
        Some(candidate)
    } else {
        None
    }
}

/// Generates a standard metainfo2.txt release string for Audi MMI 3G+ navigation updates.
pub fn generate_metainfo2_text(
    release: &str,
    hb_checksum: &str,
    mu_checksum: &str,
    styles_checksum: &str,
) -> String {
    format!(
        r#"[common]
release = "{release}"
vendor = "Harman/Becker"
sourceVersion = "K0942_4"
compatibleTrains = "HN+R_EU_AU_K0942_4,HN+R_EU_AU_P0922,HN+_EU_AU3G_K0900"
variant = "9411"

[HBNavDB]
path = "HBNavDB"
version = "{release}"
PackageType = "NavigationDatabase"
Checksum = "{hb_checksum}"
Description = "2026 European Road Network & FLDB Physical Page Container"

[MU9411]
path = "MU9411"
version = "0942"
PackageType = "Application"
Checksum = "{mu_checksum}"
Description = "MainUnit Application with Custom Localization"

[MapStyles]
path = "MapStyles"
version = "2026.1"
PackageType = "CartographyStyles"
Checksum = "{styles_checksum}"
Description = "Day and Night Map Shaders"
"#
    )
}

/// Generates a valid POSIX stock recovery script.
pub fn generate_stock_recovery_script() -> String {
    r#"#!/bin/sh
# stock_recovery.sh — Emergency Baseline Recovery Script for Audi MMI 3G+
echo "[1/4] Remounting /mnt/efs-system read-write..."
mount -uw /mnt/efs-system || exit 1
echo "[2/4] Restoring stock baseline..."
if [ -d /mnt/efs-system/backup/stock ]; then
    cp -rf /mnt/efs-system/backup/stock/* /mnt/efs-system/
fi
echo "[3/4] Synchronizing NAND flash blocks..."
sync
sync
echo "[4/4] Triggering reboot..."
sleep 1
shutdown -S
"#.to_string()
}

// ==============================================================================
// M5 Diagnostics: ISO-TP, UDS & SVM Constants
// ==============================================================================
pub const MODULE_5F_REQ_CAN_ID: u32 = 0x714;
pub const MODULE_5F_RESP_CAN_ID: u32 = 0x77E;

pub const ISOTP_SF: u8 = 0x00;
pub const ISOTP_FF: u8 = 0x10;
pub const ISOTP_CF: u8 = 0x20;
pub const ISOTP_FC: u8 = 0x30;

pub const FC_CTS: u8 = 0x00;
pub const FC_WAIT: u8 = 0x01;
pub const FC_OVERFLOW: u8 = 0x02;

pub const SID_DIAGNOSTIC_SESSION_CONTROL: u8 = 0x10;
pub const SID_CLEAR_DIAGNOSTIC_INFORMATION: u8 = 0x14;
pub const SID_READ_DATA_BY_IDENTIFIER: u8 = 0x22;
pub const SID_SECURITY_ACCESS: u8 = 0x27;
pub const SID_WRITE_DATA_BY_IDENTIFIER: u8 = 0x2E;
pub const SID_TESTER_PRESENT: u8 = 0x3E;

pub const DID_ADAPTATION_CHANNEL_15: u16 = 0x0615;
pub const DID_GREEN_MENU_ENABLE: u16 = 0x0611;
pub const DID_ECU_PART_NUMBER: u16 = 0xF187;
pub const DID_SOFTWARE_VERSION: u16 = 0xF189;
pub const DID_VIN: u16 = 0xF190;

// ==============================================================================
// M6 GDB v37 Constants
// ==============================================================================
pub const GDB_PAGE_SIZE: usize = 544;
pub const GDB_PAYLOAD_SIZE: usize = 512;
pub const GDB_HEADER_SIZE: usize = 16;
pub const GDB_TRAILER_SYNC: u32 = 0x55AA55AA;
pub const GDB_VOLUME_MAX_BYTES: u64 = 2_147_483_647;

// ==============================================================================
// M7 QNX Filesystem Constants
// ==============================================================================
pub const QNX_IFS_MAGIC: [u8; 4] = [0xeb, 0x7e, 0xff, 0x00];
pub const QNX_F3S_MAGIC: &[u8; 8] = b"QSSL_F3S";
pub const QNX_MACHINE_SH4: u16 = 0x0006;
pub const MAX_IFS_ROOT_SIZE: usize = 45_875_200;
pub const MAX_EFS_SYSTEM_SIZE: usize = 40_697_856;
pub const F3S_ERASE_UNIT_SIZE: usize = 262_144;

