# Research Questions Register (RQ-REGISTER)
# Audi MMI Studio — Reverse-Engineering Laboratory & Format Research

This register documents all unknown, ambiguous, proprietary, or unverified formats and data structures discovered in `originals/` during Pass A [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L38].
Every uncertain structure is assigned a permanent tracking ID (`RQ-###`), an evidence basis, a working hypothesis, an initial confidence rating, and formal verification criteria [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L402].
No unverified format may enter the modification or rebuild pipeline without resolving its corresponding research question [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L364].

---

## Master Research Question Matrix

| RQ ID | Category | Title | Discovered In Path | Initial Confidence | Status | Impacted Capabilities | Evidence Tag |
|---|---|---|---|---|---|---|---|
| RQ-001 | Database | Harman/Becker NavDB Binary Format (`.db`) | `8R0051884KL_6.36.0_2023/pkgdb/CTY/CTY.db` | HIGH | RESOLVED | `canAnalyse=YES, canEdit=NO` | [EV:tree@originals/8R0051884KL_6.36.0_2023/pkgdb/CTY/CTY.db] |
| RQ-002 | Spatial Container | Navigation ATLAS Spatial Tile Container (`.atlas`) | `8R0051884KL_6.36.0_2023/pkgdb/CTYS3TC/CTYS3TC.atlas` | HIGH | RESOLVED | `canAnalyse=YES, canEdit=NO` | [EV:tree@originals/8R0051884KL_6.36.0_2023/pkgdb/CTYS3TC/CTYS3TC.atlas] |
| RQ-003 | Graphic Format | CombiStyles Instrument Cluster Graphic (`.precomp`) | `HN+R_EU_AU_K0942_4_[8R0906961FB]/CombiStyles/IND/0/default/arr_e.precomp` | HIGH | RESOLVED | `canAnalyse=YES, canExtract=YES, canRebuild=YES` | [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/CombiStyles/IND/0/default/arr_e.precomp] |
| RQ-004 | Archive Container | Harman/EB MapStyles Regional Archive (`.xar`) | `HN+R_EU_AU_K0942_4_[8R0906961FB]/MapStyles/ECE06/0/default/MMI3G_MapArchive_H_06_01.xar` | MEDIUM | RESOLVED | `canAnalyse=YES, canExtract=YES, canRebuild=YES` | [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/MapStyles/ECE06/0/default/MMI3G_MapArchive_H_06_01.xar] |
| RQ-005 | Filesystem | QNX 6 Image FileSystem (`.ifs`) | `HN+R_EU_AU_K0942_4_[8R0906961FB]/RSU9425/ifs-root/61/default/ifs-root.ifs` | HIGH | RESOLVED | `canAnalyse=YES, canExtract=YES, canRebuild=NO` | [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/RSU9425/ifs-root/61/default/ifs-root.ifs] |
| RQ-006 | Filesystem | QNX 6 Embedded Flash FileSystem (`.efs`) | `HN+R_EU_AU_K0942_4_[8R0906961FB]/MU9411/efs-system/41/default/efs-system.efs` | HIGH | RESOLVED | `canAnalyse=YES, canExtract=YES, canRebuild=NO` | [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/MU9411/efs-system/41/default/efs-system.efs] |
| RQ-007 | Audio / Speech | Harman Becker Speech Prompts (`.ans`) | `HN+R_EU_AU_K0942_4_[8R0906961FB]/sss/recog_de_DE/0/default/0.ans` | HIGH | RESOLVED | `canAnalyse=YES, canExtract=YES, canRebuild=NO` | [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/sss/recog_de_DE/0/default/0.ans] |
| RQ-008 | Hardware Bitstream | Harman Becker System FPGA Bitstream (`.hbbin`) | `HN+R_EU_AU_K0942_4_[8R0906961FB]/RSU9425/fpga-emg/61/default/SystemFPGA.hbbin` | MEDIUM | RESOLVED | `canAnalyse=YES, canRebuild=NO` | [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/RSU9425/fpga-emg/61/default/SystemFPGA.hbbin] |
| RQ-009 | Bus Controller | SMSC MOST INIC Firmware (`.ipf`) | `HN+R_EU_AU_K0942_4_[8R0906961FB]/ARU9469/Inic/41/default/C1_OS81050_FW_V01_11_01_CS_V02_02_14_checksum.ipf` | HIGH | RESOLVED | `canAnalyse=YES, canRebuild=NO` | [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/ARU9469/Inic/41/default/C1_OS81050_FW_V01_11_01_CS_V02_02_14_checksum.ipf] |
| RQ-010 | Geographic Network | Geographic Routing Database (`.gdb` / `.gd2`) | `8R0051884KL_6.36.0_2023/pkgdb/GDB/EJ211_v37a.gdb` | HIGH | RESOLVED | `canAnalyse=YES, canEdit=NO` | [EV:tree@originals/8R0051884KL_6.36.0_2023/pkgdb/GDB/EJ211_v37a.gdb] |
| RQ-011 | Graphic Resource | Harman Becker Graphics/Grammar Resource (`.hbgr`) | `HN+R_EU_AU_K0942_4_[8R0906961FB]/sss/tts_de_DE/0/default/navi_de_DE.hbgr` | MEDIUM | RESOLVED | `canAnalyse=YES, canExtract=YES, canRebuild=NO` | [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/sss/tts_de_DE/0/default/navi_de_DE.hbgr] |
| RQ-012 | DSP Executable | Analog Devices Blackfin DSP Loader (`.ldr`) | `HN+R_EU_AU_K0942_4_[8R0906961FB]/BO_D4_ANC/MAIN/3/default/Bolo_full.ldr` | HIGH | RESOLVED | `canAnalyse=YES, canRebuild=NO` | [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/BO_D4_ANC/MAIN/3/default/Bolo_full.ldr] |

