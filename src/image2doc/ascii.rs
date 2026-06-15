use image::{DynamicImage, GenericImageView};
use anyhow::Result;

/// Clean ASCII character set from dark to bright
const ASCII_CHARS: &[char] = &[
    ' ', '.', ':', '-', '=', '+', '*', '#', '%', '@',
];

/// Convert image to clean ASCII art.
/// No dithering — just clean character mapping for readability.
/// max_lines: if Some, stop after that many lines.
pub fn image_to_ascii(img: &DynamicImage, max_width: usize, max_lines: Option<usize>) -> Result<String> {
    let (width, height) = img.dimensions();
    let target_width = max_width.min(width as usize);
    let target_height = (target_width as f32 / (width as f32 / height as f32)) as usize;
    let line_limit = max_lines.unwrap_or(target_height).min(target_height);

    let mut ascii = String::with_capacity(target_width * line_limit);

    for ty in 0..line_limit {
        for tx in 0..target_width {
            let src_y = (ty as f32 * height as f32 / target_height as f32) as u32;
            let src_x = (tx as f32 * width as f32 / target_width as f32) as u32;
            let pixel = img.get_pixel(src_x, src_y);

            let brightness = 0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32;
            let char_idx = ((brightness / 255.0) * (ASCII_CHARS.len() - 1) as f32) as usize;
            let char_idx = char_idx.min(ASCII_CHARS.len() - 1);
            ascii.push(ASCII_CHARS[char_idx]);
        }
        if ty + 1 < line_limit {
            ascii.push('\n');
        }
    }

    Ok(ascii)
}
