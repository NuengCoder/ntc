use crate::navigator::Navigator;
use crate::output::{print_error, print_warning, raw_ansi_allowed};
use crate::t;

use anyhow::Result;
use colored::*;
use std::io::{self, Write};
use std::path::Path;

const IMAGE_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "bmp", "ico", "tiff", "tif", "webp",
];

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

/// Viewport (cells) for inline display: one screenful so the prompt
/// stays visible. Pure for testability.
fn display_viewport(term_w: u16, term_h: u16) -> (usize, usize) {
    (
        (term_w.saturating_sub(2)).max(20) as usize,
        (term_h.saturating_sub(6)).max(10) as usize,
    )
}

/// Cell dims preserving aspect for a `cols × rows` cell grid whose
/// cells pack `sx × sy` sub-pixels: `(cols, rows)` for an `ow × oh`
/// image fitted inside `(vw, vh)`. Pure for testability.
fn fit_cells(ow: u32, oh: u32, vw: usize, vh: usize, sx: u32, sy: u32) -> (usize, usize) {
    if ow == 0 || oh == 0 {
        return (vw.min(80), vh.min(40));
    }
    let scale = ((vw as f32 * sx as f32) / ow as f32).min((vh as f32 * sy as f32) / oh as f32);
    let cols = ((ow as f32 * scale / sx as f32) as usize).clamp(1, vw);
    let rows = ((oh as f32 * scale / sy as f32) as usize).clamp(1, vh);
    (cols, rows)
}

/// Paint one sextant grid row with raw 24-bit ANSI colors:
/// `\x1b[38;2;…m\x1b[48;2;…m<ch>\x1b[0m` per cell. Pure for testability.
fn paint_sextant_row(cells: &[crate::nfe::image::ImageCell]) -> String {
    let mut out = String::with_capacity(cells.len() * 24);
    for c in cells {
        out.push_str(&format!(
            "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m{}\x1b[0m",
            c.fg.0, c.fg.1, c.fg.2, c.bg.0, c.bg.1, c.bg.2, c.ch
        ));
    }
    out
}

/// Render `path` inline in the scrolling terminal.
///
/// - Kitty env → Kitty PNG chunks · Sixel env → Sixel DCS
/// - otherwise → truecolor ANSI sextants
///
/// A terminal without color/ANSI support cannot show an inline image, so
/// the command reports that instead of falling back to ASCII art.
///
/// Detection is env-table only (`detect_backend(..., None)`): the
/// interactive shell runs stdin cooked, where a DA1/cell-size
/// round-trip could block, so no terminal queries happen here.
fn display_image(path: &std::path::Path, force_sixel: bool) -> Result<()> {
    use crate::nfe::sixel::PixelBackend;

    let (term_w, term_h) = crossterm::terminal::size().unwrap_or((80, 24));
    let (vw, vh) = display_viewport(term_w, term_h);

    let (ow, oh) = image::image_dimensions(path)
        .map_err(|e| anyhow::anyhow!("Cannot read image '{}': {}", path.display(), e))?;
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "image".to_string());

    if !raw_ansi_allowed() {
        print_warning(&t!("cmd.image.no_ansi", name));
        return Ok(());
    }

    let (detected, _) = crate::nfe::sixel::detect_backend(&crate::nfe::sixel::snapshot_env(), None);
    if force_sixel && detected != PixelBackend::Sixel {
        let label = match detected {
            PixelBackend::Kitty => "kitty",
            PixelBackend::Sixel => "sixel",
            PixelBackend::Sextant => "sextants",
        };
        print_warning(&t!("cmd.image.sixel_forced_warn", label));
    }
    let backend = if force_sixel {
        PixelBackend::Sixel
    } else {
        detected
    };
    match backend {
        PixelBackend::Kitty => {
            println!(
                "{}",
                t!("cmd.image.displaying", name, ow, oh, "kitty").dimmed()
            );
            // PNG at viewport device pixels (conservative 8x16 cells:
            // no CSI 14 t query in cooked shell — sizing stays
            // aspect-correct regardless).
            let scale =
                ((vw as f32 * 8.0) / ow.max(1) as f32).min((vh as f32 * 16.0) / oh.max(1) as f32);
            let dw = ((ow as f32 * scale) as u32).clamp(1, vw as u32 * 8);
            let dh = ((oh as f32 * scale) as u32).clamp(1, vh as u32 * 16);
            let img = image::open(path)
                .map_err(|e| anyhow::anyhow!("Cannot decode '{}': {}", path.display(), e))?;
            let small = img.resize_exact(dw, dh, image::imageops::FilterType::Lanczos3);
            let mut png = Vec::new();
            small
                .write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)
                .map_err(|e| anyhow::anyhow!("{}", e))?;
            let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &png);
            let cols = dw.div_ceil(8) as usize;
            let rows = dh.div_ceil(16) as usize;
            for chunk in crate::nfe::kitty::transmit_chunks(&b64, cols, rows) {
                print!("{}", chunk);
            }
            println!();
            io::stdout().flush()?;
        }
        PixelBackend::Sixel => {
            println!(
                "{}",
                t!("cmd.image.displaying", name, ow, oh, "sixel").dimmed()
            );
            let scale =
                ((vw as f32 * 8.0) / ow.max(1) as f32).min((vh as f32 * 16.0) / oh.max(1) as f32);
            let dw = ((ow as f32 * scale) as u32).clamp(1, vw as u32 * 8);
            let dh = ((oh as f32 * scale) as u32).clamp(1, vh as u32 * 16);
            let img = image::open(path)
                .map_err(|e| anyhow::anyhow!("Cannot decode '{}': {}", path.display(), e))?;
            let small = img
                .resize_exact(dw, dh, image::imageops::FilterType::Lanczos3)
                .to_rgb8();
            let px: Vec<(u8, u8, u8)> = small.pixels().map(|p| (p[0], p[1], p[2])).collect();
            let pal = crate::nfe::sixel::median_cut(&px, 255);
            let idx = crate::nfe::sixel::map_to_palette(&px, &pal);
            let seq =
                crate::nfe::sixel::encode_sixel(&idx, dw as usize, dh as usize, &pal, (0, 0, 0));
            print!("{}", seq);
            println!();
            io::stdout().flush()?;
        }
        PixelBackend::Sextant => {
            println!(
                "{}",
                t!("cmd.image.displaying", name, ow, oh, "sextants").dimmed()
            );
            let (cols, rows) = fit_cells(ow, oh, vw, vh, 2, 3);
            let grid = crate::nfe::image::load_sextants(path, cols, rows)
                .map_err(|e| anyhow::anyhow!("Cannot preview '{}': {}", path.display(), e))?;
            for row in &grid.rows {
                println!("{}", paint_sextant_row(row));
            }
        }
    }
    Ok(())
}

