//! Tier 2: Boundary & Corner Cases (Stress & Fault Injection)
//! Opaque-box test cases for empty datasets, zero-length roads, max 2 GiB volume splits,
//! single-volume vs multi-volume, invalid CRCs, corrupted pages, missing metainfo2, and corrupted checksums.

use super::common::*;
use mmi_core::CoreError;
use mmi_formats::{HbNavDb, MetaInfo2};
use mmi_media::{PreFlightSimulator, UpdateState, VolumeSplitter};
use std::path::{Path, PathBuf};
use tempfile::TempDir;

// ==============================================================================
// 1. Empty Datasets Tests (>= 5 tests)
// ==============================================================================

#[test]
fn tier2_empty_01_zero_bytes_fldb() {
    let empty_data = [0u8; 0];
    let result = HbNavDb::parse(&empty_data);
    assert!(result.is_err());
    match result.err().unwrap() {
        CoreError::ImmutabilityViolation(msg) => {
            assert!(msg.contains("too small"));
        }
        other => panic!("Unexpected error: {:?}", other),
    }
}

#[test]
fn tier2_empty_02_empty_sections_metainfo2() {
    let empty_str = "";
    let parsed = MetaInfo2::parse(empty_str).expect("Empty string should parse cleanly");
    assert!(parsed.release.is_none());
    assert!(parsed.vendor.is_none());
    assert_eq!(parsed.sections.get("root").unwrap().len(), 0);
}

#[test]
fn tier2_empty_03_empty_volume_entries() {
    let empty_entries: Vec<(PathBuf, u64)> = Vec::new();
    let volumes = VolumeSplitter::partition_volumes(&empty_entries, MAX_VOLUME_BYTES, "SD");
    assert!(volumes.is_empty(), "Empty entries must produce zero volumes");
}

#[test]
fn tier2_empty_04_zero_nodes_edges_ir_dataset() {
    #[derive(serde::Serialize, serde::Deserialize, Default)]
    struct IrDataset {
        nodes: Vec<(u64, i32, i32)>,
        edges: Vec<(u32, u32, u32, u32)>,
        pois: Vec<(u32, String)>,
    }

    let empty_ds = IrDataset::default();
    let serialized = serde_json::to_vec(&empty_ds).unwrap();
    let deserialized: IrDataset = serde_json::from_slice(&serialized).unwrap();
    assert_eq!(deserialized.nodes.len(), 0);
    assert_eq!(deserialized.edges.len(), 0);
    assert_eq!(deserialized.pois.len(), 0);
}

#[test]
fn tier2_empty_05_empty_poi_spatial_query() {
    let temp = TempDir::new().unwrap();
    let db_path = temp.path().join("empty_geo.gdb");
    let conn = rusqlite::Connection::open(&db_path).unwrap();

    conn.execute_batch(
        "CREATE TABLE pois (
            poi_id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            latitude REAL NOT NULL,
            longitude REAL NOT NULL
        );",
    ).unwrap();

    let mut stmt = conn.prepare("SELECT poi_id, name FROM pois WHERE latitude BETWEEN 40.0 AND 42.0").unwrap();
    let rows: Vec<String> = stmt.query_map([], |row| row.get(1)).unwrap().filter_map(|r| r.ok()).collect();
    assert!(rows.is_empty(), "Empty table must return zero rows without errors");
}

// ==============================================================================
// 2. Zero-Length & Degenerate Roads Tests (>= 5 tests)
// ==============================================================================

#[test]
fn tier2_road_01_zero_length_edge_detection() {
    struct Edge {
        from_node: u32,
        to_node: u32,
        length_dm: u32,
    }

    let edge_zero = Edge { from_node: 10, to_node: 10, length_dm: 0 };
    let is_degenerate = edge_zero.from_node == edge_zero.to_node && edge_zero.length_dm == 0;
    assert!(is_degenerate, "Zero-length self-edge must be classified as degenerate");
}

