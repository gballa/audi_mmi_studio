use mmi_media::{Fat32Constraints, MediaBuilder, PreFlightSimulator, VolumeSplitter};
use std::path::{Path, PathBuf};
use tempfile::TempDir;

#[test]
fn test_volume_splitter_splits_when_limit_exceeded() {
    let files = vec![
        (PathBuf::from("file1.bin"), 1000),
        (PathBuf::from("file2.bin"), 900),
        (PathBuf::from("file3.bin"), 800),
    ];

    // Max volume size 2000 bytes -> (1000 + 900 = 1900 <= 2000) fits vol 1, (800) fits vol 2
    let volumes = VolumeSplitter::partition_volumes(&files, 2000, "MMI_TEST");
    assert_eq!(volumes.len(), 2);
    assert_eq!(volumes[0].volume_label, "MMI_TEST_1");
    assert_eq!(volumes[0].files.len(), 2);
    assert_eq!(volumes[1].volume_label, "MMI_TEST_2");
    assert_eq!(volumes[1].files.len(), 1);
}

#[test]
fn test_media_builder_and_manifest_generation() {
    let temp_stage = TempDir::new().unwrap();
    let temp_output = TempDir::new().unwrap();

    // Create stage structure
    let file_path = temp_stage.path().join("nav").join("models.dat");
    std::fs::create_dir_all(file_path.parent().unwrap()).unwrap();
    std::fs::write(&file_path, b"test_navigation_model").unwrap();

    let constraints = Fat32Constraints::default();
    let results = MediaBuilder::build(
        temp_stage.path(),
        temp_output.path(),
        "MMI3G_NAV",
        &constraints,
    )
    .unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].volume_label, "MMI3G_NAV");
    assert!(temp_output.path().join("media_manifest.json").exists());
}

#[test]
fn test_pre_flight_simulator_on_genuine_corpus() {
    let corpus_dir = Path::new("originals/HN+R_EU_AU_K0942_4_[8R0906961FB]");
    if !corpus_dir.exists() {
        eprintln!("Corpus directory not found, skipping test.");
        return;
    }

    let report = PreFlightSimulator::simulate_media(corpus_dir).unwrap();
    assert_eq!(report.disclaimer, "SIMULATED — NOT A GUARANTEE");
    assert!(report.overall_success);
    assert_eq!(report.final_state, mmi_media::UpdateState::Completed);
    assert!(report.steps.len() >= 5);
}

#[test]
fn test_media_sanitizer_cleans_junk_and_preserves_payload() {
    use mmi_media::MediaSanitizer;
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();

    // Legitimate firmware files
    let metainfo = root.join("metainfo2.txt");
    std::fs::write(&metainfo, b"release = 2026_ECE").unwrap();
    let valid_sub = root.join("HBNavDB").join("nav_data.db");
    std::fs::create_dir_all(valid_sub.parent().unwrap()).unwrap();
    std::fs::write(&valid_sub, b"fldb_page_data").unwrap();

    // Host OS junk files
    let ds_store = root.join(".DS_Store");
    std::fs::write(&ds_store, b"mac_desktop_services").unwrap();
    let apple_double = root.join("HBNavDB").join("._nav_data.db");
    std::fs::write(&apple_double, b"resource_fork").unwrap();
    let thumbs = root.join("Thumbs.db");
    std::fs::write(&thumbs, b"windows_thumbs").unwrap();
    let trash_dir = root.join(".Trashes");
    std::fs::create_dir_all(&trash_dir).unwrap();
    std::fs::write(trash_dir.join("junk"), b"trashed").unwrap();

    // 1. Dry run
    let dry_report = MediaSanitizer::sanitize(root, true).unwrap();
    assert_eq!(dry_report.purged_items.len(), 4);
    assert!(dry_report.is_dry_run);
    assert!(ds_store.exists());
    assert!(apple_double.exists());

    // 2. Real run
    let live_report = MediaSanitizer::sanitize(root, false).unwrap();
    assert_eq!(live_report.purged_items.len(), 4);
    assert!(!live_report.is_dry_run);
    assert!(!ds_store.exists());
    assert!(!apple_double.exists());
    assert!(!thumbs.exists());
    assert!(!trash_dir.exists());

    // Legitimate files MUST be preserved
    assert!(metainfo.exists());
    assert!(valid_sub.exists());
}
