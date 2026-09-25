# Original User Request

## Initial Request — 2026-09-18T08:58:30Z

# Teamwork Project Prompt — Draft

> Status: Launched
> Goal: Craft prompt → get user approval → delegate to teamwork_preview
> Requested team: Use a full team of agents.

Execute Pass A (Evidence & Census Audit) and produce the foundation for Pass B of **Audi MMI Studio** in accordance with `AUDI_MMI_STUDIO_AGENT_PROMPT.md`, treating `originals/` as an immutable evidence repository.

Working directory: `/Users/gerald/Antigravity/AudiMMI`
Integrity mode: development

## Requirements

### R1. Greenfield Verification & Tiered Scan (Pass A)
Perform repository verification confirming greenfield status (§0.0). Conduct a tiered scan (L0 Enumerate, L1 Identify, L2 Hash SHA-256/BLAKE3, L3 Table of Contents without extraction, L4 Sample ≤256 MiB / 5,000 files prioritizing metadata, headers, font tables, and visual assets). Never modify or create files within `originals/`.

### R2. Storage Manifest & Findings Ledger
Generate `originals-manifest.sqlite` capturing file metadata, BLAKE3 and SHA-256 hashes, detected mime/container types, and relationships, accompanied by a JSON export. Catalog all unverified or ambiguous structures into `docs/research/RQ-REGISTER.md`.

### R3. Asset Census
Catalog all candidate visual and font assets across `originals/` during sampling, capturing path, module, container, detected codec, dimensions, bit depth, palette details, and decode status into the census section of `SOURCE_AUDIT.md`.

### R4. Scope Conflict & Signed Artefact Gating
Produce `SOURCE_AUDIT.md` containing:
- § Scope Conflicts: catalog activation/licensing materials (`6.22.4 Vlasoff maps activation`, `License/`), classifying each item (IN SCOPE / OUT OF SCOPE — DOCUMENT ONLY / AMBIGUOUS — USER DECISION REQUIRED) without implementing bypasses.
- § Signed Artefacts: list all payloads with detached or embedded signatures (e.g. `MMI3G_ECE_Hi_R_6_36_0.pkg`, `MMI3GP_ECE_Hi_R_6_36_0.pkg`), enforcing `canEdit = NO` and `canRebuild = NO`.
- Stop after Pass A for explicit user determination on scope conflicts before proceeding to Pass B (`PROJECT_PLAN.md`).

### R5. Evidence Tagging & Compliance Check
Enforce evidence tagging discipline (`[EV:...]`, `[INF:...]`, `[UNK]`) across all generated documentation and audit deliverables. Implement and run `scripts/check-evidence-tags.py` to ensure zero untagged normative statements exist.

## Acceptance Criteria

### Corpus Safety & Immutability
- [ ] `originals/` is strictly read-only; no files created, modified, renamed, or deleted within `originals/`.
- [ ] Hashes (SHA-256 and BLAKE3) captured and stored in `originals-manifest.sqlite`.

### Evidence Deliverables
- [ ] `SOURCE_AUDIT.md` generated with complete sections for Repository Status, Discovered Domains, § Scope Conflicts, § Signed Artefacts, and § Asset Census.
- [ ] `docs/research/RQ-REGISTER.md` created with initial research questions for every unknown container/format.
- [ ] `originals-manifest.sqlite` and exported JSON exist and match.

### Scope & Policy Guardrails
- [ ] Signed payloads marked analysis-only; zero write or rebuild paths specified for signed artefacts.
- [ ] Clear categorization of activation/licensing material without implementing DRM or activation bypasses.
- [ ] Tag verification script `scripts/check-evidence-tags.py` passes on all generated documentation.
- [ ] Execution stops after Pass A deliverables are complete for user review prior to Pass B plan formulation.

## Follow-up — 2026-09-19T12:10:20Z

Use a full team of agents.

Build an automated OpenStreetMap (OSM) and Google Maps Platform geodata ingestion, enrichment, and compilation pipeline that converts open-source vector networks and commercial POIs into native Audi MMI 3G+ (HN+) Harman/Becker FLDB (544-byte physical page) navigation databases covering the full European (ECE) territory (~29 GB multi-volume layout) ready for 100% functional in-vehicle SD card deployment.

Working directory: /Users/gerald/Antigravity/AudiMMI
Integrity mode: demo

## Requirements

### R1. Geodata Ingestion & Enrichment Pipeline (OSM + Google Maps Platform)
Ingest OpenStreetMap (OSM) road network geometry, lane topology, directional turn restrictions, and speed limits across Europe (ECE), while querying Google Maps Platform APIs (Places API & Geocoding API) for high-accuracy commercial POI lookup (EV charging networks, fuel brands, speed enforcement radar coordinates, and address point geocoding), projected into WGS84 Mercator coordinates.

### R2. Native Harman/Becker FLDB Compiler & Spatial Indexer
Compile transformed vector networks into native Audi MMI 3G+ navigation database structures:
- 544-byte physical disk page alignment with CRC-16 page header checksums.
- Multi-volume database partitioning (~29 GB total ECE coverage split across volumes as specified in `docs/research/Audi MMI 3G+ Maps Research.md`).
- R-Tree spatial index hierarchy and routing graph node/edge tables.
- SQLite `Geographic.gdb` database containing indexed POIs and city names.
- MapStyles `.xar` day and night shaders.

### R3. SD Media Packaging & SVM Error Prevention
Package the compiled database into the standard SD card root layout (`metainfo2.txt`, `HBNavDB/`, `MU9411/`, `MapStyles/`), including automated Software Version Management (SVM) adaptation handling and emergency rollback script (`stock_recovery.sh`) to eliminate errors `03175` and `03276`.

