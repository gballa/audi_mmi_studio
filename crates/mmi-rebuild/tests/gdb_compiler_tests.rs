use std::fs;
use tempfile::tempdir;
use mmi_rebuild::geo::{BoundingBox, IrDataset, IrEdge, IrNode, IrTurnRestriction};
use mmi_rebuild::gdb_compiler::GdbCompiler;
use mmi_formats::hb_gdb::{HbGdb, HbGdbReader, GDB_PAGE_SIZE, GDB_VERSION_37, GDB_MAGIC};

#[test]
fn test_gdb_compiler_roundtrip() {
    let temp_dir = tempdir().expect("create temp dir");
    let bbox = BoundingBox::new(41.0, 19.0, 42.0, 20.0);
    let mut dataset = IrDataset::new(bbox, Some("AL".to_string()));

    // Add nodes
    dataset.nodes.push(IrNode {
        node_id: 1,
        x_coord: 1000,
        y_coord: 2000,
        elevation_m: 50,
        junction_flags: 0,
        edge_count: 1,
    });
    dataset.nodes.push(IrNode {
        node_id: 2,
        x_coord: 1050,
        y_coord: 2050,
        elevation_m: 52,
        junction_flags: 1,
        edge_count: 2,
    });
    dataset.nodes.push(IrNode {
        node_id: 3,
        x_coord: 1100,
        y_coord: 2100,
        elevation_m: 55,
        junction_flags: 0,
        edge_count: 1,
    });

    // Add edges across multiple FRC layers
    let edge1 = IrEdge {
        edge_id: 101,
        from_node: 1,
        to_node: 2,
        length_dm: 500,
        frc: 0, // FRC 0 (Motorway)
        speed_forward: 130,
        speed_reverse: 0,
        lane_count: 3,
        turn_lane_mask: 0,
        geometry: vec![(1000, 2000), (1050, 2050)],
        access_flags: 0,
    };
    let edge2 = IrEdge {
        edge_id: 102,
        from_node: 2,
        to_node: 3,
        length_dm: 300,
        frc: 2, // FRC 2 (Secondary)
        speed_forward: 90,
        speed_reverse: 0,
        lane_count: 2,
        turn_lane_mask: 0,
        geometry: vec![(1050, 2050), (1100, 2100)],
        access_flags: 0,
    };
    dataset.edges.push(edge1);
    dataset.edges.push(edge2);

    // Add turn restriction
    dataset.restrictions.push(IrTurnRestriction {
        from_edge: 101,
        via_node: 2,
        to_edge: 102,
        restriction_type: 1, // No Left Turn
        penalty_s: 15,
    });

    let summary = GdbCompiler::compile_gdb_package(
        &dataset,
        temp_dir.path(),
        "routing_graph.gdb",
    ).expect("compile gdb package failed");

    assert!(summary.total_pages >= 3);
    assert_eq!(summary.node_count, 3);
    assert_eq!(summary.edge_count, 2);
    assert_eq!(summary.restriction_count, 1);
    assert_eq!(summary.frc_layer_counts[0], 1);
    assert_eq!(summary.frc_layer_counts[2], 1);

    // Read and parse binary with format parser
    let data = fs::read(&summary.primary_file).expect("read gdb file");
    assert_eq!(data.len() % GDB_PAGE_SIZE, 0);

    let parsed = HbGdb::parse(&data).expect("parse gdb file failed");
    assert_eq!(&parsed.header.magic, GDB_MAGIC);
    assert_eq!(parsed.header.version, GDB_VERSION_37);
    assert_eq!(parsed.header.node_count, 3);
    assert_eq!(parsed.header.edge_count, 2);
    assert_eq!(parsed.header.restriction_count, 1);

    // Verify through HbGdbReader
    let reader = HbGdbReader::from_bytes(&data).expect("open via HbGdbReader");
    reader.validate_all_pages().expect("validate all pages");

    let nodes = reader.read_nodes().expect("read nodes");
    assert_eq!(nodes.len(), 3);

    let frc0_edges = reader.read_edges_frc(0).expect("read frc0 edges");
    assert_eq!(frc0_edges.len(), 1);
    assert_eq!(frc0_edges[0].edge_id, 101);

    let restrs = reader.read_restrictions().expect("read restrs");
    assert_eq!(restrs.len(), 1);
    assert_eq!(restrs[0].penalty_s, 15);

    let tiles = reader.read_spatial_tiles().expect("read tiles");
    assert_eq!(tiles.len(), 1);
}

