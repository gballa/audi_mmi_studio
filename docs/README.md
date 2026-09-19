# Audi MMI Studio — Master Documentation Portal

This portal serves as the authoritative entry point and navigational index for all documentation across the **Audi MMI Studio** engineering platform [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L924].

---

## 1. User Intent Navigation ("I want to...")

Find the relevant documentation based on your immediate engineering goal [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L924]:

| I want to... | Target Guide / Section | Primary Audience |
| :--- | :--- | :--- |
| **Install and build the workstation** | [Getting Started: Installation](getting-started/installation.md) [EV:doc:docs/getting-started/installation.md#L1] | New Developers |
| **Run a 5-minute first exploration** | [Getting Started: Quick Start](getting-started/quick-start.md) [EV:doc:docs/getting-started/quick-start.md#L1] | All Users |
| **Learn how to use the CLI** | [User Guide: CLI](user-guide/cli.md) and [Reference: CLI](reference/cli.md) [EV:doc:apps/mmi-studio-cli/README.md#L1] | CLI Operators |
| **Learn how to use the Desktop GUI** | [User Guide: GUI](user-guide/gui.md) [EV:doc:apps/mmi-studio-desktop/README.md#L1] | UI Designers / Skinners |
| **Accomplish a specific engineering task** | [How-To Library Index](how-to/README.md) [EV:doc:docs/how-to/README.md#L1] | Engineers |
| **Look up reverse-engineered formats** | [Reference: File Formats (RQ-001..012)](reference/file-formats.md) [EV:doc:docs/research/RQ-REGISTER.md#L10] | Reverse Engineers |
| **Understand system architecture & crates** | [Architecture: Overview](architecture/overview.md) and [Crates Guide](../crates/README.md) [EV:doc:crates/README.md#L1] | Architects & Developers |
| **Develop, test, and debug code** | [Development: Setup](development/setup.md) and [Testing Strategy](development/testing.md) [EV:doc:docs/development/setup.md#L1] | Contributors |
| **Deploy updates to vehicle hardware** | [Operations: Deployment](operations/deployment.md) [EV:doc:docs/operations/deployment.md#L1] | Deployment Technicians |
| **Recover a head unit from a boot loop** | [Operations: Recovery](operations/recovery.md) and [How-To: Stock Recovery](how-to/emergency-stock-recovery.md) [EV:doc:docs/operations/recovery.md#L1] | Recovery Operators |
| **Review architectural decisions (ADRs)** | [Decisions: ADR Master Index](decisions/README.md) [EV:doc:docs/decisions/README.md#L1] | Architects |
| **Consult technical terms and acronyms** | [Reference: Glossary](reference/glossary.md) [EV:doc:docs/reference/glossary.md#L1] | All Readers |

---

## 2. Canonical Documentation Architecture Tree

```text
docs/
├── README.md                                 # Master documentation portal (this file)
│
├── getting-started/                          # Onboarding & first run
│   ├── prerequisites.md                      # Host OS, toolchain, hardware maintainer specs
│   ├── installation.md                       # Offline compilation for CLI and Desktop
│   ├── quick-start.md                        # 5-minute practical tutorial
│   ├── configuration.md                      # Workspace layout, CAS storage, target profiles
│   └── first-run.md                          # Workstation verification & safety verdicts
│
├── user-guide/                               # User manuals
│   ├── overview.md                           # Personas, interfaces, and core safety gates
│   ├── cli.md                                # Headless CLI guide & operational examples
│   ├── gui.md                                # Full desktop manual (5 studio panels)
│   ├── workflows.md                          # End-to-end user workflows
│   ├── configuration.md                      # Presets, theme recipes, and plugin configs
│   └── troubleshooting.md                    # Common user errors and resolutions
│
├── how-to/                                   # Procedural task guides
│   ├── README.md                             # How-To library index
│   ├── inspect-unknown-binary.md             # Format detection, entropy, and carving
│   ├── decode-and-replace-assets.md          # Extracting .precomp, editing, conforming
│   ├── create-and-rebase-theme-recipe.md     # Declarative JSON theming & drift rebase
│   ├── rebuild-and-validate-firmware.md      # Deterministic packing & 6-tier validation
│   ├── prepare-sd-deployment-media.md        # FAT32 32 KiB cluster geometry formatting
│   ├── simulate-qnx-update.md                # QNX pre-flight flashing simulation
│   ├── emergency-stock-recovery.md           # Stock bundle packaging & UART flashing
│   └── develop-format-plugin.md              # Writing ABI v1 format adapter plugins
│
├── reference/                                # Exhaustive technical specifications
│   ├── cli.md                                # Complete reference for all 22 CLI commands
│   ├── file-formats.md                       # Specifications for all 12 formats (RQ-001..012)
│   ├── api.md                                # Rust API reference across all 12 crates & IPC
│   ├── configuration.md                      # JSON schemas for recipes, profiles, plugins
│   ├── environment.md                        # Environment variables & isolation boundaries
│   ├── exit-codes.md                         # Process exit codes and handling
│   └── glossary.md                           # Automotive & reverse-engineering terms
│
├── architecture/                             # System design & principles
│   ├── overview.md                           # Core principles & scope boundaries
│   ├── components.md                         # 12 workspace crates + 2 apps hierarchy
│   ├── data-flow.md                          # Ingestion, CAS, StageStore, Rebuild pipelines
│   ├── runtime.md                            # Determinism, memory limits, and sandboxing
│   ├── deployment.md                         # Audi MMI 3G hardware topology & IPL stages
│   └── security.md                           # STRIDE threat model & Egress Airlock
│
├── development/                              # Engineering & contributor guides
│   ├── setup.md                              # Local developer environment setup
│   ├── project-structure.md                  # Comprehensive repository layout
│   ├── development-workflow.md               # Coding standards, branches, evidence tags
│   ├── testing.md                            # Testing tiers, E2E theming pipeline
│   ├── debugging.md                          # Tracing (RUST_LOG), panics, and hex debugging
│   └── release-process.md                    # Standalone packaging, checksums, and SBOM
│
├── operations/                               # Physical deployment & recovery runbooks
│   ├── deployment.md                         # Red Engineering Menu flashing procedures
│   ├── monitoring.md                         # Real-time update progress & UART monitoring
│   ├── backups.md                            # Baseline preservation, CAS immutability
│   ├── recovery.md                           # 3-button hard reset, emergency stock rollback
│   └── troubleshooting.md                    # Hardware error codes and mitigations
│
├── decisions/                                # Architectural decisions
│   ├── README.md                             # ADR index
│   └── adr/                                  # ADR-001 through ADR-007
│
├── audit/                                    # Reference corpus scan records
│   └── SOURCE_AUDIT.md                       # Scan of 63.97 GiB genuine reference corpus
├── research/                                 # Reverse engineering research records
│   ├── RQ-REGISTER.md                        # Master registry of 12 reverse engineering RQs
│   ├── "Audi MMI 3G+ Maps Research.md"       # Navigation FLDB, 544B pages, OSM & GMP analysis
│   └── "Audi MMI 3G:3G+ infotainment Research.md" # QNX NOR flash layout, SWDL & scripts analysis
└── spec/                                     # Master engineering specifications
    ├── AUDI_MMI_STUDIO_AGENT_PROMPT.md       # Master engineering specification
    ├── PROJECT.md                            # High-level architecture & scope
    ├── PROJECT_PLAN.md                       # 19-phase engineering roadmap
    ├── ORIGINAL_REQUEST.md                   # Initial user requirements
    └── CHECKPOINT_PAUSE.md                   # State transition checkpoint
```

---

## 3. Documentation Inventory & Status

All project documents are cataloged and categorized below [EV:doc:docs/README.md#L1]:

| Canonical Document | Category | Status | Primary Purpose |
| :--- | :--- | :--- | :--- |
| [`README.md`](../README.md) | Entry Point | Active | Root entry point and high-level directory map [EV:doc:README.md#L1] |
| [`CONTRIBUTING.md`](../CONTRIBUTING.md) | Governance | Active | Contributor guidelines, safety rules, pull request checklist [EV:doc:CONTRIBUTING.md#L1] |
| [`CHANGELOG.md`](../CHANGELOG.md) | Release Notes | Active | Release history following Keep-a-Changelog [EV:doc:CHANGELOG.md#L1] |
| [`LICENSE`](../LICENSE) | Legal | Active | Dual MIT / Apache-2.0 license file [EV:doc:LICENSE#L1] |
| [`crates/README.md`](../crates/README.md) | Reference | Active | Crate layer breakdown and public API reference [EV:doc:crates/README.md#L1] |
| [`apps/mmi-studio-cli/README.md`](../apps/mmi-studio-cli/README.md) | Manual | Active | Headless CLI manual and command reference [EV:doc:apps/mmi-studio-cli/README.md#L1] |
| [`apps/mmi-studio-desktop/README.md`](../apps/mmi-studio-desktop/README.md) | Manual | Active | Desktop GUI architecture and panel reference [EV:doc:apps/mmi-studio-desktop/README.md#L1] |
| [`docs/getting-started/*`](getting-started/quick-start.md) | Getting Started | Active | Onboarding, prerequisites, installation, and first run [EV:doc:docs/getting-started/quick-start.md#L1] |
| [`docs/user-guide/*`](user-guide/overview.md) | User Guide | Active | CLI and GUI usage guides, workflows, troubleshooting [EV:doc:docs/user-guide/overview.md#L1] |
| [`docs/how-to/*`](how-to/README.md) | How-To | Active | Task-focused procedural guides [EV:doc:docs/how-to/README.md#L1] |
| [`docs/reference/*`](reference/file-formats.md) | Reference | Active | CLI syntax, 12 binary formats, APIs, schemas, glossary [EV:doc:docs/reference/file-formats.md#L1] |
| [`docs/architecture/*`](architecture/overview.md) | Architecture | Active | System design, components, data flows, security [EV:doc:docs/architecture/overview.md#L1] |
| [`docs/development/*`](development/setup.md) | Development | Active | Developer setup, coding rules, testing, release packaging [EV:doc:docs/development/setup.md#L1] |
| [`docs/operations/*`](operations/deployment.md) | Operations | Active | In-vehicle deployment, monitoring, emergency recovery [EV:doc:docs/operations/deployment.md#L1] |
| [`docs/decisions/*`](decisions/README.md) | Decisions | Active | Architecture Decision Records (ADR-001..ADR-007) [EV:doc:docs/decisions/README.md#L1] |
| [`docs/audit/SOURCE_AUDIT.md`](audit/SOURCE_AUDIT.md) | Audit | Preserved | Complete scan of 63.97 GiB reference corpus [EV:doc:docs/audit/SOURCE_AUDIT.md#L1] |
| [`docs/research/RQ-REGISTER.md`](research/RQ-REGISTER.md) | Research | Preserved | Research register for 12 reverse engineering questions [EV:doc:docs/research/RQ-REGISTER.md#L1] |
| [`docs/spec/*`](spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md) | Specification | Preserved | Master prompt specification and project roadmap [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L1] |

---

## 4. Documentation Gap & Drift Report

### Gap Analysis
- **Finding**: All 12 reverse engineering formats (RQ-001 through RQ-012) have complete format specifications, unit decoders, and integration tests [EV:doc:docs/research/RQ-REGISTER.md#L10].
- **Finding**: All 22 CLI subcommands are documented with argument, option, and exit code tables [EV:doc:docs/reference/cli.md#L1].
- **Identified Gap**: Hardware-specific timings for physical UART unbricking vary slightly between MMI 3G High (2009–2012) and MMI 3G Plus (2012–2016) hardware revisions [EV:doc:docs/operations/monitoring.md#L15].
- **Recommendation**: Maintain oscilloscope trace logs and logic analyzer captures in a future `docs/operations/hardware-traces/` directory when bench units are profiled [EV:doc:docs/operations/monitoring.md#L15].

### Drift Analysis
- **Finding**: The former root README contained deep technical specifications that drifted from individual crate APIs [EV:doc:docs/spec/PROJECT.md#L10].
- **Resolution**: Streamlined the root `README.md` to serve as a clean project entry point, delegating technical reference to `docs/reference/` and `crates/README.md` [EV:doc:README.md#L1].
- **Finding**: Previous ad-hoc scripts used inconsistent exit codes [EV:doc:docs/reference/exit-codes.md#L1].
- **Resolution**: Standardized exit codes across `mmi-studio-cli`: `0` (Success), `1` (General error), `2` (Syntax), and `3` (Signed payload lock) [EV:doc:docs/reference/exit-codes.md#L1].

---

## 5. Documentation Maintenance Guide

Future contributors must follow these rules when updating documentation [EV:doc:CONTRIBUTING.md#L1]:
1. **Single Source of Truth**: Source code and unit tests are the source of truth for CLI flags, schemas, and API signatures [EV:doc:CONTRIBUTING.md#L30].
2. **Evidence Tagging Discipline**: Every normative statement in core architectural documentation must include a valid evidence tag (`[EV:...]`, `[INF:...]`, `[UNK]`) [EV:doc:CONTRIBUTING.md#L50].
3. **No Broken Links**: Run `/usr/bin/python3 scripts/check-docs-links.py` prior to submitting changes to verify all internal links and anchors resolve [EV:doc:CONTRIBUTING.md#L60].
4. **Banned Terminology**: Never use the phrase `"SAFE TO INSTALL"` anywhere in documentation or code (§14.9) [EV:doc:docs/spec/AUDI_MMI_STUDIO_AGENT_PROMPT.md#L742].
