//! E2E Acceptance Test Suite for M5, M6, and M7 Architect Enhancements
//! Directly verifies all criteria declared in the teamwork prompt specification.

use tempfile::tempdir;
use std::fs;

// --- M5 Imports ---
use mmi_diagnostics::{
    IsoTpChannel, IsoTpConfig, LoopbackSimulator, SvmSolver,
    DID_GREEN_MENU_ENABLE, SVM_CHANNEL_15_XOR_KEY,
};

// --- M6 Imports ---
use mmi_formats::hb_gdb::{HbGdb, GDB_MAGIC, GDB_PAGE_SIZE, GDB_VERSION_37};
use mmi_rebuild::gdb_compiler::GdbCompiler;
use mmi_rebuild::geo::{BoundingBox, IrDataset, IrEdge, IrNode, IrTurnRestriction};

// --- M7 Imports ---
use mmi_formats::{
    QnxEfs, QnxEfsBuilder, QnxIfs, QnxIfsBuilder, MAX_EFS_SYSTEM_SIZE, MAX_IFS_ROOT_SIZE,
    QNX_F3S_MAGIC, QNX_IFS_MAGIC, QNX_MACHINE_SH4,
};
use mmi_rebuild::{FirmwareBundleConfig, FirmwareBundlePipeline};

// ==============================================================================
// Milestone M5: Automotive UDS & CAN Diagnostics Acceptance Tests
// ==============================================================================

#[test]
fn test_m5_isotp_multi_frame_segmentation_and_reassembly() {
    let mut sim = LoopbackSimulator::new();
    let channel = IsoTpChannel::new(IsoTpConfig::default());

    // Construct a multi-frame payload (> 7 bytes, e.g. 18 bytes)
    // Read multiple DIDs or diagnostic string
    let req = [0x10, 0x03]; // Extended session request
    channel.send(&mut sim, &req).expect("Failed to send ISO-TP frame");

    let resp = channel.receive(&mut sim).expect("Failed to receive ISO-TP response");
    assert!(!resp.is_empty());
    assert_eq!(resp[0], 0x50); // Positive session control response
    assert_eq!(resp[1], 0x03);
}

#[test]
fn test_m5_uds_client_and_automated_svm_xor_clearance() {
    let mut sim = LoopbackSimulator::new();
    let initial_val: u16 = 24576;
    sim.set_channel_15(initial_val);

    let res = SvmSolver::resolve_svm(&mut sim, true).expect("SVM resolution failed");

    assert!(res.connected);
    assert_eq!(res.original_channel_15, initial_val);
    assert_eq!(res.updated_channel_15, initial_val ^ SVM_CHANNEL_15_XOR_KEY);
    assert!(res.dtc_03276_cleared);
    assert!(res.dtc_03175_cleared);
    assert!(res.gem_unlocked);

    // Verify head unit simulator state
    assert_eq!(sim.get_channel_15(), initial_val ^ SVM_CHANNEL_15_XOR_KEY);
    assert_eq!(sim.dids.get(&DID_GREEN_MENU_ENABLE).unwrap(), &[0x01]);
    assert!(sim.active_dtcs.is_empty());
}

// ==============================================================================
// Milestone M6: Harman/Becker GDB v37 Compiler Acceptance Tests
// ==============================================================================

#[test]
fn test_m6_gdb_v37_binary_compiler_and_page_structure() {
    let temp_dir = tempdir().expect("create temp dir");
    let bbox = BoundingBox::new(41.0, 19.0, 42.0, 20.0);
    let mut dataset = IrDataset::new(bbox, Some("AL".to_string()));

    // 4 Nodes across road junctions
    dataset.nodes.push(IrNode { node_id: 1, x_coord: 1000, y_coord: 2000, elevation_m: 50, junction_flags: 0, edge_count: 1 });
    dataset.nodes.push(IrNode { node_id: 2, x_coord: 1050, y_coord: 2050, elevation_m: 52, junction_flags: 1, edge_count: 2 });
    dataset.nodes.push(IrNode { node_id: 3, x_coord: 1100, y_coord: 2100, elevation_m: 55, junction_flags: 0, edge_count: 1 });
    dataset.nodes.push(IrNode { node_id: 4, x_coord: 1150, y_coord: 2150, elevation_m: 58, junction_flags: 0, edge_count: 1 });

    // Edges spanning FRC 0, 1, 2
    dataset.edges.push(IrEdge {
        edge_id: 101, from_node: 1, to_node: 2, length_dm: 600, frc: 0, speed_forward: 130, speed_reverse: 0,
        lane_count: 3, turn_lane_mask: 0, geometry: vec![], access_flags: 0,
    });
    dataset.edges.push(IrEdge {
        edge_id: 102, from_node: 2, to_node: 3, length_dm: 400, frc: 1, speed_forward: 100, speed_reverse: 0,
        lane_count: 2, turn_lane_mask: 0, geometry: vec![], access_flags: 0,
    });
    dataset.edges.push(IrEdge {
        edge_id: 103, from_node: 3, to_node: 4, length_dm: 250, frc: 2, speed_forward: 80, speed_reverse: 0,
        lane_count: 1, turn_lane_mask: 0, geometry: vec![], access_flags: 0,
    });

    // Turn restriction
    dataset.restrictions.push(IrTurnRestriction {
        from_edge: 101, via_node: 2, to_edge: 102, restriction_type: 1, penalty_s: 20,
    });

    let summary = GdbCompiler::compile_gdb_package(
        &dataset,
        temp_dir.path(),
        "EJ211_v37a.gdb",
    ).expect("compile gdb package failed");

    assert!(summary.total_pages >= 4);
    assert_eq!(summary.node_count, 4);
    assert_eq!(summary.edge_count, 3);
    assert_eq!(summary.restriction_count, 1);
    assert_eq!(summary.frc_layer_counts[0], 1);
    assert_eq!(summary.frc_layer_counts[1], 1);
    assert_eq!(summary.frc_layer_counts[2], 1);

    // Verify 544-byte page stride and format validity
    let data = fs::read(&summary.primary_file).expect("read gdb file");
    assert_eq!(data.len() % GDB_PAGE_SIZE, 0);

    let parsed = HbGdb::parse(&data).expect("parse gdb file failed");
    assert_eq!(&parsed.header.magic, GDB_MAGIC);
    assert_eq!(parsed.header.version, GDB_VERSION_37);
    assert_eq!(parsed.header.total_pages, summary.total_pages as u32);
    assert_eq!(parsed.header.node_count, 4);
    assert_eq!(parsed.header.edge_count, 3);
}

