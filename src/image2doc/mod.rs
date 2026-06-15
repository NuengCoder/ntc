// src/image2doc/mod.rs
// ntc v2.3.0 - Image to Document Converter
// Convert images to JSON, Markdown, HTML for AI consumption

use anyhow::Result;
use std::path::{Path, PathBuf};
use crate::output::{print_warning};

mod converter;
mod color;
mod ascii;
mod analyzer;
mod classify;
mod layout;
mod narrative;
mod formats;

#[cfg(feature = "ocr")]
mod ocr;

// Re-export public types
pub use converter::ImageData;

/// Supported output formats
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
    Json,           // Structured JSON with metadata + extracted data
    Markdown,       // Markdown with embedded ASCII art + extracted text
    Html,           // HTML with colored blocks + extracted content
    Txt,            // Plain text ASCII art
    Description,    // Natural-language description for AI consumption
}

impl std::str::FromStr for OutputFormat {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(Self::Json),
            "md" | "markdown" => Ok(Self::Markdown),
            "html" => Ok(Self::Html),
            "txt" | "text" => Ok(Self::Txt),
            "desc" | "description" => Ok(Self::Description),
            _ => Err(()),
        }
    }
}

impl OutputFormat {
    
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::Markdown => "md",
            Self::Html => "html",
            Self::Txt => "txt",
            Self::Description => "md",
        }
    }
}

/// Conversion options
#[derive(Debug, Clone)]
pub struct ConversionOptions {
    pub max_width: usize,           // Resize to max width (preserve aspect ratio)
    pub color_quantize: usize,      // Number of colors to quantize to (2-256)
    pub extract_text: bool,         // Use OCR to extract text (requires tesseract)
    pub ai_describe: bool,          // Use AI to describe image (requires API key)
    pub detect_layout: bool,        // Detect UI elements (buttons, forms, etc.)
    pub output_format: OutputFormat,
    pub output_path: Option<PathBuf>,
}

impl Default for ConversionOptions {
    fn default() -> Self {
        Self {
            max_width: 80,           // Fit in terminal width
            color_quantize: 16,      // 16 colors is good balance
            extract_text: false,
            ai_describe: false,
            detect_layout: false,
            output_format: OutputFormat::Markdown,
            output_path: None,
        }
    }
}

/// Main conversion function
pub fn convert_image(image_path: &Path, options: ConversionOptions) -> Result<String> {
    use analyzer::classify;
    
    // Load and decode image (with format detection)
    let (img, img_format) = converter::load_image(image_path)?;
    
    // Resize if needed
    let img = converter::resize_image(img, options.max_width);
    
    // Extract image metadata with actual format from decoder
    let metadata = converter::extract_metadata(image_path, &img, img_format)?;
    
    // Compute edge metrics once and share between classify and analyze
    let edge_density = analyzer::compute_edge_density(&img);
    let text_density = analyzer::compute_text_density(&img);
    
    // Classify image type (screenshot, photo, document, etc.)
    let classification = classify(&img, edge_density, text_density);
    
    // Analyze image (colors, brightness, etc.)
    let analysis = analyzer::analyze_image(&img, options.color_quantize, edge_density, text_density)?;
    
    // Extract text via OCR (if enabled)
    let extracted_text = if options.extract_text {
        #[cfg(feature = "ocr")]
        {
            ocr::extract_text(image_path)?
        }
        #[cfg(not(feature = "ocr"))]
        {
            print_warning("OCR not enabled. Recompile with 'ocr' feature or install tesseract.");
            None
        }
    } else {
        None
    };
    
    // Detect layout elements (buttons, forms, headings)
    let layout = if options.detect_layout {
        analyzer::detect_layout(&img)?
    } else {
        Vec::new()
    };
    
    // Link OCR words to layout elements by spatial matching
    #[cfg(feature = "ocr")]
    let layout = {
        if options.extract_text && !layout.is_empty() {
            let words = ocr::extract_words(image_path).unwrap_or_default();
            if !words.is_empty() {
                let bounds: Vec<_> = layout.iter().map(|e| e.bounds).collect();
                let texts = ocr::match_words_to_elements(&words, &bounds);
                layout.into_iter().enumerate().map(|(i, mut elem)| {
                    if i < texts.len() && !texts[i].is_empty() {
                        elem.text = Some(texts[i].clone());
                    }
                    elem
                }).collect()
            } else {
                layout
            }
        } else {
            layout
        }
    };
    
    // Sort layout into reading order
    let mut layout = layout;
    if !layout.is_empty() {
        analyzer::sort_reading_order(&mut layout);
    }

    // Generate ASCII art (skip for description format, limit for JSON)
    let ascii_art = if options.output_format == OutputFormat::Description {
        String::new()
    } else {
        let limit = if options.output_format == OutputFormat::Json { Some(20) } else { None };
        ascii::image_to_ascii(&img, options.max_width, limit)?
    };
    
    // Format output based on selected format
    let output = match options.output_format {
        OutputFormat::Json => formats::json::render(&metadata, &analysis, &extracted_text, &layout, &ascii_art, &classification),
        OutputFormat::Markdown => formats::md::render(&metadata, &analysis, &extracted_text, &layout, &ascii_art, &classification),
        OutputFormat::Html => formats::html::render(&metadata, &analysis, &extracted_text, &layout, &ascii_art, &classification),
        OutputFormat::Txt => formats::txt::render(&metadata, &analysis, &extracted_text, &layout, &ascii_art, &classification),
        OutputFormat::Description => narrative::generate_description(&metadata, &analysis, &extracted_text, &layout, &classification),
    };
    
    Ok(output)
}

/// Save conversion result to file
pub fn save_conversion(content: &str, output_path: &Path) -> Result<()> {
    std::fs::write(output_path, content)?;
    Ok(())
}