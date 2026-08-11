// src/image2doc/classify.rs
// Image type classification — determines what kind of image we're looking at
// for better AI context (screenshot, UI mockup, photo, document, logo)

use image::{DynamicImage, GenericImageView};

/// The type of image detected
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImageType {
    Screenshot,
    UiMockup,
    Photo,
    Document,
    Logo,
}

impl ImageType {
    pub fn label(&self) -> &'static str {
        match self {
            ImageType::Screenshot => "Screenshot",
            ImageType::UiMockup => "UI Mockup",
            ImageType::Photo => "Photo",
            ImageType::Document => "Document",
            ImageType::Logo => "Logo",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ImageType::Screenshot => "A screenshot of a screen or application UI",
            ImageType::UiMockup => "A UI design mockup or wireframe with distinct regions",
            ImageType::Photo => "A natural photograph with continuous tones",
            ImageType::Document => "A scanned document or text-heavy page",
            ImageType::Logo => "A logo or icon with few colors and high contrast",
        }
    }
}

/// Classification result with confidence
#[derive(Debug, Clone)]
pub struct Classification {
    pub image_type: ImageType,
    pub confidence: f32,
}

/// Classify an image by analyzing its visual characteristics
/// Accepts pre-computed edge_density and text_density to avoid duplicate computation
pub fn classify_image(img: &DynamicImage, edge_density: f32, text_density: f32) -> Classification {
    let (width, height) = img.dimensions();
    let total_pixels = (width * height) as f32;

    // Compute remaining metrics (these are not shared with analyzer)
    let color_count = estimate_color_count(img, 32);
    let color_uniformity = measure_color_uniformity(img);
    let border_uniformity = measure_border_uniformity(img);
    let brightness_variance = measure_brightness_variance(img);

    // Use shared metrics
    let edge_ratio = edge_density;
    let text_region_density = text_density * (1.0 - color_uniformity).max(0.0);

    // Classify based on heuristics

    // Logo: small, few colors, high contrast, centered, uniform borders
    let logo_score = score_logo(
        color_count,
        color_uniformity,
        edge_ratio,
        border_uniformity,
        total_pixels,
    );

    // Screenshot: defined borders, high text density, sharp edges, uniform color regions
    let screenshot_score = score_screenshot(
        border_uniformity,
        text_region_density,
        edge_ratio,
        brightness_variance,
        color_count,
    );

    // Document: high contrast, uniform background, organized text lines
    let document_score = score_document(
        border_uniformity,
        color_count,
        edge_ratio,
        text_region_density,
        color_uniformity,
    );

    // UI Mockup: many rectangular regions, low entropy, defined color palette
    let ui_score = score_ui_mockup(
        color_count,
        edge_ratio,
        text_region_density,
        brightness_variance,
    );

    // Photo: high color count, natural tones, no defined borders, smooth gradients
    let photo_score = score_photo(
        color_count,
        brightness_variance,
        border_uniformity,
        edge_ratio,
    );

    // Normalize scores so each category can reach 1.0
    let max_scores = [0.9, 0.9, 0.9, 0.75, 0.75];
    let scores = [
        (ImageType::Logo, logo_score / max_scores[0]),
        (ImageType::Screenshot, screenshot_score / max_scores[1]),
        (ImageType::Document, document_score / max_scores[2]),
        (ImageType::UiMockup, ui_score / max_scores[3]),
        (ImageType::Photo, photo_score / max_scores[4]),
    ];

    let best = scores
        .iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap();
    let confidence = best.1.clamp(0.0, 1.0);

    Classification {
        image_type: best.0,
        confidence,
    }
}

fn estimate_color_count(img: &DynamicImage, quantize_levels: u32) -> u32 {
    let mut colors = std::collections::HashSet::new();
    let step = 256 / quantize_levels;
    for pixel in img.pixels() {
        let rgba = pixel.2;
        let q = (
            rgba[0] as u32 / step,
            rgba[1] as u32 / step,
            rgba[2] as u32 / step,
        );
        colors.insert(q);
        if colors.len() > 500 {
            break; // Cap for performance
        }
    }
    colors.len() as u32
}

