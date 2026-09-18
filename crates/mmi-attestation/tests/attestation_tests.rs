use mmi_attestation::{
    AttestationGenerator, SafetyLinter, StatusVocabulary, StockRecoveryBundler,
};
use tempfile::TempDir;

#[test]
fn test_safety_linter_bans_forbidden_phrase() {
    let benign = "BUILD READY — DEPLOYMENT NOT VERIFIED";
    assert!(SafetyLinter::assert_no_forbidden_phrase(benign).is_ok());

    let forbidden = "This modification is completely SAFE TO INSTALL on vehicle";
    assert!(SafetyLinter::assert_no_forbidden_phrase(forbidden).is_err());
}

#[test]
fn test_attestation_generator_produces_valid_manifest() {
    let temp_src = TempDir::new().unwrap();
    let temp_build = TempDir::new().unwrap();

    std::fs::write(temp_src.path().join("input.dat"), b"source_data").unwrap();
    std::fs::write(temp_build.path().join("output.dat"), b"built_data").unwrap();

    let manifest = AttestationGenerator::generate(
        temp_src.path(),
        temp_build.path(),
        "HN+R_EU_AU_K0942_4",
        "stage_test",
        Some("recipe_123".to_string()),
    )
    .unwrap();

    assert_eq!(manifest.source_train, "HN+R_EU_AU_K0942_4");
    assert_eq!(manifest.target_stage, "stage_test");
    assert_eq!(manifest.total_input_files, 1);
    assert_eq!(manifest.total_output_files, 1);
    assert_eq!(manifest.status_verdict, StatusVocabulary::BuildReadyDeploymentNotVerified);
}

#[test]
fn test_stock_recovery_bundler_handles_missing_and_existing() {
    let temp_orig = TempDir::new().unwrap();
    let temp_rec = TempDir::new().unwrap();

    // 1. Test missing train -> flags HIGH RISK — NO VERIFIED RECOVERY PATH
    let missing_report = StockRecoveryBundler::prepare_recovery_bundle(
        temp_orig.path(),
        "NON_EXISTENT_TRAIN",
        temp_rec.path(),
    )
    .unwrap();

    assert_eq!(
        missing_report.status,
        StatusVocabulary::HighRiskNoVerifiedRecoveryPath
    );
    assert!(!missing_report.is_recovery_ready);

    // 2. Test valid existing train
    let train_dir = temp_orig.path().join("STOCK_TRAIN");
    std::fs::create_dir_all(&train_dir).unwrap();
    std::fs::write(train_dir.join("metainfo2.txt"), b"MetaInfo2").unwrap();

    let valid_report = StockRecoveryBundler::prepare_recovery_bundle(
        temp_orig.path(),
        "STOCK_TRAIN",
        temp_rec.path(),
    )
    .unwrap();

    assert_eq!(valid_report.status, StatusVocabulary::Verified);
    assert!(valid_report.is_recovery_ready);
    assert!(temp_rec.path().join("RECOVERY_README.md").exists());
}