#[test]
fn test_gdb_compiler_morton_spatial_clustering() {
    let temp_dir = tempdir().expect("create temp dir");
    let bbox = BoundingBox::new(40.0, 18.0, 43.0, 21.0);
    let mut dataset = IrDataset::new(bbox, Some("AL".to_string()));

    // Create 70 nodes spread across distinct coordinates to test multi-tile clustering
    for i in 1..=70 {
        dataset.nodes.push(IrNode {
            node_id: i,
            x_coord: 1000 + (i as i32 * 50),
            y_coord: 2000 + (i as i32 * 50),
            elevation_m: 50,
            junction_flags: 0,
            edge_count: 1,
        });
    }

    // Connect pairs with edges
    for i in 1..70 {
        dataset.edges.push(IrEdge {
            edge_id: i,
            from_node: i,
            to_node: i + 1,
            length_dm: 250,
            frc: 3,
            speed_forward: 80,
            speed_reverse: 80,
            lane_count: 2,
            turn_lane_mask: 0,
            geometry: vec![],
            access_flags: 0,
        });
    }

    let summary = GdbCompiler::compile_gdb_package(
        &dataset,
        temp_dir.path(),
        "clustered.gdb",
    ).expect("compile clustered gdb");

    // 70 nodes with chunk size 32 -> 3 spatial tiles
    assert_eq!(summary.morton_tile_count, 3);

    let reader = HbGdbReader::open(&summary.primary_file).expect("open reader");
    reader.validate_all_pages().expect("validate all pages");

    let tiles = reader.read_spatial_tiles().expect("read tiles");
    assert_eq!(tiles.len(), 3);
    assert_eq!(tiles[0].tile_id, 0);
    assert_eq!(tiles[0].node_count, 32);
    assert_eq!(tiles[1].tile_id, 1);
    assert_eq!(tiles[1].node_count, 32);
    assert_eq!(tiles[2].tile_id, 2);
    assert_eq!(tiles[2].node_count, 6);

    // Verify tile bounding boxes expand properly
    assert!(tiles[0].min_lat <= tiles[0].max_lat);
    assert!(tiles[0].min_lon <= tiles[0].max_lon);
}

