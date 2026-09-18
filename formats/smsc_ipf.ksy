meta:
  id: smsc_ipf
  title: SMSC MOST Intelligent Network Interface Controller Programming File
  file-extension: ipf
  endian: be
doc: |
  SMSC IPF binary container for flashing OS81050/OS81110 MOST bus controllers.
seq:
  - id: signature
    contents: [0x01, 0x0f, 0xff, 0xff, 0xff, 0xff, 0x01, 0x01, 0x00, 0x00, 0x20, 0x00, 0x00, 0x01, 0xdc, 0x00]
  - id: payload
    size-eos: true
