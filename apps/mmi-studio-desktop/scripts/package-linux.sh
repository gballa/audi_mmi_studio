#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DESKTOP_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
WORKSPACE_ROOT="$(cd "$DESKTOP_DIR/../.." && pwd)"

VERSION="0.1.0"
APP_NAME="audi-mmi-studio"
PKG_NAME="audi-mmi-studio_${VERSION}_amd64"
DIST_DIR="$DESKTOP_DIR/dist-packages/linux"
STAGE_DIR="$DIST_DIR/$PKG_NAME"

echo "=========================================================="
echo " Packaging Audi MMI Studio for Linux / Ubuntu ($VERSION)"
echo "=========================================================="

mkdir -p "$STAGE_DIR/DEBIAN"
mkdir -p "$STAGE_DIR/usr/bin"
mkdir -p "$STAGE_DIR/usr/share/applications"
mkdir -p "$STAGE_DIR/usr/share/icons/hicolor/512x512/apps"
mkdir -p "$STAGE_DIR/usr/share/icons/hicolor/256x256/apps"
mkdir -p "$STAGE_DIR/usr/share/icons/hicolor/128x128/apps"
mkdir -p "$STAGE_DIR/usr/share/icons/hicolor/32x32/apps"

# 1. DEBIAN Control File
cat <<EOF > "$STAGE_DIR/DEBIAN/control"
Package: $APP_NAME
Version: $VERSION
Section: utils
Priority: optional
Architecture: amd64
Maintainer: Audi MMI Studio Engineers <support@audimmi.local>
Depends: libwebkit2gtk-4.1-0 | libwebkit2gtk-4.0-37, libgtk-3-0, libssl3 | libssl1.1
Description: Audi MMI 3G/3G+ Reverse Engineering & Cartography Workstation
 Offline engineering workstation for Audi MMI 3G/3G+ firmware decoding,
 asset customization, navigation cartography compilation, and SD card packaging.
EOF

# 2. Desktop Entry
cat <<EOF > "$STAGE_DIR/usr/share/applications/$APP_NAME.desktop"
[Desktop Entry]
Name=Audi MMI Studio
Comment=Audi MMI 3G/3G+ Reverse Engineering Workstation
Exec=/usr/bin/mmi-studio-desktop
Icon=audi-mmi-studio
Terminal=false
Type=Application
Categories=Development;Engineering;Utility;
StartupWMClass=audi-mmi-studio
EOF
chmod 644 "$STAGE_DIR/usr/share/applications/$APP_NAME.desktop"

# 3. Copy Icons
cp "$DESKTOP_DIR/src-tauri/icons/512x512.png" "$STAGE_DIR/usr/share/icons/hicolor/512x512/apps/$APP_NAME.png"
cp "$DESKTOP_DIR/src-tauri/icons/256x256.png" "$STAGE_DIR/usr/share/icons/hicolor/256x256/apps/$APP_NAME.png"
cp "$DESKTOP_DIR/src-tauri/icons/128x128.png" "$STAGE_DIR/usr/share/icons/hicolor/128x128/apps/$APP_NAME.png"
cp "$DESKTOP_DIR/src-tauri/icons/32x32.png" "$STAGE_DIR/usr/share/icons/hicolor/32x32/apps/$APP_NAME.png"

# 4. Check for binary
BINARY_SRC="$WORKSPACE_ROOT/target/release/mmi-studio-desktop"
if [ -f "$BINARY_SRC" ]; then
  cp "$BINARY_SRC" "$STAGE_DIR/usr/bin/mmi-studio-desktop"
  chmod 755 "$STAGE_DIR/usr/bin/mmi-studio-desktop"
else
  echo "Notice: target/release/mmi-studio-desktop not found locally (expected during Linux cross-packaging template assembly)."
fi

# 5. Build .deb if dpkg-deb is available
if command -v dpkg-deb >/dev/null 2>&1; then
  echo "==> Building Debian .deb package via dpkg-deb..."
  dpkg-deb --build --root-owner-group "$STAGE_DIR" "$DIST_DIR/${PKG_NAME}.deb"
  echo "✓ Debian Package: $DIST_DIR/${PKG_NAME}.deb"
else
  echo "Notice: dpkg-deb not present on host OS. Debian structure staged at $STAGE_DIR (compiled natively on Ubuntu CI runners)."
fi

# 6. Build portable tarball
echo "==> Creating portable Linux tarball..."
TARBALL_PATH="$DIST_DIR/Audi-MMI-Studio-$VERSION-linux-x86_64.tar.gz"
tar -czf "$TARBALL_PATH" -C "$STAGE_DIR" usr

echo "=========================================================="
echo "✓ Linux / Ubuntu Packaging Structure Ready!"
echo "  Staging Directory: $STAGE_DIR"
echo "  Portable Tarball:  $TARBALL_PATH"
echo "=========================================================="
