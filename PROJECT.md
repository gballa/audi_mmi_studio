# Project: Audi MMI Studio — 2026 Navigation Database Pipeline
# Scope: Full European (ECE) Territory (~29 GB Multi-Volume Layout)

## Architecture
The system converts OpenStreetMap (OSM) vector networks and Google Maps Platform commercial POIs into native Audi MMI 3G+ (HN+) Harman/Becker FLDB (544-byte physical page) navigation databases and packaged SD update media.

```
                    ┌───────────────────────────┐
                    │   OpenStreetMap (OSM)     │
                    │   - Road Geometry         │
                    │   - Lane Topology         │
                    │   - Turn Restrictions     │
                    │   - Maxspeed & Stat Fallback│
                    └─────────────┬─────────────┘
                                  │
                                  ▼
┌───────────────────────┐   ┌───────────────────────────┐
│ Google Maps Platform  │──▶│ Ingestion & Enrichment    │
│ - Places API (New)    │   │ Pipeline (crates/mmi-core)│
│ - Geocoding API       │   │ - Fixed-Point WGS84       │
│ - EV Chargers / POIs  │   │ - Morton Z-Order Curves   │
└───────────────────────┘   └─────────────┬─────────────┘
                                          │
                                          ▼
                            ┌───────────────────────────┐
                            │ Harman/Becker FLDB        │
                            │ Compiler (crates/mmi-     │
                            │ formats & mmi-rebuild)    │
                            │ - 544-byte Physical Pages │
                            │ - CRC-16 Checksum Header  │
                            │ - Multi-Volume Split 2 GiB│
                            │ - R-Tree Spatial Index    │
                            │ - Routing Graph (GDB v37) │
                            │ - SQLite Geographic.gdb   │
                            │ - MapStyles .xar Shaders  │
                            └─────────────┬─────────────┘
                                          │
                                          ▼
                            ┌───────────────────────────┐
                            │ SD Media Packaging &      │
                            │ Attestation (mmi-media)   │
                            │ - metainfo2.txt (2026_ECE)│
                            │ - SVM 03175 / 03276 Ciphers│
                            │ - stock_recovery.sh       │
                            │ - 6/6 Step Simulator      │
                            └─────────────┬─────────────┘
                                          │
                    ┌─────────────────────┴─────────────────────┐
                    ▼                                           ▼
┌───────────────────────────────────────┐   ┌───────────────────────────────────────┐
│ mmi-studio-cli (CLI Integration)      │   │ 🗺️ 2026 Map Studio (Workstation GUI)  │
│ - mmi-studio-cli maps compile         │   │ - Interactive 800x480 Canvas          │
│ - Regional & ECE flags                │   │ - Layer Toggles & Day/Night Shaders   │
│ - Telemetry Streams                   │   │ - Real-Time Compilation Telemetry     │
└───────────────────────────────────────┘   └───────────────────────────────────────┘
```

