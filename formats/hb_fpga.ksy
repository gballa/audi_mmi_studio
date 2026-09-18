meta:
  id: hb_fpga
  title: Harman Becker System FPGA Bitstream Container
  file-extension: hbbin
  endian: le
doc: |
  Harman Becker tagged chunk container encapsulating Xilinx FPGA bitstream
  configuration for Audi MMI 3G+ hardware glue logic. Discovered in RSU9425/fpga.
seq:
  - id: chunks
    type: chunk
    repeat: eos

types:
  chunk:
    seq:
      - id: tag
        size: 4
        type: str
        encoding: ASCII
      - id: length
        type: u4
      - id: payload
        size: length - 8