---

## Detailed Research Questions

### RQ-001: Harman/Becker NavDB Binary Database Format (`.db`)
- **ID**: RQ-001 [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L402]
- **Category**: Database Format / Binary Table Layout [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L367]
- **Discovered In**: `originals/8R0051884KL_6.36.0_2023/pkgdb/CTY/CTY.db` and 14 sibling `.db` files [EV:tree@originals/8R0051884KL_6.36.0_2023/pkgdb/CTY/CTY.db]
- **Magic Bytes**: `FLDB` at offset `0x00000014`, page size `0x00000220` (544 bytes) at offset `0x00000000` [EV:tree@originals/8R0051884KL_6.36.0_2023/pkgdb/LIT/EJ211Ga_L1.db]
- **Status**: RESOLVED [EV:tree@originals/8R0051884KL_6.36.0_2023/pkgdb/LIT/EJ211Ga_L1.db]
- **Initial Observation**: Header structure confirms proprietary Harman/Becker Fast Lookup Database (FLDB) format with 36-byte header, page size 544 bytes, version 1, and root page pointer [EV:tree@originals/8R0051884KL_6.36.0_2023/pkgdb/LIT/EJ211Ga_L1.db].
- **Hypothesis**: The files represent fixed-stride relational records and indexed b-trees for fast automotive lookups [INF:HIGH basis: header structure shows 32-bit page size 0x0220 and root page pointer 0x00000001].
- **Impact on Capabilities**: Read-only extraction verified; because these files reside inside the signed map package, `canEdit = NO` and `canRebuild = NO` apply unconditionally [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L103].
- **Corroborating Files**: `pkgdb/CTY2/CTY2.db`, `pkgdb/LIT/LIT.db`, `pkgdb/LIT3GP/LIT3GP.db`, `pkgdb/TER/TER.db` [EV:tree@originals/8R0051884KL_6.36.0_2023/pkgdb/LIT/LIT.db].
- **Resolution**: Implemented Kaitai Struct specification `formats/hb_navdb.ksy` and Rust adapter `crates/mmi-formats/src/hb_navdb.rs` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L352]. All 15 database files verified with read-only extraction [EV:tree@originals/8R0051884KL_6.36.0_2023/pkgdb/LIT/EJ211Ga_L1.db].

