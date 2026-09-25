# ADR-012: Production In-Car SD Media Packaging & Hardware Defense Attestation

## Status
ACCEPTED (Architectural Baseline)

## Date
2026-09-24

## Context
Deploying compiled 2026 navigation databases onto real vehicle hardware (Renesas SH-4 QNX Neutrino 6.3.2) carries physical bricking risks if volume formatting, file system metadata, or hardware variant targets mismatch. Past testing identified macOS dotfile corruption (`._*`, `.DS_Store`), NOR flash write conflicts with `mmi3g-flashctl` garbage collection, and FSC activation lockouts.

## Decision
1. **Unified Release Packager (`crates/mmi-media/src/builder.rs`)**:
   - Package all components into standard OEM layout:
     - `metainfo2.txt` (SHA-1 checksums, `release = "2026_ECE"`, `compatibleTrains = "HN+R_EU_AU_K0942_4..."`)
     - `HBNavDB/` (FLDB multi-volume chunks, GDB v37, Geographic.gdb)
     - `pkgdb/LIT3GP/` (LIT search B-Tree and `LIT3GP.conf`)
     - `pkgdb/TER/` & `pkgdb/CTY/` (Orion ATLAS 3D terrain and building meshes)
     - `MU9411/` (Albanian and regional phonetic prompts)
     - `MapStyles/` (`rax\0` day/night shaders)
   - Include autonomous runner `copie_scr.sh` containing QNX 6.3.2 shims, `/etc/pci-3g_9411.cfg` hardware gate, `/tmp/disableReclaim` interlock, and emergency `stock_recovery.sh`.
2. **Media Geometry Sanitizer**:
   - Automatically strip hidden OS metadata (`.DS_Store`, `._*`, `Thumbs.db`).
   - Validate FAT32 32 KiB cluster geometry alignment and MBR partition table.
3. **Automated Attestation Gate**:
   - Enforce 6/6 step pass in `crates/mmi-media/src/simulator.rs`:
     `MediaDetection` -> `MetaInfoParsing` -> `ChecksumVerification` -> `ScriptExecution` -> `PackageInstallation` -> `RebootPending` -> `COMPLETED`.

## Consequences
- Guarantees zero head-unit hangs caused by corrupted filesystem dotfiles.
- Safeguards the vehicle NOR flash with automatic hardware defense checks.
