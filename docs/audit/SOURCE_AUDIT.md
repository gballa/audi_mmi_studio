# Source Audit: Audi MMI Evidence Corpus (Pass A)
# Working Directory: /Users/gerald/Antigravity/AudiMMI
# Timestamp: 2026-09-18T12:00:00Z
# Target Corpus: originals/ (Immutable Evidence Repository)

---

## 1. Repository Status (§0.0 Greenfield Verification)

The repository is confirmed to be in a 100% greenfield starting state (§0.0), containing zero legacy application source code under `apps/`, `crates/`, `packages/`, `docs/`, or `tests/`, with `originals/` serving as the sole immutable evidence repository [EV:cmd@ls-la].
Initial inspection of the repository root confirms the absence of any initialized `.git` tracking database and confirms that the workspace consists solely of the evidence repository, coordination prompt files, and agent working metadata [EV:cmd@git-status].

### Initial Root Manifest

| File / Directory | Type | Size | Status | Evidence Tag |
|---|---|---|---|---|
| `originals/` | Directory | 63.97 GiB (24,661 files) | Immutable reference corpus | [EV:tree@originals] |
| `AUDI_MMI_STUDIO_AGENT_PROMPT.md` | Document | 56,401 bytes | Authoritative specification | [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md] |
| `ORIGINAL_REQUEST.md` | Document | 3,516 bytes | Project requirements baseline | [EV:doc:ORIGINAL_REQUEST.md] |
| `originals_tree.txt` | Survey Cache | 1,195,006 bytes | Initial file hierarchy dump | [EV:fs@originals_tree.txt] |
| `.agents/` | Directory | Metadata | Multi-agent coordination state | [EV:fs@.agents] |
| `.DS_Store` | OS Metadata | 6,148 bytes | Operating system artifact | [EV:fs@.DS_Store] |

---

## 2. Discovered Domains (§4, §7)

The evidence repository `originals/` contains 24,661 files across 1,844 directories aggregating 68,687,999,654 bytes (63.97 GiB), organized into five symmetric pairs of container archives and their extracted directory counterparts [EV:survey@scan_originals.py].

### Discovered Package Inventory

| Package Path | Domain Category | Format | Total Files | Total Size | Primary Classification | Evidence Tag |
|---|---|---|---|---|---|---|
| `HN+R_EU_AU_K0942_4_[8R0906961FB]/` | `HN+R_SOFTWARE` | Directory | 20,554 | 1,453,652,416 B | Firmware Train (MMI 3G+ High) | [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]] |
| `HN+R_EU_AU_K0942_4_[8R0906961FB].zip` | `HN+R_SOFTWARE` | ZIP Archive | 1 | 996,605,909 B | Software Update Container | [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB].zip] |
| `8R0051884KL_6.36.0_2023/` | `MAP_PACKAGE` | Directory | 77 | 40,693,125,670 B | Navigation Map Database (ECE 2023) | [EV:tree@originals/8R0051884KL_6.36.0_2023] |
| `8R0051884KL_6.36.0_2023.7z` | `MAP_PACKAGE` | 7z Archive | 1 | 24,681,684,754 B | Navigation Package Container | [EV:tree@originals/8R0051884KL_6.36.0_2023.7z] |
| `Software/` | `HNAV_SOFTWARE` | Directory | 4,001 | 496,716,440 B | Supporting Train (HNav US K0133) | [EV:tree@originals/Software] |
| `Software.zip` | `HNAV_SOFTWARE` | ZIP Archive | 1 | 361,538,824 B | Supporting Train Container | [EV:tree@originals/Software.zip] |
| `License/` | `LICENSE_ACTIVATION` | Directory | 22 | 3,059,468 B | Script & Utility Harness | [EV:tree@originals/License] |
| `License.zip` | `LICENSE_ACTIVATION` | ZIP Archive | 1 | 1,608,166 B | License Container Archive | [EV:tree@originals/License.zip] |
| `6.22.4 Vlasoff maps activation/` | `MAP_ACTIVATION` | Directory | 2 | 3,861 B | Activation Payload Candidate | [EV:tree@originals/6.22.4 Vlasoff maps activation] |
| `6.22.4 Vlasoff maps activation.7z` | `MAP_ACTIVATION` | 7z Archive | 1 | 4,146 B | Activation Container Archive | [EV:tree@originals/6.22.4 Vlasoff maps activation.7z] |

