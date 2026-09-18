use std::path::Path;
use mmi_core::SourceStore;
use mmi_re_lab::{
    ByteHistogram, CarvedFormat, EntropyCalculator, EntropyClass, HexViewer, SignatureCarver,
    StringExtractor,
};

#[test]
fn test_hex_viewer_rendering() {
    let sample = b"Hello, Audi MMI Studio RE Lab!\x00\xFF\x01\x02";
    let rows = HexViewer::render_slice(sample, 0, 16);
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].offset, 0);
    assert_eq!(rows[0].ascii, "Hello, Audi MMI ");

    let hexdump = HexViewer::format_hexdump(sample, 0x1000);
    assert!(hexdump.contains("00001000"));
    assert!(hexdump.contains("Hello, Audi MMI "));
}

#[test]
fn test_entropy_computation_and_classification() {
    // 1. Zero-filled buffer -> Entropy 0.0
    let zeros = vec![0u8; 1024];
    let h_zeros = EntropyCalculator::shannon_entropy(&zeros);
    assert_eq!(h_zeros, 0.0);
    assert_eq!(EntropyClass::from_entropy(h_zeros), EntropyClass::Plaintext);

    // 2. English text -> Entropy ~ 4.0 - 5.0
    let text = b"The quick brown fox jumps over the lazy dog. Repetitive English text sample for testing.";
    let h_text = EntropyCalculator::shannon_entropy(text);
    assert!(h_text > 3.0 && h_text < 5.5);

    // 3. Segment strip computation
    let mut mixed = vec![0u8; 512];
    mixed.extend((0..=255).cycle().take(512));
    let strip = EntropyCalculator::compute_strip(&mixed, 512);
    assert_eq!(strip.len(), 2);
    assert_eq!(strip[0].classification, EntropyClass::Plaintext);
    assert!(strip[1].entropy > 7.5);
}

#[test]
fn test_string_extraction() {
    let extractor = StringExtractor::new(4);
    let buffer = b"\x00\x01\x02Hello World\x00\xFF\xFE\x00Good Bye\x00";
    let extracted = extractor.extract_ascii_utf8(buffer);

    assert_eq!(extracted.len(), 2);
    assert_eq!(extracted[0].value, "Hello World");
    assert_eq!(extracted[0].offset, 3);
    assert_eq!(extracted[1].value, "Good Bye");
}

#[test]
fn test_byte_histogram() {
    let buffer = b"\x00\x00\x00\x00ABCD";
    let hist = ByteHistogram::compute(buffer);
    assert_eq!(hist.total_bytes, 8);
    assert_eq!(hist.counts[0], 4);
    assert_eq!(hist.counts[b'A' as usize], 1);
    assert_eq!(hist.null_byte_ratio, 0.5);
    assert_eq!(hist.printable_ratio, 0.5);
}

#[test]
fn test_signature_carver_on_real_corpus_files() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let originals_dir = workspace_dir.join("originals");

    let store = SourceStore::new(&originals_dir).expect("Failed to open SourceStore");

    // 1. Test carving on metainfo2.txt
    let metainfo_bytes = store
        .read_bytes("HN+R_EU_AU_K0942_4_[8R0906961FB]/metainfo2.txt")
        .expect("Failed to read metainfo2.txt");
    let strings = StringExtractor::default().extract_ascii_utf8(&metainfo_bytes);
    assert!(strings.iter().any(|s| s.value.contains("HN+R_EU_AU_K0942_4")));

    // 2. Test carving on a known precomp file
    let precomp_path = "HN+R_EU_AU_K0942_4_[8R0906961FB]/CombiStyles/IND/0/default/arr_e.precomp";
    let precomp_bytes = store.read_bytes(precomp_path).expect("Failed to read arr_e.precomp");
    let carved = SignatureCarver::scan(&precomp_bytes);
    assert!(carved.iter().any(|c| c.format == CarvedFormat::HarmanPrecomp));
    assert!(carved.iter().any(|c| c.format == CarvedFormat::ZlibStream));
}
