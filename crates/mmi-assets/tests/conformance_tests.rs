use image::{ImageBuffer, Rgba};
use mmi_assets::{AssetConstraints, AssetDecoder, AssetReplacer, ConformancePipeline, ConformanceVerdict};
use mmi_core::ContentAddressedStore;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_conformance_pipeline_admit_precomp() {
    let dir = tempdir().unwrap();
    let cas = ContentAddressedStore::new(dir.path()).unwrap();

    // Create a 100x100 PNG candidate image
    let candidate_img: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_pixel(100, 100, Rgba([255, 0, 0, 255]));
    let mut candidate_png = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut candidate_png);
    candidate_img
        .write_to(&mut cursor, image::ImageFormat::Png)
        .unwrap();

    // Constraints requiring 64x64 precomp
    let constraints = AssetConstraints {
        target_width: 64,
        target_height: 64,
        target_format: "precomp".to_string(),
        max_encoded_bytes: Some(10000),
        require_alpha: false,
    };

    let verdict = ConformancePipeline::process(&candidate_png, &constraints, &cas);
    match verdict {
        ConformanceVerdict::Admitted {
            blob_id,
            width,
            height,
            format,
            ..
        } => {
            assert_eq!(width, 64);
            assert_eq!(height, 64);
            assert_eq!(format, "precomp");
            assert!(cas.has_blob(&blob_id));

            // Verify the stored blob decodes properly
            let blob_data = cas.read_bytes(&blob_id).unwrap();
            let decoded = AssetDecoder::decode(&blob_data).unwrap();
            assert_eq!(decoded.width, 64);
            assert_eq!(decoded.height, 64);
        }
        ConformanceVerdict::Rejected { reason, details, .. } => {
            panic!("Expected Admitted, got Rejected: {} ({})", reason, details);
        }
    }
}

#[test]
fn test_conformance_pipeline_reject_oversized() {
    let dir = tempdir().unwrap();
    let cas = ContentAddressedStore::new(dir.path()).unwrap();

    let candidate_img: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_pixel(64, 64, Rgba([0, 255, 0, 255]));
    let mut candidate_png = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut candidate_png);
    candidate_img
        .write_to(&mut cursor, image::ImageFormat::Png)
        .unwrap();

    // Max encoded bytes is ridiculously small (10 bytes)
    let constraints = AssetConstraints {
        target_width: 64,
        target_height: 64,
        target_format: "precomp".to_string(),
        max_encoded_bytes: Some(10),
        require_alpha: false,
    };

    let verdict = ConformancePipeline::process(&candidate_png, &constraints, &cas);
    assert!(matches!(verdict, ConformanceVerdict::Rejected { stage, .. } if stage == "VERIFY"));
}

#[test]
fn test_asset_replacer_on_real_corpus_asset() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let precomp_path = workspace_dir
        .join("originals")
        .join("HN+R_EU_AU_K0942_4_[8R0906961FB]")
        .join("CombiStyles")
        .join("IND")
        .join("0")
        .join("default")
        .join("arr_e.precomp");

    if !precomp_path.exists() {
        return;
    }

    let orig_bytes = std::fs::read(&precomp_path).unwrap();
    let dir = tempdir().unwrap();
    let cas = ContentAddressedStore::new(dir.path()).unwrap();

    // Replacement candidate: square 200x200 PNG
    let candidate_img: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_pixel(200, 200, Rgba([100, 150, 200, 255]));
    let mut candidate_png = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut candidate_png);
    candidate_img
        .write_to(&mut cursor, image::ImageFormat::Png)
        .unwrap();

    let record = AssetReplacer::replace(
        "arr_e.precomp",
        &orig_bytes,
        &candidate_png,
        &cas,
    )
    .expect("Replacement failed");

    assert_eq!(record.target_asset, "arr_e.precomp");
    assert_eq!(record.width, 500); // Conformed to original dimensions
    assert_eq!(record.height, 248);
    assert_eq!(record.format, "precomp");
    assert!(cas.has_blob(&record.replacement_blob_id));
}
