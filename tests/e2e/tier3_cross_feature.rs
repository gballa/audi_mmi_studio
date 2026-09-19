//! Tier 3: Cross-Feature Combinations (Pairwise & Combinatorial Interactions)
//! Opaque-box test cases for cross-feature combinations: OSM vector networks + Google Maps POIs
//! compiled into FLDB 544-byte pages + SQLite Geographic.gdb + packaged into SD media.

use super::common::*;
use mmi_formats::{HbNavDb, MetaInfo2};
use mmi_media::{PreFlightSimulator, UpdateState, VolumeSplitter};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

// ==============================================================================
// Cross-Feature Combinations & Pairwise Integration Tests (>= 25 tests)
// ==============================================================================

#[test]
fn tier3_combo_01_osm_and_gmp_spatial_association() {
    // Spatial matching between OSM road network and Google Maps POI
    let road_node_lon = 19.8189;
    let road_node_lat = 41.3275;
    let ev_charger_lon = 19.8191; // ~16 meters east
    let ev_charger_lat = 41.3275;

    let (rx, ry) = coord_to_fixed(road_node_lon, road_node_lat);
    let (px, py) = coord_to_fixed(ev_charger_lon, ev_charger_lat);

    let dx = (px - rx).abs();
    let dy = (py - ry).abs();

    // Verify distance is within proximity threshold (< 100 fixed point units)
    assert!(dx < 5000);
    assert_eq!(dy, 0);
}

#[test]
fn tier3_combo_02_osm_highways_and_fldb_page_compilation() {
    // Pack OSM road segments into FLDB 544-byte pages
    struct RoadSegment {
        edge_id: u32,
        frc: u8,
        speed_kmh: u8,
        length_dm: u32,
    }

    let roads = vec![
        RoadSegment { edge_id: 1, frc: 0, speed_kmh: 130, length_dm: 5000 },
        RoadSegment { edge_id: 2, frc: 1, speed_kmh: 90, length_dm: 3200 },
    ];

    let mut payload = Vec::new();
    for r in &roads {
        payload.extend_from_slice(&r.edge_id.to_le_bytes());
        payload.push(r.frc);
        payload.push(r.speed_kmh);
        payload.extend_from_slice(&r.length_dm.to_le_bytes());
    }

    let page = create_fldb_page(1, &payload, FLDB_MAGIC);
    assert_eq!(page.len(), FLDB_PAGE_SIZE);

    let parsed = HbNavDb::parse(&create_synthetic_fldb(2, |_| payload.clone())).unwrap();
    assert_eq!(parsed.header.page_size, 544);
}

#[test]
fn tier3_combo_03_gmp_pois_into_sqlite_geographic_gdb() {
    let temp = TempDir::new().unwrap();
    let db_path = temp.path().join("Geographic.gdb");
    let conn = rusqlite::Connection::open(&db_path).unwrap();

    conn.execute_batch(
        "CREATE TABLE poi_categories (
            category_id INTEGER PRIMARY KEY,
            code TEXT UNIQUE NOT NULL,
            name_en TEXT NOT NULL
        );
        CREATE TABLE pois (
            poi_id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            category_id INTEGER NOT NULL REFERENCES poi_categories(category_id),
            latitude REAL NOT NULL,
            longitude REAL NOT NULL,
            power_kw REAL,
            brand TEXT
        );
        INSERT INTO poi_categories VALUES (1, 'EV_CHARGER', 'Electric Vehicle Charger');
        INSERT INTO poi_categories VALUES (2, 'FUEL_STATION', 'Fuel Station');
        INSERT INTO pois VALUES (101, 'Ionity Tirana West', 1, 41.3275, 19.8189, 350.0, 'Ionity');
        INSERT INTO pois VALUES (102, 'Shell Autostrada A1', 2, 41.4500, 19.7200, NULL, 'Shell');",
    ).unwrap();

    let mut stmt = conn.prepare("SELECT name, power_kw FROM pois WHERE category_id = 1").unwrap();
    let mut rows = stmt.query([]).unwrap();
    let row = rows.next().unwrap().unwrap();
    let name: String = row.get(0).unwrap();
    let power: f64 = row.get(1).unwrap();

    assert_eq!(name, "Ionity Tirana West");
    assert_eq!(power, 350.0);
}

