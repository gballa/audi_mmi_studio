# Audi MMI Studio — Core Architecture & Workspace Crates (`crates/`)

The `crates/` directory contains the modular, offline-first Rust libraries that power **Audi MMI Studio**. Each crate addresses a discrete architectural concern, enforcing strict layer isolation, deterministic execution, and automotive safety guarantees.

---

## Workspace Dependency Graph

```
                            ┌────────────────────────┐
                            │     mmi-studio-cli     │
                            │  mmi-studio-desktop    │
                            └───────────┬────────────┘
                                        │
        ┌───────────────────────────────┴───────────────────────────────┐
        │                                                               │
┌───────▼────────┐  ┌────────────────┐  ┌────────────────┐  ┌───────────▼────┐
│   mmi-recipe   │  │   mmi-canvas   │  │   mmi-media    │  │ mmi-validation │
└───────┬────────┘  └───────┬────────┘  └───────┬────────┘  └───────────┬────┘
        │                   │                   │                       │
┌───────▼────────┐  ┌───────▼────────┐  ┌───────▼────────┐  ┌───────────▼────┐
│   mmi-assets   │  │   mmi-rebuild  │  │mmi-attestation │  │  mmi-imagegen  │
└───────┬────────┘  └───────┬────────┘  └───────┬────────┘  └───────────┬────┘
        │                   │                   │                       │
┌───────▼────────┐  ┌───────▼────────┐  ┌───────▼────────┐  ┌───────────▼────┐
│  mmi-formats   │  │   mmi-re-lab   │  │   mmi-plugin   │  │    mmi-core    │
└───────┬────────┘  └───────┬────────┘  └───────┬────────┘  └───────────┬────┘
        │                   │                   │                       │
        └───────────────────┴───────────────────┴───────────────────────┘
                                        │
                               ┌────────▼───────┐
                               │ originals/ CAS │
                               └────────────────┘
```

---

## Architectural Layers

| Layer | Crates | Focus |
| :--- | :--- | :--- |
| **Layer 1: Foundation & Attestation** | `mmi-core`, `mmi-attestation`, `mmi-validation` | Content-addressed storage, project models, 6-tier validation, cryptographic attestation. |
| **Layer 2: Reverse Engineering & Formats** | `mmi-formats`, `mmi-re-lab` | 12 automotive binary decoders, Shannon entropy, carving, virtual hex inspector. |
| **Layer 3: Theming, Assets & Canvas** | `mmi-assets`, `mmi-recipe`, `mmi-canvas` | Bitmap conversion, font conformance, declarative JSON theming, 800x480 screen composition. |
| **Layer 4: Deterministic Packaging & Media** | `mmi-rebuild`, `mmi-media` | Identity-rebuild gate, signed payload locking, FAT32 SD media packaging, update simulation. |
| **Layer 5: Extensibility & Airlock** | `mmi-imagegen`, `mmi-plugin` | Egress-airlocked asset generation, sandboxed format plugins with ABI validation. |

---

## Crate Directory & How-To Guide

### 1. `mmi-core`
- **Location**: `crates/mmi-core`
- **Role**: Base types, error definitions, Content-Addressed Storage (CAS) with BLAKE3, and read-only immutable access to `originals/`.
- **Key Structs**: `ContentAddressedStore`, `SourceStore`, `StageStore`, `MMIProject`, `CoreError`.
- **How-To Use**:
  ```rust
  use mmi_core::{ContentAddressedStore, SourceStore};
  use std::path::Path;

  // Read safely from the immutable originals/ directory
  let source = SourceStore::new(Path::new("originals")).unwrap();
  let bytes = source.read_bytes("MU9411/HBNavDB/database.db").unwrap();

  // Store payload into Content-Addressed Storage
  let cas = ContentAddressedStore::new(Path::new(".mmistudio/cas")).unwrap();
  let blob_id = cas.put_blob(&bytes).unwrap();
  println!("Stored blob: {}", blob_id);
  ```
- **Test Command**: `cargo test -p mmi-core --offline`

---

