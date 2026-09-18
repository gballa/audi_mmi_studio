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

### Step 2: Build Deployment Media Structure via CLI
Use `mmi-studio-cli build-media` to construct the update package:
```bash
./target/release/mmi-studio-cli build-media \
  --stage default \
  --output /Volumes/MMI3G_NAV \
  --volume-label "MMI3G_NAV" \
  --volume-size-gb 32
```
*Expected Result*: Output populates `/Volumes/MMI3G_NAV/` with:
- `metainfo2.txt` (Main release manifest and CRC32/SHA-1 checksums)
- Module directories (`MU9411/`, `ScreenLayouts/`)
- Volume split index files if the total size exceeds 32 GB.

### Step 3: Unmount Safely
Always cleanly flush disk caches before ejecting:
```bash
# macOS
diskutil eject /dev/diskN

# Linux
sync && sudo umount /media/MMI3G_NAV
```
