use mmi_core::{ContentAddressedStore, CoreError};
use mmi_recipe::{
    JournalChain, Recipe, RecipeEngine, RecipeMetadata, RecipeOperation, RebaseEngine,
    RebaseStatus, RiskClass, SemanticSelector,
};
use tempfile::TempDir;

#[test]
fn test_recipe_model_json_serialization_and_risk() {
    let recipe = Recipe {
        api_version: "mmi.recipe/v1".to_string(),
        metadata: RecipeMetadata {
            name: "Sport RS Theme".to_string(),
            version: "1.0.0".to_string(),
            author: "Audi MMI Studio Engineers".to_string(),
            description: Some("RS Red Theme with customized arrow and navigation strings".to_string()),
            base_train: "HN+R_EU_AU_K0942_4".to_string(),
            target_trains: vec!["HN+R_US_AU_K0942_5".to_string()],
            created_at: "2026-09-18T17:30:00Z".to_string(),
        },
        operations: vec![
            RecipeOperation::ReplaceAsset {
                selector: SemanticSelector::AssetPath("CombiStyles/IND/0/default/arr_e.precomp".to_string()),
                replacement_path: "replacements/custom_arr.png".to_string(),
                evidence_tag: Some("[EV:test:rs_arrow]".to_string()),
            },
            RecipeOperation::SetString {
                selector: SemanticSelector::StringKey {
                    catalog: "sss/tts_de_DE/0/default/plde-DE0.txt".to_string(),
                    key: "NAVI_PROMPT".to_string(),
                },
                new_value: "Sport Navigation Bereit".to_string(),
                evidence_tag: None,
            },
            RecipeOperation::SetConfig {
                selector: SemanticSelector::ConfigKey {
                    file: "metainfo2.txt".to_string(),
                    key: "ThemeVariant".to_string(),
                },
                new_value: "RS_SPORT".to_string(),
                evidence_tag: Some("[EV:test:rs_config]".to_string()),
            },
        ],
    };

    // Overall risk must be Structural due to SetConfig
    assert_eq!(recipe.overall_risk(), RiskClass::Structural);

    let json_text = recipe.to_json_pretty().unwrap();
    let deserialized: Recipe = Recipe::from_json(&json_text).unwrap();
    assert_eq!(deserialized.metadata.name, "Sport RS Theme");
    assert_eq!(deserialized.operations.len(), 3);
}

#[test]
fn test_recipe_engine_locks_signed_artefacts() {
    let temp_dir = TempDir::new().unwrap();
    let cas = ContentAddressedStore::new(temp_dir.path()).unwrap();
    let mut journal = JournalChain::default();

    let recipe_locked = Recipe {
        api_version: "mmi.recipe/v1".to_string(),
        metadata: RecipeMetadata {
            name: "Illegal Modification".to_string(),
            version: "0.1.0".to_string(),
            author: "Malicious Actor".to_string(),
            description: None,
            base_train: "HN+R_EU_AU_K0942_4".to_string(),
            target_trains: vec![],
            created_at: "2026-09-18T17:30:00Z".to_string(),
        },
        operations: vec![RecipeOperation::ReplaceAsset {
            selector: SemanticSelector::AssetPath("ifs-root.pkg".to_string()),
            replacement_path: "evil.pkg".to_string(),
            evidence_tag: None,
        }],
    };

    let result = RecipeEngine::apply(&recipe_locked, temp_dir.path(), &cas, &mut journal);
    assert!(result.is_err());
    match result.unwrap_err() {
        CoreError::SignedArtefactImmutable(msg) => {
            assert!(msg.contains("ERR_SIGNED_ARTEFACT_IMMUTABLE"));
        }
        other => panic!("Expected SignedArtefactImmutable error, got: {:?}", other),
    }
}