#[test]
fn test_gdb_compiler_statutory_speed_fallback() {
    let temp_dir = tempdir().expect("create temp dir");
    let bbox = BoundingBox::new(48.0, 11.0, 49.0, 12.0);
    let mut dataset = IrDataset::new(bbox, Some("DE".to_string())); // Germany

    dataset.nodes.push(IrNode {
        node_id: 1,
        x_coord: 1000,
        y_coord: 2000,
        elevation_m: 50,
        junction_flags: 0,
        edge_count: 1,
    });
    dataset.nodes.push(IrNode {
        node_id: 2,
        x_coord: 1100,
        y_coord: 2100,
        elevation_m: 50,
        junction_flags: 0,
        edge_count: 1,
    });

    // Add FRC 0 (Autobahn) with speed = 0 -> should fall back to 250 km/h
    dataset.edges.push(IrEdge {
        edge_id: 501,
        from_node: 1,
        to_node: 2,
        length_dm: 1000,
        frc: 0,
        speed_forward: 0,
        speed_reverse: 0,
        lane_count: 3,
        turn_lane_mask: 0,
        geometry: vec![],
        access_flags: 0,
    });

    // Add FRC 6 (Urban residential) with speed = 0 -> should fall back to 50 km/h
    dataset.edges.push(IrEdge {
        edge_id: 502,
        from_node: 2,
        to_node: 1,
        length_dm: 1000,
        frc: 6,
        speed_forward: 0,
        speed_reverse: 0,
        lane_count: 1,
        turn_lane_mask: 0,
        geometry: vec![],
        access_flags: 0,
    });

    let summary = GdbCompiler::compile_gdb_package(
        &dataset,
        temp_dir.path(),
        "statutory.gdb",
    ).expect("compile statutory gdb");

    let reader = HbGdbReader::open(&summary.primary_file).expect("open reader");
    let frc0 = reader.read_edges_frc(0).expect("read frc0");
    assert_eq!(frc0.len(), 1);
    assert_eq!(frc0[0].speed_forward, 250); // German Autobahn fallback
    assert_eq!(frc0[0].speed_backward, 250);

    let frc6 = reader.read_edges_frc(6).expect("read frc6");
    assert_eq!(frc6.len(), 1);
    assert_eq!(frc6[0].speed_forward, 50); // German urban fallback
    assert_eq!(frc6[0].speed_backward, 50);
}

#[test]
fn test_gdb_compiler_multi_volume_split() {
    let temp_dir = tempdir().expect("create temp dir");
    let bbox = BoundingBox::new(41.0, 19.0, 42.0, 20.0);
    let mut dataset = IrDataset::new(bbox, Some("AL".to_string()));

    // Create 50 nodes and 50 edges
    for i in 1..=50 {
        dataset.nodes.push(IrNode {
            node_id: i,
            x_coord: 1000 + (i as i32 * 10),
            y_coord: 2000 + (i as i32 * 10),
            elevation_m: 50,
            junction_flags: 0,
            edge_count: 1,
        });
    }
    for i in 1..50 {
        dataset.edges.push(IrEdge {
            edge_id: i,
            from_node: i,
            to_node: i + 1,
            length_dm: 100,
            frc: 1,
            speed_forward: 100,
            speed_reverse: 100,
            lane_count: 2,
            turn_lane_mask: 0,
            geometry: vec![],
            access_flags: 0,
        });
    }

    // Force volume split by setting max_volume_bytes to 3 pages (3 * 544 = 1632 bytes)
    let max_vol = 3 * GDB_PAGE_SIZE as u64;
    let summary = GdbCompiler::compile_with_max_volume_size(
        &dataset,
        temp_dir.path(),
        "EJ211_v37a.gdb",
        max_vol,
        "6.36.0",
    ).expect("compile multi-volume gdb");

    assert!(summary.volume_count >= 2, "must split into at least 2 volumes");
    assert!(summary.secondary_file.is_some());

    let vol1_bytes = fs::read(&summary.primary_file).expect("read vol1");
    let vol2_bytes = fs::read(summary.secondary_file.as_ref().unwrap()).expect("read vol2");

    assert_eq!(vol1_bytes.len() % GDB_PAGE_SIZE, 0);
    assert_eq!(vol2_bytes.len() % GDB_PAGE_SIZE, 0);

    // Verify both volumes have valid GDB pages and CRC-16
    let reader1 = HbGdbReader::from_bytes(&vol1_bytes).expect("parse vol1");
    reader1.validate_all_pages().expect("vol1 pages must pass CRC");
    assert_eq!(&reader1.header.magic, GDB_MAGIC);

    let reader2 = HbGdbReader::from_bytes(&vol2_bytes).expect("parse vol2");
    reader2.validate_all_pages().expect("vol2 pages must pass CRC");
    assert_eq!(&reader2.header.magic, GDB_MAGIC);
    assert_eq!(reader2.header.flags, 0x0002); // Continuation flag
}

