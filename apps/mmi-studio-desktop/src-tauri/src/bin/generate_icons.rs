use image::{ImageBuffer, Rgba};
use std::fs;
use std::path::Path;
use std::process::Command;

fn render_icon(size: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut img = ImageBuffer::new(size, size);
    let center = size as f32 / 2.0;
    let radius = center * 0.92;

    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - center;
            let dy = y as f32 - center;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist > radius {
                // Transparent outer border with rounded squircle feel
                img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
                continue;
            }

            // Dial bezel outer metallic ring
            if dist >= radius - (center * 0.08) {
                let angle = dy.atan2(dx);
                let shade = ((angle.sin() * 0.5 + 0.5) * 40.0) as u8;
                img.put_pixel(x, y, Rgba([140 + shade, 150 + shade, 165 + shade, 255]));
                continue;
            }

            // Crimson illuminated inner ring
            if dist >= radius * 0.78 && dist <= radius * 0.88 {
                let glow = ((dist - radius * 0.78) / (radius * 0.10) - 0.5).abs() * 2.0;
                let red = 225 - (glow * 40.0) as u8;
                img.put_pixel(x, y, Rgba([red, 29, 72, 255]));
                continue;
            }

            // Dark knurled dial face
            if dist >= radius * 0.35 {
                let r_norm = dist / radius;
                let bg = (14.0 + (1.0 - r_norm) * 16.0) as u8;
                img.put_pixel(x, y, Rgba([bg, bg + 4, bg + 10, 255]));
                continue;
            }

            // Center Audi red compass / navigation arrow
            let nx = dx / (radius * 0.35);
            let ny = dy / (radius * 0.35);
            if ny < 0.6 && nx.abs() <= (0.6 - ny) * 0.8 {
                // Arrow head pointing up
                if nx < 0.0 {
                    img.put_pixel(x, y, Rgba([244, 63, 94, 255])); // Red needle
                } else {
                    img.put_pixel(x, y, Rgba([225, 29, 72, 255])); // Shadowed needle
                }
            } else if ny >= -0.2 && ny <= 0.8 && nx.abs() <= (ny + 0.2) * 0.5 {
                // Silver base pointing down
                if nx < 0.0 {
                    img.put_pixel(x, y, Rgba([241, 245, 249, 255]));
                } else {
                    img.put_pixel(x, y, Rgba([148, 163, 184, 255]));
                }
            } else {
                img.put_pixel(x, y, Rgba([8, 12, 20, 255]));
            }
        }
    }

    img
}

fn build_ico_file(images: &[(u8, Vec<u8>)]) -> Vec<u8> {
    let mut ico = Vec::new();
    // ICONDIR header
    ico.extend_from_slice(&0u16.to_le_bytes()); // Reserved
    ico.extend_from_slice(&1u16.to_le_bytes()); // Type 1 = ICO
    ico.extend_from_slice(&(images.len() as u16).to_le_bytes()); // Number of images

    let mut current_offset = 6 + (images.len() * 16);
    for (dim, png_bytes) in images {
        ico.push(*dim); // Width (0 for 256)
        ico.push(*dim); // Height
        ico.push(0);    // Color count
        ico.push(0);    // Reserved
        ico.extend_from_slice(&1u16.to_le_bytes());  // Planes
        ico.extend_from_slice(&32u16.to_le_bytes()); // Bit count
        ico.extend_from_slice(&(png_bytes.len() as u32).to_le_bytes());
        ico.extend_from_slice(&(current_offset as u32).to_le_bytes());
        current_offset += png_bytes.len();
    }

    for (_, png_bytes) in images {
        ico.extend_from_slice(png_bytes);
    }

    ico
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("icons");
    fs::create_dir_all(&base_dir)?;

    let iconset_dir = base_dir.join("icon.iconset");
    fs::create_dir_all(&iconset_dir)?;

    println!("Generating icon assets in {:?}...", base_dir);

    let sizes = [
        ("icon_16x16.png", 16),
        ("icon_16x16@2x.png", 32),
        ("icon_32x32.png", 32),
        ("icon_32x32@2x.png", 64),
        ("icon_128x128.png", 128),
        ("icon_128x128@2x.png", 256),
        ("icon_256x256.png", 256),
        ("icon_256x256@2x.png", 512),
        ("icon_512x512.png", 512),
        ("icon_512x512@2x.png", 1024),
    ];

    for (name, sz) in &sizes {
        let img = render_icon(*sz);
        let path = iconset_dir.join(name);
        img.save(&path)?;
    }

    // Save standard icons directly into base_dir
    let icon_32 = render_icon(32);
    icon_32.save(base_dir.join("32x32.png"))?;

    let icon_128 = render_icon(128);
    icon_128.save(base_dir.join("128x128.png"))?;

    let icon_256 = render_icon(256);
    icon_256.save(base_dir.join("128x128@2x.png"))?;
    icon_256.save(base_dir.join("256x256.png"))?;

    let icon_512 = render_icon(512);
    icon_512.save(base_dir.join("icon.png"))?;
    icon_512.save(base_dir.join("512x512.png"))?;

    // Build Windows ICO container
    let p32 = fs::read(base_dir.join("32x32.png"))?;
    let p128 = fs::read(base_dir.join("128x128.png"))?;
    let p256 = fs::read(base_dir.join("256x256.png"))?;
    let ico_bytes = build_ico_file(&[(32, p32), (128, p128), (0, p256)]);
    fs::write(base_dir.join("icon.ico"), ico_bytes)?;
    println!("✓ Generated Windows icon: {:?}", base_dir.join("icon.ico"));

    // Build macOS ICNS container via iconutil if available
    if Command::new("which").arg("iconutil").output()?.status.success() {
        let status = Command::new("iconutil")
            .args(["-c", "icns", "-o"])
            .arg(base_dir.join("icon.icns"))
            .arg(&iconset_dir)
            .status()?;
        if status.success() {
            println!("✓ Generated macOS icon: {:?}", base_dir.join("icon.icns"));
        } else {
            eprintln!("Warning: iconutil returned error code");
        }
    }

    println!("✓ All icon assets successfully generated!");
    Ok(())
}
