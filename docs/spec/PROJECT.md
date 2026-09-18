# Project Master Document: Audi MMI Studio (Pass A Foundation)
# Working Directory: /Users/gerald/Antigravity/AudiMMI
# Timestamp: 2026-09-18T12:05:00Z
# Status: Pass A Complete · Awaiting User Determination on Scope Conflicts

---

## 1. Project Architecture & Pass A Baseline

Audi MMI Studio is an offline-first engineering workstation designed for structural analysis, reverse-engineering, visual exploration, controlled modification, and deterministic repackaging of lawfully held Audi Multi Media Interface (MMI) software and navigation updates [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L6].
The primary target corpus is the Harman Becker MMI 3G+ (HN+ / HN+R) European software trains and corresponding navigation map packages [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L4].

### Two-Pass Execution Protocol

The project is strictly governed by an immutable two-pass process [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L30]:
1. **Pass A — Evidence & Census Audit (Completed)**:
   - Greenfield verification confirming zero legacy application codebase [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L34].
   - Tiered scan of `originals/` across levels L0 to L4 [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L35].
   - Generation of normalized manifest database `originals-manifest.sqlite` and deterministic JSON export `originals-manifest.json` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L36].
   - Production of `SOURCE_AUDIT.md` (including § Scope Conflicts, § Signed Artefacts, and § Asset Census) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L37].
   - Creation of the research question register `docs/research/RQ-REGISTER.md` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L38].
   - Enforcement of strict evidence tagging discipline via `scripts/check-evidence-tags.py` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L48].
   - Halting execution for explicit user review of scope conflicts prior to Pass B [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L46].
2. **Pass B — Plan & Architecture (Pending User Decision)**:
   - Formulation of `PROJECT_PLAN.md` derived strictly from Pass A evidence, capped at ~2,000 lines [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L41].
   - Structural design of the Rust workspace, Tauri desktop UI, Kaitai Struct adapters, and Air-gapped AI asset pipeline [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L278].

---

## 2. Pass A Feature Inventory & Edge Cases

