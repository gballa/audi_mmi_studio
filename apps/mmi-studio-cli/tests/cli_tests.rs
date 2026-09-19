use std::path::Path;
use std::process::Command;

#[test]
fn test_cli_inspect_metainfo2() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let metainfo_path = workspace_dir
        .join("originals")
        .join("HN+R_EU_AU_K0942_4_[8R0906961FB]")
        .join("metainfo2.txt");

    let output = Command::new(env!("CARGO_BIN_EXE_mmi-studio-cli"))
        .arg("inspect")
        .arg(&metainfo_path)
        .arg("--json")
        .output()
        .expect("Failed to execute mmi-studio-cli inspect");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("HN+R_EU_AU_K0942_4"));
    assert!(stdout.contains("HBAS"));
}

#[test]
fn test_cli_hexdump_and_entropy() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let metainfo_path = workspace_dir
        .join("originals")
        .join("HN+R_EU_AU_K0942_4_[8R0906961FB]")
        .join("metainfo2.txt");

    // Hexdump test
    let hex_out = Command::new(env!("CARGO_BIN_EXE_mmi-studio-cli"))
        .arg("hexdump")
        .arg(&metainfo_path)
        .arg("--length")
        .arg("64")
        .output()
        .expect("Failed to execute hexdump");
    assert!(hex_out.status.success());
    let hex_str = String::from_utf8_lossy(&hex_out.stdout);
    assert!(hex_str.contains("00000000"));

    // Entropy test
    let ent_out = Command::new(env!("CARGO_BIN_EXE_mmi-studio-cli"))
        .arg("entropy")
        .arg(&metainfo_path)
        .arg("--json")
        .output()
        .expect("Failed to execute entropy");
    assert!(ent_out.status.success());
    let ent_str = String::from_utf8_lossy(&ent_out.stdout);
    assert!(ent_str.contains("overall_entropy"));
}

#[test]
fn test_cli_carve_precomp() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let precomp_path = workspace_dir
        .join("originals")
        .join("HN+R_EU_AU_K0942_4_[8R0906961FB]")
        .join("CombiStyles")
        .join("IND")
        .join("0")
        .join("default")
        .join("arr_e.precomp");

    let output = Command::new(env!("CARGO_BIN_EXE_mmi-studio-cli"))
        .arg("carve")
        .arg(&precomp_path)
        .arg("--json")
        .output()
        .expect("Failed to execute carve");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("HarmanPrecomp"));
    assert!(stdout.contains("ZlibStream"));
}

#[test]
fn test_cli_extract_sample() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let metainfo_path = workspace_dir
        .join("originals")
        .join("HN+R_EU_AU_K0942_4_[8R0906961FB]")
        .join("metainfo2.txt");

    let output = Command::new(env!("CARGO_BIN_EXE_mmi-studio-cli"))
        .current_dir(workspace_dir)
        .arg("extract")
        .arg(&metainfo_path)
        .arg("--stage")
        .arg("cli_test_stage")
        .arg("--json")
        .output()
        .expect("failed to execute cli");

    assert!(output.status.success(), "CLI stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("PRJ-cli_test_stage"));
}

#[test]
fn test_cli_assets_export_precomp() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let precomp_path = workspace_dir
        .join("originals")
        .join("HN+R_EU_AU_K0942_4_[8R0906961FB]")
        .join("RSU")
        .join("graphics")
        .join("view_kombi_k0.precomp");

    if !precomp_path.exists() {
        return;
    }

    let temp_out = tempfile::NamedTempFile::new().unwrap();
    let out_path = temp_out.path().with_extension("png");

    let output = Command::new(env!("CARGO_BIN_EXE_mmi-studio-cli"))
        .current_dir(workspace_dir)
        .arg("assets")
        .arg("export")
        .arg(&precomp_path)
        .arg("--output")
        .arg(&out_path)
        .output()
        .expect("failed to execute cli assets export");

    assert!(output.status.success(), "CLI stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(out_path.exists());
    let png_bytes = std::fs::read(&out_path).unwrap();
    assert_eq!(&png_bytes[1..4], b"PNG");
}

