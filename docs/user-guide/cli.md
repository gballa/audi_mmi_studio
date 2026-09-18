# User Guide — Headless CLI (`mmi-studio-cli`)

`mmi-studio-cli` is the command-line entry point for **Audi MMI Studio**, offering 100% feature parity with the native Rust backend.

---

## 1. General Syntax & Global Flags

```bash
mmi-studio-cli [COMMAND] [OPTIONS]
```

### Global Flags
- `-h`, `--help`: Prints command help and syntax options.
- `-V`, `--version`: Prints toolchain and binary version.
- `--json`: Formats command output as structured JSON for scripting and automation.

---

## 2. Command Categories

### Binary Inspection & Reverse Engineering
- `inspect <file>`: Detects proprietary format, parses header fields, computes BLAKE3 and SHA-256 hashes, and checks signed status.
- `hexdump <file> [--offset <n>] [--length <n>]`: Formats a byte range with hexadecimal values and ASCII characters.
- `entropy <file> [--window <n>]`: Calculates sliding Shannon entropy (0.0 to 8.0) to identify plaintext, compressed streams, and encrypted code.
- `carve <file>`: Scans for embedded filesystem headers (IFS, EFS), image signatures, and archive tables.

### Asset & Typography Tooling
- `assets list [--stage <name>]`: Lists decoded graphics and font assets cataloged in staging.
- `assets export <asset_file> --output <png_path>`: Decodes proprietary `.precomp` UI bitmaps to standard PNG.
- `assets replace --target <orig> --replacement <new_img> --output <conformed>`: Validates replacement dimensions and encodes into target format.
- `strings inspect <catalog_file>`: Decodes multi-encoding string tables (ASCII, UTF-8, UTF-16LE, EUC-JP).
- `strings check-overflow --text "<txt>" --font <ttf> [--max-width <px>]`: Renders text bounds against TrueType font metrics to detect UI text clipping.

### Screen Composition & Visual Simulation
- `render-screen [--screen <id>] [--mode day|night|reduced] --output <png>`: Synthesizes a composite 800x480 screen frame with chrome and active widgets.

### Declarative Theme Recipes
- `recipe apply --recipe <file.json> [--stage <name>]`: Executes a non-destructive declarative theme recipe on a staging workspace.
- `recipe rebase --recipe <file.json> --target-train <dir>`: Compares recipe rules against an alternative software train to calculate drift and portability.

### Rebuild Gating & Validation
- `verify-rebuild <candidate_file>`: Evaluates a candidate update file against the Identity-Rebuild Gate and checks for signature locks.
- `rebuild [--stage <name>] --output <dir> [--verify-against <orig>]`: Repackages a staged tree with deterministic ordering.
- `validate <target_dir> [--profile <target_profile.json>] [--level <0..5>]`: Executes 6-tier automotive validation hierarchy.

### Hardware Deployment & Media
- `build-media [--stage <name>] --output <dir> [--volume-label <lbl>]`: Constructs FAT32 SD card media tree with `metainfo2.txt`.
- `simulate-update <media_dir>`: Simulates QNX head-unit update validation, dependency ordering, and partition boundaries.
- `attest --source <src> --build-dir <dir>`: Emits cryptographic SHA-256 build attestation manifest.
- `stock-recovery [--baseline-train <name>] --output <dir>`: Packages pristine original files with emergency UART recovery shell scripts.

---

## 3. Real-World Shell Example: Automated Theme Build Pipeline

```bash
#!/usr/bin/env bash
set -euo pipefail

# 1. Apply declarative theme recipe to default stage
./target/release/mmi-studio-cli recipe apply \
  --recipe recipes/rs_performance_red.json \
  --stage default

# 2. Deterministically repackage staged files
./target/release/mmi-studio-cli rebuild \
  --stage default \
  --output output/candidate_rs_theme \
  --verify-against originals/MU9411

# 3. Validate against 6-tier automotive gates
./target/release/mmi-studio-cli validate output/candidate_rs_theme

# 4. Build deployable SD card image
./target/release/mmi-studio-cli build-media \
  --stage default \
  --output output/sd_deploy_rs

# 5. Simulate head-unit installation
./target/release/mmi-studio-cli simulate-update output/sd_deploy_rs
```

For complete option-by-option reference tables, see the [CLI Reference](../reference/cli.md).
