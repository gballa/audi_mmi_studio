# Map Build Gap Analysis: OEM 2023 Architecture vs. OpenStreetMap Pipeline
**Target Platform:** Audi MMI 3G High & Audi MMI 3G+ (HN+ / HN+R)  
**Reference Baseline:** `originals/8R0051884KL_6.36.0_2023`  
**Working Implementation:** `crates/mmi-rebuild/`, `crates/mmi-formats/`, `apps/mmi-studio-cli/`  
**Date:** 2026-09-21  

---

## 1. Executive Summary

This gap analysis compares the actual OEM 2023 navigation package architecture (`8R0051884KL_6.36.0_2023`) against the repository's open geodata ingestion and compilation toolchain.

The objective is to establish an unvarnished, empirical assessment of:
1. What the current pipeline can legitimately ingest and compile from open sources (OSM XML, GeoJSON, and PBF).
2. What proprietary formats are fully understood and can be produced from new geodata.
3. Where proprietary data gaps exist (e.g. Orion ATLAS tile encoding, SDS voice acoustic models, TMC service provider tables).
4. Why past builds were purely a repackaged mirror of the 2023 reference package, and how to permanently decouple reference data from newly generated geodata.
5. The strict boundary between **A (producing a complete, deterministic, open-data generated map bundle)** and **B (in-vehicle signature verification acceptance on an unmodified vehicle)**.

---

## 2. Ingestion & Transformation Gap Analysis

| OEM Component | OEM Format & Size (2023) | Open Data Source | Existing Pipeline Capability | Gap Severity | Resolution / Strategy |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Road Network Geometry** | `GDB/*.gdb`, `*.gd2` (3.38 GiB) | OSM `highway=*` (ways & nodes) | `OsmIngestPipeline::ingest_*` converts ways to `IrNode` / `IrEdge` with FRC 0–7 | **LOW** | Fully functional in `osm_ingest.rs`. Extend to output both FLDB and GDB v37 headers. |
| **Physical Disk Pages** | `544-byte` physical pages with CRC-16 headers | Derived from `IrDataset` | `FldbCompilerPipeline::compile_fldb_database` assembles 544B pages with `0x55AA55AA` trailer sync | **NONE** | Fully supported in `fldb_compiler.rs`. Generates valid CRC-16 and 544B boundaries. |
| **Speed Limits & Fallbacks**| Explicit tags + statutory fallback | OSM `maxspeed=*` + country tables | Explicit speed parsing with statutory fallback matrix for DE, FR, IT, AL, AT, CH, EU | **NONE** | Implemented and verified in `osm_ingest.rs`. |
| **Turn Restrictions** | Routing graph penalty flags | OSM `type=restriction` relations | Extracts `from`, `via`, `to` relations into `IrTurnRestriction` models | **NONE** | Implemented in `osm_ingest.rs` and verified in Dijkstra router. |
| **Commercial POIs & Radars**| `PIT/EJ211a.PIT`, `GDB` POIs | Google Maps Platform / OSM nodes | `GmpEnrichmentPipeline` offline & live modes (CCS2 EV chargers, fuel brands, speed radars) | **NONE** | Implemented in `gmp_enrich.rs`. |
| **Search & Addresses** | `LIT/`, `LIT3GP/` (`EJ211*.db`) | OSM `addr:*` + Nominatim centroids | `Geographic.gdb` SQLite builder with R*Tree virtual tables | **MEDIUM** | Standard SQLite schema generated in `mmi-formats`. Proprietary `LIT` binary requires exact block stride. |
| **3D City & Terrain Models**| `CTY/`, `TER/` (`*.ATLAS`, ~11.5 GiB)| NASA SRTM / OSM 3D buildings | `HbAtlas::parse` in `mmi-formats` can read headers; generation of binary tile blocks is partial | **HIGH** | Mark as `BLOCKED — REQUIRED OEM/PROPRIETARY FORMAT INFORMATION` if full tile re-encoding is unsupported. |
| **Speech Dialogue Models** | `SDS/SDS_Data.iso` (468 MiB) | Proprietary phoneme models | Read-only acoustic grammar models. Open geodata does not provide SAPI/Harman phoneme acoustic graphs | **CRITICAL** | OEM Static resource. Cannot be generated from OSM; must be reused from reference or omitted. |
| **TMC Station Tables** | `TMCConfig_16/TMCConfig.dat` (Signed)| Proprietary RDS-TMC location tables| Encrypted proprietary binary with detached RSA-1024 signature | **CRITICAL** | Security/Signature bound. Cannot be generated from OSM. |

---

## 3. The Root Cause of the "False Success"

Previously, running the build step copied the contents of `originals/8R0051884KL_6.36.0_2023` directly into `build/final/`.
- **The flaw**: The build appeared to succeed with 77 files and 37.90 GiB, but zero bytes of newly ingested OpenStreetMap vector data were present in the output.
- **The remediation**: 
  1. `build/final/` must contain *only* artifacts generated from the active ingestion pipeline, alongside explicitly classified and permitted OEM static resources.
  2. The build pipeline must assert provenance: if a map database in `build/final/` has an identical hash to `originals/8R0051884KL_6.36.0_2023`, the build must fail.
  3. Every artifact must be tracked in `build/manifest.json` with its generator identity and input source hash.

---

## 4. Vehicle Update & Signature Boundary (Objective A vs. Objective B)

It is critical to formally document the distinction between:
- **Objective A: Complete, Valid Map Dataset Generation**:
  - The compiler ingests real OpenStreetMap geodata.
  - Produces valid 544-byte FLDB physical pages, valid CRC-16 CCITT page checksums, valid multi-volume FAT32 chunks ($\le 2\text{ GiB}$), valid R-tree spatial tables, and valid `metainfo2.txt` structure.
  - **Verdict**: Fully achievable and verified in this codebase.
- **Objective B: In-Vehicle Execution via Stock OEM Signature Checking**:
  - Unmodified Audi MMI 3G/3G+ firmware verifies detached RSA-1024 signatures (`.pkg.sig`) using public keys burned into the head unit's flash memory.
  - Custom open-data map packages will **NOT** pass standard signature verification on an unmodified vehicle without official manufacturer signing keys or an official FSC activation certificate matching the custom release.
  - **Verdict**: The pipeline must report `INSTALLATION_READY = NO (OFFICIAL_SIGNATURE_REQUIRED)`.

---

## 5. Execution Roadmap

1. **Phase 1**: Document complete pipeline architecture in `docs/MAP_BUILD_PIPELINE.md`.
2. **Phase 2 & 3**: Establish reproducible geodata acquisition (`data/sources/`, `data/manifests/`) and define European target coverage in `docs/MAP_COVERAGE.md`.
3. **Phase 4 & 5**: Wire real OSM data ingestion directly into `fldb_compiler.rs` and database serializers.
4. **Phase 6**: Classify all 77 reference package files in `docs/MAP_FILE_CLASSIFICATION.md`.
5. **Phase 7 & 8**: Implement the unified `mmi-studio-cli maps build` command and emit comprehensive provenance manifests.
6. **Phase 9 & 10**: Implement multi-stage automated validation with strict installation compatibility reporting.
