# Map Package Technical Analysis & Reverse-Engineering Report
**Target Platform:** Audi MMI 3G (High) & Audi MMI 3G+ (HN+ / HN+R)  
**Primary Reference:** `originals/8R0051884KL_6.36.0_2023`  
**Standard Specification:** OEM Harman/Becker SWDL Automotive Navigation Architecture  
**Generated Build Directory:** `build/analysis/`  

---

## 1. Executive Summary

This report delivers an exhaustive reverse-engineering breakdown of the official Audi 2023 European Navigation Update Package (`8R0051884KL_6.36.0_2023`), establishing the formal technical baseline for all downstream package assembly, validation, and SD-card preparation.

In strict conformance with project safety rules (§1.2, §1.3, §1.4):
- **Zero DRM/Licensing Circumvention**: No crack scripts, private key generation, or memory patch exploits (`slay vdev-logvolmgr`, `copie_scr.sh` injection) are executed or incorporated.
- **Signed Artefact Immutability**: All four cryptographically signed payloads (`MMI3GP_ECE_Hi_R_6_36_0.pkg`, `MMI3G_ECE_Hi_R_6_36_0.pkg`, `TMCConfig.dat`, and their adjacent `.sig` RSA-1024 signatures) are treated as strictly immutable (`canEdit = NO`, `canRebuild = NO`).
- **Official Licensing Artifact Identified**: The required OEM feature enablement certificate is formally identified as FSC `00040025` (`fsc@40025;region@1;model@1`).

---

## 2. Platform & Generation Identification

| Parameter | Value | Evidence Location |
| :--- | :--- | :--- |
| **MMI Generation** | Dual Target: MMI 3G High (HNav) & MMI 3G Plus (HN+ / HN+R) | `metainfo2.txt:31-50` |
| **Vendor** | Becker Automotive Systems (HBAS) | `metainfo2.txt:16` |
| **Assembly ID** | `29ce7bd1` | `metainfo2.txt:6`, `MMI3GP/metainfo2.txt:6` |
| **Part Number** | `8R0060884KL` (Media part: `8R0051884KL`) | `DBInfo.txt:2` |
| **Application Version**| `3600` (Database Version `6.36.0`) | `DBInfo.txt:3`, `metainfo2.txt:5` |
| **System Name** | `EUR 2023     ` (13 characters fixed-width) | `DBInfo.txt:4` |
| **Region / Market** | Europe (`region = "Europe"`, `market = ECE`) | `metainfo2.txt:11`, `pkgdb/*.pkg:27` |
| **Target Hardware Variants** | `9307`, `9308` (MMI3G); `9411`, `9408`, `9409`, `9410`, `9498`, `9499`, `9425`, `9436` (MMI3GP) | `metainfo2.txt:18-27` |

---

## 3. Storage Hierarchy & Packaging Layout

The root SD card structure of the reference package comprises 77 files spanning 37 directories, aggregating **40,693,125,670 bytes (37.90 GiB)**:

```text
SD_ROOT/
├── DBInfo.txt               # Master database identification (PartNumber, Version, SystemName)
├── build1                   # Release build flag (29 bytes)
├── config.nfm               # Navigation File Manager descriptor (configfmtversion=1.2.3, version="E5.2")
├── metainfo2.txt            # Root SWDL metafile directing unit to sub-trains (Release_1 vs Release_2)
├── MMI3G/
│   └── metainfo2.txt        # Sub-manifest for MMI 3G High (variants 9307, 9308)
├── MMI3GP/
│   └── metainfo2.txt        # Sub-manifest for MMI 3G Plus (variants 9411, 9408, 9409, 9410, 9425, etc.)
└── pkgdb/                   # Unified package database containing 34 module directories
    ├── MMI3G_ECE_Hi_R_6_36_0.pkg       # MMI 3G package manifest (Signed)
    ├── MMI3G_ECE_Hi_R_6_36_0.pkg.sig   # RSA-1024 detached signature
    ├── MMI3GP_ECE_Hi_R_6_36_0.pkg      # MMI 3G+ package manifest (Signed)
    ├── MMI3GP_ECE_Hi_R_6_36_0.pkg.sig  # RSA-1024 detached signature
    ├── CTY/, CTY2/, CTY3/              # City index and 3D city models (ATLAS containers)
    ├── CTYS3TC/, CTYS3TC2/             # S3TC compressed 3D texture atlas tiles
    ├── GDB/, GDB2/                     # Geographic routing network & road geometry
    ├── LABEL/                          # Geographic labels database (Label.DB)
    ├── LIT/, LIT2/, LIT3/, LIT4/       # MMI 3G Street address and point search indexes
    ├── LIT3GP/, LIT3GP2/..LIT3GP5/     # MMI 3G+ High-density search & phonetic indexes
    ├── NaviPersistence_ALL_3/          # Post-update cleanup scripts (clearLastCityHistory.sh)
    ├── PIT/                            # Point of Interest classification tables (EJ211a.PIT)
    ├── PSD/, PSD2/, PSD3/              # Predictive Safety Data / ADAS curve profiles
    ├── SDS/, SDS3GP/                   # Speech Dialogue Systems audio acoustic models (.iso)
    ├── StyleDBMMI3G_ECE_94/            # Cartography style shaders for MMI 3G (.xar)
    ├── StyleDBMMI3GP_ECE_9129/         # Cartography style shaders for MMI 3G+ (.xar)
    ├── TER/, TER2/                     # Digital Elevation Model (DEM) terrain atlas
    ├── TMC/, TMC3GP/                   # Traffic Message Channel location tables
    ├── TMCConfig_16/                   # Encrypted TMC station service tables (Signed)
    └── XAC/, XAC2/, XAC3/              # Cross-network intersection and lane guidance tables
```

