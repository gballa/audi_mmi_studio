use std::io::Cursor;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;

use mmi_rebuild::pbf_proto::{PbfBlobReader, ProtoWireReader, ProtoWireType};
use mmi_rebuild::osm_ingest::CompactNodeStore;

#[test]
fn test_proto_wire_reader_varint() {
    // 0 -> [0x00]
    let mut reader = ProtoWireReader::new(&[0x00]);
    assert_eq!(reader.read_varint().unwrap(), 0);

    // 1 -> [0x01]
    let mut reader = ProtoWireReader::new(&[0x01]);
    assert_eq!(reader.read_varint().unwrap(), 1);

    // 300 -> [0xAC, 0x02]
    let mut reader = ProtoWireReader::new(&[0xAC, 0x02]);
    assert_eq!(reader.read_varint().unwrap(), 300);

    // u64::MAX
    let max_bytes = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01];
    let mut reader = ProtoWireReader::new(&max_bytes);
    assert_eq!(reader.read_varint().unwrap(), u64::MAX);
}

#[test]
fn test_proto_wire_reader_zigzag() {
    // 0 -> 0
    let mut reader = ProtoWireReader::new(&[0x00]);
    assert_eq!(reader.read_sint64().unwrap(), 0);

    // -1 -> 1
    let mut reader = ProtoWireReader::new(&[0x01]);
    assert_eq!(reader.read_sint64().unwrap(), -1);

    // 1 -> 2
    let mut reader = ProtoWireReader::new(&[0x02]);
    assert_eq!(reader.read_sint64().unwrap(), 1);

    // -2 -> 3
    let mut reader = ProtoWireReader::new(&[0x03]);
    assert_eq!(reader.read_sint64().unwrap(), -2);

    // Test sint32 as well
    let mut reader = ProtoWireReader::new(&[0x01]);
    assert_eq!(reader.read_sint32().unwrap(), -1);
}

#[test]
fn test_proto_wire_reader_packed_arrays() {
    // Packed sint64: [-1, 1, -2, 2] -> zigzag [1, 2, 3, 4]
    // Length delimited: len=4, followed by 0x01, 0x02, 0x03, 0x04
    let buf = [0x04, 0x01, 0x02, 0x03, 0x04];
    let mut reader = ProtoWireReader::new(&buf);
    let vals = reader.read_packed_sint64().unwrap();
    assert_eq!(vals, vec![-1, 1, -2, 2]);

    // Packed uint32: [10, 20, 30]
    let buf_u32 = [0x03, 10, 20, 30];
    let mut reader = ProtoWireReader::new(&buf_u32);
    let u32s = reader.read_packed_uint32().unwrap();
    assert_eq!(u32s, vec![10, 20, 30]);
}

#[test]
fn test_proto_wire_reader_tags_and_skipping() {
    // Tag: field 1, type Varint (tag = (1 << 3) | 0 = 8), val = 42
    // Tag: field 2, type LengthDelimited (tag = (2 << 3) | 2 = 18), len = 5, "hello"
    // Tag: field 3, type Fixed32 (tag = (3 << 3) | 5 = 29), 4 bytes
    let mut buf = Vec::new();
    buf.push(8);
    buf.push(42);
    buf.push(18);
    buf.push(5);
    buf.extend_from_slice(b"hello");
    buf.push(29);
    buf.extend_from_slice(&[1, 2, 3, 4]);

    let mut reader = ProtoWireReader::new(&buf);
    let (field1, type1) = reader.next_tag().unwrap().unwrap();
    assert_eq!(field1, 1);
    assert_eq!(type1, ProtoWireType::Varint);
    assert_eq!(reader.read_varint().unwrap(), 42);

    let (field2, type2) = reader.next_tag().unwrap().unwrap();
    assert_eq!(field2, 2);
    assert_eq!(type2, ProtoWireType::LengthDelimited);
    assert_eq!(reader.read_string().unwrap(), "hello");

    let (field3, type3) = reader.next_tag().unwrap().unwrap();
    assert_eq!(field3, 3);
    assert_eq!(type3, ProtoWireType::Fixed32);
    reader.skip_field(type3).unwrap();
    assert!(reader.is_empty());
}

#[test]
fn test_pbf_blob_reader_roundtrip() {
    // Create an OSMData blob with zlib payload
    let payload = b"UNCOMPRESSED_PRIMITIVE_BLOCK_DATA_PAYLOAD";
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(payload).unwrap();
    let compressed_zlib = encoder.finish().unwrap();

    // Encode Blob protobuf: field 3 (zlib_data, LengthDelimited)
    let mut blob_proto = Vec::new();
    // tag: (3 << 3) | 2 = 26
    blob_proto.push(26);
    blob_proto.push(compressed_zlib.len() as u8);
    blob_proto.extend_from_slice(&compressed_zlib);

    // Encode BlobHeader protobuf: field 1 = "OSMData", field 3 = datasize
    let mut header_proto = Vec::new();
    // field 1 (type): (1 << 3) | 2 = 10
    header_proto.push(10);
    header_proto.push(7); // len("OSMData")
    header_proto.extend_from_slice(b"OSMData");
    // field 3 (datasize): (3 << 3) | 0 = 24
    header_proto.push(24);
    header_proto.push(blob_proto.len() as u8);

    // Pack into wire stream: 4-byte BE length + header + blob
    let mut pbf_stream = Vec::new();
    pbf_stream.extend_from_slice(&(header_proto.len() as u32).to_be_bytes());
    pbf_stream.extend_from_slice(&header_proto);
    pbf_stream.extend_from_slice(&blob_proto);

    let mut cursor = Cursor::new(&pbf_stream);
    let block_header = PbfBlobReader::read_block_header(&mut cursor)
        .expect("Read block header succeeded")
        .expect("Header present");

    assert_eq!(block_header.block_type, "OSMData");
    assert_eq!(block_header.datasize, blob_proto.len());

    let mut decompressed = Vec::new();
    PbfBlobReader::read_decompressed_payload(&mut cursor, block_header.datasize, &mut decompressed)
        .expect("Decompress succeeded");

    assert_eq!(decompressed, payload);
}

#[test]
fn test_compact_node_store() {
    let mut store = CompactNodeStore::new();
    assert!(store.is_empty());
    assert_eq!(store.len(), 0);

    store.insert(1001, 41.3275, 19.8189, Some(110));
    store.insert(1002, 41.3280, 19.8195, None);

    assert_eq!(store.len(), 2);
    assert!(!store.is_empty());

    let node1 = store.get(1001).expect("Found node 1001");
    assert_eq!(node1.0, 41.3275);
    assert_eq!(node1.1, 19.8189);
    assert_eq!(node1.2, Some(110));

    let node2 = store.get(1002).expect("Found node 1002");
    assert_eq!(node2.0, 41.3280);
    assert_eq!(node2.1, 19.8195);
    assert_eq!(node2.2, None);

    assert!(store.get(9999).is_none());
}
