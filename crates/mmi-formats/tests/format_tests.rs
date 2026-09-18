use std::path::Path;
use mmi_core::SourceStore;
use mmi_formats::{
    FormatAdapter, MapStyleXar, MapStyleXarAdapter, MetaInfo2, MetaInfo2Adapter, PrecompAdapter,
    PrecompImage,
};

#[test]
fn test_metainfo2_real_corpus_parsing() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let originals_dir = workspace_dir.join("originals");
    let store = SourceStore::new(&originals_dir).expect("Failed to open SourceStore");

    let metainfo_bytes = store
        .read_bytes("HN+R_EU_AU_K0942_4_[8R0906961FB]/metainfo2.txt")
        .expect("Failed to read metainfo2.txt");
    let text = String::from_utf8_lossy(&metainfo_bytes);

    let parsed = MetaInfo2::parse(&text).expect("Failed to parse metainfo2.txt");
    assert_eq!(parsed.release.as_deref(), Some("HN+R_EU_AU_K0942_4"));
    assert_eq!(parsed.vendor.as_deref(), Some("HBAS"));
    assert!(parsed.sections.contains_key("common"));

    let adapter = MetaInfo2Adapter::default();
    assert!(adapter.detect(&metainfo_bytes));
    assert_eq!(adapter.coverage_ratio(&metainfo_bytes), 1.0);
    assert!(adapter.capabilities().can_rebuild);
}

#[test]
fn test_precomp_decode_and_identity_roundtrip() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let originals_dir = workspace_dir.join("originals");
    let store = SourceStore::new(&originals_dir).expect("Failed to open SourceStore");

    let precomp_path = "HN+R_EU_AU_K0942_4_[8R0906961FB]/CombiStyles/IND/0/default/arr_e.precomp";
    let raw_bytes = store.read_bytes(precomp_path).expect("Failed to read arr_e.precomp");

    let adapter = PrecompAdapter::default();
    assert!(adapter.detect(&raw_bytes));
    assert_eq!(adapter.coverage_ratio(&raw_bytes), 1.0);

    // 1. Decode Precomp
    let decoded = PrecompImage::decode(&raw_bytes).expect("Failed to decode Precomp");
    assert!(decoded.width > 0);
    assert!(decoded.height > 0);
    let expected_pixel_len = (decoded.width as usize) * (decoded.height as usize) * 4;
    assert_eq!(decoded.pixels.len(), expected_pixel_len);

    // 2. Encode back (Round-trip)
    let re_encoded = decoded.encode().expect("Failed to encode Precomp");

    // 3. Decode re-encoded bytes to verify bit-for-bit pixel preservation
    let re_decoded = PrecompImage::decode(&re_encoded).expect("Failed to decode re-encoded Precomp");
    assert_eq!(decoded.width, re_decoded.width);
    assert_eq!(decoded.height, re_decoded.height);
    assert_eq!(decoded.pixels, re_decoded.pixels);
}

#[test]
fn test_mapstyle_xar_real_corpus_parsing() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let originals_dir = workspace_dir.join("originals");
    let store = SourceStore::new(&originals_dir).expect("Failed to open SourceStore");

    let xar_path = "HN+R_EU_AU_K0942_4_[8R0906961FB]/MapStyles/ECE06/0/default/MMI3G_MapArchive_H_06_01.xar";
    let raw_bytes = store.read_bytes(xar_path).expect("Failed to read xar archive");

    let adapter = MapStyleXarAdapter::default();
    assert!(adapter.detect(&raw_bytes));
    assert!(adapter.coverage_ratio(&raw_bytes) > 0.8);

    let parsed = MapStyleXar::parse(&raw_bytes).expect("Failed to parse xar archive");
    assert!(parsed.declared_file_size > 0);
    assert!(parsed.member_count > 0);
}

