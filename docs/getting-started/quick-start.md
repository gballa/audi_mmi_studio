# Getting Started — Quick Start Guide

This 5-minute quick start guide demonstrates how to inspect genuine firmware files, profile binary entropy, export proprietary UI graphics to PNG, and validate theme recipes using `mmi-studio-cli`.

---

## Step 1: Inspect a Proprietary Binary

Audi MMI firmware contains proprietary containers such as `.precomp` and `.db`. Inspect a file to detect its format and compute its BLAKE3 hash:

```bash
./target/release/mmi-studio-cli inspect originals/MU9411/ScreenLayouts/CombiStyles.precomp
```

Expected Output:
```text
File: originals/MU9411/ScreenLayouts/CombiStyles.precomp
Size: 12480 bytes
BLAKE3: 4a8b...
Detected Format: Precomp Graphic (.precomp)
Signed Status: Unsigned (Modifiable)
```

---

## Step 2: Compute Sliding Shannon Entropy

Entropy profiling distinguishes uncompressed headers from compressed images or encrypted code:

```bash
./target/release/mmi-studio-cli entropy originals/MU9411/ScreenLayouts/CombiStyles.precomp --window 256
```

Regions with entropy ~7.9 indicate zlib-compressed graphic payloads, while low-entropy zones (<3.5) represent headers and padding.

---

## Step 3: Decode UI Graphic to Standard PNG

Extract proprietary `.precomp` UI bitmaps to a standard PNG image:

```bash
./target/release/mmi-studio-cli assets export \
  originals/MU9411/ScreenLayouts/CombiStyles.precomp \
  --output /tmp/combi_icon.png
```

You can now open `/tmp/combi_icon.png` in any standard image viewer to view the decoded gauge or icon graphic.

---

## Step 4: Validate a Theme Recipe

Validate a declarative JSON theme recipe against the workspace schema:

```bash
./target/release/mmi-studio-cli recipe apply \
  --recipe recipes/audi_sport_amber.json \
  --stage default
```

The recipe engine validates:
- Color mappings (`#FFB300`)
- Replacement rules against target dimensions
- Signed asset protection locks

---

## Step 5: Run Workstation Verification

Ensure all system components and evidence tags are functioning properly:

```bash
./scripts/verify-workstation.sh
```

Next steps:
- Read the [User Guide Overview](../user-guide/overview.md) to explore common engineering workflows.
- Browse the [How-To Library](../how-to/README.md) for step-by-step modification guides.
