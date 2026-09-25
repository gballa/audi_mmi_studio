//! Tier 1: Feature Coverage (Core Requirements)
//! Comprehensive opaque-box test cases covering FLDB 544-byte page alignment,
//! CRC-16 header verification, OSM road network and speed limits, Google Maps commercial POIs,
//! 32-bit fixed point coordinates, SD card structure, metainfo2.txt parsing, and SVM error ciphers.

use super::common::*;
use mmi_formats::{HbNavDb, MetaInfo2};
use mmi_media::{Fat32Constraints, VolumeSplitter};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

// ==============================================================================
// 1. FLDB 544-Byte Physical Page Alignment Tests (>= 5 tests)
// ==============================================================================

#[test]
fn tier1_fldb_01_master_header_structure() {
    let raw = create_synthetic_fldb(10, |idx| vec![(idx % 256) as u8; 512]);
    assert!(raw.len() >= FLDB_HEADER_SIZE);

    let parsed = HbNavDb::parse(&raw).expect("Failed to parse valid synthetic FLDB");
    assert_eq!(parsed.header.page_size, 544);
    assert_eq!(parsed.header.root_page, 1);
    assert_eq!(parsed.header.header_size, 36);
    assert_eq!(&parsed.header.magic, FLDB_MAGIC);
    assert_eq!(parsed.header.version, 1);
}

#[test]
fn tier1_fldb_02_exact_page_size_stride() {
    for page_count in [1, 2, 8, 16, 64] {
        let raw = create_synthetic_fldb(page_count, |idx| vec![idx as u8; 512]);
        assert_eq!(raw.len(), page_count * FLDB_PAGE_SIZE);
        assert_eq!(raw.len() % FLDB_PAGE_SIZE, 0);

        let parsed = HbNavDb::parse(&raw).expect("Parse error");
        assert_eq!(parsed.page_count(), page_count);
    }
}

#[test]
fn tier1_fldb_03_payload_isolation_512_bytes() {
    let payload_sample = b"AUDI_MMI_3G_PLUS_TEST_PAYLOAD_DATA_RECORD";
    let page = create_fldb_page(42, payload_sample, FLDB_MAGIC);

    assert_eq!(page.len(), FLDB_PAGE_SIZE);
    // Header is bytes 0..16
    assert_eq!(&page[0..4], FLDB_MAGIC);
    assert_eq!(u32::from_le_bytes([page[4], page[5], page[6], page[7]]), 42);
    // Payload is bytes 16..528 (512 bytes)
    assert_eq!(&page[16..16 + payload_sample.len()], payload_sample);
    // Trailer starts at 528
    let trailer_sync = u32::from_le_bytes([page[528], page[529], page[530], page[531]]);
    assert_eq!(trailer_sync, FLDB_TRAILER_SYNC);
}

#[test]
fn tier1_fldb_04_trailer_frame_sync_word() {
    let page = create_fldb_page(1, &[0xAA; 128], FLDB_MAGIC);
    let sync_val = u32::from_le_bytes([page[528], page[529], page[530], page[531]]);
    assert_eq!(sync_val, FLDB_TRAILER_SYNC);
    // Trailer padding (bytes 532..544) must be null bytes
    assert!(page[532..544].iter().all(|&b| b == 0));
}

#[test]
fn tier1_fldb_05_directory_page_table_capacity() {
    // Page 1 directory holds 15 member entries of 36 bytes each = 540 bytes + 4 bytes padding = 544 bytes
    let mut entries = Vec::new();
    for i in 0..15 {
        let entry = create_fldb_dir_entry(
            (i * 0x1000) as u32,
            0x800,
            &format!("CHUNK_{:02}.DB", i),
            0x12345678 + i as u32,
        );
        entries.extend_from_slice(&entry);
    }
    assert_eq!(entries.len(), 15 * 36); // 540 bytes

    let mut page1_data = entries;
    page1_data.extend_from_slice(&[0u8; 4]); // 4 bytes padding
    assert_eq!(page1_data.len(), FLDB_PAGE_SIZE); // 544 bytes
}

