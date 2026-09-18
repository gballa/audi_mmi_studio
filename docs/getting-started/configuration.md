# Getting Started — Configuration

This document explains the runtime directory structure, storage layers, Content-Addressed Storage (CAS), and target profiles in **Audi MMI Studio**.

---

## 1. Workspace Filesystem Layout

Audi MMI Studio uses a deterministic, isolated workspace layout:

```text
AudiMMI/
├── .mmistudio/                         # Local workstation storage
│   ├── cas/                            # Content-Addressed Storage (BLAKE3-indexed blobs)
│   ├── stages/                         # Transactional staging workspaces (StageStore)
│   │   └── default/                    # Active staging directory
│   ├── plugins/                        # Third-party format adapter plugins
│   └── profiles/                       # Target hardware and firmware profiles
├── originals/                          # Immutable reference firmware corpus (Read-Only)
├── output/                             # Rebuild and deployment media output folder
└── recipes/                            # Declarative JSON theming recipes
```

---

## 2. Storage System Layers

The workstation manages data across three strictly segregated storage layers:

1. **`originals/` (Corpus Immutability)**:
   - Contains genuine, untouched firmware releases (e.g. `MU9411/`).
   - Managed by `SourceStore` in `mmi-core`. Writes are strictly prohibited.
2. **`.mmistudio/cas/` (Content-Addressed Storage)**:
   - Stores deduplicated blobs indexed by their BLAKE3 cryptographic hash.
   - Used for asset thumbnails, intermediate decoded bitmaps, and audit artifacts.
3. **`.mmistudio/stages/` (StageStore)**:
   - An isolated copy-on-write staging environment where candidate replacements and theme recipes are applied.
   - Changes here never affect `originals/`.

---

## 3. Environment Variables

| Variable | Default | Purpose |
| :--- | :--- | :--- |
| `CARGO_NET_OFFLINE` | `true` | Enforces offline builds across all Cargo invocations. |
| `RUST_LOG` | `info` | Configures logging verbosity (`error`, `warn`, `info`, `debug`, `trace`). |
| `MMI_STUDIO_DIR` | `.mmistudio` | Root directory for local CAS, staging, and plugin stores. |
| `MMI_ORIGINALS_DIR` | `originals` | Root path to genuine firmware packages. |

---

## 4. Target Hardware Profiles

Target profiles define the hardware specifications, partition limits, and firmware versions against which candidate updates are evaluated during Level 5 (`L5`) validation.

Example profile (`.mmistudio/profiles/mmi3g_high_eu.json`):
```json
{
  "profile_id": "mmi3g_high_eu",
  "name": "Audi MMI 3G High / Plus (EU/AU)",
  "hardware_variant": "9411",
  "display_resolution": {
    "width": 800,
    "height": 480
  },
  "max_ifs_size_bytes": 16777216,
  "max_efs_size_bytes": 67108864,
  "supported_trains": [
    "HN+R_EU_AU_K0942_4_[8R0906961FB]",
    "HN+_EU_AU_K0900"
  ]
}
```

Profiles are loaded during validation with the `--profile` flag:
```bash
./target/release/mmi-studio-cli validate output/candidate_stage --profile .mmistudio/profiles/mmi3g_high_eu.json
```
