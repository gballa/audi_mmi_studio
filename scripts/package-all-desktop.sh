#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
DESKTOP_DIR="$REPO_ROOT/apps/mmi-studio-desktop"

echo "=========================================================="
echo " Audi MMI Studio — Multi-Platform Desktop Packaging Suite"
echo " Targets: macOS (.app/.dmg), Ubuntu (.deb), Windows (.exe)"
echo "=========================================================="

# 1. Generate Icons
echo "==> [1/4] Ensuring all platform icon assets are fresh..."
cargo run --bin generate_icons --manifest-path "$DESKTOP_DIR/src-tauri/Cargo.toml"

# 2. Package macOS
echo "==> [2/4] Packaging macOS Application & DMG..."
"$DESKTOP_DIR/scripts/package-macos.sh"

# 3. Package Linux / Ubuntu
echo "==> [3/4] Staging Linux / Ubuntu Debian & Tarball..."
"$DESKTOP_DIR/scripts/package-linux.sh"

# 4. Package Windows
echo "==> [4/4] Staging Windows NSIS & Portable ZIP..."
"$DESKTOP_DIR/scripts/package-windows.sh"

echo "=========================================================="
echo "✓ Multi-Platform Packaging Complete!"
echo "=========================================================="
echo "Artifacts generated in apps/mmi-studio-desktop/dist-packages/:"
echo "  🍏 macOS:   $(ls -lh "$DESKTOP_DIR/dist-packages/macos/"*.dmg 2>/dev/null | awk '{print $9, "(" $5 ")"}')"
echo "  🐧 Linux:   $(ls -lh "$DESKTOP_DIR/dist-packages/linux/"*.tar.gz 2>/dev/null | awk '{print $9, "(" $5 ")"}')"
echo "  🪟 Windows: $(ls -lh "$DESKTOP_DIR/dist-packages/windows/"*.zip 2>/dev/null | awk '{print $9, "(" $5 ")"}')"
echo "=========================================================="