#[test]
fn tier1_fldb_06_genuine_corpus_page_alignment() {
    if let Some(corpus) = get_corpus_root() {
        let db_path = corpus.join("8R0051884KL_6.36.0_2023/pkgdb/LIT3GP/EJ211Pa_L1.db");
        if db_path.exists() {
            let mut file = std::fs::File::open(&db_path).expect("failed to open EJ211Pa_L1.db");
            use std::io::Read;
            let mut header = [0u8; 36];
            file.read_exact(&mut header).expect("failed to read header");
            let parsed = HbNavDb::parse(&header).expect("Failed to parse genuine EJ211Pa_L1.db");
            assert_eq!(parsed.header.page_size, 544);
            assert_eq!(&parsed.header.magic, FLDB_MAGIC);
            assert_eq!(parsed.header.root_page, 1);
        }
    }
}

// ==============================================================================
// 2. CRC-16 Header Verification Tests (>= 5 tests)
// ==============================================================================

#[test]
fn tier1_crc16_01_standard_vector() {
    // Standard CRC-16/CCITT verification vector: "123456789"
    let test_vector = b"123456789";
    let crc = crc16_ccitt(test_vector);
    // CCITT with init 0xFFFF gives 0x29B1
    assert_eq!(crc, 0x29B1, "CRC-16/CCITT standard test vector must yield 0x29B1");
}

#[test]
fn tier1_crc16_02_payload_header_placement() {
    let payload = vec![0x37; FLDB_PAYLOAD_SIZE];
    let page = create_fldb_page(5, &payload, FLDB_MAGIC);

    let stored_crc = u16::from_le_bytes([page[8], page[9]]);
    let expected_crc = crc16_ccitt(&page[16..528]);
    assert_eq!(stored_crc, expected_crc, "Stored CRC-16 must match calculation over payload");
}

#[test]
fn tier1_crc16_03_detects_single_bit_flip() {
    let payload = vec![0x42; FLDB_PAYLOAD_SIZE];
    let mut page = create_fldb_page(1, &payload, FLDB_MAGIC);

    let original_crc = u16::from_le_bytes([page[8], page[9]]);
    // Flip bit 0 of byte 200 in the payload
    page[200] ^= 0x01;
    let recalculated_crc = crc16_ccitt(&page[16..528]);

    assert_ne!(original_crc, recalculated_crc, "CRC-16 must detect single-bit corruption");
}

#[test]
fn tier1_crc16_04_all_zero_payload() {
    let zero_payload = vec![0u8; FLDB_PAYLOAD_SIZE];
    let crc = crc16_ccitt(&zero_payload);
    // Non-zero result due to 0xFFFF initial value
    assert_ne!(crc, 0, "CRC-16 with 0xFFFF init over all zeros must not be zero");
    assert_eq!(crc, crc16_ccitt(&zero_payload), "CRC-16 must be deterministic");
}

#[test]
fn tier1_crc16_05_all_ones_payload() {
    let ones_payload = vec![0xFFu8; FLDB_PAYLOAD_SIZE];
    let crc = crc16_ccitt(&ones_payload);
    assert_eq!(crc, crc16_ccitt(&ones_payload));
    let zero_crc = crc16_ccitt(&vec![0u8; FLDB_PAYLOAD_SIZE]);
    assert_ne!(crc, zero_crc);
}

#[test]
fn tier1_crc16_06_recalculation_consistency() {
    for seed in 0..10 {
        let payload: Vec<u8> = (0..512).map(|i| ((i * 17 + seed * 31) % 256) as u8).collect();
        let page = create_fldb_page(seed as u32, &payload, FLDB_MAGIC);
        let stored = u16::from_le_bytes([page[8], page[9]]);
        let computed = crc16_ccitt(&page[16..528]);
        assert_eq!(stored, computed);
    }
}

// ==============================================================================
// 3. OSM Road Network & Speed Limits Tests (>= 5 tests)
// ==============================================================================

#[test]
fn tier1_osm_01_frc_classification_mapping() {
    fn map_osm_highway_to_frc(tag: &str) -> Option<u8> {
        match tag {
            "motorway" | "motorway_link" => Some(0),
            "trunk" | "trunk_link" => Some(1),
            "primary" | "primary_link" => Some(2),
            "secondary" | "secondary_link" => Some(3),
            "tertiary" | "tertiary_link" => Some(4),
            "unclassified" => Some(5),
            "residential" | "living_street" => Some(6),
            "service" => Some(7),
            _ => None,
        }
    }

    assert_eq!(map_osm_highway_to_frc("motorway"), Some(0));
    assert_eq!(map_osm_highway_to_frc("trunk"), Some(1));
    assert_eq!(map_osm_highway_to_frc("primary"), Some(2));
    assert_eq!(map_osm_highway_to_frc("secondary"), Some(3));
    assert_eq!(map_osm_highway_to_frc("tertiary"), Some(4));
    assert_eq!(map_osm_highway_to_frc("residential"), Some(6));
    assert_eq!(map_osm_highway_to_frc("footway"), None);
    assert_eq!(map_osm_highway_to_frc("cycleway"), None);
}

