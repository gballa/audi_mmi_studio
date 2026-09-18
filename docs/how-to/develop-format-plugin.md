# How-To: Develop a Third-Party Format Plugin

This guide explains how to build and register custom binary format decoders using the **Audi MMI Studio Plugin SDK** (`mmi_plugin`).

---

## Goal
Extend the workstation to parse proprietary or third-party vehicle binary containers without modifying core workspace crates.

## Prerequisites
- Rust 1.80+ or C-compatible ABI toolchain.
- `mmi_plugin` crate.

---

## Procedure

### Step 1: Create the Plugin Directory
Plugins reside under `.mmistudio/plugins/<plugin-id>/`:
```bash
mkdir -p .mmistudio/plugins/audio-codec-plugin
```

### Step 2: Author `plugin.json` Manifest
Create `.mmistudio/plugins/audio-codec-plugin/plugin.json`:
```json
{
  "name": "audio-codec-plugin",
  "version": "0.1.0",
  "abi_version": 1,
  "author": "Community Contributor",
  "description": "Parser for proprietary acoustic speech model archives (.ans)",
  "supported_extensions": ["ans"],
  "memory_limit_mb": 64
}
```

### Step 3: Implement the Adapter Interface
Plugins conform to the `FormatAdapter` trait:
```rust
use mmi_formats::FormatAdapter;

pub struct AudioCodecPlugin;

impl FormatAdapter for AudioCodecPlugin {
    fn name(&self) -> &'static str {
        "audio-codec-plugin"
    }

    fn detect(&self, data: &[u8]) -> bool {
        data.len() >= 4 && &data[0..4] == b"ANS\0"
    }

    fn extract_strings(&self, data: &[u8]) -> Vec<String> {
        // Return extracted prompt labels
        vec![]
    }

    fn validate(&self, data: &[u8]) -> bool {
        self.detect(data)
    }
}
```

### Step 4: Verify Plugin in the Workstation
Use `mmi-studio-cli` to inspect and test your plugin:
```bash
# 1. Discover registered plugins
./target/release/mmi-studio-cli plugins list

# 2. Inspect ABI compatibility
./target/release/mmi-studio-cli plugins inspect .mmistudio/plugins/audio-codec-plugin

# 3. Test sandbox isolation against a genuine sample
./target/release/mmi-studio-cli plugins verify \
  .mmistudio/plugins/audio-codec-plugin \
  --test-file originals/MU9411/Speech/prompts.ans
```
*Expected Output*: Exit code `0` (`Plugin verified: ABI compatible, sandbox memory limit respected`).