## Feature Inventory
| # | Feature | Description | Milestone | Source |
|---|---------|-------------|-----------|--------|
| 1 | OSM PBF Ingestion & Parsing | Ingest OSM road network geometry & topology | M1 | survey |
| 2 | FRC 0-7 Classification | Map OSM highways to Functional Road Classes 0-7 | M1 | survey |
| 3 | Lane Topology & Guidance | Multi-lane arrow masks and turn lane vectors | M1 | survey |
| 4 | Relation Turn Restrictions | Prohibitive & mandatory turn restrictions (from/via/to)| M1 | survey |
| 5 | Speed Limits & Defaults | Explicit maxspeed and statutory country fallback matrix | M1 | survey |
| 6 | Google Maps Places API (New)| Commercial POI lookup (EV charging networks CCS2/kW) | M1 | survey |
| 7 | Fuel Brands & Speed Radars | Commercial fuel brands and speed camera geocodes | M1 | survey |
| 8 | Geocoding API Addresses | Municipal street centroids and rooftop geocoding | M1 | survey |
| 9 | WGS84 Fixed-Point Projection| 32-bit signed integer Mercator coordinates (<1 cm) | M1 | survey |
| 10 | Intermediate Representation| IrNode, IrEdge, IrTurnRestriction, IrPoi models | M1 | survey |
| 11 | 3-Tier Regional Profiles | Micro AL/WB (~50MB), Regional DACH (~6.2GB), ECE (~28.2GB)| M1 | survey |
| 12 | FLDB Master Header | 36-byte header with magic FLDB, page_size 544, root_page 1 | M2 | survey |
| 13 | FLDB 544-byte Physical Page| 512B payload + 32B ECC/CRC-16 header/trailer (0x55AA55AA) | M2 | survey |
| 14 | Multi-Volume Partitioning | ~28.19 GB split across 21 volumes, each <= 2 GiB | M2 | survey |
| 15 | 200 MiB Verification Chunks| CheckSumSize 209715200 chunk boundaries | M2 | survey |
| 16 | R-Tree Spatial Index | Morton Z-order curve, fanout 25, 5 levels for 50M links | M2 | survey |
| 17 | Routing Graph (GDB v37) | Magic 0xDEADBEEF, node/edge tables, transit gateways | M2 | survey |
| 18 | SQLite Geographic.gdb | Relational POI database with SQLite R*Tree virtual table | M2 | survey |
| 19 | MapStyles .xar Shaders | Regional archive (rax\0) day and night XML cartography | M2 | survey |
| 20 | SD Media Root Layout | Standard layout: metainfo2.txt, HBNavDB/, MU9411/, MapStyles| M3 | survey |
| 21 | metainfo2.txt Manifest | Release manifest with 2026_ECE release & SHA-1 checksums | M3 | survey |
| 22 | SVM Error 03276 Resolver | Channel 15 XOR 51666 (0xC9D2) adaptation handling | M3 | survey |
| 23 | SVM Error 03175 Resolver | Green engineering menu +1/-1 parameter rehash | M3 | survey |
| 24 | Emergency Rollback Script | stock_recovery.sh POSIX script for QNX UART / telnet | M3 | survey |
| 25 | Pre-Flight Flashing Sim | Passes mmi-studio-cli simulate-update 6/6 steps (COMPLETED)| M3 | survey |
| 26 | CLI maps compile Command | Expose compiler in mmi-studio-cli maps compile | M4 | survey |
| 27 | Workstation GUI Integration| 🗺️ 2026 Map Studio compilation bridge | M4 | survey |
| 28 | 800x480 Preview Canvas | Interactive 800x480 navigation cluster preview | M4 | survey |
| 29 | Multi-Layer Toggling | Road, POI, Palette, and cluster overlay toggles | M4 | survey |
| 30 | Compilation Telemetry | Real-time progress bars and stage logs | M4 | survey |
| 31 | 100% E2E Test Pass | Opaque-box Tier 1-4 tests passing cleanly | Final | survey |
| 32 | Adversarial Hardening | Tier 5 white-box challenger coverage hardening | Final | survey |
| 33 | ISO-TP Multi-Frame Transport | Segmentation & reassembly (SF, FF, CF, FC) up to 4095B | M5 | survey |
| 34 | UDS Diagnostic Services | Services 0x10, 0x22, 0x2E, 0x27, 0x14, 0x3E | M5 | survey |
| 35 | CAN & Serial Diagnostics Adapters | SocketCAN, Serial ELM327/STN1170, and LoopbackSimulator | M5 | survey |
| 36 | SVM 03276 & 03175 Fault Clearance | Channel 15 XOR 51666 cipher & green menu rehash workflows | M5 | survey |
| 37 | CLI obd Diagnostics Interrogation | Device selection, scan, fault clear, structured JSON | M5 | survey |
| 38 | GDB v37 Master Header & Topology | Magic 0xDEADBEEF, version 37, node & link tables | M6 | survey |
| 39 | FRC 0-7 Layers & Statutory Speeds | Road class partitioning, turn restrictions, speed fallback | M6 | survey |
| 40 | Morton Z-Curve Spatial Index | 32-bit fixed-point spatial tile indexing | M6 | survey |
| 41 | 544-byte Paging & CRC-16 Checksum | 16B header, 512B payload, CRC-16 0x1021, trailer 0x55AA55AA | M6 | survey |
| 42 | GDB Multi-Volume Partitioning | 2 GiB FAT32 bounds (EJ211_v37a.gdb, .gd2, .conf) | M6 | survey |
| 43 | CLI maps build GDB Integration | Integrates GDB v37 compiler into maps build pipeline | M6 | survey |
| 44 | QNX Boot Header & SH-4 Target | Machine 0x0006, startup_header, image_header | M7 | survey |
| 45 | QNX IFS Inodes & Compression | image_dirent, image_attr, 4KB page align, RAW/ZLIB/LZO | M7 | survey |
| 46 | Bit-Accurate QSSL_F3S Filesystem | unit_info_s, unit_logi_s, boot_info_s, /mnt/efs-system | M7 | survey |
| 47 | NOR Flash Hard Partition Limits | Fail fast if ifs-root > 45,875,200B or efs-system > 40,697,856B | M7 | survey |
| 48 | CLI firmware package Integration | Real IFS/F3S builders in firmware bundle & metainfo2 | M7 | survey |
| 49 | Unified CLI & Desktop Surface | Unified commands (obd, maps build, firmware package) & UI | M8 | survey |
| 50 | 100% Offline Workspace Verification| All existing 132+ and new tests pass, verify-workstation.sh | Final | survey |

