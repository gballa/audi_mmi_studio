meta:
  id: precomp
  title: Harman Becker Precomp Instrument Cluster Graphic
  file-extension: precomp
  endian: be
doc: |
  Harman Becker instrument cluster graphics format used in Audi MMI 3G+ HN+ / HN+R
  software trains under CombiStyles/ directories. Contains a 10-byte header with dimensions
  followed by a standard zlib-compressed 32-bit RGBA raw bitmap.

seq:
  - id: magic
    contents: [0x00, 0x01, 0x00, 0x00, 0x00, 0x00]
    doc: 6-byte fixed precomp container header signature

  - id: width
    type: u2
    doc: Pixel width of the uncompressed graphic (16-bit big-endian)

  - id: height
    type: u2
    doc: Pixel height of the uncompressed graphic (16-bit big-endian)

  - id: compressed_data
    size-eos: true
    process: zlib
    doc: Raw 32-bit RGBA pixel stream (width * height * 4 bytes uncompressed)