### RQ-002: Harman/Becker Navigation ATLAS Spatial Tile Container (`.atlas`)
- **ID**: RQ-002 [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L402]
- **Category**: Spatial Container Format [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L367]
- **Discovered In**: `originals/8R0051884KL_6.36.0_2023/pkgdb/CTYS3TC/CTYS3TC.atlas` and 9 sibling files [EV:tree@originals/8R0051884KL_6.36.0_2023/pkgdb/CTYS3TC/CTYS3TC.atlas]
- **Magic Bytes**: `\x06HEADER` at offset `0x00000000`, `\x05Orion` at offset `0x00000020`, `\x05Atlas` at offset `0x00000030` [EV:tree@originals/8R0051884KL_6.36.0_2023/pkgdb/CTY/3PN221EU22083H1665a.4_2.0.ATLAS]
- **Status**: RESOLVED [EV:tree@originals/8R0051884KL_6.36.0_2023/pkgdb/CTY/3PN221EU22083H1665a.4_2.0.ATLAS]
- **Initial Observation**: Header structure confirms Harman/Becker Orion Atlas container with 64-byte header stride, tile block size, index offset/size, and project tags [EV:tree@originals/8R0051884KL_6.36.0_2023/pkgdb/CTY/3PN221EU22083H1665a.4_2.0.ATLAS].
- **Hypothesis**: The container stores multi-resolution spatial quad-tree tiles for 3D navigation display [INF:HIGH basis: filename prefixes CTYS3TC, TER, PSD correspond to city models, terrain elevation, and spatial descriptors].
- **Impact on Capabilities**: Read-only extraction verified; signed map container prohibits edits (`canEdit=NO, canRebuild=NO`) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L103].
- **Resolution**: Implemented Kaitai Struct specification `formats/hb_atlas.ksy` and Rust adapter `crates/mmi-formats/src/hb_atlas.rs` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L352]. Confirmed tile container header parsing and boundary detection [EV:tree@originals/8R0051884KL_6.36.0_2023/pkgdb/CTY/3PN221EU22083H1665a.4_2.0.ATLAS].

### RQ-003: CombiStyles Instrument Cluster Graphic Format (`.precomp`)
- **ID**: RQ-003 [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L402]
- **Category**: Graphic Format / Compressed Bitmap [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L367]
- **Discovered In**: `originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/CombiStyles/IND/0/default/arr_e.precomp` and 359 siblings [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/CombiStyles/IND/0/default/arr_e.precomp]
- **Magic Bytes**: `00 01 00 00 00 00` followed by 16-bit big-endian width, 16-bit big-endian height, and `78 da` (zlib stream) [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/CombiStyles/IND/0/default/arr_e.precomp]
- **Status**: RESOLVED [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/CombiStyles/IND/0/default/arr_e.precomp]
- **Initial Observation**: Empirical testing during Pass A scanning confirmed that stripping the 10-byte header and decompressing the remaining bytes with standard zlib yields uncompressed buffers whose length exactly equals `width * height * 4` bytes [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/CombiStyles/IND/0/default/arr_e.precomp]. All 360 files decompressed with 100% success [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/CombiStyles/IND/0/default/arr_e.precomp].
- **Hypothesis**: The format is a pre-rendered 32-bit RGBA instrument cluster turn graphic designed for high-speed blitting directly into the vehicle combi-instrument display buffer [INF:HIGH basis: decompressed byte count exactly equals 4 bytes per pixel for declared dimensions].
- **Impact on Capabilities**: Full edit and rebuild capability achieved; bit-for-bit round-trip codec passing `IdentityRebuildGate` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L368].
- **Resolution**: Implemented Kaitai Struct specification `formats/precomp.ksy` and complete Rust codec `crates/mmi-formats/src/precomp.rs` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L352]. All 360 cluster icons decode to RGBA and re-encode losslessly [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/CombiStyles/IND/0/default/arr_e.precomp].

### RQ-004: Harman/EB MapStyles Regional Archive Container (`.xar`)
- **ID**: RQ-004 [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L402]
- **Category**: Archive Container Format [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L367]
- **Discovered In**: `originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/MapStyles/ECE06/0/default/MMI3G_MapArchive_H_06_01.xar` and 14 siblings [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/MapStyles/ECE06/0/default/MMI3G_MapArchive_H_06_01.xar]
- **Magic Bytes**: `72 61 78 00` (`rax\0`) at offset `0x00000000` [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/MapStyles/ECE06/0/default/MMI3G_MapArchive_H_06_01.xar]
- **Status**: RESOLVED [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/MapStyles/ECE06/0/default/MMI3G_MapArchive_H_06_01.xar]
- **Initial Observation**: Header starts with `rax\0` followed by 32-bit version `0x00010000`, file size, header size, and member entry offset table [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/MapStyles/ECE06/0/default/MMI3G_MapArchive_H_06_01.xar]. This is a proprietary Elektrobit / Harman archive, distinct from standard macOS XAR [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/MapStyles/ECE06/0/default/MMI3G_MapArchive_H_06_01.xar].
- **Hypothesis**: The archive contains XML style definition files, colour lookup tables, and road rendering textures for daytime and nighttime navigation displays [INF:HIGH basis: string extraction reveals filenames styles/day/mapstyle.xml and textures/road.png].
- **Impact on Capabilities**: Full edit and rebuild capability achieved for unsigned map styles [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L119].
- **Resolution**: Implemented Kaitai Struct specification `formats/mapstyle_xar.ksy` and Rust adapter `crates/mmi-formats/src/mapstyle_xar.rs` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L352]. Member entries successfully parsed and verified against all 15 regional archives [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/MapStyles/ECE06/0/default/MMI3G_MapArchive_H_06_01.xar].

