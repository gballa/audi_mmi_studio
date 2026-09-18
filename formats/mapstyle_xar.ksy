meta:
  id: mapstyle_xar
  title: Harman / Elektrobit MapStyles Regional Archive
  file-extension: xar
  endian: le
doc: |
  Harman / Elektrobit regional map styling archive container used in Audi MMI 3G+
  MapStyles/ and StyleDB/ directories. Distinct from macOS XAR; starts with magic 'rax\0'.

seq:
  - id: magic
    contents: ['r', 'a', 'x', 0x00]
    doc: 4-byte magic signature 'rax\0'

  - id: version
    type: u4
    doc: Container format version (typically 0x00010000)

  - id: total_file_size
    type: u4
    doc: Total size of the archive in bytes

  - id: header_size
    type: u4
    doc: Size of the archive header and table of contents
