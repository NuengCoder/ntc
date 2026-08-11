// src/image2doc/layout.rs
// Real UI layout detection using edge gradients, connected-component analysis,
// and heuristic pattern matching for UI elements

use anyhow::Result;
use image::{DynamicImage, GenericImageView, GrayImage, Luma};

use crate::image2doc::analyzer::{ElementType, LayoutElement};

/// Detect layout elements in an image using edge analysis and region grouping
pub fn detect_layout(img: &DynamicImage) -> Result<Vec<LayoutElement>> {
    let (width, height) = img.dimensions();
    if width < 10 || height < 10 {
        return Ok(Vec::new());
    }

    let gray = img.to_luma8();
    let edges = compute_sobel_edges(&gray);
    let regions = find_connected_regions(&edges, width, height);
    let elements = classify_regions(&regions, width, height, img);

    Ok(elements)
}

/// Compute Sobel edge magnitude image
fn compute_sobel_edges(gray: &GrayImage) -> GrayImage {
    let (w, h) = gray.dimensions();
    let mut edges = GrayImage::new(w, h);

    // Sobel kernels
    let sobel_x: [[i16; 3]; 3] = [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];
    let sobel_y: [[i16; 3]; 3] = [[-1, -2, -1], [0, 0, 0], [1, 2, 1]];

    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let mut gx: i16 = 0;
            let mut gy: i16 = 0;

            for ky in 0..3 {
                for kx in 0..3 {
                    let px = gray.get_pixel(x + kx - 1, y + ky - 1)[0] as i16;
                    gx += px * sobel_x[ky as usize][kx as usize];
                    gy += px * sobel_y[ky as usize][kx as usize];
                }
            }

            let magnitude = ((gx as f32).hypot(gy as f32) as u16).min(255) as u8;
            edges.put_pixel(x, y, Luma([magnitude]));
        }
    }

    edges
}

/// A detected rectangular region in the image
#[derive(Debug, Clone)]
struct Region {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    edge_density: f32, // Average edge strength within region
    pixel_count: u32,  // Number of non-background pixels
}

/// Find connected regions of high edge activity
fn find_connected_regions(edges: &GrayImage, img_w: u32, img_h: u32) -> Vec<Region> {
    let (w, h) = edges.dimensions();
    let mut visited = vec![false; (w * h) as usize];
    let mut regions = Vec::new();
    let threshold: u8 = 60; // Edge threshold
    let min_region_size = 20u32; // Minimum pixels for a valid region

    for y in 0..h {
        for x in 0..w {
            let idx = (y * w + x) as usize;
            if visited[idx] || edges.get_pixel(x, y)[0] < threshold {
                continue;
            }

            // BFS to find connected edge pixels
            let mut stack = vec![(x, y)];
            visited[idx] = true;
            let mut min_x = x;
            let mut max_x = x;
            let mut min_y = y;
            let mut max_y = y;
            let mut pixel_count = 0u32;
            let mut edge_sum = 0u32;

            while let Some((cx, cy)) = stack.pop() {
                pixel_count += 1;
                edge_sum += edges.get_pixel(cx, cy)[0] as u32;
                min_x = min_x.min(cx);
                max_x = max_x.max(cx);
                min_y = min_y.min(cy);
                max_y = max_y.max(cy);

                // Check 4-connected neighbors
                for &(nx, ny) in &[
                    (cx.wrapping_sub(1), cy),
                    (cx + 1, cy),
                    (cx, cy.wrapping_sub(1)),
                    (cx, cy + 1),
                ] {
                    if nx < w && ny < h {
                        let nidx = (ny * w + nx) as usize;
                        if !visited[nidx] && edges.get_pixel(nx, ny)[0] >= threshold {
                            visited[nidx] = true;
                            stack.push((nx, ny));
                        }
                    }
                }
            }

            if pixel_count >= min_region_size {
                let rw = (max_x - min_x + 1).max(1);
                let rh = (max_y - min_y + 1).max(1);
                let edge_density = edge_sum as f32 / (pixel_count as f32 * 255.0);

                // Expand region by 2px for context
                let ex = (min_x as i32 - 2).max(0) as u32;
                let ey = (min_y as i32 - 2).max(0) as u32;
                let ew = (rw + 4).min(img_w - ex);
                let eh = (rh + 4).min(img_h - ey);

                regions.push(Region {
                    x: ex,
                    y: ey,
                    width: ew,
                    height: eh,
                    edge_density,
                    pixel_count,
                });
            }
        }
    }

    // Merge overlapping regions
    merge_regions(&mut regions);
    regions
}

