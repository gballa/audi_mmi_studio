use mmi_formats::hb_lit::{
    compute_rotary_alpha_mask, LitSpellerBranch, LitSpellerNode,
    LitStreetRecord,
};

#[test]
fn test_rotary_dial_alpha_mask() {
    let mask = compute_rotary_alpha_mask(b"TIRANA");
    // Should have bits for A, I, N, R, T
    assert_ne!(mask & (1 << (b'A' - b'A')), 0);
    assert_ne!(mask & (1 << (b'I' - b'A')), 0);
    assert_ne!(mask & (1 << (b'N' - b'A')), 0);
    assert_ne!(mask & (1 << (b'R' - b'A')), 0);
    assert_ne!(mask & (1 << (b'T' - b'A')), 0);
    // B should NOT be set
    assert_eq!(mask & (1 << (b'B' - b'A')), 0);
}

#[test]
fn test_speller_node_payload_roundtrip() {
    let node = LitSpellerNode {
        page_idx: 5,
        valid_alpha_mask: 0x000F_0000,
        node_flags: 1,
        match_count: 1420,
        branches: vec![
            LitSpellerBranch {
                branch_char: b'A',
                match_subcount: 120,
                child_page_idx: 12,
                entity_record_idx: 4,
            },
            LitSpellerBranch {
                branch_char: b'B',
                match_subcount: 85,
                child_page_idx: 13,
                entity_record_idx: 0,
            },
        ],
    };

    let payload = node.serialize_payload();
    assert_eq!(payload.len(), 512);

    let parsed = LitSpellerNode::parse_payload(5, &payload).expect("Parse payload");
    assert_eq!(parsed, node);
}

#[test]
fn test_street_record_roundtrip() {
    let rec = LitStreetRecord {
        street_id: 101,
        city_id: 1,
        frc: 2,
        name: "RRUGA E DIBRES".to_string(),
        primary_edge_id: 504,
        centroid_x: 236234000,
        centroid_y: 492000000,
    };

    let bytes = rec.serialize();
    let parsed = LitStreetRecord::parse(&bytes);
    assert_eq!(parsed, rec);
}