/// Interactive number-picker for `image` with no path.
/// Returns the chosen (name, path).
fn prompt_pick_image(nav: &Navigator) -> Result<Option<(String, std::path::PathBuf)>> {
    let files = find_image_files(nav)?;

    if files.is_empty() {
        print_error(&t!("cmd.image.no_files"));
        return Ok(None);
    }

    println!();
    println!(
        "{}",
        "══════════════════════════════════════════════".cyan()
    );
    println!("{}", t!("cmd.image.select_image").cyan().bold());
    println!(
        "{}",
        "══════════════════════════════════════════════".cyan()
    );

    for (i, (name, path)) in files.iter().enumerate() {
        let size_str = get_file_size(path);
        println!(
            "  {}. {} {}",
            (i + 1).to_string().yellow(),
            name.blue(),
            size_str.dimmed()
        );
    }

    println!("{}", t!("cmd.image.desc_cancel", 0).red());
    println!();
    print!(
        "{} ",
        t!("cmd.image.enter_number", files.len()).green()
    );
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if input == "0" || input.is_empty() {
        println!("{}", t!("cmd.image.cancelled").dimmed());
        return Ok(None);
    }

    let idx = match input.parse::<usize>() {
        Ok(n) if n > 0 && n <= files.len() => n - 1,
        _ => {
            print_error(&format!("Invalid selection: {}", input));
            return Ok(None);
        }
    };

    Ok(Some(files[idx].clone()))
}

/// Split an `image display|show ...` tail into `(force_sixel, path)`.
/// `--sixel` may sit before or after the path; at most one path token.
/// No path → `None` (caller runs the interactive picker).
/// Pure for testability — `Err` carries the exact message to print.
fn parse_display_tail(tail: &[&str]) -> Result<(bool, Option<String>), String> {
    let mut force_sixel = false;
    let mut path: Option<String> = None;
    for tok in tail {
        if tok.eq_ignore_ascii_case("--sixel") {
            force_sixel = true;
        } else if tok.starts_with('-') {
            return Err(format!("Unknown option: {}", tok));
        } else if path.is_none() {
            path = Some(tok.to_string());
        } else {
            return Err(format!("Unknown option: {}", tok));
        }
    }
    Ok((force_sixel, path))
}

