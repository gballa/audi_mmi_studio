# Test Infrastructure & Strategy Specification — Audi MMI Studio (2026 Navigation Pipeline)

**Document Version**: 1.0.0  
**Target Platform**: Audi MMI 3G+ (HN+) Harman/Becker Navigation Runtime  
**Status**: ACTIVE — OPAQUE-BOX VERIFICATION GATE  

---

## 1. Test Philosophy & Methodologies

### 1.1 Opaque-Box & Requirement-Driven Testing
Audi MMI Studio treats all map compiler subsystems, spatial indexers, format adapters, and media packaging engines as **opaque boxes**. Verification is conducted exclusively against:
1. **Observable External Interface Contracts**: Public API parameters, data serialization layouts, and return types defined in `PROJECT.md`.
2. **Physical Binary Invariants**: Sector alignment (544-byte physical pages), byte offsets, frame sync guards (`0x55AA55AA`), and CRC-16/CCITT checksum integrity.
3. **Partition & Media Boundary Constraints**: $2^{31}-1$ byte (2 GiB) per-volume file size ceiling, FAT32 allocation rules, and $\le 32\text{ GB}$ SDHC media capacity.
4. **Diagnostic & Cryptographic Invariants**: Software Version Management (SVM) Channel 15 XOR $51666$ (`0xC9D2`) adaptation ciphers and SHA-1 release checksums.

No test relies on internal private implementation details or mutable global state. Every test case is self-contained, creates its own temporary environment via `tempfile::tempdir()`, and guarantees zero leakage or mutation of the immutable `originals/` evidence corpus.

### 1.2 Formal Test Design Methodologies