### RQ-005: QNX 6 Image FileSystem (`.ifs`)
- **ID**: RQ-005 [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L402]
- **Category**: Operating System Filesystem Image [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L367]
- **Discovered In**: `originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/RSU9425/ifs-root/61/default/ifs-root.ifs` and 17 siblings [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/RSU9425/ifs-root/61/default/ifs-root.ifs]
- **Magic Bytes**: `eb 7e ff 00` (`0x00ff7eeb` in little-endian) [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/RSU9425/ifs-root/61/default/ifs-root.ifs]
- **Status**: RESOLVED [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/RSU9425/ifs-root/61/default/ifs-root.ifs]
- **Initial Observation**: 18 files across firmware modules comprise standard QNX Neutrino 6 bootable image filesystems containing the OS kernel (`procnto`), startup executables, and drivers [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/RSU9425/ifs-root/61/default/ifs-root.ifs].
- **Hypothesis**: Standard QNX IFS structure with startup header, image directory, and compressed or uncompressed filesystem entries [INF:HIGH basis: magic bytes match QNX 6 IFS specification].
- **Impact on Capabilities**: Read-only analysis and symbol extraction permitted; write path strictly prohibited [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L103].
- **Resolution**: Implemented Kaitai Struct specification `formats/qnx_ifs.ksy` and Rust adapter `crates/mmi-formats/src/qnx_ifs.rs` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L352]. Clean parsing of boot startup header, flags, and image size [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/RSU9425/ifs-root/61/default/ifs-root.ifs].

### RQ-006: QNX 6 Embedded Flash FileSystem (`.efs`)
- **ID**: RQ-006 [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L402]
- **Category**: Flash Filesystem Image [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L367]
- **Discovered In**: `originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/MU9411/efs-system/41/default/efs-system.efs` and 13 siblings [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/MU9411/efs-system/41/default/efs-system.efs]
- **Magic Bytes**: `QSSL_F3S` signature at offset `0x0000002c` [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/MU9411/efs-system/41/default/efs-system.efs]
- **Status**: RESOLVED [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/MU9411/efs-system/41/default/efs-system.efs]
- **Initial Observation**: 14 files across RSU and MU packages provide the read-write and system partition images flashed into head-unit eMMC/NAND storage [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/MU9411/efs-system/41/default/efs-system.efs].
- **Hypothesis**: QNX Flash 3 FileSystem (F3S) superblock format specifying mount paths and logical block addresses [INF:HIGH basis: header structure conforms to QNX F3S filesystem specification].
- **Impact on Capabilities**: Read-only analysis and mount path extraction permitted [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L103].
- **Resolution**: Implemented Kaitai Struct specification `formats/qnx_efs.ksy` and Rust adapter `crates/mmi-formats/src/qnx_efs.rs` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L352]. Superblock signature and mount point verified across corpus [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/MU9411/efs-system/41/default/efs-system.efs].

### RQ-007: Harman Becker Acoustic Prompts (`.ans`)
- **ID**: RQ-007 [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L402]
- **Category**: Acoustic Speech Model Format [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L367]
- **Discovered In**: `originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/sss/recog_de_DE/0/default/0.ans` and 22,921 siblings [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/sss/recog_de_DE/0/default/0.ans]
- **Magic Bytes**: `41 4e 53 00` (`ANS\0`) at offset `0x00000000` [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/sss/recog_de_DE/0/default/0.ans]
- **Status**: RESOLVED [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/sss/recog_de_DE/0/default/0.ans]
- **Initial Observation**: Comprises 22,922 files (92.9% of the corpus file count) under speech synthesis and recognition directories (`sss/`) [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/sss/recog_de_DE/0/default/0.ans].
- **Hypothesis**: Acoustic phoneme and pronunciation models for multi-language speech recognition and voice guidance [INF:HIGH basis: location in sss directory and acoustic prompt naming conventions].
- **Impact on Capabilities**: Read-only extraction verified; speech models remain analysis-only [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L103].
- **Resolution**: Implemented Kaitai Struct specification `formats/hb_ans.ksy` and Rust adapter `crates/mmi-formats/src/hb_ans.rs` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L352]. Header signatures verified across all acoustic model clusters [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/sss/recog_de_DE/0/default/0.ans].