### Domain Technical Profiles

1. **MMI 3G+ High Navigation European Firmware Train (`HN+R_EU_AU_K0942_4_[8R0906961FB]`)**:
   - Manifest header confirms release identifier `HN+R_EU_AU_K0942_4`, vendor `HBAS` (Harman Becker Automotive Systems), targeting variants `9425`, `9406`, `9407`, `9408`, `9409`, `9410`, and `9411` across European and Rest-of-World regions [EV:file@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/metainfo2.txt:8-25].
   - Target vehicle platforms include Audi A6/A7 (C7), A8 (D4), and Q3/A1 with multi-hardware revisions `41`, `51`, and `61` [INF:HIGH basis: directory naming 41, 51, 61 within RSU9425 and MU9411].
   - Internal module taxonomy encompasses navigation layer graphics (`GEMMI`), emergency root filesystem (`RSU9425`), main unit application (`MU94xx`), radio and tuner firmware (`ARU93xx`/`ARU94xx`), display controllers (`DU902A`/`DUA017`), sound amplifiers (`Bose`), keyboard input controllers (`KBDREAR`), and regional map styles (`MapStyles`) [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]].

2. **Europe 2023 Navigation Database Package (`8R0051884KL_6.36.0_2023`)**:
   - Navigation database release metadata confirms part number `8R0060884KL`, application software version `3600`, system title `EUR 2023`, build version `6.36.0` [EV:file@originals/8R0051884KL_6.36.0_2023/DBInfo.txt:1-4].
   - Dual target profiles support both legacy MMI 3G (`common_Release_1`, variants `9307`, `9308`) and MMI 3G+ (`common_Release_2`, variants `9411`, `9408`, `9409`, `9410`, `9498`, `9499`, `9425`, `9436`) [EV:file@originals/8R0051884KL_6.36.0_2023/metainfo2.txt:31-50].
   - Internal structure comprises geographic tile containers (`ATLAS`), proprietary navigation databases (`CTY.db`, `LIT.db`, `TER.db`), routing databases (`GDB`), speech dialogue images (`SDS_Data.iso`), regional style archives (`StyleDB`), and cryptographically signed map update package manifests (`.pkg` + `.sig`) [EV:tree@originals/8R0051884KL_6.36.0_2023/pkgdb].

3. **MMI 3G High North American Software Train (`Software/`)**:
   - Manifest header confirms release identifier `HNav_US_K0133_3_D1`, vendor `HBAS`, region `USA`, targeting variants `9307` and `9308` [EV:file@originals/Software/metainfo2.txt:8-12].
   - Provides a valuable cross-train reference baseline for differential analysis against MMI 3G+ European packages [INF:HIGH basis: architectural comparison between HNav and HN+R trains].

4. **License and Script Utility Harness (`License/`)**:
   - Third-party QNX KornShell execution harness containing update launcher scripts, compiled utilities, and screen graphics [EV:tree@originals/License].

5. **Map Activation Payload (`6.22.4 Vlasoff maps activation`)**:
   - Third-party feature enablement code certificate and shell injection script [EV:tree@originals/6.22.4 Vlasoff maps activation].

---

## 3. § Scope Conflicts (§1.2, §1.3)

Section 1.2 of the project specification establishes non-negotiable hard boundaries prohibiting the design, specification, or implementation of any functionality whose purpose or primary effect is defeating cryptographic signature verification, code signing, DRM, authentication, licensing enforcement, feature-enablement codes (FSC), or vehicle immobilisers [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L65].
Corpus inspection revealed several third-party materials that conflict directly with §1.2 [EV:tree@originals/6.22.4 Vlasoff maps activation].
In accordance with §1.3, every activation or licensing item is cataloged, classified, and assessed for impact [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L88].

### Scope Conflict Classification Matrix

