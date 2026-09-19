# ADR-008: In-Car SD Deployment Hardening, Hardware Defense, and Media Sanitization

## Status
ACCEPTED

## Date
2026-09-19

## Context
When an operator exports a custom firmware update bundle or 2026 navigation cartography database onto an SD card and inserts it into an Audi MMI 3G / 3G+ vehicle, the head unit's operating environment imposes severe embedded constraints:

1. **Stripped QNX Neutrino 6.3.2 Userland**:
   The Renesas SH-4 minimal root image lacks standard POSIX/GNU command-line tools:
   - Missing: `head`, `basename`, `wc`, `printf`, `awk`, `readlink`, `sort`.
   - Broken behavior: `mkdir -p` fails with `"Function not implemented"` on nested directories.
   - Timestamp unreliability: `date` returns elapsed seconds since boot until the vehicle CAN gateway synchronizes wall-clock time.
2. **Host OS Metadata Incompatibility**:
   macOS and Windows automatically write hidden indexing and resource files (`.DS_Store`, AppleDouble `._*`, `Thumbs.db`, `.Spotlight-V100`, `.Trashes`). The Audi MMI Harman/Becker Software Download (SWDL) manager and `proc_scriptlauncher` daemon frequently crash or abort with `"Medium unreadable"` when parsing volumes containing these files.
3. **Hardware Variant Risks**:
   Flashing a high-tier MMI 3G+ (HN+) payload onto a low-tier MMI 3G Basic (BNav) or High (HNav) unit can permanently corrupt the NOR flash partition table or brick the unit.
4. **F3S Flash Garbage Collection Conflicts**:
   The `mmi3g-flashctl` daemon runs background flash reclamation every 5 minutes against `/mnt/efs-system` and `/HBpersistence`. Writing to flash during garbage collection causes filesystem corruption.
5. **Map Activation Locking**:
   The `vdev-logvolmgr` process generates `/mnt/lvm/acios_db.ini`, triggering `MMI3GApplication` to verify FSC licenses. If the FSC does not match 2026 maps, navigation access locks after 3 minutes.

---

## Decision

We implement a comprehensive three-tier hardening suite across the workstation and deployment artifacts:

### 1. Media Sanitization Engine (`crates/mmi-media`)
- Implement `MediaSanitizer` to recursively scan removable SD media volumes and purge `.DS_Store`, `._*`, `Thumbs.db`, `.Spotlight-V100`, and `.Trashes`.
- Expose the sanitizer via CLI (`mmi-studio-cli sanitize-media <TARGET>`) and the desktop GUI (`BuildStudio.tsx`).

### 2. Autonomous In-Car Defense Runner (`copie_scr.sh`)
Upgrade the SD insertion script generator in `crates/mmi-rebuild/src/firmware_bundle.rs` into an autonomous 8.5 KB runner containing:
- **QNX 6.3.2 Shims**: Drop-in shell replacements for `head`, `basename`, `wc`, `printf`, `awk`, and a recursive directory builder `_qnx_mkdir_p`.
- **Wall-Clock Time Resolution**: Uses `/bin/getTime` (Harman-Becker binary) with fallback to `date +%s`.
- **Authoritative Hardware Defense**: Inspects `/etc/pci-3g_XXXX.cfg` (`9304` = Basic, `9308` = High, `9411` = Plus, `9436` = A1, `9478` = RNS-850). If the physical hardware ID mismatches the target build, the script safely halts with an explanatory error logged to SD.
- **F3S Flash Reclaim Interlock**: Creates `/tmp/disableReclaim` and installs an `EXIT INT TERM` trap to block `mmi3g-flashctl` garbage collection during writes.
- **Pre-Update Safety Backup**: Dumps NVRAM, `/etc/version/`, `/dev/shmem/sw_trainname.txt`, and active scripts to `$SDPATH/backup_<timestamp>/`.
- **Visual Framebuffer Proof**: Calls `/usr/bin/screendump` to capture `screen_before.png` and `screen_after.png`.
- **Navigation Activation Bypass (Keldo/DrGER2)**: Injects `(waitfor /mnt/lvm/acios_db.ini 180 && sleep 10 && slay vdev-logvolmgr) &` into `/mnt/efs-system/usr/bin/manage_cd.sh` if not present.
- **GEM Screen Deployment**: Copies `custom_telemetry.esd` and `map_inspector.esd` into `/mnt/efs-system/etc/screens/`.

### 3. Verification & Diagnostic Solvers
- Retain the OBD-II SVM Error 03276 solver (`0xC0DE ^ 0x516B`) to clear post-update DTCs.
- Enforce §14.9 Safety Policy (`BUILD READY — DEPLOYMENT NOT VERIFIED`).

---

## Alternatives Considered

### 1. Direct Binary Patching of `MMI3GApplication` for FSC Validation
- **Approach**: Patching `0x0B40` to `0x00E0` at offset `0x1B11F6` in `MMI3GApplication` to force `EscRsa_DecryptSignature` to return 0.
- **Rejected as default**: Modifying the primary SH-4 executable requires unpacking and repacking `ifs-root.ifs`, carrying a higher brick risk if power is lost during SWDL flashing. The `manage_cd.sh` daemon lifecycle hook accomplishes the same map unblocking safely in userland without modifying flash binaries.

### 2. Manual User Deletion of OS Dotfiles
- **Approach**: Documenting manual `rm -rf ._*` commands for users.
- **Rejected**: High human error rate. Users frequently forget, leading to recurring "Medium unreadable" support issues. Automated sanitization eliminates this class of failure completely.

---

## Consequences

- **Safety**: In-car scripts cannot run on incompatible head unit hardware. Flash writes cannot collide with F3S garbage collection.
- **Turnkey Reliability**: SD cards inserted into the car execute cleanly without missing utility errors or media format rejections.
- **Reversibility**: Pristine factory configuration is backed up to SD before any flash modification occurs.
- **Compliance**: Adheres strictly to §14.9 Safety Policy; outputs remain tagged `BUILD READY — DEPLOYMENT NOT VERIFIED`.
