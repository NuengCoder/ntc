use crate::config::Config;
use crate::navigator::Navigator;
use crate::output::{print_error, print_success, print_info};

use anyhow::Result;
use colored::*;
use std::io::{self, Write};
use std::path::Path;

const IMAGE_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "gif", "bmp", "ico", "tiff", "tif", "webp"];

fn is_image_file(path: &Path) -> bool {
    path.is_file()
        && path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| IMAGE_EXTENSIONS.contains(&e.to_lowercase().as_str()))
            .unwrap_or(false)
}

fn find_image_files(nav: &Navigator) -> Result<Vec<(String, std::path::PathBuf)>> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(nav.current_path()) {
        for entry in entries.flatten() {
            let path = entry.path();
            if is_image_file(&path) {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    files.push((name.to_string(), path));
                }
            }
        }
    }
    files.sort_by_cached_key(|a| a.0.to_lowercase());
    Ok(files)
}

fn get_file_size(path: &std::path::Path) -> String {
    if let Ok(metadata) = std::fs::metadata(path) {
        let size = metadata.len();
        if size < 1024 {
            format!("({} B)", size)
        } else if size < 1024 * 1024 {
            format!("({:.1} KB)", size as f64 / 1024.0)
        } else {
            format!("({:.1} MB)", size as f64 / (1024.0 * 1024.0))
        }
    } else {
        String::new()
    }
}

fn open_image_default(path: &std::path::Path) -> Result<()> {
    #[cfg(windows)]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", &path.to_string_lossy()])
            .status()?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open").arg(path).status()?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open").arg(path).status()?;
    }
    #[cfg(target_os = "android")]
    {
        std::process::Command::new("termux-open").arg(path).status()?;
    }
    Ok(())
}

fn prompt_format() -> Result<(crate::image2doc::OutputFormat, bool)> {
    loop {
        println!();
        println!("{}", "Select output format:".cyan().bold());
        println!("  {}. Narrative description (for AI)", "1".yellow());
        println!("  {}. Markdown (default)", "2".yellow());
        println!("  {}. JSON", "3".yellow());
        println!("  {}. HTML", "4".yellow());
        println!("  {}. TXT", "5".yellow());
        println!("  {}. Cancel", "0".red());
        println!();
        print!("{} ", "format>".green().bold());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        let (fmt, save) = match input.as_str() {
            "1" | "desc" | "description" => (crate::image2doc::OutputFormat::Description, false),
            "2" | "md" | "markdown" | "" => (crate::image2doc::OutputFormat::Markdown, true),
            "3" | "json" => (crate::image2doc::OutputFormat::Json, false),
            "4" | "html" => (crate::image2doc::OutputFormat::Html, false),
            "5" | "txt" | "text" => (crate::image2doc::OutputFormat::Txt, false),
            "0" | "q" | "exit" => {
                println!("{}", "Cancelled.".dimmed());
                return Ok((crate::image2doc::OutputFormat::Markdown, false));
            }
            _ => {
                print_error(&format!("Unknown format: {}", input));
                continue;
            }
        };

        println!();
        print!("{} ", "Save to output path? (y/n, default: y): ".cyan());
        io::stdout().flush()?;

        let mut save_input = String::new();
        io::stdin().read_line(&mut save_input)?;
        let save_input = save_input.trim().to_lowercase();
        let save = save || save_input != "n";

        if !save {
            return Ok((fmt, false));
        }
        return Ok((fmt, true));
    }
}

fn prompt_bool(prompt: &str, default: bool) -> Result<bool> {
    let default_str = if default { "Y/n" } else { "y/N" };
    print!("{} ({}): ", prompt.cyan(), default_str.dimmed());
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();
    if input.is_empty() {
        return Ok(default);
    }
    Ok(input == "y" || input == "yes")
}

fn convert_and_output(
    image_path: &std::path::Path,
    output_format: crate::image2doc::OutputFormat,
    save_to_file: bool,
    ocr_enabled: bool,
    layout_enabled: bool,
) -> Result<()> {
    let ext = output_format.extension();
    let options = crate::image2doc::ConversionOptions {
        output_format,
        extract_text: ocr_enabled,
        detect_layout: layout_enabled,
        ..Default::default()
    };

    let result = crate::image2doc::convert_image(image_path, options)?;

    if save_to_file {
        let stem = image_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("image");
        let out_dir = Config::global_get_output_path();
        let output_path = out_dir.join(format!("{}_converted.{}", stem, ext));
        crate::image2doc::save_conversion(&result, &output_path)?;
        print_success(&format!(
            "Image converted and saved to: {}",
            output_path.display()
        ));
    } else {
        println!("{}", result);
    }

    Ok(())
}

