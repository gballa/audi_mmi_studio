# Architecture Decision Records (ADRs)

This directory documents the foundational architectural and design decisions governing the **Audi MMI Studio** platform.

---

## Index of Architectural Decisions

| ADR ID | Title | Status | Date | Primary Focus |
| :--- | :--- | :--- | :--- | :--- |
| [**ADR-001**](adr/ADR-001-offline-first-workspace-architecture.md) | Offline-First Modular Rust Workspace Architecture | ACCEPTED | 2026-09-18 | Multi-crate Rust workspace, SourceStore immutability, zero network egress. |
| [**ADR-002**](adr/ADR-002-tauri-desktop-architecture.md) | Tauri Desktop Architecture and Native Processing Boundary | ACCEPTED | 2026-09-18 | Tauri v2 + React 19 / TypeScript GUI, native IPC boundary, local assets. |
| [**ADR-003**](adr/ADR-003-reverse-engineering-discipline-and-rebuild-gate.md) | Reverse-Engineering Discipline and the Identity-Rebuild Gate | ACCEPTED | 2026-09-18 | RQ-REGISTER, format adapters, two-tier rebuild gate, signed payload protection. |
| [**ADR-004**](adr/ADR-004-declarative-theme-recipes-and-cross-train-rebasing.md) | Declarative Theme Recipes and Cross-Train Rebasing | ACCEPTED | 2026-09-18 | Declarative JSON recipes, semantic selectors, cryptographic audit journals, rebase engine. |
| [**ADR-005**](adr/ADR-005-navigation-cartography-fldb-compiler-and-multi-volume-partitioning.md) | Navigation Cartography FLDB Compiler and Multi-Volume Partitioning | ACCEPTED | 2026-09-19 | 544-byte FLDB physical pages, CRC-16/CCITT, OSM ingestion, GMP ToS compliance, 2 GiB FAT32 splitting. |
| [**ADR-006**](adr/ADR-006-full-system-firmware-qnx-nor-flash-packaging-and-swdl-manifests.md) | Full System Firmware QNX NOR Flash Packaging and SWDL Manifests | ACCEPTED | 2026-09-19 | QnxIfsBuilder, QnxEfsBuilder, MetaInfo2Builder with 512KB CRC32 blocks, NOR flash bounds, script launchers. |
| [**ADR-007**](adr/ADR-007-albanian-language-localization-and-typography-metrics-pipeline.md) | Albanian Language Localization and Typography Metrics Pipeline | ACCEPTED | 2026-09-19 | Albanian (sq_AL) Harman ANS binary catalog, TrueType font metric validation, and UI clipping prevention. |
