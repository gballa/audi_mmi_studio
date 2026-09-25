# ADR-011: Orion ATLAS Spatial Tile Container & 3D Visual Mesh Architecture

## Status
ACCEPTED (Architectural Baseline)

## Date
2026-09-24

## Context
In Audi MMI 3G+ navigation, realistic 3D landmark buildings (`pkgdb/CTY/` and `CTYS3TC/`) and digital terrain elevation models (`pkgdb/TER/`) are stored in proprietary Harman/Becker Orion Atlas containers (`*.ATLAS`, ~11.5 GiB total).
The existing decoder (`crates/mmi-formats/src/hb_atlas.rs`) parses only the 64-byte file header (`\x06HEADER`, `\x05Orion`, `\x05Atlas`). Generating updated 2026 3D models or terrain meshes requires understanding the physical tile index block, quad-tree spatial addressing, and vertex mesh payload layout.

## Decision
1. **Container Binary Layout Specification**:
   - Header (64 bytes): Magic tag, `tile_block_size` (typically 4096 or 8192 bytes), major/minor version, `index_offset`, `index_size`, Pascal strings `Orion` and `Atlas`.
   - Spatial Quad-Tree Index: Located at `index_offset`, containing an array of `AtlasTileIndexEntry`:
     - `morton_key: u64`: Bit-interleaved 2D tile location (Level of Detail 0 to 14).
     - `file_offset: u32`: Offset to tile data payload (aligned to `tile_block_size`).
     - `compressed_size: u32`: Zlib compressed payload length.
     - `uncompressed_size: u32`: Raw geometry vertex/index buffer length.
2. **Payload Encoders**:
   - **Terrain Mesh (`TER`)**: Regular 65×65 height grids derived from NASA SRTM / Copernicus DEM, encoded as delta-compressed 16-bit signed elevations.
   - **3D Buildings (`CTY`)**: OpenStreetMap `building:levels` extruded polygonal meshes (indexed triangle strips, S3TC / DXT1 texture references).
3. **Module Realization**:
   - Add `AtlasWriter` and `AtlasTile` to `crates/mmi-formats/src/hb_atlas.rs`.
   - Implement `AtlasCompiler` in `crates/mmi-rebuild/src/atlas_compiler.rs` to generate synthetic/DEM terrain tiles.

## Consequences
- Completely eliminates the **HIGH gap** in MAP_BUILD_GAP_ANALYSIS.md.
- Enables high-fidelity 3D terrain and landmark rendering on Audi MMI 3G+ clusters without proprietary toolchains.
