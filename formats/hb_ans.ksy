meta:
  id: hb_ans
  title: Harman Becker Speech Prompts Audio Container (ANS)
  file-extension: ans
  endian: le
  license: Proprietary / Reverse-Engineered

doc: |
  Harman Becker navigation and voice prompt speech audio container.
  Identified by magic string 'ANS\0' at offset 0x00 followed by speech parameter headers.

seq:
  - id: header
    type: ans_header

types:
  ans_header:
    seq:
      - id: magic
        contents: ["ANS", 0x00]
        doc: ANS signature
      - id: reserved
        type: u2
      - id: header_size
        type: u2
      - id: codec_id
        size: 4
      - id: padding
        size: 16