The table below catalogs all features and system behaviors discovered, implemented, and verified during Pass A in accordance with the Specification Miner protocol [EV:doc:ORIGINAL_REQUEST.md#L18]:

| Feature ID | Category | Feature Name | Description | Verification Status | Evidence Tag |
|---|---|---|---|---|---|
| F-001 | Repository Status | Greenfield Verification Engine | Verifies absence of pre-existing application code in repository root (§0.0) | Verified Greenfield | [EV:cmd@ls-la] |
| F-002 | Tiered Ingestion | L0 Filesystem Enumerator | Traverses `originals/` collecting relative paths, sizes, mtimes | Verified (24,661 files, 1,844 dirs) | [EV:survey@scan_originals.py] |
| F-003 | Identification | L1 Magic Sniffing & Classification | Inspects file headers and classifies files into 12 standard categories | Verified (100% classified) | [EV:survey@scan_originals.py] |
| F-004 | Fingerprinting | L2 Dual Streaming Hasher | Streams SHA-256 and native C BLAKE3 in a single read pass | Verified (63.97 GiB hashed in 105.19s) | [EV:survey@scan_originals.py] |
| F-005 | Archive Inspection | L3 Archive Table of Contents (TOC) | Extracts member lists for `.zip`, `.7z`, and `.iso` without disk extraction | Verified (34,608 entries cataloged) | [EV:survey@scan_originals.py] |
| F-006 | Bounded Sampling | L4 Header & Asset Sampler | Samples priority manifests, headers, font tables, and visual assets | Verified (under 256 MiB / 5,000 files limit) | [EV:survey@scan_originals.py] |
| F-007 | Asset System | Visual & Font Asset Census | Extracts dimensions, color types, alpha, and glyph counts for 444 assets | Verified (444 assets, 100% decode success) | [EV:survey@scan_originals.py] |
| F-008 | Manifest Database | SQLite Storage Manifest Generator | Populates normalized 8-table relational database `originals-manifest.sqlite` | Verified (33.07 MiB SQLite WAL) | [EV:survey@scan_originals.py] |
| F-009 | Manifest Export | Deterministic JSON Exporter | Serializes SQLite manifest to structured, immutable `originals-manifest.json` | Verified (43.81 MiB JSON) | [EV:survey@scan_originals.py] |
| F-010 | Legal & Scope | Scope Conflict Classifier | Classifies activation/licensing materials under §1.2 prohibitions | Verified (12 items classified) | [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L88] |
| F-011 | Security & Signing | Signed Artefact Gating Engine | Enforces `canEdit = NO` and `canRebuild = NO` on payloads with detached `.sig` | Verified (4 signed payloads gated) | [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L103] |
| F-012 | Research Tracking | Research Question (RQ) Ledger | Catalogs all unknown formats into `docs/research/RQ-REGISTER.md` | Verified (12 active RQs registered) | [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L402] |
| F-013 | Audit Documentation | SOURCE_AUDIT.md Assembler | Produces complete five-section source audit document | Verified (Complete 5 sections) | [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L37] |
| F-014 | Quality Assurance | Evidence Tag Compliance Checker | Validates `[EV:...]`, `[INF:...]`, and `[UNK]` syntax across Markdown docs | Verified (`scripts/check-evidence-tags.py`) | [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L146] |
| F-015 | Process Control | Pass A Execution Halt Gate | Halts execution after Pass A deliverables for explicit user decision | Verified (Awaiting user review) | [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L94] |

### Edge Cases Handled in Pass A

1. **System Metadata Exclusion**: Hidden operating system files (`.DS_Store`) were detected and filtered during L0 enumeration to prevent polluting manifest tables [EV:survey@scan_originals.py].
2. **Zero-Byte File Hashing**: Empty files (such as `License/upd`) correctly generated standard SHA-256 (`e3b0c44...`) and BLAKE3 (`af1349b...`) digests [EV:survey@scan_originals.py].
3. **Multi-Gigabyte Archive Inspection**: The 24.7 GiB 7-Zip container `8R0051884KL_6.36.0_2023.7z` was parsed in memory via `bsdtar` in under 1 second without performing destructive or space-consuming disk extraction [EV:survey@scan_originals.py].
4. **Proprietary Precomp Bitmap Decompression**: All 360 `.precomp` cluster graphics were verified to contain a 10-byte header followed by zlib-compressed 32-bit RGBA raw pixels (`width * height * 4` bytes) [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/CombiStyles/IND/0/default/arr_e.precomp].
5. **Complex Script Font Tables**: Linotype TrueType fonts (`AudiUnivers540Med-AGCC.ttf` and `LT_Univers440_88perc_Arabic.ttf`) were confirmed to contain 867 glyphs and OpenType `otlayout:arab` tables for bidirectional Arabic text shaping [EV:cmd@fc-scan-font3].
6. **Detached Signature Pairing**: Signed map packages (`MMI3GP_ECE_Hi_R_6_36_0.pkg`) and traffic configuration files (`TMCConfig.dat`) were linked to their respective 128-byte RSA signature files (`.pkg.sig`, `.dat.sig`), enforcing permanent analysis-only constraints [EV:survey@scan_originals.py].
7. **OEM Reference Distinctions**: Authentic OEM manifests listing supported FSC codes (`AudiFSC.txt`) were distinguished from aftermarket activation cracking patches (`2380_00040009.fsc`), preserving genuine reference data while gating circumvention payloads [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L83].

---

## 3. Milestones & Interface Contracts

The development roadmap is divided into structured milestones [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L854]:

```
Pass A (Evidence & Census) [COMPLETE]
  ├── Greenfield Verification (§0.0)
  ├── Tiered Scan (L0–L4) & Manifest Databases
  ├── Research Questions Register (docs/research/RQ-REGISTER.md)
  ├── Source Audit & Asset Census (SOURCE_AUDIT.md)
  └── Evidence Tag Compliance Verification (scripts/check-evidence-tags.py)
       │
       ▼ [USER DECISION GATE: Scope Conflicts & Signed Artefacts]
Pass B (Plan & Architecture) [PENDING USER DECISION]
  └── PROJECT_PLAN.md (~2,000 lines)
       │
       ▼
Phase 1: CAS Storage & SourceStore
Phase 2: Reverse-Engineering Laboratory Core
Phase 3: Format Research (Kaitai Struct Adapters)
Phase 4: Application Foundation (Tauri 2 + Rust Core)
Phase 5: Extraction & Normalisation
Phase 6: Asset Decode & Asset Board UI
Phase 7: Identity-Rebuild Gate Enforcement
Phase 8: Screen Canvas & Typography Renderer
Phase 9: Conformance Pipeline & Manual Asset Replacement
Phase 10: AI Asset Authoring (Gemini API Egress Airlock)
Phase 11: Localisation & Arabic Shaping
Phase 12: Recipe Engine & Rebase
Phase 13: Deterministic Rebuild Engine
Phase 14: Profile & Compatibility Rules Engine
Phase 15: Media Image Builder & Pre-Flight Simulator
Phase 16: Build Attestation & Stock Baseline Recovery
Phase 17: Plugin SDK & Sandboxing
Phase 18: Deployment Documentation
Phase 19: Hardening, Performance & Final Release
```

### Core Interface Contracts

1. **Corpus Immutability Contract**:
   - `originals/` is strictly read-only [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L160].
   - All derived artifacts and working stages reside under `.mmistudio/`, `workspace/`, or `output/` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L300].
