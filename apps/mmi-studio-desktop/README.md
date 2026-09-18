# Audi MMI Studio — Desktop Application (`mmi-studio-desktop`)

`mmi-studio-desktop` is the graphical engineering workstation interface for **Audi MMI Studio**. It couples a high-performance Rust backend via **Tauri v2** with a reactive **React 18 / TypeScript** frontend.

The application operates under strict **offline isolation**: the webview runs exclusively from local origins (`tauri://localhost`) and all binary processing, format decoding, and canvas compositions execute in native Rust crates.

---

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Component Structure](#component-structure)
3. [Rust IPC Backend (`src-tauri`)](#rust-ipc-backend-src-tauri)
4. [How-To Guides](#how-to-guides)
   - [1. Running Backend IPC Tests](#1-running-backend-ipc-tests)
   - [2. Invoking IPC Handlers Programmatically in Rust](#2-invoking-ipc-handlers-programmatically-in-rust)
   - [3. Running the Frontend Development Server](#3-running-the-frontend-development-server)
   - [4. Building the Production Desktop Bundle](#4-building-the-production-desktop-bundle)
   - [5. Using Studio Panels (Hex, Asset, Canvas, Recipe, Typography)](#5-using-studio-panels)
5. [Security & Isolation Guarantees](#security--isolation-guarantees)

---

## Architecture Overview

```
apps/mmi-studio-desktop/
├── src-tauri/                              <-- Rust Native Backend (Tauri v2 bridge)
│   ├── Cargo.toml                          <-- Links to mmi-core, mmi-re-lab, mmi-formats, mmi-canvas, mmi-assets
│   ├── src/
│   │   ├── lib.rs                          <-- Library root & re-exports
│   │   └── ipc.rs                          <-- Strongly-typed IPC command handlers
│   └── tests/
│       └── desktop_tests.rs                <-- IPC roundtrip integration tests
├── src/                                    <-- React 18 / TypeScript Frontend
│   ├── components/
│   │   ├── HexViewer.tsx                   <-- Virtualized canvas-based hex inspector
│   │   ├── AssetBoard.tsx                  <-- Thumbnail gallery & palette swap inspector
│   │   ├── ScreenCanvas.tsx                <-- 800x480 pixel-grid MMI screen simulator
│   │   ├── RecipeStudio.tsx                <-- Visual declarative JSON theme recipe editor
│   │   └── TypographyStudio.tsx            <-- Multi-encoding string catalog & overflow checker
│   ├── App.tsx                             <-- Workspace layout & panel routing
│   ├── types.ts                            <-- IPC TypeScript interface definitions
│   └── main.tsx                            <-- React application entry point
├── package.json                            <-- Frontend dependencies (Vite + React)
└── index.html                              <-- Local HTML shell
```

---

## Component Structure

The user interface is organized into five specialized engineering panels:

1. **`HexViewer`**:
   - High-throughput virtualized rendering of arbitrary binary files.
   - Byte offset navigation, colored Shannon entropy bar, and ASCII representation.
   - Jump-to-offset and byte-pattern searching.

2. **`AssetBoard`**:
   - Visual catalog of decoded UI bitmaps (e.g. `.precomp` navigation icons, buttons, gauge styles).
   - High-DPI thumbnail preview backed by Content-Addressed Storage (CAS).
   - Dimension, bit depth, and color space constraint inspectors.

3. **`ScreenCanvas`**:
   - Interactive 800x480 MMI high-resolution screen simulator.
   - Day, Night, and Reduced palette switching.
   - Layer provenance visualization (background, chrome, active widgets).

4. **`RecipeStudio`**:
   - Visual builder for declarative JSON theme recipes (`mmi_recipe`).
   - Real-time rule validation against the target firmware schema.
   - One-click cross-train rebase analysis and drift detection.

5. **`TypographyStudio`**:
   - Multi-encoding localized string inspector (ASCII, UTF-8, UTF-16LE, EUC-JP).
   - Dynamic bounding box simulation against real TrueType fonts.
   - Real-time text clipping and overflow warnings.

---

## Rust IPC Backend (`src-tauri`)

The desktop backend provides four core IPC handlers in `src-tauri/src/ipc.rs`:

| IPC Handler | Rust Function | Description |
| :--- | :--- | :--- |
| `hexdump` | `handle_hexdump(path, offset, length)` | Returns formatted hex rows (`HexRow`) with byte offsets and ASCII characters. |
| `entropy` | `handle_entropy(path, window)` | Returns sliding Shannon entropy floats and region classifications (`Plaintext`, `Compressed`, `Encrypted`). |
| `inspect` | `handle_inspect_file(path, cas_base)` | Detects format, computes BLAKE3 hash, and generates CAS thumbnail. |
| `render_screen` | `handle_render_screen(mode)` | Synthesizes an 800x480 screen frame using `mmi_canvas` and returns pixel dimensions. |

---

## How-To Guides

### 1. Running Backend IPC Tests

Validate all Tauri backend IPC command handlers against genuine corpus files without launching a window:

```bash
# Run desktop crate unit and integration tests offline
cargo test -p mmi-studio-desktop --offline
```

Expected output:
```text
running 3 tests
test test_desktop_ipc_hexdump_and_entropy ... ok
test test_desktop_ipc_inspect_precomp_and_thumbnail ... ok
test test_desktop_ipc_render_screen_composition ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

### 2. Invoking IPC Handlers Programmatically in Rust

To use the desktop IPC bridge directly within Rust applications or test suites:

```rust
use mmi_studio_desktop::{handle_hexdump, handle_entropy, handle_inspect_file, handle_render_screen};
use std::path::Path;

// 1. Inspect a binary and generate thumbnail in Content-Addressed Storage
let cas_dir = tempfile::tempdir().unwrap();
let inspect_result = handle_inspect_file(
    "MU9411/ScreenLayouts/CombiStyles.precomp",
    cas_dir.path()
).unwrap();

println!("Detected Format: {}", inspect_result.detected_format);
println!("BLAKE3 Hash: {}", inspect_result.blake3_hash);

// 2. Query 128 bytes from offset 0x100
let dump = handle_hexdump("MU9411/HBNavDB/database.db", 0x100, 128).unwrap();
for row in dump.rows {
    println!("{:08X}: {}", row.offset, row.hex_bytes);
}

// 3. Compute sliding Shannon entropy
let entropy = handle_entropy("MU9411/HBNavDB/database.db", 512).unwrap();
println!("Average Entropy: {:.2} (Classification: {})", entropy.average_entropy, entropy.classification);

// 4. Render an 800x480 night-mode screen composition
let screen = handle_render_screen("night").unwrap();
println!("Rendered {}x{} screen in mode {}", screen.width, screen.height, screen.active_mode);
```

---

### 3. Running the Frontend Development Server

To run the React 18 frontend in browser-preview mode:

```bash
cd apps/mmi-studio-desktop

# Install local dependencies (if not cached)
npm install

# Start Vite development server
npm run dev
```

The frontend will be accessible at `http://localhost:5173`. In development mode without the Tauri shell, the UI gracefully uses local mock data for IPC calls.

---

### 4. Building the Production Desktop Bundle

To compile the TypeScript bundle and package the native desktop application:

```bash
cd apps/mmi-studio-desktop

# Build TypeScript assets
npm run build

# Package Tauri v2 application (requires tauri-cli)
cargo tauri build
```

---

### 5. Using Studio Panels

- **Hex & Entropy Investigation**:
  Select any file from the sidebar. The top entropy bar indicates structural transitions (e.g. green for plaintext metadata, purple for compressed assets, red for encrypted bootloaders).
- **Theme Recipe Authoring**:
  Open `RecipeStudio`, choose an existing recipe (`recipes/audi_sport_amber.json`), and modify color mappings. The live preview updates instantly.
- **Typography Proofing**:
  Enter translations in `TypographyStudio` and observe the bounding box indicator. Strings exceeding the allocated pixel width will highlight in red with exact overflow metrics.

---

## Security & Isolation Guarantees

In accordance with ADR-002 and `SECURITY.md`:

1. **Local Webview Origin**: The webview is strictly bound to `tauri://localhost`. Remote navigation and external scripts are prohibited by Content Security Policy.
2. **Offline by Default**: The application binds to no public network ports and transmits no telemetry.
3. **Egress Airlock**: External AI image generation requests must flow through the isolated `mmi-imagegen` crate with token isolation and strict brand/PII redaction.
4. **Signed Payload Protection**: Attempting to load or replace signed bootloader or IFS binaries triggers an immediate UI warning and is locked by the backend rebuild gate.
