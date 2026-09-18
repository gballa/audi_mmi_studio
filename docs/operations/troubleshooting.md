# Operations — Operational Troubleshooting

This document addresses hardware, media, and communication errors encountered during in-vehicle firmware deployment.

---

## 1. Problem: Head Unit Reports "No Valid Update Found" on SD Card

### Symptoms
After selecting **Update** in the Red Engineering Menu, the head unit scans the SD card and reports: `No valid update found`.

### Likely Causes
1. `metainfo2.txt` is missing from the root of the SD card or is located inside a subfolder.
2. SD card partition is not formatted as FAT32 with exact 32 KiB cluster geometry.
3. Card capacity exceeds 32 GB without proper partition boundary alignment.

### Resolution
1. Verify `metainfo2.txt` is placed directly in the SD root directory (`/Volumes/MMI3G_NAV/metainfo2.txt`).
2. Reformat the SD card with 32 KiB cluster geometry (`mkfs.vfat -F 32 -s 64 -n "MMI3G_NAV"`).
3. Ensure the physical card is an SD/SDHC card (not SDXC formatted as exFAT).

---

## 2. Problem: Update Halts with "Error Code 140" or "Error Code 152"

### Symptoms
The QNX update progress bar halts midway and displays an error dialog indicating `Error 140` (Checksum failure) or `Error 152` (Device read error).

### Likely Causes
- Intermittent contact on the SD card slot pins.
- Battery voltage dipped below 12.0V, causing flash write throttling.
- Corrupted file transfer during SD card writing.

### Resolution
1. Connect a 13.5V battery maintainer to the vehicle jump posts.
2. Re-write the media files using `mmi-studio-cli build-media`.
3. Clean the physical SD card contacts with isopropyl alcohol.
4. Try SD Slot 2 if SD Slot 1 has physical pin wear.

---

## 3. Problem: Head Unit Hangs at Splash Screen After Reboot

### Symptoms
After completing an update, the head unit restarts, displays the four rings splash logo, and remains frozen.

### Likely Causes
- Graphic asset dimensions in `CombiStyles.precomp` exceeded display framebuffer limits.
- Memory corruption in writable flash partition.

### Resolution
1. Perform a Level 1 Three-Button Reset (`[SETUP] + [ROTARY] + [UPPER-RIGHT SOFTKEY]`).
2. If the freeze recurs, execute Level 2 Emergency Red Menu Stock Rollback or Level 3 UART Rollback.
