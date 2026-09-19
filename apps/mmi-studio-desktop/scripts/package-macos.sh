#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DESKTOP_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
WORKSPACE_ROOT="$(cd "$DESKTOP_DIR/../.." && pwd)"

VERSION="0.1.0"
APP_NAME="Audi MMI Studio"
BUNDLE_IDENTIFIER="com.audi.mmi.studio"
DIST_DIR="$DESKTOP_DIR/dist-packages/macos"
APP_BUNDLE="$DIST_DIR/$APP_NAME.app"
DMG_NAME="Audi-MMI-Studio-$VERSION-darwin.dmg"
DMG_PATH="$DIST_DIR/$DMG_NAME"

echo "=========================================================="
echo " Packaging Audi MMI Studio for macOS ($VERSION)"
echo "=========================================================="

# 1. Build frontend
echo "==> [1/4] Building React + Vite frontend..."
cd "$DESKTOP_DIR"
npm run build

# 2. Build release binary
echo "==> [2/4] Compiling Rust release binary..."
cargo build --release --manifest-path "$DESKTOP_DIR/src-tauri/Cargo.toml"

BINARY_SRC="$WORKSPACE_ROOT/target/release/mmi-studio-desktop"
if [ ! -f "$BINARY_SRC" ]; then
  echo "Error: Binary not found at $BINARY_SRC"
  exit 1
fi

# 3. Assemble .app bundle
echo "==> [3/4] Assembling macOS Application Bundle ($APP_NAME.app)..."
rm -rf "$APP_BUNDLE"
mkdir -p "$APP_BUNDLE/Contents/MacOS"
mkdir -p "$APP_BUNDLE/Contents/Resources"

# Copy binary
cp "$BINARY_SRC" "$APP_BUNDLE/Contents/MacOS/mmi-studio-desktop"
chmod +x "$APP_BUNDLE/Contents/MacOS/mmi-studio-desktop"

# Copy icon
cp "$DESKTOP_DIR/src-tauri/icons/icon.icns" "$APP_BUNDLE/Contents/Resources/icon.icns"

# Write Info.plist
cat <<EOF > "$APP_BUNDLE/Contents/Info.plist"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>mmi-studio-desktop</string>
    <key>CFBundleIdentifier</key>
    <string>$BUNDLE_IDENTIFIER</string>
    <key>CFBundleName</key>
    <string>$APP_NAME</string>
    <key>CFBundleDisplayName</key>
    <string>$APP_NAME</string>
    <key>CFBundleIconFile</key>
    <string>icon.icns</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>$VERSION</string>
    <key>CFBundleVersion</key>
    <string>$VERSION</string>
    <key>LSMinimumSystemVersion</key>
    <string>11.0</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSSupportsAutomaticGraphicsSwitching</key>
    <true/>
</dict>
</plist>
EOF

# Write PkgInfo
echo "APPL????" > "$APP_BUNDLE/Contents/PkgInfo"

# Ad-hoc codesign for local execution
codesign --force --deep --sign - "$APP_BUNDLE" || true

# 4. Create DMG and Archive
echo "==> [4/4] Creating standalone DMG disk image..."
TMP_STAGING="$(mktemp -d /tmp/mmi_dmg.XXXXXX)"
cp -R "$APP_BUNDLE" "$TMP_STAGING/"
ln -s /Applications "$TMP_STAGING/Applications"

rm -f "$DMG_PATH"

hdiutil create -fs HFS+ -volname "$APP_NAME" \
  -srcfolder "$TMP_STAGING" \
  -ov -format UDZO \
  "$DMG_PATH"

rm -rf "$TMP_STAGING"

# Also create portable tar.gz archive
ARCHIVE_PATH="$DIST_DIR/Audi-MMI-Studio-$VERSION-darwin-app.tar.gz"
echo "==> Creating portable macOS archive ($ARCHIVE_PATH)..."
tar -czf "$ARCHIVE_PATH" -C "$DIST_DIR" "$APP_NAME.app"

echo "=========================================================="
echo "✓ macOS Packaging Complete!"
echo "  Application Bundle: $APP_BUNDLE"
echo "  Disk Image (DMG):   $DMG_PATH ($(du -sh "$DMG_PATH" | cut -f1))"
echo "  Portable Archive:   $ARCHIVE_PATH ($(du -sh "$ARCHIVE_PATH" | cut -f1))"
echo "  Size: $(du -sh "$DMG_PATH" | cut -f1)"
echo "=========================================================="
