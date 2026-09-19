# Audi MMI Studio Workstation — Complete GUI & SD Deployment Guide

Welcome to the **Audi MMI Studio Workstation** user guide. This document provides step-by-step instructions for customizing Audi MMI 3G High / Plus (`HN+`) systems, translating system strings into **Albanian (`sq_AL`)**, injecting **2026 navigation network updates**, restyling UI elements with **Gemini Nano Banana AI**, compiling the complete update package, and deploying it to an SD card.

---

## Table of Contents
1. [Quick Start & Launching the GUI](#1-quick-start--launching-the-gui)
2. [Studio 1: 🖥️ Screen & Component Customizer](#2-studio-1-️-screen--component-customizer)
3. [Studio 2: 🍌 Gemini Nano Banana AI Element Modifier](#3-studio-2--gemini-nano-banana-ai-element-modifier)
4. [Studio 3: 🇦🇱 Localization & Albanian Translation Studio](#4-studio-3--localization--albanian-translation-studio)
5. [Studio 4: 🗺️ 2026 Map Studio & Road Network Injector](#5-studio-4-️-2026-map-studio--road-network-injector)
6. [Studio 5: 🚀 Build & SD Deploy Studio](#6-studio-5--build--sd-deploy-studio)
7. [Where is the Output Bundle?](#7-where-is-the-output-bundle)
8. [Step-by-Step SD Card Preparation & Vehicle Flashing](#8-step-by-step-sd-card-preparation--vehicle-flashing)
9. [Safety Policy & Emergency Recovery](#9-safety-policy--emergency-recovery)

---

## 1. Quick Start & Launching the GUI

### In Browser Development Mode (Recommended)
```bash
cd apps/mmi-studio-desktop
npm run dev
```
Open **[http://localhost:5173](http://localhost:5173)** in your browser. The application runs immediately with full interactive state, reactive previewing, and mock IPC fallbacks.

### In Standalone Native Desktop Mode (Tauri)
```bash
cd apps/mmi-studio-desktop
npm run tauri dev
```
Launches a native desktop window powered by the Rust backend bridge.

---

## 2. Studio 1: 🖥️ Screen & Component Customizer

The **Screen & Components** studio simulates the physical **800×480 @ 60 Hz** high-resolution Audi MMI 3G+ dashboard cluster.

### What You Can Do:
* **Switch Screen Views**:
  * **Navigation 3D**: Realistic perspective road geometry, active turn banner in Albanian/English, speed limit roundel (130 km/h), 3D compass rose, and ETA metrics.
  * **Audi Drive Select**: Rotary mode dial (*Regjimi Komod*, *Auto*, *Regjimi Dinamik Sportiv*, *Regjimi i Personalizuar*) with vehicle silhouette and suspension/steering status.
  * **Media Jukebox**: Internal HDD Jukebox audio player, acoustic waveform, track scrubber, and audio source tabs.
  * **Climate Control**: Dual-zone driver and passenger temperature dials, fan speed radial dial, and zone synchronization (`SYNC`).
  * **Main Carousel**: Iconic rotating arc menu with icons for Navigation, Telephone, Media, and Car Setup.
* **Customize Theme Colors**:
  * **Accent Color**: Presets for *Audi Sport Amber* (`#FF9900`), *RS Misano Red* (`#E0001B`), *Turbo Blue* (`#0077FF`), *Lime Green*, *Glacier White*, or custom Hex picker.
  * **Gauge Needle Color**: Amber, Red, Cyan, or White.
  * **Typography**: Switch between `AudiType Normal`, `AudiType Bold`, `AudiType Extended`, and `AudiUnivers 540 Medium` with dynamic font scaling slider (80% to 130%).
  * **Component Toggles**: Turn on/off Status Bar, Clock, Outside Temperature, 3D Compass Rose, and Ambient Neon Glow.
  * **Instant Language Toggle**: Switch between **🇦🇱 Shqip (`sq`)**, **🇬🇧 English (`en`)**, and **🇩🇪 Deutsch (`de`)** to immediately preview strings in real time.
* **Export Recipe**: Click **"Export Custom Theme & UI Recipe"** to generate an official §23 declarative recipe JSON.

---

## 3. Studio 2: 🍌 Gemini Nano Banana AI Element Modifier

The **Gemini AI Elements** studio provides side-by-side preview and AI-assisted restyling of MMI UI elements using Google's **Gemini Nano Banana** image models.

### Step-by-Step AI Asset Authoring:
1. **Select an Element from the Left Catalog**:
   * **Gauges**: Analog needle pointer, Speed limit roundel, 3D Compass rose.
   * **Navigation**: 3D maneuver turn arrow, destination pin.
   * **Vehicle**: Audi Drive Select aerodynamic silhouette (Sportback / RS Avant).
   * **Climate**: Rotary temperature glow ring, ventilation airflow glyphs.
   * **Menu Icons**: Navigation, Telephone, Media, Radio, Car, and Setup carousel icons.
   * **Textures**: Audi Sport 2×2 twill carbon fiber weave texture.
2. **Review the Live Comparison Stage**:
   * Inspect the **Stock Baseline Asset** on the left.
   * Inspect the **Gemini Nano Banana Restyled Asset** on the right.
   * Switch between **Side-by-Side**, **Before/After Split**, or **Alpha Mask** views.
3. **Select a Gemini Model**:
   * `gemini-3.1-flash-image` (Nano Banana 2 — Default, recommended for high-detail icons & textures)
   * `gemini-3.1-flash-lite-image` (Nano Banana 2 Lite — Ultra-low latency)
   * `gemini-2.5-flash-image` (Nano Banana Legacy)
4. **Choose or Enter a Prompt**:
   * Click one of the quick automotive presets (e.g. *"RS Crimson Sport with sharp glowing glass tip"*, *"Matte 2x2 Carbon Fiber Twill Weave"*), or type custom instructions.
5. **Pass the Mandatory Egress Airlock Gate (§12.3)**:
   * Click **"🍌 Generate with Gemini Nano Banana"**.
   * An **Egress Airlock Confirmation Modal** opens showing the exact asset dimensions, format (`RGBA8888`), model identifier, and proof of non-signed asset classification.
   * Click **"Confirm Egress & Generate"**.
6. **Apply to Screen & Build**:
   * Click **"Apply to Live Screen"** to immediately update the virtual cluster canvas.
   * The modified asset is automatically registered for inclusion in the SD card update build!

---

## 4. Studio 3: 🇦🇱 Localization & Albanian Translation Studio

The **Localization (sq_AL)** studio provides complete translation management across all Audi MMI functional areas with real-time text overflow checking.

### Key Capabilities:
* **System String Directory**: Covers Navigation, Media, Radio, Telephone, Car Setup, Climate, and Emergency/System Alerts.
* **Authentic Albanian Diacritics**: Integrated one-click buttons for inserting `ë`, `ç`, `Ë`, and `Ç`.
* **Hardware Bounding Box Overflow Verifier**:
  * Audi MMI displays allocate fixed pixel bounding boxes for labels.
  * The built-in Linotype/TrueType metric rasterizer measures text pixel widths dynamically.
  * If an Albanian phrase is too long, a bright **⚠ OVERFLOW WARNING** badge displays the exact excess pixels (e.g. `+24px`) so you can shorten the translation before flashing to hardware.
* **Live Screen Link**: Click **"👁️ Preview on 800x480 Screen"** to navigate directly to the screen view containing that string.
* **Export Catalog**: Click **"Export sq_AL Translation Catalog"** to download the compiled `audi_mmi3g_sq_AL_strings.json`.

---

## 5. Studio 4: 🗺️ 2026 Map Studio & Road Network Injector

The **2026 Map Studio** enables analyzing navigation database containers and injecting modern Western Balkans infrastructure.

### Technical Modifiability Verdict:
* **Can MMI Maps be modified?**: **YES**. Navigation geometry, turn restrictions, POI tables, and speed limits can be patched by recalculating page-level CRC-16 and CRC-32 checksums within the Harman FLDB 544-byte page container. Cartographic stylesheets are modifiable via `.xar` archives.
* **Firmware Notice**: Production vehicles require standard FEC activation code (`02100028`) or an SD patch script.

### 2026 Road Network Updates Included:
1. **A1: Thumanë - Kashar Expressway** (21.0 km, 130 km/h) — Bypasses Fushë-Krujë congestion.
2. **Rruga e Arbrit** (27.5 km, 90 km/h) — Murriz Tunnel bypass connecting Tirana to Klos & Bulqizë.
3. **SH8: Llogara Bypass Tunnel** (6.0 km, 80 km/h) — Connects Dukat to Palasë/Himarë in 7 minutes.
4. **Vlorë Coastal Bypass & Orikum Link** (29.0 km, 90 km/h) — Direct A2 highway extension.
5. **Korçë - Ersekë Modernized Highway** (35.0 km, 80 km/h) — Modernized plateau corridor.
6. **2026 Western Balkans Speed Radar Cameras** (180 fixed enforcement radar POIs).
7. **2026 Ultra-Fast EV Fast-Charging Hubs** (42 high-power 150kW–350kW CCS2 DC locations).
8. **2026 Updated Speed Limit Matrix** (130 km/h motorways, 110 km/h expressways, 90 km/h interurban).

### Compiling Map Updates:
* Check or uncheck desired infrastructure projects.
* Click **"⚡ Compile & Inject 2026 Map Update"** to run the 4-stage compilation pipeline with live console logs.

---

## 6. Studio 5: 🚀 Build & SD Deploy Studio

The **Build & SD Deploy** studio brings together your Albanian translations, Gemini AI assets, custom theme colors, and 2026 map updates into a single deployable SD card update bundle.

### How to Build:
1. Open the **🚀 Build & SD Deploy** tab.
2. Review the **Staged Package Components** card (shows counts of translated strings, staged map projects, and active theme).
3. Click **"⚡ Re-Compile Complete SD Update Media"**.
4. Watch the 6-stage build runner execute:
   * **Stage 1**: Normalizing and compiling `sq_AL.ans` binary string tables.
   * **Stage 2**: Packaging theme precomps with custom accent palette & Gemini AI assets.
   * **Stage 3**: Compiling 2026 road network vectors & FLDB 544-byte pages in `nav_data.db`.
   * **Stage 4**: Generating root `metainfo2.txt` release manifest and SHA-1 checksums.
   * **Stage 5**: Emitting cryptographic attestation (`build_manifest.json`) and `stock_recovery.sh`.
   * **Stage 6**: Running QNX Head-Unit Pre-Flight Simulator (verifies 6/6 steps pass).

---

## 7. Where is the Output Bundle?

All generated files are written directly to your workspace at:

```
/Users/gerald/Antigravity/AudiMMI/output/mmi3g_sd_card_update/
```

### Generated File Structure:
```text
output/mmi3g_sd_card_update/
├── metainfo2.txt               <-- Root Audi QNX update manifest (REQUIRED at SD root)
├── build_manifest.json         <-- Build provenance, BLAKE3 hashes & attestation
├── README_SD_CARD.txt          <-- On-card deployment guide
├── stock_recovery.sh           <-- Emergency baseline recovery script
│
├── MU9411/                     <-- MainUnit firmware partition
│   ├── strings/
│   │   ├── sq_AL.ans           <-- Compiled Albanian string catalog
│   │   └── sq_AL_catalog.json  <-- Diagnostic JSON string catalog
│   └── precomp/
│       └── theme_custom.precomp<-- Custom theme colors & Gemini AI assets
│
├── HBNavDB/                    <-- Navigation database partition
│   ├── 2026_albania_patch.pkg  <-- 2026 road network & POI patch
│   └── nav_data.db             <-- FLDB 544-byte physical pages
│
└── MapStyles/                  <-- Cartography vector stylesheets
    ├── styles_day.xar          <-- Day map shaders
    └── styles_night.xar        <-- Night map shaders
```

---

## 8. Step-by-Step SD Card Preparation & Vehicle Flashing

### Step 1: Format the SD Card
* **Capacity**: Use a high-quality **32 GB** or **64 GB** Class 10 / UHS-I SD card.
* **Filesystem**: Format strictly as **FAT32** (MS-DOS FAT).
* **Partition Scheme**: **Master Boot Record (MBR)**.
* **Allocation Unit Size**: **32 KB** (recommended for MMI QNX performance).

### Step 2: Copy Files to SD Card Root (CRITICAL)
* Open the `output/mmi3g_sd_card_update/` folder.
* Select **ALL files and folders inside it** and copy them directly to the **ROOT** directory of the SD card.
* **Do NOT put them in a subfolder** like `mmi3g_sd_card_update/` on the card.
* When you open the SD card in Finder or Explorer, `metainfo2.txt` must sit directly in the root:
  ```
  [SD Card Root]
  ├── metainfo2.txt
  ├── build_manifest.json
  ├── README_SD_CARD.txt
  ├── stock_recovery.sh
  ├── MU9411/
  ├── HBNavDB/
  └── MapStyles/
  ```

### Step 3: Vehicle Preparation
* Connect a **12V 30A+ battery maintainer** to the vehicle's engine bay jump posts (prevents voltage drop during flashing).
* Turn the vehicle ignition **ON** (engine **OFF**).
* Insert the SD card into **SD SLOT 1** (the left SD slot under the dashboard MMI screen/DVD drive).

### Step 4: Flashing through the Engineering Menu
1. Press and hold **[SETUP] + [RETURN]** (or `[CAR] + [BACK]` depending on console model) simultaneously for **5 seconds** to enter the Red Engineering Menu.
2. In the menu, scroll the rotary dial and select **Update**.
3. Select source: **SD 1**.
4. Select **Standard** (or *User-Defined* to select packages individually).
5. Scroll down to the bottom of the package list and click **Start Update**.
6. The MMI will flash the packages (`MU9411`, `HBNavDB`, `MapStyles`) and display progress bars. Do not turn off the ignition.
7. Once flashing reaches 100%, scroll to the bottom and select **Restart MMI**.
8. The system reboots with your new **Albanian language pack**, **2026 navigation network**, and **custom Gemini AI UI theme**!

---

## 9. Safety Policy & Emergency Recovery

### Mandatory §14.9 Policy
All builds and documentation carry the mandatory status:
> **`BUILD READY — DEPLOYMENT NOT VERIFIED`** · **`SIMULATED — NOT A GUARANTEE`**
> Claims of *"SAFE TO INSTALL"* are strictly prohibited. Flashing modified automotive firmware carries inherent hardware risks. Always verify on bench equipment before vehicle deployment.

### Emergency Baseline Recovery
If an update is interrupted or causes a boot loop:
1. Boot into QNX emergency recovery mode via the serial UART console or recovery SD card.
2. Run the provided rollback script:
   ```sh
   sh /mnt/sdcard/stock_recovery.sh
   ```
3. The script remounts `/mnt/efs-system` read-write, restores stock string tables from `/mnt/efs-system/backup/`, syncs filesystem blocks, and reboots cleanly.