---

## 4. Module & Container Deep-Dive

### 4.1 544-Byte Physical Page Containers (`*.db`)
- **Structure**: All major search and attribution databases (`EJ211Pa_L1.db` through `L5.db`, `kN221EUx01_0.db` through `_2.db`) are formatted into discrete **544-byte physical disk blocks**:
  - `0..4`: Container magic (`FLDB` or vendor equivalent).
  - `4..8`: 32-bit Little-Endian Page Index.
  - `8..10`: CRC-16/CCITT checksum over bytes `16..528`.
  - `16..528`: 512-byte payload data.
  - `528..532`: Trailer sync guard word (`0x55AA55AA`).
- **Volume Boundaries**: Maximum single file size is strictly capped below $2\text{ GiB} - 1\text{ byte}$ ($2,147,483,647\text{ B}$) to maintain compatibility with QNX FAT32 file allocation limits.

### 4.2 Orion ATLAS Geographic Containers (`*.ATLAS`)
- **Structure**: Used for 3D city models, texture bitmaps, ADAS road horizons, and 3D terrain elevation meshes (`3PN221EU22083H1665a.4_2.0.ATLAS`, `APN221EU22093P1664a.5_1.0.ATLAS`, `72_Europe.4_2.0.ATLAS`).
- **Checksumming**: Validated via per-module `.conf` definitions asserting ISO image geometry, MD5 digests, and Quick-Check (`qa,100`) sampling blocks.

### 4.3 Routing & Spatial Graphs (`*.gdb` / `*.gd2`)
- **Structure**: `EJ211_v37a.gdb` (2.00 GiB) and `EJ211_v37a.gd2` (1.38 GiB) store the directional routing topology, link connectivity, turning restriction penalties, and spatial R-tree hierarchy.

### 4.4 Style Archives (`*.xar`)
- **Structure**: `MMI3G_MapArchive_H_06_01.xar` stores regional day, night, motorway, water, and POI vector shaders, unpacked dynamically into QNX graphics memory during navigation boot.

---

## 5. Security, Checksum & Verification Topology

1. **Root SWDL Metafile (`metainfo2.txt`)**:
   - Contains 8 cryptographic manufacturer signatures (`[AudiSignature]` `signature1`..`signature8`).
   - Asserts `MetafileChecksum = "8714b251"` calculated over normalized INI entries.
2. **Sub-Train Metafiles (`MMI3GP/metainfo2.txt`)**:
   - Computes multi-stage CRC32 chunk hashes (`CheckSum`, `CheckSum1`..`CheckSum10`) using a chunk boundary of **209,715,200 bytes (200 MiB)**.
   - Enforces pre-copy directory sanitization (`DeleteFilesBeforeCopy = "/mnt/nav/db/pkgdb/*_ECE_*.pkg..."`).
3. **Module Definitions (`pkgdb/*.conf`)**:
   - Each module directory contains an authoritative `.conf` file specifying exact byte sizes, full MD5 digests, and `checkcrc` hex values.
4. **Official Feature Enablement Code (FSC)**:
   - Line 33 of `MMI3GP_ECE_Hi_R_6_36_0.pkg` dictates: `userflags=fsc@40025;region@1;model@1`.
   - **Requirement**: The vehicle MMI head unit must possess a valid, digitally signed FSC certificate matching Application `00040025` for European map release 2023.