#[test]
fn test_cli_verify_rebuild_gate() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let precomp_path = workspace_dir
        .join("originals")
        .join("HN+R_EU_AU_K0942_4_[8R0906961FB]")
        .join("CombiStyles")
        .join("IND")
        .join("0")
        .join("default")
        .join("arr_e.precomp");

    if !precomp_path.exists() {
        return;
    }

    let output = Command::new(env!("CARGO_BIN_EXE_mmi-studio-cli"))
        .current_dir(workspace_dir)
        .arg("verify-rebuild")
        .arg(&precomp_path)
        .arg("--json")
        .output()
        .expect("failed to execute cli verify-rebuild");

    assert!(output.status.success(), "CLI stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("\"can_rebuild\": true"));
}

#[test]
fn test_cli_render_screen() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let temp_out = tempfile::NamedTempFile::new().unwrap();
    let out_path = temp_out.path().with_extension("png");

    let output = Command::new(env!("CARGO_BIN_EXE_mmi-studio-cli"))
        .current_dir(workspace_dir)
        .arg("render-screen")
        .arg("--screen")
        .arg("TEST_NAV_SCREEN")
        .arg("--mode")
        .arg("night")
        .arg("--output")
        .arg(&out_path)
        .output()
        .expect("failed to execute render-screen");

    assert!(output.status.success(), "CLI stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(out_path.exists());
    let png = std::fs::read(&out_path).unwrap();
    assert_eq!(&png[1..4], b"PNG");
}

#[test]
fn test_cli_assets_replace() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let precomp_path = workspace_dir
        .join("originals")
        .join("HN+R_EU_AU_K0942_4_[8R0906961FB]")
        .join("CombiStyles")
        .join("IND")
        .join("0")
        .join("default")
        .join("arr_e.precomp");

    if !precomp_path.exists() {
        return;
    }

    // Create a candidate replacement image
    let temp_candidate = tempfile::NamedTempFile::new().unwrap();
    let candidate_path = temp_candidate.path().with_extension("png");

    let img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        image::ImageBuffer::from_pixel(100, 100, image::Rgba([120, 80, 200, 255]));
    img.save(&candidate_path).unwrap();

    let temp_out = tempfile::NamedTempFile::new().unwrap();
    let out_precomp = temp_out.path().with_extension("precomp");

    let output = Command::new(env!("CARGO_BIN_EXE_mmi-studio-cli"))
        .current_dir(workspace_dir)
        .arg("assets")
        .arg("replace")
        .arg("--target")
        .arg(&precomp_path)
        .arg("--replacement")
        .arg(&candidate_path)
        .arg("--output")
        .arg(&out_precomp)
        .arg("--json")
        .output()
        .expect("failed to execute cli assets replace");

    assert!(output.status.success(), "CLI stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(out_precomp.exists());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("\"target_asset\""));
    assert!(stdout.contains("\"width\": 500"));
    assert!(stdout.contains("\"height\": 248"));
}

#[test]
fn test_cli_ai_generate_airlock() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let temp_out = tempfile::NamedTempFile::new().unwrap();
    let out_png = temp_out.path().with_extension("png");

    let output = Command::new(env!("CARGO_BIN_EXE_mmi-studio-cli"))
        .current_dir(workspace_dir)
        .arg("ai-generate")
        .arg("--prompt")
        .arg("turn right arrow icon for VIN WAUZZZ4G0BN999999")
        .arg("--output")
        .arg(&out_png)
        .arg("--json")
        .output()
        .expect("failed to execute ai-generate");

    assert!(output.status.success(), "CLI stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(out_png.exists());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("[REDACTED_VIN]"));
    assert!(stdout.contains("\"model\""));
    assert!(stdout.contains("\"blob_id\""));
}