#[test]
fn test_journal_chain_cryptographic_verification() {
    let mut journal = JournalChain::default();

    let op1 = RecipeOperation::SetString {
        selector: SemanticSelector::StringKey {
            catalog: "strings.txt".to_string(),
            key: "TITLE".to_string(),
        },
        new_value: "Audi RS".to_string(),
        evidence_tag: None,
    };
    let op2 = RecipeOperation::RecolourPalette {
        selector: SemanticSelector::AssetPath("palette.dat".to_string()),
        day_accent_hex: Some("#FF0000".to_string()),
        night_accent_hex: Some("#880000".to_string()),
        evidence_tag: None,
    };

    journal.append(op1, Some("hash_before_1".to_string()), Some("hash_after_1".to_string()));
    journal.append(op2, Some("hash_before_2".to_string()), Some("hash_after_2".to_string()));

    assert_eq!(journal.entries.len(), 2);
    assert!(journal.verify_integrity().is_ok(), "Chain should be intact");

    // Tamper with sequence or hash
    journal.entries[1].prev_hash = "bad_tampered_hash".to_string();
    assert!(journal.verify_integrity().is_err(), "Chain tampering must be detected");
}

#[test]
fn test_rebase_engine_plan_across_trains() {
    let temp_train = TempDir::new().unwrap();
    let train_root = temp_train.path();

    // Setup dummy target train structure
    let nav_dir = train_root.join("GEMMI").join("nav").join("0").join("default");
    std::fs::create_dir_all(&nav_dir).unwrap();
    std::fs::write(nav_dir.join("models"), b"model_data").unwrap();

    // Create a drifted file located in different subfolder
    let drifted_dir = train_root.join("CustomDriftedDir");
    std::fs::create_dir_all(&drifted_dir).unwrap();
    std::fs::write(drifted_dir.join("target_icon.png"), b"icon").unwrap();

    let recipe = Recipe {
        api_version: "mmi.recipe/v1".to_string(),
        metadata: RecipeMetadata {
            name: "Cross Train Test".to_string(),
            version: "1.0.0".to_string(),
            author: "Tester".to_string(),
            description: None,
            base_train: "OLD_TRAIN".to_string(),
            target_trains: vec!["NEW_TRAIN".to_string()],
            created_at: "2026-09-18T17:30:00Z".to_string(),
        },
        operations: vec![
            // 1. Direct hit
            RecipeOperation::ReplaceAsset {
                selector: SemanticSelector::AssetPath("GEMMI/nav/0/default/models".to_string()),
                replacement_path: "new_models".to_string(),
                evidence_tag: None,
            },
            // 2. Drifted path
            RecipeOperation::ReplaceAsset {
                selector: SemanticSelector::AssetPath("OldOriginalDir/target_icon.png".to_string()),
                replacement_path: "icon.png".to_string(),
                evidence_tag: None,
            },
            // 3. Not found
            RecipeOperation::ReplaceAsset {
                selector: SemanticSelector::AssetPath("NonExistent/file.dat".to_string()),
                replacement_path: "file.dat".to_string(),
                evidence_tag: None,
            },
            // 4. Unsupported / locked signed payload
            RecipeOperation::ReplaceAsset {
                selector: SemanticSelector::AssetPath("payload.pkg".to_string()),
                replacement_path: "new.pkg".to_string(),
                evidence_tag: None,
            },
        ],
    };

    let report = RebaseEngine::plan_rebase(&recipe, train_root);
    assert_eq!(report.clean_count, 1);
    assert_eq!(report.drift_count, 1);
    assert_eq!(report.missing_count, 1);
    assert_eq!(report.unsupported_count, 1);

    assert_eq!(report.operations[0].status, RebaseStatus::Applied);
    assert_eq!(report.operations[1].status, RebaseStatus::AppliedWithDrift);
    assert_eq!(report.operations[2].status, RebaseStatus::SelectorNotFound);
    assert_eq!(report.operations[3].status, RebaseStatus::UnsupportedOnBase);
}

#[test]
fn test_prebuilt_theme_recipes_parse_and_validate() {
    use std::fs;
    use std::path::Path;

    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let recipes_dir = workspace_dir.join("recipes");

    let expected_recipes = [
        "audi_sport_amber.json",
        "rs_performance_red.json",
        "dark_line_minimalist.json",
    ];

    for recipe_file in &expected_recipes {
        let path = recipes_dir.join(recipe_file);
        assert!(path.exists(), "Recipe file {:?} must exist", path);
        let content = fs::read_to_string(&path).expect("Failed to read recipe file");
        let recipe = Recipe::from_json(&content).expect("Failed to parse recipe JSON");
        assert!(!recipe.metadata.name.is_empty());
        assert!(!recipe.operations.is_empty());
        assert_eq!(recipe.overall_risk(), RiskClass::Content);
    }
}
