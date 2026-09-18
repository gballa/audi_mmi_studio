use mmi_rebuild::{
    BundlePackager, DeterminismParity, RebuildVerifier, StageNormalizer,
};
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_stage_normalizer_lexicographical_order() {
    let temp_stage = TempDir::new().unwrap();
    let root = temp_stage.path();

    // Create files out of order in different directories
    std::fs::create_dir_all(root.join("z_dir")).unwrap();
    std::fs::create_dir_all(root.join("a_dir")).unwrap();
    std::fs::write(root.join("z_dir").join("file2.txt"), b"data2").unwrap();
    std::fs::write(root.join("a_dir").join("file1.txt"), b"data1").unwrap();
    std::fs::write(root.join("root_file.txt"), b"root").unwrap();

    let entries = StageNormalizer::collect_normalized_tree(root).unwrap();
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].relative_path, "a_dir/file1.txt");
    assert_eq!(entries[1].relative_path, "root_file.txt");
    assert_eq!(entries[2].relative_path, "z_dir/file2.txt");
}

#[test]
fn test_repackager_and_verifier_roundtrip_on_corpus_precomp() {
    let corpus_precomp = Path::new("originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/CombiStyles/IND/0/default/arr_e.precomp");
    if !corpus_precomp.exists() {
        eprintln!("Corpus precomp not found, skipping test.");
        return;
    }

    let temp_stage = TempDir::new().unwrap();
    let temp_output = TempDir::new().unwrap();

    // Setup stage structure
    let stage_precomp = temp_stage.path().join("arr_e.precomp");
    std::fs::copy(corpus_precomp, &stage_precomp).unwrap();

    let results = BundlePackager::repackage_stage(temp_stage.path(), temp_output.path()).unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].relative_path, "arr_e.precomp");

    // Verify determinism parity
    let summary = RebuildVerifier::verify_tree(temp_stage.path(), temp_output.path()).unwrap();
    assert!(summary.is_deterministic);
    assert_eq!(summary.discrepancy_count, 0);
    assert!(
        summary.files[0].parity == DeterminismParity::BitForBit
            || summary.files[0].parity == DeterminismParity::Canonical
    );
}

#[test]
fn test_verifier_detects_signed_payload_protection() {
    let temp_stage = TempDir::new().unwrap();
    let temp_output = TempDir::new().unwrap();

    // Add a signed file
    let signed_path = temp_stage.path().join("ifs-root.pkg");
    std::fs::write(&signed_path, b"signed_binary_content").unwrap();

    BundlePackager::repackage_stage(temp_stage.path(), temp_output.path()).unwrap();

    let summary = RebuildVerifier::verify_tree(temp_stage.path(), temp_output.path()).unwrap();
    assert_eq!(summary.signed_immutable_count, 1);
    assert_eq!(summary.files[0].parity, DeterminismParity::SignedImmutable);
    assert!(summary.files[0].notes.contains("Signed payload protected"));
}
