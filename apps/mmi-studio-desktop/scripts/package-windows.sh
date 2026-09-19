#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DESKTOP_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
WORKSPACE_ROOT="$(cd "$DESKTOP_DIR/../.." && pwd)"

VERSION="0.1.0"
DIST_DIR="$DESKTOP_DIR/dist-packages/windows"
STAGE_DIR="$DIST_DIR/Audi-MMI-Studio-$VERSION-windows-x64"

echo "=========================================================="
echo " Packaging Audi MMI Studio for Windows ($VERSION)"
echo "=========================================================="

rm -rf "$STAGE_DIR"
mkdir -p "$STAGE_DIR"

# 1. Copy icon and docs
cp "$DESKTOP_DIR/src-tauri/icons/icon.ico" "$STAGE_DIR/icon.ico"
cat <<EOF > "$STAGE_DIR/README.txt"
Audi MMI Studio Workstation ($VERSION)
======================================
Offline engineering workstation for Audi MMI 3G/3G+ reverse engineering,
asset customization, navigation cartography compilation, and SD card packaging.

Launch mmi-studio-desktop.exe to start the workstation.
EOF

# 2. Check for binary
BINARY_SRC="$WORKSPACE_ROOT/target/release/mmi-studio-desktop.exe"
if [ -f "$BINARY_SRC" ]; then
  cp "$BINARY_SRC" "$STAGE_DIR/mmi-studio-desktop.exe"
else
  # Check if non-.exe binary exists (e.g. cross-platform dry-run)
  FALLBACK_BIN="$WORKSPACE_ROOT/target/release/mmi-studio-desktop"
  if [ -f "$FALLBACK_BIN" ]; then
    cp "$FALLBACK_BIN" "$STAGE_DIR/mmi-studio-desktop.exe"
  fi
fi

# 3. Compile NSIS installer if makensis is available
if command -v makensis >/dev/null 2>&1; then
  echo "==> Compiling NSIS installer..."
  cd "$SCRIPT_DIR"
  makensis installer.nsi
  echo "✓ NSIS Installer: $DIST_DIR/Audi-MMI-Studio-$VERSION-windows-setup.exe"
else
  echo "Notice: makensis not found on host. Installer will be compiled natively on Windows CI runner."
fi

# 4. Create portable ZIP archive
echo "==> Creating portable Windows ZIP archive..."
ZIP_PATH="$DIST_DIR/Audi-MMI-Studio-$VERSION-windows-x64.zip"
rm -f "$ZIP_PATH"
if command -v zip >/dev/null 2>&1; then
  (cd "$DIST_DIR" && zip -rq "$ZIP_PATH" "Audi-MMI-Studio-$VERSION-windows-x64")
  echo "✓ Portable ZIP: $ZIP_PATH"
fi

echo "=========================================================="
echo "✓ Windows Packaging Structure Ready!"
echo "  Staging Directory: $STAGE_DIR"
if [ -f "$ZIP_PATH" ]; then
  echo "  Portable Archive:  $ZIP_PATH ($(du -sh "$ZIP_PATH" | cut -f1))"
fi
echo "=========================================================="
