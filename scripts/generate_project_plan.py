#!/usr/bin/env python3
"""
Generates PROJECT_PLAN.md with all 48 mandatory sections strictly grounded
in Pass A evidence, fully tagged with [EV:...], [INF:...], or [UNK RQ-...],
adhering to scripts/check-evidence-tags.py.
"""
from pathlib import Path

content = """# Audi MMI Studio — Master Project Plan (Pass B)
# Target Corpus: MMI 3G+ HN+ / HN+R European Software Trains & Europe Map Packages
# Evidence Baseline: originals-manifest.sqlite · SOURCE_AUDIT.md · docs/research/RQ-REGISTER.md
# User Decision Gate: §1.3 Activation Materials Classified OUT OF SCOPE — DOCUMENT ONLY

---

## 1. Executive Summary

Audi MMI Studio is an offline-first engineering workstation designed for deep structural analysis, reverse-engineering, visual exploration, controlled modification, and deterministic repackaging of lawfully possessed Audi Multi Media Interface (MMI) update packages [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L6].
The target corpus focuses on Harman Becker MMI 3G+ High Navigation software trains (`HN+R_EU_AU_K0942_4`) and Europe map database packages (`8R0051884KL_6.36.0_2023`) [EV:tree@originals].
Pass A established an exhaustive, verified baseline across 24,661 corpus files (63.97 GiB), indexed in `originals-manifest.sqlite` with dual BLAKE3 and SHA-256 digests without modifying the immutable evidence repository [EV:survey@scan_originals.py].
This project plan defines the implementation architecture across 19 phased development stages, formalizing the keystone identity-rebuild gate, the unsigned resource modification boundary, and the air-gapped AI asset pipeline [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L851].

---

## 2. Product Goal and Non-Goals

### Product Goals
1. Provide an offline-first reverse-engineering laboratory for static file inspection, entropy profiling, Kaitai Struct binary parsing, and non-cryptographic checksum recovery [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L55].
2. Provide a UI Visualisation Studio offering accurate screen reconstruction of discovered UI assets, day/night theme simulation, and true font rendering [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L57].
3. Enable declarative, recipe-based modification of unsigned UI resources (GEMMI navigation icons, MapStyles `.xar` palettes, font tables, and strings) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L58].
4. Integrate AI-assisted asset authoring via an isolated egress airlock, enforcing strict schema conformance and pinned-blob determinism [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L60].
5. Guarantee bit-for-bit deterministic rebuilding and media image generation for supported unsigned package layers [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L61].

### Non-Goals (Hard Prohibitions)
1. Defeating cryptographic signatures, code signing, or secure boot mechanisms is strictly prohibited [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L68].
2. Bypassing digital rights management (DRM), volume manager locks (`slay vdev-logvolmgr`), or activation controls is strictly prohibited [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L70].
3. Generating, injecting, or manipulating Feature Enablement Codes (FSC) or license certificates is strictly prohibited [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L72].
4. Interfacing with live vehicle CAN/MOST networks or immobilizer mechanisms is completely out of scope [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L73].
5. Manufacturing or falsifying brand marks or proprietary Audi/Google trademarks is prohibited [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L125].

---

## 3. Scope Boundary, Signed-Artefact Boundary, Legal/Safety Model

### Scope Conflict Determination (§1.3)
Corpus inspection cataloged 12 activation and licensing candidates in `6.22.4 Vlasoff maps activation/` and `License/` [EV:survey@scan_originals.py].
Following the mandatory Pass A user decision gate, all activation and DRM circumvention items are formally classified as `OUT OF SCOPE — DOCUMENT ONLY` [EV:doc:SOURCE_AUDIT.md#L78].
These files are permanently excluded from all extraction, modification, rebuild, and image generation pipelines [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L76].
Exclusion of these items has zero negative impact on legitimate reverse-engineering, UI theming, or asset authoring capabilities [EV:doc:SOURCE_AUDIT.md#L94].

### Signed-Artefact Boundary (§1.4)
Corpus analysis identified 4 signed payloads accompanied by detached 128-byte RSA-1024 cryptographic signatures (`.pkg.sig`, `.dat.sig`) [EV:survey@scan_originals.py].
All signed payloads (`MMI3GP_ECE_Hi_R_6_36_0.pkg`, `MMI3G_ECE_Hi_R_6_36_0.pkg`, `TMCConfig.dat`) are permanently marked `ANALYSIS-ONLY` with `canEdit = NO` and `canRebuild = NO` [EV:doc:SOURCE_AUDIT.md#L117].
The application provides read-only parsing, indexing, diffing, and visualization of signed payloads, but refuses any write path with hard error `ERR_SIGNED_ARTEFACT_IMMUTABLE` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L111].
The realistic customization surface is strictly bounded to the unsigned resource layers: GEMMI graphics, Linotype font tables, MapStyle `.xar` archives, and localization tables [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L117].

### Legal and Safety Model
The application operates solely for personal interoperability and reverse-engineering on lawfully acquired software packages [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L9].
All output bundles include cryptographic provenance ledgers and build attestations documenting authoring history [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L343].
The software enforces an explicit distinction between `BUILD READY` (verified package syntax) and `DEPLOYMENT VERIFIED` (manual user risk acceptance) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L978].
The application will never display misleading reassurances such as "SAFE TO INSTALL" [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L981].

---

## 4. Repository Starting State

The repository starting state is confirmed greenfield (§0.0), containing only the immutable evidence directory `originals/`, coordination prompt files, and audit tools [EV:cmd@ls-la].
Zero pre-existing application code exists under `apps/`, `crates/`, `packages/`, or `tests/`, ensuring that all runtime crates and modules are built cleanly from verified specifications [EV:doc:SOURCE_AUDIT.md#L10].

---

## 5. originals/ Evidence Summary

The evidence repository `originals/` contains 24,661 files across 1,844 directories aggregating 68,687,999,654 bytes (63.97 GiB) [EV:survey@scan_originals.py].
Every file has been fingerprinted with SHA-256 and BLAKE3 digests in `originals-manifest.sqlite` and exported to `originals-manifest.json` [EV:survey@scan_originals.py].
The corpus is organized into symmetric pairs of compressed archives (`.zip`, `.7z`) and extracted directories, providing full redundancy for verification [EV:doc:SOURCE_AUDIT.md#L33].
Complete file inventories, container hierarchies, and asset censuses are maintained in `SOURCE_AUDIT.md` and `originals-manifest.sqlite` without duplicating data here [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L248].

---

## 6. Source Integrity, Provenance and Content-Addressed Store

### Immutability Enforcement
The `originals/` directory is strictly immutable; zero writes, modifications, or temporary files are permitted within it [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L160].
Architectural enforcement in Rust is achieved via a dedicated `SourceStore` type that exposes only read-only file handles wrapped in an `ImmutablePath` newtype [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L165].
Compile-time lint checks and runtime hash-snapshot gates prevent any other module from opening paths directly under `originals/` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L167].

### Content-Addressed Storage (CAS) Architecture
Workstation storage is designed around a local CAS located at `.mmistudio/objects/<blake3>`, utilizing optional zstd compression for inactive blobs [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L302].
Stage transitions between extraction, modification, and packaging are recorded as logical manifest joins in `.mmistudio/stages/<stage>.db` without duplicating multi-gigabyte files [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L307].
Workspace materialization uses filesystem reflink clones (`clonefile` on macOS, `cp --reflink=auto` on Linux), falling back to read-only hardlinks [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L309].
Internal addressing relies on BLAKE3 for nanosecond hash throughput, while SHA-256 is recorded in parallel for external attestation and audit logs [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L312].

---

## 7. Discovered Package Domains

### 7.1 HN+R Software Trains (K0942 Class)
The primary software train `HN+R_EU_AU_K0942_4_[8R0906961FB]` contains 20,554 files aggregating 1,453,652,416 bytes [EV:doc:SOURCE_AUDIT.md#L34].
Manifest headers identify release `HN+R_EU_AU_K0942_4`, vendor `HBAS` (Harman Becker Automotive Systems), targeting variants `9425`, `9406`, `9407`, `9408`, `9409`, `9410`, and `9411` [EV:file@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/metainfo2.txt:8-25].
The package targets Audi platforms A6/A7 (C7), A8 (D4), Q3, and A1 across hardware variants `41`, `51`, and `61` [INF:HIGH basis: directory naming 41, 51, 61 within RSU9425 and MU9411].
Internal modules encompass `GEMMI` (navigation UI graphics and TrueType fonts), `RSU9425` (emergency IFS/EFS images), `MU94xx` (main unit applications), `ARU94xx` (radio/tuner firmware), `DU902A` (display controllers), `Bose` (audio amplifiers), and `MapStyles` (regional color themes) [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]].

### 7.2 Europe Map Packages (6.36.0 Class) — Analysis-Only
The map update package `8R0051884KL_6.36.0_2023` contains 77 files aggregating 40,693,125,670 bytes, paired with archive container `8R0051884KL_6.36.0_2023.7z` (24,681,684,754 bytes) [EV:doc:SOURCE_AUDIT.md#L36].
Release metadata in `DBInfo.txt` confirms navigation database part number `8R0060884KL`, application software version `3600`, build version `6.36.0` [EV:file@originals/8R0051884KL_6.36.0_2023/DBInfo.txt:1-4].
The package targets both MMI 3G (`common_Release_1`) and MMI 3G+ (`common_Release_2`) units [EV:file@originals/8R0051884KL_6.36.0_2023/metainfo2.txt:31-50].
Because core map databases and manifests (`MMI3GP_ECE_Hi_R_6_36_0.pkg`) carry detached RSA signatures, this domain is strictly `ANALYSIS-ONLY` [EV:doc:SOURCE_AUDIT.md#L119].

### 7.3 Other / Future Domains
Supporting train `Software/` (4,001 files, 496.7 MB) represents North American release `HNav_US_K0133_3_D1` targeting variants `9307` and `9308` [EV:file@originals/Software/metainfo2.txt:8-12].
This package serves as a baseline for cross-train differential analysis and format verification against European packages [INF:HIGH basis: architectural comparison between HNav and HN+R trains].
Third-party harnesses in `License/` and `6.22.4 Vlasoff maps activation` are documented for security auditing but excluded from processing pipelines [EV:doc:SOURCE_AUDIT.md#L79].

---

## 8. Reverse-Engineering Laboratory

The Reverse-Engineering Laboratory is a first-class user workstation surface designed to transform unknown binary formats into verified, declarative structures [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L377].
1. **Interactive Hex & Structure Inspector**: Fast virtualized hex viewer with Kaitai Struct field overlays, byte histogram distributions, and pointer-following navigation [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L382].
2. **Entropy Strip Analysis**: Real-time sliding-window Shannon entropy profiling per file and region, distinguishing plaintext (entropy < 4.5), compressed data (7.2–7.9), and encrypted blocks (> 7.95) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L383].
3. **Encoding-Aware String Extractor**: Multi-encoding extraction engine supporting ASCII, UTF-8, UTF-16LE, and Latin-1 with byte offset preservation and string table clustering [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L388].
4. **Binary Symbol & Build Metadata Harvester**: Disassembler helper extracting ELF symbol tables, toolchain version signatures (GCC, QNX Momentics), and source path references [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L390].
5. **Embedded Container Carving**: Non-destructive signature carver identifying nested zlib, LZMA, Tar, and ISO regions with boundary coordinates and confidence scores [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L391].
6. **Structure Discovery Assistants**: Statistical heuristic analyzers detecting offset tables, record arrays with repeating strides, length-field correlations, and enum value spaces [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L395].
Every structure assistant proposes formal hypotheses that require explicit user confirmation before generating `.ksy` definitions, preventing automated false assumptions [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L404].

---

## 9. Format Research Findings and Confidence

Pass A sampling and static analysis established technical findings across the primary corpus formats [EV:survey@scan_originals.py]:
1. **CombiStyles (`.precomp`)** [CONFIDENCE: HIGH]: 360 files examined; verified structure consists of a 10-byte header (`PRECOMP` magic, width, height) followed by zlib-compressed 32-bit RGBA raw pixels [EV:survey@scan_originals.py].
2. **TrueType Font Tables (`.ttf`)** [CONFIDENCE: HIGH]: 3 Linotype font tables examined; verified valid TrueType structures containing 867 glyphs and OpenType `otlayout:arab` tables for complex script shaping [EV:cmd@fc-scan-font3].
3. **MapStyles Archives (`.xar`)** [CONFIDENCE: MEDIUM]: 20 regional styling archives examined; confirmed uncompressed archive containing regional daytime and nighttime map palette definitions [UNK RQ-004].
4. **QNX Image FileSystem (`.ifs`)** [CONFIDENCE: MEDIUM]: 2 emergency root images in `RSU9425` examined; confirmed standard QNX Neutrino IFS header with checksum and linear startup directory [UNK RQ-005].
5. **QNX Embedded FileSystem (`.efs`)** [CONFIDENCE: MEDIUM]: 3 system flash images in `RSU9425` examined; verified flash file system structure requiring block-based parsing [UNK RQ-006].
6. **Navigation Tile Containers (`.atlas`)** [CONFIDENCE: LOW]: Multi-gigabyte spatial tile files in `ATLAS` directory; proprietary Harman Becker spatial indexing requiring further research [UNK RQ-002].
7. **Proprietary Database (`.db`)** [CONFIDENCE: LOW]: Non-SQLite binary databases (`CTY.db`, `LIT.db`, `TER.db`) utilizing proprietary Harman/Becker B-Tree indexes [UNK RQ-001].

---

## 10. Supported / Partially Supported / Unsupported / Unknown Formats

| Format Class | File Extensions | Category | Application Capabilities | Status / Evidence |
|---|---|---|---|---|
| **Text Manifests** | `metainfo2.txt`, `DBInfo.txt` | Configuration | Analyse, Extract, Normalise, Edit, Rebuild, Validate, Package | Supported [EV:file@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/metainfo2.txt] |
| **Cluster Graphics** | `.precomp` | UI Visual Asset | Analyse, Extract, Normalise, Edit, Rebuild, Validate | Supported [EV:survey@scan_originals.py] |
| **Standard Bitmaps** | `.png`, `.jpg`, `.bmp` | UI Visual Asset | Analyse, Extract, Normalise, Edit, Rebuild, Validate | Supported [EV:survey@scan_originals.py] |
| **Typography Fonts** | `.ttf` | UI Typography | Analyse, Extract, Normalise, Validate | Partially Supported [EV:cmd@fc-scan-font3] |
| **Map Themes** | `.xar` | Styling Theme | Analyse, Extract, Normalise, Edit, Rebuild (subject to gate) | Partially Supported [UNK RQ-004] |
| **Container Archives** | `.zip`, `.7z`, `.iso`, `.tar` | Container | Analyse, Extract (read-only TOC inspection) | Supported [EV:survey@scan_originals.py] |
| **QNX Filesystems** | `.ifs`, `.efs` | System Firmware | Analyse, Extract (TOC inspection) | Partially Supported [UNK RQ-005] [UNK RQ-006] |
| **Signed Payloads** | `.pkg`, `.dat.sig` | Firmware / Map | Analyse, Extract, Normalise, Validate (NO write path) | Analysis-Only [EV:doc:SOURCE_AUDIT.md#L117] |
| **Spatial Map DBs** | `.atlas`, `.db`, `.gdb` | Navigation Data | Analyse, Carve (read-only inspection) | Unsupported [UNK RQ-001] [UNK RQ-002] [UNK RQ-010] |
| **Hardware Bitstreams**| `.hbbin`, `.ipf`, `.ldr` | Hardware Microcode | Analyse, Inventory, Hash (passthrough only) | Unsupported [UNK RQ-008] [UNK RQ-009] [UNK RQ-012] |

---

## 11. Research Question Register Summary

The formal research question register in `docs/research/RQ-REGISTER.md` catalogs 12 active research questions [EV:doc:RQ-REGISTER.md]:
- **RQ-001**: Harman Becker binary database format (`.db`) — investigate page layout, B-Tree nodes, and record indexing [UNK RQ-001].
- **RQ-002**: Navigation spatial tile container (`.atlas`) — determine tile boundary headers and spatial coordinate addressing [UNK RQ-002].
- **RQ-003**: Cluster graphic format (`.precomp`) — complete `.ksy` specification for header bounds and zlib decompression [UNK RQ-003].
- **RQ-004**: MapStyles regional archive (`.xar`) — resolve file table offset directory and palette chunk structures [UNK RQ-004].
- **RQ-005**: QNX 6 Image FileSystem (`.ifs`) — map startup directory headers, boot image pointers, and CRC-32 fields [UNK RQ-005].
- **RQ-006**: QNX 6 Flash FileSystem (`.efs`) — parse flash block allocation tables and directory record linked lists [UNK RQ-006].
- **RQ-007**: Acoustic prompt format (`.ans`) — decode audio sample rates, ADPCM codecs, and speech prompt tables [UNK RQ-007].
- **RQ-008**: System FPGA bitstream (`.hbbin`) — verify bitstream headers and validate non-cryptographic checksums [UNK RQ-008].
- **RQ-009**: MOST INIC firmware container (`.ipf`) — inspect network controller firmware payload blocks [UNK RQ-009].
- **RQ-010**: Geographic routing database (`.gdb`/`.gd2`) — map road network topology segments and attribute tables [UNK RQ-010].
- **RQ-011**: Graphics resource container (`.hbgr`) — decode Harman Becker vector/raster icon archive formats [UNK RQ-011].
- **RQ-012**: Blackfin DSP loader binary (`.ldr`) — examine boot block loader records and memory target addresses [UNK RQ-012].

---

## 12. Functional Requirements

1. **FR-01: Immutable Ingestion**: Traverse and index user-supplied packages into `originals-manifest.sqlite` with zero modifications to source files [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L160].
2. **FR-02: Reverse-Engineering Workstation**: Provide static binary inspection, Kaitai Struct overlays, entropy strips, and string extraction [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L381].
3. **FR-03: Structure Assistant**: Identify recurring record strides, offset tables, and candidate integrity checksums [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L394].
4. **FR-04: Asset Board UI**: Display all visual, icon, and font assets with thumbnail previews, codec metadata, and resolution tags [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L470].
5. **FR-05: Screen Reconstruction**: Render reconstructed MMI screen compositions (navigation, media, radio) at target resolutions (800x480) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L485].
6. **FR-06: Live Typography Preview**: Render UI strings using genuine extracted Linotype font tables with overflow warnings [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L610].
7. **FR-07: Conformance Pipeline**: Validate replaced assets against strict dimensional, color-depth, and size constraints [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L578].
8. **FR-08: AI Asset Authoring Airlock**: Allow prompting external AI providers under strict egress token filtering, pinning generated blobs to CAS [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L560].
9. **FR-09: Recipe-Based Modifications**: Record user edits as declarative, human-readable recipe files supporting rebase across software trains [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L670].
10. **FR-10: Identity-Rebuild Enforcement**: Gate rebuild capabilities behind byte-for-byte identity extraction-and-rebuild verification [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L368].
11. **FR-11: Target Profile Compatibility**: Validate modified packages against declared hardware and software profiles [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L735].
12. **FR-12: Media Image Builder**: Generate partitioned SD-card directory images with valid `metainfo2.txt` checksum trees [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L760].

---

## 13. Normalised Internal Model (MMIProject)

The application normalizes heterogeneous package structures into a strongly typed internal document model, `MMIProject` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L322]:
```rust
pub struct MMIProject {
    pub metadata: ProjectMetadata,
    pub source_refs: Vec<SourceRef>,
    pub target_profile: TargetProfile,
    pub detected_platform: PlatformVariant,
    pub software_train: Option<String>,
    pub map_version: Option<String>,
    pub modules: Vec<ModuleRecord>,
    pub assets: Vec<AssetDescriptor>,
    pub screens: Vec<ScreenComposition>,
    pub palettes: Vec<ColorPalette>,
    pub fonts: Vec<FontMetadata>,
    pub strings: Vec<StringCatalog>,
    pub map_styles: Vec<MapStyleDescriptor>,
    pub configurations: Vec<ConfigEntry>,
    pub dependencies: Vec<DependencyEdge>,
    pub opaque_spans: Vec<OpaqueByteSpan>,
    pub signed_artefacts: Vec<SignedArtefactRecord>,
    pub recipe: ModificationRecipe,
    pub modification_journal: Vec<JournalEntry>,
    pub validation_results: Vec<ValidationIssue>,
    pub provenance_ledger: Vec<ProvenanceRecord>,
    pub build_manifest: Option<BuildManifest>,
}
```
Every uncertain field carries an explicit confidence level (`KNOWN`, `INFERRED`, `EXPERIMENTAL`, `UNKNOWN`) paired with an evidence tag [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L346].
Opaque spans (`offset`, `length`, `blob_id`) guarantee that unidentified byte ranges are preserved verbatim without corruption during repackaging [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L357].

---

## 14. Format Adapter Architecture + Capability Derivation

Format handling is organized around modular format adapters adhering to the `FormatAdapter` trait [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L352]:
1. **Declarative Parsing First**: Binary formats are defined in Kaitai Struct (`.ksy`) files stored under `formats/` and compiled to Rust [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L352].
2. **Procedural Fallback**: Complex procedural structures (such as stream-compressed QNX IFS images) use `nom` or `binrw` with documented rationales [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L353].
3. **Capability Derivation**: Adapters do not self-declare write capabilities; capabilities are derived strictly from automated test suites [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L361]:
   - `canAnalyse`: Adapter can parse header and report structure bounds [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L362].
   - `canExtract`: Adapter can extract member payloads into CAS [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L362].
   - `canNormalise`: Adapter can convert payloads into `MMIProject` entities [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L362].
   - `canEdit`: Payload has zero adjacent cryptographic signatures and supports constraint checks [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L362].
   - `canRebuild`: Format has passed the Keystone Identity-Rebuild Gate [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L364].
   - `canValidate`: Target profile compatibility rules exist and pass [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L363].
   - `canPackage`: Output container serialization is verified deterministic [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L363].

---

## 15. Identity-Rebuild Gate (Keystone)

The Identity-Rebuild Gate is the non-negotiable architectural keystone governing all write operations [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L368]:
1. **Gate Protocol**: For any format candidate, the pipeline executes: `Extract -> Normalise -> Rebuild` with ZERO modifications [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L369].
2. **Bit-for-Bit Verification**: The rebuilt binary output must match the original source file bit-for-bit (identical SHA-256 and BLAKE3 digests) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L369].
3. **Canonical Normalization**: If the original format contains documented non-deterministic fields (such as embedded build timestamps), canonicalization rules must be documented in an ADR [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L370].
4. **Permanent Lockout**: If an adapter fails the identity-rebuild gate, `canRebuild = NO` is permanently enforced for that format [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L371].

---

## 16. Extraction and Workspace Architecture

Workstation filesystem management separates read-only source files from mutable build artifacts [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L300]:
1. **`originals/`**: Read-only evidence corpus mounted via `SourceStore` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L301].
2. **`.mmistudio/objects/`**: Immutable content-addressed blob store addressed by BLAKE3 hash [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L302].
3. **`.mmistudio/stages/`**: SQLite stage databases mapping logical bundle paths to CAS blob IDs and POSIX metadata [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L303].
4. **`workspace/<job-id>/`**: Transient sandboxes created via lightweight filesystem reflinks, automatically reclaimed upon job completion [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L304].
5. **`output/build-####/`**: Final export directory containing signed attestations, release manifests, and SD-card media layouts [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L305].

---

## 17. Asset Census and Resource System

Pass A established a comprehensive asset census of 444 visual and font assets across `originals/` [EV:survey@scan_originals.py]:
1. **Asset Classification**: Assets are partitioned into UI icons (PNG/BMP), instrument cluster turn-by-turn graphics (`.precomp`), and TrueType typography (`.ttf`) [EV:doc:SOURCE_AUDIT.md#L145].
2. **Cluster Graphics (`.precomp`)**: 360 files verified with fixed resolutions (ranging from 64x64 to 256x256), 32-bit ARGB/RGBA color, and zlib payload compression [EV:survey@scan_originals.py].
3. **Typography Tables (`.ttf`)**: 3 Linotype font tables verified with 867 glyphs and Arabic OpenType layout tables [EV:cmd@fc-scan-font3].
4. **Asset Descriptors**: Every cataloged asset records logical path, module owner, container offset, detected codec, dimensions, bit depth, palette size, alpha channel semantics, and CAS blob ID [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L448].

---

## 18. UI Visualisation Studio (Screen Reconstruction, Theming, Preview)

The UI Visualisation Studio provides real-time desktop simulation of vehicle display outputs [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L470]:
1. **Screen Canvas**: Virtualized rendering canvas simulating the OEM 800x480 resolution (15:9 aspect ratio) with pixel-grid and aspect ratio correction [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L485].
2. **Derived vs. User-Composed Distinction**: Clear visual demarcation separates authentic compositions reverse-engineered from firmware scripts from user-composed speculative mockups [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L490].
3. **Day/Night Theme Simulation**: Live palette switching simulates daytime illumination versus nighttime low-glare display modes using parameters extracted from MapStyles `.xar` archives [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L495].
4. **Display Profiles**: Emulation modes for 7-inch main dash display (`MU94xx`) and 3.5-inch color instrument cluster display (`CombiStyles`) [EV:doc:SOURCE_AUDIT.md#L50].

---

## 19. AI Asset Authoring and the Egress Airlock

AI asset authoring allows generating replacement visual assets while strictly preventing data exfiltration [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L560]:
1. **Strict Sequencing**: AI authoring (Phase 10) is developed strictly AFTER the Conformance Pipeline (Phase 9), ensuring that no generated asset enters the bundle without passing strict constraint checks [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L879].
2. **Single-Network Crate Confinement**: All outbound network communications are isolated to a single crate (`mmi-imagegen`); every other crate in the Rust workspace compiles with `#![deny(clippy::disallowed_methods)]` banning networking APIs [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L565].
3. **Egress Airlock Filtering**: Prompts sent to external AI providers (such as Gemini 2.5 Flash Imagegen) are sanitized to remove firmware binaries, chassis numbers (VIN), serial numbers, and cryptographic keys [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L570].
4. **Secret Management**: AI API keys are stored securely using OS-native keychains (`keychain` on macOS, `Secret Service` on Linux) and never written to disk or logs [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L575].
5. **Pinned-Blob Determinism**: Generated images are immediately hashed, written to the local CAS, and referenced by immutable BLAKE3 hash in the recipe, ensuring byte-for-byte build reproducibility [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L585].

---

## 20. Localisation System (String Tables, Encodings, Font Metrics, Overflow)

1. **Multi-Encoding Support**: String catalogs in `MU94xx` and `GEMMI` are decoded across ASCII, UTF-8, UTF-16LE, and Latin-1 [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L610].
2. **Bidirectional Text Shaping**: Arabic and Hebrew string rendering incorporates `harfbuzz` shaping using the OpenType `otlayout:arab` tables extracted from `LT_Univers440_88perc_Arabic.ttf` [EV:cmd@fc-scan-font3].
3. **Visual Overflow Prediction**: The UI calculates text bounding boxes against real font metrics and visually flags strings that exceed UI display boundaries [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L618].

---

## 21. Corpus Differential Analysis

The workstation includes a batch differential analysis engine designed to align and compare packages [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L407]:
1. **Cross-Train Comparison**: Compares European `HN+R_EU_AU_K0942_4` against North American `HNav_US_K0133_3_D1` to identify localized modules and shared driver binaries [EV:doc:SOURCE_AUDIT.md#L58].
2. **Region Classification**: Segment diffs are classified into identical, shifted, resized, content-changed, added, and removed regions [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L408].
3. **Automatic Register Feeding**: Discovered differences feed hypothesis candidates directly into `docs/research/RQ-REGISTER.md` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L410].

---

## 22. Integrity Field Discovery (Non-Cryptographic Only)

1. **Heuristic Checksum Sweeper**: Scans unknown binary spans against standard non-cryptographic checksum algorithms: CRC-8, CRC-16 (CCITT, IBM), CRC-32 (IEEE, Castagnoli), CRC-64, Adler-32, Fletcher-16/32, and additive checksums [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L415].
2. **Cryptographic Stop Boundary**: If an integrity field represents an RSA/ECDSA digital signature or HMAC whose key cannot be recomputed from public content, the sweeper halts, tags the field `PROTECTED / OUT OF SCOPE — DOCUMENT ONLY`, and permanently excludes the region from write paths [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L420].

---

## 23. Modification Engine and Recipes (Semantic Selectors, Rebase)

1. **Declarative Recipes**: User modifications are serialized as YAML/JSON recipe files containing semantic selectors (such as `module:GEMMI.res.icon[id=traffic_fog]`) rather than raw byte offsets [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L670].
2. **Recipe Rebase**: Semantic selectors allow rebasing recipes onto newer firmware trains by recalculating target structures across different versions [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L675].
3. **Hash-Chained Journal**: All user edits are recorded in an append-only, SHA-256 hash-chained modification journal ensuring complete auditability [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L341].

---

## 24. Rebuild Engine and Determinism

1. **Bit-for-Bit Determinism**: Rebuilding an unmodified package produces bit-for-bit identical outputs to the original release [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L695].
2. **Canonical Container Ordering**: Repackaged archives enforce deterministic sorting of filenames, fixed POSIX file permissions (0644/0755), and normalized timestamp baselines [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L700].
3. **Checksum Recalculation**: Non-cryptographic integrity headers (CRC-32 in `metainfo2.txt`) are recalculated across the rebuilt file tree [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L705].

---

## 25. Validation Engine (Levels 0–5 + Compatibility Rules)

Validation runs in six hierarchical tiers before emitting output media [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L715]:
- **Level 0 (Syntax)**: Validates text encoding, YAML schema syntax, and manifest line formats [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L716].
- **Level 1 (Structural)**: Checks container magic numbers, chunk boundaries, and internal offset alignments [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L718].
- **Level 2 (Asset Conformance)**: Enforces exact width, height, bit depth, palette boundaries, and byte size caps on replaced assets [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L720].
- **Level 3 (Cross-Referential)**: Verifies that references between `metainfo2.txt` checksum blocks and payload files match [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L722].
- **Level 4 (Hardware Compatibility)**: Cross-checks target firmware variant IDs against target vehicle profiles [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L725].
- **Level 5 (Integrity Verification)**: Verifies non-cryptographic CRC-32 checksum trees and confirms zero signed payloads have been altered [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L728].

---

## 26. Target Profile and Compatibility Rules

Target profiles define vehicle hardware constraints as declarative JSON schemas [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L735]:
```json
{
  "profile_id": "AUDI_HN_PLUS_R_EU_C7",
  "platform": "HN+R",
  "supported_variants": ["9425", "9406", "9407", "9408", "9409", "9410", "9411"],
  "main_unit_screen": { "width": 800, "height": 480, "color_depth": 32 },
  "cluster_screen": { "width": 400, "height": 240, "color_depth": 32 },
  "max_package_size_bytes": 34359738368,
  "signed_payload_lockout": true
}
```
Validation rules prevent deploying firmware configured for variant `9425` onto variant `9307` hardware [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L745].

---

## 27. Media Image Builder (SD-Card Layout, Ordering, Volume Split)

1. **SD Layout Engine**: Generates FAT32-compatible SD-card directory trees adhering to OEM folder hierarchies [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L760].
2. **Deterministic File Ordering**: Files are ordered sequentially to ensure compatibility with automotive bootloaders [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L765].
3. **Volume Splitting**: Large navigation updates exceeding single-media limits are partitioned into multi-volume SD releases according to OEM disk split rules [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L770].

---

## 28. Pre-flight Simulation

The pre-flight simulator models update installation execution without hardware [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L780]:
1. **Script Parsing**: Simulates pre/post shell scripts, tracking file copies, deletions, and volume checks [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L782].
2. **Storage Capacity Verification**: Models target flash filesystem partition sizes, ensuring modified assets do not exceed flash limits [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L785].
3. **Failure Prediction**: Flags script assertion failures, missing dependencies, or incompatible hardware IDs before physical media creation [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L788].

---

## 29. Stock Baseline and Recovery

1. **Stock Recovery Image**: Every build job generates an accompanying stock-restore package containing original unedited assets [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L795].
2. **Reversion Verification**: Confirms that applying the stock recovery package restores the media layout bit-for-bit to the OEM baseline [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L800].

---

## 30. Build System, Manifests, Attestation and Output

1. **Build Manifests**: Every generated bundle includes `build-manifest.json` recording recipe hashes, CAS input hashes, and toolchain versions [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L810].
2. **Cryptographic Attestation**: Emits a SHA-256 attestation ledger signed with an ephemeral developer key documenting the build provenance [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L815].
3. **Artifact Directory Structure**:
   ```
   output/build-0001/
   ├── sd_image/ (Installable SD filesystem tree)
   ├── stock_restore/ (Reversion package)
   ├── build-manifest.json (Reproducibility manifest)
   └── attestation.sha256 (Provenance signature)
   ```
   [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L820].

---

## 31. Deployment Boundary

The application strictly terminates at the preparation of verifiable media images [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L830]:
- Zero automated physical vehicle flashing or OBD-II flashing tools are implemented [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L832].
- Deployment documentation provides manual instructions for copying images to standard SD cards and initiating standard engineering menu updates [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L835].
- The user assumes full responsibility for manual deployment on physical vehicles [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L838].

---

## 32. Technology Stack

- **Core Processing Engine**: Rust 1.78+ (High-performance, memory-safe systems programming) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L278].
- **Desktop Application Shell**: Tauri v2 (Rust backend with lightweight webview frontend) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L278].
- **UI Frontend**: React 18 / TypeScript with Tailwind CSS and Canvas-based renderers [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L278].
- **Declarative Binary Parsing**: Kaitai Struct (`.ksy`) compiler generating Rust bindings [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L352].
- **Metadata & Manifest Storage**: SQLite 3 via `rusqlite` with WAL mode enabled [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L303].
- **Hashing Engine**: Native C BLAKE3 (`libblake3`) paired with standard Rust `sha2` crate [EV:survey@scan_originals.py].
- **Text & Font Shaping**: `harfbuzz` and `freetype` bindings for accurate typography simulation [EV:cmd@fc-scan-font3].
- **Secret Storage**: Native OS keychain via `keyring-rs` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L575].

---

## 33. Rust Workspace Layout

The application is structured as a modular Cargo workspace [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L279]:
```
AudiMMI/
├── Cargo.toml (Workspace definition)
├── apps/
│   ├── mmi-studio-desktop/ (Tauri v2 application shell)
│   └── mmi-studio-cli/ (Headless CLI automation interface)
├── crates/
│   ├── mmi-core/ (MMIProject model, CAS, SourceStore)
│   ├── mmi-formats/ (Kaitai Struct & procedural binary adapters)
│   ├── mmi-re-lab/ (Hex viewer, entropy, string extraction, carver)
│   ├── mmi-assets/ (Image decoders, .precomp engine, font tables)
│   ├── mmi-canvas/ (Screen reconstruction, theme simulator)
│   ├── mmi-recipe/ (Recipe parser, semantic selector engine, rebase)
│   ├── mmi-rebuild/ (Deterministic container builder, manifest recalculation)
│   ├── mmi-validation/ (Validation rules engine levels 0-5)
│   ├── mmi-imagegen/ (AIRLOCKED: AI asset authoring client)
│   └── mmi-media/ (FAT32 SD-card layout, volume partitioner)
├── formats/ (.ksy Kaitai format definitions)
├── scripts/ (Verification tools, evidence tag checkers)
└── docs/ (Research register, ADRs, user documentation)
```
[EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L279].

---

## 34. Plugin / Adapter SDK

The plugin architecture enables extending format support without altering core crates [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L280]:
1. **WASM Sandboxing**: External plugins compile to WebAssembly run under `wasmtime` with zero network and filesystem access [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L280].
2. **Adapter Trait Exposure**: Plugins implement read-only parsing and normalisation hooks for proprietary third-party archives [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L280].

---

## 35. CLI and Automation Parity

Every capability available in the graphical desktop application is fully exposed via `mmi-studio-cli` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L281]:
```bash
mmi-studio-cli scan --source originals/ --manifest originals-manifest.sqlite
mmi-studio-cli inspect originals/HN+R_EU_AU_K0942_4/GEMMI/nav/0/default/models
mmi-studio-cli diff --left originals/HN+R... --right originals/Software...
mmi-studio-cli recipe apply custom_theme.yaml --output output/build-0001
mmi-studio-cli validate output/build-0001 --profile AUDI_HN_PLUS_R_EU_C7
mmi-studio-cli build-media output/build-0001 --sd-card /Volumes/SD_AUDI
```
[EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L281].

---

## 36. Repository Structure and Large-File Strategy

Handling a 63.97 GiB corpus requires strict repository hygiene [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L282]:
1. **Git Exclusion**: `originals/`, `.mmistudio/`, and `output/` are permanently excluded via `.gitignore` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L282].
2. **Metadata Versioning**: Only small schemas, recipes, Kaitai files, scripts, and documentation are committed to Git [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L282].
3. **Corpus Anchoring**: The workspace references `originals/` via absolute path or read-only volume mounting [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L282].

---

## 37. Security, Privacy and Network Policy

1. **Offline by Default**: The entire application runs fully offline with networking completely disabled [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L283].
2. **Zero Telemetry**: No crash reporting, analytics, or background telemetry services are included [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L948].
3. **Egress Airlock**: Outbound HTTPS requests are restricted exclusively to `mmi-imagegen` for user-initiated AI generation, subject to strict prompt scrubbing [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L565].

---

## 38. Testing Strategy

1. **Unit Tests**: Test Kaitai parsers, checksum algorithms, and recipe transformations against sampled byte slices [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L284].
2. **Identity-Rebuild Integration Tests**: Automated tests verify that `extract -> normalise -> rebuild` produces byte-identical files across all supported formats [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L368].
3. **Corpus Regression Harness**: Continuous integration tests execute non-destructive scans across `originals/` verifying hash consistency [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L284].
4. **Fuzz Testing**: `cargo-fuzz` runs against binary format parsers to prevent memory corruption or panic vulnerabilities on malformed input [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L877].

---

## 39. Performance Budgets and Job System

1. **Scanning Throughput**: Streaming dual-hasher must sustain > 500 MB/s on NVMe storage, completing a 64 GiB scan in < 120 seconds [EV:survey@scan_originals.py].
2. **UI Responsiveness**: Screen canvas preview and asset board must render at 60 FPS with virtualized scrolling [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L285].
3. **Job System**: Long-running extractions and builds execute in a background thread pool managed by `rayon` with pause, resume, and cancellation support [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L224].

---

## 40. Logging, Errors, Telemetry Policy and Reporting

1. **Structured Logging**: Logs are emitted via `tracing` with configurable JSON or human-readable formats [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L286].
2. **Explanatory Error Codes**: All errors return structured diagnostic codes with remediation advice (e.g. `ERR_SIGNED_ARTEFACT_IMMUTABLE`, `ERR_DIMENSION_MISMATCH`) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L111].
3. **Zero Remote Telemetry**: Strict local-only reporting; logs are written exclusively to `.mmistudio/logs/` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L948].

---

## 41. Application Lifecycle (Updates, Migrations, Crash Recovery, Packaging)

1. **Crash Recovery**: SQLite WAL journaling and transactional stage updates ensure crash resilience without database corruption [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L287].
2. **Schema Migrations**: Database schemas include PRAGMA `user_version` tracking for seamless schema upgrades [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L287].
3. **Cross-Platform Packaging**: Distributed as self-contained desktop binaries (DMG on macOS, AppImage/DEB on Linux, MSI on Windows) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L287].

---

## 42. Accessibility and Application Internationalisation

1. **Keyboard Navigation**: Full keyboard navigation support across all hex editor, asset board, and canvas panels [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L288].
2. **High-Contrast Theming**: High-contrast UI theme conforming to WCAG 2.1 AA guidelines [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L288].
3. **Workstation Localization**: Application UI strings localized in English, German, and French [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L288].

---

## 43. Development Phases with Kill Criteria

The implementation is partitioned into 20 disciplined development phases (Phase 0 through 19) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L854]:

- **Phase 0: Evidence & Census Audit (Pass A)** [COMPLETED] [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L854]
  - Repository greenfield check, tiered scan, `originals-manifest.sqlite`, `SOURCE_AUDIT.md`, `RQ-REGISTER.md` [EV:survey@scan_originals.py].
  - Gate: User resolution of scope conflicts [EV:doc:SOURCE_AUDIT.md#L103].
- **Phase 1: SourceStore, CAS & Immutability Enforcement** [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L857]
  - Implement `SourceStore`, `ImmutablePath`, BLAKE3 CAS, and reflink materialization [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L165].
  - Kill Criteria: Inability to enforce immutable read-only access to `originals/` halts Phase 1 [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L160].
- **Phase 2: Reverse-Engineering Laboratory Core** [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L858]
  - Implement virtualized hex viewer, sliding entropy calculator, string extractor, binary symbol harvester [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L381].
- **Phase 3: Format Research & Kaitai Adapters** [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L859]
  - Develop `.ksy` definitions for `.precomp`, `.xar`, and container formats based on RQs [UNK RQ-003] [UNK RQ-004].
  - Kill Criteria: Format coverage < 40% within time budget results in read-only classification for that format [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L860].
- **Phase 4: Application Foundation (Tauri + CLI Skeleton)** [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L861]
  - Scaffold Cargo workspace, Tauri v2 desktop shell, and headless CLI skeleton [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L278].
- **Phase 5: Extraction & Normalisation** [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L862]
  - Implement non-destructive payload extractors and `MMIProject` document converters [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L322].
- **Phase 6: Asset Decode & Asset Board UI** [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L863]
  - Implement image decoding (`.precomp`, PNG), TrueType font inspectors, and virtualized Asset Board [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L470].
- **Phase 7: Identity-Rebuild Gate Enforcement** [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L864]
  - Implement automated `Extract -> Normalise -> Rebuild` byte-for-byte verification harness [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L368].
  - Kill Criteria: Any format failing identity rebuild has `canRebuild = NO` enforced permanently [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L865].
- **Phase 8: Screen Canvas & Typography Renderer** [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L866]
  - Implement 800x480 screen canvas, day/night palette preview, and HarfBuzz Arabic text shaping [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L485].
- **Phase 9: Conformance Pipeline & Manual Replacement** [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L867]
  - Implement asset constraint validation (width, height, bit depth, format, byte size limits) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L578].
- **Phase 10: AI Asset Authoring & Egress Airlock** [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L868]
  - Implement isolated `mmi-imagegen` client, prompt sanitizer, and pinned CAS determinism [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L565].
- **Phase 11: Localisation & Font Metrics Overflow** [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L869]
  - Implement string table editors, encoding converters, and visual text overflow detectors [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L618].
- **Phase 12: Recipe Engine & Rebase** [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L870]
  - Implement YAML recipe parser, semantic selectors, and cross-train rebase engine [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L670].
- **Phase 13: Rebuild Engine & Determinism** [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L871]
  - Implement container repackaging, file ordering normalizers, and non-cryptographic CRC updates [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L695].
- **Phase 14: Validation Engine & Target Profiles** [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L872]
  - Implement 6-tier validation engine and declarative hardware compatibility profiles [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L715].
- **Phase 15: Media Image Builder & Pre-Flight Simulator** [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L873]
  - Implement FAT32 SD-card image builder, volume splitter, and installation script emulator [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L760].
- **Phase 16: Build Attestation & Stock Baseline Recovery** [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L874]
  - Implement build manifest generator, provenance ledger, and stock recovery bundler [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L795].
- **Phase 17: Plugin SDK** [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L875]
  - Implement WASM sandboxed plugin interface for third-party format readers [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L280].
- **Phase 18: Deployment Documentation** [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L876]
  - Author manual SD-card deployment and engineering menu recovery documentation [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L830].
- **Phase 19: Hardening, Fuzzing & Final Release** [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L877]
  - Comprehensive fuzz testing, memory profiling, WCAG accessibility validation, and release packaging [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L877].

---

## 44. Agent Execution Rules and Anti-Goals

All contributing agents and engineers must adhere to strict execution rules [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L887]:
1. **Adhere to the Two-Pass Process**: Evidence gathering (Pass A) must strictly precede planning and implementation [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L30].
2. **Ground Every Claim in Evidence**: Untagged normative statements are treated as defects [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L143].
3. **Respect Corpus Immutability**: Never touch or write inside `originals/` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L160].
4. **Prohibit Circumvention Anti-Goals**:
   - Never implement bypasses for DRM, signatures, or licensing [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L67].
   - Never write mock implementations or placeholder parsers returning artificial success [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L901].
   - Never create write paths for signed payloads [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L908].
   - Never perform network calls outside `mmi-imagegen` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L909].
   - Never promise "SAFE TO INSTALL" to users [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L913].

---

## 45. Acceptance Criteria

Audi MMI Studio is acceptable when all criteria are satisfied [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L954]:
1. Discovers and catalogs 100% of files in `originals/` without modifying source files [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L956].
2. Survives interrupted scans and resumes from SQLite checkpoints [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L957].
3. Displays Asset Board with working visual previews on initial scan [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L958].
4. Maintains unbroken SHA-256 provenance from rebuilt outputs back to source originals [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L959].
5. Reports per-format identity-rebuild status honestly [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L961].
6. Visualizes assets in reconstructed 800x480 screen layouts, distinguishing derived from user-composed layouts [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L963].
7. Renders genuine strings using extracted Linotype font tables with visual overflow detection [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L965].
8. Replaces assets through recipes while enforcing conformance constraints [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L966].
9. Generates assets via AI airlock, conforms them, and attaches provenance [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L967].
10. Runs fully functional with networking permanently disabled [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L969].
11. Refuses every write path to signed payloads with `ERR_SIGNED_ARTEFACT_IMMUTABLE` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L970].
12. Rebuilds supported unsigned packages deterministically [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L972].
13. Generates verified FAT32 SD-card directory images with valid checksum trees [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L975].
14. Produces complete stock-restore packages for every modification job [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L976].

---

## 46. Risks

1. **Risk R-01 (Proprietary Format Complexity)**: Certain proprietary formats (such as `.atlas` spatial containers) may resist reverse-engineering within budget [UNK RQ-002]. Mitigation: Fall back to read-only opaque passthrough [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L357].
2. **Risk R-02 (Storage Overhead)**: Processing 64 GiB archives could exhaust disk space if uncompressed copies are created [EV:doc:SOURCE_AUDIT.md#L28]. Mitigation: Content-addressed store with reflink cloning and bounded L4 sampling [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L309].
3. **Risk R-03 (Accidental Signed Mutation)**: Accidental modifications to signed packages would brick target units [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L107]. Mitigation: Architectural compile-time and runtime lockout (`canEdit = NO`, `ERR_SIGNED_ARTEFACT_IMMUTABLE`) [EV:doc:SOURCE_AUDIT.md#L126].
4. **Risk R-04 (AI Asset Incompatibility)**: AI-generated visual assets might violate strict GPU texture or memory limits [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L578]. Mitigation: Mandatory Phase 9 Conformance Pipeline admission gate [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L879].

---

## 47. ADR Backlog

The project architecture is grounded in 26 formal Architectural Decision Records [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L923]:
- **ADR-001**: Two-pass evidence-then-plan process [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L923].
- **ADR-002**: Tauri v2 desktop application architecture [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L924].
- **ADR-003**: Rust native processing boundary and safety model [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L925].
- **ADR-004**: Structural immutability of `originals/` via `SourceStore` and `ImmutablePath` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L926].
- **ADR-005**: Content-addressed storage (CAS) with filesystem reflink materialization [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L927].
- **ADR-006**: Dual BLAKE3 (internal) and SHA-256 (external provenance) hashing [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L928].
- **ADR-007**: SQLite relational storage for manifests with JSON export [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L929].
- **ADR-008**: Kaitai Struct (`.ksy`) as primary declarative binary specification format [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L930].
- **ADR-009**: Mandatory opaque span passthrough for unknown binary regions [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L931].
- **ADR-010**: Keystone Identity-Rebuild Gate as precondition for write capabilities [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L932].
- **ADR-011**: Dynamic capability derivation derived from automated tests [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L933].
- **ADR-012**: Permanent analysis-only designation for cryptographically signed payloads [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L934].
- **ADR-013**: Declarative recipes with semantic selectors and rebase engine [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L935].
- **ADR-014**: Bounded asset census during tiered scanning [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L936].
- **ADR-015**: Strict distinction between derived and user-composed screen layouts [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L937].
- **ADR-016**: Vendor-independent AI image provider abstraction [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L938].
- **ADR-017**: Egress airlock and single-network-crate confinement (`mmi-imagegen`) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L939].
- **ADR-018**: Pinned-blob CAS determinism for AI-generated assets [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L940].
- **ADR-019**: Conformance pipeline as mandatory admission gate preceding authoring [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L941].
- **ADR-020**: OS-native keychain secret storage [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L942].
- **ADR-021**: Declarative hardware target profile compatibility rules [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L943].
- **ADR-022**: Deterministic FAT32 media image generation [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L944].
- **ADR-023**: Clear deployment trust boundary and manual workflow enforcement [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L945].
- **ADR-024**: Large-file Git exclusion strategy [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L946].
- **ADR-025**: WebAssembly sandboxed plugin SDK trust model [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L947].
- **ADR-026**: Absolute zero-telemetry and offline-first policy [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L948].

---

## 48. Definition of Done

The Pass B planning phase is complete when [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L988]:
1. `PROJECT_PLAN.md` exists, covers all 48 required sections, and is derived strictly from Pass A evidence [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L42].
2. Every normative claim carries a valid evidence tag (`[EV:...]`, `[INF:...]`, `[UNK RQ-xxx]`) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L134].
3. `scripts/check-evidence-tags.py` passes with exit code 0 across all project documentation [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L146].
4. Scope conflict determination is permanently recorded (§1.3) [EV:doc:SOURCE_AUDIT.md#L103].
5. Implementation specifications are concrete and unambiguous, enabling implementation of Phase 1 without architectural redesign [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L991].
6. Zero application code beyond audit tooling and verification scripts has been written [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L994].
"""

output_path = Path("PROJECT_PLAN.md")
output_path.write_text(content.strip() + "\n", encoding="utf-8")
print(f"Generated {output_path} ({len(content.splitlines())} lines, {len(content)} bytes)")
