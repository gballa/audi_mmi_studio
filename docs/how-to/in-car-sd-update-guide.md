# How-To: Complete In-Car SD Card Upgrade Guide

This guide provides a step-by-step, fail-safe procedure for packaging, sanitizing, and installing firmware, navigation maps (2026 FLDB), and custom themes onto an Audi MMI 3G / 3G+ (HN+ / HN+R) vehicle using physical SD media.

---

## Overview

The in-car deployment pipeline utilizes two coordinated mechanisms supported by the Harman/Becker QNX operating system:
1. **Automated Insertion Script Runner (`copie_scr.sh`)**: Executed automatically upon SD card insertion by `proc_scriptlauncher` to verify hardware, snapshot NVRAM, deploy Green Menu screens, and unblock map licenses.
2. **Standard Software Download (SWDL)**: Triggered through the Red Engineering Menu (REM) to flash core operating system partitions (`ifs-root.ifs`, `efs-system.efs`) and navigation database volumes (`HBNavDB/nav_data.db`).

---

## Prerequisites

### Hardware Requirements
- **SD Card**: High-grade Full-Size SD/SDHC card (8 GB to 32 GB, Class 10 / UHS-I). Avoid micro-SD adapters if possible.
- **Power Supply**: 12V 25A+ external battery maintainer connected to the vehicle jump-start posts under the hood (prevents low-voltage shutdown mid-flash).
- **Vehicle State**: Ignition **ON**, Engine **OFF**, all vehicle accessories (headlights, climate control, seat heaters) **OFF**.

### Vehicle Preparation
1. Remove all SD cards from **Slot 2**.
2. Remove any SIM card from the MMI center slot.
3. Disconnect any iPod/USB/Bluetooth media adapters from the Audi Music Interface (AMI) port in the glovebox.

---

## Step 1: Generate Update Bundle on Workstation

You can generate the complete SD deployment bundle using either the Desktop GUI or the Headless CLI.

### Option A: Using Audi MMI Studio Desktop GUI
1. Launch `Audi MMI Studio`.
2. Navigate to the **BuildStudio** tab.
3. Select your target **OSM Map Cartography Profile** (e.g. *Albania & WB Corridor* or *Balkans Transit*).
4. Verify the **Diagnostic Long Coding Helper** options (Green Menu enabled, Drive Select, 3D Landmarks).
5. Click **Build Full System Firmware Bundle**. The bundle will be assembled into:
   ```
   output/mmi3g_sd_card_update/
   ```

### Option B: Using the Standalone CLI
Run the firmware packaging command:
```bash
./target/release/mmi-studio-cli firmware package \
  --output output/mmi3g_sd_card_update \
  --train "HN+R_EU_AU_K0942_4" \
  --release "2026_ECE" \
  --variant "MU9411"
```

Verify pre-flight simulation before copying:
```bash
./target/release/mmi-studio-cli simulate-update output/mmi3g_sd_card_update
```
*Expected Result*: All 6/6 simulation stages must report `[PASS]`.

---

## Step 2: Format & Sanitize Physical SD Media

The Audi MMI QNX bootloader requires Master Boot Record (MBR) partition geometry formatted as FAT32 with 32 KiB cluster size (64 sectors per cluster).

### 1. Format the SD Card

#### On macOS:
```bash
# Identify disk number (e.g. /dev/disk4)
diskutil list

# Format as FAT32 with MBR scheme
diskutil eraseDisk FAT32 MMI3G_NAV MBRFormat /dev/diskN
```

#### On Linux:
```bash
# Partition as FAT32 MBR
sudo fdisk /dev/sdX

# Format with 32 KiB clusters (-s 64 with 512B sectors)
sudo mkfs.vfat -F 32 -s 64 -n "MMI3G_NAV" /dev/sdX1
```

### 2. Copy Files to SD Root
Copy all contents of `output/mmi3g_sd_card_update/` directly to the **root** of the SD card:
```
/Volumes/MMI3G_NAV/
├── metainfo2.txt               <-- Root SWDL manifest
├── build_manifest.json         <-- Cryptographic hashes
├── copie_scr.sh                <-- Automated script runner & hardware defense
├── finalScript                 <-- Post-install sync script
├── stock_recovery.sh           <-- Emergency UART rollback
├── MU9411/                     <-- Core QNX partitions (ifs-root.ifs, efs-system.efs)
├── HBNavDB/                    <-- 2026 Navigation database pages (nav_data.db)
├── MapStyles/                  <-- High-contrast Day & Night shaders
└── gem/                        <-- Custom Green Menu telemetry screens
```

> [!CAUTION]
> Do NOT copy the parent folder `mmi3g_sd_card_update` onto the card. The file `metainfo2.txt` must reside directly at `X:\metainfo2.txt`.

