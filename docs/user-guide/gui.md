# User Guide — Graphical Desktop GUI (`mmi-studio-desktop`)

`mmi-studio-desktop` is the graphical engineering workstation for **Audi MMI Studio**, built with **Tauri v2** and **React 18 / TypeScript**.

---

## 1. Launching the Desktop Application

### In Browser Development Mode
```bash
cd apps/mmi-studio-desktop
npm run dev
```
Open `http://localhost:5173` in your browser. The frontend launches with built-in mock handlers for all native IPC commands.

### In Native Desktop Mode (Tauri)
```bash
cd apps/mmi-studio-desktop
npm run tauri dev
```
This starts the native window backed by the compiled Rust `src-tauri` backend bridge.

---

## 2. Window Layout & Workspace Navigation

The desktop interface consists of:
1. **Top Control Bar**: Software train selector, active stage indicator, and global safety status badge (`BUILD READY — DEPLOYMENT NOT VERIFIED`).
2. **Left Navigation Sidebar**: Quick navigation between the five studio engineering panels:
   - **Hex Viewer**
   - **Asset Board**
   - **Screen Canvas**
   - **Recipe Studio**
   - **Typography Studio**
3. **Main Content Canvas**: The active panel's interactive viewport.
4. **Bottom Diagnostic Bar**: BLAKE3 hash, offset indicator, and CAS cache telemetry.

---

## 3. Engineering Panels

### Panel 1: Hex Viewer (`HexViewer.tsx`)
Designed for high-throughput exploration of multi-megabyte binary images:
- **Entropy Timeline**: Colored horizontal bar at the top representing regional Shannon entropy (Green = plaintext, Orange = structured tables, Purple = compressed assets, Red = encrypted payload).
- **Virtualized Grid**: Smoothly scrolls millions of bytes by only rendering visible rows.
- **Offset Jumper**: Jump directly to absolute hex offsets (e.g. `0x0004A200`).
- **Data Inspector**: Displays signed/unsigned 8-bit, 16-bit, 32-bit integers and strings at the selected cursor position.

### Panel 2: Asset Board (`AssetBoard.tsx`)
Visual exploration of decoded UI graphics:
- **Thumbnail Grid**: Displays icons, buttons, and backgrounds decoded from proprietary `.precomp` files.
- **Format Properties**: Inspects width, height, bit depth, and alpha channel status.
- **Side-by-Side Comparison**: Compare stock graphics against candidate theme replacements.

### Panel 3: Screen Canvas (`ScreenCanvas.tsx`)
Pixel-perfect simulation of the 800x480 high-resolution Audi MMI screen:
- **Day / Night / Reduced Mode Toggle**: Preview graphics in bright daylight mode, night amber/red illumination, or dimmed reduced display mode.
- **Layer Stacking**: Inspect base chrome, speedometer gauges, navigation compass arrows, and active UI menus.
- **Export Preview**: Export composite 800x480 screen frames directly to PNG.

### Panel 4: Recipe Studio (`RecipeStudio.tsx`)
Visual declarative theme authoring:
- **Recipe Editor**: Modify color replacement rules (e.g. replace Audi standard red `#FF0000` with Audi Sport Amber `#FFB300`).
- **Live Validation**: Immediate visual indicator for schema violations or target dimension mismatches.
- **One-Click Rebase**: Run cross-train drift analysis to check recipe portability to newer firmware trains.

### Panel 5: Typography Studio (`TypographyStudio.tsx`)
Localisation and string catalog inspection:
- **String Catalog Viewer**: Search and filter localized strings across English, German, French, and Asian character sets.
- **Bounding Box Overflow Simulator**: Test whether translated phrases exceed physical MMI display bounds before staging.
- **Warning Highlights**: Strings that clip or overflow turn bright red with exact overflow pixel measurements.

---

## 4. Desktop Shortcuts & Keybindings

| Shortcut | Action |
| :--- | :--- |
| `Cmd/Ctrl + 1` | Switch to Hex Viewer |
| `Cmd/Ctrl + 2` | Switch to Asset Board |
| `Cmd/Ctrl + 3` | Switch to Screen Canvas |
| `Cmd/Ctrl + 4` | Switch to Recipe Studio |
| `Cmd/Ctrl + 5` | Switch to Typography Studio |
| `Cmd/Ctrl + D` | Toggle Day / Night palette mode |
| `Cmd/Ctrl + S` | Export current view or active recipe |
