meta:
  id: hb_navdb
  title: Harman Becker Navigation Fast Lookup Database (FLDB)
  file-extension: db
  endian: le
  license: Proprietary / Reverse-Engineered

doc: |
  Harman Becker Navigation database format discovered in Audi MMI 3G High / Plus map packages.
  Identified by fixed 36-byte header with 'FLDB' signature at offset 0x14.
  Contains fixed-stride pages and lookup records for city points, POIs, and street names.

seq:
  - id: header
    type: header

types:
  header:
    seq:
      - id: page_size
        type: u4
        doc: Page size in bytes (typically 0x0220 = 544 bytes)
      - id: root_page
        type: u4
        doc: Index of the root B-tree node or entry directory
      - id: timestamp
        type: u4
        doc: Unix epoch build timestamp
      - id: version
        type: u4
        doc: Database structure version (typically 1)
      - id: header_size
        type: u4
        doc: Size of this header structure (typically 0x24 = 36 bytes)
      - id: magic
        contents: "FLDB"
        doc: Fast Lookup Database magic signature
      - id: reserved
        size: 16
        doc: Reserved padding or table flags
