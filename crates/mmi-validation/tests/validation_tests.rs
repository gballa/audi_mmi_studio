use mmi_validation::{
    BuildReadinessStatus, FindingSeverity, MmiGeneration, TargetProfile, ValidationEngine,
    ValidationLevel,
};
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_target_profile_serialization() {
    let profile = TargetProfile {
        name: "Audi A6 C7 Target".to_string(),
        generation: MmiGeneration::Mmi3GPlus,
        head_unit_part_number: "4G0035670".to_string(),
        current_software_train: "HN+R_EU_AU_K0942_4".to_string(),
        target_software_train: "HN+R_EU_AU_K0942_4".to_string(),
        hardware_revisions: vec!["41".to_string(), "51".to_string()],
        region: "EU".to_string(),
        display_resolution: (800, 480),
        user_verified: true,
    };

    let json_str = profile.to_json_pretty().unwrap();
    let deserialized = TargetProfile::from_json(&json_str).unwrap();
    assert_eq!(deserialized.name, profile.name);
    assert_eq!(deserialized.generation, MmiGeneration::Mmi3GPlus);
    assert_eq!(deserialized.hardware_revisions.len(), 2);
}

#[test]
fn test_validation_engine_on_clean_corpus_sample() {
    let corpus_dir = Path::new("originals/HN+R_EU_AU_K0942_4_[8R0906961FB]");
    if !corpus_dir.exists() {
        eprintln!("Corpus directory not found, skipping test.");
        return;
    }

    let profile = TargetProfile::default();
    let report = ValidationEngine::validate_stage(corpus_dir, Some(&profile), None).unwrap();

    // The genuine stock corpus contains valid metainfo2.txt and valid precomp binaries
    assert_eq!(report.error_count, 0, "Stock bundle must have zero validation errors");
    assert!(
        report.status == BuildReadinessStatus::Verified
            || report.status == BuildReadinessStatus::BuildReadyDeploymentNotVerified
    );
}

#[test]
fn test_validation_engine_catches_invalid_magic_error() {
    let temp_stage = TempDir::new().unwrap();
    let corrupt_precomp = temp_stage.path().join("broken.precomp");
    std::fs::write(&corrupt_precomp, b"NOT_A_PRECOMP").unwrap();

    let profile = TargetProfile::default();
    let report = ValidationEngine::validate_stage(temp_stage.path(), Some(&profile), Some(ValidationLevel::L1FileStructure)).unwrap();

    assert!(report.error_count > 0, "Should detect invalid magic header as error");
    assert_eq!(report.status, BuildReadinessStatus::Failed);
    assert!(report.findings.iter().any(|f| f.code == "L1_INVALID_MAGIC" && f.severity == FindingSeverity::Error));
}

#[test]
fn test_validation_without_profile_yields_compatibility_unknown() {
    let temp_stage = TempDir::new().unwrap();
    std::fs::write(temp_stage.path().join("readme.txt"), b"documentation").unwrap();

    let report = ValidationEngine::validate_stage(temp_stage.path(), None, None).unwrap();
    assert_eq!(report.status, BuildReadinessStatus::CompatibilityUnknown);
}
