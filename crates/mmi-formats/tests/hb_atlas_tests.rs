use std::io::Cursor;
use mmi_formats::{
    AtlasWriter, HbAtlas, HbAtlasAdapter, FormatAdapter,
    ATLAS_DEFAULT_BLOCK_SIZE, ATLAS_HEADER_MIN_SIZE,
};

#[test]
fn test_atlas_writer_and_reader_roundtrip() {
    let mut buffer = Cursor::new(Vec::new());

    // 1. Create container and write sample tiles
    let mut writer = AtlasWriter::new(&mut buffer, ATLAS_DEFAULT_BLOCK_SIZE, "Orion", "Atlas")
        .expect("create writer");

    let tile1_payload = b"TERRAIN_DEM_ELEVATION_MESH_65x65_LEVEL_10";
    let tile2_payload = b"3D_LANDMARK_BUILDING_EXTRUDED_POLYGONS_LEVEL_12";

    writer.write_tile(0x1234_5678_9ABC_DEF0, tile1_payload).expect("write tile 1");
    writer.write_tile(0xFEDC_BA98_7654_3210, tile2_payload).expect("write tile 2");

    let tile_count = writer.finalize().expect("finalize writer");
    assert_eq!(tile_count, 2);

    let data = buffer.into_inner();
    assert!(data.len() > ATLAS_HEADER_MIN_SIZE);

    // 2. Format detection
    let adapter = HbAtlasAdapter::default();
    assert!(adapter.detect(&data));
    assert_eq!(adapter.coverage_ratio(&data), 1.0);

    // 3. Header verification
    let parsed = HbAtlas::parse(&data).expect("parse atlas");
    assert_eq!(parsed.header.tile_block_size, ATLAS_DEFAULT_BLOCK_SIZE);
    assert_eq!(parsed.header.version_major, 1);
    assert_eq!(parsed.header.version_minor, 0);
    assert_eq!(parsed.header.project_name, "Orion");
    assert_eq!(parsed.header.container_type, "Atlas");
    assert_eq!(parsed.header.index_size, 40); // 2 entries * 20 bytes

    // 4. Index table verification
    let index = parsed.parse_index(&data).expect("parse index");
    assert_eq!(index.len(), 2);

    assert_eq!(index[0].morton_key, 0x1234_5678_9ABC_DEF0);
    assert_eq!(index[0].uncompressed_size, tile1_payload.len() as u32);
    assert!(index[0].compressed_size > 0);
    assert_eq!(index[0].file_offset, ATLAS_HEADER_MIN_SIZE as u32);

    assert_eq!(index[1].morton_key, 0xFEDC_BA98_7654_3210);
    assert_eq!(index[1].uncompressed_size, tile2_payload.len() as u32);

    // 5. Decompressed tile payload roundtrip verification
    let decomp1 = parsed.read_tile(&data, &index[0]).expect("read tile 1");
    assert_eq!(decomp1, tile1_payload);

    let decomp2 = parsed.read_tile(&data, &index[1]).expect("read tile 2");
    assert_eq!(decomp2, tile2_payload);
}

#[test]
fn test_atlas_block_alignment_padding() {
    let mut buffer = Cursor::new(Vec::new());
    let mut writer = AtlasWriter::new(&mut buffer, 4096, "Orion", "Atlas").expect("create writer");

    let small_payload = vec![42u8; 100];
    writer.write_tile(1, &small_payload).expect("write tile");
    writer.finalize().expect("finalize");

    let data = buffer.into_inner();
    let parsed = HbAtlas::parse(&data).expect("parse atlas");
    let index = parsed.parse_index(&data).expect("parse index");

    // The index offset must be aligned to block size (Header: 64B + Padded block: 4096B = 4160B)
    assert_eq!(index[0].file_offset, 64);
    assert_eq!(parsed.header.index_offset, 64 + 4096);
}
