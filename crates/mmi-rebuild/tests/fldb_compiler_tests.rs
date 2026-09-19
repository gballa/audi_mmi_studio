//! Tests for fldb_compiler.rs: Native Harman/Becker FLDB compiler,
//! spatial indexer, multi-volume partitioner, and SD card deployment media builder.

use mmi_rebuild::geo::{
    BoundingBox, IrDataset, IrEdge, IrNode, IrPoi, RegionalProfile,
};
use mmi_rebuild::{
    compile_fldb_database, create_fldb_page, resolve_svm_03276, simulate_svm_03175_rehash,
    split_into_volumes, FldbCompilerPipeline, FLDB_MAGIC, FLDB_PAGE_SIZE,
    FLDB_TRAILER_SYNC, SVM_CHANNEL_15_XOR_CIPHER,
};
use tempfile::tempdir;

#[test]
fn test_create_fldb_page_structure() {
    let payload = b"Hello Audi MMI 3G+ Navigation System FLDB";
    let page = create_fldb_page(42, payload, FLDB_MAGIC);

    assert_eq!(page.len(), FLDB_PAGE_SIZE); // Exactly 544 bytes
    assert_eq!(&page[0..4], b"FLDB"); // Magic
    assert_eq!(u32::from_le_bytes(page[4..8].try_into().unwrap()), 42); // Page index

    // Trailer sync guard at byte 528..532
    let trailer = u32::from_le_bytes(page[528..532].try_into().unwrap());
    assert_eq!(trailer, FLDB_TRAILER_SYNC);

    // Payload verified at 16..
    assert_eq!(&page[16..16 + payload.len()], payload);

    // CRC-16 non-zero
    let crc = u16::from_le_bytes(page[8..10].try_into().unwrap());
    assert_ne!(crc, 0);
}

#[test]
fn test_compile_fldb_database_pages() {
    let bbox = BoundingBox::new(41.0, 19.5, 41.5, 20.0);
    let mut dataset = IrDataset::new(bbox, Some("AL".to_string()));

    // Add nodes
    dataset.nodes.push(IrNode::from_wgs84(1, 41.3275, 19.8187, 110, 0));
    dataset.nodes.push(IrNode::from_wgs84(2, 41.3280, 19.8200, 112, 0));

    // Add edge
    dataset.edges.push(IrEdge {
        edge_id: 100,
        from_node: 1,
        to_node: 2,
        length_dm: 1500,
        frc: 1,
        speed_forward: 50,
        speed_reverse: 50,
        lane_count: 2,
        turn_lane_mask: 0,
        geometry: vec![(1, 1), (2, 2)],
        access_flags: 0,
    });

    let binary = compile_fldb_database(&dataset);

    // Binary must be exact multiple of 544 bytes
    assert_eq!(binary.len() % FLDB_PAGE_SIZE, 0);
    let page_count = binary.len() / FLDB_PAGE_SIZE;
    assert!(page_count >= 3); // Page 0 (header), Page 1 (dir), Page 2 (nodes/edges)

    // Verify Page 0 header
    assert_eq!(&binary[0..4], &(FLDB_PAGE_SIZE as u32).to_le_bytes());
    assert_eq!(&binary[20..24], b"FLDB");
    assert_eq!(
        u32::from_le_bytes(binary[528..532].try_into().unwrap()),
        FLDB_TRAILER_SYNC
    );
}

#[test]
fn test_split_into_volumes_small() {
    let data = vec![0xAA; 1024];
    let volumes = split_into_volumes(&data, "nav_data.db");
    assert_eq!(volumes.len(), 1);
    assert_eq!(volumes[0].0, "nav_data.db");
    assert_eq!(volumes[0].1.len(), 1024);
}

#[test]
fn test_resolve_svm_03276() {
    // In VCDS, Adaptation Channel 15 value is XORed with 51666 (0xC9D2)
    let original = 12345u32;
    let new_val = resolve_svm_03276(original);
    assert_eq!(new_val, original ^ SVM_CHANNEL_15_XOR_CIPHER);
    // XOR is self-inverting
    assert_eq!(resolve_svm_03276(new_val), original);
}

#[test]
fn test_simulate_svm_03175_rehash() {
    let (s1, s2, ok) = simulate_svm_03175_rehash(10);
    assert_eq!(s1, 11);
    assert_eq!(s2, 10);
    assert!(ok);
}

#[test]
fn test_fldb_compiler_pipeline_compile_and_package() {
    let tmp = tempdir().unwrap();
    let out_dir = tmp.path().join("sd_card_out");

    let profile = RegionalProfile::micro_albania();
    let mut dataset = IrDataset::new(profile.bbox, Some(profile.code));
    dataset.nodes.push(IrNode::from_wgs84(1, 41.3275, 19.8187, 110, 0));
    dataset.edges.push(IrEdge {
        edge_id: 1,
        from_node: 1,
        to_node: 1,
        length_dm: 500,
        frc: 2,
        speed_forward: 60,
        speed_reverse: 60,
        lane_count: 2,
        turn_lane_mask: 0,
        geometry: vec![],
        access_flags: 0,
    });
    dataset.pois.push(IrPoi::new(
        1,
        "Tirana Central Station".to_string(),
        "transit_station".to_string(),
        41.33,
        19.82,
        Some("Tirana Transport".to_string()),
    ));

    let result = FldbCompilerPipeline::compile_and_package(&dataset, &out_dir, Some("2026_ECE"))
        .expect("Compilation failed");

    assert_eq!(result.release, "2026_ECE");
    assert_eq!(result.status, "BUILD READY — DEPLOYMENT NOT VERIFIED");
    assert!(result.total_pages >= 3);
    assert!(result.total_bytes > 0);

    // Verify disk files
    assert!(out_dir.join("metainfo2.txt").exists());
    assert!(out_dir.join("README_SD_CARD.txt").exists());
    assert!(out_dir.join("stock_recovery.sh").exists());
    assert!(out_dir.join("build_manifest.json").exists());
    assert!(out_dir.join("HBNavDB").join("nav_data.db").exists());
    assert!(out_dir.join("HBNavDB").join("2026_ece_patch.pkg").exists());
    assert!(out_dir.join("MU9411").join("strings").join("sq_AL.ans").exists());
    assert!(out_dir.join("MapStyles").join("styles_day.xar").exists());

    let metainfo = std::fs::read_to_string(out_dir.join("metainfo2.txt")).unwrap();
    assert!(metainfo.contains("[HBNavDB]"));
    assert!(metainfo.contains("[MU9411]"));
    assert!(metainfo.contains("HN+R_EU_AU_K0942_4"));
}