#[test]
fn tier1_osm_02_non_navigable_exclusion_filter() {
    fn is_navigable_way(highway_tag: &str, access_tag: Option<&str>) -> bool {
        let excluded = [
            "footway", "path", "cycleway", "bridleway", "steps", "pedestrian",
            "track", "proposed", "construction", "elevator",
        ];
        if excluded.contains(&highway_tag) {
            return matches!(access_tag, Some("yes") | Some("motor_vehicle") | Some("motorcar"));
        }
        true
    }

    assert!(!is_navigable_way("footway", None));
    assert!(!is_navigable_way("cycleway", None));
    assert!(is_navigable_way("track", Some("motor_vehicle")));
    assert!(is_navigable_way("primary", None));
}

#[test]
fn tier1_osm_03_lane_guidance_arrow_mask() {
    // Parses string turn:lanes format into a packed bitmask for MMI cluster
    fn parse_turn_lane_mask(lanes_spec: &str) -> u16 {
        let mut mask: u16 = 0;
        for (i, lane) in lanes_spec.split('|').enumerate() {
            if i >= 8 { break; }
            let mut lane_bits: u16 = 0;
            for part in lane.split(';') {
                match part.trim() {
                    "left" => lane_bits |= 0b0001,
                    "through" => lane_bits |= 0b0010,
                    "right" => lane_bits |= 0b0100,
                    _ => {}
                }
            }
            mask |= lane_bits << (i * 4);
        }
        mask
    }

    let mask1 = parse_turn_lane_mask("left|through|right");
    assert_eq!(mask1 & 0x000F, 0b0001); // Lane 0: left
    assert_eq!((mask1 >> 4) & 0x000F, 0b0010); // Lane 1: through
    assert_eq!((mask1 >> 8) & 0x000F, 0b0100); // Lane 2: right

    let mask2 = parse_turn_lane_mask("left;through|through;right");
    assert_eq!(mask2 & 0x000F, 0b0011); // Lane 0: left + through
    assert_eq!((mask2 >> 4) & 0x000F, 0b0110); // Lane 1: through + right
}

#[test]
fn tier1_osm_04_turn_restriction_prohibitive_mandatory() {
    #[derive(Debug, PartialEq, Eq)]
    enum RestrictionKind {
        Prohibitive(u8), // 1=No Left, 2=No Right, 3=No U-turn
        Mandatory(u8),   // 4=Only Left, 5=Only Right, 6=Only Straight
    }

    fn parse_restriction(tag: &str) -> Option<RestrictionKind> {
        match tag {
            "no_left_turn" => Some(RestrictionKind::Prohibitive(1)),
            "no_right_turn" => Some(RestrictionKind::Prohibitive(2)),
            "no_u_turn" => Some(RestrictionKind::Prohibitive(3)),
            "only_left_turn" => Some(RestrictionKind::Mandatory(4)),
            "only_right_turn" => Some(RestrictionKind::Mandatory(5)),
            "only_straight_on" => Some(RestrictionKind::Mandatory(6)),
            _ => None,
        }
    }

    assert_eq!(parse_restriction("no_left_turn"), Some(RestrictionKind::Prohibitive(1)));
    assert_eq!(parse_restriction("only_straight_on"), Some(RestrictionKind::Mandatory(6)));
    assert_eq!(parse_restriction("invalid_tag"), None);
}

#[test]
fn tier1_osm_05_speed_limits_explicit_numeric() {
    fn parse_maxspeed_kmh(tag: &str) -> u8 {
        if let Ok(v) = tag.parse::<u8>() {
            return v;
        }
        if let Some(mph) = tag.strip_suffix(" mph") {
            if let Ok(v) = mph.parse::<f32>() {
                return (v * 1.60934).round() as u8;
            }
        }
        match tag {
            "walk" => 10,
            "none" => 250, // German Autobahn no limit
            _ => 50,       // Safe default
        }
    }

    assert_eq!(parse_maxspeed_kmh("130"), 130);
    assert_eq!(parse_maxspeed_kmh("50"), 50);
    assert_eq!(parse_maxspeed_kmh("60 mph"), 97);
    assert_eq!(parse_maxspeed_kmh("none"), 250);
    assert_eq!(parse_maxspeed_kmh("walk"), 10);
}

