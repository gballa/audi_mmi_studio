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

## Milestones
| # | Name | Scope | Dependencies | Status |
|---|------|-------|-------------|--------|
| M1 | Geodata Ingestion & Enrichment | OSM ingestion, FRC classification, lane guidance, turn restrictions, speed limits, Google Maps Places/Geocoding enrichment, WGS84 fixed-point projection, IR data models, regional profiles | none | IN_PROGRESS |
| M2 | Native FLDB Compiler & Spatial Indexer | 544-byte FLDB pages, CRC-16 headers, multi-volume 2 GiB partitioner, R-Tree spatial index, GDB routing graph, SQLite Geographic.gdb, MapStyles .xar | M1 | PLANNED |
| M3 | SD Media Packaging & SVM Error Prevention | SD root layout, metainfo2.txt generator with SHA-1, SVM 03175/03276 adaptation handling, stock_recovery.sh, simulate-update 6/6 steps pass | M2 | PLANNED |
| M4 | Workstation GUI & CLI Integration | mmi-studio-cli maps compile, 🗺️ 2026 Map Studio desktop integration, 800x480 preview canvas, layer toggles, compilation telemetry | M3 | PLANNED |
| Final | Final E2E Test Pass & Adversarial Hardening | 100% pass across E2E test suite (Tiers 1-4), in-car simulation verification, and Tier 5 adversarial hardening | M1, M2, M3, M4 | PLANNED |

## Interface Contracts
### Geodata Ingestion (M1) ↔ FLDB Compiler (M2)
- Input: `IrDataset` struct containing:
  - `nodes: Vec<IrNode>`: 32-bit fixed-point coordinates (`x_coord: i32`, `y_coord: i32`, `elevation_m: i16`, `junction_flags: u8`).
  - `edges: Vec<IrEdge>`: (`edge_id: u32`, `from_node: u32`, `to_node: u32`, `length_dm: u32`, `frc: u8`, `speed_forward: u8`, `speed_reverse: u8`, `lane_count: u8`, `turn_lane_mask: u16`, `geometry: Vec<(i32, i32)>`).
  - `restrictions: Vec<IrTurnRestriction>`: (`from_edge: u32`, `via_node: u32`, `to_edge: u32`, `restriction_type: u8`, `penalty_s: u16`).
  - `pois: Vec<IrPoi>`: (`id: u32`, `name: String`, `category: String`, `lat: f64`, `lon: f64`, `brand: Option<String>`, `power_kw: Option<f32>`, `connectors: Vec<String>`).
- Invariant: All coordinates fixed-point scaled by $2^{31} - 1$ over $[ -180, 180 ]$ and $[ -90, 90 ]$.

### FLDB Compiler (M2) ↔ SD Media Packager (M3)
- Output layout in `output_stage/`:
  - `HBNavDB/nav_data.db`: FLDB 544-byte pages (`magic: FLDB`, `page_size: 544`, `root_page: 1`, CRC-16 CCITT `0x1021`, trailer `0x55AA55AA`).
  - `HBNavDB/*.pkg`: Multi-volume database chunks (each <= 2,147,483,647 bytes).
  - `HBNavDB/Geographic.gdb`: SQLite 3 database with `pois`, `categories`, `cities`, `streets`, and `poi_spatial_idx` R*Tree.
  - `MapStyles/styles_day.xar` & `styles_night.xar`: Valid `rax\0` archives with day/night mapstyle XML and textures.

### SD Media Packager (M3) ↔ CLI/GUI & Simulation (M4 & Final)
- SD Media Root:
  - `metainfo2.txt`: Valid INI format, section `[common]` with `release = "2026_ECE"`, sections `[HBNavDB]`, `[MU9411]`, `[MapStyles]`, valid 40-char SHA-1 checksums.
  - `stock_recovery.sh`: Executable POSIX shell script remounting `/mnt/efs-system` rw, restoring stock baseline, syncing flash, and rebooting.
  - `build_manifest.json`: Cryptographic bill of materials.
- Flashing Simulation:
  - `mmi-studio-cli simulate-update <media_dir>` returns `overall_success: true`, `steps_successful: 6`, `steps_total: 6`, `final_state: UpdateState::Completed`.

## Code Layout
- `crates/mmi-formats/src/hb_navdb.rs`: FLDB physical page generator and serializer.
- `crates/mmi-formats/src/hb_gdb.rs`: GDB version 37 routing graph serializer.
- `crates/mmi-formats/src/geographic_gdb.rs`: SQLite Geographic.gdb schema and builder.
- `crates/mmi-formats/src/mapstyle_xar.rs`: MapStyles `.xar` day/night packager.
- `crates/mmi-rebuild/src/osm_ingest.rs`: OSM PBF ingestion, lane topology, and turn restrictions.
- `crates/mmi-rebuild/src/gmp_enrich.rs`: Google Maps Platform Places API (New) & Geocoding API client.
- `crates/mmi-rebuild/src/fldb_compiler.rs`: High-level multi-volume compiler and spatial indexer.
- `crates/mmi-media/src/sd_packager.rs`: SD media packager, metainfo2 generator, SVM adaptation, and recovery scripts.
- `apps/mmi-studio-cli/src/commands/maps.rs`: CLI subcommand `maps compile`.
- `apps/mmi-studio-desktop/src/components/MapStudio.tsx`: Desktop Map Studio UI integration.
- `tests/e2e/`: Opaque-box E2E test suites (Tiers 1-4).
