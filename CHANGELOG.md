# Changelog

All notable changes to **Audi MMI Studio** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

---

## [0.1.0] - 2026-09-18

### Added
- **Foundational Workspace Architecture**:
  - Implemented 12 modular Rust workspace crates under `crates/`.
  - Immutable corpus reader (`SourceStore`) and Content-Addressed Storage (`ContentAddressedStore`) with BLAKE3 in `mmi-core`.
  - Identity-Rebuild Gate and bit-for-bit determinism verifier in `mmi-rebuild`.
  - 6-tier (`L0` to `L5`) automotive validation engine in `mmi-validation`.
- **Proprietary Binary Format Reverse Engineering (RQ-001 through RQ-012)**:
  - Decoders for all 12 discovered automotive binary formats:
    - HBNavDB (`.db`)
    - Orion ATLAS (`.atlas`)
    - CombiStyles (`.precomp`)
    - MapStyles (`.xar`)
    - QNX 6 IFS (`.ifs`)
    - QNX 6 EFS / F3S (`.efs`)
    - Speech Prompts (`.ans`)
    - System FPGA (`.hbbin`)
    - SMSC MOST INIC (`.ipf`)
    - Geographic Routing (`.gdb`)
    - Binary Grammar (`.hbgr`)
    - ADI Blackfin DSP (`.ldr`)
- **Reverse-Engineering Laboratory (`mmi-re-lab`)**:
  - Virtualized hex viewer with ASCII representation.
  - Sliding-window Shannon entropy calculator with region classification.
  - Byte histogram profiling and string extractor.
  - Signature carver for embedded file systems, containers, and graphics.
- **Visual Assets & Screen Simulation**:
  - Precomp graphic decoder and encoder in `mmi-assets`.
  - Asset conformer with Lanczos3 resampler and dimension constraint checker.
  - 800x480 pixel-perfect MMI display compositor in `mmi-canvas` supporting Day, Night, and Reduced palette modes.
- **Declarative Theming & Cross-Train Rebasing (`mmi-recipe`)**:
  - JSON theme recipe model with risk classifications (`COSMETIC`, `CONTENT`, `STRUCTURAL`).
  - Cryptographic journaling and tampering detection.
  - Cross-train rebasing engine evaluating asset drift across firmware releases.
  - Pre-built theme recipes: `audi_sport_amber.json`, `rs_performance_red.json`, `dark_line_minimalist.json`.
- **Hardware Deployment & Media Packaging**:
  - FAT32 SD card media packaging engine with 32 KiB cluster geometry alignment in `mmi-media`.
  - Dynamic volume splitting for updates exceeding 32GB SD card capacity.
  - QNX head-unit pre-flight update simulator.
  - Build attestation manifest generator and emergency stock recovery packager in `mmi-attestation`.
- **Extensibility & Security Airlock**:
  - Strict Egress Airlock in `mmi-imagegen` with brand/PII prompt sanitization and offline mock provider.
  - Versioned plugin SDK (`ABI_VERSION = 1`) with memory-isolated sandbox execution in `mmi-plugin`.
- **Applications**:
  - Headless CLI application `mmi-studio-cli` exposing 22 subcommands with JSON output support.
  - Desktop GUI `mmi-studio-desktop` built on Tauri v2 with React 18, featuring HexViewer, AssetBoard, ScreenCanvas, RecipeStudio, and TypographyStudio panels.
- **Automated Verification Gate**:
  - Offline workstation verification script `./scripts/verify-workstation.sh` covering evidence tagging, recipe validation, workspace check, unit tests, integration tests, E2E theming pipeline, and release binary verification.
