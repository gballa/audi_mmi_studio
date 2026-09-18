meta:
  id: hb_gdb
  title: Harman Becker Geographic Routing Database
  file-extension: gdb
  endian: be
doc: |
  Harman Becker proprietary compressed road routing network graph.
seq:
  - id: magic
    contents: [0xde, 0xad, 0xbe, 0xef]
  - id: version
    type: u4
  - id: flags
    type: u4
  - id: reserved
    size: 52
  - id: data
    size-eos: true
