meta:
  id: hb_atlas
  title: Harman Becker Orion Atlas Spatial Tile Container
  file-extension: atlas
  endian: le
  license: Proprietary / Reverse-Engineered

doc: |
  Harman Becker Orion Atlas spatial tile container used in Audi MMI 3G High / Plus.
  Stores multi-resolution 3D city models, terrain elevation, and landmark geometry.
  Header starts with Pascal-string 'HEADER' padded with 0xCC, followed by project
  identifier 'Orion' and type identifier 'Atlas'.

seq:
  - id: header
    type: atlas_header

types:
  atlas_header:
    seq:
      - id: header_tag_len
        type: u1
        doc: Length of header magic tag (6)
      - id: header_tag
        type: str
        size: header_tag_len
        encoding: ASCII
        doc: "HEADER"
      - id: pad_1
        size: 9
        doc: Padding bytes (0xCC)
      - id: tile_block_size
        type: u4
        doc: Tile cluster block size
      - id: version_major
        type: u2
      - id: version_minor
        type: u2
      - id: index_offset
        type: u4
      - id: index_size
        type: u4
      - id: project_tag_len
        type: u1
        doc: Length of project identifier (5)
      - id: project_tag
        type: str
        size: project_tag_len
        encoding: ASCII
        doc: "Orion"
      - id: pad_2
        size: 10
      - id: container_tag_len
        type: u1
        doc: Length of container identifier (5)
      - id: container_tag
        type: str
        size: container_tag_len
        encoding: ASCII
        doc: "Atlas"
