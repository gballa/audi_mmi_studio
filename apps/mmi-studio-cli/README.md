# Audi MMI Studio — Command Line Interface (`mmi-studio-cli`)

`mmi-studio-cli` is the headless engineering workstation binary for the **Audi MMI Studio** platform. It provides command-line interfaces for automotive binary reverse engineering, asset extraction, declarative theme recipe execution, cross-train rebasing, deterministic rebuild verification, and FAT32 SD media creation.

The CLI operates **100% offline** with zero remote network dependencies.

---

## Table of Contents

1. [Compilation & Installation](#compilation--installation)
2. [Command Reference](#command-reference)
3. [How-To Guides](#how-to-guides)
   - [1. Inspecting Unknown Firmware Binaries](#1-inspecting-unknown-firmware-binaries)
   - [2. Decoding and Replacing UI Graphics](#2-decoding-and-replacing-ui-graphics)
   - [3. Applying and Rebasing Theme Recipes](#3-applying-and-rebasing-theme-recipes)
   - [4. Rebuilding and Validating Staged Updates](#4-rebuilding-and-validating-staged-updates)
   - [5. Building FAT32 SD Media and Simulating Updates](#5-building-fat32-sd-media-and-simulating-updates)
   - [6. Generating Stock Rollback Bundles](#6-generating-stock-rollback-bundles)
   - [7. Third-Party Plugin Inspection & Verification](#7-third-party-plugin-inspection--verification)

---

## Compilation & Installation

Build the standalone release binary using the offline workspace toolchain:

```bash
# Build optimized release binary
cargo build --release -p mmi-studio-cli --offline

# Verify executable
./target/release/mmi-studio-cli --version
```

To view full CLI help:
```bash
./target/release/mmi-studio-cli --help
```

---

## Command Reference

| Subcommand | Description | Primary Options |
| :--- | :--- | :--- |
| `inspect` | Detect format, compute BLAKE3/SHA-256, extract metadata | `file`, `--json` |
| `hexdump` | Render virtualized hex dump with ASCII representation | `file`, `-o/--offset`, `-l/--length` |
| `entropy` | Compute sliding Shannon entropy and classify regions | `file`, `-w/--window`, `--json` |
| `carve` | Scan binary for embedded containers, file systems, and graphics | `file`, `--json` |
| `extract` | Ingest package into Content-Addressed Storage and stage | `source`, `-s/--stage`, `-o/--output-project` |
| `assets list` | Catalog visual assets and fonts in a stage | `-s/--stage`, `--json` |
| `assets export` | Decode proprietary asset (`.precomp`) to PNG | `asset`, `-o/--output` |
| `assets replace` | Conform replacement image to target constraints and save | `-t/--target`, `-r/--replacement`, `-o/--output` |
| `strings inspect` | Inspect localized multi-encoding string catalogs | `file`, `--json` |
| `strings check-overflow` | Test string bounding box against TrueType font metrics | `-t/--text`, `-f/--font`, `--max-width`, `--font-size` |
| `verify-rebuild` | Verify binary through Identity-Rebuild Gate & signature locks | `file`, `--json` |
| `render-screen` | Render composite 800x480 MMI screen layout to PNG | `-s/--screen`, `-m/--mode`, `-o/--output` |
| `ai-generate` | Generate asset via Egress Airlock (offline mock / keychain) | `-p/--prompt`, `-o/--output`, `--width`, `--height` |
| `recipe apply` | Execute declarative JSON theme recipe against staging | `-r/--recipe`, `-s/--stage`, `--json` |
| `recipe rebase` | Evaluate recipe portability & drift against target train | `-r/--recipe`, `-t/--target-train`, `--json` |
| `rebuild` | Deterministically repackage staged files | `-s/--stage`, `-o/--output`, `-v/--verify-against` |
| `validate` | Run 6-tier (L0-L5) automotive validation suite | `target`, `-p/--profile`, `-l/--level`, `--json` |
| `build-media` | Construct FAT32 SD media structure with volume manifests | `-s/--stage`, `-o/--output`, `--volume-label` |
| `simulate-update` | Execute pre-flight QNX head-unit update simulation | `media`, `--json` |
| `attest` | Generate cryptographic build attestation manifest | `-s/--source`, `-b/--build-dir`, `--source-train` |
| `stock-recovery` | Build stock original emergency recovery bundle & rollback script | `--baseline-train`, `-o/--output` |
| `plugins list` | List discovered third-party format adapter plugins | `-d/--dir`, `--json` |
| `plugins inspect` | Inspect plugin manifest, ABI version, and declared types | `plugin`, `--json` |
| `plugins verify` | Verify plugin sandboxing and execute format detection | `plugin`, `-t/--test-file`, `--json` |
| `maps compile` | Compile OpenStreetMap vector data & GMP POIs into FLDB media | `-o/--output`, `-r/--region`, `-i/--osm-input`, `--enable-gmp` |
| `firmware package` | Package full system firmware, NOR flash partitions, & scripts | `-o/--output`, `-t/--train`, `-r/--release`, `-v/--variant` |

---

## How-To Guides

### 1. Inspecting Unknown Firmware Binaries

To analyze an unidentified binary file from the `originals/` firmware corpus:

```bash
# 1. Inspect general format and cryptographic signatures
./target/release/mmi-studio-cli inspect originals/MU9411/HBNavDB/database.db

# 2. Compute entropy to detect compressed or encrypted streams
./target/release/mmi-studio-cli entropy originals/MU9411/HBNavDB/database.db --window 512

# 3. View the first 256 bytes in the hex viewer
./target/release/mmi-studio-cli hexdump originals/MU9411/HBNavDB/database.db --offset 0 --length 256

# 4. Scan the binary for embedded IFS, EFS, or graphic signatures
./target/release/mmi-studio-cli carve originals/MU9411/HBNavDB/database.db
```

---

### 2. Decoding and Replacing UI Graphics

To extract proprietary `.precomp` UI bitmaps to PNG, inspect them, and produce a conformed replacement:

```bash
# Step 1: Export proprietary .precomp graphic to standard PNG
./target/release/mmi-studio-cli assets export \
  originals/MU9411/ScreenLayouts/CombiStyles.precomp \
  --output /tmp/combi_original.png

# Step 2: Prepare a modified PNG (e.g. customized icon with new palette)
# Step 3: Conform replacement against the original file's dimensions and color space
./target/release/mmi-studio-cli assets replace \
  --target originals/MU9411/ScreenLayouts/CombiStyles.precomp \
  --replacement /tmp/combi_modified.png \
  --output /tmp/CombiStyles_conformed.precomp

# Step 4: Verify the replacement binary with the rebuild gate
./target/release/mmi-studio-cli verify-rebuild /tmp/CombiStyles_conformed.precomp
```

---

### 3. Applying and Rebasing Theme Recipes

Audi MMI Studio uses declarative JSON recipes to safely transform assets without manual binary hex-editing:

```bash
# Validate and execute a declarative theme recipe on a staging workspace
./target/release/mmi-studio-cli recipe apply \
  --recipe recipes/audi_sport_amber.json \
  --stage default

# Evaluate cross-train recipe drift against a target firmware release
./target/release/mmi-studio-cli recipe rebase \
  --recipe recipes/audi_sport_amber.json \
  --target-train originals/MU9411
```

The rebase report highlights:
- Exact asset matches across software trains
- Drift metrics (offset changes, checksum differences)
- Missing target assets or incompatible formats

---

### 4. Rebuilding and Validating Staged Updates

Once changes are staged, repackage them and execute the 6-tier (L0–L5) automotive validation gate:

```bash
# 1. Deterministically repackage staged files
./target/release/mmi-studio-cli rebuild \
  --stage default \
  --output output/candidate_rebuild \
  --verify-against originals/MU9411

# 2. Run the 6-tier validation suite (L0 header, L1 structure, L2 assets, L3 layout, L4 rebuild, L5 target profile)
./target/release/mmi-studio-cli validate output/candidate_rebuild
```

---

### 5. Building FAT32 SD Media and Simulating Updates

To generate SD card deployment media structured in compliance with MMI 3G update specifications:

```bash
# 1. Build FAT32 media folder with Metainfo2.txt and volume split manifests
./target/release/mmi-studio-cli build-media \
  --stage default \
  --output output/sd_deploy \
  --volume-label "MMI3G_NAV" \
  --volume-size-gb 32

# 2. Execute pre-flight QNX head-unit update simulation
./target/release/mmi-studio-cli simulate-update output/sd_deploy
```

The simulation tests:
- `metainfo2.txt` syntax and checksum matches
- Dependency ordering across modules
- QNX IFS/EFS partition boundary limits

---

### 6. Generating Stock Rollback Bundles

Before testing any update, produce an emergency rollback package directly from the stock baseline:

```bash
# Generate stock rollback bundle with UART flash scripts
./target/release/mmi-studio-cli stock-recovery \
  --baseline-train "HN+R_EU_AU_K0942_4_[8R0906961FB]" \
  --output output/stock_recovery_bundle
```

The generated bundle contains:
- `manifest.json`: Cryptographic SHA-256 hashes of all stock binaries.
- `emergency_rollback.sh`: Standalone POSIX shell script to restore stock files via QNX IPL / UART console.
- `STOCK_SHA256SUMS`: Verification hashes for every stock binary.

---

### 7. Third-Party Plugin Inspection & Verification

To inspect or verify custom format adapter plugins in `.mmistudio/plugins/`:

```bash
# List discovered plugins
./target/release/mmi-studio-cli plugins list

# Inspect ABI compatibility and format definitions
./target/release/mmi-studio-cli plugins inspect .mmistudio/plugins/sample-plugin

# Verify sandbox isolation against a binary sample
./target/release/mmi-studio-cli plugins verify \
  .mmistudio/plugins/sample-plugin \
  --test-file originals/MU9411/Speech/prompts.ans
```

---

### 8. Packaging Full System Firmware SD Bundles

To produce an end-to-end SD card bundle with QNX NOR flash partitions (`ifs-root.ifs`, `efs-system.efs`), Albanian localization, and SWDL block CRC32 manifests:

```bash
./target/release/mmi-studio-cli firmware package \
  --output output/firmware_sd_bundle \
  --train "HN+R_EU_AU_K0942_4" \
  --release "2026_ECE" \
  --variant "MU9411"
```

The output contains:
- `MU9411/ifs-root.ifs`: SH-4 QNX root partition with 2026 splash screen and HMI bytecode (checked against 43.74 MB NOR limit).
- `MU9411/efs-system.efs`: QNX F3S filesystem mounted at `/mnt/efs-system` containing Albanian language catalogs and GEM menus (checked against 38.8 MB limit).
- `HBNavDB/nav_data.db`: Native 544-byte physical pages with CRC-16.
- `metainfo2.txt`: Harman/Becker SWDL manifest with per-512KB CRC32 blocks.
- `copie_scr.sh`: Automatic SD card insertion launcher payload for `proc_scriptlauncher`.
- `finalScript`: Post-installation buffer flush and reboot script.
- `stock_recovery.sh`: Emergency NAND rollback for QNX UART serial console.

---

### 9. Compiling 2026 Navigation Maps (OSM & Google Maps Platform)

To compile modern OpenStreetMap cartography and enrich it with Google Maps Platform POIs:

```bash
# 1. Compile Albania & Western Balkans Micro profile
./target/release/mmi-studio-cli maps compile \
  --output /Volumes/MMI3G_NAV \
  --region AL \
  --release "2026_ECE"

# 2. Compile with Google Maps Platform POI enrichment (EV charging, fuel, radars)
./target/release/mmi-studio-cli maps compile \
  --output /Volumes/MMI3G_NAV \
  --region AL \
  --enable-gmp \
  --gmp-api-key "YOUR_API_KEY"
```

---

## Exit Codes

- `0`: Operation succeeded; all checks and validations passed.
- `1`: Validation failure, format violation, or rebuild mismatch.
- `2`: Invalid argument or missing required parameter.
- `3`: Safety lock violation (attempted modification of a signed bootloader/IFS payload).
