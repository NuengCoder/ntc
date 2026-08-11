// src/image2doc/color.rs

use image::{DynamicImage, GenericImageView, Rgba};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ColorAnalysis {
    pub dominant_colors: Vec<(Rgba<u8>, f32)>, // Color + percentage
    pub brightness: f32,                       // Average brightness (0-1)
    pub contrast: f32,                         // Contrast ratio
    pub is_dark: bool,                         // Dark vs light theme
    pub color_count: usize,                    // Unique colors
    pub gradient_ratio: f32,                   // Fraction of smooth gradient transitions
}

const NAMED_COLORS: &[(&str, u8, u8, u8)] = &[
    ("black", 0, 0, 0),
    ("white", 255, 255, 255),
    ("red", 255, 0, 0),
    ("lime", 0, 255, 0),
    ("blue", 0, 0, 255),
    ("yellow", 255, 255, 0),
    ("cyan", 0, 255, 255),
    ("magenta", 255, 0, 255),
    ("silver", 192, 192, 192),
    ("gray", 128, 128, 128),
    ("maroon", 128, 0, 0),
    ("olive", 128, 128, 0),
    ("green", 0, 128, 0),
    ("purple", 128, 0, 128),
    ("teal", 0, 128, 128),
    ("navy", 0, 0, 128),
    ("orange", 255, 165, 0),
    ("pink", 255, 192, 203),
    ("brown", 165, 42, 42),
    ("darkred", 139, 0, 0),
    ("darkgreen", 0, 100, 0),
    ("darkblue", 0, 0, 139),
    ("darkgray", 169, 169, 169),
    ("lightgray", 211, 211, 211),
    ("indigo", 75, 0, 130),
    ("violet", 238, 130, 238),
    ("gold", 255, 215, 0),
    ("coral", 255, 127, 80),
    ("tomato", 255, 99, 71),
    ("salmon", 250, 128, 114),
    ("skyblue", 135, 206, 235),
    ("steelblue", 70, 130, 180),
    ("slateblue", 106, 90, 205),
    ("mediumblue", 0, 0, 205),
    ("royalblue", 65, 105, 225),
    ("forestgreen", 34, 139, 34),
    ("seagreen", 46, 139, 87),
    ("limegreen", 50, 205, 50),
    ("yellowgreen", 154, 205, 50),
    ("darkorange", 255, 140, 0),
    ("orangered", 255, 69, 0),
    ("crimson", 220, 20, 60),
    ("firebrick", 178, 34, 34),
    ("saddlebrown", 139, 69, 19),
    ("chocolate", 210, 105, 30),
    ("peru", 205, 133, 63),
    ("tan", 210, 180, 140),
    ("wheat", 245, 222, 179),
    ("bisque", 255, 228, 196),
    ("blanchedalmond", 255, 235, 205),
    ("mintcream", 245, 255, 250),
    ("azure", 240, 255, 255),
    ("aliceblue", 240, 248, 255),
    ("lavender", 230, 230, 250),
    ("thistle", 216, 191, 216),
    ("plum", 221, 160, 221),
    ("orchid", 218, 112, 214),
    ("hotpink", 255, 105, 180),
    ("deeppink", 255, 20, 147),
    ("palevioletred", 219, 112, 147),
    ("mediumvioletred", 199, 21, 133),
    ("aquamarine", 127, 255, 212),
    ("palegreen", 152, 251, 152),
    ("lightgreen", 144, 238, 144),
    ("mediumspringgreen", 0, 250, 154),
    ("springgreen", 0, 255, 127),
    ("turquoise", 64, 224, 208),
    ("mediumturquoise", 72, 209, 204),
    ("darkturquoise", 0, 206, 209),
    ("paleturquoise", 175, 238, 238),
    ("lightcyan", 224, 255, 255),
    ("cornflowerblue", 100, 149, 237),
    ("mediumslateblue", 123, 104, 238),
    ("dodgerblue", 30, 144, 255),
    ("deepskyblue", 0, 191, 255),
    ("lightskyblue", 135, 206, 250),
    ("midnightblue", 25, 25, 112),
];

pub fn color_name(r: u8, g: u8, b: u8) -> &'static str {
    let mut best = "unknown";
    let mut best_dist = f32::MAX;
    for (name, nr, ng, nb) in NAMED_COLORS {
        let dr = *nr as f32 - r as f32;
        let dg = *ng as f32 - g as f32;
        let db = *nb as f32 - b as f32;
        let dist = dr * dr + dg * dg + db * db;
        if dist < best_dist {
            best_dist = dist;
            best = name;
        }
    }
    best
}

