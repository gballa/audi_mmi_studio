use image::{ImageBuffer, Rgba};
use mmi_canvas::{CanvasRenderer, DisplayMode, LayoutProvenance, ScreenComposition, ScreenLayer};
use mmi_core::ContentAddressedStore;
use tempfile::tempdir;

#[test]
fn test_screen_composition_and_rendering() {
    let dir = tempdir().unwrap();
    let cas = ContentAddressedStore::new(dir.path()).unwrap();

    // 1. Create a synthetic layer icon (64x64 blue square)
    let icon_img: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_pixel(64, 64, Rgba([30, 144, 255, 255]));
    let mut icon_png = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut icon_png);
    icon_img
        .write_to(&mut cursor, image::ImageFormat::Png)
        .unwrap();

    let (icon_blob_id, _) = cas.put_bytes(&icon_png).unwrap();

    // 2. Build ScreenComposition
    let mut screen = ScreenComposition::new_main_display(
        "SCREEN-NAV-01",
        "Navigation Main View",
        LayoutProvenance::Derived {
            evidence_tag: "[EV:script:GEMMI/nav/main.lua#L42]".to_string(),
        },
    );

    screen.add_layer(ScreenLayer {
        layer_id: "layer_compass".to_string(),
        name: "Compass Icon".to_string(),
        x: 50,
        y: 50,
        width: 64,
        height: 64,
        z_index: 10,
        visible: true,
        asset_blob_id: Some(icon_blob_id),
        text_content: None,
    });

    assert_eq!(screen.target_width, 800);
    assert_eq!(screen.target_height, 480);
    assert_eq!(screen.layers.len(), 1);

    // 3. Render Day Mode
    let day_png = CanvasRenderer::render_to_png_bytes(&screen, DisplayMode::Day, &cas).unwrap();
    assert!(!day_png.is_empty());
    assert_eq!(&day_png[1..4], b"PNG");

    // 4. Render Night Mode
    let night_png =
        CanvasRenderer::render_to_png_bytes(&screen, DisplayMode::Night, &cas).unwrap();
    assert!(!night_png.is_empty());
    assert_ne!(day_png, night_png); // Colors modified in night palette
}
