# How-To: Prepare SD Deployment Media

This guide walks through formatting SD media with the exact 32 KiB cluster geometry required by the Audi MMI 3G head unit and staging update files.

---

## Goal
Prepare physical SD cards that meet Audi Harman/Becker IPL bootloader requirements and generate volume manifests.

## Prerequisites
- High-grade Full-Size SD/SDHC card (8 GB to 32 GB, Class 10 / UHS-I).
- Validated update directory (e.g. `output/candidate_rebuild`).
- Compiled `mmi-studio-cli`.

---

## Procedure

### Step 1: Format Physical SD Media (Host OS)

> [!IMPORTANT]
> The Audi MMI 3G QNX bootloader requires a MBR partition table with a single primary FAT32 partition aligned to **32 KiB cluster size** (64 sectors per cluster).

#### On macOS:
```bash
# Identify disk identifier (e.g. /dev/disk4)
diskutil list

# Format as FAT32 with MBR partition scheme
diskutil eraseDisk FAT32 MMI3G_NAV MBRFormat /dev/diskN
```

#### On Linux:
```bash
# Partition with fdisk (Type 0x0C: W95 FAT32 LBA)
sudo fdisk /dev/sdX

# Format with 32 KiB cluster size (-s 64 with 512-byte sectors)
sudo mkfs.vfat -F 32 -s 64 -n "MMI3G_NAV" /dev/sdX1
```

### Step 2: Build Deployment Media via CLI

You can generate SD media using either full firmware packaging, standalone cartography compilation, or standard stage building:

#### Option A: Package Full System Firmware (Recommended for UI/Language/Maps)
```bash
./target/release/mmi-studio-cli firmware package \
  --output /Volumes/MMI3G_NAV \
  --train "HN+R_EU_AU_K0942_4" \
  --release "2026_ECE" \
  --variant "MU9411"
```
*Expected Result*: Populates `/Volumes/MMI3G_NAV/` with:
- `metainfo2.txt` (SWDL manifest with per-512KB CRC32 blocks)
- `copie_scr.sh` (Automatic SD insertion launcher for `proc_scriptlauncher`)
- `finalScript` (Post-flash sync and reboot script)
- `stock_recovery.sh` (Emergency UART rollback script)
- `MU9411/ifs-root.ifs` (SH-4 QNX root partition with 2026 splash and HMI bytecode)
- `MU9411/efs-system.efs` (QNX F3S filesystem with Albanian language catalog and GEM menus)
- `HBNavDB/nav_data.db` (Harman/Becker FLDB 544-byte pages)
- `build_manifest.json` (Cryptographic attestation and BLAKE3 hashes)

#### Option B: Compile Modern Navigation Cartography
```bash
./target/release/mmi-studio-cli maps compile \
  --output /Volumes/MMI3G_NAV \
  --region AL \
  --release "2026_ECE"
```

#### Option C: Build Custom Staged Theme Package
```bash
./target/release/mmi-studio-cli build-media \
  --stage default \
  --output /Volumes/MMI3G_NAV \
  --volume-label "MMI3G_NAV" \
  --volume-size-gb 32
```

### Step 3: Unmount Safely
Always cleanly flush disk caches before ejecting:
```bash
# macOS
diskutil eject /dev/diskN

# Linux
sync && sudo umount /media/MMI3G_NAV
```
