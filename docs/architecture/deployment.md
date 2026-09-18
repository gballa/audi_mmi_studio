# Architecture — Automotive Hardware Deployment Topology

This document details the target vehicle hardware topology, communication interfaces, and physical bootloader stages involved when deploying updates to an Audi MMI 3G High / Plus head unit.

---

## 1. Head Unit System Architecture

Audi MMI 3G+ (HN+ / HN+R) systems manufactured by Harman/Becker consist of a distributed embedded hardware architecture:

```text
                               ┌────────────────────────┐
                               │  800x480 Color Display │
                               └───────────▲────────────┘
                                           │ LVDS Video
┌──────────────────────────────────────────┴──────────────────────────────────────────┐
│  Audi MMI 3G+ Main Unit (MU9411)                                                    │
│                                                                                     │
│  ┌─────────────────────────┐   ┌────────────────────────┐   ┌────────────────────┐  │
│  │ Renesas SH-4A / ARM CPU │   │ Xilinx System FPGA     │   │ Analog Devices DSP │  │
│  │ QNX 6.5.0 Neutrino RTOS │   │ Video Switch & Glue    │   │ Blackfin BF53x     │  │
│  └───────────┬─────────────┘   └───────────┬────────────┘   └──────────┬─────────┘  │
│              │                             │                           │            │
│  ┌───────────▼─────────────────────────────▼───────────────────────────▼─────────┐  │
│  │ Internal High-Speed Peripherals Bus                                           │  │
│  └───────────┬─────────────────────────────┬───────────────────────────┬─────────┘  │
│              │                             │                           │            │
│  ┌───────────▼───────────┐   ┌─────────────▼──────────┐   ┌────────────▼─────────┐  │
│  │ Dual SD Card Reader   │   │ MOST Optical INIC      │   │ Quadlock Diagnostic  │  │
│  │ (SD1: Primary / SD2)  │   │ (OS81050/OS81110)      │   │ (CAN Bus & 3.3V UART)│  │
│  └───────────────────────┘   └────────────────────────┘   └──────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Bootloader Hierarchy & Update Stages

1. **Initial Program Loader (IPL)**:
   - Resides in protected NOR flash.
   - Executes hardware power-on self-test (POST).
   - Evaluates whether the Red Engineering Menu (`[CAR] + [BACK]`) update sequence is active.
   - Checks cryptographic digital signatures on QNX IFS bootloader images.
2. **QNX IFS Kernel (`ifs-root.ifs`)**:
   - Contains microkernel, flash drivers, and filesystem drivers (`devf-generic`).
   - Mounts flash partitions into `/fs/` mount points.
3. **QNX EFS Flash Mounts**:
   - Holds application binaries, localized string catalogs, and UI screen layout bitmaps.
   - Target destination for declarative theme modifications.

---

## 3. Communication & Safety Interfaces

- **FAT32 SD Card Slots**: Primary ingress path for legitimate customer updates and theme deployments.
- **Red Engineering Menu (REM)**: Firmware flasher built into the QNX installer subsystem.
- **Quadlock UART Serial Interface**: 3.3V TTL serial console (115200 baud, 8N1) used exclusively for emergency recovery and low-level diagnostic logs.