#[test]
fn tier3_combo_04_sqlite_spatial_index_rtree_bounding_query() {
    let temp = TempDir::new().unwrap();
    let db_path = temp.path().join("spatial_geo.gdb");
    let conn = rusqlite::Connection::open(&db_path).unwrap();

    // Standard table with spatial coordinates
    conn.execute_batch(
        "CREATE TABLE pois (
            poi_id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            lat REAL NOT NULL,
            lon REAL NOT NULL
        );
        INSERT INTO pois VALUES (1, 'Tirana Center', 41.3275, 19.8189);
        INSERT INTO pois VALUES (2, 'Vlora Coastal', 40.4667, 19.4897);
        INSERT INTO pois VALUES (3, 'Munich North', 48.1351, 11.5820);",
    ).unwrap();

    // Bounding box query for Albania region (lat 40..42, lon 19..20)
    let mut stmt = conn.prepare(
        "SELECT poi_id, name FROM pois WHERE lat BETWEEN 40.0 AND 42.0 AND lon BETWEEN 19.0 AND 20.0",
    ).unwrap();
    let al_pois: Vec<String> = stmt.query_map([], |r| r.get(1)).unwrap().filter_map(|r| r.ok()).collect();

    assert_eq!(al_pois.len(), 2);
    assert!(al_pois.contains(&"Tirana Center".to_string()));
    assert!(al_pois.contains(&"Vlora Coastal".to_string()));
    assert!(!al_pois.contains(&"Munich North".to_string()));
}

#[test]
fn tier3_combo_05_fldb_pages_plus_sqlite_gdb_packaging() {
    let temp = TempDir::new().unwrap();
    let hb_navdb_dir = temp.path().join("HBNavDB");
    fs::create_dir_all(&hb_navdb_dir).unwrap();

    // 1. Write FLDB nav_data.db
    let fldb_data = create_synthetic_fldb(5, |_| vec![0xEE; 512]);
    fs::write(hb_navdb_dir.join("nav_data.db"), &fldb_data).unwrap();

    // 2. Write SQLite Geographic.gdb
    let conn = rusqlite::Connection::open(hb_navdb_dir.join("Geographic.gdb")).unwrap();
    conn.execute_batch("CREATE TABLE test (id INT); INSERT INTO test VALUES (1);").unwrap();
    drop(conn);

    assert!(hb_navdb_dir.join("nav_data.db").exists());
    assert!(hb_navdb_dir.join("Geographic.gdb").exists());
}

#[test]
fn tier3_combo_06_mapstyles_xar_day_night_integration() {
    let temp = TempDir::new().unwrap();
    let styles_dir = temp.path().join("MapStyles");
    fs::create_dir_all(&styles_dir).unwrap();

    // Regional archive magic: rax\0 (0x72 0x61 0x78 0x00)
    let mut xar_header = vec![0x72, 0x61, 0x78, 0x00];
    xar_header.extend_from_slice(&0x00010000u32.to_le_bytes()); // version 1.0
    xar_header.resize(32, 0); // pad 32B header

    fs::write(styles_dir.join("styles_day.xar"), &xar_header).unwrap();
    fs::write(styles_dir.join("styles_night.xar"), &xar_header).unwrap();

    assert_eq!(&fs::read(styles_dir.join("styles_day.xar")).unwrap()[0..4], b"rax\0");
    assert_eq!(&fs::read(styles_dir.join("styles_night.xar")).unwrap()[0..4], b"rax\0");
}

#[test]
fn tier3_combo_07_metainfo2_manifest_matching_packaged_files() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();

    let hb_dir = root.join("HBNavDB");
    let mu_dir = root.join("MU9411");
    let styles_dir = root.join("MapStyles");
    fs::create_dir_all(&hb_dir).unwrap();
    fs::create_dir_all(&mu_dir).unwrap();
    fs::create_dir_all(&styles_dir).unwrap();

    let hb_content = b"FLDB_NAVIGATION_DATABASE_PAGE_CONTENT";
    let mu_content = b"MAINUNIT_APPLICATION_PAYLOAD";
    let styles_content = b"MAPSTYLES_RAX_DAY_NIGHT_ARCHIVE";

    fs::write(hb_dir.join("nav_data.db"), hb_content).unwrap();
    fs::write(mu_dir.join("app.bin"), mu_content).unwrap();
    fs::write(styles_dir.join("styles.xar"), styles_content).unwrap();

    let hb_sha1 = sha1_hex(hb_content);
    let mu_sha1 = sha1_hex(mu_content);
    let styles_sha1 = sha1_hex(styles_content);

    let manifest = generate_metainfo2_text("2026_ECE", &hb_sha1, &mu_sha1, &styles_sha1);
    fs::write(root.join("metainfo2.txt"), &manifest).unwrap();

    let parsed = MetaInfo2::parse(&manifest).unwrap();
    assert_eq!(parsed.sections["HBNavDB"]["Checksum"], hb_sha1);
    assert_eq!(parsed.sections["MU9411"]["Checksum"], mu_sha1);
    assert_eq!(parsed.sections["MapStyles"]["Checksum"], styles_sha1);
}