### 2. `mmi-formats`
- **Location**: `crates/mmi-formats`
- **Role**: Specialized parsers and decoders for all 12 Audi MMI proprietary binary formats:
  - `HBNavDB` (`.db`)
  - `OrionAtlas` (`.atlas`)
  - `PrecompAdapter` / `PrecompImage` (`.precomp`)
  - `MapStyles` (`.xar`)
  - `QnxIfs` (`.ifs`)
  - `QnxEfs` (`.efs`)
  - `SpeechAns` (`.ans`)
  - `SystemFpga` (`.hbbin`)
  - `SmscInic` (`.ipf`)
  - `GeographicGdb` (`.gdb`)
  - `BinaryGrammar` (`.hbgr`)
  - `AdiBlackfinDsp` (`.ldr`)
- **Key Traits**: `FormatAdapter`, `FormatMetadata`.
- **How-To Use**:
  ```rust
  use mmi_formats::{FormatAdapter, PrecompAdapter, PrecompImage};

  let adapter = PrecompAdapter::default();
  let data = std::fs::read("originals/MU9411/ScreenLayouts/CombiStyles.precomp").unwrap();

  if adapter.detect(&data) {
      let image = PrecompImage::decode(&data).unwrap();
      println!("Decoded .precomp image: {}x{} ({} bytes RGBA)", image.width, image.height, image.pixels.len());
  }
  ```
- **Test Command**: `cargo test -p mmi-formats --offline`

---

### 3. `mmi-assets`
- **Location**: `crates/mmi-assets`
- **Role**: Bitmap decoding, thumbnail generation, color-space conformance, and font metric calculations.
- **Key Structs**: `DecodedBitmap`, `FormatConformer`, `ThumbnailGenerator`, `ConformResult`.
- **How-To Use**:
  ```rust
  use mmi_assets::decoder::DecodedBitmap;
  use mmi_assets::conformer::FormatConformer;

  let target_spec = DecodedBitmap {
      width: 64,
      height: 64,
      rgba_pixels: vec![255; 64 * 64 * 4],
      source_format: "precomp".to_string(),
      has_alpha: true,
  };

  // Check whether a proposed candidate image matches target constraints
  let conform_res = FormatConformer::conform_image(&candidate_bytes, &target_spec).unwrap();
  println!("Conform result: matches={}", conform_res.is_exact_match);
  ```
- **Test Command**: `cargo test -p mmi-assets --offline`

---

### 4. `mmi-canvas`
- **Location**: `crates/mmi-canvas`
- **Role**: Framebuffer synthesis and multi-layer rendering for 800x480 high-resolution MMI screens.
- **Key Structs**: `CanvasRenderer`, `ScreenComposition`, `DisplayMode`, `LayoutProvenance`.
- **How-To Use**:
  ```rust
  use mmi_canvas::{CanvasRenderer, DisplayMode, ScreenComposition};

  let mut comp = ScreenComposition::new(800, 480, DisplayMode::Night);
  comp.add_chrome_layer("GaugeCluster_OuterRing");
  comp.add_widget("Speedometer", 120, 90, 240, 240);

  let renderer = CanvasRenderer::new();
  let frame = renderer.render_frame(&comp).unwrap();
  println!("Rendered frame: {} pixels (RGBA)", frame.rgba_pixels.len() / 4);
  ```
- **Test Command**: `cargo test -p mmi-canvas --offline`

---

### 5. `mmi-re-lab`
- **Location**: `crates/mmi-re-lab`
- **Role**: Reverse-engineering laboratory utilities including Shannon entropy calculation, byte histograms, string extraction, and signature carving.
- **Key Structs**: `EntropyCalculator`, `HexViewer`, `ByteHistogram`, `SignatureCarver`, `StringExtractor`.
- **How-To Use**:
  ```rust
  use mmi_re_lab::{EntropyCalculator, SignatureCarver, HexViewer};

  let data = std::fs::read("originals/MU9411/HBNavDB/database.db").unwrap();

  // 1. Calculate entropy
  let (avg_entropy, classification) = EntropyCalculator::calculate_entropy(&data, 512);
  println!("Avg Entropy: {:.2} ({:?})", avg_entropy, classification);

  // 2. Scan for embedded containers
  let carver = SignatureCarver::default();
  let matches = carver.carve_signatures(&data);
  for m in matches {
      println!("Carved {:?} at offset 0x{:08X}", m.kind, m.offset);
  }
  ```