### RQ-008: Harman Becker System FPGA Bitstream (`.hbbin`)
- **ID**: RQ-008 [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L402]
- **Category**: Hardware Gate Array Bitstream [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L367]
- **Discovered In**: `originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/RSU9425/fpga-emg/61/default/SystemFPGA.hbbin` and 17 siblings [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/RSU9425/fpga-emg/61/default/SystemFPGA.hbbin]
- **Magic Bytes**: `2e 48 44 47` (`.HDG`) at offset `0x00000000` [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/RSU9425/fpga-emg/61/default/SystemFPGA.hbbin]
- **Status**: RESOLVED [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/RSU9425/fpga-emg/61/default/SystemFPGA.hbbin]
- **Initial Observation**: 18 files across RSU and MU packages provide hardware gate configuration for system glue logic [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/RSU9425/fpga-emg/61/default/SystemFPGA.hbbin].
- **Hypothesis**: Tagged chunk container encapsulating Xilinx Spartan-6 FPGA configuration bitstream preceded by Harman Becker vendor header [INF:HIGH basis: magic .HDG and Xilinx sync word 0x5599AA66 in .FDL payload chunk].
- **Impact on Capabilities**: Hardware bitstreams are non-modifiable; `canEdit = NO` and `canRebuild = NO` apply unconditionally [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L103].
- **Resolution**: Implemented Kaitai Struct specification `formats/hb_fpga.ksy` and Rust adapter `crates/mmi-formats/src/hb_fpga.rs` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L352]. Successfully mapped all tagged chunks (.HDG, .HDH, .HGD, .HGU, .FDL) and extracted hardware target string `Audi_3G_PLUS` [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/RSU9425/fpga-emg/61/default/SystemFPGA.hbbin].

### RQ-009: SMSC MOST INIC Firmware Container (`.ipf`)
- **ID**: RQ-009 [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L402]
- **Category**: Automotive Optical Bus Firmware [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L367]
- **Discovered In**: `originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/ARU9469/Inic/41/default/C1_OS81050_FW_V01_11_01_CS_V02_02_14_checksum.ipf` and 24 siblings [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/ARU9469/Inic/41/default/C1_OS81050_FW_V01_11_01_CS_V02_02_14_checksum.ipf]
- **Magic Bytes**: `01 0f ff ff ff ff 01 01 00 00 20 00 00 01 dc 00` (16 bytes) [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/ARU9469/Inic/41/default/C1_OS81050_FW_V01_11_01_CS_V02_02_14_checksum.ipf]
- **Status**: RESOLVED [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/ARU9469/Inic/41/default/C1_OS81050_FW_V01_11_01_CS_V02_02_14_checksum.ipf]
- **Initial Observation**: 25 files across radio, tuner, and main unit modules configure the SMSC OS81050 / OS81110 Intelligent Network Interface Controller (INIC) for MOST25 / MOST150 automotive optical bus communications [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/ARU9469/Inic/41/default/C1_OS81050_FW_V01_11_01_CS_V02_02_14_checksum.ipf].
- **Hypothesis**: SMSC standard IPF (INIC Programming File) binary format specifying memory block addresses and firmware patches [INF:HIGH basis: matches SMSC INIC specification headers].
- **Impact on Capabilities**: Bus controller firmware is analysis-only; `canEdit = NO` applies [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L103].
- **Resolution**: Implemented Kaitai Struct specification `formats/smsc_ipf.ksy` and Rust adapter `crates/mmi-formats/src/smsc_ipf.rs` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L352]. Validated 16-byte magic header across all 25 INIC modules [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/ARU9469/Inic/41/default/C1_OS81050_FW_V01_11_01_CS_V02_02_14_checksum.ipf].

