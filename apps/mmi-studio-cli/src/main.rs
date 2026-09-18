//! Audi MMI Studio Headless CLI Application.

use std::path::PathBuf;
use clap::{Parser, Subcommand};

mod commands;
use commands::{
    cmd_ai_generate, cmd_assets_export, cmd_assets_list, cmd_assets_replace,
    cmd_attest, cmd_build_media, cmd_canvas_preview, cmd_carve, cmd_entropy, cmd_extract,
    cmd_hexdump, cmd_inspect, cmd_plugins_inspect, cmd_plugins_list, cmd_plugins_verify,
    cmd_rebuild, cmd_recipe_apply, cmd_recipe_rebase, cmd_simulate_update, cmd_stock_recovery,
    cmd_strings_inspect, cmd_strings_overflow, cmd_validate, cmd_verify_rebuild,
};

#[derive(Parser)]
#[command(name = "mmi-studio-cli")]
#[command(author = "Audi MMI Studio Engineers")]
#[command(version = "0.1.0")]
#[command(about = "Offline engineering workstation for Audi MMI reverse-engineering and asset tooling", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Inspect binary format, metadata, hashes, and string contents
    Inspect {
        /// Path to target file
        file: PathBuf,
        /// Output formatted as JSON
        #[arg(long)]
        json: bool,
    },
    /// Render formatted virtualized hex dump of a byte slice
    Hexdump {
        /// Path to target file
        file: PathBuf,
        /// Starting byte offset
        #[arg(short, long, default_value_t = 0)]
        offset: usize,
        /// Number of bytes to display
        #[arg(short, long, default_value_t = 256)]
        length: usize,
    },
    /// Calculate sliding Shannon entropy and display segmented regions
    Entropy {
        /// Path to target file
        file: PathBuf,
        /// Analysis window size in bytes
        #[arg(short, long, default_value_t = 1024)]
        window: usize,
        /// Output formatted as JSON
        #[arg(long)]
        json: bool,
    },
    /// Scan target binary for embedded containers, file systems, and image headers
    Carve {
        /// Path to target file
        file: PathBuf,
        /// Output formatted as JSON
        #[arg(long)]
        json: bool,
    },
    /// Ingest and normalize package into Content-Addressed Storage and StageStore
    Extract {
        /// Source path to file or package folder
        source: PathBuf,
        /// Stage name
        #[arg(short, long, default_value = "default")]
        stage: String,
        /// Optional path to export normalized MMIProject JSON
        #[arg(short, long)]
        output_project: Option<PathBuf>,
        /// Output formatted as JSON
        #[arg(long)]
        json: bool,
    },
    /// Inspect decoded assets, conform replacements, or export them to PNG
    Assets {
        #[command(subcommand)]
        action: AssetCommands,
    },
    /// Localisation string catalog inspection and typography overflow verification
    Strings {
        #[command(subcommand)]
        action: StringsCommands,
    },
    /// Evaluate a candidate binary through the Identity-Rebuild Gate
    VerifyRebuild {
        /// Path to candidate binary
        file: PathBuf,
        /// Output formatted as JSON
        #[arg(long)]
        json: bool,
    },
    /// Render reconstructed screen canvas to PNG
    RenderScreen {
        /// Screen identifier
        #[arg(short, long, default_value = "MAIN_SCREEN")]
        screen: String,
        /// Palette mode: day, night, reduced
        #[arg(short, long, default_value = "day")]
        mode: String,
        /// Output PNG path
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Generate or modify an asset using AI models under strict Egress Airlock
    /// AI asset authoring with egress airlock
    AiGenerate {
        /// Prompt describing the requested visual asset
        #[arg(short, long)]
        prompt: String,
        /// Optional reference source asset
        #[arg(short, long)]
        source_asset: Option<PathBuf>,
        /// Output PNG path
        #[arg(short, long)]
        output: PathBuf,
        /// Width in pixels
        #[arg(long, default_value_t = 64)]
        width: u32,
        /// Height in pixels
        #[arg(long, default_value_t = 64)]
        height: u32,
        /// Output formatted as JSON
        #[arg(long)]
        json: bool,
    },
    /// Apply or rebase declarative modification recipes
    Recipe {
        #[command(subcommand)]
        action: RecipeCommands,
    },
    /// Deterministically repackage a staged tree into candidate update files
    Rebuild {
        /// Name of the stage to rebuild
        #[arg(short, long, default_value = "default")]
        stage: String,
        /// Output directory for rebuilt files
        #[arg(short, long)]
        output: PathBuf,
        /// Optional original directory to verify determinism against
        #[arg(short, long)]
        verify_against: Option<PathBuf>,
        /// Output formatted as JSON
        #[arg(long)]
        json: bool,
    },
    /// Run 6-tier (L0-L5) validation suite against a stage or package
    Validate {
        /// Path to target stage or directory
        target: PathBuf,
        /// Optional target profile JSON file
        #[arg(short, long)]
        profile: Option<PathBuf>,
        /// Maximum validation level to evaluate (0..5)
        #[arg(short, long)]
        level: Option<usize>,
        /// Output formatted as JSON
        #[arg(long)]
        json: bool,
    },
    /// Build deployable FAT32 SD media directory structure and volume manifests
    BuildMedia {
        /// Name of the stage to build into media
        #[arg(short, long, default_value = "default")]
        stage: String,
        /// Target output directory for media
        #[arg(short, long)]
        output: PathBuf,
        /// Volume label (max 11 characters)
        #[arg(long, default_value = "MMI3G_NAV")]
        volume_label: String,
        /// Maximum volume capacity in GB (default 32)
        #[arg(long, default_value_t = 32)]
        volume_size_gb: u64,
        /// Output formatted as JSON
        #[arg(long)]
        json: bool,
    },
    /// Simulate pre-flight QNX head-unit update flow on a media directory
    SimulateUpdate {
        /// Path to target deployment media directory
        media: PathBuf,
        /// Output formatted as JSON
        #[arg(long)]
        json: bool,
    },
    /// Emit immutable cryptographic build attestation manifest
    Attest {
        /// Path to source directory
        #[arg(short, long)]
        source: PathBuf,
        /// Path to built output directory
        #[arg(short, long)]
        build_dir: PathBuf,
        /// Source software train identifier
        #[arg(long, default_value = "HN+R_EU_AU_K0942_4")]
        source_train: String,
        /// Target stage identifier
        #[arg(long, default_value = "default")]
        stage: String,
        /// Optional recipe ID
        #[arg(long)]
        recipe_id: Option<String>,
        /// Output formatted as JSON
        #[arg(long)]
        json: bool,
    },
    /// Identify, package, and verify stock original baseline into an emergency recovery bundle
    StockRecovery {
        /// Baseline software train name (in originals/)
        #[arg(short, long, default_value = "HN+R_EU_AU_K0942_4_[8R0906961FB]")]
        baseline_train: String,
        /// Output directory for emergency recovery bundle
        #[arg(short, long)]
        output: PathBuf,
        /// Output formatted as JSON
        #[arg(long)]
        json: bool,
    },
    /// Manage, inspect, and verify third-party format adapter plugins
    Plugins {
        #[command(subcommand)]
        action: PluginCommands,
    },
}