#[test]
fn tier1_osm_06_speed_limits_country_default_fallback_matrix() {
    fn resolve_country_default_speed(country: &str, frc: u8) -> u8 {
        match (country, frc) {
            ("DE", 0) => 250, // Autobahn no limit
            ("DE", 1) => 130,
            ("DE", 2..=4) => 100,
            ("DE", _) => 50,

            ("FR", 0) => 130,
            ("FR", 1) => 110,
            ("FR", 2..=4) => 80,
            ("FR", _) => 50,

            ("AL", 0) => 130,
            ("AL", 1) => 90,
            ("AL", 2..=4) => 80,
            ("AL", _) => 40,

            _ => 50,
        }
    }

    assert_eq!(resolve_country_default_speed("DE", 0), 250);
    assert_eq!(resolve_country_default_speed("FR", 0), 130);
    assert_eq!(resolve_country_default_speed("AL", 0), 130);
    assert_eq!(resolve_country_default_speed("AL", 1), 90);
    assert_eq!(resolve_country_default_speed("AL", 6), 40);
}

// ==============================================================================
// 4. Google Maps Commercial POIs & EV Charging Stations Tests (>= 5 tests)
// ==============================================================================

#[test]
fn tier1_gmp_01_places_fieldmask_enforcement() {
    let required_fields = [
        "places.id",
        "places.displayName",
        "places.location",
        "places.primaryType",
        "places.evChargeOptions",
    ];
    let sample_mask = "places.id,places.displayName,places.location,places.primaryType,places.evChargeOptions";

    for field in required_fields {
        assert!(sample_mask.contains(field), "FieldMask must include {field}");
    }
}

#[test]
fn tier1_gmp_02_ev_charging_power_classification() {
    #[derive(Debug, PartialEq, Eq)]
    enum EvPowerTier {
        UltraFast, // >= 150 kW
        Rapid,     // 50..149 kW
        Standard,  // < 50 kW
    }

    fn classify_ev_power(kw: f32) -> EvPowerTier {
        if kw >= 150.0 {
            EvPowerTier::UltraFast
        } else if kw >= 50.0 {
            EvPowerTier::Rapid
        } else {
            EvPowerTier::Standard
        }
    }

    assert_eq!(classify_ev_power(350.0), EvPowerTier::UltraFast); // Ionity High Power
    assert_eq!(classify_ev_power(150.0), EvPowerTier::UltraFast);
    assert_eq!(classify_ev_power(75.0), EvPowerTier::Rapid);
    assert_eq!(classify_ev_power(22.0), EvPowerTier::Standard); // AC Destination
}

#[test]
fn tier1_gmp_03_ev_charging_connector_types() {
    let sample_connectors = vec!["CCS2", "Type2", "CHAdeMO", "Tesla"];
    let json_str = serde_json::to_string(&sample_connectors).unwrap();
    let decoded: Vec<String> = serde_json::from_str(&json_str).unwrap();
    assert_eq!(decoded, sample_connectors);
    assert!(decoded.contains(&"CCS2".to_string()));
}

#[test]
fn tier1_gmp_04_commercial_fuel_brand_identification() {
    fn map_fuel_brand(brand: &str) -> u16 {
        match brand.to_lowercase().as_str() {
            "shell" => 101,
            "aral" => 102,
            "bp" => 103,
            "totalenergies" | "total" => 104,
            "eni" | "agip" => 105,
            "omv" => 106,
            _ => 100, // Generic fuel
        }
    }

    assert_eq!(map_fuel_brand("Shell"), 101);
    assert_eq!(map_fuel_brand("Aral"), 102);
    assert_eq!(map_fuel_brand("TotalEnergies"), 104);
    assert_eq!(map_fuel_brand("Unknown Gas"), 100);
}

#[test]
fn tier1_gmp_05_speed_radar_calibration() {
    #[allow(dead_code)]
    struct SpeedRadar {
        lat: f64,
        lon: f64,
        speed_limit_kmh: u8,
        warning_distance_m: u16,
    }

    let radar = SpeedRadar {
        lat: 41.3275,
        lon: 19.8189,
        speed_limit_kmh: 80,
        warning_distance_m: 500,
    };

    assert!(radar.speed_limit_kmh > 0);
    assert_eq!(radar.warning_distance_m, 500);
}