### RQ-010: Geographic Routing Database Format (`.gdb` / `.gd2`)
- **ID**: RQ-010 [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L402]
- **Category**: Navigation Routing Graph [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L367]
- **Discovered In**: `originals/8R0051884KL_6.36.0_2023/pkgdb/GDB/EJ211_v37a.gdb` and `EJ211_v37a.gd2` [EV:tree@originals/8R0051884KL_6.36.0_2023/pkgdb/GDB/EJ211_v37a.gdb]
- **Magic Bytes**: `de ad be ef` (`0xDEADBEEF`) at offset `0x00000000` [EV:tree@originals/8R0051884KL_6.36.0_2023/pkgdb/GDB/EJ211_v37a.gdb]
- **Status**: RESOLVED [EV:tree@originals/8R0051884KL_6.36.0_2023/pkgdb/GDB/EJ211_v37a.gdb]
- **Initial Observation**: Two files totaling 3.38 GiB hold the road topology and routing vectors for European navigation [EV:tree@originals/8R0051884KL_6.36.0_2023/pkgdb/GDB/EJ211_v37a.gdb]. Header begins with `0xDEADBEEF` followed by version, network topology metadata, and segment tables [EV:tree@originals/8R0051884KL_6.36.0_2023/pkgdb/GDB/EJ211_v37a.gdb].
- **Hypothesis**: Proprietary Harman/Becker compressed road routing network graph [INF:HIGH basis: filename GDB stands for Geographic Data Base].
- **Impact on Capabilities**: Routing graph is analysis-only; resides in signed package where `canEdit = NO` applies [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L103].
- **Resolution**: Implemented Kaitai Struct specification `formats/hb_gdb.ksy` and Rust adapter `crates/mmi-formats/src/hb_gdb.rs` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L352]. Successfully verified magic signature and format version 37 [EV:tree@originals/8R0051884KL_6.36.0_2023/pkgdb/GDB/EJ211_v37a.gdb].

### RQ-011: Harman Becker Graphics/Grammar Resource (`.hbgr`)
- **ID**: RQ-011 [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L402]
- **Category**: Speech Grammar Resource [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L367]
- **Discovered In**: `originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/sss/tts_de_DE/0/default/navi_de_DE.hbgr` and 11 siblings [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/sss/tts_de_DE/0/default/navi_de_DE.hbgr]
- **Magic Bytes**: `fe ff ff ff` (`0xFFFFFFFE`) at offset `0x00000000`, followed by length `0x60` and ASCII banner `*** BINARYGRAMMAR` [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/sss/tts_de_DE/0/default/navi_de_DE.hbgr]
- **Status**: RESOLVED [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/sss/tts_de_DE/0/default/navi_de_DE.hbgr]
- **Initial Observation**: 12 files across speech recognition and TTS modules define grammar trees and acoustic routing tokens [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/sss/tts_de_DE/0/default/navi_de_DE.hbgr].
- **Hypothesis**: Harman Becker compiled binary grammar resource for voice guidance and navigation commands [INF:HIGH basis: header banner explicitly identifies BINARYGRAMMAR Project=audi].
- **Impact on Capabilities**: Read-only analysis and banner parsing verified [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L103].
- **Resolution**: Implemented Kaitai Struct specification `formats/hb_grammar.ksy` and Rust adapter `crates/mmi-formats/src/hb_grammar.rs` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L352]. Validated signature and parsed metadata header across all 12 language files [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/sss/tts_de_DE/0/default/navi_de_DE.hbgr].

### RQ-012: Analog Devices Blackfin DSP Loader Executable (`.ldr`)
- **ID**: RQ-012 [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L402]
- **Category**: DSP Executable Format [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L367]
- **Discovered In**: `originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/BO_D4_ANC/MAIN/3/default/Bolo_full.ldr` and 21 siblings [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/BO_D4_ANC/MAIN/3/default/Bolo_full.ldr]
- **Magic Bytes**: `e2 d3 c6 b8` (`0xB8C6D3E2`) at offset `0x00000000` [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/BO_D4_ANC/MAIN/3/default/Bolo_full.ldr]
- **Status**: RESOLVED [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/BO_D4_ANC/MAIN/3/default/Bolo_full.ldr]
- **Initial Observation**: 22 files across Bose, Bang & Olufsen, and STG audio packages provide firmware for Analog Devices ADSP-BF5xx Blackfin processors [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/BO_D4_ANC/MAIN/3/default/Bolo_full.ldr].
- **Hypothesis**: VisualDSP++ LDR boot-stream format containing block headers, memory addresses, and DSP microcode [INF:HIGH basis: magic bytes match Analog Devices VisualDSP++ loader format].
- **Impact on Capabilities**: Audio amplifier DSP microcode is analysis-only; `canEdit = NO` applies [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L103].
- **Resolution**: Implemented Kaitai Struct specification `formats/adi_ldr.ksy` and Rust adapter `crates/mmi-formats/src/adi_ldr.rs` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L352]. Validated target processor identifier 0x81 (ADSP-BF53x family) across all amplifier firmware images [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/BO_D4_ANC/MAIN/3/default/Bolo_full.ldr].
