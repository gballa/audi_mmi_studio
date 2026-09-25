use std::fs;
use tempfile::tempdir;

use mmi_rebuild::fldb_compiler::{
    crc16_ccitt, FLDB_MAGIC, FLDB_PAGE_SIZE, FLDB_TRAILER_SYNC,
};
use mmi_rebuild::StreamingFldbWriter;

#[test]
fn test_streaming_fldb_writer_single_volume() {
    let dir = tempdir().expect("Create temp dir");
    let mut writer = StreamingFldbWriter::new(dir.path(), "nav_data.db").expect("Create writer");

    for i in 0..10 {
        let payload = format!("PAGE_PAYLOAD_{:04}", i);
        let page_idx = writer.write_page(payload.as_bytes(), FLDB_MAGIC).expect("Write page");
        assert_eq!(page_idx, i as u32);
    }

    assert_eq!(writer.total_pages(), 10);
    assert_eq!(writer.total_bytes(), 10 * (FLDB_PAGE_SIZE as u64));
    assert_eq!(writer.volume_count(), 1);

    writer.finish().expect("Finish writer");

    let db_path = dir.path().join("nav_data.db");
    assert!(db_path.exists());
    let metadata = fs::metadata(&db_path).expect("Read metadata");
    assert_eq!(metadata.len(), 10 * (FLDB_PAGE_SIZE as u64));

    // Verify 544-byte structure and CRC on disk
    let file_bytes = fs::read(&db_path).expect("Read db file");
    for i in 0..10 {
        let page = &file_bytes[i * FLDB_PAGE_SIZE..(i + 1) * FLDB_PAGE_SIZE];

        // Check magic
        assert_eq!(&page[0..4], FLDB_MAGIC);

        // Check trailer
        let trailer = u32::from_le_bytes([page[528], page[529], page[530], page[531]]);
        assert_eq!(trailer, FLDB_TRAILER_SYNC);

        // Check CRC-16
        let stored_crc = u16::from_le_bytes([page[8], page[9]]);
        let computed_crc = crc16_ccitt(&page[16..528]);
        assert_eq!(stored_crc, computed_crc);
    }
}