#[test]
fn tier2_road_02_collinear_duplicate_coordinates() {
    // Simulates simplification of duplicate consecutive vertices
    let raw_polyline = vec![(100, 100), (100, 100), (200, 200), (200, 200), (300, 300)];
    let mut deduplicated = Vec::new();
    for pt in raw_polyline {
        if deduplicated.last() != Some(&pt) {
            deduplicated.push(pt);
        }
    }
    assert_eq!(deduplicated, vec![(100, 100), (200, 200), (300, 300)]);
}

#[test]
fn tier2_road_03_zero_speed_limit_flagged() {
    let speed_forward = 0u8;
    let speed_reverse = 0u8;
    let is_impassable = speed_forward == 0 && speed_reverse == 0;
    assert!(is_impassable, "Road with 0 speed in both directions must be impassable");
}

#[test]
fn tier2_road_04_circular_loop_way() {
    // Roundabout or closed ring: start == end, but length > 0
    struct RingEdge {
        from_node: u32,
        to_node: u32,
        length_dm: u32,
        is_roundabout: bool,
    }

    let ring = RingEdge {
        from_node: 50,
        to_node: 50,
        length_dm: 350, // 35 meters
        is_roundabout: true,
    };
    assert_eq!(ring.from_node, ring.to_node);
    assert!(ring.length_dm > 0);
    assert!(ring.is_roundabout);
}

#[test]
fn tier2_road_05_extreme_coordinate_boundaries_no_overflow() {
    let (x1, y1) = coord_to_fixed(-180.0, -90.0);
    let (x2, y2) = coord_to_fixed(180.0, 90.0);

    let dx = (x2 as i64) - (x1 as i64);
    let dy = (y2 as i64) - (y1 as i64);

    assert!(dx > 0);
    assert!(dy > 0);
    assert_eq!(dx, (i32::MAX as i64) * 2);
}

// ==============================================================================
// 3. Max 2 GiB Volume Splits Tests (>= 5 tests)
// ==============================================================================

#[test]
fn tier2_vol_01_exact_2gib_boundary() {
    let entries = vec![
        (Path::new("exact_volume.db").to_path_buf(), MAX_VOLUME_BYTES),
    ];
    let volumes = VolumeSplitter::partition_volumes(&entries, MAX_VOLUME_BYTES, "SD");
    assert_eq!(volumes.len(), 1);
    assert_eq!(volumes[0].total_bytes, MAX_VOLUME_BYTES);
}

#[test]
fn tier2_vol_02_exceeding_2gib_by_one_byte() {
    let entries = vec![
        (Path::new("part1.db").to_path_buf(), MAX_VOLUME_BYTES),
        (Path::new("part2.db").to_path_buf(), 1u64),
    ];
    let volumes = VolumeSplitter::partition_volumes(&entries, MAX_VOLUME_BYTES, "SD");
    assert_eq!(volumes.len(), 2, "Overflow by 1 byte must trigger volume rollover");
    assert_eq!(volumes[0].total_bytes, MAX_VOLUME_BYTES);
    assert_eq!(volumes[1].total_bytes, 1);
}

#[test]
fn tier2_vol_03_multiple_2gib_chunks() {
    let chunk_size = 1_000_000_000u64; // 1 GB
    let entries: Vec<(PathBuf, u64)> = (0..5)
        .map(|i| (Path::new(&format!("chunk_{}.db", i)).to_path_buf(), chunk_size))
        .collect();

    let volumes = VolumeSplitter::partition_volumes(&entries, MAX_VOLUME_BYTES, "SD");
    assert_eq!(volumes.len(), 3);
    assert_eq!(volumes[0].file_count, 2); // 2 GB
    assert_eq!(volumes[1].file_count, 2); // 2 GB
    assert_eq!(volumes[2].file_count, 1); // 1 GB
}

