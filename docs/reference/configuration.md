# Reference — Configuration Schemas

This document provides formal JSON schemas and field definitions for declarative theme recipes, hardware target profiles, and third-party format plugins.

---

## 1. Theme Recipe Schema (`RecipeModel`)

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "RecipeModel",
  "type": "object",
  "required": ["recipe_id", "name", "version", "target_train", "risk_level"],
  "properties": {
    "recipe_id": {
      "type": "string",
      "pattern": "^[a-z0-9_]+$"
    },
    "name": {
      "type": "string"
    },
    "version": {
      "type": "string",
      "pattern": "^\\d+\\.\\d+\\.\\d+$"
    },
    "author": {
      "type": "string"
    },
    "target_train": {
      "type": "string"
    },
    "risk_level": {
      "type": "string",
      "enum": ["COSMETIC", "CONTENT", "STRUCTURAL"]
    },
    "color_mappings": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["source_hex", "target_hex"],
        "properties": {
          "source_hex": { "type": "string", "pattern": "^#[0-9A-Fa-f]{6}$" },
          "target_hex": { "type": "string", "pattern": "^#[0-9A-Fa-f]{6}$" },
          "description": { "type": "string" }
        }
      }
    },
    "asset_replacements": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["target_path", "replacement_source"],
        "properties": {
          "target_path": { "type": "string" },
          "replacement_source": { "type": "string" }
        }
      }
    }
  }
}
```

---

## 2. Target Profile Schema (`TargetProfile`)

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "TargetProfile",
  "type": "object",
  "required": ["profile_id", "name", "hardware_variant", "display_resolution", "max_ifs_size_bytes", "max_efs_size_bytes"],
  "properties": {
    "profile_id": { "type": "string" },
    "name": { "type": "string" },
    "hardware_variant": { "type": "string" },
    "display_resolution": {
      "type": "object",
      "required": ["width", "height"],
      "properties": {
        "width": { "type": "integer" },
        "height": { "type": "integer" }
      }
    },
    "max_ifs_size_bytes": { "type": "integer" },
    "max_efs_size_bytes": { "type": "integer" },
    "supported_trains": {
      "type": "array",
      "items": { "type": "string" }
    }
  }
}
```

---

## 3. Plugin Manifest Schema (`PluginManifest`)

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "PluginManifest",
  "type": "object",
  "required": ["name", "version", "abi_version", "supported_extensions", "memory_limit_mb"],
  "properties": {
    "name": { "type": "string" },
    "version": { "type": "string" },
    "abi_version": { "type": "integer", "enum": [1] },
    "author": { "type": "string" },
    "description": { "type": "string" },
    "supported_extensions": {
      "type": "array",
      "items": { "type": "string" }
    },
    "memory_limit_mb": { "type": "integer", "minimum": 16, "maximum": 512 }
  }
}
```
