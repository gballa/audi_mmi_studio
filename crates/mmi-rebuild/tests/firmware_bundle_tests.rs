//! Integration tests for FirmwareBundlePipeline (§5, RQ-005, RQ-006, §14.9 Safety Policy).

use mmi_formats::{MetaInfo2, Mmi3gScriptCipher};
use mmi_rebuild::{
    FirmwareBundleConfig, FirmwareBundlePipeline, DONE_PNG, RUNNING_PNG, SAFETY_POLICY_BANNER,
    SHOW_SCREEN_BIN,
};
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
        ..FirmwareBundleConfig::default()
    };

    let pipeline = FirmwareBundlePipeline::new(config);
    let report = pipeline.build(&bundle_dir).expect("Firmware bundle build failed");

    // 1. Verify Report
    assert_eq!(report.target_train, "HN+R_EU_AU_K0942_4");
    assert_eq!(report.target_release, "2026_ECE");
    assert_eq!(report.target_variant, "MU9411");
    assert_eq!(report.safety_status, SAFETY_POLICY_BANNER);
    assert_eq!(report.partitions.len(), 0); // No longer flashing partitions!

    // 2. Verify Generated Files Exist on Disk
    let expected_files = [
        "payload/splash.png",
        "payload/sq_AL.ans",
        "payload/menu_2026.esd",
        "HBNavDB/nav_data.db",
        "MapStyles/night_2026.gdb",
        "metainfo2.txt",
        "bin/showScreen",
        "lib/running.png",
        "lib/done.png",
        "run.sh",
        "copie_scr_plain.sh",
        "copie_scr.sh",
        "finalScript",
        "stock_recovery.sh",
        "gem/screens/custom_telemetry.esd",
        "gem/screens/map_inspector.esd",
        "gem/screens/ToolkitDTC.esd",
        "gem/screens/GaugesDashboard.esd",
        "gem/scripts/bench_diag.sh",
        "gem/scripts/sysinfo_dump.sh",
        "gem/scripts/password_dump.sh",
    ];
    for f in &expected_files {
        let p = bundle_dir.join(f);
        assert!(p.exists(), "Expected bundle file missing: {:?}", p);
    }

    // 2b. Verify HUD Assets Bytes
    let show_screen_disk = std::fs::read(bundle_dir.join("bin/showScreen")).unwrap();
    assert_eq!(show_screen_disk, SHOW_SCREEN_BIN);
    let running_png_disk = std::fs::read(bundle_dir.join("lib/running.png")).unwrap();
    assert_eq!(running_png_disk, RUNNING_PNG);
    let done_png_disk = std::fs::read(bundle_dir.join("lib/done.png")).unwrap();
    assert_eq!(done_png_disk, DONE_PNG);

    // 2c. Verify GEM Custom Screen ESD binary header & MMI3G-Toolkit screens
    let custom_telemetry_bytes = std::fs::read(bundle_dir.join("gem/screens/custom_telemetry.esd")).unwrap();
    assert!(custom_telemetry_bytes.starts_with(b"ESD\x01"));
    let dtc_content = std::fs::read_to_string(bundle_dir.join("gem/screens/ToolkitDTC.esd")).unwrap();
    assert!(dtc_content.contains("Active DTCs"));
    assert!(dtc_content.contains("ClearErrmem"));
    let gauges_content = std::fs::read_to_string(bundle_dir.join("gem/screens/GaugesDashboard.esd")).unwrap();
    assert!(gauges_content.contains("Battery (x100 mV)"));
    assert!(gauges_content.contains("GPS Sats Used"));

    // 5. Verify MetaInfo2 Manifest Parsing and Block CRCs
    let meta_txt = std::fs::read_to_string(bundle_dir.join("metainfo2.txt")).unwrap();
    let meta_parsed = MetaInfo2::parse(&meta_txt).expect("Failed to parse metainfo2.txt");
    assert_eq!(meta_parsed.release.as_deref(), Some("2026_ECE"));
    assert!(!meta_parsed.sections.contains_key("MU9411_ifs_root")); // No longer present
    assert!(!meta_parsed.sections.contains_key("MU9411_efs_system")); // No longer present
    assert!(meta_parsed.sections.contains_key("HBNavDB"));
    assert!(meta_parsed.sections.contains_key("MapStyles"));

    // 6. Verify run.sh and copie_scr.sh (Harman PRNG cipher verification)
    let run_content = std::fs::read_to_string(bundle_dir.join("run.sh")).unwrap();
    assert!(run_content.contains("HN+R_EU_AU_K0942_4"));
    assert!(run_content.contains(SAFETY_POLICY_BANNER));
    assert!(run_content.contains("_qnx_mkdir_p"));
    assert!(run_content.contains("pci-3g_"));
    assert!(run_content.contains("disableReclaim"));
    assert!(run_content.contains("acios_db.ini"));
    assert!(run_content.contains("showScreen"));
    assert!(run_content.contains("PAYLOAD_STRINGS_DIR"));

    let plain_content = std::fs::read_to_string(bundle_dir.join("copie_scr_plain.sh")).unwrap();
    assert!(plain_content.contains("exec ksh ./run.sh"));

    // copie_scr.sh is encrypted with seed 0x001be3ac; decrypting it must match copie_scr_plain.sh!
    let encoded_copie_bytes = std::fs::read(bundle_dir.join("copie_scr.sh")).unwrap();
    assert_ne!(encoded_copie_bytes, plain_content.as_bytes());
    let decrypted = Mmi3gScriptCipher::transform(&encoded_copie_bytes);
    assert_eq!(decrypted, plain_content.as_bytes());

    // 7. Verify Immutability Protection Against originals/ Directory
    let forbidden_dir = Path::new("/some/path/originals/my_bundle");
    let err = pipeline.build(forbidden_dir);
    assert!(err.is_err());
}