- **Test Command**: `cargo test -p mmi-re-lab --offline`

---

### 6. `mmi-recipe`
- **Location**: `crates/mmi-recipe`
- **Role**: Declarative JSON theme recipe engine, cryptographic journaling, and cross-train rebasing.
- **Key Structs**: `RecipeModel`, `ReplacementRule`, `ColorMapping`, `FontSubstitution`, `RecipeEngine`, `RebaseEngine`.
- **How-To Use**:
  ```rust
  use mmi_recipe::{RecipeEngine, RecipeModel, RebaseEngine};
  use std::path::Path;

  // Load and validate a theme recipe
  let recipe = RecipeModel::load_from_json(Path::new("recipes/audi_sport_amber.json")).unwrap();
  recipe.validate().unwrap();

  // Check cross-train drift against a target firmware version
  let rebase_report = RebaseEngine::evaluate_drift(&recipe, Path::new("originals/MU9411")).unwrap();
  println!("Rebase portability score: {:.1}%", rebase_report.portability_score * 100.0);
  ```
- **Test Command**: `cargo test -p mmi-recipe --offline`

---

### 7. `mmi-rebuild`
- **Location**: `crates/mmi-rebuild`
- **Role**: The Identity-Rebuild Gate: deterministic stage normalization, binary repackaging, and signed artefact protection.
- **Key Structs**: `StageNormalizer`, `Repackager`, `IdentityRebuildVerifier`.
- **How-To Use**:
  ```rust
  use mmi_rebuild::{IdentityRebuildVerifier, StageNormalizer};
  use std::path::Path;

  // Verify whether a binary is safe to modify or locked by cryptographic signatures
  let candidate = std::fs::read("output/rebuilt_file.precomp").unwrap();
  let verifier = IdentityRebuildVerifier::new();
  let check = verifier.verify_candidate(&candidate, Path::new("ScreenLayouts/CombiStyles.precomp"));

  assert!(check.is_permitted, "Modification must not violate signature locks");
  ```
- **Test Command**: `cargo test -p mmi-rebuild --offline`

---

### 8. `mmi-validation`
- **Location**: `crates/mmi-validation`
- **Role**: 6-tier (L0–L5) validation engine enforcing automotive compliance before deployment.
  - **L0**: Magic bytes and file headers
  - **L1**: Section offsets and structural layouts
  - **L2**: Asset dimensions, bit depth, and color palettes
  - **L3**: Text bounding box overflow and UI margins
  - **L4**: Determinism gate and signed payload integrity
  - **L5**: Target hardware and firmware compatibility profile
- **Key Structs**: `ValidationEngine`, `ValidationReport`, `TargetProfile`, `ValidationLevel`.
- **How-To Use**:
  ```rust
  use mmi_validation::{ValidationEngine, TargetProfile};
  use std::path::Path;

  let engine = ValidationEngine::new();
  let profile = TargetProfile::default_mmi3g_high();

  let report = engine.validate_directory(Path::new("output/candidate_stage"), Some(&profile)).unwrap();
  println!("L0-L5 Validation Status: {:?}", report.status);
  assert!(report.errors.is_empty(), "Validation passed without errors");
  ```
- **Test Command**: `cargo test -p mmi-validation --offline`

---

