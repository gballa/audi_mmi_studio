//! Firmware package command integrating QNX IFS and F3S filesystem synthesis engines.

use std::path::Path;
use mmi_core::CoreError;
use mmi_rebuild::{FirmwareBundleConfig, FirmwareBundlePipeline};

/// Builds and packages the full system firmware SD card bundle.
pub fn cmd_firmware_bundle(
    output: &Path,
    train: &str,
    release: &str,
    variant: &str,
    splash_png: Option<&Path>,
    strings_ans: Option<&Path>,
    gem_esd: Option<&Path>,
    nav_db: Option<&Path>,
    as_json: bool,
) -> Result<(), CoreError> {
    let splash_screen_png = if let Some(p) = splash_png {
        Some(std::fs::read(p)?)
    } else {
        None
    };

    let albanian_strings_ans = if let Some(p) = strings_ans {
        Some(std::fs::read(p)?)
    } else {
        None
    };

    let gem_screen_esd = if let Some(p) = gem_esd {
        Some(std::fs::read(p)?)
    } else {
        None
    };

    let nav_database_fldb = if let Some(p) = nav_db {
        Some(std::fs::read(p)?)
    } else {
        None
    };

    let config = FirmwareBundleConfig {
        train: train.to_string(),
        release: release.to_string(),
        variant: variant.to_string(),
        splash_screen_png,
        albanian_strings_ans,
        gem_screen_esd,
        nav_database_fldb,
        map_styles_gdb: None,
        regional_profile: Some("AL".to_string()),
        ..Default::default()
    };

    let pipeline = FirmwareBundlePipeline::new(config);
    let report = pipeline.build(output)?;

    if as_json {
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
    } else {
        println!("══════════════════════════════════════════════════════════════════════════");
        println!(" Audi MMI 3G/3G+ Full System Firmware SD Bundle Generator");
        println!("══════════════════════════════════════════════════════════════════════════");
        println!("Target Train:         {}", report.target_train);
        println!("Release Version:      {}", report.target_release);
        println!("Hardware Variant:     {}", report.target_variant);
        println!("Output Bundle Dir:    {}", report.output_dir);
        println!("Safety Status:        {}", report.safety_status);
        println!("──────────────────────────────────────────────────────────────────────────");
        println!("NOR Flash Partition Capacities:");
        for p in &report.partitions {
            println!(
                "  - {:<16} {:>8} / {:>8} bytes ({:.2}% used)",
                p.partition_name, p.allocated_bytes, p.max_bytes, p.percentage_used
            );
        }
        println!("──────────────────────────────────────────────────────────────────────────");
        println!("Generated SD Bundle Artifacts ({} files):", report.files.len());
        for f in &report.files {
            println!("  • {:<30} ({:>8} bytes)  BLAKE3: {}...", f.path, f.size_bytes, &f.blake3[..12]);
        }
        println!("──────────────────────────────────────────────────────────────────────────");
        println!("Deployment Instructions (§14.9 Pre-Flash Checklist):");
        println!("  1. Copy entire contents of '{}' to the root of a FAT32 SD card.", report.output_dir);
        println!("  2. Insert into SD Slot 1 of Audi MMI 3G+ unit.");
        println!("  3. Automatic execution: proc_scriptlauncher detects 'copie_scr.sh'.");
        println!("  4. Manual SWDL upgrade: Red Engineering Menu (CAR + BACK or SETUP + RETURN).");
        println!("  5. Emergency UART Rollback: 'sh /fs/sda0/stock_recovery.sh'.");
        println!("══════════════════════════════════════════════════════════════════════════");
    }

    Ok(())
}

/// Alias for `cmd_firmware_bundle` conforming to CLI nomenclature.
#[allow(dead_code)]
pub fn cmd_firmware_package(
    output: &Path,
    train: &str,
    release: &str,
    variant: &str,
    splash_png: Option<&Path>,
    strings_ans: Option<&Path>,
    gem_esd: Option<&Path>,
    nav_db: Option<&Path>,
    as_json: bool,
) -> Result<(), CoreError> {
    cmd_firmware_bundle(
        output,
        train,
        release,
        variant,
        splash_png,
        strings_ans,
        gem_esd,
        nav_db,
        as_json,
    )
}
