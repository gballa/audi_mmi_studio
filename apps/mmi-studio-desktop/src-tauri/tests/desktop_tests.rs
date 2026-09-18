use mmi_studio_desktop::*;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_desktop_ipc_inspect_precomp_and_thumbnail() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let precomp_path = "HN+R_EU_AU_K0942_4_[8R0906961FB]/CombiStyles/IND/0/default/arr_e.precomp";
    if !workspace_dir.join("originals").join(precomp_path).exists() {
        return;
    }

    let temp_cas = TempDir::new().unwrap();
    let res = handle_inspect_file(precomp_path, temp_cas.path()).unwrap();

    assert_eq!(res.detected_format, "Precomp Graphic (.precomp)");
    assert!(res.thumbnail_blob_id.is_some());
    assert!(!res.blake3_hash.is_empty());
}

#[test]
fn test_desktop_ipc_hexdump_and_entropy() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let meta_path = "HN+R_EU_AU_K0942_4_[8R0906961FB]/metainfo2.txt";
    if !workspace_dir.join("originals").join(meta_path).exists() {
        return;
    }

    let hex_res = handle_hexdump(meta_path, 0, 64).unwrap();
    assert_eq!(hex_res.offset, 0);
    assert_eq!(hex_res.length, 64);
    assert!(!hex_res.rows.is_empty());

    let entropy_res = handle_entropy(meta_path, 256).unwrap();
    assert!(entropy_res.average_entropy > 0.0);
    assert!(!entropy_res.segments.is_empty());
}

#[test]
fn test_desktop_ipc_render_screen_composition() {
    let temp_cas = TempDir::new().unwrap();
    let res_day = handle_render_screen("day", temp_cas.path()).unwrap();
    assert_eq!(res_day.width, 800);
    assert_eq!(res_day.height, 480);
    assert_eq!(res_day.active_mode, "Day");
    assert_eq!(res_day.pixel_count, 800 * 480);

    let res_night = handle_render_screen("night", temp_cas.path()).unwrap();
    assert_eq!(res_night.width, 800);
    assert_eq!(res_night.height, 480);
    assert_eq!(res_night.active_mode, "Night");
    assert_eq!(res_night.pixel_count, 800 * 480);
}
