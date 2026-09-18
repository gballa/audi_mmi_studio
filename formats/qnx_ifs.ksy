meta:
  id: qnx_ifs
  title: QNX 6 Image FileSystem (IFS)
  file-extension: ifs
  endian: le
  license: Proprietary / Reverse-Engineered

doc: |
  QNX Neutrino 6 bootable Image FileSystem (.ifs) containing OS kernel (procnto),
  startup executable, hardware drivers, and system services.
  Identified by startup signature 0x00ff7eeb at offset 0x00.

seq:
  - id: header
    type: startup_header

types:
  startup_header:
    seq:
      - id: magic
        contents: [0xeb, 0x7e, 0xff, 0x00]
        doc: QNX startup header signature (0x00ff7eeb in little-endian)
      - id: version
        type: u2
      - id: flags
        type: u2
      - id: header_size
        type: u2
      - id: machine_type
        type: u2
      - id: startup_size
        type: u4
      - id: stored_size
        type: u4
      - id: image_size
        type: u4
      - id: ram_size
        type: u4
