use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::output::{print_info, print_warning};
use crate::t;

/// OCR languages requested from Tesseract (pipe-separated for the CLI `-l` flag).
const OCR_LANGS: &str = "eng+ara+chi_sim+chi_tra+jpn+kor+spa+fra+deu+por+rus";

#[derive(Debug, Clone)]
pub struct TextWord {
    pub text: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub confidence: f32,
}

/// Locate the `tesseract` executable.
///
/// Search order:
/// 1. On PATH (`which::which`)
/// 2. Active ntpkg package bin dirs
/// 3. Common install locations (`C:\Program Files\Tesseract-OCR`, etc.)
/// 4. The self-contained libs cache dir (`~/.config/ntc/libs`)
fn find_tesseract() -> Option<PathBuf> {
    if let Ok(path) = which::which("tesseract") {
        return Some(path);
    }

    let exe_name = if cfg!(windows) { "tesseract.exe" } else { "tesseract" };

    for dir in crate::ntpkg::build_package_path_entries() {
        let candidate = dir.join(exe_name);
        if candidate.is_file() {
            return Some(candidate);
        }
    }

    #[cfg(windows)]
    {
        for base in [
            "C:\\Program Files\\Tesseract-OCR",
            "C:\\Program Files (x86)\\Tesseract-OCR",
        ] {
            let candidate = PathBuf::from(base).join(exe_name);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }

    if let Some(dir) = libs_cache_dir() {
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let candidate = path.join(exe_name);
                    if candidate.is_file() {
                        return Some(candidate);
                    }
                }
            }
        }
    }

    None
}

/// Directory holding self-contained ntc runtime libraries.
fn libs_cache_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("ntc").join("libs"))
}

/// Print localized guidance when tesseract is missing.
fn print_missing_guidance() {
    print_warning(&t!("image2doc.ocr_missing"));
    if crate::ntpkg::manager::is_termux() {
        print_info(&t!("ntpkg.termux_use_pkg"));
        print_info(&t!("ntpkg.termux_cmd", "tesseract"));
    } else {
        print_info(&t!("image2doc.ocr_install"));
    }
    #[cfg(windows)]
    {
        print_info(&t!("image2doc.ocr_path_win"));
    }
    #[cfg(not(windows))]
    {
        print_info(&t!("image2doc.ocr_path_unix"));
    }
    print_info(&t!("image2doc.ocr_rerun"));
}

fn tesseract_bin() -> Option<PathBuf> {
    let path = find_tesseract()?;
    Some(path)
}

/// Run `tesseract <img> stdout -l <langs>` and return trimmed stdout.
fn run_tesseract(image_path: &Path, extra: &[&str]) -> Result<Option<String>> {
    let exe = match tesseract_bin() {
        Some(exe) => exe,
        None => {
            print_missing_guidance();
            return Ok(None);
        }
    };

    let path_str = image_path
        .to_str()
        .with_context(|| format!("Invalid image path: {}", image_path.display()))?;

    let mut cmd = Command::new(&exe);
    cmd.arg(path_str).arg("stdout").arg("-l").arg(OCR_LANGS);
    cmd.args(extra);

    let output = cmd.output().with_context(|| "OCR command failed")?;
    if !output.status.success() {
        print_warning(&t!("image2doc.ocr_failed"));
        return Ok(None);
    }

    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if text.is_empty() {
        Ok(None)
    } else {
        Ok(Some(text))
    }
}

/// Extract raw text from an image using Tesseract OCR.
pub fn extract_text(image_path: &Path) -> Result<Option<String>> {
    run_tesseract(image_path, &[])
}