The test suite is engineered using four formal software testing techniques:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         Test Design Methodologies                           │
├──────────────────────────┬──────────────────────────────────────────────────┤
│ Category-Partition       │ Equivalence class decomposition of inputs,       │
│ Method (Ostrand & Balcer)│ constraints, and error categories across formats.│
├──────────────────────────┼──────────────────────────────────────────────────┤
│ Boundary Value Analysis  │ Verification at and immediately adjacent to      │
│ (BVA)                    │ capacity, size, and arithmetic limits (2 GiB,    │
│                          │ 544B pages, 32-bit coordinates, empty sets).     │
├──────────────────────────┼──────────────────────────────────────────────────┤
│ Pairwise / Orthogonal    │ Combinatorial interaction coverage across OSM    │
│ Testing                  │ networks, Google Maps POIs, FLDB pages, SQLite   │
│                          │ Geographic.gdb, and SD media layouts.            │
├──────────────────────────┼──────────────────────────────────────────────────┤
│ Workload / Scenario      │ Full-scale operational workflows: Albania/WB     │
│ Testing                  │ micro-corridor compilation, 6/6 step in-car      │
│                          │ flashing simulation, and SVM fault elimination.  │
└──────────────────────────┴──────────────────────────────────────────────────┘
```

---

## 2. Feature Inventory & Mapping to Test Tiers

The 30 pipeline features defined in `PROJECT.md` are systematically partitioned into 4 distinct verification tiers:

| Feature # | Feature Name | Description | Test Tier | Primary Verification Method |
| :---: | :--- | :--- | :---: | :--- |
| **F1** | OSM PBF Ingestion & Parsing | Ingestion of road geometries, nodes, and ways | **Tier 1 / Tier 3** | Category-Partition (valid/invalid ways) |
| **F2** | FRC 0-7 Classification | Mapping OSM highway tags to Functional Road Classes 0-7 | **Tier 1** | Equivalence class tag matrix |
| **F3** | Lane Topology & Guidance | Turn lane guidance masks and arrow vectors | **Tier 1** | Bitmask validation (8-bit cluster mask) |
| **F4** | Relation Turn Restrictions | Prohibitive and mandatory turn restrictions | **Tier 1** | Graph penalty and movement gating |
| **F5** | Speed Limits & Defaults | Explicit maxspeed and statutory country defaults | **Tier 1** | Country fallback matrix validation |
| **F6** | Google Maps Places API (New) | Commercial POI lookup (EV charging networks) | **Tier 1 / Tier 3** | Schema conformance, CCS2/kW parsing |
| **F7** | Fuel Brands & Speed Radars | Fuel brand icon mapping and radar coordinates | **Tier 1** | Brand ID and approach vector validation |
| **F8** | Geocoding API Addresses | Municipal street centroids and rooftop geocoding | **Tier 1** | Precision ranking and coordinate bounds |
| **F9** | WGS84 Fixed-Point Projection | 32-bit signed integer Mercator coordinates | **Tier 1 / Tier 2** | Precision proof ($<1\text{ cm}$), bounds check |
| **F10** | Intermediate Representation | IrNode, IrEdge, IrTurnRestriction, IrPoi models | **Tier 1 / Tier 3** | Serialization roundtrip and struct contracts |
| **F11** | 3-Tier Regional Profiles | Micro AL/WB (~50MB), DACH (~6.2GB), ECE (~28.2GB) | **Tier 3 / Tier 4** | Workload scaling and memory ceiling |
| **F12** | FLDB Master Header | 36-byte header: magic FLDB, page_size 544, root 1 | **Tier 1 / Tier 2** | Header layout and magic discriminator |
| **F13** | FLDB 544-byte Physical Page | 512B payload + 32B ECC/CRC-16 + 0x55AA55AA trailer | **Tier 1 / Tier 2** | Byte stride, alignment, frame sync guard |
| **F14** | Multi-Volume Partitioning | ~28.19 GB split across 21 volumes ($\le 2\text{ GiB}$) | **Tier 1 / Tier 2** | Volume boundary rule ($\le 2,147,483,647\text{ B}$) |
| **F15** | 200 MiB Verification Chunks | CheckSumSize 209715200 chunk boundaries | **Tier 1 / Tier 2** | Multi-stage CRC-32 chunk integrity |
| **F16** | R-Tree Spatial Index | Morton Z-order curve, fanout 25, 5 levels | **Tier 1 / Tier 2** | Fanout capacity, depth calculation ($O(\log N)$) |
| **F17** | Routing Graph (GDB v37) | Magic 0xDEADBEEF, node/edge tables, version 37 | **Tier 1 / Tier 3** | Graph topology and header validation |
| **F18** | SQLite Geographic.gdb | Relational POI database with SQLite R*Tree table | **Tier 1 / Tier 3** | Schema validation, spatial queries |
| **F19** | MapStyles .xar Shaders | Regional archive (rax\0) day and night XML shaders | **Tier 1 / Tier 3** | Archive unpack, XML style tags |
| **F20** | SD Media Root Layout | Root: metainfo2.txt, HBNavDB/, MU9411/, MapStyles/ | **Tier 1 / Tier 3** | Directory tree hierarchy conformance |
| **F21** | metainfo2.txt Manifest | Release manifest with 2026_ECE release & SHA-1 hashes | **Tier 1 / Tier 2** | INI parser, 40-char SHA-1 validation |
| **F22** | SVM Error 03276 Resolver | Channel 15 XOR 51666 (0xC9D2) adaptation handling | **Tier 1 / Tier 4** | Mathematical bijection and roundtrip |
| **F23** | SVM Error 03175 Resolver | Green engineering menu +1/-1 parameter rehash | **Tier 1 / Tier 4** | Calibration state toggle simulation |
| **F24** | Emergency Rollback Script | stock_recovery.sh POSIX script for QNX UART/telnet | **Tier 1 / Tier 4** | POSIX shell syntax, mount rw, sync, reboot |
| **F25** | Pre-Flight Flashing Sim | Passes mmi-studio-cli simulate-update 6/6 steps | **Tier 1 / Tier 4** | 6-stage state machine completion |
| **F26** | CLI maps compile Command | Expose compiler in mmi-studio-cli maps compile | **Tier 1 / Tier 4** | CLI invocation, flags, exit status 0 |
| **F27** | Workstation GUI Integration | 🗺️ 2026 Map Studio compilation bridge | **Tier 3 / Tier 4** | IPC data structures, telemetry payloads |
| **F28** | 800x480 Preview Canvas | Interactive 800x480 navigation cluster preview | **Tier 1 / Tier 3** | Exact aspect ratio ($800\times 480$), pixel buffer |
| **F29** | Multi-Layer Toggling | Road, POI, Palette, and cluster overlay toggles | **Tier 1 / Tier 3** | Layer enable/disable filtering |
| **F30** | Compilation Telemetry | Real-time progress bars and stage logs | **Tier 1 / Tier 4** | Telemetry event sequence and completion |

---

## 3. Detailed Test Tier Architecture

### Tier 1: Feature Coverage (Core Requirements)
*Minimum Requirement*: $\ge 5$ test cases per requirement / feature.
- **FLDB 544-byte Page Alignment**: Master header fields, physical page size ($544\text{ B}$), 512B payload isolation, trailer sync word (`0x55AA55AA` / `0xAA55AA55`), Page 1 directory entry format (36B entries, 15 per page).
- **CRC-16 Header Verification**: CRC-16 CCITT (`0x1021`) calculation across bytes 16..527, initial value `0xFFFF`, byte-order preservation, detection of 1-bit flips, header offset `0x08` placement.
- **OSM Road Network & Speed Limits**: Highway classification FRC 0–7, lane guidance bitmask parsing, turn restrictions (prohibitive vs mandatory), explicit `maxspeed` conversion, country-specific default speed fallback matrix (DE, FR, IT, AL, AT, CH).
- **Google Maps Commercial POIs & EV Charging Stations**: Places API (New) field mask enforcement, EV charging network classification (CCS2, kW power rating tiers: Ultra-Fast $\ge 150\text{ kW}$, Rapid $50\text{–}149\text{ kW}$, Standard $< 50\text{ kW}$), fuel brand identification, fixed speed radar calibration, 30-day caching governance timestamp checks.
- **32-bit Fixed-Point Coordinates**: WGS84 Mercator conversion, coordinate scaling by $2^{31}-1$, sub-centimeter horizontal resolution verification ($< 1\text{ cm}$), longitude and latitude wrapping boundaries.
- **SD Card Structure**: Standard root layout hierarchy (`metainfo2.txt`, `HBNavDB/`, `MU9411/`, `MapStyles/`, `stock_recovery.sh`), FAT32 single-partition compatibility.
- **metainfo2.txt Parsing**: INI section validation (`[common]`, `[HBNavDB]`, `[MU9411]`, `[MapStyles]`), `release = "2026_ECE"`, 40-character SHA-1 hexadecimal hash syntax.
- **SVM Error Ciphers**: Channel 15 XOR $51666$ (`0xC9D2`) algorithmic correctness, mathematical reversibility ($((x \oplus 51666) \oplus 51666) == x$), Green Engineering Menu +1/-1 dataset rehash simulation.

### Tier 2: Boundary & Corner Cases (Stress & Fault Injection)
*Minimum Requirement*: $\ge 5$ test cases per feature.
- **Empty Datasets**: Zero-node/zero-edge OSM input, zero-POI dataset, empty IR tables, compilation handling without panics.
- **Zero-Length & Degenerate Roads**: Way with identical start/end nodes, zero-length geometry, self-intersecting loops, collinear segments, extreme coordinate spikes.
- **Max 2 GiB Volume Splits**: Exact boundary at $2,147,483,647$ bytes ($2^{31}-1$), off-by-one boundary test ($2^{31}-1$ vs $2^{31}$), multi-volume sequence numbering (.db, .gd2, .ATLAS), single-volume vs multi-volume threshold.
- **Invalid CRCs & Corrupted Pages**: Corrupted CRC-16 checksums, bit-flipped payload data, invalid page magic (e.g. `RIFF` instead of `FLDB`), truncated pages ($<544$ bytes), corrupted trailer sync words.
- **Missing & Corrupted metainfo2**: Absent `metainfo2.txt`, truncated file, missing `[common]` section, invalid release string, malformed checksums, corrupt line breaks, unsupported encoding.
- **200 MiB Verification Chunk Boundaries**: Files exactly $209,715,200$ bytes, $209,715,201$ bytes, and partial chunk verification.

### Tier 3: Cross-Feature Combinations (Pairwise Interactions)
Comprehensive interaction testing combining independent subsystem components:
- **OSM Vector Network + Google Maps POIs**: Spatial joining of commercial EV chargers and fuel brands onto OSM highway vertices via nearest-neighbor projection.
- **Vector Network + FLDB Compiler + SQLite Geographic.gdb**: Compilation of unified network into both 544-byte FLDB pages and SQLite R*Tree virtual tables with synchronized IDs.
- **FLDB Database + MapStyles .xar + metainfo2.txt Packaging**: SD media root packaging containing compiled `.db`, `.xar` day/night shaders, and computed SHA-1 release manifest.
- **R-Tree Index + GDB Routing Graph**: Spatial tile traversal indexing GDB version 37 routing nodes with transit gateways.
- **Layer Toggles + 800x480 Cluster Preview**: Canvas rendering pipeline responding to dynamic layer filtering (roads, POIs, radar warnings, day/night palette).

### Tier 4: Real-World Workload Scenarios (Full E2E Flows)
- **Scenario 1: Albania / Western Balkans 2026 Micro-Corridor Update**:
  Complete compilation of real-world 2026 infrastructure:
  * A1 Thumanë-Kashar Expressway (dual carriageway, 130 km/h, FRC 0).
  * Rruga e Arbrit Corridor (bypasses and mountain grades, FRC 1).
  * Llogara Tunnel (6 km tunnel bypass, FRC 1, tunnel flags).
  * Vlorë Bypass (coastal bypass, FRC 2).
  * Enriched EV charging hubs (Tirana, Durrës, Vlorë: CCS2, 150 kW).
  * Fixed speed radar warnings on A1 corridor.
- **Scenario 2: In-Car Flashing Simulation (6/6 Steps COMPLETED)**:
  Execution of `PreFlightSimulator::simulate_media` on compiled media:
  1. `MediaDetection`: Validate media directory presence.
  2. `MetaInfoParsing`: Validate `metainfo2.txt` release `2026_ECE`.
  3. `ChecksumVerification`: BLAKE3 stage normalization and SHA-1 verification.
  4. `ScriptExecution`: Hook detection.
  5. `PackageInstallation`: Simulated flash memory writing.
  6. `RebootPending`: Clean reboot trigger.
  Verdict: `overall_success: true`, `steps_successful: 6`, `steps_total: 6`, `final_state: UpdateState::Completed`.
- **Scenario 3: Post-Flashing SVM Diagnostic Harmonization**:
  Simulation of OBD-II diagnostics clearing ECU 5F fault codes:
  * Error 03276 cleared via Adaptation Channel 15 XOR $51666$.
  * Error 03175 cleared via Green Engineering Menu calibration toggle.
- **Scenario 4: Emergency Rollback Execution**:
  Validation of `stock_recovery.sh` POSIX shell commands:
  * Read-write remount of `/mnt/efs-system`.
  * Restoration of factory baseline files.
  * Flash synchronization (`sync`).
  * Clean reboot trigger (`shutdown -S`).

---

## 4. Test Runner Commands & CI Integration

All tests run **100% offline** and require zero external network dependencies:

```bash
# 1. Execute entire workspace test suite (including all unit & integration tests)
cargo test --offline --workspace