| Item Name | Path in `originals/` | Detected Nature | Classification | Prohibited Clause | Impacted Criteria | Recommendation | Evidence Tag |
|---|---|---|---|---|---|---|---|
| Vlasoff FSC Certificate | `6.22.4 Vlasoff maps activation/2380_00040009.fsc` | Binary FSC certificate with VIN `WAUZZZ4G8DN082977` and code `00040009` | `OUT OF SCOPE — DOCUMENT ONLY` | §1.2 Feature-enablement codes & licensing bypass | None | Exclude from all build and modification pipelines | [EV:hex@originals/6.22.4 Vlasoff maps activation/2380_00040009.fsc] |
| Vlasoff Injection Script | `6.22.4 Vlasoff maps activation/copie_scr.sh` | Encrypted QNX shell script for FSC injection | `OUT OF SCOPE — DOCUMENT ONLY` | §1.2 Activation controls & script injection | None | Exclude from all build and modification pipelines | [EV:cmd@file-vlasoff] |
| Vlasoff Container Archive | `6.22.4 Vlasoff maps activation.7z` | 7-Zip container packaging FSC bypass | `OUT OF SCOPE — DOCUMENT ONLY` | §1.2 Distribution of bypass materials | None | Inventory and hash only; no extraction | [EV:tree@originals/6.22.4 Vlasoff maps activation.7z] |
| License DRM Kill Script | `License/run.sh` | Shell script issuing `slay vdev-logvolmgr` to bypass map volume validation | `OUT OF SCOPE — DOCUMENT ONLY` | §1.2 DRM and licensing enforcement defeat | None | Exclude completely; do not implement or replicate | [EV:file@originals/License/run.sh:38-44] |
| License SD Auto-Run | `License/copie_scr.sh` | Auto-run launcher script invoking `run.sh` | `OUT OF SCOPE — DOCUMENT ONLY` | §1.2 DRM bypass launcher | None | Exclude from all build pipelines | [EV:tree@originals/License/copie_scr.sh] |
| License Decode Utility | `License/utils/DecodeScript` | Proprietary ELF binary for decrypting scripts | `OUT OF SCOPE — DOCUMENT ONLY` | §1.2 Authentication & access control bypass | None | Exclude from all toolchains | [EV:tree@originals/License/utils/DecodeScript] |
| QNX 6 Standard Binaries | `License/utils/` (`sqlite3`, `sed`, `awk`, `sysctl`, `pax`, `showScreen`, `libc.so*`) | Standard QNX Neutrino 6.5 SH-4 operating system utilities | `AMBIGUOUS — USER DECISION REQUIRED` | Dual-use toolchain (§1.2 vs benign diagnostic analysis) | None | Await user determination before Pass B | [EV:tree@originals/License/utils] |
| License Update Trigger | `License/upd` | Zero-byte trigger file causing execution | `AMBIGUOUS — USER DECISION REQUIRED` | Exploit trigger mechanism | None | Await user determination before Pass B | [EV:tree@originals/License/upd] |
| License Container Archive | `License.zip` | Archive container bundling scripts and utilities | `AMBIGUOUS — USER DECISION REQUIRED` | Mixed container packaging bypass scripts | None | Await user determination before Pass B | [EV:tree@originals/License.zip] |
| License Status Screens | `License/screens/*.png` (`scriptStart.png`, `scriptDone.png`, `error1.png`) | 800x480 pixel bitmap dialog images | `IN SCOPE` | N/A (Standard visual assets) | Display resolution verification | Include in Asset Census for screen canvas baseline | [EV:tree@originals/License/screens/scriptStart.png] |
| OEM FSC Catalog Manifest | `HN+R_.../AudiSupportedFscs/.../AudiFSC.txt` | OEM reference text manifest listing supported FSC IDs and RSA signatures | `IN SCOPE` | N/A (Genuine OEM firmware manifest) | Target profile compatibility verification | Include as reference for stock baseline checks | [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/AudiSupportedFscs/AudiSupportedFscs/0/default/AudiFSC.txt] |
| Supporting Train HNav | `Software/` and `Software.zip` | Genuine OEM MMI 3G High US firmware train | `IN SCOPE` | N/A (Genuine OEM firmware update) | Cross-train differential analysis | Include in differential analysis engine | [EV:tree@originals/Software] |

### Analysis of Impact on Acceptance Criteria

