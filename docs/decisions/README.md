# Architecture Decision Records (ADRs)

This directory documents the foundational architectural and design decisions governing the **Audi MMI Studio** platform.

---

## Index of Architectural Decisions

| ADR ID | Title | Status | Date | Primary Focus |
| :--- | :--- | :--- | :--- | :--- |
| [**ADR-001**](adr/ADR-001-offline-first-workspace-architecture.md) | Offline-First Modular Rust Workspace Architecture | ACCEPTED | 2026-09-18 | Multi-crate Rust workspace, SourceStore immutability, zero network egress. |
| [**ADR-002**](adr/ADR-002-tauri-desktop-architecture.md) | Tauri Desktop Architecture and Native Processing Boundary | ACCEPTED | 2026-09-18 | Tauri v2 + React 18 / TypeScript GUI, native IPC boundary, local assets. |
| [**ADR-003**](adr/ADR-003-reverse-engineering-discipline-and-rebuild-gate.md) | Reverse-Engineering Discipline and the Identity-Rebuild Gate | ACCEPTED | 2026-09-18 | RQ-REGISTER, format adapters, two-tier rebuild gate, signed payload protection. |
| [**ADR-004**](adr/ADR-004-declarative-theme-recipes-and-cross-train-rebasing.md) | Declarative Theme Recipes and Cross-Train Rebasing | ACCEPTED | 2026-09-18 | Declarative JSON recipes, semantic selectors, cryptographic audit journals, rebase engine. |