/// Detect gradient ratio — proportion of pixel transitions that are smooth (gradients)
/// rather than sharp (edges). High gradient ratio = modern UI, photos, illustrations.
pub fn detect_gradient_ratio(img: &DynamicImage) -> f32 {
    let (w, h) = img.dimensions();
    if w < 4 || h < 4 {
        return 0.0;
    }

    let step = (w / 50).clamp(1, 10);
    let mut smooth = 0u64;
    let mut total = 0u64;

    // Sample horizontal lines
    for y in (0..h).step_by(step as usize) {
        for x in 0..w.saturating_sub(3) {
            let p0 = img.get_pixel(x, y);
            let p1 = img.get_pixel(x + 1, y);
            let p2 = img.get_pixel(x + 2, y);
            let d0 = color_diff(&p0, &p1);
            let d1 = color_diff(&p1, &p2);
            // Smooth gradient: both differences are small but non-zero
            if d0 > 5 && d0 < 40 && d1 > 5 && d1 < 40 && (d0 as i16 - d1 as i16).abs() < 20 {
                smooth += 1;
            }
            total += 1;
        }
    }
    // Sample vertical lines
    for x in (0..w).step_by(step as usize) {
        for y in 0..h.saturating_sub(3) {
            let p0 = img.get_pixel(x, y);
            let p1 = img.get_pixel(x, y + 1);
            let p2 = img.get_pixel(x, y + 2);
            let d0 = color_diff(&p0, &p1);
            let d1 = color_diff(&p1, &p2);
            if d0 > 5 && d0 < 40 && d1 > 5 && d1 < 40 && (d0 as i16 - d1 as i16).abs() < 20 {
                smooth += 1;
            }
            total += 1;
        }
    }

    if total == 0 {
        0.0
    } else {
        smooth as f32 / total as f32
    }
}

fn color_diff(a: &Rgba<u8>, b: &Rgba<u8>) -> u16 {
    (a[0] as i16 - b[0] as i16).unsigned_abs()
        + (a[1] as i16 - b[1] as i16).unsigned_abs()
        + (a[2] as i16 - b[2] as i16).unsigned_abs()
}

/// Analyze image colors
pub fn analyze_colors(img: &DynamicImage, quantize: usize) -> ColorAnalysis {
    let mut color_counts: HashMap<u32, u32> = HashMap::new();
    let (width, height) = img.dimensions();
    let total_pixels = (width * height) as f32;

    // Single pass: collect color frequencies and calculate brightness simultaneously
    let mut total_brightness = 0.0;
    for pixel in img.pixels() {
        let rgba = pixel.2;
        // Quantize colors by reducing bit depth
        let quantized = quantize_color(rgba, quantize);
        *color_counts.entry(quantized).or_insert(0) += 1;
        // Accumulate brightness in the same pass
        let brightness = 0.299 * rgba[0] as f32 + 0.587 * rgba[1] as f32 + 0.114 * rgba[2] as f32;
        total_brightness += brightness;
    }

    let avg_brightness = total_brightness / total_pixels / 255.0;

    // Find dominant colors
    let mut sorted: Vec<(u32, u32)> = color_counts.into_iter().collect();
    sorted.sort_by_key(|b| std::cmp::Reverse(b.1));

    let dominant_colors: Vec<(Rgba<u8>, f32)> = sorted
        .iter()
        .take(5)
        .map(|(color, count)| {
            let rgba = dequantize_color(*color);
            let percentage = (*count as f32 / total_pixels) * 100.0;
            (rgba, percentage)
        })
        .collect();

    let contrast = calculate_contrast(&dominant_colors);

    ColorAnalysis {
        dominant_colors,
        brightness: avg_brightness,
        contrast,
        is_dark: avg_brightness < 0.5,
        color_count: sorted.len(),
        gradient_ratio: detect_gradient_ratio(img),
    }
}

/// Quantize color to reduce palette size (includes alpha channel)
fn quantize_color(rgba: Rgba<u8>, levels: usize) -> u32 {
    let step = 256 / levels;
    let r = (rgba[0] as usize / step) as u32;
    let g = (rgba[1] as usize / step) as u32;
    let b = (rgba[2] as usize / step) as u32;
    let a = (rgba[3] as usize / step) as u32;
    (r << 24) | (g << 16) | (b << 8) | a
}

fn dequantize_color(quantized: u32) -> Rgba<u8> {
    let r = ((quantized >> 24) & 0xFF) as u8;
    let g = ((quantized >> 16) & 0xFF) as u8;
    let b = ((quantized >> 8) & 0xFF) as u8;
    let a = (quantized & 0xFF) as u8;
    Rgba([r, g, b, a])
}

fn calculate_contrast(colors: &[(Rgba<u8>, f32)]) -> f32 {
    if colors.len() < 2 {
        return 1.0;
    }
    let lum1 = relative_luminance(colors[0].0);
    let lum2 = relative_luminance(colors[1].0);
    let lighter = lum1.max(lum2);
    let darker = lum1.min(lum2);
    (lighter + 0.05) / (darker + 0.05)
}

fn relative_luminance(rgba: Rgba<u8>) -> f32 {
    let r = srgba_to_linear(rgba[0] as f32 / 255.0);
    let g = srgba_to_linear(rgba[1] as f32 / 255.0);
    let b = srgba_to_linear(rgba[2] as f32 / 255.0);
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

fn srgba_to_linear(c: f32) -> f32 {
    if c <= 0.03928 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}