Exclusion of the activation and DRM circumvention items (`6.22.4 Vlasoff maps activation` and `License/` execution scripts) has ZERO negative impact on the core capabilities or acceptance criteria of Audi MMI Studio [INF:HIGH basis: workstation intent is reverse-engineering and visual asset modification, not map piracy].
Specifically:
- Core Reverse-Engineering Laboratory capabilities (hex viewing, entropy analysis, Kaitai Struct parsing, symbol extraction) operate entirely on genuine OEM firmware modules and navigation packages [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L377].
- UI Visualisation Studio (Asset Board, Screen Canvas, live font rendering, day/night simulation) relies on genuine graphics in `GEMMI` and Linotype font tables [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L470].
- AI Asset Authoring and Conformance Pipeline operates on individual bitmap assets, with an egress airlock that strictly denies transmission of activation-related materials [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L568].
- Rebuild Engine and Target Profile validation operate on genuine unsigned resource layers and stock profiles [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L695].

Excluding circumvention tools protects the software workstation from legal exposure and enforces the offline-first engineering boundary established in §1.1 and §1.2 [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L65].

### Mandatory User Decision Gate

Pass A execution concludes here regarding scope conflicts, awaiting formal user instructions on the items classified as `AMBIGUOUS — USER DECISION REQUIRED` prior to Pass B plan formulation [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L94].

---

## 4. § Signed Artefacts (§1.4)

Corpus inspection identified four payloads accompanied by adjacent detached cryptographic signatures (`.sig`) [EV:survey@scan_originals.py].
Each detached `.sig` file consists of exactly 128 bytes (1024-bit cryptographic signature computed via OEM private key) [EV:fs@sig_files].
In accordance with §1.4, all signed payloads are permanently designated **ANALYSIS-ONLY** [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L103].

### Signed Artefacts Ledger

| Payload File Path | Detached Signature Path | Signature Type | Payload SHA-256 | Payload BLAKE3 | canEdit | canRebuild | Status | Evidence Tag |
|---|---|---|---|---|---|---|---|---|
| `8R0051884KL_6.36.0_2023/pkgdb/MMI3GP_ECE_Hi_R_6_36_0.pkg` | `8R0051884KL_6.36.0_2023/pkgdb/MMI3GP_ECE_Hi_R_6_36_0.pkg.sig` | Detached RSA-1024 | `3c8fb2354c41870bb785055b89a64f526fa17c093a8e9e1c0702c89280d0d621` | `5c3451515efab9b9802058784d1bc4307ef02e071723fdbecdc0cf8f54316d3f` | NO | NO | ANALYSIS-ONLY | [EV:tree@originals/8R0051884KL_6.36.0_2023/pkgdb/MMI3GP_ECE_Hi_R_6_36_0.pkg] |
| `8R0051884KL_6.36.0_2023/pkgdb/MMI3G_ECE_Hi_R_6_36_0.pkg` | `8R0051884KL_6.36.0_2023/pkgdb/MMI3G_ECE_Hi_R_6_36_0.pkg.sig` | Detached RSA-1024 | `1f3f4c0228385002b8a07c91353278912e9b0151ba976378c66e921d72aa34e9` | `295a043c2c77d4037593c6f882194917d59b20e03503f1e967a53c306d8b9437` | NO | NO | ANALYSIS-ONLY | [EV:tree@originals/8R0051884KL_6.36.0_2023/pkgdb/MMI3G_ECE_Hi_R_6_36_0.pkg] |
| `8R0051884KL_6.36.0_2023/pkgdb/TMCConfig_16/TMCConfig.dat` | `8R0051884KL_6.36.0_2023/pkgdb/TMCConfig_16/TMCConfig.dat.sig` | Detached RSA-1024 | `12d671006b4657201dedc202604c47936823c70da3de60baaf3aa3b06d515f43` | `6256f0e495204432a677caeefd7dfd3fe32e09ff7b8849b2ba44bbd9a3bfa404` | NO | NO | ANALYSIS-ONLY | [EV:tree@originals/8R0051884KL_6.36.0_2023/pkgdb/TMCConfig_16/TMCConfig.dat] |
| `HN+R_EU_AU_K0942_4_[8R0906961FB]/TMCConfig/TMCConfig.dat` | `HN+R_EU_AU_K0942_4_[8R0906961FB]/TMCConfig/TMCConfig.dat.sig` | Detached RSA-1024 | `12d671006b4657201dedc202604c47936823c70da3de60baaf3aa3b06d515f43` | `6256f0e495204432a677caeefd7dfd3fe32e09ff7b8849b2ba44bbd9a3bfa404` | NO | NO | ANALYSIS-ONLY | [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/TMCConfig/TMCConfig.dat] |

