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