fn measure_color_uniformity(img: &DynamicImage) -> f32 {
    // Measure how uniform colors are across the image
    let (w, h) = img.dimensions();
    if w < 4 || h < 4 {
        return 1.0;
    }
    let mut total_diff = 0.0;
    let mut samples = 0;
    // Sample grid of points
    for y in (0..h).step_by(8) {
        for x in (0..w).step_by(8) {
            let p1 = img.get_pixel(x, y);
            let p2 = img.get_pixel((x + 4).min(w - 1), y);
            let diff = (p1[0] as i16 - p2[0] as i16).abs() as f32
                + (p1[1] as i16 - p2[1] as i16).abs() as f32
                + (p1[2] as i16 - p2[2] as i16).abs() as f32;
            total_diff += diff;
            samples += 1;
        }
    }
    if samples == 0 {
        return 1.0;
    }
    let avg_diff = total_diff / samples as f32;
    // Normalize: 0 = uniform, 1 = varied
    (avg_diff / (255.0 * 3.0)).min(1.0)
}

fn measure_border_uniformity(img: &DynamicImage) -> f32 {
    // Measure how uniform the border pixels are
    // Screenshots have very uniform borders (toolbar, title bar)
    let (w, h) = img.dimensions();
    if w < 4 || h < 4 {
        return 1.0;
    }

    let mut border_pixels = Vec::new();
    let sample_limit = 200u32;
    // Top border
    for x in 0..w.min(sample_limit) {
        border_pixels.push(img.get_pixel(x, 0));
    }
    // Bottom border
    if h > 1 {
        for x in 0..w.min(sample_limit) {
            border_pixels.push(img.get_pixel(x, h - 1));
        }
    }
    // Left border
    for y in 0..h.min(sample_limit) {
        border_pixels.push(img.get_pixel(0, y));
    }
    // Right border
    if w > 1 {
        for y in 0..h.min(sample_limit) {
            border_pixels.push(img.get_pixel(w - 1, y));
        }
    }

    if border_pixels.is_empty() {
        return 1.0;
    }

    // Calculate variance
    let mut r_vals = Vec::new();
    let mut g_vals = Vec::new();
    let mut b_vals = Vec::new();
    for p in &border_pixels {
        r_vals.push(p[0] as f32);
        g_vals.push(p[1] as f32);
        b_vals.push(p[2] as f32);
    }

    let n = r_vals.len() as f32;
    let r_mean = r_vals.iter().sum::<f32>() / n;
    let g_mean = g_vals.iter().sum::<f32>() / n;
    let b_mean = b_vals.iter().sum::<f32>() / n;

    let r_variance = r_vals.iter().map(|v| (v - r_mean).powi(2)).sum::<f32>() / n;
    let g_variance = g_vals.iter().map(|v| (v - g_mean).powi(2)).sum::<f32>() / n;
    let b_variance = b_vals.iter().map(|v| (v - b_mean).powi(2)).sum::<f32>() / n;

    let total_variance = (r_variance + g_variance + b_variance) / (255.0_f32 * 255.0 * 3.0);
    // Invert: low variance = uniform = high score
    (1.0 - total_variance.min(1.0)).max(0.0)
}

fn measure_brightness_variance(img: &DynamicImage) -> f32 {
    let (w, h) = img.dimensions();
    if w < 2 || h < 2 {
        return 0.0;
    }
    let mut brightnesses = Vec::new();
    for y in (0..h).step_by(4) {
        for x in (0..w).step_by(4) {
            let p = img.get_pixel(x, y);
            let b = 0.299 * p[0] as f32 + 0.587 * p[1] as f32 + 0.114 * p[2] as f32;
            brightnesses.push(b);
        }
    }
    if brightnesses.is_empty() {
        return 0.0;
    }
    let mean = brightnesses.iter().sum::<f32>() / brightnesses.len() as f32;
    let variance =
        brightnesses.iter().map(|v| (v - mean).powi(2)).sum::<f32>() / brightnesses.len() as f32;
    (variance.sqrt() / 255.0).min(1.0)
}