#[test]
fn test_cli_strings_inspect_and_overflow() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let plde_path = workspace_dir
        .join("originals")
        .join("HN+R_EU_AU_K0942_4_[8R0906961FB]")
        .join("sss")
        .join("tts_de_DE")
        .join("0")
        .join("default")
        .join("plde-DE0.txt");

    if plde_path.exists() {
        let output = Command::new(env!("CARGO_BIN_EXE_mmi-studio-cli"))
            .current_dir(workspace_dir)
            .arg("strings")
            .arg("inspect")
            .arg(&plde_path)
            .arg("--json")
            .output()
            .expect("failed to execute cli strings inspect");

        assert!(output.status.success(), "CLI stderr: {}", String::from_utf8_lossy(&output.stderr));
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("\"encoding\": \"Latin1\""));
        assert!(stdout.contains("\"entries\""));
    }

    let font_path = workspace_dir
        .join("originals")
        .join("HN+R_EU_AU_K0942_4_[8R0906961FB]")
        .join("GEMMI")
        .join("nav")
        .join("0")
        .join("default")
        .join("res")
        .join("AudiUnivers540Med.ttf");

    if font_path.exists() {
        let output = Command::new(env!("CARGO_BIN_EXE_mmi-studio-cli"))
            .current_dir(workspace_dir)
            .arg("strings")
            .arg("check-overflow")
            .arg("--text")
            .arg("Navigation Destination Input")
            .arg("--font")
            .arg(&font_path)
            .arg("--max-width")
            .arg("500")
            .arg("--json")
            .output()
            .expect("failed to execute cli strings check-overflow");

        assert!(output.status.success(), "CLI stderr: {}", String::from_utf8_lossy(&output.stderr));
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("\"fits\": true"));
    }
}

#[test]
fn test_cli_recipe_rebase() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let temp_recipe = tempfile::NamedTempFile::new().unwrap();
    let recipe_json = r#"{
        "api_version": "mmi.recipe/v1",
        "metadata": {
            "name": "CLI Rebase Test",
            "version": "1.0.0",
            "author": "Engineers",
            "base_train": "HN+R_EU_AU_K0942_4",
            "created_at": "2026-09-18T17:30:00Z"
        },
        "operations": [
            {
                "op": "replace_asset",
                "selector": {
                    "type": "AssetPath",
                    "target": "GEMMI/nav/0/default/res/AudiUnivers540Med.ttf"
                },
                "replacement_path": "new_font.ttf"
            },
            {
                "op": "replace_asset",
                "selector": {
                    "type": "AssetPath",
                    "target": "ifs-root.pkg"
                },
                "replacement_path": "new.pkg"
            }
        ]
    }"#;
    std::fs::write(temp_recipe.path(), recipe_json).unwrap();

    let target_train = workspace_dir.join("originals").join("HN+R_EU_AU_K0942_4_[8R0906961FB]");

    if target_train.exists() {
        let output = Command::new(env!("CARGO_BIN_EXE_mmi-studio-cli"))
            .current_dir(workspace_dir)
            .arg("recipe")
            .arg("rebase")
            .arg("--recipe")
            .arg(temp_recipe.path())
            .arg("--target-train")
            .arg(&target_train)
            .arg("--json")
            .output()
            .expect("failed to execute cli recipe rebase");

        assert!(output.status.success(), "CLI stderr: {}", String::from_utf8_lossy(&output.stderr));
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("\"clean_count\": 1"));
        assert!(stdout.contains("\"unsupported_count\": 1"));
    }
}

