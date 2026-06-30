// src/image2doc/analyzer.rs

use anyhow::Result;
use image::{DynamicImage, GenericImageView};
use crate::image2doc::color::ColorAnalysis;
use crate::image2doc::classify::{Classification, classify_image};
use crate::image2doc::layout;

#[derive(Debug, Clone)]
pub struct ImageAnalysis {
    pub color_analysis: ColorAnalysis,
    pub edge_density: f32,      // Percentage of edges detected
    pub text_density: f32,      // Likelihood of text regions
    pub symmetry: f32,          // Horizontal symmetry score
}

#[derive(Debug, Clone)]
pub struct LayoutElement {
    pub element_type: ElementType,
    pub bounds: (u32, u32, u32, u32),  // x, y, width, height
    pub confidence: f32,
    pub text: Option<String>,          // OCR text associated with this element
    pub font_size_pt: Option<u32>,     // Estimated font size in points
}


#[derive(Debug, Clone, PartialEq)]
pub enum ElementType {
    Button,
    TextField,
    Heading,
    Paragraph,
    Image,
    Logo,
    Separator,
}

/// Compute edge density once for sharing between classify and analyze
pub fn compute_edge_density(img: &DynamicImage) -> f32 {
    let (w, h) = img.dimensions();
    if w < 2 || h < 2 {
        return 0.0;
    }
    let mut edge_count = 0u64;
    let mut total = 0u64;
    let threshold: i16 = 40;

    for y in 0..h {
        for x in 0..w - 1 {
            let p1 = img.get_pixel(x, y);
            let p2 = img.get_pixel(x + 1, y);
            let diff = (p1[0] as i16 - p2[0] as i16).abs()
                + (p1[1] as i16 - p2[1] as i16).abs()
                + (p1[2] as i16 - p2[2] as i16).abs();
            if diff > threshold {
                edge_count += 1;
            }
            total += 1;
        }
    }
    // Also check vertical edges
    for y in 0..h - 1 {
        for x in 0..w {
            let p1 = img.get_pixel(x, y);
            let p2 = img.get_pixel(x, y + 1);
            let diff = (p1[0] as i16 - p2[0] as i16).abs()
                + (p1[1] as i16 - p2[1] as i16).abs()
                + (p1[2] as i16 - p2[2] as i16).abs();
            if diff > threshold {
                edge_count += 1;
            }
            total += 1;
        }
    }
    if total == 0 {
        return 0.0;
    }
    edge_count as f32 / total as f32
}

/// Compute text density once for sharing between classify and analyze
pub fn compute_text_density(img: &DynamicImage) -> f32 {
    let (w, h) = img.dimensions();
    if w < 4 || h < 4 {
        return 0.0;
    }
    
    let mut high_freq_rows = 0u32;
    let total_rows = h;
    
    for y in 0..h {
        let mut transitions = 0u32;
        let mut last_brightness = 0i16;
        
        for x in 0..w {
            let p = img.get_pixel(x, y);
            let brightness = (p[0] as i16 + p[1] as i16 + p[2] as i16) / 3;
            if x > 0 && (brightness - last_brightness).abs() > 30 {
                transitions += 1;
            }
            last_brightness = brightness;
        }
        
        if transitions > w / 4 {
            high_freq_rows += 1;
        }
    }
    
    high_freq_rows as f32 / total_rows as f32
}

/// Analyze image for layout elements
pub fn analyze_image(img: &DynamicImage, quantize: usize, edge_density: f32, text_density: f32) -> Result<ImageAnalysis> {
    let color_analysis = crate::image2doc::color::analyze_colors(img, quantize);
    let symmetry = calculate_symmetry(img);
    
    Ok(ImageAnalysis {
        color_analysis,
        edge_density,
        text_density,
        symmetry,
    })
}

/// Detect UI layout elements using real image processing
pub fn detect_layout(img: &DynamicImage) -> Result<Vec<LayoutElement>> {
    layout::detect_layout(img)
}

/// Classify the image type (screenshot, photo, document, etc.)
pub fn classify(img: &DynamicImage, edge_density: f32, text_density: f32) -> Classification {
    classify_image(img, edge_density, text_density)
}

/// Sort layout elements into reading order (top-to-bottom, left-to-right),
/// with z-order: overlapping elements are ordered by area (smallest on top).
pub fn sort_reading_order(elements: &mut [LayoutElement]) {
    // First pass: sort by y, then x (reading flow)
    elements.sort_by(|a, b| {
        let (ax, ay, ..) = a.bounds;
        let (bx, by, ..) = b.bounds;
        // Same row (within 20px tolerance): left-to-right
        if (ay as i32 - by as i32).abs() <= 20 {
            ax.cmp(&bx)
        } else {
            ay.cmp(&by)
        }
    });

    // Second pass: detect overlaps — smaller element on top (z-order)
    for i in 0..elements.len() {
        for j in i + 1..elements.len() {
            let (ax, ay, aw, ah) = elements[i].bounds;
            let (bx, by, bw, bh) = elements[j].bounds;
            let overlap_x = ax.max(bx) < (ax + aw).min(bx + bw);
            let overlap_y = ay.max(by) < (ay + ah).min(by + bh);
            if overlap_x && overlap_y {
                let area_a = aw * ah;
                let area_b = bw * bh;
                // Smaller area = on top (higher z-order), swap if needed
                if area_a < area_b && i > j {
                    elements.swap(i, j);
                }
            }
        }
    }
}

fn calculate_symmetry(img: &DynamicImage) -> f32 {
    let (width, height) = img.dimensions();
    if width < 2 || height < 1 {
        return 0.0;
    }
    let mut sim_sum = 0.0;
    let half_width = width / 2;
    let compared = half_width * height;
    if compared == 0 {
        return 0.0;
    }

    // Use perceptual color distance with gamma weighting
    for y in 0..height {
        for x in 0..half_width {
            let left = img.get_pixel(x, y);
            let right = img.get_pixel(width - 1 - x, y);

            // Weighted RGB distance — emphasizes luminance differences
            let dr = left[0] as f32 - right[0] as f32;
            let dg = left[1] as f32 - right[1] as f32;
            let db = left[2] as f32 - right[2] as f32;
            let dist = (0.299 * dr * dr + 0.587 * dg * dg + 0.114 * db * db).sqrt();

            // Score: 1.0 = identical, 0.0 = very different (threshold at ~ 60)
            let score = (1.0 - (dist / 60.0).min(1.0)).max(0.0);
            sim_sum += score;
        }
    }

    sim_sum / compared as f32
}