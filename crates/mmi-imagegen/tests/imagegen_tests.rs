use mmi_core::ContentAddressedStore;
use mmi_imagegen::{EgressAirlock, ImageEditProvider, ImageGenError, OfflineMockProvider};
use tempfile::tempdir;

#[test]
fn test_airlock_denies_signed_payload_egress() {
    let airlock = EgressAirlock::new(true);

    let res = airlock.verify_asset_egress("GEMMI/nav/models/MMI3GP_ECE_Hi_R_6_36_0.pkg");
    assert!(matches!(res, Err(ImageGenError::EgressDenied(_))));

    let res_lic = airlock.verify_asset_egress("License/Vlasoff_activation.sh");
    assert!(matches!(res_lic, Err(ImageGenError::EgressDenied(_))));

    let res_ok = airlock.verify_asset_egress("RSU/graphics/view_kombi_k0.precomp");
    assert!(res_ok.is_ok());
}

#[test]
fn test_airlock_prompt_sanitization_and_brand_check() {
    let airlock = EgressAirlock::new(true);

    let prompt = "Please draw a high-tech arrow icon for VIN WAUZZZ4G0BN123456 located at originals/HN+R/RSU";
    let sanitized = airlock.sanitize_prompt(prompt);

    assert!(!sanitized.contains("WAUZZZ4G0BN123456"));
    assert!(sanitized.contains("[REDACTED_VIN]"));
    assert!(!sanitized.contains("originals/HN+R/RSU"));
    assert!(sanitized.contains("[REDACTED_PATH]"));

    let brand_warning = airlock.check_brand_mark("Draw a chrome Audi emblem");
    assert!(brand_warning.is_some());
    assert!(brand_warning.unwrap().contains("audi"));
}

#[test]
fn test_offline_mock_provider_and_cas_pinning() {
    let dir = tempdir().unwrap();
    let cas = ContentAddressedStore::new(dir.path()).unwrap();

    let provider = OfflineMockProvider::default();
    assert!(provider.capabilities().is_offline);

    let asset = provider
        .generate("arrow navigation symbol", 64, 64)
        .expect("Generation failed");

    assert_eq!(asset.width, 64);
    assert_eq!(asset.height, 64);
    assert_eq!(&asset.png_bytes[1..4], b"PNG");

    // Pin to CAS
    let (blob_id, sha256_hex) = cas.put_bytes(&asset.png_bytes).unwrap();
    assert!(cas.has_blob(&blob_id));
    assert!(!sha256_hex.is_empty());
}