#[test]
fn test_cli_rebuild_stage() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let temp_stage_dir = workspace_dir.join(".mmistudio").join("stages").join("test_stage");
    std::fs::create_dir_all(&temp_stage_dir).unwrap();
    std::fs::write(temp_stage_dir.join("config.txt"), b"TITLE=TEST_STAGE\n").unwrap();

    let temp_output = tempfile::TempDir::new().unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_mmi-studio-cli"))
        .current_dir(workspace_dir)
        .arg("rebuild")
        .arg("--stage")
        .arg("test_stage")
        .arg("--output")
        .arg(temp_output.path())
        .arg("--json")
        .output()
        .expect("failed to execute cli rebuild");

    assert!(output.status.success(), "CLI stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("\"stage\": \"test_stage\""));
    assert!(stdout.contains("\"repackaged_files\""));
    assert!(temp_output.path().join("config.txt").exists());

    // Clean up test stage
    let _ = std::fs::remove_dir_all(&temp_stage_dir);
}

#[test]
fn test_cli_validate_corpus() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let corpus_dir = workspace_dir.join("originals").join("HN+R_EU_AU_K0942_4_[8R0906961FB]");

    if corpus_dir.exists() {
        let output = Command::new(env!("CARGO_BIN_EXE_mmi-studio-cli"))
            .current_dir(workspace_dir)
            .arg("validate")
            .arg(&corpus_dir)
            .arg("--json")
            .output()
            .expect("failed to execute cli validate");

        assert!(output.status.success(), "CLI stderr: {}", String::from_utf8_lossy(&output.stderr));
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("\"error_count\": 0"));
        assert!(stdout.contains("\"status\""));
    }
}

#[test]
fn test_cli_simulate_update_corpus() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let corpus_dir = workspace_dir.join("originals").join("HN+R_EU_AU_K0942_4_[8R0906961FB]");

    if corpus_dir.exists() {
        let output = Command::new(env!("CARGO_BIN_EXE_mmi-studio-cli"))
            .current_dir(workspace_dir)
            .arg("simulate-update")
            .arg(&corpus_dir)
            .arg("--json")
            .output()
            .expect("failed to execute cli simulate-update");

        assert!(output.status.success(), "CLI stderr: {}", String::from_utf8_lossy(&output.stderr));
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("\"disclaimer\": \"SIMULATED — NOT A GUARANTEE\""));
        assert!(stdout.contains("\"overall_success\": true"));
    }
}

#[test]
fn test_cli_attest_and_stock_recovery() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let temp_src = tempfile::TempDir::new().unwrap();
    let temp_build = tempfile::TempDir::new().unwrap();
    std::fs::write(temp_src.path().join("in.txt"), b"in").unwrap();
    std::fs::write(temp_build.path().join("out.txt"), b"out").unwrap();

    let output_attest = Command::new(env!("CARGO_BIN_EXE_mmi-studio-cli"))
        .current_dir(workspace_dir)
        .arg("attest")
        .arg("--source")
        .arg(temp_src.path())
        .arg("--build-dir")
        .arg(temp_build.path())
        .arg("--source-train")
        .arg("TEST_TRAIN")
        .arg("--stage")
        .arg("test_stage")
        .arg("--json")
        .output()
        .expect("failed to execute cli attest");

    assert!(output_attest.status.success(), "CLI stderr: {}", String::from_utf8_lossy(&output_attest.stderr));
    let stdout_attest = String::from_utf8(output_attest.stdout).unwrap();
    assert!(stdout_attest.contains("\"attestation_id\""));
    assert!(stdout_attest.contains("\"status_verdict\""));

    // Test stock-recovery with missing train to verify HIGH RISK status
    let temp_rec = tempfile::TempDir::new().unwrap();
    let output_rec = Command::new(env!("CARGO_BIN_EXE_mmi-studio-cli"))
        .current_dir(workspace_dir)
        .arg("stock-recovery")
        .arg("--baseline-train")
        .arg("UNKNOWN_TRAIN_TEST")
        .arg("--output")
        .arg(temp_rec.path())
        .arg("--json")
        .output()
        .expect("failed to execute cli stock-recovery");

    assert!(output_rec.status.success(), "CLI stderr: {}", String::from_utf8_lossy(&output_rec.stderr));
    let stdout_rec = String::from_utf8(output_rec.stdout).unwrap();
    assert!(stdout_rec.contains("HIGH RISK — NO VERIFIED RECOVERY PATH"));
}

