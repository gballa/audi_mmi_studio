# Architecture Decision Records (ADRs)

This directory documents the foundational architectural and design decisions governing the Audi MMI Studio workstation [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L924].

---

## Index of Architectural Decisions

| ADR ID | Title | Status | Date | Primary Focus |
|---|---|---|---|---|
| [ADR-001](ADR-001-offline-first-workspace-architecture.md) | Offline-First Modular Rust Workspace Architecture | ACCEPTED | 2026-09-18 | Multi-crate Rust workspace, SourceStore immutability, zero network egress [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L771] |
| [ADR-002](ADR-002-tauri-desktop-architecture.md) | Tauri Desktop Architecture and Native Processing Boundary | ACCEPTED | 2026-09-18 | Tauri v2 + React 19 / TypeScript GUI, native IPC boundary, local assets [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L748] |
| [ADR-003](ADR-003-reverse-engineering-discipline-and-rebuild-gate.md) | Reverse-Engineering Discipline and the Identity-Rebuild Gate | ACCEPTED | 2026-09-18 | RQ-REGISTER, Kaitai Struct schemas, two-tier rebuild gate, signed payload protection [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L348] |
| [ADR-004](ADR-004-declarative-theme-recipes-and-cross-train-rebasing.md) | Declarative Theme Recipes and Cross-Train Rebasing | ACCEPTED | 2026-09-18 | Declarative JSON recipes, semantic selectors, cryptographic audit journals, rebase engine [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L555] |
| [ADR-005](ADR-005-navigation-cartography-fldb-compiler-and-multi-volume-partitioning.md) | Navigation Cartography FLDB Compiler and Multi-Volume Partitioning | ACCEPTED | 2026-09-19 | 544-byte FLDB physical pages, CRC-16/CCITT, OSM ingestion, GMP ToS compliance, 2 GiB FAT32 splitting [EV:doc:docs/research/Audi%20MMI%203G+%20Maps%20Research.md#L1] |
| [ADR-006](ADR-006-full-system-firmware-qnx-nor-flash-packaging-and-swdl-manifests.md) | Full System Firmware QNX NOR Flash Packaging and SWDL Manifests | ACCEPTED | 2026-09-19 | QnxIfsBuilder, QnxEfsBuilder, MetaInfo2Builder with 512KB CRC32 blocks, NOR flash bounds, script launchers [EV:doc:docs/research/Audi%20MMI%203G:3G+%20infotainment%20Research.md#L1] |
| [ADR-007](ADR-007-albanian-language-localization-and-typography-metrics-pipeline.md) | Albanian Language Localization and Typography Metrics Pipeline | ACCEPTED | 2026-09-19 | Albanian (sq_AL) Harman ANS binary catalog, TrueType font metric validation, and UI clipping prevention [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L777] |
| [ADR-008](ADR-008-in-car-sd-deployment-hardening-and-hardware-defense.md) | In-Car SD Deployment Hardening, Hardware Defense, and Media Sanitization | ACCEPTED | 2026-09-19 | QNX 6.3 userland shims, pci-3g hardware defense, F3S reclaim lock, Keldo map unblocker, MediaSanitizer [EV:doc:docs/how-to/in-car-sd-update-guide.md#L1] |
