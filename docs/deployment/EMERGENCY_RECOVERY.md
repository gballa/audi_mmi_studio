# Audi MMI 3G+ Emergency Recovery and Rollback Procedures

This document outlines emergency rollback workflows, diagnostic fault troubleshooting, and recovery procedures in the event of an interrupted update or system malfunction on Audi MMI 3G+ hardware [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L830].

---

## 1. Stock Baseline Restoration Workflow

Before deploying any modified package, an unmodified stock baseline package must be created and verified using Audi MMI Studio (§14.7) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L795]:
```bash
mmi-studio-cli stock-recovery --baseline-train HN+R_EU_AU_K0942_4_[8R0906961FB] --output /Volumes/MMI3G_STOCK
```
If an installation of modified assets exhibits visual glitches, font truncation, or instability, follow this protocol [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L798]:
1. Power down the head unit and connect an external 12.5V battery maintainer [INF:HIGH basis: stable voltage requirement for firmware flashing].
2. Insert the stock baseline recovery SD card into `SD Slot 1` [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]].
3. Access the Red Engineering Menu (`CAR` + `BACK`) [INF:HIGH basis: standard MMI engineering menu access].
4. Select `Update` -> `SD 1` -> `Standard` [INF:HIGH basis: OEM baseline reinstallation flow].
5. Complete the re-flash to restore factory binaries and Linotype fonts verbatim [EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/GEMMI/nav/0/default/res].

---

## 2. Hard System Reset (Three-Button Restart)

If the MMI display freezes, crashes to black screen, or fails to respond to input wheel controls, trigger a hardware watchdog reboot [INF:HIGH basis: documented Audi MMI 3G hardware reset shortcut]:
- **Audi A4 / A5 / Q5 / Q7 (Console Controls)**:
  - Simultaneously press and hold `SETUP` (or `MENU`) + `Center Control Knob` + `Upper-Right Softkey` [INF:HIGH basis: MMI 3G console shortcut].
- **Audi A6 (C7) / A7 / A8 (Console Controls)**:
  - Simultaneously press and hold `MENU` + `Center Control Knob` + `Upper-Right Softkey` [INF:HIGH basis: MMI 3G+ C7 console shortcut].
- Hold the combination for 2 seconds until the screen snaps off, then release. The head unit will cycle power and reload the active QNX root IFS image [INF:HIGH basis: QNX ifs-root execution sequence].

---

## 3. Common Update Fault Codes & Mitigations

During QNX software download (`swdl`), specific error numbers may be reported on the screen [INF:HIGH basis: QNX swdl error code taxonomy]:
- **Error 140 (Checksum / CRC Failure)**:
  - Cause: File corruption on SD card or bad memory block [INF:HIGH basis: metainfo2 CRC mismatch during swdl verification].
  - Resolution: Re-format SD card with 32 KiB cluster alignment using `mmi-studio-cli build-media` and verify SHA-256 digests against `media_manifest.json` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L765].
- **Error 160 (Device / Communication Timeout)**:
  - Cause: Bus communication drop or excessive read latency on low-grade SD media [INF:MEDIUM basis: devb-eide read timeout].
  - Resolution: Replace media with a genuine Class 10 / UHS-I SD card [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L760].
- **Emergency Update Loop**:
  - Cause: The system entered emergency download mode after a failed flash of the application partition [INF:HIGH basis: QNX emergency IFS fallback].
  - Resolution: Insert an official OEM firmware update card in `SD Slot 1`; the bootloader will automatically trigger the emergency flash loader [INF:HIGH basis: bootloader emergency recovery loop].

---

## 4. Hardware UART & D-Link DUB-E100 Serial Recovery

For units that fail to reach the bootloader GUI due to corrupt application files, recovery requires low-level terminal access [INF:HIGH basis: QNX engineering console architecture]:
- **USB-to-Ethernet Adapter**: Genuine D-Link DUB-E100 (Revision A1 or B1 with ASIX AX88172/AX88772 chipset) connects to the MMI AMI port via USB cable [INF:HIGH basis: devn-asix driver in QNX ifs-root].
- **Network Configuration**: Host PC set to static IP `192.168.1.100`, subnet `255.255.255.0`; MMI default IP is `192.168.1.4` [INF:HIGH basis: standard QNX network configuration in MMI 3G+].
- **Telnet Access**: Connect via telnet to port 23 on `192.168.1.4` to open the root QNX shell [INF:HIGH basis: QNX inetd telnet service on MMI units]:
  ```bash
  telnet 192.168.1.4
  ```
- **Direct UART Serial**: 3-pin serial connection on the rear quadlock connector (`TX`, `RX`, `GND`) running at 115200 baud, 8-N-1 allows monitoring QNX IPL (Initial Program Loader) and kernel output [INF:HIGH basis: hardware serial debug interface on Harman Becker automotive units].