#[test]
fn test_cli_plugins_list_inspect_and_verify() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let temp_plugins = tempfile::TempDir::new().unwrap();
    let plugin_dir = temp_plugins.path().join("mib2_custom_codec");
    std::fs::create_dir_all(&plugin_dir).unwrap();

    let manifest_json = r#"{
        "abi_version": 1,
        "plugin_id": "mmi.format.mib2.custom",
        "name": "MIB2 Custom Codec",
        "version": "0.5.0",
        "author": "Community",
        "description": "Parses custom MIB2 packages",
        "target_generation": "MIB2_HIGH",
        "supported_extensions": ["m2bin"],
        "capabilities": {
            "can_detect": true,
            "can_parse": true,
            "can_extract": false,
            "can_rebuild": false
        }
    }"#;
    std::fs::write(plugin_dir.join("plugin.json"), manifest_json).unwrap();

    // 1. Test plugins list
    let out_list = Command::new(env!("CARGO_BIN_EXE_mmi-studio-cli"))
        .current_dir(workspace_dir)
        .arg("plugins")
        .arg("list")
        .arg("--dir")
        .arg(temp_plugins.path())
        .arg("--json")
        .output()
        .expect("failed to execute cli plugins list");

    assert!(out_list.status.success(), "CLI stderr: {}", String::from_utf8_lossy(&out_list.stderr));
    let stdout_list = String::from_utf8(out_list.stdout).unwrap();
    assert!(stdout_list.contains("mmi.format.mib2.custom"));

    // 2. Test plugins inspect
    let out_inspect = Command::new(env!("CARGO_BIN_EXE_mmi-studio-cli"))
        .current_dir(workspace_dir)
        .arg("plugins")
        .arg("inspect")
        .arg(&plugin_dir)
        .arg("--json")
        .output()
        .expect("failed to execute cli plugins inspect");

    assert!(out_inspect.status.success(), "CLI stderr: {}", String::from_utf8_lossy(&out_inspect.stderr));
    let stdout_inspect = String::from_utf8(out_inspect.stdout).unwrap();
    assert!(stdout_inspect.contains("\"abi_version\": 1"));
    assert!(stdout_inspect.contains("\"MIB2_HIGH\""));

    // 3. Test plugins verify with sample test file
    let sample_file = temp_plugins.path().join("sample.m2bin");
    std::fs::write(&sample_file, b"test payload bytes").unwrap();

    let out_verify = Command::new(env!("CARGO_BIN_EXE_mmi-studio-cli"))
        .current_dir(workspace_dir)
        .arg("plugins")
        .arg("verify")
        .arg(&plugin_dir)
        .arg("--test-file")
        .arg(&sample_file)
        .arg("--json")
        .output()
        .expect("failed to execute cli plugins verify");

    assert!(out_verify.status.success(), "CLI stderr: {}", String::from_utf8_lossy(&out_verify.stderr));
    let stdout_verify = String::from_utf8(out_verify.stdout).unwrap();
    assert!(stdout_verify.contains("\"abi_compatible\": true"));
    assert!(stdout_verify.contains("\"detection_passed\": true"));
}

#[test]
fn test_cli_obd_dry_run_and_svm_resolution() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_mmi-studio-cli"))
        .current_dir(workspace_dir)
        .arg("obd")
        .arg("--dry-run")
        .arg("--solve-svm")
        .arg("--enable-gem")
        .arg("--json")
        .output()
        .expect("failed to execute cli obd");

    assert!(output.status.success(), "CLI stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("\"connected\": true"));
    assert!(stdout.contains("\"adaptation_channel\": 15"));
    assert!(stdout.contains("\"dtc_03276_cleared\": true"));
    assert!(stdout.contains("\"gem_unlocked\": true"));
}