2. **Signed Payload Contract**:
   - Payloads with adjacent `.sig` files have `canEdit = NO` and `canRebuild = NO` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L103].
   - Customization is strictly confined to the unsigned resource layer [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L117].
3. **Identity-Rebuild Gate (Keystone)**:
   - No format adapter may declare `canRebuild = YES` without proving that extract -> normalise -> rebuild yields byte-identical output to stock [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L368].
4. **AI Egress Airlock Contract**:
   - Network access is disabled by default and confined exclusively to crate `mmi-imagegen` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L558].
   - Egress deny-list prohibits transmission of any asset classified as activation, signed, or out-of-scope [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L567].
   - AI outputs must pass the five-stage Conformance Pipeline before admission into the project [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L588].

---

## 4. Code Layout Conventions

The workspace follows strict separation of concerns, ensuring that agent metadata remains isolated in `.agents/` and application code resides in designated directories [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L771]:

```
/Users/gerald/Antigravity/AudiMMI/
├── .agents/                      # Multi-agent coordination metadata ONLY
├── originals/                    # Immutable evidence repository (READ-ONLY)
├── docs/                         # Engineering documentation & research registers
│   └── research/
│       └── RQ-REGISTER.md        # Research Question Register
├── scripts/                      # Pass A audit & verification tooling
│   ├── blake3.c                  # Native C BLAKE3 streaming engine
│   ├── blake3_pure.py            # Pure Python BLAKE3 fallback implementation
│   ├── libblake3.dylib           # Compiled BLAKE3 shared library
│   ├── scan_originals.py         # Pass A tiered scanner & census engine
│   └── check-evidence-tags.py    # Evidence tagging compliance checker
├── crates/                       # (Phase 1–19) Rust backend crates
├── formats/                      # (Phase 3) Declarative Kaitai Struct (.ksy) definitions
├── rules/                        # (Phase 14) Declarative YAML compatibility rules
├── tests/                        # Integration and fuzzing suites
├── originals-manifest.sqlite     # Generated relational storage manifest
├── originals-manifest.json       # Generated deterministic JSON export
├── SOURCE_AUDIT.md               # Source audit report & asset census
├── PROJECT.md                    # Project master plan & feature inventory
├── AUDI_MMI_STUDIO_AGENT_PROMPT.md # Authoritative specification
└── ORIGINAL_REQUEST.md           # User project requirements
```

All source, tests, and data files must strictly reside in their designated workspace directories, never inside `.agents/` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L19].
