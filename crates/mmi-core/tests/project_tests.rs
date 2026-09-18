use mmi_core::{
    ConfidenceLevel, ContentAddressedStore, MMIProject, PackageExtractor, PlatformVariant,
    SourceStore, StageStore,
};
use tempfile::tempdir;

#[test]
fn test_mmi_project_model_defaults() {
    let mut project = MMIProject::new("PRJ-001", "Audi MMI 3G+ Navigation");
    assert_eq!(project.metadata.name, "Audi MMI 3G+ Navigation");
    assert_eq!(project.detected_platform.confidence, ConfidenceLevel::Unknown);

    // Register signed payload
    project.register_signed_artefact(
        "MMI3GP_ECE_Hi_R_6_36_0.pkg",
        Some("MMI3GP_ECE_Hi_R_6_36_0.pkg.sig".to_string()),
        "blake3_hash",
        "sha256_hash",
    );

    assert_eq!(project.signed_artefacts.len(), 1);
    assert!(!project.signed_artefacts[0].can_edit);
    assert!(!project.signed_artefacts[0].can_rebuild);
    assert!(project.signed_artefacts[0].policy.contains("ANALYSIS-ONLY"));
}

#[test]
fn test_stage_store_in_memory() {
    let mut stage = StageStore::in_memory("test_stage").expect("stage created");
    assert_eq!(stage.count_entries().unwrap(), 0);

    let entry = mmi_core::StageEntry {
        logical_path: "train/module/file.txt".to_string(),
        blob_id: "fake_blob_id".to_string(),
        sha256_hex: "fake_sha256".to_string(),
        byte_size: 1234,
        is_signed: false,
        module_name: Some("module".to_string()),
    };

    stage.put_entry(&entry).expect("entry stored");
    assert_eq!(stage.count_entries().unwrap(), 1);

    let fetched = stage.get_entry("train/module/file.txt").unwrap().expect("fetched");
    assert_eq!(fetched.blob_id, "fake_blob_id");
    assert_eq!(fetched.byte_size, 1234);
}

#[test]
fn test_package_extractor_with_sample() {
    let dir = tempdir().unwrap();
    let source_dir = dir.path().join("originals");
    let train_dir = source_dir.join("HN+R_EU_AU_K0942_4");
    let mod_dir = train_dir.join("RSU");
    std::fs::create_dir_all(&mod_dir).unwrap();

    let meta_path = train_dir.join("metainfo2.txt");
    std::fs::write(
        &meta_path,
        "[common]\nrelease = \"HN+R_EU_AU_K0942_4\"\nvendor = \"HBAS\"\n",
    )
    .unwrap();

    let dummy_pkg = mod_dir.join("payload.pkg");
    std::fs::write(&dummy_pkg, b"DUMMY_SIGNED_PAYLOAD").unwrap();

    let source_store = SourceStore::new(&source_dir).unwrap();
    let cas_dir = dir.path().join(".mmistudio");
    let cas = ContentAddressedStore::new(&cas_dir).unwrap();

    let stages_dir = cas_dir.join("stages");
    let mut stage_store = StageStore::open(&stages_dir, "test_job").unwrap();
    let mut project = MMIProject::new("PRJ-SAMPLE", "Test Project");

    let extractor = PackageExtractor::new(&source_store, &cas);
    extractor
        .extract_package("HN+R_EU_AU_K0942_4", &mut stage_store, &mut project)
        .unwrap();

    assert_eq!(stage_store.count_entries().unwrap(), 2);
    assert_eq!(project.source_refs.len(), 2);

    // Verify platform and train detection
    assert_eq!(project.detected_platform.confidence, ConfidenceLevel::Known);
    assert_eq!(project.detected_platform.value, PlatformVariant::HnPlusR);
    assert_eq!(
        project.software_train.as_ref().unwrap().value,
        "HN+R_EU_AU_K0942_4"
    );

    // Verify signed artefact lockout
    assert_eq!(project.signed_artefacts.len(), 1);
    assert_eq!(project.signed_artefacts[0].can_edit, false);
    assert_eq!(project.signed_artefacts[0].can_rebuild, false);

    // Verify module aggregation
    assert_eq!(project.modules.len(), 1);
    assert_eq!(project.modules[0].name, "RSU");
    assert!(project.modules[0].is_signed);
}