pub fn cmd_image(args: &str, nav: &mut Navigator) -> Result<bool> {
    // `image display|show [path]` and the bare `image [path]` both render
    // inline. With no path, pick interactively first.
    let head: Vec<&str> = args.trim().split_whitespace().collect();
    let tail = match head.first() {
        Some(first)
            if first.eq_ignore_ascii_case("display") || first.eq_ignore_ascii_case("show") =>
        {
            &head[1..]
        }
        _ => &head[..],
    };

    let (force_sixel, picked_path) = match parse_display_tail(tail) {
        Ok(v) => v,
        Err(msg) => {
            print_error(&msg);
            println!("{}", t!("cmd.image.usage"));
            return Ok(false);
        }
    };
    let path = if let Some(p) = picked_path {
        let expanded = crate::utils::paths::expand_path_str(&p);
        let p = Path::new(&expanded);
        if p.is_absolute() {
            p.to_path_buf()
        } else {
            nav.current_path().join(p)
        }
    } else if let Some((_, picked)) = prompt_pick_image(nav)? {
        picked
    } else {
        return Ok(false);
    };
    if !path.exists() {
        print_error(&format!("Image not found: {}", path.display()));
        return Ok(false);
    }
    if !is_image_file(&path) {
        print_error(&format!("Not a displayable image: {}", path.display()));
        return Ok(false);
    }
    display_image(&path, force_sixel)?;
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viewport_clamps_and_reserves_prompt() {
        assert_eq!(display_viewport(120, 30), (118, 24));
        assert_eq!(display_viewport(0, 0), (20, 10));
        assert_eq!(display_viewport(10, 5), (20, 10));
    }

    #[test]
    fn fit_cells_keeps_aspect_inside_viewport() {
        // Wide image in a wide viewport: fills width.
        let (c, r) = fit_cells(800, 200, 100, 40, 2, 3);
        assert_eq!((c, r), (100, 16));
        // Tall image: fills height instead.
        let (c, r) = fit_cells(100, 800, 100, 40, 2, 3);
        assert_eq!((c, r), (7, 40));
        // Never exceeds the viewport, never zero.
        let (c, r) = fit_cells(1, 1, 100, 40, 2, 3);
        assert!(c >= 1 && c <= 100 && r >= 1 && r <= 40);
        let (c, r) = fit_cells(0, 0, 100, 40, 2, 3);
        assert!(c >= 1 && c <= 100 && r >= 1 && r <= 40);
    }

    #[test]
    fn sextant_row_emits_truecolor_ansi() {
        let cells = vec![
            crate::nfe::image::ImageCell {
                ch: '▀',
                fg: (255, 0, 0),
                bg: (0, 0, 255),
            },
            crate::nfe::image::ImageCell {
                ch: ' ',
                fg: (1, 2, 3),
                bg: (1, 2, 3),
            },
        ];
        let s = paint_sextant_row(&cells);
        assert!(s.starts_with("\x1b[38;2;255;0;0m\x1b[48;2;0;0;255m▀\x1b[0m"));
        assert!(s.ends_with("\x1b[0m"));
        assert_eq!(s.matches("\x1b[0m").count(), 2);
    }

    #[test]
    fn display_tail_parses_sixel_in_any_position() {
        assert_eq!(
            parse_display_tail(&["a.png", "--sixel"]),
            Ok((true, Some("a.png".to_string())))
        );
        assert_eq!(
            parse_display_tail(&["--sixel", "a.png"]),
            Ok((true, Some("a.png".to_string())))
        );
        assert_eq!(parse_display_tail(&["--SIXEL"]), Ok((true, None)));
        assert_eq!(
            parse_display_tail(&["a.png"]),
            Ok((false, Some("a.png".to_string())))
        );
        assert_eq!(parse_display_tail(&[]), Ok((false, None)));
        // Two paths still error exactly like before.
        assert_eq!(
            parse_display_tail(&["a.png", "b.png"]),
            Err("Unknown option: b.png".to_string())
        );
        // Any other flag still errors.
        assert_eq!(
            parse_display_tail(&["a.png", "--ocr"]),
            Err("Unknown option: --ocr".to_string())
        );
    }

    #[test]
    fn shell_detection_is_env_only_no_probe() {
        // Mirrors the display_image contract: detect_backend with
        // da1=None (cooked stdin must never be queried).
        use std::collections::HashMap;
        let env: HashMap<String, String> = [("KITTY_WINDOW_ID".to_string(), "1".to_string())]
            .into_iter()
            .collect();
        assert_eq!(
            crate::nfe::sixel::detect_backend(&env, None).0,
            crate::nfe::sixel::PixelBackend::Kitty
        );
        let env: HashMap<String, String> = [("WT_SESSION".to_string(), "x".to_string())]
            .into_iter()
            .collect();
        assert_eq!(
            crate::nfe::sixel::detect_backend(&env, None).0,
            crate::nfe::sixel::PixelBackend::Sixel
        );
        let env: HashMap<String, String> = HashMap::new();
        assert_eq!(
            crate::nfe::sixel::detect_backend(&env, None).0,
            crate::nfe::sixel::PixelBackend::Sextant
        );
    }
}
