use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use svg_to_icons::{
    create_icns, create_ico, create_png_512, create_pngs, create_social_media_png,
    create_web_targets, svg_to_icon_data,
};

#[test]
fn main() -> std::io::Result<()> {
    let mut svg_data = String::new();
    File::open("icon.svg")?.read_to_string(&mut svg_data)?;

    let output_dir = PathBuf::from("icons");
    std::fs::create_dir_all(&output_dir)?;

    let icon_sizes = [
        (16, "is32"),
        (32, "il32"),
        (48, "ih32"),
        (64, "ih32"),
        (128, "it32"),
        (256, "ic08"),
        (512, "ic09"),
        (1024, "ic10"),
    ];

    let icon_entries = svg_to_icon_data(&svg_data, &icon_sizes)?;

    // Desktop icons
    create_icns(&icon_entries, &output_dir.join("icon.icns"))?;
    create_ico(&icon_entries, &output_dir.join("icon.ico"))?;
    create_pngs(&icon_entries, &icon_sizes, &output_dir)?;

    // 512×512 PNG
    create_png_512(&svg_data, &output_dir.join("icon-512.png"))?;

    // Web / mobile targets
    create_web_targets(&svg_data, &output_dir)?;

    // Social media banner is transparent by default, but one may choose a background color.
    create_social_media_png(
        &svg_data,
        &output_dir.join("og-image.png"),
        1200,
        630,
        None, // None = transparent
              // Some([51, 65, 85, 255])   // Example: #334155
    )?;

    println!("All icons generated successfully!");
    Ok(())
}