## Milestones
| # | Name | Scope | Dependencies | Status |
|---|------|-------|-------------|--------|
| M1 | Geodata Ingestion & Enrichment | OSM ingestion, FRC classification, lane guidance, turn restrictions, speed limits, Google Maps Places/Geocoding enrichment, WGS84 fixed-point projection, IR data models, regional profiles | none | DONE |
| M2 | Native FLDB Compiler & Spatial Indexer | 544-byte FLDB pages, CRC-16 headers, multi-volume 2 GiB partitioner, R-Tree spatial index, GDB routing graph, SQLite Geographic.gdb, MapStyles .xar | M1 | DONE |
| M3 | SD Media Packaging & SVM Error Prevention | SD root layout, metainfo2.txt generator with SHA-1, SVM 03175/03276 adaptation handling, stock_recovery.sh, simulate-update 6/6 steps pass | M2 | DONE |
| M4 | Workstation GUI & CLI Integration | mmi-studio-cli maps compile, 🗺️ 2026 Map Studio desktop integration, 800x480 preview canvas, layer toggles, compilation telemetry | M3 | DONE |
| M5 | Automotive UDS & CAN Diagnostics Engine | ISO-TP framing (SF, FF, CF, FC <=4095B), UDS services (0x10, 0x22, 0x2E, 0x27, 0x14, 0x3E), SocketCAN & Serial ELM327/STN1170 + LoopbackSimulator, SVM 03276 & 03175 fault clearance, CLI `obd` command | none | IN_PROGRESS |
| M6 | Harman/Becker GDB v37 Compiler | GDB v37 binary format (0xDEADBEEF, v37), FRC 0-7 layers, turn restrictions, statutory speeds, Morton Z-curve, 544-byte pages (CRC-16 0x1021, trailer 0x55AA55AA), 2 GiB multi-volume partitioner, CLI `maps build` | none | IN_PROGRESS |
| M7 | Bit-Accurate QNX IFS & F3S Synthesis | QNX Neutrino SH-4 (0x0006) IFS builder, startup header, inodes, compression, QSSL_F3S flash filesystem, hard partition bounds (<=43.75MB, <=38.80MB), CLI `firmware package` | none | IN_PROGRESS |
| M8 | Unified CLI, Desktop Integration & Final Verification | CLI integration, desktop UI panels, 100% offline verification across workspace, verify-workstation.sh | M5, M6, M7 | PLANNED |
| Final | Final E2E Test Pass & Adversarial Hardening | 100% pass across E2E test suite (Tiers 1-4), verify-workstation.sh, and Tier 5 adversarial hardening | M5, M6, M7, M8 | PLANNED |