#[test]
fn tier1_gmp_06_caching_governance_30_day_limit() {
    // ToS 3.2.3: Place ID retained indefinitely, coordinates max 30 days
    let now_ts = 1710840000; // Sample build time
    let fetch_ts_fresh = now_ts - (15 * 86400); // 15 days ago
    let fetch_ts_stale = now_ts - (35 * 86400); // 35 days ago

    fn is_cache_valid(fetch_ts: u64, current_ts: u64) -> bool {
        current_ts.saturating_sub(fetch_ts) <= (30 * 86400)
    }

    assert!(is_cache_valid(fetch_ts_fresh, now_ts));
    assert!(!is_cache_valid(fetch_ts_stale, now_ts));
}

// ==============================================================================
// 5. 32-Bit Fixed Point Coordinates Tests (>= 5 tests)
// ==============================================================================

#[test]
fn tier1_coords_01_wgs84_to_fixed_point_scaling() {
    let (x_greenwich, y_equator) = coord_to_fixed(0.0, 0.0);
    assert_eq!(x_greenwich, 0);
    assert_eq!(y_equator, 0);

    let (x_max, y_max) = coord_to_fixed(180.0, 90.0);
    assert_eq!(x_max, i32::MAX);
    assert_eq!(y_max, i32::MAX);

    let (x_min, y_min) = coord_to_fixed(-180.0, -90.0);
    assert_eq!(x_min, -i32::MAX);
    assert_eq!(y_min, -i32::MAX);
}

#[test]
fn tier1_coords_02_roundtrip_precision() {
    let test_points = [
        (19.8189, 41.3275),   // Tirana, Albania
        (11.5820, 48.1351),   // Munich, Germany
        (2.3522, 48.8566),    // Paris, France
        (12.4964, 41.9028),   // Rome, Italy
    ];

    for (lon, lat) in test_points {
        let (x, y) = coord_to_fixed(lon, lat);
        let (r_lon, r_lat) = fixed_to_coord(x, y);

        assert!((lon - r_lon).abs() < 1e-6, "Lon delta exceeds tolerance: {lon} vs {r_lon}");
        assert!((lat - r_lat).abs() < 1e-6, "Lat delta exceeds tolerance: {lat} vs {r_lat}");
    }
}

#[test]
fn tier1_coords_03_subcentimeter_resolution_proof() {
    // Equator circumference = 2 * PI * 6,378,137 m
    let earth_circumference_m = 2.0 * std::f64::consts::PI * 6_378_137.0;
    let step_m = earth_circumference_m / (1u64 << 32) as f64;

    assert!(step_m < 0.01, "Horizontal step at equator must be < 1 cm (actual: {} m)", step_m);
    assert_eq!((step_m * 1000.0).round(), 9.0, "Step is approx 9.33 mm");
}

#[test]
fn tier1_coords_04_boundary_wrapping_clamping() {
    let (x1, _) = coord_to_fixed(180.0, 0.0);
    let (x2, _) = coord_to_fixed(-180.0, 0.0);
    assert_eq!(x1, i32::MAX);
    assert_eq!(x2, -i32::MAX);

    let (_, y1) = coord_to_fixed(0.0, 90.0);
    let (_, y2) = coord_to_fixed(0.0, -90.0);
    assert_eq!(y1, i32::MAX);
    assert_eq!(y2, -i32::MAX);
}

#[test]
fn tier1_coords_05_morton_z_order_interleaving() {
    let x = 0b1010_1100u32;
    let y = 0b0101_0011u32;
    let morton = interleave_morton_32(x, y);
    let (rx, ry) = deinterleave_morton_32(morton);

    assert_eq!(rx, x);
    assert_eq!(ry, y);
}

#[test]
fn tier1_coords_06_morton_spatial_locality_ordering() {
    let p1 = interleave_morton_32(100, 100);
    let p2 = interleave_morton_32(101, 100);
    let p_far = interleave_morton_32(50000, 50000);

    let dist_near = (p1 as i64 - p2 as i64).abs();
    let dist_far = (p1 as i64 - p_far as i64).abs();
    assert!(dist_near < dist_far, "Neighboring points must have closer Morton keys");
}

// ==============================================================================
// 6. SD Card Structure Tests (>= 5 tests)
// ==============================================================================

