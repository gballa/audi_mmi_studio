use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_e2e_theming_and_packaging_pipeline() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let cli_bin = env!("CARGO_BIN_EXE_mmi-studio-cli");

    let stage_name = "e2e_amber_stage";
    let staged_stage_dir = workspace_dir.join(".mmistudio").join("stages").join(stage_name);
    fs::create_dir_all(&staged_stage_dir).unwrap();

    let temp_dir = TempDir::new().expect("failed to create tempdir");
    let temp_path = temp_dir.path();

    let media_dir = temp_path.join("media_out");
    let recovery_dir = temp_path.join("recovery_out");
    fs::create_dir_all(&media_dir).unwrap();
    fs::create_dir_all(&recovery_dir).unwrap();

    // 1. Copy baseline files from originals into stage
    let originals_dir = workspace_dir.join("originals");
    let train_dir = originals_dir.join("HN+R_EU_AU_K0942_4_[8R0906961FB]");
    let source_precomp = train_dir.join("CombiStyles/IND/0/default/arr_e.precomp");
    let source_metainfo = train_dir.join("metainfo2.txt");

    let staged_precomp_dir = staged_stage_dir.join("CombiStyles/IND/0/default");
    fs::create_dir_all(&staged_precomp_dir).unwrap();
    fs::copy(&source_precomp, staged_precomp_dir.join("arr_e.precomp")).unwrap();
    fs::copy(&source_metainfo, staged_stage_dir.join("metainfo2.txt")).unwrap();

    // 2. Read genuine precomp and create amber-themed candidate PNG
    let orig_bytes = fs::read(&source_precomp).unwrap();
    let decoded = mmi_formats::PrecompImage::decode(&orig_bytes).unwrap();
    assert!(decoded.width > 0 && decoded.height > 0);

    // Apply amber tint to all non-transparent pixels (RGBA)
    let mut modified_pixels = decoded.pixels.clone();
    for chunk in modified_pixels.chunks_exact_mut(4) {
        let alpha = chunk[3];
        if alpha > 30 {
            // Apply Audi Sport Amber [255, 179, 0]
            chunk[0] = 255;
            chunk[1] = 179;
            chunk[2] = 0;
        }
    }

    let img_buffer: image::RgbaImage = image::ImageBuffer::from_raw(
        decoded.width as u32,
        decoded.height as u32,
        modified_pixels,
    ).expect("failed to create image buffer");

    let candidate_png = temp_path.join("candidate_arr_e.png");
    img_buffer.save(&candidate_png).expect("failed to save candidate PNG");

    // 3. Formulate declarative Theme Recipe
    let recipe_json = format!(
        r#"{{
  "api_version": "mmi.recipe/v1",
  "metadata": {{
    "name": "Audi Sport Amber Indicator",
    "version": "1.0.0",
    "author": "Audi MMI Studio",
    "description": "Custom amber turn indicator style for instrument cluster",
    "base_train": "HN+R_EU_AU_K0942_4",
    "target_trains": ["HN+R_EU_AU_K0942_4", "HN+R_EU_AU_P0900_1"],
    "created_at": "2026-09-18T21:00:00Z"
  }},
  "operations": [
    {{
      "op": "replace_asset",
      "selector": {{
        "type": "AssetPath",
        "target": "CombiStyles/IND/0/default/arr_e.precomp"
      }},
      "replacement_path": "{}",
      "evidence_tag": "[EV:tree@originals/HN+R_EU_AU_K0942_4_[8R0906961FB]/CombiStyles/IND/0/default/arr_e.precomp]"
    }}
  ]
}}"#,
        candidate_png.display()
    );

    let recipe_path = temp_path.join("amber_theme.recipe.json");
    fs::write(&recipe_path, recipe_json).unwrap();

    // 4. CLI: recipe apply (Conform PNG into target precomp)
    let out_recipe = Command::new(cli_bin)
        .current_dir(workspace_dir)
        .arg("recipe")
        .arg("apply")
        .arg("--recipe")
        .arg(&recipe_path)
        .arg("--stage")
        .arg(stage_name)
        .arg("--json")
        .output()
        .expect("failed to execute cli recipe apply");

    assert!(out_recipe.status.success(), "stderr: {}", String::from_utf8_lossy(&out_recipe.stderr));
    let stdout_recipe = String::from_utf8_lossy(&out_recipe.stdout);
    assert!(stdout_recipe.contains("\"operations_applied\": 1"));

    // 5. CLI: verify-rebuild (Identity-Rebuild Gate on modified staged asset)
    let out_verify_rebuild = Command::new(cli_bin)
        .current_dir(workspace_dir)
        .arg("verify-rebuild")
        .arg(staged_precomp_dir.join("arr_e.precomp"))
        .arg("--json")
        .output()
        .expect("failed to execute cli verify-rebuild");

    assert!(out_verify_rebuild.status.success(), "stderr: {}", String::from_utf8_lossy(&out_verify_rebuild.stderr));
    let stdout_rebuild = String::from_utf8_lossy(&out_verify_rebuild.stdout);
    assert!(stdout_rebuild.contains("\"can_rebuild\": true"));
    assert!(stdout_rebuild.contains("Passed"));

    // 6. CLI: validate (Conformance Pipeline against stage)
    let out_validate = Command::new(cli_bin)
        .current_dir(workspace_dir)
        .arg("validate")
        .arg(&staged_stage_dir)
        .arg("--json")
        .output()
        .expect("failed to execute cli validate");

    assert!(out_validate.status.success(), "stderr: {}", String::from_utf8_lossy(&out_validate.stderr));
    let stdout_validate = String::from_utf8_lossy(&out_validate.stdout);
    assert!(stdout_validate.contains("\"error_count\": 0"));

    // 7. CLI: attest (Cryptographic Attestation Chain)
    let out_attest = Command::new(cli_bin)
        .current_dir(workspace_dir)
        .arg("attest")
        .arg("--source")
        .arg(&train_dir)
        .arg("--build-dir")
        .arg(&staged_stage_dir)
        .arg("--json")
        .output()
        .expect("failed to execute cli attest");

    assert!(out_attest.status.success(), "stderr: {}", String::from_utf8_lossy(&out_attest.stderr));
    let stdout_attest = String::from_utf8_lossy(&out_attest.stdout);
    assert!(stdout_attest.contains("\"status_verdict\""));
    assert!(!stdout_attest.to_uppercase().contains("SAFE TO INSTALL"));

    // 8. CLI: build-media (Deployable SD card structure)
    let out_media = Command::new(cli_bin)
        .current_dir(workspace_dir)
        .arg("build-media")
        .arg("--stage")
        .arg(stage_name)
        .arg("--output")
        .arg(&media_dir)
        .arg("--volume-label")
        .arg("MMI3G_AMBER")
        .arg("--json")
        .output()
        .expect("failed to execute cli build-media");

    assert!(out_media.status.success(), "stderr: {}", String::from_utf8_lossy(&out_media.stderr));
    let stdout_media = String::from_utf8_lossy(&out_media.stdout);
    assert!(stdout_media.contains("\"volume_label\""));

    // Determine path where volume metainfo2.txt resides
    let target_sim_dir = if media_dir.join("metainfo2.txt").exists() {
        media_dir.clone()
    } else {
        media_dir.join("MMI3G_AMBER_VOL1")
    };

    // 9. CLI: simulate-update (Pre-flight QNX update simulator)
    let out_sim = Command::new(cli_bin)
        .current_dir(workspace_dir)
        .arg("simulate-update")
        .arg(&target_sim_dir)
        .arg("--json")
        .output()
        .expect("failed to execute cli simulate-update");

    assert!(out_sim.status.success(), "stderr: {}", String::from_utf8_lossy(&out_sim.stderr));
    let stdout_sim = String::from_utf8_lossy(&out_sim.stdout);
    assert!(stdout_sim.contains("\"overall_success\": true"));
    assert!(stdout_sim.contains("SIMULATED — NOT A GUARANTEE"));

    // 10. CLI: stock-recovery (Emergency rollback bundle creation)
    let out_rec = Command::new(cli_bin)
        .current_dir(workspace_dir)
        .arg("stock-recovery")
        .arg("--baseline-train")
        .arg("HN+R_EU_AU_K0942_4_[8R0906961FB]")
        .arg("--output")
        .arg(&recovery_dir)
        .arg("--json")
        .output()
        .expect("failed to execute cli stock-recovery");

    assert!(out_rec.status.success(), "stderr: {}", String::from_utf8_lossy(&out_rec.stderr));
    let stdout_rec = String::from_utf8_lossy(&out_rec.stdout);
    assert!(stdout_rec.contains("\"is_recovery_ready\": true"));
    assert!(recovery_dir.join("RECOVERY_README.md").exists());


    // Clean up temporary stage directory
    let _ = fs::remove_dir_all(&staged_stage_dir);
}
