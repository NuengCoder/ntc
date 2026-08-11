// src/image2doc/converter.rs

use anyhow::{Context, Result};
use image::{DynamicImage, GenericImageView, ImageFormat};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ImageData {
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub file_size: u64,
    pub modified: String,
    pub pixels: Vec<u8>, // RGBA pixels
}

/// Load image and detect its format from the decoder
pub fn load_image(path: &Path) -> Result<(DynamicImage, ImageFormat)> {
    let reader = image::ImageReader::open(path)
        .with_context(|| format!("Failed to open image: {}", path.display()))?;
    let format = reader
        .format()
        .context("Unable to determine image format")?;
    let img = reader
        .decode()
        .with_context(|| format!("Failed to decode image: {}", path.display()))?;
    Ok((img, format))
}

/// Resize image while maintaining aspect ratio
pub fn resize_image(img: DynamicImage, max_width: usize) -> DynamicImage {
    let (width, height) = img.dimensions();
    if width == 0 || height == 0 {
        return img;
    }
    let max_width = max_width as u32;

    if width <= max_width {
        return img;
    }

    let new_width = max_width;
    let new_height = (height as f64 * (max_width as f64 / width as f64)) as u32;
    img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3)
}

/// Extract metadata from image
pub fn extract_metadata(path: &Path, img: &DynamicImage, format: ImageFormat) -> Result<Metadata> {
    let metadata = std::fs::metadata(path)?;
    let (width, height) = img.dimensions();

    let format_str = format.to_mime_type().to_string();

    let modified = metadata
        .modified()
        .ok()
        .map(|t| {
            let dt: chrono::DateTime<chrono::Local> = t.into();
            dt.format("%Y-%m-%d %H:%M:%S").to_string()
        })
        .unwrap_or_else(|| "Unknown".to_string());

    Ok(Metadata {
        path: path.to_string_lossy().to_string(),
        format: format_str,
        width,
        height,
        file_size: metadata.len(),
        modified,
        aspect_ratio: if height == 0 {
            1.0
        } else {
            width as f64 / height as f64
        },
    })
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub path: String,
    pub format: String,
    pub width: u32,
    pub height: u32,
    pub file_size: u64,
    pub modified: String,
    pub aspect_ratio: f64,
}