### Binding Rules for Signed Payloads

1. **Strict Immutability**:
   - `canEdit = NO` and `canRebuild = NO` apply permanently across all signed payloads, regardless of how thoroughly their internal structure is decoded [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L103].
   - Any modification attempt by user recipe or CLI command triggers an immediate hard error: `ERR_SIGNED_ARTEFACT_IMMUTABLE` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L111].
2. **Analysis Capabilities Permitted**:
   - The application may parse, inspect, index, diff, carve, visualise, and generate reports on signed payload contents (`canAnalyse = YES`, `canExtract = YES`, `canNormalise = YES`, `canValidate = YES`) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L105].
3. **No Re-signing or Stubbing**:
   - The application must never re-sign, strip, stub, or bypass signatures, and must never emit a modified media image containing an altered signed payload [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L107].

### Definition of Realistic Customization Boundary

Because cryptographic signatures protect the core map package manifests and payloads, the realistic customization surface of Audi MMI packages is strictly confined to the **unsigned resource layer** [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L117]:
- GEMMI navigation layer visual assets (traffic icons, POI overlays, Google Earth brand marks, cursor graphics) [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/GEMMI/nav/0/default/models].
- Authentic typography and Linotype font tables (`AudiUnivers540Med.ttf`, `LT_Univers440_88perc_Arabic.ttf`) [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/GEMMI/nav/0/default/res].
- Regional MapStyle archives (`StyleDB.../*.xar` and `MapStyles/*/*.xar`) governing daytime and nighttime colour palettes and road line styles [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/MapStyles].
- Instrument cluster turn-by-turn indicator graphics (`CombiStyles/*/*.precomp`) [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/CombiStyles].
- Localisation string tables and language translation resources [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L58].
- Module hardware configuration files and update scripts (`.ini`, `.conf`) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L59].

---

## 5. § Asset Census (§10.1, R3)

During Pass A L4 sampling, an exhaustive census was performed over all visual and font assets across `originals/`, recording path, module, container, detected codec, dimensions, bit depth, colour type, alpha semantics, and decode status [EV:survey@scan_originals.py].
A total of **444 assets** were cataloged, comprising 80 PNG images, 360 `.precomp` instrument cluster graphics, 3 Linotype TrueType fonts, and 1 Collada 3D mesh model [EV:survey@scan_originals.py].

### 5.1 Screen Canvas Geometry Baseline

Inspection of all full-screen prompt dialogs in the corpus confirms that the native display resolution of the MMI 3G+ (HN+R) system is **800 x 480 pixels** [EV:tree@originals/License/screens/scriptStart.png]:

| Asset Path | Width | Height | Bit Depth | Colour Type | Role | Evidence Tag |
|---|---|---|---|---|---|---|
| `License/screens/scriptStart.png` | 800 | 480 | 8 | RGB (Truecolor) | Full-screen workflow prompt | [EV:tree@originals/License/screens/scriptStart.png] |
| `License/screens/scriptDone.png` | 800 | 480 | 8 | RGB (Truecolor) | Full-screen workflow prompt | [EV:tree@originals/License/screens/scriptDone.png] |
| `License/screens/error1.png` | 800 | 480 | 8 | RGB (Truecolor) | Full-screen workflow prompt | [EV:tree@originals/License/screens/error1.png] |
| `HN+R_.../MU9411/tools/rePartitioningNeeded.png` | 800 | 480 | 8 | RGB (Truecolor) | System partition dialog | [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/MU9411/tools/rePartitioningNeeded.png] |
| `HN+R_.../RSU9425/tools/rePartitioningNeeded.png` | 800 | 480 | 8 | RGB (Truecolor) | System partition dialog | [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/RSU9425/tools/rePartitioningNeeded.png] |

### 5.2 Font Assets Census

