# How-To: Rebuild and Validate Staged Firmware

This guide explains how to repackage staged modifications into deterministic update containers and run the 6-tier (`L0`–`L5`) automotive validation hierarchy.

---

## Goal
Repackage modified assets deterministically and verify that candidate files satisfy all automotive safety and hardware constraints before media generation.

## Prerequisites
- Staged modifications in `.mmistudio/stages/default/`.
- Original baseline directory (`originals/MU9411`).
- Compiled `mmi-studio-cli`.

---

## Procedure

### Step 1: Deterministically Repackage Staged Files
Repackage the staged tree into candidate update files with deterministic lexicographical ordering and normalized timestamps:
```bash
./target/release/mmi-studio-cli rebuild \
  --stage default \
  --output output/candidate_rebuild \
  --verify-against originals/MU9411
```
*Expected Result*: Output confirms all files are repackaged and unmodified files match bit-for-bit with originals.

### Step 2: Run 6-Tier Automotive Validation
Execute the complete validation suite across all tiers:
```bash
./target/release/mmi-studio-cli validate output/candidate_rebuild
```

### Validation Tier Breakdown:
- **Level 0 (Magic & Header Integrity)**: Verifies valid magic bytes for all 12 formats.
- **Level 1 (Section Layouts)**: Checks offset tables and structural section lengths.
- **Level 2 (Asset Constraints)**: Validates graphic dimensions, bit depth, and RGBA color space.
- **Level 3 (UI Margins & Strings)**: Verifies text bounding boxes and layout boundaries.
- **Level 4 (Rebuild Determinism)**: Confirms bitwise parity against unmodified baseline and detects signed payload tampering.
- **Level 5 (Target Profile)**: Evaluates hardware profile constraints, partition size ceilings, and software train compatibility.

*Expected Verdict*:
```text
Validation Complete:
  L0 Magic: PASS
  L1 Structure: PASS
  L2 Assets: PASS
  L3 Margins: PASS
  L4 Rebuild: PASS
  L5 Profile: PASS
Status: BUILD READY — DEPLOYMENT NOT VERIFIED
```