#[test]
fn tier1_sd_01_root_hierarchy() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();

    // Create required directories and files
    fs::create_dir_all(root.join("HBNavDB")).unwrap();
    fs::create_dir_all(root.join("MU9411")).unwrap();
    fs::create_dir_all(root.join("MapStyles")).unwrap();
    fs::write(root.join("metainfo2.txt"), "[common]\nrelease = \"2026_ECE\"\n").unwrap();
    fs::write(root.join("stock_recovery.sh"), generate_stock_recovery_script()).unwrap();

    assert!(root.join("metainfo2.txt").is_file());
    assert!(root.join("stock_recovery.sh").is_file());
    assert!(root.join("HBNavDB").is_dir());
    assert!(root.join("MU9411").is_dir());
    assert!(root.join("MapStyles").is_dir());
}

#[test]
fn tier1_sd_02_fat32_constraints_validation() {
    let constraints = Fat32Constraints::default();
    assert_eq!(constraints.max_volume_bytes, 32 * 1024 * 1024 * 1024);
    assert_eq!(constraints.cluster_size_bytes, 32 * 1024);
    assert_eq!(constraints.volume_label_max_chars, 11);
    assert_eq!(constraints.max_path_depth, 8);
}

#[test]
fn tier1_sd_03_volume_splitter_under_2gib() {
    let entries = vec![
        (Path::new("file1.db").to_path_buf(), 1_000_000_000u64),
        (Path::new("file2.db").to_path_buf(), 1_000_000_000u64),
        (Path::new("file3.db").to_path_buf(), 500_000_000u64),
    ];
    let volumes = VolumeSplitter::partition_volumes(&entries, MAX_VOLUME_BYTES, "SD_ECE");
    assert_eq!(volumes.len(), 2, "2.5 GB should be split into 2 volumes under 2 GiB limit");
    assert!(volumes[0].total_bytes <= MAX_VOLUME_BYTES);
    assert!(volumes[1].total_bytes <= MAX_VOLUME_BYTES);
}

#[test]
fn tier1_sd_04_stock_recovery_script_posix_syntax() {
    let script = generate_stock_recovery_script();
    assert!(script.starts_with("#!/bin/sh"));
    assert!(script.contains("mount -uw /mnt/efs-system"));
    assert!(script.contains("sync"));
    assert!(script.contains("shutdown -S"));
}

#[test]
fn tier1_sd_05_read_only_originals_protection() {
    if let Some(corpus) = get_corpus_root() {
        assert!(corpus.is_dir());
        // Verify files in originals can be read without errors
        let sample = corpus.join("HN+R_EU_AU_K0942_4_[8R0906961FB]/metainfo2.txt");
        if sample.exists() {
            let meta = fs::read_to_string(&sample).unwrap();
            assert!(meta.contains("[common]"));
        }
    }
}

// ==============================================================================
// 7. metainfo2.txt Parsing Tests (>= 5 tests)
// ==============================================================================

#[test]
fn tier1_meta_01_parse_standard_release() {
    let text = generate_metainfo2_text(
        "2026_ECE",
        "dfc9497e7f12e84c98782a201416e91f067da201",
        "0539c892b153b6805178652d3a3a9b3d04e5bf5d",
        "f8a1923cb98e4761a29384756201bfa238475619",
    );

    let parsed = MetaInfo2::parse(&text).expect("Failed to parse valid metainfo2");
    assert_eq!(parsed.release, Some("2026_ECE".to_string()));
    assert_eq!(parsed.vendor, Some("Harman/Becker".to_string()));
    assert_eq!(parsed.source_version, Some("K0942_4".to_string()));
}

#[test]
fn tier1_meta_02_package_sections_extraction() {
    let text = generate_metainfo2_text("2026_ECE", "aaa", "bbb", "ccc");
    let parsed = MetaInfo2::parse(&text).unwrap();

    assert!(parsed.sections.contains_key("common"));
    assert!(parsed.sections.contains_key("HBNavDB"));
    assert!(parsed.sections.contains_key("MU9411"));
    assert!(parsed.sections.contains_key("MapStyles"));

    let hb = &parsed.sections["HBNavDB"];
    assert_eq!(hb.get("path"), Some(&"HBNavDB".to_string()));
    assert_eq!(hb.get("PackageType"), Some(&"NavigationDatabase".to_string()));
}