/// Merge overlapping or adjacent regions, with a cap on max region size
fn merge_regions(regions: &mut Vec<Region>) {
    if regions.is_empty() {
        return;
    }
    // Estimate image dimensions from the largest region extent
    let max_extent_x = regions.iter().map(|r| r.x + r.width).max().unwrap_or(1);
    let max_extent_y = regions.iter().map(|r| r.y + r.height).max().unwrap_or(1);
    let total_area = (max_extent_x * max_extent_y) as f32;

    let mut changed = true;
    while changed {
        changed = false;
        let mut i = 0;
        while i < regions.len() {
            let mut j = i + 1;
            while j < regions.len() {
                let a = &regions[i];
                let b = &regions[j];

                // Check overlap: are they close or overlapping?
                let overlap_x = a.x.max(b.x) < (a.x + a.width).min(b.x + b.width) + 10;
                let overlap_y = a.y.max(b.y) < (a.y + a.height).min(b.y + b.height) + 10;

                if overlap_x && overlap_y {
                    // Compute merged bounds
                    let x1 = a.x.min(b.x);
                    let y1 = a.y.min(b.y);
                    let x2 = (a.x + a.width).max(b.x + b.width);
                    let y2 = (a.y + a.height).max(b.y + b.height);
                    let merged_area = ((x2 - x1) * (y2 - y1)) as f32;

                    // Don't merge if it would create a region > 60% of the image
                    if merged_area > total_area * 0.6 {
                        j += 1;
                        continue;
                    }

                    // Merge b into a
                    regions[i] = Region {
                        x: x1,
                        y: y1,
                        width: x2 - x1,
                        height: y2 - y1,
                        edge_density: (a.edge_density + b.edge_density) / 2.0,
                        pixel_count: a.pixel_count + b.pixel_count,
                    };
                    regions.remove(j);
                    changed = true;
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
    }

    // Sort by y position (top to bottom)
    regions.sort_by_key(|a| a.y);
}

/// Estimate font size in points for text-like elements by analyzing
/// connected component heights within the region.
fn estimate_font_size(region: &Region, img: &DynamicImage) -> Option<u32> {
    let step = 1u32.max(region.width / 20).min(5);
    let mut char_heights: Vec<u32> = Vec::new();

    // Scan columns looking for alternating dark-light patterns (text strokes)
    for x in (region.x..(region.x + region.width).min(img.width() - 1)).step_by(step as usize) {
        let mut in_dark = false;
        let mut run_start = 0u32;
        for y in region.y..(region.y + region.height).min(img.height() - 1) {
            let p = img.get_pixel(x, y);
            let lum = 0.299 * p[0] as f32 + 0.587 * p[1] as f32 + 0.114 * p[2] as f32;
            let is_dark = lum < 128.0;
            if is_dark && !in_dark {
                in_dark = true;
                run_start = y;
            } else if !is_dark && in_dark {
                in_dark = false;
                let height = y - run_start;
                if height >= 3 && height <= region.height / 2 {
                    char_heights.push(height);
                }
            }
        }
    }

    if char_heights.is_empty() {
        return None;
    }

    // Median height as estimated font size in pixels
    char_heights.sort();
    let median_px = char_heights[char_heights.len() / 2];
    // Convert pixels to points at ~96 DPI: 1pt = 1.33px
    let pt = (median_px as f32 / 1.33).round() as u32;
    Some(pt.clamp(6, 72))
}

/// Classify each detected region into a UI element type
fn classify_regions(
    regions: &[Region],
    img_w: u32,
    img_h: u32,
    img: &DynamicImage,
) -> Vec<LayoutElement> {
    let mut elements = Vec::new();

    for region in regions {
        let element_type = classify_single_region(region, img_w, img_h, img);
        let confidence = compute_confidence(&element_type, region, img_w, img_h);

        if confidence > 0.2 {
            let font_size = match element_type {
                ElementType::Heading
                | ElementType::Paragraph
                | ElementType::TextField
                | ElementType::Button => estimate_font_size(region, img),
                _ => None,
            };

            elements.push(LayoutElement {
                element_type,
                bounds: (region.x, region.y, region.width, region.height),
                confidence,
                text: None,
                font_size_pt: font_size,
            });
        }
    }

    elements
}

fn classify_single_region(
    region: &Region,
    img_w: u32,
    img_h: u32,
    img: &DynamicImage,
) -> ElementType {
    let rw = region.width as f32;
    let rh = region.height as f32;
    let aspect = rw / rh;
    let area_ratio = (rw * rh) / (img_w as f32 * img_h as f32);
    let is_wide = aspect > 3.0;
    let is_tall = aspect < 0.3;
    let is_square = aspect > 0.7 && aspect < 1.4;
    let is_top = region.y < img_h / 4;
    let is_bottom = region.y + region.height > img_h * 3 / 4;
    let is_center_x =
        region.x + region.width / 2 > img_w / 3 && region.x + region.width / 2 < img_w * 2 / 3;
    let rel_h = rh / img_h as f32; // relative height as fraction of image

    // Check color properties
    let color_variance = measure_region_color_variance(region, img);

    // Heading: wide rectangle at top, low color variance
    if is_wide && is_top && area_ratio < 0.3 && color_variance < 0.4 {
        return ElementType::Heading;
    }

    // Button: compact rectangle, often centered or at bottom
    if aspect > 1.5 && aspect < 5.0 && rel_h < 0.17 && (is_center_x || is_bottom) {
        return ElementType::Button;
    }

    // TextField: wide, not too tall, anywhere
    if is_wide && area_ratio < 0.2 && !is_top && color_variance < 0.5 {
        return ElementType::TextField;
    }

    // Separator: very thin and wide (relative to image height)
    if is_wide && rel_h < 0.015 {
        return ElementType::Separator;
    }

    // Image/Logo: square-ish, in center or corners
    if is_square && area_ratio < 0.3 && color_variance > 0.3 {
        if region.edge_density > 0.5 {
            return ElementType::Image;
        } else {
            return ElementType::Logo;
        }
    }

    // Paragraph: medium-to-large rectangular block, not too wide
    if !is_wide && !is_tall && area_ratio > 0.05 && area_ratio < 0.5 {
        return ElementType::Paragraph;
    }

    // Default: based on position
    if is_top {
        ElementType::Heading
    } else if is_bottom && aspect > 2.0 {
        ElementType::Button
    } else {
        ElementType::Paragraph
    }
}

fn compute_confidence(element_type: &ElementType, region: &Region, img_w: u32, img_h: u32) -> f32 {
    let rel_w = region.width as f32 / img_w.max(1) as f32;
    let rel_h = region.height as f32 / img_h.max(1) as f32;
    let rel_y = region.y as f32 / img_h.max(1) as f32;
    let rel_pixels = region.pixel_count as f32 / (img_w.max(1) * img_h.max(1)) as f32;

    match element_type {
        ElementType::Button => {
            let mut c: f32 = 0.5;
            if region.edge_density > 0.3 {
                c += 0.2;
            }
            if rel_pixels > 0.001 {
                c += 0.15;
            }
            c.min(0.95_f32)
        }
        ElementType::Heading => {
            let mut c: f32 = 0.5;
            if rel_y < 0.05 {
                c += 0.2;
            }
            if region.edge_density > 0.2 {
                c += 0.15;
            }
            c.min(0.95_f32)
        }
        ElementType::TextField => {
            let mut c: f32 = 0.4;
            if rel_w > 0.1 {
                c += 0.2;
            }
            if region.edge_density > 0.25 {
                c += 0.15;
            }
            c.min(0.9_f32)
        }
        ElementType::Paragraph => {
            let mut c: f32 = 0.4;
            if region.edge_density > 0.3 {
                c += 0.2;
            }
            if rel_pixels > 0.002 {
                c += 0.15;
            }
            c.min(0.85_f32)
        }
        ElementType::Image => {
            let mut c: f32 = 0.5;
            if region.edge_density > 0.4 {
                c += 0.2;
            }
            c.min(0.9_f32)
        }
        ElementType::Logo => {
            let mut c: f32 = 0.4;
            if region.edge_density < 0.3 {
                c += 0.2;
            }
            if rel_w < 0.15 && rel_h < 0.15 {
                c += 0.15;
            }
            c.min(0.85_f32)
        }
        ElementType::Separator => {
            let mut c: f32 = 0.6;
            if rel_h < 0.005 {
                c += 0.2;
            }
            c.min(0.95_f32)
        }
    }
}

/// Measure color variance within a region to help distinguish element types
fn measure_region_color_variance(region: &Region, img: &DynamicImage) -> f32 {
    let mut r_vals = Vec::new();
    let mut g_vals = Vec::new();
    let mut b_vals = Vec::new();
    let step = 3u32.max(region.width / 10).min(10);
    let y_start = region.y;
    let y_end = (region.y + region.height).min(img.height() - 1);
    let x_start = region.x;
    let x_end = (region.x + region.width).min(img.width() - 1);

    for y in (y_start..y_end).step_by(step as usize) {
        for x in (x_start..x_end).step_by(step as usize) {
            let p = img.get_pixel(x, y);
            r_vals.push(p[0] as f32);
            g_vals.push(p[1] as f32);
            b_vals.push(p[2] as f32);
        }
    }

    if r_vals.is_empty() {
        return 0.0;
    }

    let n = r_vals.len() as f32;
    let r_mean = r_vals.iter().sum::<f32>() / n;
    let g_mean = g_vals.iter().sum::<f32>() / n;
    let b_mean = b_vals.iter().sum::<f32>() / n;

    let r_var = r_vals.iter().map(|v| (v - r_mean).powi(2)).sum::<f32>() / n;
    let g_var = g_vals.iter().map(|v| (v - g_mean).powi(2)).sum::<f32>() / n;
    let b_var = b_vals.iter().map(|v| (v - b_mean).powi(2)).sum::<f32>() / n;

    let total_var = (r_var + g_var + b_var) / (255.0_f32 * 255.0 * 3.0);
    total_var.min(1.0)
}