#[derive(Subcommand)]
enum PluginCommands {
    /// List all discovered plugins in a directory
    List {
        /// Optional directory to scan (default .mmistudio/plugins)
        #[arg(short, long)]
        dir: Option<PathBuf>,
        /// Output formatted as JSON
        #[arg(long)]
        json: bool,
    },
    /// Inspect metadata and ABI compatibility of a plugin
    Inspect {
        /// Path to plugin directory or plugin.json
        plugin: PathBuf,
        /// Output formatted as JSON
        #[arg(long)]
        json: bool,
    },
    /// Verify plugin sandbox isolation and format detection against a test file
    Verify {
        /// Path to plugin directory or plugin.json
        plugin: PathBuf,
        /// Optional test file to run format detection and parsing on
        #[arg(short, long)]
        test_file: Option<PathBuf>,
        /// Output formatted as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum RecipeCommands {
    /// Apply a declarative recipe to a staging workspace
    Apply {
        /// Path to recipe JSON file
        #[arg(short, long)]
        recipe: PathBuf,
        /// Target staging name
        #[arg(short, long, default_value = "default")]
        stage: String,
        /// Output formatted as JSON
        #[arg(long)]
        json: bool,
    },
    /// Evaluate cross-train recipe portability and drift against a target train
    Rebase {
        /// Path to recipe JSON file
        #[arg(short, long)]
        recipe: PathBuf,
        /// Path to target train root folder
        #[arg(short, long)]
        target_train: PathBuf,
        /// Output formatted as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum StringsCommands {
    /// Inspect a multi-encoding localized string catalog file
    Inspect {
        /// Path to string catalog file
        file: PathBuf,
        /// Output formatted as JSON
        #[arg(long)]
        json: bool,
    },
    /// Evaluate whether a localized string overflows a target UI bounding box
    CheckOverflow {
        /// Text string to evaluate
        #[arg(short, long)]
        text: String,
        /// Path to TrueType font file (.ttf)
        #[arg(short, long)]
        font: PathBuf,
        /// Maximum allowed width in pixels
        #[arg(long, default_value_t = 300.0)]
        max_width: f32,
        /// Maximum allowed height in pixels
        #[arg(long, default_value_t = 40.0)]
        max_height: f32,
        /// Font size in points
        #[arg(long, default_value_t = 16.0)]
        font_size: f32,
        /// Maximum line count
        #[arg(long, default_value_t = 1)]
        max_lines: usize,
        /// Output formatted as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum AssetCommands {
    /// List visual and font assets cataloged in a stage
    List {
        /// Stage name
        #[arg(short, long, default_value = "default")]
        stage: String,
        /// Output formatted as JSON
        #[arg(long)]
        json: bool,
    },
    /// Decode a proprietary asset (e.g. .precomp) and export to standard PNG
    Export {
        /// Path to input asset file
        asset: PathBuf,
        /// Output PNG path
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Conform and replace an asset with a candidate image file
    Replace {
        /// Path to target asset file (defines required constraints)
        #[arg(short, long)]
        target: PathBuf,
        /// Path to candidate replacement image (PNG/BMP/etc.)
        #[arg(short, long)]
        replacement: PathBuf,
        /// Optional path to export the resulting conformed binary
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Output formatted as JSON
        #[arg(long)]
        json: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Inspect { file, json } => cmd_inspect(file, *json),
        Commands::Hexdump { file, offset, length } => cmd_hexdump(file, *offset, *length),
        Commands::Entropy { file, window, json } => cmd_entropy(file, *window, *json),
        Commands::Carve { file, json } => cmd_carve(file, *json),
        Commands::Extract {
            source,
            stage,
            output_project,
            json,
        } => cmd_extract(source, stage, output_project.as_deref(), *json),
        Commands::Assets { action } => match action {
            AssetCommands::List { stage, json } => cmd_assets_list(stage, *json),
            AssetCommands::Export { asset, output } => cmd_assets_export(asset, output),
            AssetCommands::Replace {
                target,
                replacement,
                output,
                json,
            } => cmd_assets_replace(target, replacement, output.as_deref(), *json),
        },
        Commands::Strings { action } => match action {
            StringsCommands::Inspect { file, json } => cmd_strings_inspect(file, *json),
            StringsCommands::CheckOverflow {
                text,
                font,
                max_width,
                max_height,
                font_size,
                max_lines,
                json,
            } => cmd_strings_overflow(
                text,
                font,
                *max_width,
                *max_height,
                *font_size,
                *max_lines,
                *json,
            ),
        },
        Commands::VerifyRebuild { file, json } => cmd_verify_rebuild(file, *json),
        Commands::RenderScreen {
            screen,
            mode,
            output,
        } => cmd_canvas_preview(screen, mode, output),
        Commands::AiGenerate {
            prompt,
            source_asset,
            output,
            width,
            height,
            json,
        } => cmd_ai_generate(
            prompt,
            source_asset.as_deref(),
            output,
            *width,
            *height,
            *json,
        ),
        Commands::Recipe { action } => match action {
            RecipeCommands::Apply {
                recipe,
                stage,
                json,
            } => cmd_recipe_apply(recipe, stage, *json),
            RecipeCommands::Rebase {
                recipe,
                target_train,
                json,
            } => cmd_recipe_rebase(recipe, target_train, *json),
        },
        Commands::Rebuild {
            stage,
            output,
            verify_against,
            json,
        } => cmd_rebuild(stage, output, verify_against.as_deref(), *json),
        Commands::Validate {
            target,
            profile,
            level,
            json,
        } => cmd_validate(target, profile.as_deref(), *level, *json),
        Commands::BuildMedia {
            stage,
            output,
            volume_label,
            volume_size_gb,
            json,
        } => cmd_build_media(stage, output, volume_label, *volume_size_gb, *json),
        Commands::SimulateUpdate { media, json } => cmd_simulate_update(media, *json),
        Commands::Attest {
            source,
            build_dir,
            source_train,
            stage,
            recipe_id,
            json,
        } => cmd_attest(
            source,
            build_dir,
            source_train,
            stage,
            recipe_id.clone(),
            *json,
        ),
        Commands::StockRecovery {
            baseline_train,
            output,
            json,
        } => cmd_stock_recovery(baseline_train, output, *json),
        Commands::Plugins { action } => match action {
            PluginCommands::List { dir, json } => cmd_plugins_list(dir.as_deref(), *json),
            PluginCommands::Inspect { plugin, json } => cmd_plugins_inspect(plugin, *json),
            PluginCommands::Verify {
                plugin,
                test_file,
                json,
            } => cmd_plugins_verify(plugin, test_file.as_deref(), *json),
        },
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