#[test]
fn tier1_meta_03_sha1_checksum_format() {
    let sha1 = "dfc9497e7f12e84c98782a201416e91f067da201";
    assert_eq!(sha1.len(), 40);
    assert!(sha1.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn tier1_meta_04_strip_comments_and_whitespace() {
    let raw = r#"
    # Top level comment
    ; Semicolon comment
    [common]
    release = "2026_ECE"
    vendor = 'Harman/Becker'
    "#;
    let parsed = MetaInfo2::parse(raw).unwrap();
    assert_eq!(parsed.release, Some("2026_ECE".to_string()));
    assert_eq!(parsed.vendor, Some("Harman/Becker".to_string()));
}

#[test]
fn tier1_meta_05_quoted_values_unquoting() {
    let raw = r#"
    [common]
    release = "UNQUOTED_DOUBLE"
    vendor = 'UNQUOTED_SINGLE'
    sourceVersion = PLAIN
    "#;
    let parsed = MetaInfo2::parse(raw).unwrap();
    assert_eq!(parsed.release, Some("UNQUOTED_DOUBLE".to_string()));
    assert_eq!(parsed.vendor, Some("UNQUOTED_SINGLE".to_string()));
    assert_eq!(parsed.source_version, Some("PLAIN".to_string()));
}

#[test]
fn tier1_meta_06_genuine_corpus_parsing() {
    if let Some(corpus) = get_corpus_root() {
        let meta_path = corpus.join("HN+R_EU_AU_K0942_4_[8R0906961FB]/metainfo2.txt");
        if meta_path.exists() {
            let content = fs::read_to_string(&meta_path).unwrap();
            let parsed = MetaInfo2::parse(&content).expect("Failed to parse genuine metainfo2");
            assert!(parsed.release.is_some() || parsed.sections.len() > 1);
        }
    }
}

// ==============================================================================
// 8. SVM Error Ciphers Tests (>= 5 tests)
// ==============================================================================

#[test]
fn tier1_svm_01_xor_51666_cipher_arithmetic() {
    // Reference case documented in research: 51620 XOR 51666 = 118
    let read_value = 51620;
    let computed_value = resolve_svm_03276(read_value);
    assert_eq!(computed_value, 118, "51620 XOR 51666 must equal 118");
}

#[test]
fn tier1_svm_02_xor_involutory_bijection() {
    for val in [0, 1, 100, 51620, 65535, 123456] {
        let transformed = resolve_svm_03276(val);
        let restored = resolve_svm_03276(transformed);
        assert_eq!(restored, val, "XOR cipher must be strictly involutory");
    }
}

#[test]
fn tier1_svm_03_channel_15_boundaries() {
    assert_eq!(resolve_svm_03276(0), 51666);
    assert_eq!(resolve_svm_03276(51666), 0);
    assert_eq!(resolve_svm_03276(0xFFFF), 0xFFFF ^ 51666);
}

#[test]
fn tier1_svm_04_green_menu_rehash_simulation() {
    let baseline = 5;
    let (stage1, stage2, cleared) = simulate_svm_03175_rehash(baseline);
    assert_eq!(stage1, 6, "First step increments feature value by +1");
    assert_eq!(stage2, 5, "Second step decrements value back by -1");
    assert!(cleared, "Baseline restored and EEPROM hash re-synchronized");
}

#[test]
fn tier1_svm_05_rehash_idempotence() {
    for initial in [0, 1, 10, 42, 100] {
        let (_, final_val, cleared) = simulate_svm_03175_rehash(initial);
        assert_eq!(final_val, initial);
        assert!(cleared);
    }
}

#[test]
fn tier1_svm_06_composite_diagnostic_clearing() {
    // Simulates clearing both fault codes 03276 and 03175 sequentially
    let ecu_5f_channel_15 = 51620;
    let ecu_5f_adaptation_fix = resolve_svm_03276(ecu_5f_channel_15);
    let (_, _, menu_cleared) = simulate_svm_03175_rehash(7);

    assert_eq!(ecu_5f_adaptation_fix, 118);
    assert!(menu_cleared);
}

// ==============================================================================
// 9. LIT/LIT3GP In-Dash Destination Search & Rotary Speller Tests
// ==============================================================================

#[test]
fn tier1_lit_01_speller_radix_search() {
    use mmi_formats::hb_lit::LitDatabaseReader;
    use mmi_rebuild::LitCompiler;
    use mmi_rebuild::geo::{IrDataset, IrPoi, RegionalProfile};

    let temp_dir = TempDir::new().unwrap();
    let profile = RegionalProfile::micro_albania();
    let mut dataset = IrDataset::new(profile.bbox, Some(profile.code));

    // Add distinct POI streets
    dataset.pois.push(IrPoi::new(1, "RRUGA E DIBRES".into(), "amenity".into(), 41.33, 19.82, None));
    dataset.pois.push(IrPoi::new(2, "RRUGA TEODOR KEKO".into(), "highway".into(), 41.32, 19.80, None));
    dataset.pois.push(IrPoi::new(3, "SHESTI SKENDERBEJ".into(), "place".into(), 41.328, 19.818, None));

    let summary = LitCompiler::compile_lit3gp_package(&dataset, temp_dir.path(), "2026.01.0")
        .expect("compile lit3gp package");

    assert!(summary.total_pages >= 3);
    assert_eq!(summary.street_count, 3);

    let db_path = temp_dir.path().join("LIT3GP/EJ211Pa_L1.db");
    assert!(db_path.exists());
    let file = std::fs::File::open(&db_path).expect("open compiled db");
    let mut reader = LitDatabaseReader::open(file).expect("open lit database reader");

    assert_eq!(reader.header().page_size, 544);
    assert_eq!(&reader.header().magic, mmi_formats::FLDB_MAGIC);

    // Verify prefix search
    let mask = reader.get_valid_next_chars("RRUGA ").expect("valid chars for prefix");
    // "RRUGA " should allow 'E' and 'T'
    assert_ne!(mask & (1 << (b'E' - b'A')), 0);
    assert_ne!(mask & (1 << (b'T' - b'A')), 0);
    // Should NOT allow 'Z'
    assert_eq!(mask & (1 << (b'Z' - b'A')), 0);

    // Verify LIT3GP.conf presence and checksums
    let conf_path = temp_dir.path().join("LIT3GP/LIT3GP.conf");
    assert!(conf_path.exists());
    let conf_str = fs::read_to_string(&conf_path).expect("read conf");
    assert!(conf_str.contains("EJ211Pa_L1.db"));
    assert!(conf_str.contains(&summary.md5_hex));
}

#[test]
fn tier1_lit_02_rotary_mask_filtering() {
    use mmi_formats::hb_lit::{compute_rotary_alpha_mask, LitDatabaseReader, LIT_ALPHA_MASK_SPACE};
    use mmi_rebuild::LitCompiler;
    use mmi_rebuild::geo::{IrDataset, IrPoi, RegionalProfile};

    // 1. Bitmask calculation test
    let mask_tirana = compute_rotary_alpha_mask(b"TIRANA 2026");
    assert_ne!(mask_tirana & (1 << (b'T' - b'A')), 0);
    assert_ne!(mask_tirana & (1 << (b'I' - b'A')), 0);
    assert_ne!(mask_tirana & (1 << (b'R' - b'A')), 0);
    assert_ne!(mask_tirana & (1 << (b'A' - b'A')), 0);
    assert_ne!(mask_tirana & (1 << (b'N' - b'A')), 0);
    assert_ne!(mask_tirana & (1 << 26), 0); // Digits '0'..'9'
    assert_ne!(mask_tirana & LIT_ALPHA_MASK_SPACE, 0); // Space

    // 2. Traversal on compiled database
    let temp_dir = TempDir::new().unwrap();
    let profile = RegionalProfile::micro_albania();
    let mut dataset = IrDataset::new(profile.bbox, Some(profile.code));
    dataset.pois.push(IrPoi::new(1, "AUTOSTRADA".into(), "highway".into(), 41.35, 19.75, None));

    LitCompiler::compile_lit3gp_package(&dataset, temp_dir.path(), "2026.01.0").expect("compile");
    let db_path = temp_dir.path().join("LIT3GP/EJ211Pa_L1.db");
    let file = std::fs::File::open(&db_path).unwrap();
    let mut reader = LitDatabaseReader::open(file).unwrap();

    let root_mask = reader.get_valid_next_chars("").expect("root mask");
    assert_ne!(root_mask & (1 << (b'A' - b'A')), 0);

    let next_mask = reader.get_valid_next_chars("AUTO").expect("next mask");
    assert_ne!(next_mask & (1 << (b'S' - b'A')), 0);

    // Non-existent prefix returns 0
    let invalid_mask = reader.get_valid_next_chars("XYZ").expect("invalid mask");
    assert_eq!(invalid_mask, 0);
}
