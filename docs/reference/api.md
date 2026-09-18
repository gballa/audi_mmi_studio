# Reference — Rust API & IPC Interface

This document catalogs the public Rust APIs exposed by the 12 workspace crates under `crates/` and the native Tauri IPC handlers in `apps/mmi-studio-desktop`.

---

## 1. Core Crates API

### `mmi-core`
- `ContentAddressedStore`:
  - `new(base_path: &Path) -> Result<Self, CoreError>`
  - `put_blob(&self, data: &[u8]) -> Result<String, CoreError>`
  - `get_blob(&self, blob_id: &str) -> Result<Vec<u8>, CoreError>`
  - `has_blob(&self, blob_id: &str) -> bool`
- `SourceStore`:
  - `new(originals_root: &Path) -> Result<Self, CoreError>`
  - `read_bytes(&self, rel_path: &str) -> Result<Vec<u8>, CoreError>`
  - `exists(&self, rel_path: &str) -> bool`

### `mmi-formats`
- `FormatAdapter` (Trait):
  - `fn name(&self) -> &'static str`
  - `fn detect(&self, data: &[u8]) -> bool`
  - `fn extract_strings(&self, data: &[u8]) -> Vec<String>`
  - `fn validate(&self, data: &[u8]) -> bool`
- Implementations: `HBNavDBAdapter`, `OrionAtlasAdapter`, `PrecompAdapter`, `MapStylesAdapter`, `QnxIfsAdapter`, `QnxEfsAdapter`, `SpeechAnsAdapter`, `SystemFpgaAdapter`, `SmscInicAdapter`, `GeographicGdbAdapter`, `BinaryGrammarAdapter`, `AdiBlackfinDspAdapter`.

### `mmi-assets`
- `PrecompImage`:
  - `decode(data: &[u8]) -> Result<Self, CoreError>`
  - `encode(&self) -> Result<Vec<u8>, CoreError>`
- `FormatConformer`:
  - `conform_image(candidate_bytes: &[u8], target_spec: &DecodedBitmap) -> Result<ConformResult, CoreError>`
- `ThumbnailGenerator`:
  - `create_thumbnail(bitmap: &DecodedBitmap, max_edge: u32, cas: &ContentAddressedStore) -> Result<String, CoreError>`

### `mmi-canvas`
- `ScreenComposition`:
  - `new(width: u32, height: u32, mode: DisplayMode) -> Self`
  - `add_chrome_layer(&mut self, name: &str)`
  - `add_widget(&mut self, name: &str, x: u32, y: u32, w: u32, h: u32)`
- `CanvasRenderer`:
  - `new() -> Self`
  - `render_frame(&self, comp: &ScreenComposition) -> Result<RenderedFrame, CoreError>`

### `mmi-re-lab`
- `EntropyCalculator`:
  - `calculate_entropy(data: &[u8], window_size: usize) -> (f64, EntropyClassification)`
  - `segment_entropy(data: &[u8], window_size: usize) -> Vec<(usize, f64)>`
- `HexViewer`:
  - `render_slice(data: &[u8], offset: usize, bytes_per_row: usize) -> Vec<HexRow>`
- `SignatureCarver`:
  - `carve_signatures(&self, data: &[u8]) -> Vec<CarvedSignature>`

### `mmi-recipe`
- `RecipeModel`:
  - `load_from_json(path: &Path) -> Result<Self, CoreError>`
  - `validate(&self) -> Result<(), CoreError>`
- `RecipeEngine`:
  - `apply_recipe(&self, recipe: &RecipeModel, stage_path: &Path) -> Result<RecipeExecutionReport, CoreError>`
- `RebaseEngine`:
  - `evaluate_drift(recipe: &RecipeModel, target_train_path: &Path) -> Result<RebaseReport, CoreError>`

### `mmi-rebuild`
- `StageNormalizer`:
  - `normalize_stage(stage_path: &Path) -> Result<NormalizedTree, CoreError>`
- `IdentityRebuildVerifier`:
  - `verify_candidate(&self, candidate_bytes: &[u8], rel_path: &Path) -> RebuildVerificationResult`

### `mmi-validation`
- `ValidationEngine`:
  - `validate_directory(&self, target_dir: &Path, profile: Option<&TargetProfile>) -> Result<ValidationReport, CoreError>`

### `mmi-media`
- `MediaBuilder`:
  - `new(volume_label: &str, volume_size_gb: u64) -> Self`
  - `build_package(&self, stage_dir: &Path, output_dir: &Path) -> Result<VolumeManifest, CoreError>`
- `UpdateSimulator`:
  - `simulate_media_update(&self, media_dir: &Path) -> Result<SimulationReport, CoreError>`

### `mmi-attestation`
- `AttestationBuilder`:
  - `generate(&self, source_dir: &Path, build_dir: &Path, recipe_path: Option<&str>) -> Result<AttestationManifest, CoreError>`
- `EmergencyRecoveryManager`:
  - `package_stock_bundle(&self, baseline_train_id: &str, output_dir: &Path) -> Result<(), CoreError>`

### `mmi-imagegen`
- `EgressAirlock`:
  - `generate_asset(&self, req: &GenerationRequest) -> Result<DecodedBitmap, CoreError>`

### `mmi-plugin`
- `PluginDiscovery`:
  - `discover_all(&self) -> Result<Vec<DiscoveredPlugin>, CoreError>`

---

## 2. Desktop IPC Bridge API (`src-tauri/src/ipc.rs`)

| Function Name | Signature | Description |
| :--- | :--- | :--- |
| `handle_inspect_file` | `(rel_path: &str, cas_base: &Path) -> Result<InspectResult, CoreError>` | Returns size, BLAKE3, detected format, and CAS thumbnail ID. |
| `handle_hexdump` | `(rel_path: &str, offset: usize, length: usize) -> Result<HexDumpResult, CoreError>` | Returns formatted hex rows for virtualized grid rendering. |
| `handle_entropy` | `(rel_path: &str, window: usize) -> Result<EntropyResult, CoreError>` | Returns sliding Shannon entropy floats and classifications. |
| `handle_render_screen` | `(mode: &str) -> Result<ScreenRenderResult, CoreError>` | Renders an 800x480 screen frame in Day, Night, or Reduced mode. |
