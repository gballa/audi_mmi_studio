use std::io::Cursor;
use mmi_formats::hb_gdb::{
    create_gdb_page, gdb_crc16_ccitt, GdbEdgeRecord, GdbMasterHeader, GdbNodeRecord,
    GdbPage, GdbPageHeader, GdbSpatialTileRecord, GdbTurnRestrictionRecord, HbGdb,
    HbGdbReader, HbGdbWriter, GDB_MAGIC, GDB_PAGE_SIZE, GDB_PAYLOAD_SIZE, GDB_TRAILER_SYNC,
    GDB_VERSION_37,
};

#[test]
fn test_gdb_crc16_ccitt_vector() {
    let test_bytes = b"123456789";
    let crc = gdb_crc16_ccitt(test_bytes);
    // Standard CCITT with 0xFFFF seed on "123456789" produces 0x29B1
    assert_eq!(crc, 0x29B1);
}

#[test]
fn test_gdb_page_header_serialize_parse_roundtrip() {
    let header = GdbPageHeader {
        page_id: 1234,
        crc16: 0xABCD,
        flags: 0x0001,
        payload_len: 480,
        reserved: [1, 2, 3, 4, 5, 6],
    };

    let serialized = header.serialize();
    assert_eq!(serialized.len(), 16);

    let parsed = GdbPageHeader::parse(&serialized);
    assert_eq!(parsed, header);
}

#[test]
fn test_gdb_page_creation_and_validation() {
    let payload = vec![0x42; 256];
    let page = GdbPage::new(42, 0, &payload);
    assert_eq!(page.header.page_id, 42);
    assert_eq!(page.header.payload_len, 256);
    assert_eq!(page.trailer_sync, GDB_TRAILER_SYNC);
    assert!(page.verify_crc());

    let bytes = page.to_bytes();
    assert_eq!(bytes.len(), GDB_PAGE_SIZE);

    let parsed_page = GdbPage::parse(&bytes).expect("parse page");
    assert_eq!(parsed_page.header.page_id, 42);
    assert_eq!(parsed_page.header.payload_len, 256);
    assert!(parsed_page.verify_crc());
}

#[test]
fn test_create_gdb_page_and_hbgdb_parse() {
    let header = GdbMasterHeader {
        magic: *GDB_MAGIC,
        version: GDB_VERSION_37,
        total_pages: 1,
        ..Default::default()
    };
    let page_bytes = create_gdb_page(0, &header.serialize_payload());
    assert_eq!(page_bytes.len(), GDB_PAGE_SIZE);

    let parsed = HbGdb::parse(&page_bytes).expect("parse paged gdb");
    assert_eq!(&parsed.header.magic, GDB_MAGIC);
    assert_eq!(parsed.header.version, GDB_VERSION_37);
    assert_eq!(parsed.total_pages, 1);
}

#[test]
fn test_gdb_page_tamper_fails_crc() {
    let payload = vec![0xAA; 512];
    let page = GdbPage::new(1, 0, &payload);
    let mut bytes = page.to_bytes();

    // Tamper with payload byte
    bytes[100] ^= 0xFF;

    let res = GdbPage::parse(&bytes);
    assert!(res.is_err(), "tampered payload must fail CRC");
}

#[test]
fn test_gdb_page_trailer_sync_corruption_fails() {
    let payload = vec![0xBB; 512];
    let page = GdbPage::new(2, 0, &payload);
    let mut bytes = page.to_bytes();

    // Corrupt sync trailer (offset 528..532)
    bytes[528] = 0x00;

    let res = GdbPage::parse(&bytes);
    assert!(res.is_err(), "tampered sync trailer must fail");
}

#[test]
fn test_gdb_master_header_roundtrip() {
    let header = GdbMasterHeader {
        magic: *GDB_MAGIC,
        version: GDB_VERSION_37,
        flags: 0x10,
        total_pages: 50,
        node_count: 500,
        edge_count: 750,
        restriction_count: 12,
        morton_root_page: 45,
        frc_layer_offsets: [5, 10, 15, 20, 25, 30, 35, 40],
        page_size: 544,
        morton_tile_count: 8,
        frc_layer_counts: [100, 100, 100, 100, 100, 100, 75, 75],
    };

    let payload = header.serialize_payload();
    assert_eq!(payload.len(), GDB_PAYLOAD_SIZE);

    let parsed = GdbMasterHeader::parse(&payload).expect("parse master header");
    assert_eq!(parsed.magic, *GDB_MAGIC);
    assert_eq!(parsed.version, 37);
    assert_eq!(parsed.flags, 0x10);
    assert_eq!(parsed.total_pages, 50);
    assert_eq!(parsed.node_count, 500);
    assert_eq!(parsed.edge_count, 750);
    assert_eq!(parsed.restriction_count, 12);
    assert_eq!(parsed.morton_root_page, 45);
    assert_eq!(parsed.page_size, 544);
    assert_eq!(parsed.morton_tile_count, 8);
    assert_eq!(parsed.frc_layer_offsets, header.frc_layer_offsets);
    assert_eq!(parsed.frc_layer_counts, header.frc_layer_counts);
}

#[test]
fn test_gdb_node_record_roundtrip() {
    let node = GdbNodeRecord::new(99999, 42123456, 19654321, 0b0000_0011, 7);
    let b = node.serialize();
    assert_eq!(b.len(), 16);
    let parsed = GdbNodeRecord::parse(&b);
    assert_eq!(parsed, node);
    assert_eq!(parsed.morton_tile_id(), 7);
}

