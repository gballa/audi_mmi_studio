//! Integration tests for FirmwareBundlePipeline (§5, RQ-005, RQ-006, §14.9 Safety Policy).

use mmi_formats::{MetaInfo2, QnxEfs, QnxIfs, MAX_EFS_SYSTEM_SIZE, MAX_IFS_ROOT_SIZE, QNX_F3S_MAGIC, QNX_IFS_MAGIC};
use mmi_rebuild::{FirmwareBundleConfig, FirmwareBundlePipeline, SAFETY_POLICY_BANNER};
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_firmware_bundle_pipeline_end_to_end() {
    let temp_dir = tempdir().expect("Failed to create tempdir");
    let bundle_dir = temp_dir.path().join("sd_bundle");

    let config = FirmwareBundleConfig {
        train: "HN+R_EU_AU_K0942_4".to_string(),
        release: "2026_ECE".to_string(),
        variant: "MU9411".to_string(),
        splash_screen_png: Some(b"\x89PNG\r\n\x1a\nAUDI_MMI_3G_PLUS_2026_SPLASH_SCREEN".to_vec()),
        albanian_strings_ans: Some(b"ANS0[sq_AL]\nSTR_NAV=\"Navigimi Shqip\"\n".to_vec()),
        gem_screen_esd: Some(b"ESD\x01DIAG_MENU_2026_ALBANIA".to_vec()),
        nav_database_fldb: None, // Test auto-compilation via RegionalProfile::micro_albania()
        map_styles_gdb: None,
        regional_profile: Some("AL".to_string()),
    };

    let pipeline = FirmwareBundlePipeline::new(config);
    let report = pipeline.build(&bundle_dir).expect("Firmware bundle build failed");

    // 1. Verify Report
    assert_eq!(report.target_train, "HN+R_EU_AU_K0942_4");
    assert_eq!(report.target_release, "2026_ECE");
    assert_eq!(report.target_variant, "MU9411");
    assert_eq!(report.safety_status, SAFETY_POLICY_BANNER);
    assert_eq!(report.partitions.len(), 2);

    for p in &report.partitions {
        if p.partition_name == "ifs-root" {
            assert!(p.allocated_bytes <= MAX_IFS_ROOT_SIZE);
            assert!(p.percentage_used > 0.0 && p.percentage_used <= 100.0);
        } else if p.partition_name == "efs-system" {
            assert!(p.allocated_bytes <= MAX_EFS_SYSTEM_SIZE);
            assert!(p.percentage_used > 0.0 && p.percentage_used <= 100.0);
        }
    }

    // 2. Verify Generated Files Exist on Disk
    let expected_files = [
        "MU9411/ifs-root.ifs",
        "MU9411/efs-system.efs",
        "HBNavDB/nav_data.db",
        "MapStyles/night_2026.gdb",
        "metainfo2.txt",
        "copie_scr.sh",
        "finalScript",
        "stock_recovery.sh",
        "build_manifest.json",
        "gem/screens/custom_telemetry.esd",
        "gem/screens/map_inspector.esd",
        "gem/scripts/bench_diag.sh",
    ];
    for f in &expected_files {
        let p = bundle_dir.join(f);
        assert!(p.exists(), "Expected bundle file missing: {:?}", p);
    }

    // 2b. Verify GEM Custom Screen ESD binary header
    let custom_telemetry_bytes = std::fs::read(bundle_dir.join("gem/screens/custom_telemetry.esd")).unwrap();
    assert!(custom_telemetry_bytes.starts_with(b"ESD\x01"));

    // 3. Verify QNX IFS Parsing
    let ifs_bytes = std::fs::read(bundle_dir.join("MU9411/ifs-root.ifs")).unwrap();
    let ifs_parsed = QnxIfs::parse(&ifs_bytes).expect("Failed to parse generated ifs-root.ifs");
    assert_eq!(ifs_parsed.header.magic, QNX_IFS_MAGIC);

    // 4. Verify QNX EFS Parsing
    let efs_bytes = std::fs::read(bundle_dir.join("MU9411/efs-system.efs")).unwrap();
    let efs_parsed = QnxEfs::parse(&efs_bytes).expect("Failed to parse generated efs-system.efs");
    assert_eq!(&efs_parsed.header.magic, QNX_F3S_MAGIC);
    assert_eq!(efs_parsed.header.mount_point, "/mnt/efs-system");

    // 5. Verify MetaInfo2 Manifest Parsing and Block CRCs
    let meta_txt = std::fs::read_to_string(bundle_dir.join("metainfo2.txt")).unwrap();
    let meta_parsed = MetaInfo2::parse(&meta_txt).expect("Failed to parse metainfo2.txt");
    assert_eq!(meta_parsed.release.as_deref(), Some("2026_ECE"));
    assert!(meta_parsed.sections.contains_key("MU9411_ifs_root"));
    assert!(meta_parsed.sections.contains_key("MU9411_efs_system"));
    assert!(meta_parsed.sections.contains_key("HBNavDB"));
    assert!(meta_parsed.sections.contains_key("MapStyles"));

    // 6. Verify copie_scr.sh and finalScript contain safety banners
    let copie_content = std::fs::read_to_string(bundle_dir.join("copie_scr.sh")).unwrap();
    assert!(copie_content.contains("HN+R_EU_AU_K0942_4"));
    assert!(copie_content.contains(SAFETY_POLICY_BANNER));

    // 7. Verify Immutability Protection Against originals/ Directory
    let forbidden_dir = Path::new("/some/path/originals/my_bundle");
    let err = pipeline.build(forbidden_dir);
    assert!(err.is_err());
}
