use crate::navigator::Navigator;
use crate::output::{print_error, print_success, print_warning};
use crate::t;
use anyhow::{anyhow, Result};
use colored::*;
use std::io::Cursor;
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
                    img.put_pixel((x * scale + sx) as u32, (y * scale + sy) as u32, color);
                }
            }
        }
    }

    img.save(path)?;
    Ok(())
}

#[cfg(feature = "image2doc")]
fn encode_image_to_uri(mut img: image::DynamicImage) -> Result<String> {
    loop {
        let mut buf: Vec<u8> = Vec::new();
        img.write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png)
            .map_err(|e| anyhow!("{}", t!("cmd.qr.open_image_fail", e)))?;
        let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &buf);
        let uri = format!("data:image/png;base64,{}", b64);
        if qrcode::QrCode::with_error_correction_level(uri.as_bytes(), qrcode::EcLevel::L).is_ok() {
            return Ok(uri);
        }
        let (w, h) = image::GenericImageView::dimensions(&img);
        if w <= 16 || h <= 16 {
            return Err(anyhow!("{}", t!("cmd.qr.image_too_large")));
        }
        img = img.resize(w / 2, h / 2, image::imageops::FilterType::Triangle);
    }
}

fn print_usage() {
    println!("{}", t!("cmd.qr.usage_title").cyan().bold());
    println!("  {}", t!("cmd.qr.help_encode"));
    println!("  {}", t!("cmd.qr.help_same_as_above"));
    println!("  {}", t!("cmd.qr.help_save_png"));
    println!("  {}", t!("cmd.qr.help_encode_image"));
    println!("  {}", t!("cmd.qr.help_decode"));
    println!();
    println!("{}", t!("cmd.qr.examples_title").green());
    println!("  {}", t!("cmd.qr.example1"));
    println!("  {}", t!("cmd.qr.example2"));
    println!("  {}", t!("cmd.qr.example_image"));
    println!("  {}", t!("cmd.qr.example3"));
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
        print_error(&t!("cmd.qr.encode_usage"));
        return Ok(false);
    }

    let (text, out_file) = parse_out_flag(args);

    if text.is_empty() {
        print_error(&t!("cmd.qr.no_text"));
        return Ok(false);
    }

    #[cfg(feature = "image2doc")]
    {
        let candidate = cwd.join(text);
        if candidate.is_file() {
            if let Ok(img) = image::open(&candidate) {
                if out_file.is_none() {
                    print_error(&t!("cmd.qr.image_requires_out"));
                    return Ok(false);
                }
                let uri = encode_image_to_uri(img)?;
                let code =
                    qrcode::QrCode::with_error_correction_level(uri.as_bytes(), qrcode::EcLevel::L)
                        .map_err(|e| anyhow!("{}", t!("cmd.qr.gen_fail", e)))?;
                let pixels = qr_to_pixels(&code);
                let full = cwd.join(out_file.unwrap());
                save_qr_image(&pixels, &full)?;
                print_success(&t!("cmd.qr.saved", full.display()));
                return Ok(false);
            }
        }
    }

    let code = qrcode::QrCode::new(text.as_bytes())
        .map_err(|e| anyhow!("{}", t!("cmd.qr.gen_fail", e)))?;

    match out_file {
        Some(path) => {
            #[cfg(feature = "image2doc")]
            {
                let pixels = qr_to_pixels(&code);
                let full = cwd.join(path);
                save_qr_image(&pixels, &full)?;
                print_success(&t!("cmd.qr.saved", full.display()));
            }
            #[cfg(not(feature = "image2doc"))]
            {
                let _ = cwd;
                print_error(&t!("cmd.qr.image_save_requires"));
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
        print_error(&t!("cmd.qr.decode_usage"));
        return Ok(false);
    }

    let file_path = cwd.join(args.trim());
    if !file_path.exists() {
        print_error(&t!("cmd.qr.file_not_found", args));
        return Ok(false);
    }

    #[cfg(feature = "image2doc")]
    {
        let img =
            image::open(&file_path).map_err(|e| anyhow!("{}", t!("cmd.qr.open_image_fail", e)))?;
        let luma = img.to_luma8();
        let mut prepared = rqrr::PreparedImage::prepare(luma);
        let grids = prepared.detect_grids();

        if grids.is_empty() {
            print_warning(&t!("cmd.qr.no_qr_detected"));
            return Ok(false);
        }

        for (i, grid) in grids.iter().enumerate() {
            if grids.len() > 1 {
                println!(
                    "{} {}",
                    t!("cmd.qr.qr_label").cyan().bold(),
                    (i + 1).to_string().cyan().bold()
                );
            }
            match grid.decode() {
                Ok((_, content)) => println!("{}", content),
                Err(e) => print_warning(&t!("cmd.qr.decode_fail", (i + 1), e)),
            }
        }
    }

    #[cfg(not(feature = "image2doc"))]
    {
        let _ = file_path;
        print_error(&t!("cmd.qr.decode_requires"));
    }

    Ok(false)
}