/// Extract words with bounding boxes from an image using Tesseract OCR.
///
/// Uses the TSV output mode whose columns are:
/// `level page_num block_num par_num line_num word_num left top width height conf text`.
pub fn extract_words(image_path: &Path) -> Result<Vec<TextWord>> {
    let raw = match run_tesseract(image_path, &["tsv"])? {
        Some(raw) => raw,
        None => return Ok(Vec::new()),
    };

    let mut words = Vec::new();
    for (idx, line) in raw.lines().enumerate() {
        if idx == 0 {
            continue; // header row
        }
        if let Some(word) = parse_tsv_line(line) {
            if word.confidence > 0.0 && !word.text.is_empty() {
                words.push(word);
            }
        }
    }

    Ok(words)
}

/// Parse a single TSV line (columns 6-10 are left/top/width/height/conf, 11 is text).
fn parse_tsv_line(line: &str) -> Option<TextWord> {
    let cols: Vec<&str> = line.split('\t').collect();
    if cols.len() < 12 {
        return None;
    }

    let parse = |i: usize| cols.get(i).and_then(|c| c.trim().parse::<i32>().ok());
    let conf = cols.get(10).and_then(|c| c.trim().parse::<f32>().ok())?;

    Some(TextWord {
        text: cols.get(11).unwrap_or(&"").trim().to_string(),
        x: parse(6)?,
        y: parse(7)?,
        width: parse(8)?,
        height: parse(9)?,
        confidence: conf,
    })
}

/// Check if a word bounding box overlaps with a layout element.
fn word_in_bounds(word: &TextWord, ex: u32, ey: u32, ew: u32, eh: u32) -> bool {
    let cx = word.x as f32 + word.width as f32 / 2.0;
    let cy = word.y as f32 + word.height as f32 / 2.0;
    cx >= ex as f32 && cx <= (ex + ew) as f32 && cy >= ey as f32 && cy <= (ey + eh) as f32
}

/// Match OCR words to layout elements, returning a map from element index to text.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_tsv_word_line() {
        let line =
            "5\t1\t1\t1\t1\t1\t120\t40\t180\t30\t92.5\tHello";
        let word = parse_tsv_line(line).expect("line should parse");
        assert_eq!(word.text, "Hello");
        assert_eq!(word.x, 120);
        assert_eq!(word.y, 40);
        assert_eq!(word.width, 180);
        assert_eq!(word.height, 30);
        assert_eq!(word.confidence, 92.5);
    }

    #[test]
    fn rejects_malformed_tsv_line() {
        assert!(parse_tsv_line("too few columns").is_none());
        assert!(parse_tsv_line("").is_none());
    }

    #[test]
    fn extracts_words_from_tsv() {
        let tsv = "level\tpage_num\tblock_num\tpar_num\tline_num\tword_num\tleft\ttop\twidth\theight\tconf\ttext\n\
                   5\t1\t1\t1\t1\t1\t0\t0\t100\t50\t95.5\tFoo\n\
                   5\t1\t1\t1\t1\t2\t100\t0\t100\t50\t90\tBar\n";
        // Exercise the raw parsing path via a small helper mirror of extract_words logic.
        let mut words = Vec::new();
        for (idx, line) in tsv.lines().enumerate() {
            if idx == 0 {
                continue;
            }
            if let Some(word) = parse_tsv_line(line) {
                if word.confidence > 0.0 && !word.text.is_empty() {
                    words.push(word);
                }
            }
        }
        assert_eq!(words.len(), 2);
        assert_eq!(words[0].text, "Foo");
        assert_eq!(words[1].text, "Bar");
    }

    #[test]
    fn matches_words_to_element_bounds() {
        let words = vec![
            TextWord {
                text: "Save".to_string(),
                x: 10,
                y: 10,
                width: 40,
                height: 20,
                confidence: 90.0,
            },
            TextWord {
                text: "Cancel".to_string(),
                x: 200,
                y: 200,
                width: 60,
                height: 20,
                confidence: 85.0,
            },
        ];
        let elements = [(0u32, 0u32, 100u32, 50u32)];
        let matched = match_words_to_elements(&words, &elements);
        assert_eq!(matched, vec!["Save".to_string()]);
    }
}
