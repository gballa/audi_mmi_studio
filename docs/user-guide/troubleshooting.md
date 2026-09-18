# User Guide — Troubleshooting

This document addresses common issues encountered when using **Audi MMI Studio**, their root causes, and recommended resolutions.

---

## 1. Problem: Command Fails with "Corpus is read-only" or `ERR_ORIGINALS_IMMUTABLE`

### Symptoms
Any command attempting to write directly to `originals/` terminates with exit code 1.

### Likely Cause
The workstation strictly enforces immutability on the reference firmware corpus (`originals/`). No operations are permitted to write to or modify `originals/`.

### Resolution
Always stage modifications into an isolated stage workspace using `--stage default` or output to an external directory:
```bash
# Correct usage: write to output/
./target/release/mmi-studio-cli rebuild --stage default --output output/rebuild_test
```

---

## 2. Problem: Rebuild Gate Fails with `ERR_SIGNED_ARTEFACT_IMMUTABLE` (Exit Code 3)

### Symptoms
The command output states that a candidate binary is locked because it contains a detached digital signature (`.pkg.sig`, `.dat.sig`) or bootloader header.

### Likely Cause
Audi MMI firmware signs critical system binaries (e.g. `ifs-root.ifs`, bootloaders). Modifying these binaries without factory private keys causes the head unit QNX IPL to enter an unrecoverable brick state. The Rebuild Gate locks these files permanently.

### Resolution
1. Do not attempt to modify signed components.
2. Confine theme customizations to unsigned asset containers (e.g. `ScreenLayouts/CombiStyles.precomp`, skinning bitmaps).
3. If an asset is incorrectly marked signed, verify its hash against `docs/audit/SOURCE_AUDIT.md`.

---

## 3. Problem: Asset Conformer Rejects Replacement Image

### Symptoms
`mmi-studio-cli assets replace` fails with:
`Error: Image dimensions (128x128) do not match target constraints (64x64)`.

### Likely Cause
The automotive graphics rendering pipeline in the QNX MMI head unit relies on fixed framebuffer offsets and pre-allocated graphic memory pools. Arbitrary image dimensions cause memory corruption or hard system resets.

### Resolution
Resize the replacement image to match the original image dimensions exactly:
```bash
# Check original dimensions
./target/release/mmi-studio-cli inspect originals/MU9411/ScreenLayouts/CombiStyles.precomp

# Resize replacement image using standard tools (e.g. imagemagick or sips)
sips -z 64 64 replacement.png --out replacement_conformed.png

# Re-run asset replacement
./target/release/mmi-studio-cli assets replace \
  --target originals/MU9411/ScreenLayouts/CombiStyles.precomp \
  --replacement replacement_conformed.png \
  --output /tmp/conformed.precomp
```

---

## 4. Problem: Desktop GUI Fails to Start Native Webview

### Symptoms
Running `npm run tauri dev` reports missing webview libraries on Linux.

### Likely Cause
Missing `webkit2gtk-4.1` development headers on the Linux workstation.

### Resolution
Install required system libraries:
```bash
# Debian / Ubuntu
sudo apt-get update && sudo apt-get install -y libwebkit2gtk-4.1-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel openssl-devel curl wget
```
Alternatively, run in browser preview mode:
```bash
npm run dev
```
