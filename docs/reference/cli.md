# Reference — Command Line Interface (`mmi-studio-cli`)

This document provides the exhaustive technical reference for all 22 subcommands supported by `mmi-studio-cli`.

---

## Global Invocations & Conventions

```bash
mmi-studio-cli [SUBCOMMAND] [OPTIONS]
```

- Standard output is plain human-readable text by default.
- Adding `--json` emits structured JSON directly to `stdout`.
- Operational logs and diagnostic messages are written to `stderr`.

---

## 1. `inspect`
Detects file format, parses header fields, computes cryptographic hashes, and determines signature lock status.

### Syntax
```bash
mmi-studio-cli inspect <FILE> [--json]
```

### Arguments & Options
| Parameter | Type | Required | Description |
| :--- | :--- | :--- | :--- |
| `FILE` | Path | Yes | Path to target binary file. |
| `--json` | Flag | No | Emit output as JSON. |

### Exit Codes
- `0`: Inspection succeeded.
- `1`: File not found or read failure.

---

## 2. `hexdump`
Renders a virtualized hex dump of a byte slice with ASCII sidebar representation.

### Syntax
```bash
mmi-studio-cli hexdump <FILE> [-o <OFFSET>] [-l <LENGTH>]
```

### Arguments & Options
| Parameter | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `FILE` | Path | Required | Target binary file. |
| `-o`, `--offset` | usize | `0` | Byte offset to start dumping from. |
| `-l`, `--length` | usize | `256` | Number of bytes to dump. |

---

## 3. `entropy`
Calculates sliding-window Shannon entropy (0.0 to 8.0) and segments regions into plaintext, structured data, compressed streams, or encrypted payloads.

### Syntax
```bash
mmi-studio-cli entropy <FILE> [-w <WINDOW>] [--json]
```

### Arguments & Options
| Parameter | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `FILE` | Path | Required | Target binary file. |
| `-w`, `--window` | usize | `1024` | Sliding analysis window size in bytes. |
| `--json` | Flag | `false` | Emit segment breakdown as JSON. |

---

## 4. `carve`
Scans binary data for embedded signatures, containers, filesystems (IFS, EFS), and image headers.

### Syntax
```bash
mmi-studio-cli carve <FILE> [--json]
```

---

## 5. `extract`
Ingests an update package into Content-Addressed Storage (CAS) and populates a StageStore workspace.

### Syntax
```bash
mmi-studio-cli extract <SOURCE> [-s <STAGE>] [-o <OUTPUT_PROJECT>] [--json]
```

### Arguments & Options
| Parameter | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `SOURCE` | Path | Required | Path to source update directory or container. |
| `-s`, `--stage` | String | `"default"` | Destination stage identifier. |
| `-o`, `--output-project`| Path | None | Path to save normalized `MMIProject` JSON. |

---

## 6. `assets`
Subcommand suite for visual graphics and font assets.

### `assets list`
```bash
mmi-studio-cli assets list [-s <STAGE>] [--json]
```

### `assets export`
```bash
mmi-studio-cli assets export <ASSET> -o <OUTPUT_PNG>
```

### `assets replace`
```bash
mmi-studio-cli assets replace -t <TARGET> -r <REPLACEMENT_IMG> -o <OUTPUT> [--json]
```

---

## 7. `strings`
Subcommand suite for localized string catalog analysis and typography overflow checking.

### `strings inspect`
```bash
mmi-studio-cli strings inspect <CATALOG_FILE> [--json]
```

### `strings check-overflow`
```bash
mmi-studio-cli strings check-overflow -t <TEXT> -f <FONT_TTF> [--max-width <PX>] [--font-size <PT>] [--json]
```

---

## 8. `render-screen`
Composes a full 800x480 high-resolution MMI display frame into a standard PNG image.

### Syntax
```bash
mmi-studio-cli render-screen [-s <SCREEN>] [-m <MODE>] -o <OUTPUT_PNG>
```

### Arguments & Options
| Parameter | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `-s`, `--screen` | String | `"MAIN_SCREEN"` | Target screen identifier. |
| `-m`, `--mode` | String | `"day"` | Palette mode: `day`, `night`, `reduced`. |
| `-o`, `--output` | Path | Required | Output PNG destination. |

---

## 9. `ai-generate`
Generates visual assets via the isolated Egress Airlock.

### Syntax
```bash
mmi-studio-cli ai-generate -p <PROMPT> -o <OUTPUT_PNG> [--width <PX>] [--height <PX>] [--json]
```

---

## 10. `recipe`
Subcommands for declarative theming and cross-train rebasing.

### `recipe apply`
```bash
mmi-studio-cli recipe apply -r <RECIPE_JSON> [-s <STAGE>] [--json]
```

### `recipe rebase`
```bash
mmi-studio-cli recipe rebase -r <RECIPE_JSON> -t <TARGET_TRAIN_DIR> [--json]
```

---

## 11. `rebuild`
Deterministically repackages a staged tree into candidate update files.