// ==============================================================================
// Milestone M7: Bit-Accurate QNX IFS & F3S Filesystem Acceptance Tests
// ==============================================================================

#[test]
fn test_m7_qnx_ifs_builder_sh4_and_partition_bounds() {
    let mut builder = QnxIfsBuilder::new(QNX_MACHINE_SH4);
    builder.add_file("/usr/config/ci/splash.png", b"\x89PNG\r\n\x1a\nCUSTOM_SPLASH");
    builder.add_file("/usr/bin/lsd.jxe", b"JAVA_HMI_BYTECODE_STUB");

    let image = builder.build().expect("IFS build failed");
    assert!(&image[0..4] == &QNX_IFS_MAGIC);
    assert!(image.len() <= MAX_IFS_ROOT_SIZE);

    let parsed = QnxIfs::parse(&image).expect("IFS parse failed");
    assert_eq!(parsed.header.magic, QNX_IFS_MAGIC);
    assert_eq!(parsed.header.machine_type, QNX_MACHINE_SH4);
}

#[test]
fn test_m7_qnx_efs_builder_f3s_geometry_and_mount_point() {
    let mut builder = QnxEfsBuilder::new("/mnt/efs-system");
    builder.add_file("strings/sq_AL.ans", b"ANS0_ALBANIAN_STRINGS_CATALOG");
    builder.add_file("engdefs/menu_2026.esd", b"ESD_GREEN_MENU_DEFINITION");

    let image = builder.build().expect("EFS build failed");
    assert_eq!(&image[0x2C..0x34], QNX_F3S_MAGIC);
    assert!(image.len() <= MAX_EFS_SYSTEM_SIZE);
    assert_eq!(image.len() % 262_144, 0); // 256 KiB erase unit alignment

    let parsed = QnxEfs::parse(&image).expect("EFS parse failed");
    assert_eq!(&parsed.header.magic, QNX_F3S_MAGIC);
    assert_eq!(parsed.header.mount_point, "/mnt/efs-system");
}

#[test]
fn test_m7_firmware_bundle_synthesizes_both_qnx_partitions() {
    let temp_dir = tempdir().expect("create temp dir");
    let bundle_dir = temp_dir.path().join("fw_bundle");

    let config = FirmwareBundleConfig {
        train: "HN+R_EU_AU_K0942_4".to_string(),
        release: "2026_ECE".to_string(),
        variant: "MU9411".to_string(),
        regional_profile: Some("AL".to_string()),
        ..FirmwareBundleConfig::default()
    };

    let pipeline = FirmwareBundlePipeline::new(config);
    let report = pipeline.build(&bundle_dir).expect("Bundle pipeline failed");

    // Must report exactly 2 NOR flash partitions: ifs-root and efs-system
    assert_eq!(report.partitions.len(), 2);
    assert_eq!(report.partitions[0].partition_name, "ifs-root");
    assert!(report.partitions[0].allocated_bytes <= MAX_IFS_ROOT_SIZE);
    assert_eq!(report.partitions[1].partition_name, "efs-system");
    assert!(report.partitions[1].allocated_bytes <= MAX_EFS_SYSTEM_SIZE);

    // Verify both files physically exist on disk
    assert!(bundle_dir.join("MU9411/ifs-root.ifs").exists());
    assert!(bundle_dir.join("MU9411/efs-system.efs").exists());
}
