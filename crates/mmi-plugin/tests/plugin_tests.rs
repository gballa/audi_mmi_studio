use mmi_plugin::*;
use std::time::Duration;
use tempfile::TempDir;

#[test]
fn test_plugin_manifest_abi_compatibility() {
    let manifest = PluginManifest {
        abi_version: ABI_VERSION,
        plugin_id: "mmi.format.mib2.ifs".to_string(),
        name: "MIB2 IFS Container Reader".to_string(),
        version: "0.1.0".to_string(),
        author: "Audi MMI Community".to_string(),
        description: "Decodes QNX IFS files for MIB2 platforms".to_string(),
        target_generation: "MIB2_HIGH".to_string(),
        supported_extensions: vec!["ifs".to_string(), "bin".to_string()],
        capabilities: PluginCapabilities::default(),
    };

    assert!(manifest.is_compatible());

    let old_manifest = PluginManifest {
        abi_version: 0,
        ..manifest.clone()
    };
    assert!(!old_manifest.is_compatible());

    let backend = Box::new(DeclarativePluginBackend::new(old_manifest));
    let err = PluginSandbox::new(backend);
    assert!(err.is_err());
}

#[test]
fn test_sandbox_payload_memory_limit() {
    let manifest = PluginManifest {
        abi_version: ABI_VERSION,
        plugin_id: "test.format".to_string(),
        name: "Test Reader".to_string(),
        version: "1.0.0".to_string(),
        author: "Dev".to_string(),
        description: "Test".to_string(),
        target_generation: "TEST".to_string(),
        supported_extensions: vec!["test".to_string()],
        capabilities: PluginCapabilities::default(),
    };

    let backend = Box::new(DeclarativePluginBackend::new(manifest));
    let limits = SandboxLimits {
        max_payload_bytes: 100, // Strict 100 bytes ceiling
        max_execution_time: Duration::from_millis(1000),
    };

    let sandbox = PluginSandbox::with_limits(backend, limits).unwrap();

    let valid_req = ParseRequest {
        file_name: "sample.test".to_string(),
        payload: vec![0x41; 50],
    };
    assert!(sandbox.parse(valid_req).is_ok());

    let oversize_req = ParseRequest {
        file_name: "sample.test".to_string(),
        payload: vec![0x41; 200],
    };
    let err = sandbox.parse(oversize_req);
    assert!(matches!(err, Err(SandboxError::MemoryExceeded { .. })));
}

#[test]
fn test_plugin_discovery_and_adapter_bridge() {
    let temp_dir = TempDir::new().unwrap();
    let plugin_dir = temp_dir.path().join("mib3_theme_reader");
    std::fs::create_dir_all(&plugin_dir).unwrap();

    let manifest_json = r#"{
        "abi_version": 1,
        "plugin_id": "mmi.format.mib3.theme",
        "name": "MIB3 Theme Container Reader",
        "version": "1.0.0",
        "author": "Audi Engineers",
        "description": "Reads MIB3 theme packages",
        "target_generation": "MIB3",
        "supported_extensions": ["m3theme", "pkg"],
        "capabilities": {
            "can_detect": true,
            "can_parse": true,
            "can_extract": true,
            "can_rebuild": false
        }
    }"#;

    std::fs::write(plugin_dir.join("plugin.json"), manifest_json).unwrap();

    let mut manager = PluginManager::new();
    let count = manager.discover_from_dir(temp_dir.path()).unwrap();
    assert_eq!(count, 1);

    let plugins = manager.list_plugins();
    assert_eq!(plugins.len(), 1);
    assert_eq!(plugins[0].plugin_id, "mmi.format.mib3.theme");

    let loaded = manager.get_plugin("mmi.format.mib3.theme").unwrap();
    let test_req = DetectRequest {
        file_name: "test.m3theme".to_string(),
        header_bytes: vec![0x00, 0x01],
        total_byte_size: 2,
    };
    let detect_res = loaded.sandbox.detect(test_req).unwrap();
    assert!(detect_res.matches);

    let bridge = PluginFormatAdapterBridge::new(loaded);

    use mmi_formats::FormatAdapter;
    assert_eq!(bridge.format_name(), "mmi.format.mib3.theme");
    assert!(bridge.capabilities().can_analyse);
    assert!(bridge.capabilities().can_extract);
    assert!(!bridge.capabilities().can_edit); // Verified untrusted edit protection (§18)
    assert_eq!(bridge.coverage_ratio(&[0x00, 0x01]), 0.0); // Unknown filename without extension yields 0 coverage
}