## Interface Contracts
### Diagnostics Engine (M5) ↔ CLI & Desktop (M8)
- Crate: `crates/mmi-diagnostics`
- Interfaces:
  - `CanAdapter` trait: `fn send(&mut self, frame: &CanFrame) -> Result<(), DiagError>`, `fn receive(&mut self, timeout: Duration) -> Result<CanFrame, DiagError>`.
  - `IsoTpTransport`: `fn send_payload(&mut self, payload: &[u8]) -> Result<(), DiagError>`, `fn receive_payload(&mut self, timeout: Duration) -> Result<Vec<u8>, DiagError>`.
  - `UdsClient`: `fn session_control(&mut self, session_type: u8) -> Result<Vec<u8>, DiagError>`, `fn read_did(&mut self, did: u16) -> Result<Vec<u8>, DiagError>`, `fn write_did(&mut self, did: u16, data: &[u8]) -> Result<(), DiagError>`, `fn security_access(&mut self, level: u8, key: &[u8]) -> Result<(), DiagError>`, `fn clear_dtc(&mut self, group: u32) -> Result<(), DiagError>`, `fn tester_present(&mut self) -> Result<(), DiagError>`.
  - SVM Clearance: `fn solve_svm_03276(challenge: u32) -> u32` (XOR 51666 / 0xC9D2), `fn solve_svm_03175() -> (u32, u32, bool)`.
  - CLI Output: `cmd_obd(port, baud, solve_svm, enable_gem, dry_run, as_json)`.

### GDB v37 Compiler (M6) ↔ Maps Pipeline (M8)
- Crate: `crates/mmi-formats/src/hb_gdb.rs`, `crates/mmi-rebuild/src/gdb_compiler.rs`
- Interfaces:
  - `HbGdbMasterHeader`: `magic: 0xDEADBEEF`, `version: 37`, `page_size: 544`, `page_count: u32`, `frc_offsets: [u32; 8]`, `morton_root: u32`.
  - Physical Page: 16-byte header (`page_id: u32`, `crc16: u16`, `flags: u16`, `payload_len: u16`, `reserved: [u8; 6]`), 512-byte payload, trailer `0x55AA55AA`.
  - `GdbCompiler`: `fn compile(&self, dataset: &IrDataset, output_dir: &Path) -> Result<GdbCompilationReport, FormatError>`.
  - Multi-Volume Output: `EJ211_v37a.gdb` (<= 2,147,483,647 B), `EJ211_v37a.gd2`, `.conf`.

### QNX Filesystem Engine (M7) ↔ Firmware Packager (M8)
- Crate: `crates/mmi-formats/src/qnx_ifs.rs`, `crates/mmi-formats/src/qnx_efs.rs`, `crates/mmi-rebuild/src/firmware_bundle.rs`
- Interfaces:
  - `QnxIfsBuilder`: `fn new(machine_type: 0x0006) -> Self`, `fn add_file(&mut self, path: &str, data: &[u8], mode: u32, compression: CompressionType) -> &mut Self`, `fn build(&self) -> Result<Vec<u8>, FormatError>`. Enforce max size 45,875,200 bytes.
  - `QnxEfsBuilder`: `fn new(unit_size: 262144, num_units: 148, mount_point: "/mnt/efs-system") -> Self`, `fn add_file(&mut self, path: &str, data: &[u8], mode: u32) -> &mut Self`, `fn build(&self) -> Result<Vec<u8>, FormatError>`. Enforce max size 40,697,856 bytes.
  - `FirmwareBundle`: writes `MU9411/ifs-root.ifs` and `MU9411/efs-system.efs`, verifies sizes against NOR flash limits, computes 512KB CRC32 blocks in `metainfo2.txt`.

## Code Layout
- `crates/mmi-diagnostics/`: New crate for ISO-TP transport, UDS client, SocketCAN, Serial ELM, and LoopbackSimulator.
- `crates/mmi-formats/src/hb_gdb.rs`: GDB version 37 binary database serializer and 544-byte physical page engine.
- `crates/mmi-formats/src/qnx_ifs.rs`: QNX Neutrino SH-4 bootable IFS generator with 64-byte startup headers and directory inodes.
- `crates/mmi-formats/src/qnx_efs.rs`: QNX F3S flash filesystem generator (`QSSL_F3S`) with erase units and mount points.
- `crates/mmi-rebuild/src/gdb_compiler.rs`: High-level GDB v37 multi-volume routing graph compiler.
- `crates/mmi-rebuild/src/firmware_bundle.rs`: Firmware packager integrating IFS/F3S builders and enforcing NOR flash limits.
- `apps/mmi-studio-cli/src/commands/obd.rs`: Expanded OBD / UDS diagnostic CLI command.
- `apps/mmi-studio-cli/src/commands/maps.rs`: Maps build command integrating GDB v37 compiler.
- `apps/mmi-studio-cli/src/commands/firmware.rs`: Firmware package command integrating IFS/F3S builders.
- `tests/e2e/`: Opaque-box E2E test suites (Tiers 1-4).

