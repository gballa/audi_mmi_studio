# User Guide — Overview

Welcome to the **Audi MMI Studio User Guide**. This guide provides an overview of the platform's user roles, primary interface options, and end-to-end capabilities.

---

## 1. User Personas

Audi MMI Studio is designed to serve four specialized engineering roles:

| Persona | Primary Goal | Recommended Interfaces |
| :--- | :--- | :--- |
| **Reverse Engineer** | Analyze unknown binary formats, identify embedded headers, calculate entropy, and write decoders. | `mmi-studio-cli hexdump`, `entropy`, `carve`; Desktop `HexViewer`. |
| **UI/UX Skinner** | Create custom visual themes, adjust color palettes, substitute fonts, and preview 800x480 screen layouts. | Desktop `ScreenCanvas`, `AssetBoard`, `RecipeStudio`; `mmi-studio-cli recipe apply`. |
| **Firmware Auditor** | Verify release determinism, inspect digital signatures, evaluate cross-train drift, and run 6-tier validation. | `mmi-studio-cli validate`, `verify-rebuild`, `recipe rebase`. |
| **Deployment Technician** | Format FAT32 SD media, simulate head-unit update procedures, and prepare emergency stock rollback bundles. | `mmi-studio-cli build-media`, `simulate-update`, `stock-recovery`. |

---

## 2. Choosing Your Interface

### Headless CLI (`mmi-studio-cli`)
Best for:
- Automated scripts, batch processing, and CI pipelines.
- Headless servers or environments without graphical webviews.
- Direct integration into shell scripts.

Read the complete [CLI User Guide](cli.md).

### Graphical Desktop GUI (`mmi-studio-desktop`)
Best for:
- Interactive visual asset exploration and palette comparisons.
- High-resolution (800x480) MMI screen preview in Day, Night, and Reduced modes.
- Visual theme recipe authoring with real-time feedback.

Read the complete [GUI User Guide](gui.md).

---

## 3. Core Safety Gates

Before modifying or packaging any automotive software, understand the workstation's safety architecture:

```text
┌─────────────────────────────────────────────────────────────┐
│                   Immutable originals/                      │
└──────────────────────────────┬──────────────────────────────┘
                               │ SourceStore (Read-Only)
┌──────────────────────────────▼──────────────────────────────┐
│                    Rebuild Gate Verification                │
│  - Unsigned Assets: Stage Permitted                         │
│  - Signed Binaries (.pkg.sig, IFS): LOCKED (Analysis-Only)  │
└──────────────────────────────┬──────────────────────────────┘
                               │
┌──────────────────────────────▼──────────────────────────────┐
│                  6-Tier Validation (L0 - L5)                │
│  L0: Magic Bytes    L1: Structural    L2: Asset Conformance │
│  L3: UI Margins     L4: Determinism   L5: Target Profile    │
└──────────────────────────────┬──────────────────────────────┘
                               │
┌──────────────────────────────▼──────────────────────────────┐
│             Deployable FAT32 SD Media (Aligned)             │
│        Status: BUILD READY — DEPLOYMENT NOT VERIFIED        │
└─────────────────────────────────────────────────────────────┘
```

Continue reading:
- [CLI User Guide](cli.md)
- [GUI User Guide](gui.md)
- [Common Workflows](workflows.md)
