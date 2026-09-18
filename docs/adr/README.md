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
