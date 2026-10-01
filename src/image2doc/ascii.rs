use anyhow::Result;
use image::{DynamicImage, GenericImageView};

/// Clean ASCII character set from dark to bright
const ASCII_CHARS: &[char] = &[' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

/// Terminal cells are roughly twice as tall as they are wide: without
/// this compensation every image renders vertically stretched ~2x.
const CELL_ASPECT: f32 = 2.0;

/// 4x4 Bayer threshold matrix (row-major, 0..15) for ordered dithering
/// of the ramp index. Breaks gradient banding deterministically.
const BAYER4: [f32; 16] = [
    0., 8., 2., 10., 12., 4., 14., 6., 3., 11., 1., 9., 15., 7., 13., 5.,
];

/// Grid dims fitting an `ow × oh` image inside `max_width × max_lines`
/// cells with cell-aspect compensation. Never upscales, never empty,
/// never exceeds the bounds. Pure for testability.
pub fn fit_ascii_grid(
    ow: u32,
    oh: u32,
    max_width: usize,
    max_lines: Option<usize>,
) -> (usize, usize) {
    let vh = max_lines.unwrap_or(usize::MAX).max(1);
    let vw = max_width.max(1);
    if ow == 0 || oh == 0 {
        return (1.min(vw), 1.min(vh));
    }
    let scale = ((vw as f32) / ow as f32)
        .min((vh as f32 * CELL_ASPECT) / oh as f32)
        .min(1.0);
    let cols = ((ow as f32 * scale) as usize).clamp(1, vw);
    let rows = ((oh as f32 * scale / CELL_ASPECT) as usize).clamp(1, vh);
    (cols, rows)
}

/// Convert image to clean ASCII art.
///
/// The whole image always fits inside `max_width × max_lines` (no
/// cropping): it is area-resampled to an aspect-corrected grid and
/// mapped with ordered dithering. `max_lines: None` means unbounded
/// height. Output is plain text (no escapes) for any console.
pub fn image_to_ascii(
    img: &DynamicImage,
    max_width: usize,
    max_lines: Option<usize>,
) -> Result<String> {
    let (width, height) = img.dimensions();
    let (cols, rows) = fit_ascii_grid(width, height, max_width, max_lines);
    let small = img.resize_exact(cols as u32, rows as u32, image::imageops::FilterType::Lanczos3);

    let mut ascii = String::with_capacity(cols * rows + rows);
    for ty in 0..rows {
        for tx in 0..cols {
            let pixel = small.get_pixel(tx as u32, ty as u32);
            let lum = 0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32;
            // Bayer bias at quarter-level amplitude: breaks banding on
            // gradients, but solid 0/255 can never flip a full level.
            let bias = (BAYER4[(ty % 4) * 4 + (tx % 4)] / 16.0 - 0.5) * 0.5;
            let idx = ((lum / 255.0) * (ASCII_CHARS.len() - 1) as f32 + bias)
                .round()
                .clamp(0.0, (ASCII_CHARS.len() - 1) as f32) as usize;
            ascii.push(ASCII_CHARS[idx]);
        }
        if ty + 1 < rows {
            ascii.push('\n');
        }
    }

    Ok(ascii)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solid(w: u32, h: u32, v: u8) -> DynamicImage {
        DynamicImage::ImageRgb8(image::RgbImage::from_pixel(w, h, image::Rgb([v, v, v])))
    }

    fn vgradient(w: u32, h: u32) -> DynamicImage {
        let mut img = image::RgbImage::new(w, h);
        for y in 0..h {
            let v = (y as f32 * 255.0 / (h - 1).max(1) as f32) as u8;
            for x in 0..w {
                img.put_pixel(x, y, image::Rgb([v, v, v]));
            }
        }
        DynamicImage::ImageRgb8(img)
    }

    #[test]
    fn grid_keeps_cell_aspect_inside_bounds() {
        // 200x100 (2:1) at (100, 40): width-bound -> (100, 25).
        assert_eq!(fit_ascii_grid(200, 100, 100, Some(40)), (100, 25));
        // 100x200 (1:2) at (100, 40): height-bound -> (40, 40).
        assert_eq!(fit_ascii_grid(100, 200, 100, Some(40)), (40, 40));
        // Square at (100, 40): height-bound -> (80, 40).
        assert_eq!(fit_ascii_grid(100, 100, 100, Some(40)), (80, 40));
        // Never upscales, never empty, never exceeds bounds.
        assert_eq!(fit_ascii_grid(4, 4, 100, Some(40)), (4, 2));
        assert_eq!(fit_ascii_grid(0, 0, 100, Some(40)), (1, 1));
        assert_eq!(fit_ascii_grid(800, 600, 118, Some(24)), (64, 24));
    }

    #[test]
    fn gradient_spans_full_height_without_cropping() {
        // Vertical black->white gradient: first row dark, last row
        // bright — the old code cropped everything past max_lines.
        let art = image_to_ascii(&vgradient(60, 120), 60, Some(20)).unwrap();
        let rows: Vec<&str> = art.lines().collect();
        assert_eq!(rows.len(), 20);
        assert!(rows[0].chars().all(|c| c == ' ' || c == '.'));
        assert!(rows[19].chars().all(|c| c == '%' || c == '@'));
    }

    #[test]
    fn dither_smooths_but_solids_stay_uniform() {
        // Near-boundary tone (150 -> level ~5.29) dithers across the
        // two neighboring ramp chars instead of banding flat...
        let art = image_to_ascii(&solid(40, 20, 150), 40, Some(20)).unwrap();
        let distinct: std::collections::HashSet<char> = art.chars().filter(|c| *c != '\n').collect();
        assert_eq!(distinct, ['+'.into(), '*'.into()].into_iter().collect());
        // ...while solid black/white never flip a full level.
        let black = image_to_ascii(&solid(40, 20, 0), 40, Some(20)).unwrap();
        assert!(black.chars().all(|c| c == ' ' || c == '\n'));
        let white = image_to_ascii(&solid(40, 20, 255), 40, Some(20)).unwrap();
        assert!(white.chars().all(|c| c == '@' || c == '\n'));
        // A full sweep uses most of the ramp (no giant flat bands).
        let sweep = image_to_ascii(&vgradient(40, 200), 40, Some(60)).unwrap();
        let used: std::collections::HashSet<char> =
            sweep.chars().filter(|c| *c != '\n').collect();
        assert!(used.len() >= 7, "{:?}", used);
    }

    #[test]
    fn output_respects_bounds_and_is_deterministic() {
        let img = vgradient(300, 200);
        let a = image_to_ascii(&img, 100, Some(30)).unwrap();
        let b = image_to_ascii(&img, 100, Some(30)).unwrap();
        assert_eq!(a, b);
        let rows: Vec<&str> = a.lines().collect();
        assert!(rows.len() <= 30);
        assert!(rows.iter().all(|r| r.chars().count() <= 100));
        // No escape bytes ever (conhost-safe by construction).
        assert!(!a.contains('\x1b'));
    }
}