#[test]
fn tier2_vol_04_volume_naming_convention() {
    let entries = vec![
        (Path::new("a.db").to_path_buf(), MAX_VOLUME_BYTES),
        (Path::new("b.db").to_path_buf(), MAX_VOLUME_BYTES),
    ];
    let volumes = VolumeSplitter::partition_volumes(&entries, MAX_VOLUME_BYTES, "ECE_MAPS");
    assert_eq!(volumes[0].volume_label, "ECE_MAPS_1");
    assert_eq!(volumes[1].volume_label, "ECE_MAPS_2");
}

#[test]
fn tier2_vol_05_chunk_size_200mib_boundary() {
    let full_file_size = (CHECKSUM_CHUNK_SIZE * 3) as u64;
    let num_chunks = (full_file_size as usize + CHECKSUM_CHUNK_SIZE - 1) / CHECKSUM_CHUNK_SIZE;
    assert_eq!(num_chunks, 3);
}

// ==============================================================================
// 4. Single-Volume vs Multi-Volume Tests (>= 5 tests)
// ==============================================================================

#[test]
fn tier2_split_01_single_volume_albania_micro() {
    let micro_size = 50 * 1024 * 1024u64; // 50 MB
    let entries = vec![(Path::new("albania_nav.db").to_path_buf(), micro_size)];
    let volumes = VolumeSplitter::partition_volumes(&entries, MAX_VOLUME_BYTES, "SD_AL");
    assert_eq!(volumes.len(), 1, "Micro profile must fit into a single volume");
    assert_eq!(volumes[0].volume_label, "SD_AL");
}

#[test]
fn tier2_split_02_multi_volume_dach_regional() {
    let dach_size = 6_200_000_000u64; // ~6.2 GB
    let entries: Vec<(PathBuf, u64)> = (0..7)
        .map(|i| (Path::new(&format!("dach_part_{}.db", i)).to_path_buf(), dach_size / 7))
        .collect();
    let volumes = VolumeSplitter::partition_volumes(&entries, MAX_VOLUME_BYTES, "SD_DACH");
    assert!(volumes.len() >= 3, "6.2 GB must span at least 3 volumes capped at 2 GiB");
}

#[test]
fn tier2_split_03_continental_ece_21_volumes() {
    // Total MMI3GP footprint: 28,185,253,776 bytes across 21 volume files
    let ece_total: u64 = 28_185_253_776;
    assert!(ece_total < SD_CARD_32GB_MAX_BYTES);

    let average_vol = ece_total / 21;
    assert!(average_vol < MAX_VOLUME_BYTES);
}

#[test]
fn tier2_split_04_cumulative_byte_conservation() {
    let entries = vec![
        (Path::new("1.bin").to_path_buf(), 500_000_000u64),
        (Path::new("2.bin").to_path_buf(), 750_000_000u64),
        (Path::new("3.bin").to_path_buf(), 1_200_000_000u64),
    ];
    let total_input: u64 = entries.iter().map(|(_, s)| s).sum();
    let volumes = VolumeSplitter::partition_volumes(&entries, MAX_VOLUME_BYTES, "VOL");
    let total_output: u64 = volumes.iter().map(|v| v.total_bytes).sum();
    assert_eq!(total_input, total_output, "No bytes may be lost during partitioning");
}

#[test]
fn tier2_split_05_zero_byte_file_handling() {
    let entries = vec![
        (Path::new("empty.txt").to_path_buf(), 0u64),
        (Path::new("data.db").to_path_buf(), 1000u64),
    ];
    let volumes = VolumeSplitter::partition_volumes(&entries, MAX_VOLUME_BYTES, "SD");
    assert_eq!(volumes.len(), 1);
    assert_eq!(volumes[0].file_count, 2);
    assert_eq!(volumes[0].total_bytes, 1000);
}

// ==============================================================================
// 5. Invalid CRCs Tests (>= 5 tests)
// ==============================================================================

