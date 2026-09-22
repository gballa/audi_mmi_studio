# Map File Classification Ledger: OEM Reference Package
**Reference Baseline:** `originals/8R0051884KL_6.36.0_2023` (77 files, 37 directories, 40,693,125,670 bytes)  
**Classification Categories:**
- `MAP_DATA`: Contains road vectors, attributes, or geographic points; regenerated from new open geodata.
- `DERIVED_INDEX`: Spatial, phonetic, or search index; regenerated from generated `MAP_DATA`.
- `DERIVED_METADATA`: Manifest or descriptor; regenerated from compiled outputs.
- `PACKAGE_METADATA`: SWDL release descriptors and checksum files.
- `OEM_STATIC`: Unchanged firmware resources (scripts, fonts, icons).
- `OEM_RESOURCE`: Language models, audio speech catalogs, or vehicle shaders.
- `SECURITY/SIGNATURE`: Detached cryptographic signatures or signed binary payloads.
- `UNKNOWN`: Unclassified binary structures.

---

## 1. Complete Classification Matrix

| File Path in Reference Package | Size (Bytes) | Category | Regenerable from OSM? | Action in 2026 Open Data Build |
| :--- | :--- | :---: | :---: | :--- |
| `DBInfo.txt` | 206 | `PACKAGE_METADATA` | YES | Regenerated with `2026_ECE` and updated version tags |
| `build1` | 29 | `OEM_STATIC` | YES | Preserved / regenerated build stamp |
| `config.nfm` | 233 | `PACKAGE_METADATA` | YES | Preserved / updated config format descriptor |
| `metainfo2.txt` | 1,444 | `PACKAGE_METADATA` | YES | Regenerated with new file paths and checksums |
| `MMI3G/metainfo2.txt` | 17,998 | `PACKAGE_METADATA` | YES | Regenerated for MMI3G sub-train |
| `MMI3GP/metainfo2.txt` | 19,383 | `PACKAGE_METADATA` | YES | Regenerated for MMI3GP sub-train |
| `pkgdb/MMI3GP_ECE_Hi_R_6_36_0.pkg` | 2,212 | `PACKAGE_METADATA` | PARTIAL | Regenerated unsigned; original is signed |
| `pkgdb/MMI3GP_ECE_Hi_R_6_36_0.pkg.sig` | 128 | `SECURITY/SIGNATURE` | **NO** | `BLOCKED — REQUIRED OEM RSA-1024 PRIVATE SIGNING KEY` |
| `pkgdb/MMI3G_ECE_Hi_R_6_36_0.pkg` | 2,019 | `PACKAGE_METADATA` | PARTIAL | Regenerated unsigned; original is signed |
| `pkgdb/MMI3G_ECE_Hi_R_6_36_0.pkg.sig` | 128 | `SECURITY/SIGNATURE` | **NO** | `BLOCKED — REQUIRED OEM RSA-1024 PRIVATE SIGNING KEY` |
| `pkgdb/TMCConfig_16/TMCConfig.dat` | 158,720 | `SECURITY/SIGNATURE` | **NO** | `BLOCKED — REQUIRED OEM/PROPRIETARY FORMAT INFORMATION` |
| `pkgdb/TMCConfig_16/TMCConfig.dat.sig` | 128 | `SECURITY/SIGNATURE` | **NO** | `BLOCKED — REQUIRED OEM RSA-1024 PRIVATE SIGNING KEY` |
| `pkgdb/GDB/EJ211_v37a.gdb` | 2,147,371,105 | `MAP_DATA` | **YES** | Regenerated as `nav_data.db` (544B physical pages) |
| `pkgdb/GDB2/EJ211_v37a.gd2` | 1,483,073,869 | `MAP_DATA` | **YES** | Regenerated as sequential FAT32 volumes |
| `pkgdb/XAC/kN221EUx01_0.db` | 2,143,248,384 | `MAP_DATA` | **YES** | Regenerated from intersection & lane topology |
| `pkgdb/XAC2/kN221EUx01_1.db` | 2,142,963,712 | `MAP_DATA` | **YES** | Regenerated from intersection & lane topology |
| `pkgdb/XAC3/kN221EUx01_2.db` | 254,181,376 | `MAP_DATA` | **YES** | Regenerated from intersection & lane topology |
| `pkgdb/LIT3GP/EJ211Pa_L1.db` | 2,147,364,864 | `DERIVED_INDEX` | **YES** | Regenerated from OSM address points (`addr:*`) |
| `pkgdb/LIT3GP2/EJ211Pa_L2.db` | 2,147,393,536 | `DERIVED_INDEX` | **YES** | Regenerated from OSM address points (`addr:*`) |
| `pkgdb/LIT3GP3/EJ211Pa_L3.db` | 2,147,395,584 | `DERIVED_INDEX` | **YES** | Regenerated from OSM address points (`addr:*`) |
| `pkgdb/LIT3GP4/EJ211Pa_L4.db` | 2,147,395,584 | `DERIVED_INDEX` | **YES** | Regenerated from OSM address points (`addr:*`) |
| `pkgdb/LIT3GP5/EJ211Pa_L5.db` | 1,218,211,840 | `DERIVED_INDEX` | **YES** | Regenerated from OSM address points (`addr:*`) |
| `pkgdb/LIT/EJ211Ga_L1.db` | 2,147,401,728 | `DERIVED_INDEX` | **YES** | Regenerated from OSM street indices |
| `pkgdb/LIT2/EJ211Ga_L2.db` | 2,147,383,296 | `DERIVED_INDEX` | **YES** | Regenerated from OSM street indices |
| `pkgdb/LIT3/EJ211Ga_L3.db` | 2,147,401,728 | `DERIVED_INDEX` | **YES** | Regenerated from OSM street indices |
| `pkgdb/LIT4/EJ211Ga_L4.db` | 631,121,920 | `DERIVED_INDEX` | **YES** | Regenerated from OSM street indices |
| `pkgdb/LABEL/Label.DB` | 2,048 | `DERIVED_INDEX` | **YES** | Regenerated from place names and administrative tags |
| `pkgdb/PIT/EJ211a.PIT` | 8,869,262 | `MAP_DATA` | **YES** | Regenerated from POI classification hierarchy |
| `pkgdb/CTY/*.ATLAS` (3 files) | 5,194,601,248 | `MAP_DATA` | PARTIAL | 3D city models. Partially regenerable via `HbAtlas` |
| `pkgdb/CTYS3TC/*.ATLAS` (2 files) | 2,658,930,736 | `MAP_DATA` | PARTIAL | S3TC compressed building textures |
| `pkgdb/TER/*.ATLAS` (2 files) | 2,214,189,520 | `MAP_DATA` | PARTIAL | Digital elevation models (DEM) |
| `pkgdb/PSD/*.ATLAS` (3 files) | 5,037,063,424 | `MAP_DATA` | **YES** | Regenerated from road slope and curvature metrics |
| `pkgdb/TMC/kN221EUx01t01.db` | 7,022,592 | `MAP_DATA` | **YES** | TMC location point tables |
| `pkgdb/TMC3GP/kN221EUx01t01.db`| 18,790,400 | `MAP_DATA` | **YES** | TMC location point tables |
| `pkgdb/TMC3GP/info.xml` | 1,440 | `DERIVED_METADATA` | **YES** | Regenerated TMC service manifest |
| `pkgdb/SDS/SDS_Data.iso` | 227,960,832 | `OEM_RESOURCE` | **NO** | `BLOCKED — REQUIRED OEM/PROPRIETARY FORMAT INFORMATION` |
| `pkgdb/SDS3GP/SDS_Data.iso` | 263,061,504 | `OEM_RESOURCE` | **NO** | `BLOCKED — REQUIRED OEM/PROPRIETARY FORMAT INFORMATION` |
| `pkgdb/StyleDBMMI3G_ECE_94/*.xar`| 4,932,124 | `OEM_RESOURCE` | **YES** | Regenerated via `MapStyleXar` (Day/Night shaders) |
| `pkgdb/StyleDBMMI3GP_ECE_9129/*.xar`| 5,727,060 | `OEM_RESOURCE` | **YES** | Regenerated via `MapStyleXar` (Day/Night shaders) |
| `pkgdb/NaviPersistence_ALL_3/*` | 1,215 | `OEM_STATIC` | **YES** | Shell and XML history clear scripts |
| `pkgdb/*/*.conf` (30 files) | ~15,000 | `DERIVED_METADATA` | **YES** | Regenerated with matching MD5 and byte sizes |
