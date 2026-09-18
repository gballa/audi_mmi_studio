use mmi_assets::{AssetCataloger, AssetDecoder, FontInspector, ThumbnailGenerator};
use mmi_core::{ContentAddressedStore, StageEntry};
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_decode_real_precomp_from_corpus() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let precomp_path = workspace_dir
        .join("originals")
        .join("HN+R_EU_AU_K0942_4_[8R0906961FB]")
        .join("RSU")
        .join("graphics")
        .join("view_kombi_k0.precomp");

    if !precomp_path.exists() {
        eprintln!("Corpus file not present, skipping");
        return;
    }

    let bytes = std::fs::read(&precomp_path).unwrap();
    let decoded = AssetDecoder::decode(&bytes).expect("Failed to decode real precomp");

    assert_eq!(decoded.source_format, "precomp");
    assert!(decoded.width > 0);
    assert!(decoded.height > 0);
    assert_eq!(decoded.rgba_pixels.len(), (decoded.width * decoded.height * 4) as usize);

    // Test PNG conversion
    let png_bytes = decoded.to_png_bytes().expect("PNG encoding failed");
    assert!(!png_bytes.is_empty());
    assert_eq!(&png_bytes[1..4], b"PNG");

    // Test thumbnail generation
    let dir = tempdir().unwrap();
    let cas = ContentAddressedStore::new(dir.path()).unwrap();
    let thumb_blob_id = ThumbnailGenerator::create_thumbnail(&decoded, 64, &cas).unwrap();
    assert!(cas.has_blob(&thumb_blob_id));
}

#[test]
fn test_font_inspection_real_corpus() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let font_path = workspace_dir
        .join("originals")
        .join("HN+R_EU_AU_K0942_4_[8R0906961FB]")
        .join("GEMMI")
        .join("res")
        .join("fonts")
        .join("AudiUnivers540Med.ttf");

    if !font_path.exists() {
        eprintln!("Font file not found, skipping");
        return;
    }

    let bytes = std::fs::read(&font_path).unwrap();
    let font_info = FontInspector::inspect(&bytes).expect("Failed to inspect font");

    assert!(font_info.family_name.is_some());
    let family = font_info.family_name.unwrap();
    assert!(family.contains("AudiUnivers") || family.contains("Univers"));
    assert!(font_info.glyph_count > 0);
    assert!(font_info.tables.contains(&"head".to_string()));
    assert!(font_info.tables.contains(&"name".to_string()));
}

#[test]
fn test_asset_cataloger_flow() {
    let dir = tempdir().unwrap();
    let cas = ContentAddressedStore::new(dir.path().join(".mmistudio")).unwrap();

    // Create synthetic precomp
    let precomp = mmi_formats::PrecompImage {
        width: 16,
        height: 16,
        pixels: vec![255; 16 * 16 * 4],
    };
    let precomp_bytes = precomp.encode().unwrap();
    let (blob_id, sha256_hex) = cas.put_bytes(&precomp_bytes).unwrap();

    let entry = StageEntry {
        logical_path: "train/graphics/test.precomp".to_string(),
        blob_id,
        sha256_hex,
        byte_size: precomp_bytes.len() as u64,
        is_signed: false,
        module_name: Some("graphics".to_string()),
    };

    let cataloger = AssetCataloger::new(&cas);
    let desc = cataloger.catalog_entry(&entry).unwrap().expect("descriptor cataloged");

    assert_eq!(desc.format, "precomp");
    assert_eq!(desc.width, Some(16));
    assert_eq!(desc.height, Some(16));
    assert!(desc.can_edit);
    assert!(desc.can_rebuild);
    assert!(cas.has_blob(&desc.blob_id));
}
