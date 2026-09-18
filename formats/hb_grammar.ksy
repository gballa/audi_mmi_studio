meta:
  id: hb_grammar
  title: Harman Becker Binary Grammar Resource
  file-extension: hbgr
  endian: le
doc: |
  Harman Becker compiled binary grammar resource for acoustic prompts and TTS.
seq:
  - id: signature
    contents: [0xfe, 0xff, 0xff, 0xff]
  - id: header_length
    type: u4
  - id: header_text
    size: header_length - 8
    type: str
    encoding: ASCII
  - id: body
    size-eos: true