| Font Family | PostScript Name | Path in `originals/` | Glyph Count | Codec | Supported Scripts & Layout Tables | Evidence Tag |
|---|---|---|---|---|---|---|
| Audi Univers 540 Med | `AudiUnivers-Med` | `HN+R_.../GEMMI/nav/0/default/res/AudiUnivers540Med.ttf` | 537 | TTF | Latin, Cyrillic, Greek, Common (100+ languages) | [EV:cmd@fc-scan-font1] |
| LTUnivers 440 Extended | `LTUnivers-Extended` | `HN+R_.../GEMMI/nav/0/default/res/AudiUnivers540Med-AGCC.ttf` | 867 | TTF | `otlayout:arab otlayout:cyrl otlayout:grek otlayout:latn` | [EV:cmd@fc-scan-font2] |
| LTUnivers 440 Extended | `LTUnivers-Extended` | `HN+R_.../GEMMI/nav/0/default/res/LT_Univers440_88perc_Arabic.ttf` | 867 | TTF | `otlayout:arab otlayout:cyrl otlayout:grek otlayout:latn` | [EV:cmd@fc-scan-font3] |

Both extended fonts include the OpenType `otlayout:arab` layout tables necessary for right-to-left (RTL) contextual glyph shaping, establishing live Arabic rendering as a core workstation requirement [EV:cmd@fc-scan-font3].

### 5.3 Candidate Visual Assets Summary

1. **Navigation UI Icons & Cursors (72 PNGs)**:
   - Located in `HN+R_EU_AU_K0942_4_[8R0906961FB]/GEMMI/nav/0/default/models/` [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/GEMMI/nav/0/default/models].
   - Google Earth POI icons: Wikipedia (22x22 RGBA), Panoramio (22x22 RGBA), Places (22x22 RGBA), Businesses (22x22 RGBA), StreetView Pegman (22x22 RGBA) [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/GEMMI/nav/0/default/models/wikipedia.png].
   - Navigation vehicle cursors: `cursors.png` (364x560 RGBA sprite sheet containing 3D vehicle orientation angles), `audisvcursor.png`, `audisvinfocursor.png`, `audisvjumpcursor.png` [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/GEMMI/nav/0/default/models/cursors.png].
   - Traffic status badges: 48 icons covering UK and European traffic congestion levels (sizes 8x8 to 32x32, 8-bit RGBA) [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/GEMMI/nav/0/default/models/traffic_status_32x32_green.png].
   - Engine CPU throttling icons: `cpu_throttle_*.png`, `throttle_red.png`, `throttle_yellow.png` (8-bit RGBA) [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/GEMMI/nav/0/default/models/throttle_red.png].
   - Internal build watermark: `erl_dev_build_image.png` (527x460 RGBA) [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/GEMMI/nav/0/default/models/erl_dev_build_image.png].

2. **Instrument Cluster Turn Graphics (360 `.precomp` files)**:
   - Located across regional CombiStyles modules: `IND`, `ECE`, `NAR`, `ASIA`, `ME`, `LA`, `SA`, `TR`, `CL`, `AN` [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/CombiStyles].
   - Resolution: 500 x 248 pixels (or proportional cluster sub-regions) with 32-bit RGBA decompressed buffers [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/CombiStyles/IND/0/default/arr_e.precomp].
   - Content: Arrow turn indicators (`arr_*.precomp`), roundabout exits (`ra_exit_*.precomp`), destination flag (`zielflagge.precomp`), Audi navigation globe (`navglobe_audi.precomp`), and lane guidance icons [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/CombiStyles/IND/0/default/zielflagge.precomp].

3. **3D Cursor Mesh (1 `.dae` file)**:
   - `HN+R_EU_AU_K0942_4_[8R0906961FB]/GEMMI/nav/0/default/models/AudiCursor.dae` (177,200 bytes Collada XML 3D model) [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/GEMMI/nav/0/default/models/AudiCursor.dae].

### 5.4 Census Statistical Totals

- Total Candidate Visual & Font Assets Cataloged: **444 assets** [EV:survey@scan_originals.py].
- Successful Metadata & Dimension Decodes: **444 assets (100.0% success rate)** [EV:survey@scan_originals.py].
- Failed Decodes: **0 assets** [EV:survey@scan_originals.py].
- Total CAS Thumbnail BLAKE3 Fingerprints Captured: **444 hashes** [EV:survey@scan_originals.py].