pub fn cmd_image(args: &str, nav: &mut Navigator) -> Result<bool> {
    let trimmed = args.trim();

    if trimmed.is_empty() {
        let files = find_image_files(nav)?;

        if files.is_empty() {
            print_error("No image files found in current directory.");
            return Ok(false);
        }

        println!();
        println!("{}", "══════════════════════════════════════════════".cyan());
        println!("{}", "  Select an image to convert".cyan().bold());
        println!("{}", "══════════════════════════════════════════════".cyan());

        for (i, (name, path)) in files.iter().enumerate() {
            let size_str = get_file_size(path);
            println!("  {}. {} {}", (i + 1).to_string().yellow(), name.blue(), size_str.dimmed());
        }

        println!("  {}. Cancel", "0".red());
        println!();
        print!("{} ", format!("Enter number (1-{}) or 0: ", files.len()).green());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "0" || input.is_empty() {
            println!("{}", "Cancelled.".dimmed());
            return Ok(false);
        }

        let idx = match input.parse::<usize>() {
            Ok(n) if n > 0 && n <= files.len() => n - 1,
            _ => {
                print_error(&format!("Invalid selection: {}", input));
                return Ok(false);
            }
        };

        let (name, image_path) = &files[idx];

        let ocr = prompt_bool("Enable OCR?", false)?;
        let layout = prompt_bool("Enable layout detection?", false)?;
        let (fmt, save) = prompt_format()?;

        if !save {
            println!();
            print_info(&format!("Converting '{}'...", name));
            convert_and_output(image_path, fmt, false, ocr, layout)?;
        } else {
            print_info(&format!("Converting '{}'...", name));
            convert_and_output(image_path, fmt, true, ocr, layout)?;
        }

        return Ok(false);
    }

    let parts: Vec<&str> = args.split_whitespace().collect();

    let image_path = {
        let path_str = parts[0];
        let path = Path::new(path_str);
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            nav.current_path().join(path_str)
        }
    };

    if !image_path.exists() {
        print_error(&format!("Image not found: {}", image_path.display()));
        return Ok(false);
    }

    if !is_image_file(&image_path) {
        let _ = open_image_default(&image_path);
        return Ok(false);
    }

    let mut output_format = crate::image2doc::OutputFormat::Markdown;
    let mut save_to_file = true;
    let mut ocr_enabled = false;
    let mut layout_enabled = false;

    let mut i = 1;
    while i < parts.len() {
        match parts[i].to_lowercase().as_str() {
            "to" => {
                if i + 1 < parts.len() {
                    i += 1;
                    match parts[i].to_lowercase().as_str() {
                        "json" => output_format = crate::image2doc::OutputFormat::Json,
                        "md" | "markdown" => output_format = crate::image2doc::OutputFormat::Markdown,
                        "html" => output_format = crate::image2doc::OutputFormat::Html,
                        "txt" | "text" => output_format = crate::image2doc::OutputFormat::Txt,
                        "desc" | "description" => output_format = crate::image2doc::OutputFormat::Description,
                        f => {
                            print_error(&format!("Unknown format: {}", f));
                            return Ok(false);
                        }
                    }
                    save_to_file = false;
                } else {
                    print_error("Missing format after 'to'.");
                    return Ok(false);
                }
            }
            "out" => {
                if i + 1 < parts.len() {
                    i += 1;
                    match parts[i].to_lowercase().as_str() {
                        "json" => output_format = crate::image2doc::OutputFormat::Json,
                        "md" | "markdown" => output_format = crate::image2doc::OutputFormat::Markdown,
                        "html" => output_format = crate::image2doc::OutputFormat::Html,
                        "txt" | "text" => output_format = crate::image2doc::OutputFormat::Txt,
                        "desc" | "description" => output_format = crate::image2doc::OutputFormat::Description,
                        f => {
                            print_error(&format!("Unknown format: {}", f));
                            return Ok(false);
                        }
                    }
                    save_to_file = true;
                } else {
                    print_error("Missing format after 'out'.");
                    return Ok(false);
                }
            }
            "--ocr" => ocr_enabled = true,
            "--layout" => layout_enabled = true,
            _ => {
                print_error(&format!("Unknown option: {}", parts[i]));
                println!("Usage: image <path> [to|out <format>] [--ocr] [--layout]");
                return Ok(false);
            }
        }
        i += 1;
    }

    convert_and_output(&image_path, output_format, save_to_file, ocr_enabled, layout_enabled)?;

    Ok(false)
}