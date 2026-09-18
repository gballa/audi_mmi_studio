# User Guide — Configuration & Presets

This document details user-facing configuration options, theme recipe schemas, and target profile customizations.

---

## 1. Theme Recipe Structure (`mmi_recipe`)

Theme recipes in **Audi MMI Studio** are declarative JSON documents specifying color transformations, bitmap replacements, and font substitutions without manual byte-patching.

### Schema Fields
- `recipe_id`: Unique identifier string (lowercase alphanumeric with underscores).
- `name`: Human-readable theme name.
- `version`: Semantic version string.
- `author`: Author or engineering team attribution.
- `target_train`: Baseline software train ID (e.g. `HN+R_EU_AU_K0942_4`).
- `risk_level`: Safety classification:
  - `COSMETIC`: Color palette shifts and icon replacements.
  - `CONTENT`: Translation string changes and acoustic prompt tweaks.
  - `STRUCTURAL`: Partition adjustments or layout geometry alterations.
- `color_mappings`: Array of source-to-destination hex color replacements.
- `asset_replacements`: Array of targeted asset paths and replacement image sources.
- `font_substitutions`: Optional font mappings.

### Example Recipe
```json
{
  "recipe_id": "audi_sport_amber",
  "name": "Audi Sport Amber Edition",
  "version": "1.0.0",
  "author": "Audi MMI Studio Engineers",
  "target_train": "HN+R_EU_AU_K0942_4",
  "risk_level": "COSMETIC",
  "color_mappings": [
    {
      "source_hex": "#FF0000",
      "target_hex": "#FFB300",
      "description": "Sport Red to Performance Amber"
    }
  ],
  "asset_replacements": [
    {
      "target_path": "ScreenLayouts/CombiStyles.precomp",
      "replacement_source": "assets/amber_combi.png"
    }
  ]
}
```

---

## 2. Target Profile Customization (`mmi_validation`)

Target profiles ensure that staged updates conform to the physical constraints of specific head unit models.

```json
{
  "profile_id": "mmi3g_high_eu",
  "name": "Audi MMI 3G High (EU)",
  "hardware_variant": "9411",
  "display_resolution": {
    "width": 800,
    "height": 480
  },
  "max_ifs_size_bytes": 16777216,
  "max_efs_size_bytes": 67108864,
  "supported_trains": [
    "HN+R_EU_AU_K0942_4_[8R0906961FB]"
  ]
}
```

---

## 3. Plugin Configuration (`mmi_plugin`)

Custom format decoders can be registered in `.mmistudio/plugins/<plugin-id>/plugin.json`:

```json
{
  "name": "sample-custom-decoder",
  "version": "0.1.0",
  "abi_version": 1,
  "author": "Contributor",
  "description": "Third-party format adapter for proprietary audio containers",
  "supported_extensions": ["ans", "snd"],
  "memory_limit_mb": 64
}
```
All plugins run within strict sandboxes with bounded memory and no network capabilities.
