# Reference — Reverse-Engineered Binary Formats (RQ-001 through RQ-012)

This document provides the definitive reverse-engineered specifications for all 12 proprietary automotive binary formats analyzed and decoded in **Audi MMI Studio**.

---

## Format Summary Matrix

| RQ ID | Extension | Format Name | Magic / Signature | Primary Use | Rebuild Gate Status |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **RQ-001** | `.db` | HBNavDB | `FLDB` (`0x464C4442`) | Navigation spatial database | Permitted (Unsigned) |
| **RQ-002** | `.atlas` | Orion ATLAS | `\x06HEADER` | Vector map tile container | Permitted (Unsigned) |
| **RQ-003** | `.precomp` | CombiStyles Graphic | 10-byte header | Gauge and UI skin bitmaps | Permitted (Unsigned) |
| **RQ-004** | `.xar` | MapStyles Archive | `rax\0` (`0x72617800`) | Map styling asset archive | Permitted (Unsigned) |
| **RQ-005** | `.ifs` | QNX 6 Image Filesystem | `0x00ff7eeb` | System bootloader & OS kernel | **LOCKED (Signed)** |
| **RQ-006** | `.efs` | QNX 6 Embedded Filesystem | `QSSL_F3S` | Writable flash filesystem | Permitted (Unsigned) |
| **RQ-007** | `.ans` | Speech Models | `ANS\0` (`0x414E5300`) | Acoustic speech models | Permitted (Unsigned) |
| **RQ-008** | `.hbbin` | System FPGA Bitstream | `.HDG`/`.HDH`/`.HGD` | Video switcher & MOST bridge | **LOCKED (Signed)** |
| **RQ-009** | `.ipf` | SMSC MOST INIC Firmware | 16-byte container | Optical bus transceiver | **LOCKED (Signed)** |
| **RQ-010** | `.gdb` | Geographic Routing DB | `0xDEADBEEF` | Routing & elevation vectors | Permitted (Unsigned) |
| **RQ-011** | `.hbgr` | Binary Speech Grammar | `0xFFFFFFFE` / `*** BINARYGRAMMAR` | Voice command state machine | Permitted (Unsigned) |
| **RQ-012** | `.ldr` | ADI Blackfin DSP Loader | `0xB8C6D3E2` | Audio equalization processor | **LOCKED (Signed)** |

---

## Detailed Specifications

### RQ-001: HBNavDB (`.db`)
- **Magic**: `FLDB` (ASCII, 4 bytes).
- **Header Size**: 36 bytes.
- **Page Stride**: 544 bytes (512-byte payload + 32-byte header/checksum block).
- **Structure**: Multi-level B-tree with root page index located at offset `0x24`.
- **Handling**: Decoded by `mmi_formats::HBNavDBAdapter`.

### RQ-002: Orion ATLAS (`.atlas`)
- **Signature**: `\x06HEADER` (ASCII length-prefixed string).
- **Payload**: Spatial indexing tiles containing vector geometry for road networks.
- **Handling**: Decoded by `mmi_formats::OrionAtlasAdapter`.

### RQ-003: CombiStyles Graphic (`.precomp`)
- **Header**: 10 bytes:
  - `u16`: Pixel width
  - `u16`: Pixel height
  - `u16`: Format code (`0x0004` = RGBA8888, `0x0002` = RGB565)
  - `u32`: Uncompressed payload size
- **Payload**: zlib-compressed raw pixel stream.
- **Handling**: Decoded by `mmi_assets::PrecompDecoder` and `mmi_formats::PrecompAdapter`.

### RQ-004: MapStyles Archive (`.xar`)
- **Signature**: `rax\0` (`0x72 0x61 0x78 0x00`).
- **Structure**: Archive container holding indexed XML style sheets and raster texture maps for daytime and nighttime 3D rendering.
- **Handling**: Decoded by `mmi_formats::MapStylesAdapter`.

### RQ-005: QNX 6 Image Filesystem (`.ifs`)
- **Startup Magic**: `0x00ff7eeb` (Little Endian).
- **Structure**: Standalone bootable image containing startup binary, QNX Neutrino kernel (`procnto`), and drivers.
- **Safety Policy**: **ANALYSIS-ONLY**. Signed by factory boot keys. Modifying triggers `ERR_SIGNED_ARTEFACT_IMMUTABLE`.

### RQ-006: QNX 6 Embedded Filesystem (`.efs` / F3S)
- **Superblock Magic**: `QSSL_F3S`.
- **Structure**: Flash filesystem with wear-leveling headers and directory extents.
- **Handling**: Unpacked and inspected via `mmi_formats::QnxEfsAdapter`.

### RQ-007: Speech Prompts (`.ans`)
- **Signature**: `ANS\0` (`0x41 0x4E 0x53 0x00`).
- **Structure**: Contains acoustic phoneme dictionaries and localized navigation voice guidance prompts.
- **Handling**: Decoded by `mmi_formats::SpeechAnsAdapter`.

### RQ-008: System FPGA Bitstream (`.hbbin`)
- **Structure**: Chunk-based container (`.HDG`, `.HDH`, `.HGD`, `.HGU`) wrapping Xilinx FPGA bitstreams (Sync word: `0x5599AA66`).
- **Safety Policy**: **ANALYSIS-ONLY**. Controls hardware bus timing; locked from unauthorized rebuilding.

### RQ-009: SMSC MOST INIC Firmware (`.ipf`)
- **Structure**: Microcode payload for SMSC OS81050 / OS81110 Intelligent Network Interface Controllers.
- **Safety Policy**: **ANALYSIS-ONLY**. Flashable only via MOST optical transceiver; locked from modification.

### RQ-010: Geographic Routing (`.gdb`)
- **Magic**: `0xDEADBEEF`.
- **Structure**: Version 37 routing database containing turn-by-turn routing topologies, speed limits, and elevation matrices.
- **Handling**: Decoded by `mmi_formats::GeographicGdbAdapter`.

### RQ-011: Binary Grammar (`.hbgr`)
- **Signature**: `0xFFFFFFFE` followed by banner text `*** BINARYGRAMMAR`.
- **Structure**: Compiled state machines for voice-recognition phrase matching.
- **Handling**: Decoded by `mmi_formats::BinaryGrammarAdapter`.

### RQ-012: ADI Blackfin DSP Loader (`.ldr`)
- **Magic**: `0xB8C6D3E2` (Target processor byte `0x81` = ADSP-BF53x).
- **Structure**: Executable loader stream for Analog Devices Blackfin digital signal processors handling audio filtering and surround sound.
- **Safety Policy**: **ANALYSIS-ONLY**. Hardware-critical signal chain; locked from rebuilding.
