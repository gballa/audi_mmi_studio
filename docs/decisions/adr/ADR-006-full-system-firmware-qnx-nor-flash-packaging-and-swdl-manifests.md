# ADR-006: Full System Firmware QNX NOR Flash Packaging and SWDL Manifests

## Status
ACCEPTED

## Date
2026-09-19

## Context
Deploying customizations (such as custom startup splash screens, theme precomps, Albanian localization, or diagnostics screens) to an Audi MMI 3G/3G+ head unit requires modifying files inside core system firmware partitions.
Grounded in reverse-engineering research (`docs/research/Audi MMI 3G:3G+ infotainment Research.md`):
- **Hardware Architecture**: The head unit runs a Renesas SH-4 CPU (`machine_type: 0x0006`) with QNX Neutrino RTOS 6.3.2 / 6.5.0.
- **NOR Flash Layout (~135 MB Total)**:
  - `ifs-root.ifs` (0x00680000..0x030FFFFF, max 43.74 MB / 45,875,200 bytes): Core QNX OS and Java HMI (`/usr/config/ci/splash.png`, `/usr/bin/lsd.jxe`).
  - `efs-system.efs` (0x03D00000..0x061FFFFF, max 38.80 MB / 40,697,856 bytes): QNX F3S embedded filesystem mounted at `/mnt/efs-system` containing localization strings (`strings/sq_AL.ans`) and GEM menus (`engdefs/menu_2026.esd`).
- **SWDL Checksum Verification**:
  - The MMI SoftWare DownLoader (`SWDL`) enforces **no per-image RSA signatures** on firmware images.
  - Integrity is strictly verified via CRC32 checksums calculated across **512 KiB blocks** in `metainfo2.txt`.
- **Script Launcher Automation**:
  - Upon SD card insertion, `proc_scriptlauncher` automatically executes `copie_scr.sh`.
  - After flashing, SWDL executes `finalScript` to flush buffers and reboot.
  - In boot-loop emergencies, UART recovery requires an automated script (`stock_recovery.sh`).

## Decision
Implement an end-to-end full system firmware packaging pipeline:
1. **Binary Builders in `mmi-formats`**:
   - `QnxIfsBuilder`: Builds valid QNX IFS images (`QNX_IFS_MAGIC` = `[0xeb, 0x7e, 0xff, 0x00]`), generates SH-4 startup headers, and validates `MAX_IFS_ROOT_SIZE`.
   - `QnxEfsBuilder`: Builds valid QNX F3S flash filesystems (`QNX_F3S_MAGIC` = `b"QSSL_F3S"`), preserves the `/mnt/efs-system` mount point at offset `0x48`, and validates `MAX_EFS_SYSTEM_SIZE`.
   - `MetaInfo2Builder`: Computes IEEE 802.3 CRC32 block checksums for every 512 KiB slice of binary payloads and formats standard INI sections.
2. **Firmware Packaging Engine in `mmi-rebuild` (`FirmwareBundlePipeline`)**:
   - Coordinates assembly of `ifs-root.ifs`, `efs-system.efs`, `HBNavDB/nav_data.db`, and `MapStyles/night_2026.gdb`.
   - Generates `metainfo2.txt`, `copie_scr.sh`, `finalScript`, `stock_recovery.sh`, and `build_manifest.json`.
   - Strictly enforces immutability of `originals/` and embeds §14.9 Safety Policy notice (`BUILD READY — DEPLOYMENT NOT VERIFIED`).
3. **CLI & Desktop GUI Surface**:
   - CLI: `mmi-studio-cli firmware package --output <DIR>`.
   - GUI: `BuildStudio.tsx` displays live NOR flash capacity gauges (showing bytes and percentage used against hardware limits) and the complete SD card filesystem tree.

## Alternatives Considered

### In-Place Byte Patching of Flash Images Without Structure Rebuild
- Pros: Simple hex replacement for fixed-length strings.
- Cons: Cannot add new files, cannot accommodate translated strings that exceed original byte lengths, and risks corrupting compressed LZO/F3S offsets.
- Rejected: Inflexible and high risk of bricking.

### External Hardware JTAG/Flash Programming Only
- Pros: Bypasses SWDL software checks.
- Cons: Requires removing the MMI unit from the vehicle dashboard, disassembling the aluminum enclosure, and soldering directly to the flash IC. Unusable for standard vehicle owners.
- Rejected: Violates accessibility and non-destructive maintenance goals.

## Consequences
- Produces complete, ready-to-flash SD card bundles runnable through standard OEM update procedures or automated script launching.
- Prevents flash memory overflow with compile-time NOR flash boundary verification.
- Guarantees immediate disaster recovery via `stock_recovery.sh` over QNX UART.
