# Audi MMI Studio

> [![License](https://img.shields.io/badge/License-MIT%20%2F%20Apache--2.0-blue.svg)](LICENSE) [![Rust](https://img.shields.io/badge/Rust-1.80%2B%20%7C%202021-orange.svg)](Cargo.toml) [![Architecture](https://img.shields.io/badge/Architecture-Offline--First-green.svg)](docs/architecture/overview.md) [![Verification](https://img.shields.io/badge/Status-BUILD%20READY%20%E2%80%94%20DEPLOYMENT%20NOT%20VERIFIED-yellow.svg)](docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md) [![Tests](https://img.shields.io/badge/Tests-189%2F189%20Passing%20Offline-brightgreen.svg)](docs/development/testing.md)
>
> **Offline engineering workstation for Audi MMI 3G / 3G+ (HN+ / HN+R) reverse-engineering, 2026 navigation cartography compilation, full system firmware packaging, and visual theming.** [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L6]
>
> **Quick Links**: [Quick Start](#quick-start) • [Documentation Portal](docs/README.md) • [CLI Manual](apps/mmi-studio-cli/README.md) • [Desktop GUI](apps/mmi-studio-desktop/README.md) • [Architecture Decisions (ADRs)](docs/decisions/README.md) • [Contributing](CONTRIBUTING.md)

---

## Overview

Audi MMI Studio provides an isolated, offline workstation toolchain for inspecting, skinning, updating, and safely repackaging lawfully possessed Audi infotainment firmware packages [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L6]:

- **100% Offline & Airlocked**: Zero telemetry, zero external network calls, and strict airlock isolation for AI prompt sanitization [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L948].
- **14 Proprietary Formats Resolved**: Native decoders for navigation DBs (FLDB), vector atlas tiles, UI graphics (`.precomp`), map styles (`.xar`/`.gdb`), QNX IFS/EFS images, acoustic speech models, FPGA bitstreams, and DSP audio loaders [EV:doc:docs/research/RQ-REGISTER.md#L10].
- **2026 Navigation Cartography Engine**: Ingests OpenStreetMap vector data, enriches POIs via Google Maps Platform under strict ToS compliance (30-day cache eviction, FieldMask filtering), and compiles 544-byte FLDB physical pages split into 2 GiB volumes [EV:doc:docs/research/Audi%20MMI%203G+%20Maps%20Research.md#L1].
- **Full System Firmware & NOR Flash Assembly**: Packages Renesas SH-4 QNX IFS root (`ifs-root.ifs` <= 43.74 MB) and QNX EFS system (`efs-system.efs` <= 38.8 MB), generating SWDL `metainfo2.txt` with per-512KB CRC32 blocks and script launchers [EV:doc:docs/research/Audi%20MMI%203G:3G+%20infotainment%20Research.md#L1].
- **Albanian (sq_AL) Localization**: Native Harman ANS binary string catalog serialization with TrueType font metric validation to prevent display clipping [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L777].
- **Reverse-Engineering Suite**: Virtualized hex viewer, sliding Shannon entropy profiler, byte histogram analyzer, string extractor, and signature carver [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L776].
- **Pixel-Perfect Screen Simulation**: Interactive 800x480 frame composition with Day, Night, and Reduced palette modes [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L779].
- **Declarative Theme Recipes**: Non-destructive JSON theming engine with cross-train drift analysis and cryptographic journaling [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L782].
- **Automotive Safety Gates**: Bit-for-bit rebuild verification, immutable signed payload locks, and 6-tier (`L0`–`L5`) validation hierarchy [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L783].
- **Deployment & Emergency Recovery**: FAT32 SD card builder (32 KiB cluster geometry), QNX update simulator, and automated stock recovery bundler with UART scripts [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L784].

---

## Quick Start

Get up and running with the standalone workstation toolchain in four steps [EV:doc:docs/getting-started/quick-start.md#L1]:

```bash
# 1. Clone the repository
git clone <repo-url>
cd AudiMMI

# 2. Build the standalone release CLI (100% offline)
cargo build --release -p mmi-studio-cli --offline

# 3. Package a complete full firmware SD update bundle
./target/release/mmi-studio-cli firmware package \
  --output output/mmi3g_sd_card_update \
  --train "HN+R_EU_AU_K0942_4" \
  --release "2026_ECE" \
  --variant "MU9411"

# 4. Run the full workstation verification gate
./scripts/verify-workstation.sh
```

---

## Applications & Interfaces

Audi MMI Studio offers both command-line and graphical interfaces [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L748]:

| Interface | Technology | Primary Capabilities | Guide |
| :--- | :--- | :--- | :--- |
| **`mmi-studio-cli`** | Rust / Clap | Headless CLI for reverse engineering, format decoding, theme recipes, map compilation, and firmware packaging. | [CLI Manual](apps/mmi-studio-cli/README.md) [EV:doc:apps/mmi-studio-cli/README.md#L1] |
| **`mmi-studio-desktop`** | Tauri v2 / React 19 | Interactive HexViewer, CAS AssetBoard, 800x480 ScreenCanvas, RecipeStudio, TypographyStudio, MapStudio, and BuildStudio. | [Desktop Manual](apps/mmi-studio-desktop/README.md) [EV:doc:apps/mmi-studio-desktop/README.md#L1] |
| **`crates/`** | 14 Rust Crates | Core storage, decoders, FLDB cartography compiler, firmware packager, recipe engine, and egress airlock. | [Crates Architecture](crates/README.md) [EV:doc:crates/README.md#L1] |

---

## Documentation System

The complete canonical documentation architecture is organized under [`docs/`](docs/README.md) [EV:doc:docs/README.md#L1]:

| Section | Scope | Quick Link |
| :--- | :--- | :--- |
| **Getting Started** | Prerequisites, offline installation, configuration, and verification | [Quick Start](docs/getting-started/quick-start.md) • [Installation](docs/getting-started/installation.md) [EV:doc:docs/getting-started/quick-start.md#L1] |
| **User Guide** | Detailed user manuals for CLI commands, GUI panels, and common workflows | [Overview](docs/user-guide/overview.md) • [CLI Guide](docs/user-guide/cli.md) • [GUI Guide](docs/user-guide/gui.md) [EV:doc:docs/user-guide/overview.md#L1] |
| **How-To Library** | Task-focused guides: inspection, asset replacement, theming, rebuilds, recovery, and SD prep | [How-To Index](docs/how-to/README.md) • [Produce 2026/Albanian Build](docs/how-to/produce-albanian-and-2026-maps-firmware.md) [EV:doc:docs/how-to/README.md#L1] |
| **Technical Reference** | CLI syntax, 14 binary formats (RQ-001..012), Rust APIs, schemas, and glossary | [Format Specs](docs/reference/file-formats.md) • [CLI Reference](docs/reference/cli.md) [EV:doc:docs/reference/file-formats.md#L1] |
| **Architecture** | System principles, crate components, data flows, and STRIDE security model | [System Overview](docs/architecture/overview.md) • [Security Model](docs/architecture/security.md) [EV:doc:docs/architecture/overview.md#L1] |
| **Development** | Workspace setup, coding standards, testing tiers, and release packaging | [Dev Setup](docs/development/setup.md) • [Testing](docs/development/testing.md) [EV:doc:docs/development/setup.md#L1] |
| **Operations** | SD media flashing, REM execution, UART monitoring, and emergency recovery | [Deployment](docs/operations/deployment.md) • [Recovery](docs/operations/recovery.md) [EV:doc:docs/operations/deployment.md#L1] |
| **Decisions (ADRs)** | Architecture Decision Records (ADR-001 through ADR-008) | [ADR Index](docs/decisions/README.md) [EV:doc:docs/decisions/README.md#L1] |

---

## Architectural Decision Records (ADRs)

Key architectural choices are formally documented under [`docs/decisions/`](docs/decisions/README.md) [EV:doc:docs/decisions/README.md#L1]:

- [**ADR-001**](docs/adr/ADR-001-offline-first-workspace-architecture.md): Offline-First Modular Rust Workspace Architecture [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L771]
- [**ADR-002**](docs/adr/ADR-002-tauri-desktop-architecture.md): Tauri Desktop Architecture and Native Processing Boundary [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L748]
- [**ADR-003**](docs/adr/ADR-003-reverse-engineering-discipline-and-rebuild-gate.md): Reverse-Engineering Discipline and the Identity-Rebuild Gate [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L348]
- [**ADR-004**](docs/adr/ADR-004-declarative-theme-recipes-and-cross-train-rebasing.md): Declarative Theme Recipes and Cross-Train Rebasing [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L555]
- [**ADR-005**](docs/adr/ADR-005-navigation-cartography-fldb-compiler-and-multi-volume-partitioning.md): Navigation Cartography FLDB Compiler and Multi-Volume Partitioning [EV:doc:docs/research/Audi%20MMI%203G+%20Maps%20Research.md#L1]
- [**ADR-006**](docs/adr/ADR-006-full-system-firmware-qnx-nor-flash-packaging-and-swdl-manifests.md): Full System Firmware QNX NOR Flash Packaging and SWDL Manifests [EV:doc:docs/research/Audi%20MMI%203G:3G+%20infotainment%20Research.md#L1]
- [**ADR-007**](docs/adr/ADR-007-albanian-language-localization-and-typography-metrics-pipeline.md): Albanian Language Localization and Typography Metrics Pipeline [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L777]
- [**ADR-008**](docs/adr/ADR-008-in-car-sd-deployment-hardening-and-hardware-defense.md): In-Car SD Deployment Hardening, Hardware Defense, and Media Sanitization [EV:doc:docs/how-to/in-car-sd-update-guide.md#L1]

---

## Automotive Safety & Compliance

- **Current Status**: `BUILD READY — DEPLOYMENT NOT VERIFIED` [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L737]
- **Signed Asset Protection**: Binary payloads signed with factory digital certificates (`.pkg.sig`, `.dat.sig`, QNX IFS kernel) are strictly analysis-only and cannot be rebuilt or staged (§1.4) [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L348].
- **Safety Policy**: The phrase `"SAFE TO INSTALL"` is **permanently forbidden** from all documentation, UI strings, logs, and toolchain outputs (§14.9) [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L742].
- **Emergency Recovery**: Every packaged SD card bundle includes a standalone `stock_recovery.sh` for instant recovery over QNX UART serial console in the event of an update interruption [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L784].

---

## Contributing & License

- **Contributing**: Please review [`CONTRIBUTING.md`](CONTRIBUTING.md) for offline build requirements and code standards [EV:doc:CONTRIBUTING.md#L1].
- **Changelog**: See [`CHANGELOG.md`](CHANGELOG.md) for release history and milestone tracking [EV:doc:CHANGELOG.md#L1].
- **License**: Dual-licensed under the [MIT License](LICENSE) or the [Apache License, Version 2.0](LICENSE) [EV:doc:LICENSE#L1].
