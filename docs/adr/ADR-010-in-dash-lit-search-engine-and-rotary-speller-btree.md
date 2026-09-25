# Architecture Decision Record (ADR-010)

**Document ID:** `ADR-010-in-dash-lit-search-engine-and-rotary-speller-btree-layout`  
**System:** Audi MMI Studio — 2026 Navigation Database Pipeline  
**Component:** `crates/mmi-formats`, `crates/mmi-rebuild`  
**Status:** ACCEPTED (Architectural Baseline)  
**Date:** 2026-09-24  

---

## 1. Context

In the Audi MMI 3G (`HNav`) and 3G+ (`HN+`) operating systems, in-vehicle destination entry (MMI rotary push-dial character input and speech dialogue address resolution) is executed by the QNX navigation search daemon (`vdev-logvolmgr` / `MMI3GApplication`).

Ground truth analysis of the reference 2023 update package (`originals/8R0051884KL_6.36.0_2023`) reveals:
1. **Physical Format Grounding**:
   - `pkgdb/LIT/` (MMI 3G High: `EJ211Ga_L1.db` through `L4.db`)
   - `pkgdb/LIT3GP/` through `LIT3GP5/` (MMI 3G+ Plus: `EJ211Pa_L1.db` through `L5.db`)
   - Every file strictly adheres to the **544-byte physical page stride** (`file_size % 544 == 0`), `FLDB` magic at offset `0x14`, `root_page = 1`, and CRC-16 CCITT payload checksums.
2. **The Rotary Speller Constraint**:
   - The vehicle rotary knob speller requires instant (<10 ms) character validation: as the operator selects letters ("T" $\to$ "I" $\to$ "R"), the system dynamically grays out invalid letters on the dial.
   - This capability cannot rely on linear scans or SQLite table queries across millions of European streets. It requires a **radix prefix tree (Trie)** embedded across 512-byte page payloads with candidate character bitmasks.
3. **Multi-Tier Layer Hierarchy**:
   - **Layer 1 (`L1`)**: Country and First-Order Administrative Boundary index.
   - **Layer 2 (`L2`)**: City / Municipality centroids and Postal Codes (ZIP).
   - **Layer 3 (`L3`)**: Roadway / Street name prefix Trie and phonetic normalization tables.
   - **Layer 4 (`L4`)**: House number ranges and street segment interpolation.
   - **Layer 5 (`L5`)** *(3G+ only)*: 3D junction views and POI cross-reference pointers.

---

## 2. Decision

We define and implement the native `LitSearchEngine` and compiler within `crates/mmi-formats/src/hb_lit.rs` and `crates/mmi-rebuild/src/lit_compiler.rs`:

```
                 ┌──────────────────────────────────────┐
                 │  OpenStreetMap Ingested IR Dataset   │
                 │  (IrNode, IrEdge, Street Names, POIs)│
                 └──────────────────┬───────────────────┘
                                    │
                                    ▼
                 ┌──────────────────────────────────────┐
                 │      LIT Hierarchy Partitioner       │
                 │  - Layer 1: Admin / Country Indices   │
                 │  - Layer 2: City / Postal Code Index │
                 │  - Layer 3: Street Speller Radix Tree│
                 │  - Layer 4: House Number Ranges      │
                 └──────────────────┬───────────────────┘
                                    │
                                    ▼
                 ┌──────────────────────────────────────┐
                 │    544-Byte Physical Page Builder    │
                 │  - Page 0: Master FLDB Header        │
                 │  - Page 1: Layer Directory & Metadata │
                 │  - Pages 2..N: Speller Trie Nodes    │
                 │  - CRC-16 CCITT (0x1021) Checksums   │
                 │  - Trailer Guard (0x55AA55AA)        │
                 └──────────────────┬───────────────────┘
                                    │
                                    ▼
                 ┌──────────────────────────────────────┐
                 │   Multi-Volume Disk Output (≤ 2 GiB) │
                 │   pkgdb/LIT3GP/EJ211Pa_L1.db         │
                 │   pkgdb/LIT3GP/LIT3GP.conf           │
                 └──────────────────────────────────────┘
```

### 2.1 Speller Radix Node Page Layout (512-Byte Payload)
Each B-Tree search node page fits within the 512-byte payload:
- **`0x00..0x02`** (2 bytes): `entry_count: u16` (number of branch edges in this node).
- **`0x02..0x06`** (4 bytes): `valid_alpha_mask: u32` (Bit 0..25 = A–Z availability; Bit 26 = Digits; Bit 27 = Space; Bit 28 = Diacritics).
- **`0x06..0x08`** (2 bytes): `node_flags: u16` (Terminal node flag, phonetic flag).
- **`0x08..0x10`** (8 bytes): `match_count: u64` (total street/city count under this subtree).
- **`0x10..0x10 + (N * 12)`**: Array of `LitSpellerBranch` entries:
  - `branch_char: u8` (ASCII character).
  - `reserved: u8`.
  - `match_subcount: u16` (subtree leaf count).
  - `child_page_idx: u32` (pointer to child FLDB page).
  - `entity_record_idx: u32` (pointer to data record if terminal).

