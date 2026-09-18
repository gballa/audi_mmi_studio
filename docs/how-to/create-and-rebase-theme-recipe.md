# How-To: Create and Rebase a Theme Recipe

This guide demonstrates how to author a declarative JSON theme recipe, apply it to a staging workspace, and evaluate its portability across different firmware versions.

---

## Goal
Safely automate theme modifications across multiple Audi MMI software trains using declarative rules.

## Prerequisites
- Compiled `mmi-studio-cli`.
- Target software trains in `originals/` (e.g. `originals/MU9411`).

---

## Procedure

### Step 1: Create a Recipe JSON File
Create `recipes/custom_night_amber.json`:
```json
{
  "recipe_id": "custom_night_amber",
  "name": "Custom Night Amber Theme",
  "version": "1.0.0",
  "author": "Engineering Team",
  "target_train": "HN+R_EU_AU_K0942_4",
  "risk_level": "COSMETIC",
  "color_mappings": [
    {
      "source_hex": "#E0001B",
      "target_hex": "#FFB300",
      "description": "Red alerts to Amber"
    }
  ],
  "asset_replacements": [
    {
      "target_path": "ScreenLayouts/CombiStyles.precomp",
      "replacement_source": "/tmp/CombiStyles_conformed.precomp"
    }
  ]
}
```

### Step 2: Apply Recipe to Staging
Execute the recipe against the isolated `default` stage:
```bash
./target/release/mmi-studio-cli recipe apply \
  --recipe recipes/custom_night_amber.json \
  --stage default
```
*Expected Output*:
```text
Recipe applied successfully to stage 'default': 1 color mappings, 1 asset replacements.
```

### Step 3: Evaluate Cross-Train Drift (Rebase)
Evaluate whether this recipe can be applied to an alternate firmware release:
```bash
./target/release/mmi-studio-cli recipe rebase \
  --recipe recipes/custom_night_amber.json \
  --target-train originals/MU9411
```
*Expected Output*:
```text
Rebase Analysis:
  Target Train: MU9411
  Matched Assets: 1/1 (100%)
  Drift Score: 0.0 (Clean match)
  Portability Verdict: REBASE_CLEAN
```
If assets have moved or changed formats between trains, the output lists the exact offset drift and warns if an asset is missing.
