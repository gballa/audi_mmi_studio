# ADR-005: Navigation Cartography FLDB Compiler and Multi-Volume Partitioning

## Status
ACCEPTED

## Date
2026-09-19

## Context
The Audi MMI 3G+ (`HN+` / `HN+R`) navigation system relies on proprietary Harman/Becker navigation databases stored in `/mnt/nav` (eMMC / HDD) and delivered via SD Media Updates (`8R0051884...`).
Grounded in reverse-engineering research (`docs/research/Audi MMI 3G+ Maps Research.md`):
- **Physical Page Stride**: The navigation database (`nav_data.db`) requires strict **544-byte physical pages** comprising a 16-byte page header, 512-byte payload, and 16-byte trailer sync guard (`0x55AA55AA`).
- **Checksum Verification**: Every page payload (bytes `16..528`) must be verified with CRC-16/CCITT (`0x1021` polynomial, `0xFFFF` init).
- **FAT32 Volume Splitting**: Filesystems formatted as FAT32 cannot handle files >= 2 GiB (or >= 4 GiB). The Audi MMI IPL splits databases exceeding 2 GiB into sequential volumes (`nav_data.db`, `nav_data.db.001`, `nav_data.db.002`, etc.).
- **Cartography Upgrades**: Upgrading maps with modern 2026 road networks (e.g. Albania & Western Balkans: Thumanë-Kashar highway, Rruga e Arbrit, Llogara Tunnel) requires converting modern vector geometries into this physical layout.
- **POI Enrichment**: Adding EV charging hubs and points of interest from Google Maps Platform must strictly comply with Terms of Service (no scraping, FieldMask filtering, 30-day cache eviction).

## Decision
Implement a native navigation cartography compilation and enrichment pipeline in `mmi-rebuild`:
1. **OSM Vector Ingestion (`osm_ingest.rs`)**:
   - Ingest OpenStreetMap PBF, XML, and GeoJSON formats into a strongly-typed Intermediate Representation (`IrDataset`).
   - Classify Functional Road Classes (FRC 0 to 7) matching automotive navigation routing graphs.
   - Enforce statutory speed limits by country code and parse turn lane masks and restrictions.
2. **Google Maps Platform Enrichment (`gmp_enrich.rs`)**:
   - Strictly adhere to Google Maps Platform ToS: zero raw vector copying; use Places API (New) solely to enrich POI attributes (EV connectors, commercial fuel brands, speed radar coordinates).
   - Enforce explicit `FieldMask` (`places.displayName,places.location,places.evChargeOptions,places.types`) to eliminate payload bloat.
   - Enforce automated 30-day cache eviction (`TOS_CACHE_MAX_AGE_SECONDS = 2,592,000`).
   - Support both live queries via API key and fixture-backed offline operation for 100% air-gapped workstations.
3. **Harman/Becker FLDB Compiler (`fldb_compiler.rs`)**:
   - Emit valid 544-byte physical pages with CRC-16/CCITT checksums and Morton Z-order spatial indexing.
   - Partition large datasets into sequential FAT32 volumes capped at 2 GiB - 1 byte (`MAX_VOLUME_BYTES = 2,147_483_647`).
   - Automated fault clearance: Resolve SVM error 03276 via Channel 15 XOR 51666 (`0xC9D2`) and error 03175 via calibration toggle.
4. **Headless & GUI Integration**:
   - CLI: `mmi-studio-cli maps compile` with `--region` (`AL`, `DACH`, `ECE`), `--osm-input`, and `--enable-gmp`.
   - GUI: `MapStudio.tsx` interactive panel with regional profile selection and live VCDS XOR calculator.

## Alternatives Considered

### Direct SQLite Database Injection Without FLDB Framing
- Pros: Easier to generate using standard database drivers.
- Cons: Incompatible with the Harman/Becker QNX navigation daemon (`vdev-logvolmgr`), causing instant mounting failure.
- Rejected: Fails head-unit compatibility requirements.

### Web-Based Online Vector Tile Streaming
- Pros: Eliminates local storage limitations.
- Cons: Audi MMI 3G+ hardware has no high-speed broadband modem or modern web engine capable of dynamic vector tile rendering.
- Rejected: Violates offline-first embedded constraints.

## Consequences
- Enables fully custom and updated 2026 navigation SD bundles compatible with genuine head units.
- Preserves vehicle stability with automated volume splitting and checksum verification.
- Enforces strict legal and technical compliance for third-party POI data enrichment.