#[test]
fn test_formats_fuzz_malformed_input_safety() {
    // Verify parsers never panic on truncated, garbage, or bomb inputs (§18)
    let garbage_inputs: Vec<Vec<u8>> = vec![
        vec![],
        vec![0x00],
        vec![0x00, 0x01, 0x00, 0x00, 0x00, 0x00], // truncated precomp header
        vec![0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF], // oversized dimension 65535x65535
        vec![0xFF; 256], // random garbage
        b"[section\nkey=val\n".to_vec(), // unclosed INI section
        b"key=\x00\xFF\xFE".to_vec(), // invalid byte sequences
    ];

    for (_i, input) in garbage_inputs.iter().enumerate() {
        let _ = PrecompImage::decode(input);
        let _ = PrecompAdapter::default().detect(input);
        let _ = MetaInfo2Adapter::default().detect(input);
        let _ = MapStyleXarAdapter::default().detect(input);
        let _ = MapStyleXar::parse(input);
        if let Ok(text) = std::str::from_utf8(input) {
            let _ = MetaInfo2::parse(text);
        }
    }
}

#[test]
fn test_hb_navdb_real_corpus_parsing() {
    use std::io::Read;
    use mmi_formats::{HbNavDb, HbNavDbAdapter};

    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let originals_dir = workspace_dir.join("originals");
    let store = SourceStore::new(&originals_dir).expect("Failed to open SourceStore");

    let db_path = "8R0051884KL_6.36.0_2023/pkgdb/LIT/EJ211Ga_L1.db";
    let mut file = store.open_read(db_path).expect("Failed to open EJ211Ga_L1.db");
    let mut header_buf = [0u8; 128];
    file.read_exact(&mut header_buf).expect("Failed to read header slice");

    let adapter = HbNavDbAdapter::default();
    assert!(adapter.detect(&header_buf));
    assert_eq!(adapter.coverage_ratio(&header_buf), 1.0);
    assert!(!adapter.capabilities().can_edit);
    assert!(!adapter.capabilities().can_rebuild);

    let parsed = HbNavDb::parse(&header_buf).expect("Failed to parse HbNavDb header");
    assert_eq!(parsed.header.page_size, 544);
    assert_eq!(parsed.header.root_page, 1);
    assert_eq!(parsed.header.version, 1);
    assert_eq!(&parsed.header.magic, b"FLDB");
}

#[test]
fn test_hb_atlas_real_corpus_parsing() {
    use std::io::Read;
    use mmi_formats::{HbAtlas, HbAtlasAdapter};

    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let originals_dir = workspace_dir.join("originals");
    let store = SourceStore::new(&originals_dir).expect("Failed to open SourceStore");

    let atlas_path = "8R0051884KL_6.36.0_2023/pkgdb/CTY/3PN221EU22083H1665a.4_2.0.ATLAS";
    let mut file = store.open_read(atlas_path).expect("Failed to open ATLAS file");
    let mut header_buf = [0u8; 128];
    file.read_exact(&mut header_buf).expect("Failed to read header slice");

    let adapter = HbAtlasAdapter::default();
    assert!(adapter.detect(&header_buf));
    assert_eq!(adapter.coverage_ratio(&header_buf), 1.0);
    assert!(!adapter.capabilities().can_edit);
    assert!(!adapter.capabilities().can_rebuild);

    let parsed = HbAtlas::parse(&header_buf).expect("Failed to parse HbAtlas header");
    assert_eq!(parsed.header.project_name, "Orion");
    assert_eq!(parsed.header.container_type, "Atlas");
}

#[test]
fn test_qnx_ifs_real_corpus_parsing() {
    use std::io::Read;
    use mmi_formats::{QnxIfs, QnxIfsAdapter, QNX_IFS_MAGIC};

    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let originals_dir = workspace_dir.join("originals");
    let store = SourceStore::new(&originals_dir).expect("Failed to open SourceStore");

    let ifs_path = "HN+R_EU_AU_K0942_4_[8R0906961FB]/MU9411/ifs-root/41/default/ifs-root.ifs";
    let mut file = store.open_read(ifs_path).expect("Failed to open ifs-root.ifs");
    let mut header_buf = [0u8; 128];
    file.read_exact(&mut header_buf).expect("Failed to read header slice");

    let adapter = QnxIfsAdapter::default();
    assert!(adapter.detect(&header_buf));
    assert_eq!(adapter.coverage_ratio(&header_buf), 1.0);

    let parsed = QnxIfs::parse(&header_buf).expect("Failed to parse QNX IFS header");
    assert_eq!(parsed.header.magic, QNX_IFS_MAGIC);
    assert!(parsed.header.startup_size > 0);
}