### 9. `mmi-media`
- **Location**: `crates/mmi-media`
- **Role**: Construction of FAT32 SD media directory trees, volume splitting (for multi-SD updates), volume manifests, and QNX head-unit update simulation.
- **Key Structs**: `MediaBuilder`, `VolumeSplitter`, `VolumeManifest`, `UpdateSimulator`.
- **How-To Use**:
  ```rust
  use mmi_media::{MediaBuilder, UpdateSimulator};
  use std::path::Path;

  let builder = MediaBuilder::new("MMI3G_NAV", 32); // 32GB SD limit
  let manifest = builder.build_package(
      Path::new("output/candidate_stage"),
      Path::new("output/sd_media")
  ).unwrap();

  // Run pre-flight update simulation
  let simulator = UpdateSimulator::new();
  let sim_res = simulator.simulate_media_update(Path::new("output/sd_media")).unwrap();
  println!("Update simulation status: {:?}", sim_res.status);
  ```
- **Test Command**: `cargo test -p mmi-media --offline`

---

### 10. `mmi-attestation`
- **Location**: `crates/mmi-attestation`
- **Role**: Cryptographic build attestation manifests (BLAKE3/SHA-256) and emergency stock recovery bundle packaging.
- **Key Structs**: `AttestationManifest`, `AttestationBuilder`, `EmergencyRecoveryManager`.
- **How-To Use**:
  ```rust
  use mmi_attestation::{AttestationBuilder, EmergencyRecoveryManager};
  use std::path::Path;

  // Generate an immutable build attestation manifest
  let builder = AttestationBuilder::new("HN+R_EU_AU_K0942_4", "git:abc1234");
  let manifest = builder.generate(
      Path::new("originals/MU9411"),
      Path::new("output/candidate_stage"),
      Some("recipes/audi_sport_amber.json")
  ).unwrap();

  // Package stock recovery bundle
  let rec_mgr = EmergencyRecoveryManager::new();
  rec_mgr.package_stock_bundle(
      "HN+R_EU_AU_K0942_4_[8R0906961FB]",
      Path::new("output/stock_recovery")
  ).unwrap();
  ```
- **Test Command**: `cargo test -p mmi-attestation --offline`

---

### 11. `mmi-imagegen`
- **Location**: `crates/mmi-imagegen`
- **Role**: Strict Egress Airlock for AI asset authoring. Enforces token isolation in OS keychain, brand/PII sanitization, egress filters, and provides an offline mock provider for zero-network environments.
- **Key Structs**: `EgressAirlock`, `AirlockConfig`, `OfflineMockProvider`, `GenerationRequest`.
- **How-To Use**:
  ```rust
  use mmi_imagegen::{EgressAirlock, AirlockConfig, GenerationRequest};

  let airlock = EgressAirlock::new(AirlockConfig::offline_default());
  let req = GenerationRequest {
      prompt: "Minimalist amber navigation compass arrow on black background".to_string(),
      width: 64,
      height: 64,
      source_reference: None,
  };

  let asset = airlock.generate_asset(&req).unwrap();
  println!("Airlocked image generated: {}x{} RGBA", asset.width, asset.height);
  ```
- **Test Command**: `cargo test -p mmi-imagegen --offline`

---

### 12. `mmi-plugin`
- **Location**: `crates/mmi-plugin`
- **Role**: Extensible third-party format plugin system. Provides manifest parsing, semantic ABI validation, memory limit enforcement, and sandboxed adapter bridges.
- **Key Structs**: `PluginManifest`, `PluginDiscovery`, `SandboxedPluginAdapter`, `PluginBridge`.
- **How-To Use**:
  ```rust
  use mmi_plugin::{PluginDiscovery, PluginManifest};
  use std::path::Path;

  // Discover and inspect plugins in .mmistudio/plugins/
  let discovery = PluginDiscovery::new(Path::new(".mmistudio/plugins"));
  let plugins = discovery.discover_all().unwrap();

  for p in plugins {
      println!("Discovered plugin: {} v{} (ABI v{})", p.manifest.name, p.manifest.version, p.manifest.abi_version);
      assert!(p.is_abi_compatible(), "Plugin must conform to workspace ABI");
  }
  ```
- **Test Command**: `cargo test -p mmi-plugin --offline`

---

## Running Workspace Crate Tests

Run tests across all 12 crates in offline mode:

```bash
cargo test --workspace --offline
```
