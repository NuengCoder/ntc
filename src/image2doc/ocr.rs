use anyhow::{Context, Result};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct TextWord {
    pub text: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub confidence: f32,
}

/// Extract raw text from an image using Tesseract OCR
pub fn extract_text(image_path: &Path) -> Result<Option<String>> {
    let path_str = image_path
        .to_str()
        .with_context(|| format!("Invalid image path: {}", image_path.display()))?;

    let text = tesseract::ocr(path_str, "eng+ara+chi_sim+chi_tra+jpn+kor+spa+fra+deu+por+rus")
        .with_context(|| "OCR text extraction failed")?
        .trim()
        .to_string();

    if text.is_empty() {
        Ok(None)
    } else {
        Ok(Some(text))
    }
}

/// Extract words with bounding boxes from an image using Tesseract OCR
pub fn extract_words(image_path: &Path) -> Result<Vec<TextWord>> {
    let path_str = image_path
        .to_str()
        .with_context(|| format!("Invalid image path: {}", image_path.display()))?;

    let words = tesseract::ocr_box(path_str, "eng+ara+chi_sim+chi_tra+jpn+kor+spa+fra+deu+por+rus")
        .with_context(|| "OCR word extraction failed")?;

    let result: Vec<TextWord> = words
        .into_iter()
        .filter(|w| w.confidence > 0.0)
        .map(|w| TextWord {
            text: w.text,
            x: w.x,
            y: w.y,
            width: w.width,
            height: w.height,
            confidence: w.confidence,
        })
        .collect();

    if result.is_empty() {
        Ok(Vec::new())
    } else {
        Ok(result)
    }
}

/// Check if a word bounding box overlaps with a layout element
fn word_in_bounds(word: &TextWord, ex: u32, ey: u32, ew: u32, eh: u32) -> bool {
    let cx = word.x as f32 + word.width as f32 / 2.0;
    let cy = word.y as f32 + word.height as f32 / 2.0;
    cx >= ex as f32
        && cx <= (ex + ew) as f32
        && cy >= ey as f32
        && cy <= (ey + eh) as f32
}

/// Match OCR words to layout elements, returning a map from element index to text
pub fn match_words_to_elements(
    words: &[TextWord],
    elements: &[(u32, u32, u32, u32)],
) -> Vec<String> {
    let mut result: Vec<String> = elements.iter().map(|_| String::new()).collect();

    for word in words {
        for (i, &(ex, ey, ew, eh)) in elements.iter().enumerate() {
            if word_in_bounds(word, ex, ey, ew, eh) {
                if !result[i].is_empty() {
                    result[i].push(' ');
                }
                result[i].push_str(&word.text);
                break;
            }
        }
    }

    result
}