#[test]
fn test_qnx_efs_real_corpus_parsing() {
    use std::io::Read;
    use mmi_formats::{QnxEfs, QnxEfsAdapter, QNX_F3S_MAGIC};

    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let originals_dir = workspace_dir.join("originals");
    let store = SourceStore::new(&originals_dir).expect("Failed to open SourceStore");

    let efs_path = "HN+R_EU_AU_K0942_4_[8R0906961FB]/MU9411/efs-system/41/default/efs-system.efs";
    let mut file = store.open_read(efs_path).expect("Failed to open efs-system.efs");
    let mut header_buf = [0u8; 128];
    file.read_exact(&mut header_buf).expect("Failed to read header slice");

    let adapter = QnxEfsAdapter::default();
    assert!(adapter.detect(&header_buf));
    assert_eq!(adapter.coverage_ratio(&header_buf), 1.0);

    let parsed = QnxEfs::parse(&header_buf).expect("Failed to parse QNX EFS header");
    assert_eq!(&parsed.header.magic, QNX_F3S_MAGIC);
    assert_eq!(parsed.header.mount_point, "/mnt/efs-system");
}

#[test]
fn test_hb_ans_real_corpus_parsing() {
    use std::io::Read;
    use mmi_formats::{HbAns, HbAnsAdapter, ANS_MAGIC};

    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let originals_dir = workspace_dir.join("originals");
    let store = SourceStore::new(&originals_dir).expect("Failed to open SourceStore");

    let ans_path = "HN+R_EU_AU_K0942_4_[8R0906961FB]/sss/recog_de_DE/0/default/0.ans";
    let mut file = store.open_read(ans_path).expect("Failed to open 0.ans");
    let mut header_buf = [0u8; 64];
    file.read_exact(&mut header_buf).expect("Failed to read header slice");

    let adapter = HbAnsAdapter::default();
    assert!(adapter.detect(&header_buf));
    assert_eq!(adapter.coverage_ratio(&header_buf), 1.0);

    let parsed = HbAns::parse(&header_buf).expect("Failed to parse ANS header");
    assert_eq!(&parsed.header.magic, ANS_MAGIC);
}

#[test]
fn test_hb_fpga_real_corpus_parsing() {
    use std::io::Read;
    use mmi_formats::{HbFpga, HbFpgaAdapter, FPGA_HDG_TAG};

    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let originals_dir = workspace_dir.join("originals");
    let store = SourceStore::new(&originals_dir).expect("Failed to open SourceStore");

    let fpga_path = "HN+R_EU_AU_K0942_4_[8R0906961FB]/RSU9425/fpga-emg/61/default/SystemFPGA.hbbin";
    let mut file = store.open_read(fpga_path).expect("Failed to open SystemFPGA.hbbin");
    let mut header_buf = vec![0u8; 4096];
    file.read_exact(&mut header_buf).expect("Failed to read header slice");

    let adapter = HbFpgaAdapter::default();
    assert!(adapter.detect(&header_buf));
    assert_eq!(adapter.coverage_ratio(&header_buf), 1.0);

    let parsed = HbFpga::parse(&header_buf).expect("Failed to parse FPGA header");
    assert!(!parsed.chunks.is_empty());
    assert_eq!(&parsed.chunks[0].tag.as_bytes(), FPGA_HDG_TAG);
    assert!(parsed.header.hardware_info.contains("Audi_3G_PLUS"));
}