#[test]
fn test_gdb_edge_record_roundtrip() {
    let edge = GdbEdgeRecord {
        edge_id: 1001,
        from_node: 50,
        to_node: 51,
        length_dm: 4200,
        frc: 1,
        speed_forward: 110,
        speed_backward: 90,
        lanes: 3,
        turn_lanes_mask: 0x0124,
        access_flags: 0x00FF,
    };
    let b = edge.serialize();
    assert_eq!(b.len(), 24);
    let parsed = GdbEdgeRecord::parse(&b);
    assert_eq!(parsed, edge);
}

#[test]
fn test_gdb_turn_restriction_record_roundtrip() {
    let restr = GdbTurnRestrictionRecord::new(1, 1001, 51, 1002, 1, 15);
    let b = restr.serialize();
    assert_eq!(b.len(), 24);
    let parsed = GdbTurnRestrictionRecord::parse(&b);
    assert_eq!(parsed.restriction_id, 1);
    assert_eq!(parsed.from_edge, 1001);
    assert_eq!(parsed.via_node, 51);
    assert_eq!(parsed.to_edge, 1002);
    assert_eq!(parsed.restriction_type, 1);
    assert_eq!(parsed.penalty_s, 15);
}

#[test]
fn test_gdb_spatial_tile_record_roundtrip() {
    let tile = GdbSpatialTileRecord {
        morton_key: 0x0123_4567_89AB_CDEF,
        tile_id: 42,
        start_node_idx: 100,
        node_count: 50,
        start_edge_idx: 200,
        edge_count: 80,
        min_lat: 41000000,
        min_lon: 19000000,
        max_lat: 42000000,
        max_lon: 20000000,
        reserved: 0,
    };
    let b = tile.serialize();
    assert_eq!(b.len(), 48);
    let parsed = GdbSpatialTileRecord::parse(&b);
    assert_eq!(parsed, tile);
}

#[test]
fn test_hb_gdb_writer_and_reader_roundtrip() {
    let mut cursor = Cursor::new(Vec::new());
    let mut writer = HbGdbWriter::new(&mut cursor).expect("create writer");

    // Write Page 1: Nodes (2 nodes)
    let node1 = GdbNodeRecord::new(1, 41320000, 19820000, 1, 0);
    let node2 = GdbNodeRecord::new(2, 41330000, 19830000, 1, 0);
    let mut node_buf = Vec::new();
    node_buf.extend_from_slice(&node1.serialize());
    node_buf.extend_from_slice(&node2.serialize());
    let node_page_id = writer.write_payload_page(0, &node_buf).expect("write node page");
    assert_eq!(node_page_id, 1);

    // Write Page 2: Edge for FRC 0
    let edge1 = GdbEdgeRecord {
        edge_id: 10,
        from_node: 1,
        to_node: 2,
        length_dm: 1500,
        frc: 0,
        speed_forward: 130,
        speed_backward: 130,
        lanes: 2,
        turn_lanes_mask: 0,
        access_flags: 1,
    };
    let edge_page_id = writer.write_payload_page(0, &edge1.serialize()).expect("write edge page");
    assert_eq!(edge_page_id, 2);

    // Write Page 3: Turn Restriction
    let restr1 = GdbTurnRestrictionRecord::new(1, 10, 2, 11, 1, 0xFFFF);
    let restr_page_id = writer.write_payload_page(0, &restr1.serialize()).expect("write restr page");
    assert_eq!(restr_page_id, 3);

    // Write Page 4: Spatial Tile
    let tile1 = GdbSpatialTileRecord {
        morton_key: 0x1234,
        tile_id: 0,
        start_node_idx: 0,
        node_count: 2,
        start_edge_idx: 0,
        edge_count: 1,
        min_lat: 41320000,
        min_lon: 19820000,
        max_lat: 41330000,
        max_lon: 19830000,
        reserved: 0,
    };
    let tile_page_id = writer.write_payload_page(0, &tile1.serialize()).expect("write tile page");
    assert_eq!(tile_page_id, 4);

    // Finalize with Master Header
    let mut frc_offsets = [0u32; 8];
    frc_offsets[0] = 2; // FRC 0 is at Page 2
    let mut frc_counts = [0u32; 8];
    frc_counts[0] = 1;

    let header = GdbMasterHeader {
        magic: *GDB_MAGIC,
        version: GDB_VERSION_37,
        flags: 0,
        total_pages: 5,
        node_count: 2,
        edge_count: 1,
        restriction_count: 1,
        morton_root_page: 4,
        frc_layer_offsets: frc_offsets,
        page_size: 544,
        morton_tile_count: 1,
        frc_layer_counts: frc_counts,
    };
    writer.finalize(&header).expect("finalize writer");

    // Verify written data with Reader
    let data = cursor.into_inner();
    assert_eq!(data.len(), 5 * GDB_PAGE_SIZE);

    let reader = HbGdbReader::from_bytes(&data).expect("read gdb bytes");
    assert_eq!(reader.page_count(), 5);
    reader.validate_all_pages().expect("validate all pages");

    let nodes = reader.read_nodes().expect("read nodes");
    assert_eq!(nodes.len(), 2);
    assert_eq!(nodes[0].node_id, 1);
    assert_eq!(nodes[1].node_id, 2);

    let frc0_edges = reader.read_edges_frc(0).expect("read frc0 edges");
    assert_eq!(frc0_edges.len(), 1);
    assert_eq!(frc0_edges[0].edge_id, 10);
    assert_eq!(frc0_edges[0].speed_forward, 130);

    let restrs = reader.read_restrictions().expect("read restrictions");
    assert_eq!(restrs.len(), 1);
    assert_eq!(restrs[0].restriction_id, 1);
    assert_eq!(restrs[0].penalty_s, 0xFFFF);

    let tiles = reader.read_spatial_tiles().expect("read tiles");
    assert_eq!(tiles.len(), 1);
    assert_eq!(tiles[0].morton_key, 0x1234);
}