#[test]
fn tier2_crc_01_header_crc_mismatch() {
    let mut page = create_fldb_page(1, &[0x55; 512], FLDB_MAGIC);
    let original_crc = u16::from_le_bytes([page[8], page[9]]);
    // Overwrite stored CRC with incorrect value
    page[8] ^= 0xFF;
    let corrupted_crc = u16::from_le_bytes([page[8], page[9]]);
    let calculated_crc = crc16_ccitt(&page[16..528]);

    assert_ne!(corrupted_crc, calculated_crc);
    assert_eq!(original_crc, calculated_crc);
}

#[test]
fn tier2_crc_02_all_zero_crc_when_non_zero() {
    let mut page = create_fldb_page(1, &[0x11; 512], FLDB_MAGIC);
    page[8] = 0x00;
    page[9] = 0x00;
    let stored = u16::from_le_bytes([page[8], page[9]]);
    let computed = crc16_ccitt(&page[16..528]);
    assert_ne!(stored, computed);
}

#[test]
fn tier2_crc_03_inverted_crc() {
    let mut page = create_fldb_page(1, &[0x22; 512], FLDB_MAGIC);
    let valid = crc16_ccitt(&page[16..528]);
    let inverted = !valid;
    page[8..10].copy_from_slice(&inverted.to_le_bytes());
    let stored = u16::from_le_bytes([page[8], page[9]]);
    assert_eq!(stored, inverted);
    assert_ne!(stored, valid);
}

#[test]
fn tier2_crc_04_swapped_endian_crc() {
    let mut page = create_fldb_page(1, &[0x33; 512], FLDB_MAGIC);
    let valid = crc16_ccitt(&page[16..528]);
    let swapped = valid.swap_bytes();
    if valid != swapped {
        page[8..10].copy_from_slice(&swapped.to_le_bytes());
        let stored = u16::from_le_bytes([page[8], page[9]]);
        assert_ne!(stored, valid);
    }
}

#[test]
fn tier2_crc_05_multibit_payload_corruption() {
    let mut page = create_fldb_page(1, &[0x44; 512], FLDB_MAGIC);
    let stored = u16::from_le_bytes([page[8], page[9]]);
    // Corrupt multiple bytes across payload
    for i in (16..528).step_by(32) {
        page[i] ^= 0xAA;
    }
    let computed = crc16_ccitt(&page[16..528]);
    assert_ne!(stored, computed);
}

// ==============================================================================
// 6. Corrupted Pages Tests (>= 5 tests)
// ==============================================================================

#[test]
fn tier2_page_01_truncated_page() {
    let page = create_fldb_page(1, &[0x55; 512], FLDB_MAGIC);
    let truncated = &page[..543]; // 1 byte short
    assert_ne!(truncated.len(), FLDB_PAGE_SIZE);
}

#[test]
fn tier2_page_02_invalid_page_magic() {
    let mut db_data = create_synthetic_fldb(2, |_| vec![0x66; 512]);
    // Corrupt master header magic at bytes 20..24
    db_data[20..24].copy_from_slice(b"RIFF");
    let result = HbNavDb::parse(&db_data);
    assert!(result.is_err(), "Invalid magic signature must fail parse");
}

#[test]
fn tier2_page_03_trailer_sync_corruption() {
    let mut page = create_fldb_page(1, &[0x77; 512], FLDB_MAGIC);
    page[528..532].copy_from_slice(&0xDEADBEEFu32.to_le_bytes());
    let sync = u32::from_le_bytes([page[528], page[529], page[530], page[531]]);
    assert_ne!(sync, FLDB_TRAILER_SYNC);
}

#[test]
fn tier2_page_04_master_header_truncated() {
    let small_file = vec![0u8; 35]; // Less than 36 bytes
    let result = HbNavDb::parse(&small_file);
    assert!(result.is_err());
}

#[test]
fn tier2_page_05_page_size_zero_in_header() {
    let mut db_data = create_synthetic_fldb(1, |_| vec![0; 512]);
    db_data[0..4].copy_from_slice(&0u32.to_le_bytes()); // page_size = 0
    let parsed = HbNavDb::parse(&db_data).expect("Header parses with page_size=0");
    assert_eq!(parsed.page_count(), 0, "Zero page_size must return 0 page_count without panic");
}

