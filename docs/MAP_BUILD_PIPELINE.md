# Map Build Pipeline Architecture & Codebase Dependency Graph
**Target System:** Audi MMI 3G High & MMI 3G+ (HN+ / HN+R)  
**Implementation Modules:** `crates/mmi-rebuild/`, `crates/mmi-formats/`, `crates/mmi-core/`, `apps/mmi-studio-cli/`  
**Date:** 2026-09-21  

---

## 1. End-to-End Pipeline Dependency Graph

The diagram below maps the actual code path from raw OpenStreetMap geodata to the packaged SD card layout:

```mermaid
flowchart TD
    subgraph Data_Layer [1. Geodata Acquisition & Sources]
        PBF["data/sources/*.osm.pbf (OSM PBF Wire Format)"]
        XML["data/sources/*.osm / .xml (OSM XML Format)"]
        GeoJSON["data/sources/*.geojson (FeatureCollection)"]
        GMP["Google Maps Platform API / Offline Fixtures"]
        Manifest["data/manifests/*.json (Source Provenance)"]
    end

    subgraph Ingestion_Layer [2. crates/mmi-rebuild/src/osm_ingest.rs]
        ParserPBF["OsmPbfParser (zlib block decompression)"]
        ParserXML["OsmXmlParser (tag & relation extraction)"]
        ParserGeoJSON["OsmGeoJsonParser (LineString parser)"]
        
        Filter["is_navigable_way() & classify_frc() (FRC 0-7)"]
        Lanes["parse_turn_lanes() & lane_guidance_mask"]
        Speeds["statutory_fallback_speed() (DE, FR, IT, AL, AT, CH, EU)"]
        Turns["Turn Restriction Parser (from/via/to relations)"]
        
        PBF --> ParserPBF
        XML --> ParserXML
        GeoJSON --> ParserGeoJSON
        ParserPBF & ParserXML & ParserGeoJSON --> Filter
        Filter --> Lanes & Speeds & Turns
    end

    subgraph Intermediate_Representation [3. crates/mmi-rebuild/src/geo.rs]
        WGS84["Wgs84Point (Sub-centimeter fixed-point scaling: 2^31 - 1)"]
        Morton["FixedPoint32::morton_key() (64-bit Z-order bit interleaving)"]
        BBox["BoundingBox & RegionalProfile (AL, DACH, ECE)"]
        Enrich["GmpEnrichmentPipeline (EV Chargers, Fuel Brands, Speed Radars)"]
        
        Lanes & Speeds & Turns --> WGS84
        WGS84 --> Morton
        Morton --> BBox
        GMP --> Enrich
        BBox & Enrich --> IR["IrDataset (nodes, edges, restrictions, pois)"]
        IR --> Router["IrDataset::shortest_path_dijkstra()"]
    end

    subgraph Compilation_Layer [4. crates/mmi-rebuild/src/fldb_compiler.rs]
        PageBuilder["create_fldb_page() (544B page, 512B payload, 0x55AA55AA trailer)"]
        CRC["crc16_ccitt() (AUTOSAR Polynomial 0x1021)"]
        Splitter["split_into_volumes() (Strictly capped <= 2 GiB per volume)"]
        XAR["MapStyleXar (rax\\0 day and night shaders)"]
        ANS["HbAns (Albanian / Custom strings binary catalog)"]
        
        IR --> PageBuilder
        PageBuilder --> CRC
        CRC --> FLDB["compile_fldb_database() -> nav_data.db"]
        FLDB --> Splitter
    end

    subgraph Packaging_Layer [5. crates/mmi-rebuild/src/fldb_compiler.rs & apps/mmi-studio-cli]
        MetaGen["metainfo2.txt Generator (Per-chunk SHA-1 & release tags)"]
        Recovery["stock_recovery.sh (POSIX baseline recovery script)"]
        SDLayout["SD Root Assembly (HBNavDB/, MU9411/, MapStyles/)"]
        Provenance["build/manifest.json (Cryptographic provenance ledger)"]
        
        Splitter --> SDLayout
        XAR & ANS --> SDLayout
        SDLayout --> MetaGen
        SDLayout --> Recovery
        MetaGen & Recovery --> Provenance
    end

    subgraph Validation_Layer [6. Validation Engine]
        ValSrc["Source Validation (PBF check, SHA-256)"]
        ValGeo["Geographic Validation (BBox bounds, node counts)"]
        ValDb["Database Validation (544B page stride, CRC-16 checks)"]
        ValPkg["Package Validation (FAT32 bounds, directory structure)"]
        ValComp["Installation Compatibility (Official signature check)"]
        
        Provenance --> ValSrc --> ValGeo --> ValDb --> ValPkg --> ValComp
    end
```

---

## 2. Component Responsibility & Code Map

1. **`crates/mmi-rebuild/src/osm_ingest.rs`**:
   - `OsmIngestPipeline::ingest_pbf(&[u8], &OsmIngestConfig)`: Ingests raw PBF blobs, unpacks zlib streams, extracts nodes/ways/relations.
   - `classify_frc(highway: &str) -> Option<u8>`: Maps OpenStreetMap highway types to Harman/Becker Functional Road Classes 0 to 7.
   - `parse_turn_lanes(&tags) -> u16`: Converts `turn:lanes` pipe-delimited strings into 16-bit lane guidance bitmasks.
   - `statutory_fallback_speed(country, frc)`: Applies national statutory highway/urban defaults when `maxspeed` is missing.

2. **`crates/mmi-rebuild/src/geo.rs`**:
   - `Wgs84Point::to_fixed_point_32(&self)`: Converts latitude and longitude to 32-bit signed fixed-point integers ($2^{31}-1$).
   - `interleave_bits_32(x, y) -> u64`: Generates 64-bit Morton Z-order curve keys for multi-dimensional spatial clustering.
   - `IrDataset`: Intermediate representation containing nodes, directional edges, turn restrictions, and POIs.
   - `IrDataset::shortest_path_dijkstra()`: Evaluates graph connectivity and computes shortest paths across the topology.

3. **`crates/mmi-rebuild/src/fldb_compiler.rs`**:
   - `compile_fldb_database(&IrDataset) -> Vec<u8>`: Assembles master header, directory table, coordinate pages, and edge pages.
   - `create_fldb_page(page_idx, payload, magic)`: Formats discrete 544-byte physical disk pages with CRC-16 headers.
   - `split_into_volumes(&data, base_name)`: Partitions datasets exceeding 2 GiB into sequential volumes (`nav_data.db`, `nav_data.db.001`, etc.).
   - `FldbCompilerPipeline::compile_and_package()`: Emits the deployable SD card root hierarchy.

4. **`apps/mmi-studio-cli/src/commands/mod.rs`**:
   - `cmd_maps_compile()`: CLI handler binding OSM file reading, regional profiles, GMP enrichment, and compiler output.
