# Map Coverage Specification: OEM Reference vs. Open Geodata Target
**Reference Baseline:** Audi ECE 2023 (`8R0051884KL_6.36.0_2023`)  
**Target Coverage Specification:** Full European Territory (45 Countries)  
**Open Geodata Provider:** OpenStreetMap (OSM) via Geofabrik / Overpass APIs  
**Date:** 2026-09-21  

---

## 1. Territory Overview & Country Inventory

The OEM 2023 package (`8R0051884KL_6.36.0_2023`) provides full pan-European road network and navigation coverage across 45 countries:

| Country Name | ISO Code | Regional Tier | Reference Package Coverage | OpenStreetMap Source URL / Mirror | Ingestion Status |
| :--- | :---: | :---: | :---: | :--- | :---: |
| **Albania** | `AL` | Micro / Balkans | 100% Detailed Network | `https://download.geofabrik.de/europe/albania-latest.osm.pbf` | **FULL** |
| **Germany** | `DE` | DACH / Central | 100% Detailed Network | `https://download.geofabrik.de/europe/germany-latest.osm.pbf` | **FULL** |
| **Austria** | `AT` | DACH / Central | 100% Detailed Network | `https://download.geofabrik.de/europe/austria-latest.osm.pbf` | **FULL** |
| **Switzerland** | `CH` | DACH / Central | 100% Detailed Network | `https://download.geofabrik.de/europe/switzerland-latest.osm.pbf`| **FULL** |
| **France** | `FR` | Western Europe | 100% Detailed Network | `https://download.geofabrik.de/europe/france-latest.osm.pbf` | **FULL** |
| **Italy** | `IT` | Southern Europe | 100% Detailed Network | `https://download.geofabrik.de/europe/italy-latest.osm.pbf` | **FULL** |
| **United Kingdom** | `GB` | Western Europe | 100% Detailed Network | `https://download.geofabrik.de/europe/great-britain-latest.osm.pbf`| **FULL** |
| **Spain** | `ES` | Southern Europe | 100% Detailed Network | `https://download.geofabrik.de/europe/spain-latest.osm.pbf` | **FULL** |
| **Netherlands** | `NL` | Western Europe | 100% Detailed Network | `https://download.geofabrik.de/europe/netherlands-latest.osm.pbf`| **FULL** |
| **Belgium** | `BE` | Western Europe | 100% Detailed Network | `https://download.geofabrik.de/europe/belgium-latest.osm.pbf` | **FULL** |
| **Poland** | `PL` | Eastern Europe | 100% Detailed Network | `https://download.geofabrik.de/europe/poland-latest.osm.pbf` | **FULL** |
| **Czech Republic** | `CZ` | Central Europe | 100% Detailed Network | `https://download.geofabrik.de/europe/czech-republic-latest.osm.pbf`| **FULL** |
| **Greece** | `GR` | Balkans / South | 100% Detailed Network | `https://download.geofabrik.de/europe/greece-latest.osm.pbf` | **FULL** |
| **North Macedonia**| `MK` | Balkans | 100% Detailed Network | `https://download.geofabrik.de/europe/macedonia-latest.osm.pbf` | **FULL** |
| **Montenegro** | `ME` | Balkans | 100% Detailed Network | `https://download.geofabrik.de/europe/montenegro-latest.osm.pbf` | **FULL** |
| **Kosovo** | `XK` | Balkans | 100% Detailed Network | `https://download.geofabrik.de/europe/kosovo-latest.osm.pbf` | **FULL** |
| **Croatia** | `HR` | Balkans / Central | 100% Detailed Network | `https://download.geofabrik.de/europe/croatia-latest.osm.pbf` | **FULL** |
| **Full Europe Ext.**| `EU` | Pan-European | Complete 45 Countries | `https://download.geofabrik.de/europe-latest.osm.pbf` (~28 GB) | **SUPPORTED** |

---

## 2. Geographic Bounding Boxes & Regional Profiles

The compiler defines three primary geographic profiles matching the physical volume allocation limits:

### Profile 1: Micro Albania & Western Balkans (`AL`)
* **Scope**: Albania and adjacent cross-border corridors (Hani i Hotit, Morina, Qafë Thanë, Kakavia).
* **Bounding Box**:
  - `min_lat`: 39.5° N
  - `min_lon`: 19.0° E
  - `max_lat`: 42.8° N
  - `max_lon`: 21.2° E
* **Volume Allocation**: Single FAT32 volume (`nav_data.db` $\le 52\text{ MB}$, compile time $<10\text{s}$).

### Profile 2: Regional DACH (`DACH`)
* **Scope**: Germany, Austria, Switzerland, Liechtenstein, and Northern Italian transit corridors.
* **Bounding Box**:
  - `min_lat`: 45.8° N
  - `min_lon`: 5.8° E
  - `max_lat`: 55.1° N
  - `max_lon`: 17.2° E
* **Volume Allocation**: 3 FAT32 volumes ($\le 6.2\text{ GB}$).

### Profile 3: Full Pan-European Territory (`ECE`)
* **Scope**: Complete OEM 8R0051884KL replacement coverage across 45 countries from Portugal to the Urals.
* **Bounding Box**:
  - `min_lat`: 34.5° N
  - `min_lon`: -25.0° W
  - `max_lat`: 71.5° N
  - `max_lon`: 45.0° E
* **Volume Allocation**: 23 FAT32 sequential volumes ($\le 28.19\text{ GB}$).

---

## 3. Data Acquisition & Download Instructions

To compile real open geodata for any region:
1. **Micro Development & Testing (Recommended)**:
   ```bash
   curl -L -o data/sources/albania-latest.osm.pbf https://download.geofabrik.de/europe/albania-latest.osm.pbf
   ```
2. **Production Pan-European Compilation**:
   ```bash
   curl -L -o data/sources/europe-latest.osm.pbf https://download.geofabrik.de/europe-latest.osm.pbf
   ```
3. Record the downloaded dataset in `data/manifests/` with exact SHA-256 and fetch timestamp.