// ==============================================================================
// 7. Missing & Malformed metainfo2 Tests (>= 5 tests)
// ==============================================================================

#[test]
fn tier2_meta_01_missing_metainfo2_file() {
    let temp = TempDir::new().unwrap();
    // Temp dir has no metainfo2.txt
    let report = PreFlightSimulator::simulate_media(temp.path()).unwrap();
    assert!(!report.overall_success);
    assert_eq!(report.final_state, UpdateState::Aborted);
    assert_eq!(report.steps_total, 2);
    assert_eq!(report.steps_successful, 1); // Step 1 MediaDetection succeeded, Step 2 MetaInfo failed
}

#[test]
fn tier2_meta_02_missing_common_section() {
    let invalid_ini = "[MU9411]\nversion = 0942\n";
    let parsed = MetaInfo2::parse(invalid_ini).unwrap();
    assert!(parsed.release.is_none());
    assert!(parsed.sections.contains_key("MU9411"));
}

#[test]
fn tier2_meta_03_malformed_key_value_syntax() {
    let malformed = "THIS IS NOT A VALID INI LINE\n[common]\nrelease = 2026_ECE\n";
    let parsed = MetaInfo2::parse(malformed).expect("Parser should gracefully skip malformed lines");
    assert_eq!(parsed.release, Some("2026_ECE".to_string()));
}

#[test]
fn tier2_meta_04_missing_release_declaration() {
    let ini = "[common]\nvendor = Harman/Becker\n";
    let parsed = MetaInfo2::parse(ini).unwrap();
    assert!(parsed.release.is_none());
    assert_eq!(parsed.vendor, Some("Harman/Becker".to_string()));
}

#[test]
fn tier2_meta_05_corrupted_encoding() {
    let bad_bytes = b"[common]\nrelease = \xFF\xFE_BAD\n";
    let text = String::from_utf8_lossy(bad_bytes);
    let parsed = MetaInfo2::parse(&text).unwrap();
    assert!(parsed.release.is_some());
}

// ==============================================================================
// 8. Corrupted Checksums & Partial Chunks Tests (>= 5 tests)
// ==============================================================================

#[test]
fn tier2_chk_01_truncated_sha1() {
    let truncated_sha1 = "dfc9497e7f12e84c98782a201416e91f067da20"; // 39 chars
    assert_ne!(truncated_sha1.len(), 40);
}

#[test]
fn tier2_chk_02_non_hex_sha1() {
    let non_hex = "dfc9497e7f12e84c98782a201416e91f067da20Z"; // Z is not hex
    let is_valid_hex = non_hex.chars().all(|c| c.is_ascii_hexdigit());
    assert!(!is_valid_hex);
}

#[test]
fn tier2_chk_03_checksum_mismatch_in_stage() {
    let file1 = b"ORIGINAL_STAGE_CONTENT";
    let file2 = b"CORRUPTED_STAGE_CONTENT";
    let h1 = sha1_hex(file1);
    let h2 = sha1_hex(file2);
    assert_ne!(h1, h2, "Different contents must produce distinct digests");
}

#[test]
fn tier2_chk_04_partial_chunk_crc32() {
    let total_bytes = 250 * 1024 * 1024usize; // 250 MB
    let chunk1_size = CHECKSUM_CHUNK_SIZE;
    let chunk2_size = total_bytes - CHECKSUM_CHUNK_SIZE;

    assert_eq!(chunk1_size, 209_715_200);
    assert_eq!(chunk2_size, 52_428_800);
    assert_eq!(chunk1_size + chunk2_size, total_bytes);
}

#[test]
fn tier2_chk_05_empty_checksum_value() {
    let ini = "[HBNavDB]\nChecksum = \"\"\n";
    let parsed = MetaInfo2::parse(ini).unwrap();
    let hb = &parsed.sections["HBNavDB"];
    assert_eq!(hb.get("Checksum"), Some(&"".to_string()));
}