### 2.2 Administrative & Street Record Structure
Street names are normalized (stripped of punctuation, uppercase ASCII with UTF-8 supplemental index) and referenced by sequential ID:
```rust
#[repr(C)]
pub struct LitStreetRecord {
    pub street_id: u32,
    pub city_id: u32,
    pub frc: u8,
    pub name_len: u8,
    pub name_ascii: [u8; 32],      // Truncated / fixed stride for direct offset lookup
    pub primary_edge_id: u32,      // Pointer into GDB routing graph
    pub centroid_x: i32,           // FixedPoint32 coordinate
    pub centroid_y: i32,
}
```

### 2.3 Volume Metadata Manifest (`LIT3GP.conf`)
Every generated LIT package includes a standard OEM `.conf` manifest:
```ini
UTF-8

[filedef]
name=LIT3GP_ECE
version=2026.01.0
type=LIT3GP
description="Audi MMI 3G+ High-Density Destination Search & Speller B-Tree"

[file]
name=EJ211Pa_L1.db
size={BYTE_EXACT_SIZE}
media=IsoImage
MD5={MD5_HASH}
checkcrc={CRC32_HEX}
[/file]
[/filedef]
```

---

## 3. Interface Definitions & Contracts

### Module: `crates/mmi-formats/src/hb_lit.rs`
```rust
//! hb_lit: Binary decoder and record models for Harman/Becker LIT search tables.

pub const LIT_MAGIC: &[u8; 4] = b"FLDB";
pub const LIT_ALPHA_MASK_A_Z: u32 = 0x03FF_FFFF; // Bits 0..25

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LitSpellerBranch {
    pub branch_char: u8,
    pub match_subcount: u16,
    pub child_page_idx: u32,
    pub entity_record_idx: u32,
}

#[derive(Debug, Clone)]
pub struct LitSpellerNode {
    pub page_idx: u32,
    pub valid_alpha_mask: u32,
    pub match_count: u64,
    pub branches: Vec<LitSpellerBranch>,
}

pub struct LitDatabaseReader<R: std::io::Read + std::io::Seek> {
    reader: R,
    header: HbNavDbHeader,
}

impl<R: std::io::Read + std::io::Seek> LitDatabaseReader<R> {
    pub fn open(reader: R) -> Result<Self, CoreError>;
    pub fn read_speller_node(&mut self, page_idx: u32) -> Result<LitSpellerNode, CoreError>;
    pub fn get_valid_next_chars(&mut self, prefix: &str) -> Result<u32, CoreError>;
}
```

### Module: `crates/mmi-rebuild/src/lit_compiler.rs`
```rust
//! lit_compiler: Assembles OpenStreetMap cities and street names into physical LIT pages.

pub struct LitCompiler {
    output_dir: std::path::PathBuf,
    writer: StreamingFldbWriter,
}

impl LitCompiler {
    pub fn new(output_dir: &std::path::Path, volume_name: &str) -> std::io::Result<Self>;
    pub fn compile_street_speller(&mut self, dataset: &IrDataset) -> std::io::Result<LitCompileSummary>;
    pub fn emit_manifest(&self, release: &str) -> std::io::Result<()>;
}

#[derive(Debug, Clone)]
pub struct LitCompileSummary {
    pub total_pages: usize,
    pub total_streets_indexed: usize,
    pub total_cities_indexed: usize,
    pub root_speller_mask: u32,
}
```

---

## 4. Non-Functional Constraints

1. **Rotary Speller Query Latency**:
   - Traversing the Radix Trie for any character prefix up to 20 letters must complete in **$\le 5\text{ physical page reads}$** (fan-out factor $\ge 16$).
2. **Page Stride & Integrity**:
   - Total file size must strictly satisfy: `file_size % 544 == 0`.
   - Trailer guard must equal `0x55AA55AA` on every written sector.
   - Page CRC-16 CCITT must validate without exception.
3. **Volume Split Boundary**:
   - Capped at `2,147,483,647 bytes` ($\le 2\text{ GiB}$) per volume file.

---

## 5. Consequences

- **Positive**: Resolves the **MEDIUM gap** in [`MAP_BUILD_GAP_ANALYSIS.md`](../MAP_BUILD_GAP_ANALYSIS.md). Replaces the SQLite `Geographic.gdb` placeholder with authentic 544-byte binary search tables compatible with the QNX in-dash speller.
- **Positive**: Guarantees zero dial lockup on Audi MMI rotary push dials during character entry.
- **Effort**: Requires building the Trie generator from OSM roadway names and city centroids during the compile pipeline.

---

## Next Steps for the Planner / Implementation Worker

1. Create `crates/mmi-formats/src/hb_lit.rs` with `LitSpellerNode` and reader logic.
2. Implement Trie construction and page emission in `crates/mmi-rebuild/src/lit_compiler.rs`.
3. Add unit tests in `crates/mmi-formats/tests/hb_lit_tests.rs` verifying speller mask generation and letter autocomplete.