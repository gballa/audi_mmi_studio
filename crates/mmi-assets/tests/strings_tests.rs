use mmi_assets::{EncodingType, FontMetricsEngine, OverflowPredictor, StringCatalog, UiBoundingBox};
use std::path::Path;

#[test]
fn test_string_catalog_encodings() {
    // 1. ASCII / Key-Value
    let ascii_data = b"ENTRY_1=Welcome\nENTRY_2=System Setup\n";
    let cat_ascii = StringCatalog::parse(ascii_data).unwrap();
    assert_eq!(cat_ascii.encoding, EncodingType::Ascii);
    assert_eq!(cat_ascii.entries.len(), 2);
    assert_eq!(cat_ascii.entries[0].key.as_deref(), Some("ENTRY_1"));
    assert_eq!(cat_ascii.entries[0].value, "Welcome");

    // 2. UTF-8
    let utf8_data = "NAVI_BERLIN=Berlin Hauptbahnhof (Äpfel, Über, Größe)\n".as_bytes();
    let cat_utf8 = StringCatalog::parse(utf8_data).unwrap();
    assert_eq!(cat_utf8.encoding, EncodingType::Utf8);
    assert_eq!(cat_utf8.entries[0].value, "Berlin Hauptbahnhof (Äpfel, Über, Größe)");

    // 3. UTF-16LE with BOM
    let mut utf16le = vec![0xFF, 0xFE];
    for c in "TITLE=Navigation Plus\n".encode_utf16() {
        utf16le.extend_from_slice(&c.to_le_bytes());
    }
    let cat_utf16le = StringCatalog::parse(&utf16le).unwrap();
    assert_eq!(cat_utf16le.encoding, EncodingType::Utf16Le);
    assert_eq!(cat_utf16le.entries[0].value, "Navigation Plus");

    // 4. ISO-8859-1 (Latin-1)
    let latin1_bytes = vec![0x53, 0x74, 0x72, 0x61, 0xdf, 0x65]; // "Straße" in Latin-1 where ß is 0xDF
    let cat_latin1 = StringCatalog::parse(&latin1_bytes).unwrap();
    assert_eq!(cat_latin1.encoding, EncodingType::Latin1);
    assert_eq!(cat_latin1.entries[0].value, "Straße");
}

#[test]
fn test_parse_real_corpus_plde_file() {
    let plde_path = Path::new("originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/sss/tts_de_DE/0/default/plde-DE0.txt");
    if !plde_path.exists() {
        eprintln!("Corpus file plde-DE0.txt not found, skipping test.");
        return;
    }

    let bytes = std::fs::read(plde_path).unwrap();
    let cat = StringCatalog::parse(&bytes).unwrap();
    assert!(cat.entries.len() > 10, "Should have parsed multiple phoneme rule entries");
    eprintln!("Parsed {} entries with encoding {:?}", cat.entries.len(), cat.encoding);
}

#[test]
fn test_font_metrics_and_overflow_with_real_font() {
    let font_path = Path::new("originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/GEMMI/nav/0/default/res/AudiUnivers540Med.ttf");
    if !font_path.exists() {
        eprintln!("Corpus font AudiUnivers540Med.ttf not found, skipping test.");
        return;
    }

    let font_bytes = std::fs::read(font_path).unwrap();
    let engine = FontMetricsEngine::parse(&font_bytes).unwrap();

    assert!(engine.units_per_em > 0);
    assert!(!engine.glyph_advances.is_empty());
    assert!(!engine.char_to_glyph.is_empty());

    // Short string fitting within 300px at 16pt
    let short_text = "Audi MMI";
    let bounds_fit = UiBoundingBox {
        max_width_px: 300.0,
        max_height_px: 40.0,
        max_lines: 1,
        font_size_pt: 16.0,
    };
    let report_fit = OverflowPredictor::evaluate(&engine, short_text, &bounds_fit);
    assert!(report_fit.fits, "Short text should fit in 300px container");
    assert_eq!(report_fit.overflow_width_px, 0.0);
    assert!(!report_fit.requires_complex_shaping);

    // Long string that overflows a tight 80px container
    let long_text = "Audi MMI Navigation Plus High Definition Multimedia";
    let bounds_tight = UiBoundingBox {
        max_width_px: 80.0,
        max_height_px: 20.0,
        max_lines: 1,
        font_size_pt: 16.0,
    };
    let report_overflow = OverflowPredictor::evaluate(&engine, long_text, &bounds_tight);
    assert!(!report_overflow.fits, "Long text must overflow an 80px container");
    assert!(report_overflow.overflow_width_px > 50.0);
    assert!(report_overflow.overflow_percentage > 50.0);
}

#[test]
fn test_arabic_font_bidi_detection() {
    let font_path = Path::new("originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/GEMMI/nav/0/default/res/LT_Univers440_88perc_Arabic.ttf");
    if !font_path.exists() {
        eprintln!("Corpus font LT_Univers440_88perc_Arabic.ttf not found, skipping test.");
        return;
    }

    let font_bytes = std::fs::read(font_path).unwrap();
    let engine = FontMetricsEngine::parse(&font_bytes).unwrap();

    let arabic_text = "نظام الملاحة أودي";
    let bounds = UiBoundingBox {
        max_width_px: 500.0,
        max_height_px: 50.0,
        max_lines: 1,
        font_size_pt: 18.0,
    };
    let report = OverflowPredictor::evaluate(&engine, arabic_text, &bounds);
    assert!(report.requires_complex_shaping, "Arabic text should trigger BiDi and complex shaping flag");
}
