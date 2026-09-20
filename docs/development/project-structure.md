# Development — Project Structure

This document details the layout of directories and key files across the **Audi MMI Studio** repository.

---

## High-Level Layout

```text
audi_mmi_studio/
├── apps/                               # Applications
│   ├── mmi-studio-cli/                 # Headless CLI application
│   │   ├── src/                        # CLI command implementations & main.rs
│   │   ├── tests/                      # CLI integration tests
│   │   └── README.md                   # CLI manual
│   └── mmi-studio-desktop/             # Tauri v2 + React 18 GUI application
│       ├── src/                        # React components (HexViewer, AssetBoard, etc.)
│       ├── src-tauri/                  # Rust native backend IPC bridge
│       ├── tests/                      # IPC roundtrip tests
│       └── README.md                   # Desktop manual
│
├── crates/                             # 12 Modular Rust Workspace Libraries
│   ├── mmi-core/                       # SourceStore, Content-Addressed Storage (CAS)
│   ├── mmi-formats/                    # Decoders for 12 automotive binary formats
│   ├── mmi-assets/                     # Bitmap codecs, format conformer, fonts
│   ├── mmi-canvas/                     # 800x480 pixel frame compositor
│   ├── mmi-re-lab/                     # Hex viewer, Shannon entropy, carving
│   ├── mmi-recipe/                     # Declarative JSON theming & rebase engine
│   ├── mmi-rebuild/                    # Identity-Rebuild Gate & stage normalizer
│   ├── mmi-validation/                 # 6-tier (L0-L5) automotive validation hierarchy
│   ├── mmi-media/                      # FAT32 SD builder & QNX update simulator
│   ├── mmi-attestation/                # Build attestation & stock recovery bundler
│   ├── mmi-imagegen/                   # Egress airlock & prompt sanitization
│   ├── mmi-plugin/                     # Untrusted plugin sandbox (ABI v1)
│   └── README.md                       # Master crate architecture & API guide
│
├── docs/                               # Canonical Documentation Architecture
│   ├── README.md                       # Master documentation portal
│   ├── getting-started/                # Onboarding & installation
│   ├── user-guide/                     # CLI, GUI, and workflow manuals
│   ├── how-to/                         # Task-oriented guides
│   ├── reference/                      # CLI reference, formats, APIs, schemas, glossary
│   ├── architecture/                   # System design, data flow, security model
│   ├── development/                    # Workspace setup, testing, release packaging
│   ├── operations/                     # Hardware deployment, monitoring, recovery
│   ├── decisions/                      # Architecture Decision Records (ADRs)
│   ├── audit/                          # Preserved corpus scan records
│   ├── research/                       # Preserved format research register
│   └── spec/                           # Master specifications & blueprints
│
├── originals/                          # Immutable reference firmware corpus (Read-Only)
├── recipes/                            # Declarative JSON theme recipes
├── scripts/                            # Verification & validation scripts
├── Cargo.toml                          # Workspace root manifest
├── Cargo.lock                          # Pinned dependency lockfile
├── CONTRIBUTING.md                     # Contributor guidelines
├── CHANGELOG.md                        # Version release history
├── LICENSE                             # Dual MIT / Apache-2.0 license
└── README.md                           # Project entry point
```
