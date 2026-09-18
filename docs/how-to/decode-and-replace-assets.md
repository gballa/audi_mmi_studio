# How-To: Decode and Replace UI Assets

This guide explains how to extract proprietary `.precomp` UI bitmaps to standard PNG, modify them in external design tools, conform the replacement to vehicle constraints, and verify the rebuild gate.

---

## Goal
Replace a stock MMI icon or gauge skin with a custom graphic while guaranteeing automotive hardware compatibility.

## Prerequisites
- Compiled `mmi-studio-cli`.
- Target `.precomp` asset (e.g. `originals/MU9411/ScreenLayouts/CombiStyles.precomp`).
- An external image editing tool (e.g., GIMP, Photoshop, Figma, ImageMagick).

---

## Procedure

### Step 1: Export Original Asset to PNG
Extract the proprietary compressed `.precomp` bitmap to standard PNG:
```bash
./target/release/mmi-studio-cli assets export \
  originals/MU9411/ScreenLayouts/CombiStyles.precomp \
  --output /tmp/combi_original.png
```

### Step 2: Edit the Graphic
Open `/tmp/combi_original.png` in your image editor.
- **Rule 1**: Maintain the exact original pixel width and height (e.g. 64x64).
- **Rule 2**: Keep RGBA color mode with alpha transparency.
- Save your edited image to `/tmp/combi_modified.png`.

### Step 3: Conform and Re-encode Replacement
Use `assets replace` to validate dimensions, resample if necessary using Lanczos3, and encode into `.precomp` format:
```bash
./target/release/mmi-studio-cli assets replace \
  --target originals/MU9411/ScreenLayouts/CombiStyles.precomp \
  --replacement /tmp/combi_modified.png \
  --output /tmp/CombiStyles_conformed.precomp
```
*Expected Output*:
```text
Conformed asset saved: /tmp/CombiStyles_conformed.precomp (exact match: true)
```

### Step 4: Verify with the Identity-Rebuild Gate
Ensure that replacing this asset does not violate signed bootloader protection:
```bash
./target/release/mmi-studio-cli verify-rebuild /tmp/CombiStyles_conformed.precomp
```
*Expected Output*: Exit code `0` (`Rebuild gate passed: Unsigned asset modification permitted`).
