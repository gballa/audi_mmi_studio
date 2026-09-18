meta:
  id: qnx_efs
  title: QNX 6 Embedded Flash FileSystem (F3S / ETFS)
  file-extension: efs
  endian: le
  license: Proprietary / Reverse-Engineered

doc: |
  QNX Embedded Flash FileSystem (F3S / ETFS) used for writable system partitions
  such as /mnt/efs-system and /mnt/efs-extended.
  Identified by signature 'QSSL_F3S' at offset 0x2C.

seq:
  - id: header
    type: efs_super_block

types:
  efs_super_block:
    seq:
      - id: pad_prefix
        size: 44
      - id: magic
        contents: "QSSL_F3S"
        doc: QSSL_F3S signature (QNX Software Systems Ltd Flash 3 FileSystem)
      - id: pad_meta
        size: 20
      - id: mount_path
        type: strz
        encoding: ASCII
        doc: Partition mount point (e.g. /mnt/efs-system)