#[test]
fn test_gdb_compiler_compile_and_package_standard_layout() {
    let temp_dir = tempdir().expect("create temp dir");
    let media_root = temp_dir.path();
    let pkgdb_dir = media_root.join("pkgdb");
    let hbnavdb_dir = media_root.join("HBNavDB");
    fs::create_dir_all(&hbnavdb_dir).expect("create HBNavDB");

    let bbox = BoundingBox::new(41.0, 19.0, 42.0, 20.0);
    let mut dataset = IrDataset::new(bbox, Some("AL".to_string()));
    dataset.nodes.push(IrNode {
        node_id: 1,
        x_coord: 1000,
        y_coord: 2000,
        elevation_m: 50,
        junction_flags: 0,
        edge_count: 1,
    });
    dataset.nodes.push(IrNode {
        node_id: 2,
        x_coord: 1100,
        y_coord: 2100,
        elevation_m: 50,
        junction_flags: 0,
        edge_count: 1,
    });
    dataset.edges.push(IrEdge {
        edge_id: 1,
        from_node: 1,
        to_node: 2,
        length_dm: 500,
        frc: 1,
        speed_forward: 90,
        speed_reverse: 90,
        lane_count: 2,
        turn_lane_mask: 0,
        geometry: vec![],
        access_flags: 0,
    });

    let summary = GdbCompiler::compile_and_package(&dataset, &pkgdb_dir, "2026_ECE")
        .expect("compile_and_package failed");

    assert_eq!(summary.node_count, 2);
    assert_eq!(summary.edge_count, 1);

    // Verify pkgdb/GDB/EJ211_v37a.gdb and GDB.conf
    let gdb_path = pkgdb_dir.join("GDB/EJ211_v37a.gdb");
    let gdb_conf_path = pkgdb_dir.join("GDB/GDB.conf");
    assert!(gdb_path.exists(), "pkgdb/GDB/EJ211_v37a.gdb must exist");
    assert!(gdb_conf_path.exists(), "pkgdb/GDB/GDB.conf must exist");

    let gdb_conf = fs::read_to_string(&gdb_conf_path).expect("read GDB.conf");
    assert!(gdb_conf.contains("name=GDB_ECE"));
    assert!(gdb_conf.contains("name=EJ211_v37a.gdb"));
    assert!(gdb_conf.contains("type=GDB"));
    assert!(gdb_conf.contains("version=2026_ECE"));

    // Verify pkgdb/GDB2/EJ211_v37a.gd2 and GDB2.conf
    let gd2_path = pkgdb_dir.join("GDB2/EJ211_v37a.gd2");
    let gdb2_conf_path = pkgdb_dir.join("GDB2/GDB2.conf");
    assert!(gd2_path.exists(), "pkgdb/GDB2/EJ211_v37a.gd2 must exist");
    assert!(gdb2_conf_path.exists(), "pkgdb/GDB2/GDB2.conf must exist");

    let gdb2_conf = fs::read_to_string(&gdb2_conf_path).expect("read GDB2.conf");
    assert!(gdb2_conf.contains("name=GDB2_ECE"));
    assert!(gdb2_conf.contains("name=EJ211_v37a.gd2"));
    assert!(gdb2_conf.contains("type=GDB"));
    assert!(gdb2_conf.contains("version=2026_ECE"));

    // Verify HBNavDB/EJ211_v37a.gdb
    let hb_gdb_path = hbnavdb_dir.join("EJ211_v37a.gdb");
    assert!(hb_gdb_path.exists(), "HBNavDB/EJ211_v37a.gdb must exist");
    let hb_bytes = fs::read(&hb_gdb_path).expect("read HBNavDB GDB");
    assert_eq!(hb_bytes.len() % GDB_PAGE_SIZE, 0);

    let reader = HbGdbReader::from_bytes(&hb_bytes).expect("parse HBNavDB GDB");
    reader.validate_all_pages().expect("all HBNavDB GDB pages valid");
}