#[test]
fn test_smsc_ipf_real_corpus_parsing() {
    use std::io::Read;
    use mmi_formats::{SmscIpf, SmscIpfAdapter, SMSC_IPF_MAGIC};

    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let originals_dir = workspace_dir.join("originals");
    let store = SourceStore::new(&originals_dir).expect("Failed to open SourceStore");

    let ipf_path = "HN+R_EU_AU_K0942_4_[8R0906961FB]/ARU9469/Inic/41/default/C1_OS81050_FW_V01_11_01_CS_V02_02_14_checksum.ipf";
    let mut file = store.open_read(ipf_path).expect("Failed to open Inic.ipf");
    let mut header_buf = [0u8; 64];
    file.read_exact(&mut header_buf).expect("Failed to read header slice");

    let adapter = SmscIpfAdapter::default();
    assert!(adapter.detect(&header_buf));
    assert_eq!(adapter.coverage_ratio(&header_buf), 1.0);

    let parsed = SmscIpf::parse(&header_buf).expect("Failed to parse SMSC IPF");
    assert_eq!(&parsed.header.magic, SMSC_IPF_MAGIC);
}

#[test]
fn test_hb_gdb_real_corpus_parsing() {
    use std::io::Read;
    use mmi_formats::{HbGdb, HbGdbAdapter, GDB_MAGIC};

    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let originals_dir = workspace_dir.join("originals");
    let store = SourceStore::new(&originals_dir).expect("Failed to open SourceStore");

    let gdb_path = "8R0051884KL_6.36.0_2023/pkgdb/GDB/EJ211_v37a.gdb";
    let mut file = store.open_read(gdb_path).expect("Failed to open GDB");
    let mut header_buf = [0u8; 64];
    file.read_exact(&mut header_buf).expect("Failed to read header slice");

    let adapter = HbGdbAdapter::default();
    assert!(adapter.detect(&header_buf));
    assert_eq!(adapter.coverage_ratio(&header_buf), 1.0);

    let parsed = HbGdb::parse(&header_buf).expect("Failed to parse GDB header");
    assert_eq!(&parsed.header.magic, GDB_MAGIC);
    assert_eq!(parsed.header.version, 37);
}

#[test]
fn test_hb_grammar_real_corpus_parsing() {
    use std::io::Read;
    use mmi_formats::{HbGrammar, HbGrammarAdapter, HBGR_SIG};

    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let originals_dir = workspace_dir.join("originals");
    let store = SourceStore::new(&originals_dir).expect("Failed to open SourceStore");

    let hbgr_path = "HN+R_EU_AU_K0942_4_[8R0906961FB]/sss/tts_de_DE/0/default/navi_de_DE.hbgr";
    let mut file = store.open_read(hbgr_path).expect("Failed to open navi_de_DE.hbgr");
    let mut header_buf = [0u8; 128];
    file.read_exact(&mut header_buf).expect("Failed to read header slice");

    let adapter = HbGrammarAdapter::default();
    assert!(adapter.detect(&header_buf));
    assert_eq!(adapter.coverage_ratio(&header_buf), 1.0);

    let parsed = HbGrammar::parse(&header_buf).expect("Failed to parse HBGR header");
    assert_eq!(&parsed.header.signature, HBGR_SIG);
    assert!(parsed.header.banner.contains("BINARYGRAMMAR"));
    assert!(parsed.header.banner.contains("Project=audi"));
}

#[test]
fn test_adi_ldr_real_corpus_parsing() {
    use std::io::Read;
    use mmi_formats::{AdiLdr, AdiLdrAdapter, ADI_LDR_MAGIC};

    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let originals_dir = workspace_dir.join("originals");
    let store = SourceStore::new(&originals_dir).expect("Failed to open SourceStore");

    let ldr_path = "HN+R_EU_AU_K0942_4_[8R0906961FB]/BO_D4_ANC/MAIN/3/default/Bolo_full.ldr";
    let mut file = store.open_read(ldr_path).expect("Failed to open Bolo_full.ldr");
    let mut header_buf = [0u8; 64];
    file.read_exact(&mut header_buf).expect("Failed to read header slice");

    let adapter = AdiLdrAdapter::default();
    assert!(adapter.detect(&header_buf));
    assert_eq!(adapter.coverage_ratio(&header_buf), 1.0);

    let parsed = AdiLdr::parse(&header_buf).expect("Failed to parse LDR header");
    assert_eq!(&parsed.header.magic, ADI_LDR_MAGIC);
    assert_eq!(parsed.header.target_processor, 0x81);
}