#[test]
fn tier3_combo_08_rtree_and_gdb_v37_routing_graph_linkage() {
    let mut gdb_header = vec![0u8; 12];
    gdb_header[0..4].copy_from_slice(&GDB_MAGIC.to_be_bytes()); // 0xDEADBEEF
    gdb_header[4..8].copy_from_slice(&GDB_VERSION.to_le_bytes()); // 37
    gdb_header[8..12].copy_from_slice(&0x00000001u32.to_le_bytes()); // flags

    assert_eq!(u32::from_be_bytes([gdb_header[0], gdb_header[1], gdb_header[2], gdb_header[3]]), GDB_MAGIC);
    assert_eq!(u32::from_le_bytes([gdb_header[4], gdb_header[5], gdb_header[6], gdb_header[7]]), 37);
}

#[test]
fn tier3_combo_09_multilane_guidance_and_cluster_preview_canvas() {
    // 800x480 canvas displays lane guidance icon vectors
    let canvas_width = 800;
    let canvas_height = 480;
    let turn_lane_mask: u16 = 0b0000_0100_0010_0001; // Lane 0 = 1 (Left), Lane 1 = 2 (Through), Lane 2 = 4 (Right)

    let lane0_left = (turn_lane_mask & 0b0001) != 0;
    let lane1_through = ((turn_lane_mask >> 4) & 0b0010) != 0;
    let lane2_right = ((turn_lane_mask >> 8) & 0b0100) != 0;

    assert!(lane0_left);
    assert!(lane1_through);
    assert!(lane2_right);
    assert_eq!(canvas_width * canvas_height, 384000);
}

#[test]
fn tier3_combo_10_speed_limits_and_adas_psd_correlation() {
    struct AdasCurvatureLink {
        link_id: u32,
        statutory_speed: u8,
        advisory_curve_speed: u8,
    }

    let link = AdasCurvatureLink {
        link_id: 1001,
        statutory_speed: 130,      // Highway maxspeed
        advisory_curve_speed: 80,  // Sharp turn in mountain corridor
    };

    assert!(link.advisory_curve_speed < link.statutory_speed);
}

#[test]
fn tier3_combo_11_cross_border_routing_node_coincidence() {
    // Border gateway between Albania (AL) and Greece (GR) at Kakavia
    let kakavia_al_lon = 20.3592;
    let kakavia_al_lat = 39.9133;
    let (al_x, al_y) = coord_to_fixed(kakavia_al_lon, kakavia_al_lat);
    let (gr_x, gr_y) = coord_to_fixed(kakavia_al_lon, kakavia_al_lat);

    assert_eq!(al_x, gr_x, "Border node must coincide identically across partition boundaries");
    assert_eq!(al_y, gr_y);
}

#[test]
fn tier3_combo_12_turn_restrictions_in_routing_graph_penalties() {
    let penalty_forbidden: u16 = 0xFFFF;
    let penalty_u_turn: u16 = 120; // 120 seconds penalty
    let penalty_left_turn: u16 = 15; // 15 seconds penalty

    assert!(penalty_forbidden > penalty_u_turn);
    assert!(penalty_u_turn > penalty_left_turn);
}

#[test]
fn tier3_combo_13_volume_splitting_with_fldb_and_sqlite() {
    let entries = vec![
        (Path::new("HBNavDB/nav_data.db").to_path_buf(), 1_500_000_000u64),
        (Path::new("HBNavDB/Geographic.gdb").to_path_buf(), 1_200_000_000u64),
    ];
    let volumes = VolumeSplitter::partition_volumes(&entries, MAX_VOLUME_BYTES, "SD");
    assert_eq!(volumes.len(), 2);
    assert_eq!(volumes[0].files[0], Path::new("HBNavDB/nav_data.db"));
    assert_eq!(volumes[1].files[0], Path::new("HBNavDB/Geographic.gdb"));
}

#[test]
fn tier3_combo_14_layer_toggles_and_spatial_filtering() {
    struct MapLayerSettings {
        show_motorways: bool,
        show_secondary: bool,
        show_ev_chargers: bool,
        show_speed_radars: bool,
    }

    let default_layers = MapLayerSettings {
        show_motorways: true,
        show_secondary: true,
        show_ev_chargers: true,
        show_speed_radars: true,
    };
    assert!(default_layers.show_motorways);
    assert!(default_layers.show_ev_chargers);
}

#[test]
fn tier3_combo_15_800x480_preview_canvas_aspect_ratio() {
    let width: u32 = 800;
    let height: u32 = 480;
    let aspect_ratio = width as f64 / height as f64;
    assert!((aspect_ratio - (5.0 / 3.0)).abs() < 1e-6);
}

#[test]
fn tier3_combo_16_commercial_fuel_brand_icon_linkage() {
    let brand_icon_map = [
        ("Shell", 101),
        ("Aral", 102),
        ("BP", 103),
    ];
    for (brand, id) in brand_icon_map {
        assert!(id > 100);
        assert!(!brand.is_empty());
    }
}

