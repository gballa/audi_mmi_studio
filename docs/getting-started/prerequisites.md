# Getting Started — Prerequisites

This document outlines the hardware, system software, toolchains, and reference data required to build, test, and run **Audi MMI Studio**.

---

## 1. Operating System & Host Environment

Audi MMI Studio is cross-platform and fully supported on:
- **macOS**: Sonoma (14.x) or Sequoia (15.x) on Apple Silicon (M1/M2/M3/M4) or x86_64.
- **Linux**: Ubuntu 22.04 LTS / 24.04 LTS, Debian 12, Fedora 39/40, or Arch Linux.
- **Windows**: Windows 11 (64-bit) with MSVC C++ Build Tools.

Hardware recommendations:
- **CPU**: 4+ physical cores recommended for concurrent test execution.
- **Memory**: Minimum 8 GB RAM (16 GB recommended for high-volume CAS caching and disk image carving).
- **Disk Storage**: At least 80 GB available space if hosting the full genuine MMI 3G+ firmware corpus (`originals/`).

---

## 2. Software Toolchains & Dependencies

### Rust Toolchain (Core & CLI)
- **Version**: Rust 1.80+ stable (tested up to 1.98.1).
- **Offline Cache**: All workspace dependencies are declared in `Cargo.lock` and vendored or pre-cached in Cargo cache.
- **Verification**:
  ```bash
  rustc --version
  cargo --version
  ```

### Node.js & Webview (Desktop GUI)
- **Node.js**: Version 18.0 LTS or 20.0 LTS.
- **Package Manager**: `npm` (v9+).
- **System Webview**:
  - macOS: Built-in WKWebView.
  - Linux: `webkit2gtk-4.1` (`libwebkit2gtk-4.1-dev`).
  - Windows: Microsoft Edge WebView2.

### Python Runtime (Validation Tooling)
- **Python**: 3.9+ with standard library (`argparse`, `pathlib`, `re`, `json`).

---

## 3. Reference Firmware Corpus (`originals/`)

Audi MMI Studio operates against lawfully possessed Audi MMI 3G+ update media stored in the `originals/` directory.

- **Storage Structure**:
  - `originals/MU9411/`: Main unit software train package (HN+ / HN+R).
  - Subdirectories include `HBNavDB/`, `ScreenLayouts/`, `Speech/`, `FPGA/`, `DSP/`.
- **Immutability Guarantee**:
  - The workstation mounts `originals/` in strict read-only mode via `SourceStore`.
  - Never place temporary files or build outputs inside `originals/`.

---

## 4. In-Vehicle Deployment Prerequisites (Hardware Flashing)

If staging SD media for deployment to an Audi MMI 3G+ head unit:
- **SD Card**: High-quality Full-Size SD/SDHC card (Class 10 / UHS-I, 8 GB to 32 GB capacity).
- **Filesystem**: FAT32 formatted with exact 32 KiB cluster geometry.
- **Power Supply**: Automotive battery maintainer (minimum 13.5V continuous, 25A–40A rating) connected to the vehicle jump posts. Never perform head unit firmware flashing on battery power alone.
