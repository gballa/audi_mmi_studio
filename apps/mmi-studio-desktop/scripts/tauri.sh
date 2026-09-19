#!/usr/bin/env bash
set -e

ACTION="${1:-build}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APP_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

if [ "$ACTION" = "build" ]; then
  echo "==> [1/2] Compiling React + Vite desktop frontend..."
  cd "$APP_DIR" && npm run build
  echo "==> [2/2] Compiling Native Tauri Desktop backend binary..."
  cargo build --manifest-path "$APP_DIR/src-tauri/Cargo.toml" --release
  echo "=========================================================="
  echo "✓ Native Audi MMI Studio Desktop binary ready:"
  echo "  $APP_DIR/../../target/release/mmi-studio-desktop"
  echo "=========================================================="
elif [ "$ACTION" = "dev" ]; then
  cd "$APP_DIR" && npm run dev
else
  echo "Usage: npm run tauri [build|dev]"
fi