### Syntax
```bash
mmi-studio-cli rebuild [-s <STAGE>] -o <OUTPUT_DIR> [-v <VERIFY_AGAINST>] [--json]
```

---

## 12. `verify-rebuild`
Evaluates a candidate file against the Identity-Rebuild Gate and checks for signature locks.

### Syntax
```bash
mmi-studio-cli verify-rebuild <FILE> [--json]
```

---

## 13. `validate`
Executes the 6-tier (`L0`–`L5`) automotive validation hierarchy against a candidate directory.

### Syntax
```bash
mmi-studio-cli validate <TARGET_DIR> [-p <PROFILE_JSON>] [-l <LEVEL>] [--json]
```

---

## 14. `build-media`
Constructs a deployable FAT32 SD media directory structure aligned to 32 KiB cluster geometry.

### Syntax
```bash
mmi-studio-cli build-media [-s <STAGE>] -o <OUTPUT_DIR> [--volume-label <LABEL>] [--volume-size-gb <GB>] [--json]
```

---

## 15. `simulate-update`
Simulates the QNX head-unit update installer process in software.

### Syntax
```bash
mmi-studio-cli simulate-update <MEDIA_DIR> [--json]
```

---

## 16. `attest`
Emits an immutable cryptographic build attestation manifest for a rebuilt update.

### Syntax
```bash
mmi-studio-cli attest -s <SOURCE_DIR> -b <BUILD_DIR> [--source-train <ID>] [--recipe-id <ID>] [--json]
```

---

## 17. `stock-recovery`
Packages genuine baseline files into an emergency recovery bundle with UART recovery scripts.

### Syntax
```bash
mmi-studio-cli stock-recovery [--baseline-train <ID>] -o <OUTPUT_DIR> [--json]
```

---

## 18. `plugins`
Subcommands for inspecting, listing, and verifying third-party format adapter plugins.

### `plugins list`
```bash
mmi-studio-cli plugins list [-d <DIR>] [--json]
```

### `plugins inspect`
```bash
mmi-studio-cli plugins inspect <PLUGIN_PATH> [--json]
```

### `plugins verify`
```bash
mmi-studio-cli plugins verify <PLUGIN_PATH> [-t <TEST_FILE>] [--json]
```

---

## 19. `maps compile`
Compiles OpenStreetMap vector networks, POIs, and Google Maps Platform enrichment data into native Audi MMI 3G+ Harman/Becker FLDB 544-byte physical pages with automatic 2 GiB volume partitioning.

### Syntax
```bash
mmi-studio-cli maps compile -o <OUTPUT_DIR> [-r <REGION>] [-i <OSM_INPUT>] [--release <TAG>] [--enable-gmp] [--gmp-api-key <KEY>] [--json]
```

### Arguments & Options
| Parameter | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `-o`, `--output` | Path | Required | Target output directory for SD deployment media. |
| `-r`, `--region` | String | `"AL"` | Regional profile: `AL` (Albania & Western Balkans Micro), `DACH`, or `ECE`. |
| `-i`, `--osm-input` | Path | None | Optional path to custom OSM vector file (`.pbf`, `.xml`, `.geojson`). |
| `--release` | String | `"2026_ECE"` | Release version string for manifest generation. |
| `--enable-gmp` | Flag | `false` | Enable Google Maps Platform POI enrichment under strict ToS compliance. |
| `--gmp-api-key` | String | None | Optional Google Maps Platform API key (or demo key). |
| `--json` | Flag | `false` | Emit compilation report as JSON. |

---

## 20. `firmware package`
Packages full system firmware partitions (`ifs-root.ifs` and `efs-system.efs`), Albanian localization, navigation databases, and automation scripts into a ready-to-flash SD card bundle.

### Syntax
```bash
mmi-studio-cli firmware package -o <OUTPUT_DIR> [-t <TRAIN>] [-r <RELEASE>] [-v <VARIANT>] [--splash-png <FILE>] [--strings-ans <FILE>] [--gem-esd <FILE>] [--nav-db <FILE>] [--json]
```

### Arguments & Options
| Parameter | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `-o`, `--output` | Path | Required | Target directory for SD deployment bundle. |
| `-t`, `--train` | String | `"HN+R_EU_AU_K0942_4"` | Software train identifier. |
| `-r`, `--release` | String | `"2026_ECE"` | Release version string. |
| `-v`, `--variant` | String | `"MU9411"` | Hardware variant code. |
| `--splash-png` | Path | None | Optional custom splash screen PNG for `/usr/config/ci/splash.png`. |
| `--strings-ans` | Path | None | Optional Albanian or custom strings `.ans` catalog for `strings/sq_AL.ans`. |
| `--gem-esd` | Path | None | Optional Green Engineering Menu `.esd` definition for `engdefs/menu_2026.esd`. |
| `--nav-db` | Path | None | Optional pre-compiled navigation database `.db`. |
| `--json` | Flag | `false` | Output bundle summary and partition statistics as JSON. |