// Scoring functions — each returns a confidence between 0 and 1

fn score_logo(
    color_count: u32,
    uniformity: f32,
    _edge_ratio: f32,
    border_uniformity: f32,
    total_pixels: f32,
) -> f32 {
    // _edge_ratio intentionally unused - logos have variable edge ratios
    let mut score = 0.0;
    // Logos have few colors
    if color_count <= 20 {
        score += 0.3;
    } else if color_count <= 50 {
        score += 0.15;
    }
    // Logos are often small
    if total_pixels < 50000.0 {
        score += 0.2;
    }
    // Uniform borders (logo on white/transparent background)
    if border_uniformity > 0.9 {
        score += 0.2;
    }
    // High color uniformity (solid color blocks)
    if uniformity < 0.3 {
        score += 0.2;
    }
    score
}

fn score_screenshot(
    border_uniformity: f32,
    text_density: f32,
    edge_ratio: f32,
    brightness_variance: f32,
    color_count: u32,
) -> f32 {
    let mut score = 0.0;
    // Screenshots have very uniform borders (title bar, taskbar)
    if border_uniformity > 0.85 {
        score += 0.25;
    } else if border_uniformity > 0.7 {
        score += 0.1;
    }
    // Screenshots have high text density
    if text_density > 0.3 {
        score += 0.2;
    }
    // Sharp edges from UI elements
    if edge_ratio > 0.15 && edge_ratio < 0.6 {
        score += 0.15;
    }
    // Moderate brightness variance (windows, content areas)
    if brightness_variance > 0.15 && brightness_variance < 0.6 {
        score += 0.15;
    }
    // Moderate color count (not too few like logo, not too many like photo)
    if color_count > 30 && color_count < 200 {
        score += 0.15;
    }
    score
}

fn score_document(
    border_uniformity: f32,
    color_count: u32,
    edge_ratio: f32,
    text_density: f32,
    uniformity: f32,
) -> f32 {
    let mut score = 0.0;
    // Documents have high border uniformity (white borders)
    if border_uniformity > 0.9 {
        score += 0.2;
    }
    // Few colors (black text on white)
    if color_count <= 40 {
        score += 0.25;
    } else if color_count <= 100 {
        score += 0.1;
    }
    // High edge ratio from text
    if edge_ratio > 0.3 && edge_ratio < 0.7 {
        score += 0.2;
    }
    // High text region density
    if text_density > 0.3 {
        score += 0.15;
    }
    // High uniformity (white background)
    if uniformity < 0.2 {
        score += 0.1;
    }
    score
}

fn score_ui_mockup(
    color_count: u32,
    edge_ratio: f32,
    text_density: f32,
    brightness_variance: f32,
) -> f32 {
    let mut score = 0.0;
    // UI mockups have defined color palette
    if color_count > 20 && color_count < 100 {
        score += 0.2;
    }
    // High edge ratio from component boundaries
    if edge_ratio > 0.2 && edge_ratio < 0.5 {
        score += 0.2;
    }
    // Some text but not dense
    if text_density > 0.15 && text_density < 0.4 {
        score += 0.2;
    }
    // Moderate brightness variance
    if brightness_variance > 0.2 && brightness_variance < 0.5 {
        score += 0.15;
    }
    score
}

fn score_photo(
    color_count: u32,
    brightness_variance: f32,
    border_uniformity: f32,
    edge_ratio: f32,
) -> f32 {
    let mut score = 0.0;
    // Photos have many colors
    if color_count > 150 {
        score += 0.25;
    } else if color_count > 80 {
        score += 0.15;
    }
    // High brightness variance (natural lighting)
    if brightness_variance > 0.3 {
        score += 0.2;
    }
    // Non-uniform borders (no clear frame)
    if border_uniformity < 0.6 {
        score += 0.15;
    }
    // Moderate edge ratio (not as sharp as text, not as smooth as gradients)
    if edge_ratio > 0.05 && edge_ratio < 0.3 {
        score += 0.15;
    }
    score
}