# 2. Execute dedicated E2E Map Pipeline test suite
cargo test --test e2e_map_pipeline --offline

# 3. Execute with verbose logging and full backtrace
RUST_BACKTRACE=1 cargo test --test e2e_map_pipeline -- --nocapture

# 4. Execute specific test tier
cargo test --test e2e_map_pipeline tier1_ --offline
cargo test --test e2e_map_pipeline tier2_ --offline
cargo test --test e2e_map_pipeline tier3_ --offline
cargo test --test e2e_map_pipeline tier4_ --offline

# 5. Full Workstation Offline Verification Gate
bash scripts/verify-workstation.sh
```

---

## 5. Coverage Thresholds & Quality Gates

```
Threshold Formula: Minimum Test Cases >= 11 * N + max(5, N / 2)
  - Core Feature Count (N): 8 core domains (FLDB alignment, CRC-16, OSM/speed,
    Google Maps POIs, Fixed-point WGS84, SD card layout, metainfo2, SVM ciphers).
  - Calculated Minimum Threshold: 11 * 8 + max(5, 4) = 88 + 5 = 93 tests.
  - Extended Feature Count (N = 10): 11 * 10 + 5 = 115 tests.
  - Target Implemented Suite: >= 115 comprehensive opaque-box test cases.

Quality Gates:
  - 100% Pass Rate: Zero failures, zero test skips.
  - Opaque-Box Fidelity: All assertions evaluate external binary structures or public contracts.
  - Immutability: originals/ must remain untouched (0 byte delta).
  - Offline Determinism: Zero external HTTP requests; all fixtures self-contained.
```
