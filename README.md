# Audi MMI Studio

> **Audi MMI Studio is an offline-first engineering workstation for deep structural analysis, reverse-engineering, visual exploration, controlled modification, and deterministic repackaging of Audi Multi Media Interface (MMI 3G+ / HN+) infotainment software packages.** [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L6]

---

## Key Capabilities

- **100% Offline Execution**: Zero telemetry, zero external network dependencies, and airlocked execution boundaries [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L948].
- **12 Proprietary Automotive Formats Resolved**: Native decoders for navigation databases, Orion atlas tiles, UI graphics (`.precomp`), map styles, QNX IFS/EFS images, speech models, FPGA bitstreams, and DSP loaders [EV:doc:docs/research/RQ-REGISTER.md#L10].
- **Reverse-Engineering Laboratory**: Virtualized hex viewer, sliding Shannon entropy profiler, byte histogram analyzer, string extractor, and signature carver [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L776].
- **Visual Theming & Screen Simulation**: Pixel-perfect 800x480 MMI display synthesis supporting Day, Night, and Reduced palettes [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L779].
- **Declarative Theme Recipes & Rebasing**: Non-destructive JSON-based theming engine with cross-train drift analysis and cryptographic journaling [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L782].
- **Deterministic Rebuild & Automotive Safety Gates**: Bit-for-bit rebuild verification, immutable signed payload locks, and 6-tier (`L0`–`L5`) validation hierarchy [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L783].
- **Hardware Deployment & Emergency Rollback**: FAT32 SD card builder (32 KiB cluster geometry), QNX pre-flight update simulator, and automated stock recovery packager [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L784].

---

## Status

- **Release**: `v0.1.0` (Workstation Verification Passed 100% offline) [EV:doc:CHANGELOG.md#L13]
- **Automotive Verification Verdict**: `BUILD READY — DEPLOYMENT NOT VERIFIED` [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L737]
- **Safety Policy**: The phrase `"SAFE TO INSTALL"` is permanently forbidden (§14.9) [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L742].

---

## Requirements

- **Operating System**: macOS, Linux, or Windows (x86_64 or Apple Silicon / ARM64) [EV:doc:docs/getting-started/prerequisites.md#L10].
- **Rust Toolchain**: 1.80+ (stable toolchain with offline dependencies cached) [EV:doc:docs/getting-started/prerequisites.md#L15].
- **Node.js**: 18+ and `npm` (required for desktop GUI frontend) [EV:doc:docs/getting-started/prerequisites.md#L20].
- **Reference Corpus**: Lawfully acquired MMI 3G+ firmware update packages in `originals/` [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L926].

---

## Quick Start

Build and verify the workstation in four steps [EV:doc:docs/getting-started/quick-start.md#L1]:

```bash
# 1. Clone the repository
git clone <repo-url>
cd AudiMMI

# 2. Build the optimized CLI binary (100% offline)
cargo build --release -p mmi-studio-cli --offline

# 3. Run a quick binary inspection against the reference corpus
./target/release/mmi-studio-cli inspect originals/MU9411/ScreenLayouts/CombiStyles.precomp

# 4. Run the full workstation verification gate
./scripts/verify-workstation.sh
```

For a comprehensive walkthrough, see the [Quick Start Guide](docs/getting-started/quick-start.md) [EV:doc:docs/getting-started/quick-start.md#L1].

---

## Applications & Entry Points

Audi MMI Studio provides both headless and graphical interfaces [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L748]:

### 1. Headless CLI (`mmi-studio-cli`)
Production command-line workstation for reverse engineering, asset transformation, rebuild gating, and media packaging [EV:doc:apps/mmi-studio-cli/README.md#L1]:
```bash
./target/release/mmi-studio-cli --help
```
Read the full [`mmi-studio-cli` Guide](apps/mmi-studio-cli/README.md) [EV:doc:apps/mmi-studio-cli/README.md#L1].

### 2. Desktop GUI (`mmi-studio-desktop`)
Tauri v2 + React 18 graphical workstation featuring HexViewer, AssetBoard, ScreenCanvas (800x480 simulator), RecipeStudio, and TypographyStudio [EV:doc:apps/mmi-studio-desktop/README.md#L1]:
```bash
cd apps/mmi-studio-desktop && npm run dev
```
Read the full [`mmi-studio-desktop` Guide](apps/mmi-studio-desktop/README.md) [EV:doc:apps/mmi-studio-desktop/README.md#L1].

### 3. Modular Crates (`crates/`)
12 decoupled Rust libraries providing the underlying storage, decoders, engines, and validators [EV:doc:crates/README.md#L1]:
Read the [Workspace Crates Architecture](crates/README.md) [EV:doc:crates/README.md#L1].

---

## Documentation Navigation

The complete documentation system is organized under [`docs/`](docs/README.md) [EV:doc:docs/README.md#L1]:

| Section | Description | Target Link |
| :--- | :--- | :--- |
| **Getting Started** | Prerequisites, installation, quick-start, and first run | [`docs/getting-started/`](docs/getting-started/quick-start.md) [EV:doc:docs/getting-started/quick-start.md#L1] |
| **User Guide** | Detailed user manual for CLI, GUI panels, workflows, and configuration | [`docs/user-guide/`](docs/user-guide/overview.md) [EV:doc:docs/user-guide/overview.md#L1] |
| **How-To Guides** | Task-oriented step-by-step guides for reverse engineering and theming | [`docs/how-to/`](docs/how-to/README.md) [EV:doc:docs/how-to/README.md#L1] |
| **Reference** | Exhaustive CLI reference, file formats, Rust APIs, schemas, and glossary | [`docs/reference/`](docs/reference/file-formats.md) [EV:doc:docs/reference/file-formats.md#L1] |
| **Architecture** | System principles, crate components, data flows, and security model | [`docs/architecture/`](docs/architecture/overview.md) [EV:doc:docs/architecture/overview.md#L1] |
| **Development** | Setup, project structure, testing workflows, and release packaging | [`docs/development/`](docs/development/setup.md) [EV:doc:docs/development/setup.md#L1] |
| **Operations** | Hardware SD media flashing, monitoring, emergency recovery, and rollback | [`docs/operations/`](docs/operations/deployment.md) [EV:doc:docs/operations/deployment.md#L1] |
| **Decisions (ADRs)** | Architecture Decision Records (ADR-001 through ADR-004) | [`docs/decisions/`](docs/decisions/README.md) [EV:doc:docs/decisions/README.md#L1] |

---

## Development & Contributing

- **Development Guide**: See [`docs/development/setup.md`](docs/development/setup.md) for local workspace configuration and testing [EV:doc:docs/development/setup.md#L1].
- **Contributing Guidelines**: See [`CONTRIBUTING.md`](CONTRIBUTING.md) for pull request rules and safety constraints [EV:doc:CONTRIBUTING.md#L1].
- **Changelog**: See [`CHANGELOG.md`](CHANGELOG.md) for release notes and version history [EV:doc:CHANGELOG.md#L1].

---

## License

Audi MMI Studio is open-source software dual-licensed under the [MIT License](LICENSE) or the [Apache License, Version 2.0](LICENSE) [EV:doc:LICENSE#L1].