### 3. Sanitize SD Media (Purge OS Dotfiles)
macOS automatically generates hidden AppleDouble metadata (`._*`, `.DS_Store`) that cause the MMI update manager to reject media with *"Medium unreadable"*.

Sanitize the volume via CLI:
```bash
./target/release/mmi-studio-cli sanitize-media /Volumes/MMI3G_NAV
```
*(Or click the **Sanitize** button in BuildStudio).*

Unmount cleanly:
```bash
# macOS
diskutil eject /dev/diskN

# Linux
sync && sudo umount /media/MMI3G_NAV
```

---

## Step 3: Vehicle Installation Procedure

### Phase A: Automated Script Execution (ScriptLauncher)
1. Turn ignition ON (engine OFF).
2. Insert SD card into **SD SLOT 1** (left slot on the MMI console).
3. The MMI's `proc_scriptlauncher` daemon automatically detects `copie_scr.sh`:
   - Validates that head unit hardware matches `/etc/pci-3g_XXXX.cfg`.
   - Engages `/tmp/disableReclaim` F3S flash lock to prevent write corruption.
   - Saves a pre-update NVRAM safety snapshot and `screen_before.png` to `/fs/sda0/backup_<timestamp>/`.
   - Deploys custom Green Menu screens (`custom_telemetry.esd`, `map_inspector.esd`).
   - Injects the navigation unblocker hook into `manage_cd.sh` so 2026 maps never lock after 3 minutes.
   - Saves `screen_after.png` and logs to `/fs/sda0/var/`.
4. Wait approximately 30 seconds for disk activity LED to cease.

---

### Phase B: Full Firmware & Navigation SWDL Upgrade
If updating system partitions (`ifs-root`, `efs-system`, or `HBNavDB` maps):

1. **Enter Red Engineering Menu (REM)**:
   - For **MMI 3G+**: Press and hold **[CAR]** + **[BACK]** simultaneously for 5 seconds.
   - For **MMI 3G High**: Press and hold **[SETUP]** + **[RETURN]** simultaneously for 5 seconds.
2. Select **Update** on the MMI screen.
3. Select Source: **SD 1**.
4. Select **Standard** update mode (or *User-Defined* to inspect individual components).
5. Scroll to the bottom and select **Start Update**.
6. Select **Start** to confirm flashing.
   - The MMI will flash partitions sequentially: `ifs-root` -> `efs-system` -> `HBNavDB`.
   - **Do NOT turn off ignition or touch controls during flashing.**
7. When flashing reaches 100%, the MMI will display an update summary.
8. Scroll to the bottom and select **Continue**.
9. Select **Abort documentation** (or *Cancel documentation*) to bypass Audi dealer online feedback.
10. Select **Restart MMI**.

---

## Step 4: Post-Update Verification & Diagnostics

### 1. Perform 3-Finger MMI Hardware Reset
Reboot the unit cleanly to initialize new drivers:
- **3G+ Reset Sequence**: Press and hold simultaneously:
  - **[MENU]** + **[Rotary Center Knob]** + **[Upper-Right Softkey]**
  - Release all three buttons simultaneously.

### 2. Clear SVM Error 03276 (Software Version Management)
Firmware updates trigger a non-fatal DTC 03276 in Module 5F. To clear:
1. Connect VCDS, OBDeleven, or launch the **BuildStudio OBD-II Bridge**.
2. Open **Module 5F (Information Electronics)** -> **Adaptation (Channel 15)**.
3. Note the decimal challenge number (e.g. `12345`).
4. Click **Solve SVM Error 03276** in BuildStudio (or run: `challenge XOR 0x516B`).
5. Enter the resulting decimal code into Channel 15 and click **Save**.
6. Read fault memory: DTC 03276 is cleared.

### 3. Open Green Engineering Menu (GEM)
1. Hold **[CAR]** + **[MENU]** for 5 seconds (or *[CAR]* + *[SETUP]* on 3G High).
2. Navigate to `/screens/` to view:
   - **Live Telemetry & Engine Gauges** (`custom_telemetry.esd`)
   - **2026 Navigation Cache & FLDB Health** (`map_inspector.esd`)

---

## Emergency Rollback Procedure

If the unit experiences an unexpected issue or requires baseline restoration:
1. Connect to the MMI UART serial console or telnet shell on port 2323.
2. Insert the SD card into Slot 1.
3. Execute the rollback runner:
   ```sh
   sh /fs/sda0/stock_recovery.sh
   ```
4. All modified files are restored from pristine factory baselines.

---

## Automotive Safety Notice
All update artifacts and procedures adhere strictly to §14.9 Safety Policy:
`BUILD READY — DEPLOYMENT NOT VERIFIED`