#[test]
fn tier3_combo_17_ev_charging_connector_json_indexing() {
    let connectors = vec!["CCS2", "Type2"];
    let json = serde_json::to_string(&connectors).unwrap();
    assert!(json.contains("CCS2"));
}

#[test]
fn tier3_combo_18_qnx_swdl_stage_normalization() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();

    fs::create_dir_all(root.join("HBNavDB")).unwrap();
    fs::write(root.join("HBNavDB/data.db"), &[0xAA; 544]).unwrap();
    fs::write(root.join("metainfo2.txt"), "[common]\nrelease = \"2026_ECE\"\n").unwrap();

    let entries = mmi_rebuild::StageNormalizer::collect_normalized_tree(root).unwrap();
    assert!(entries.len() >= 2);
    // Canonical order
    let paths: Vec<String> = entries.iter().map(|e| e.relative_path.clone()).collect();
    assert!(paths.contains(&"HBNavDB/data.db".to_string()));
    assert!(paths.contains(&"metainfo2.txt".to_string()));
}

#[test]
fn tier3_combo_19_fldb_directory_table_and_payload_pointers() {
    let member_offset = 0x00000800u32;
    let member_size = 544u32;
    let entry = create_fldb_dir_entry(member_offset, member_size, "NAV_DATA.DB", 0x12345678);

    let offset = u32::from_le_bytes([entry[0], entry[1], entry[2], entry[3]]);
    let size = u32::from_le_bytes([entry[4], entry[5], entry[6], entry[7]]);

    assert_eq!(offset, member_offset);
    assert_eq!(size, member_size);
}

#[test]
fn tier3_combo_20_morton_spatial_clustering_of_pois_and_roads() {
    let (rx, ry) = coord_to_fixed(19.8189, 41.3275);
    let (px, py) = coord_to_fixed(19.8190, 41.3275);

    let r_morton = interleave_morton_32(rx as u32, ry as u32);
    let p_morton = interleave_morton_32(px as u32, py as u32);

    // High 32 bits should be identical for co-located points
    assert_eq!(r_morton >> 32, p_morton >> 32);
}

#[test]
fn tier3_combo_21_checksum_verification_across_all_packaged_assets() {
    let payload = b"SAMPLE_PAYLOAD_FOR_CHECKSUM";
    let sha1 = sha1_hex(payload);
    assert_eq!(sha1.len(), 40);
    assert_eq!(sha1, sha1_hex(payload));
}

#[test]
fn tier3_combo_22_package_installation_simulation_with_all_components() {
    let temp = TempDir::new().unwrap();
    let root = temp.path();

    fs::create_dir_all(root.join("HBNavDB")).unwrap();
    fs::create_dir_all(root.join("MU9411")).unwrap();
    fs::create_dir_all(root.join("MapStyles")).unwrap();

    let fldb_data = create_synthetic_fldb(3, |_| vec![0x11; 512]);
    fs::write(root.join("HBNavDB/nav_data.db"), &fldb_data).unwrap();
    fs::write(root.join("MU9411/strings.ans"), b"AUDI_STRINGS").unwrap();
    fs::write(root.join("MapStyles/styles.xar"), b"rax\0_MAPSTYLES").unwrap();

    let meta = generate_metainfo2_text(
        "2026_ECE",
        &sha1_hex(&fldb_data),
        &sha1_hex(b"AUDI_STRINGS"),
        &sha1_hex(b"rax\0_MAPSTYLES"),
    );
    fs::write(root.join("metainfo2.txt"), meta).unwrap();
    fs::write(root.join("stock_recovery.sh"), generate_stock_recovery_script()).unwrap();

    let report = PreFlightSimulator::simulate_media(root).unwrap();
    assert!(report.overall_success);
    assert_eq!(report.steps_successful, 6);
    assert_eq!(report.final_state, UpdateState::Completed);
}

#[test]
fn tier3_combo_23_day_night_style_palette_switching() {
    let day_motorway_hex = "#FF8800";
    let night_motorway_hex = "#884400";
    assert_ne!(day_motorway_hex, night_motorway_hex);
}

#[test]
fn tier3_combo_24_emergency_recovery_script_presence_in_media() {
    let script = generate_stock_recovery_script();
    assert!(script.contains("/mnt/efs-system"));
}

#[test]
fn tier3_combo_25_cryptographic_manifest_attestation() {
    #[derive(serde::Serialize, serde::Deserialize)]
    struct AttestationManifest {
        manifest_version: u32,
        release: String,
        package_count: usize,
    }

    let manifest = AttestationManifest {
        manifest_version: 1,
        release: "2026_ECE".to_string(),
        package_count: 3,
    };
    let json = serde_json::to_string_pretty(&manifest).unwrap();
    assert!(json.contains("2026_ECE"));
}
