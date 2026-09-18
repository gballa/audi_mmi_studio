use std::path::Path;
use mmi_core::{ContentAddressedStore, CoreError, SourceStore};
use tempfile::tempdir;

#[test]
fn test_source_store_read_known_file() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let originals_dir = workspace_dir.join("originals");

    let store = SourceStore::new(&originals_dir).expect("Failed to initialize SourceStore");
    assert_eq!(store.root().as_path(), originals_dir.canonicalize().unwrap());

    // Test resolving a known file inside originals
    let rel_file = "HN+R_EU_AU_K0942_4_[8R0906961FB]/metainfo2.txt";
    let resolved = store.resolve(rel_file).expect("Failed to resolve metainfo2.txt");
    assert!(resolved.exists());
    assert!(resolved.is_file());

    // Test reading bytes
    let bytes = store.read_bytes(rel_file).expect("Failed to read metainfo2.txt bytes");
    assert!(!bytes.is_empty());
    assert!(String::from_utf8_lossy(&bytes).contains("HN+R_EU_AU_K0942_4"));

    // Test streaming hash
    let (blake3_hex, sha256_hex) = store.stream_hash(rel_file).expect("Failed to stream hash");
    assert_eq!(blake3_hex.len(), 64);
    assert_eq!(sha256_hex.len(), 64);
}

#[test]
fn test_source_store_path_traversal_rejection() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let originals_dir = workspace_dir.join("originals");
    let store = SourceStore::new(&originals_dir).expect("Failed to initialize SourceStore");

    // Attempt escaping with ..
    let result = store.resolve("../Cargo.toml");
    match result {
        Err(CoreError::PathEscape(_)) => { /* Expected */ }
        Err(e) => panic!("Expected PathEscape error, got: {:?}", e),
        Ok(path) => panic!("Path traversal succeeded unexpectedly: {:?}", path),
    }
}

#[test]
fn test_cas_ingest_dedup_and_materialize() {
    let tmp = tempdir().expect("Failed to create tempdir");
    let cas = ContentAddressedStore::new(tmp.path().join(".mmistudio")).expect("Failed to create CAS");

    let test_payload = b"AUDI_MMI_STUDIO_CAS_TEST_PAYLOAD_1234567890";
    let (b3_1, sha_1) = cas.put_bytes(test_payload).expect("Failed to put bytes in CAS");

    assert!(cas.has_blob(&b3_1));
    assert_eq!(cas.read_bytes(&b3_1).unwrap(), test_payload);

    // Test deduplication
    let (b3_2, sha_2) = cas.put_bytes(test_payload).expect("Failed to put duplicate bytes in CAS");
    assert_eq!(b3_1, b3_2);
    assert_eq!(sha_1, sha_2);

    // Test materialization
    let materialized_dst = tmp.path().join("workspace").join("job-1").join("extracted.bin");
    cas.materialize(&b3_1, &materialized_dst).expect("Failed to materialize blob");
    assert!(materialized_dst.exists());
    let read_back = std::fs::read(&materialized_dst).expect("Failed to read materialized file");
    assert_eq!(read_back, test_payload);
}

#[test]
fn test_signed_artefact_error_representation() {
    let err = CoreError::SignedArtefactImmutable("MMI3GP_ECE_Hi_R_6_36_0.pkg".to_string());
    assert!(err.to_string().contains("ERR_SIGNED_ARTEFACT_IMMUTABLE"));
}