### R4. Workstation GUI & CLI Integration
Expose the compiler in both `mmi-studio-cli` (`mmi-studio-cli maps compile`) and the desktop workstation (`🗺️ 2026 Map Studio`), providing region filtering, interactive 800x480 preview, layer toggling, and real-time compilation telemetry.

## Acceptance Criteria

### Binary & Protocol Fidelity
- [ ] All compiled navigation database pages adhere strictly to 544-byte physical page boundaries with valid CRC-16 checksums.
- [ ] Spatial index and routing graph allow multi-country route calculation without memory faults.
- [ ] Multi-volume file split conforms to the ~29 GB ECE specification and fits on 32 GB SD cards.

### In-Car Flashing Simulation & Verification
- [ ] `mmi-studio-cli simulate-update` passes 6/6 steps (`COMPLETED`).
- [ ] Root `metainfo2.txt` contains valid file paths, version strings (`2026_ECE`), and SHA-1 checksums.
- [ ] Pre-flight verification confirms zero file corruption and valid emergency rollback scripting (`stock_recovery.sh`).

## 2026-09-24T20:52:06Z

Implement and apply the core architectural enhancements identified in the project analysis: full Harman/Becker GDB v37 routing graph compilation, physical automotive UDS diagnostic engine (ISO-TP/UDS over CAN/OBD-II), and bit-accurate QNX Neutrino IFS/F3S filesystem reconstruction.

Working directory: /Users/gerald/Antigravity/AudiMMI
Integrity mode: development

## Requirements

### R1. Automotive UDS & CAN Diagnostic Engine
Provide a complete automotive diagnostic communication layer implementing ISO-TP (ISO 15765-2) transport framing and UDS (ISO 14229-1) diagnostic services (Session Control `0x10`, Read/Write Data by Identifier `0x22`/`0x2E`, Security Access `0x27`, Clear Diagnostic Info `0x14`, Tester Present `0x3E`). Support physical/virtual adapters (SocketCAN and Serial ELM327/STN1170) alongside a high-fidelity loopback simulator, and automate SVM 03276 (Channel 15 XOR cipher) and 03175 fault clearance.

### R2. Harman/Becker GDB v37 Navigation Routing Graph Compiler
Build a native routing graph compiler translating ingested geographic networks into binary Harman/Becker GDB v37 database formats. The compiler must structure road topology across Functional Road Class (FRC 0-7) layers, incorporate turn restrictions and statutory speeds, generate Morton Z-curve spatial tile indexes, and frame outputs into native 544-byte physical pages with verified CRC-16/CCITT payload checksums and 2 GiB FAT32 multi-volume partitioning.

### R3. Bit-Accurate QNX Neutrino IFS and F3S Filesystem Synthesis Engine
Implement bit-accurate generators for QNX 6.3/6.5 Image Filesystems (`ifs-root.ifs`) and Flash Filesystems (`efs-system.efs`) targeting Renesas SH-4 architecture. Generated structures must include valid QNX Neutrino boot headers, directory inodes, and compressed payload blocks, strictly conforming to head-unit NOR flash partition bounds (max 43.75 MB for `ifs-root`, max 38.80 MB for `efs-system`).

### R4. Unified CLI, Desktop Integration & Verification Surface
Integrate the new diagnostic, routing, and filesystem engines into the existing CLI commands (`mmi-studio-cli obd`, `mmi-studio-cli maps build`, `mmi-studio-cli firmware package`) and desktop GUI panels. Maintain 100% offline determinism, clean compilation, and zero regressions across existing and newly added test suites.

## Acceptance Criteria

### Diagnostics & Vehicle Bridge (M5)
- [ ] ISO-TP framing correctly segments and reassembles multi-frame payloads up to 4095 bytes with Flow Control flow status, block size, and separation time parameters.
- [ ] UDS client executes standard diagnostic flows in both loopback and hardware adapter modes, including automated Channel 15 XOR cipher calculation (`0xC9D2`) for SVM Error 03276 resolution.
- [ ] CLI subcommand `mmi-studio-cli obd` supports physical serial/CAN device selection, diagnostics interrogation, and live fault clearance with structured JSON output.

### Cartography & GDB v37 Compiler (M6)
- [ ] GDB v37 compiler emits binary databases with magic `0xDEADBEEF`, version 37, FRC layer partitioning, and valid node-link topology.
- [ ] All generated database pages adhere strictly to the 544-byte physical page stride with valid 16-byte headers, CRC-16/CCITT checksums (`0x1021`), and sync guards (`0x55AA55AA`).
- [ ] `mmi-studio-cli maps build` integrates the GDB v37 output alongside FLDB, LIT3GP, and ATLAS artifacts in the final distribution layout.

### QNX Filesystem Reconstruction (M7)
- [ ] IFS builder produces valid QNX Neutrino images with machine type `0x0006` (Renesas SH-4), startup headers, and bootable directory layout.
- [ ] F3S builder produces valid `QSSL_F3S` flash filesystem structures with mount point definitions and file allocations.
- [ ] Hard partition boundaries are strictly enforced at build time, failing fast if `ifs-root` exceeds 45,875,200 bytes or `efs-system` exceeds 40,697,856 bytes.

### Build & Test Health
- [ ] `cargo check --workspace --offline` succeeds with zero errors.
- [ ] `cargo test --workspace --offline` passes with 100% success rate on both existing (132+ tests) and new unit/integration tests.
- [ ] Workstation verification script `./scripts/verify-workstation.sh` passes completely offline.
