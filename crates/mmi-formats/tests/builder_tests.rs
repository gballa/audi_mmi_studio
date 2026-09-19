//! Unit tests for QnxIfsBuilder, QnxEfsBuilder, and MetaInfo2Builder (§5, RQ-005, RQ-006).

use mmi_formats::{
    MetaInfo2, MetaInfo2Builder, QnxEfs, QnxEfsBuilder, QnxIfs,
    QnxIfsBuilder, MAX_EFS_SYSTEM_SIZE, MAX_IFS_ROOT_SIZE, QNX_F3S_MAGIC, QNX_IFS_MAGIC,
};

#[test]
fn test_qnx_ifs_builder_roundtrip() {
    let mut builder = QnxIfsBuilder::default();
    builder.add_file("/usr/config/ci/splash.png", b"PNG_SPLASH_SCREEN_2026_AUDI_MMI");
    builder.add_file("/usr/bin/lsd.jxe", b"JAVA_HMI_BYTECODE_MODIFIED_ALBANIAN");

    let binary = builder.build().expect("IFS build failed");
    assert_eq!(&binary[0..4], &QNX_IFS_MAGIC);
    assert!(binary.len() <= MAX_IFS_ROOT_SIZE);

    let parsed = QnxIfs::parse(&binary).expect("IFS parse failed");
    assert_eq!(parsed.header.magic, QNX_IFS_MAGIC);
    assert_eq!(parsed.total_size, binary.len());
}

#[test]
fn test_qnx_efs_builder_roundtrip() {
    let mut builder = QnxEfsBuilder::new("/mnt/efs-system");
    builder.add_file("strings/sq_AL.ans", b"HARMAN_ANS_ALBANIAN_STRINGS_CATALOG");
    builder.add_file("engdefs/menu_2026.esd", b"GEM_ESD_CUSTOM_DIAGNOSTIC_SCREEN");

    let binary = builder.build().expect("EFS build failed");
    assert_eq!(&binary[0x2C..0x34], QNX_F3S_MAGIC);
    assert!(binary.len() <= MAX_EFS_SYSTEM_SIZE);

    let parsed = QnxEfs::parse(&binary).expect("EFS parse failed");
    assert_eq!(&parsed.header.magic, QNX_F3S_MAGIC);
    assert_eq!(parsed.header.mount_point, "/mnt/efs-system");
}

#[test]
fn test_metainfo2_builder_with_block_crcs() {
    let mut builder = MetaInfo2Builder::new("2026_ECE", "HN+R_EU_AU_K0942_4");

    let fake_ifs = vec![0xAB; 1_200_000]; // > 2 blocks of 512 KiB
    builder.add_binary_with_blocks("MU9411_ifs_root", "MU9411/ifs-root.ifs", &fake_ifs);

    let manifest = builder.build();
    assert!(manifest.contains("[common]"));
    assert!(manifest.contains("release = \"2026_ECE\""));
    assert!(manifest.contains("HN+R_EU_AU_K0942_4"));
    assert!(manifest.contains("[MU9411_ifs_root]"));
    assert!(manifest.contains("CheckSum.1"));
    assert!(manifest.contains("CheckSum.2"));
    assert!(manifest.contains("CheckSum.3"));

    // Verify MetaInfo2 can parse it back
    let parsed = MetaInfo2::parse(&manifest).expect("Manifest parse failed");
    assert_eq!(parsed.release.as_deref(), Some("2026_ECE"));
    let section = parsed.sections.get("MU9411_ifs_root").expect("Section missing");
    assert_eq!(section.get("path").map(|s| s.as_str()), Some("MU9411/ifs-root.ifs"));
}
