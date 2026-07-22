use crate::navigator::Navigator;
use crate::output::{print_error, print_success, print_warning};
use anyhow::{anyhow, Result};
use colored::*;
use std::path::Path;

fn parse_out_flag(args: &str) -> (&str, Option<&str>) {
    let args = args.trim();
    if let Some(pos) = args.rfind(" -o ") {
        let target = args[..pos].trim();
        let out = args[pos + 4..].trim();
        if !out.is_empty() && !out.starts_with('-') {
            return (target, Some(out));
        }
    }
    if let Some(pos) = args.rfind(" --output ") {
        let target = args[..pos].trim();
        let out = args[pos + 10..].trim();
        if !out.is_empty() && !out.starts_with('-') {
            return (target, Some(out));
        }
    }
    (args, None)
}

fn qr_to_pixels(code: &qrcode::QrCode) -> Vec<Vec<bool>> {
    let size = code.width();
    let mut pixels = Vec::with_capacity(size);
    for y in 0..size {
        let mut row = Vec::with_capacity(size);
        for x in 0..size {
            row.push(code[(x, y)] == qrcode::Color::Dark);
        }
        pixels.push(row);
    }
    pixels
}

#[cfg(feature = "image2doc")]
fn save_qr_image(pixels: &[Vec<bool>], path: &Path) -> Result<()> {
    let size = pixels.len();
    let scale = 8;
    let img_size = (size * scale) as u32;
    let mut img = image::RgbImage::new(img_size, img_size);

    for (y, row) in pixels.iter().enumerate() {
        for (x, &dark) in row.iter().enumerate() {
            let color = if dark {
                image::Rgb([0, 0, 0])
            } else {
                image::Rgb([255, 255, 255])
            };
            for sy in 0..scale {
                for sx in 0..scale {
                    img.put_pixel(
                        (x * scale + sx) as u32,
                        (y * scale + sy) as u32,
                        color,
                    );
                }
            }
        }
    }

    img.save(path)?;
    Ok(())
}

fn print_usage() {
    println!("{}", "Usage:".cyan().bold());
    println!("  qr <text>                        Encode and display QR code in terminal");
    println!("  qr encode <text>                 Same as above");
    println!("  qr encode <text> -o <file>       Save QR as PNG image");
    println!("  qr decode <image_file>           Decode QR code from image");
    println!();
    println!("{}", "Examples:".green());
    println!("  qr \"https://example.com\"");
    println!("  qr encode \"hello world\" -o qr.png");
    println!("  qr decode qr.png");
}

pub fn cmd_qr(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let cwd = _nav.current_path().to_path_buf();
    let args = args.trim();

    if args.is_empty() {
        print_usage();
        return Ok(false);
    }

    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    let sub = parts.first().copied().unwrap_or("");
    let rest = parts.get(1).unwrap_or(&"").trim();

    match sub.to_lowercase().as_str() {
        "encode" | "en" => cmd_qr_encode(rest, &cwd),
        "decode" | "de" => cmd_qr_decode(rest, &cwd),
        _ => {
            // No subcommand → treat entire args as text to encode
            cmd_qr_encode(args, &cwd)
        }
    }
}

fn cmd_qr_encode(args: &str, cwd: &Path) -> Result<bool> {
    if args.is_empty() {
        print_error("Usage: qr encode <text> [-o <file>]");
        return Ok(false);
    }

    let (text, out_file) = parse_out_flag(args);

    if text.is_empty() {
        print_error("No text to encode");
        return Ok(false);
    }

    let code = qrcode::QrCode::new(text.as_bytes())
        .map_err(|e| anyhow!("Failed to generate QR code: {}", e))?;

    match out_file {
        Some(path) => {
            #[cfg(feature = "image2doc")]
            {
                let pixels = qr_to_pixels(&code);
                let full = cwd.join(path);
                save_qr_image(&pixels, &full)?;
                print_success(&format!("QR code saved to: {}", full.display()));
            }
            #[cfg(not(feature = "image2doc"))]
            {
                let _ = cwd;
                print_error("Image saving requires feature 'image2doc' (enabled by default)");
            }
        }
        None => {
            let qr_str = code
                .render::<qrcode::render::unicode::Dense1x2>()
                .module_dimensions(2, 1)
                .build();
            println!();
            println!("{}", qr_str);
        }
    }

    Ok(false)
}

fn cmd_qr_decode(args: &str, cwd: &Path) -> Result<bool> {
    if args.is_empty() {
        print_error("Usage: qr decode <image_file>");
        return Ok(false);
    }

    let file_path = cwd.join(args.trim());
    if !file_path.exists() {
        print_error(&format!("File not found: {}", args));
        return Ok(false);
    }

    #[cfg(feature = "image2doc")]
    {
        let img = image::open(&file_path)
            .map_err(|e| anyhow!("Failed to open image: {}", e))?;
        let luma = img.to_luma8();
        let mut prepared = rqrr::PreparedImage::prepare(luma);
        let grids = prepared.detect_grids();

        if grids.is_empty() {
            print_warning("No QR code detected in the image.");
            return Ok(false);
        }

        for (i, grid) in grids.iter().enumerate() {
            if grids.len() > 1 {
                println!("{} {}", "QR #".cyan().bold(), (i + 1).to_string().cyan().bold());
            }
            match grid.decode() {
                Ok((_, content)) => println!("{}", content),
                Err(e) => print_warning(&format!("Failed to decode QR #{}: {}", i + 1, e)),
            }
        }
    }

    #[cfg(not(feature = "image2doc"))]
    {
        let _ = file_path;
        print_error("QR decode requires feature 'image2doc' (enabled by default). Run with --features image2doc.");
    }

    Ok(false)
}
