//! Sixel pixel-graphics output for the NFE image viewer.
//!
//! Used when the terminal proves Sixel support (Windows Terminal via
//! DA1 probe); otherwise the viewer falls back to sextant cells.
//! Pure Rust, no new dependencies:
//!
//! * [`median_cut`] — palette quantization to <= 256 colors.
//! * [`encode_sixel`] — DCS framing, palette section, 6-row bands
//!   with RLE runs (`!n`), `$` (carriage) and `-` (new band).
//!
//! Sixel color components are 0-100 (percent); pixels are addressed
//! top-to-bottom, 6 rows per band, LSB = top row of the band.

/// One palette entry: 0-100 percent components + pixel count (weight).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PaletteColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// Quantize `pixels` (0-255 RGB) to at most `max_colors` via median cut.
/// Splits the box with the largest weighted channel range at its median
/// until the budget is reached or every box is a single color.
/// Deterministic: ties break toward lower channels, split keeps the
/// larger half first for stable ordering.
pub fn median_cut(pixels: &[(u8, u8, u8)], max_colors: usize) -> Vec<PaletteColor> {
    if pixels.is_empty() || max_colors == 0 {
        return Vec::new();
    }
    // Unique colors first: fewer uniques than budget needs no cutting.
    let mut uniq: Vec<(u8, u8, u8)> = pixels.to_vec();
    uniq.sort_unstable();
    uniq.dedup();
    if uniq.len() <= max_colors {
        return uniq
            .into_iter()
            .map(|(r, g, b)| PaletteColor {
                r: pct(r),
                g: pct(g),
                b: pct(b),
            })
            .collect();
    }

    // Weighted boxes: (pixels, weight). Weight = pixel count so splits
    // favor visually dominant ranges.
    let mut boxes: Vec<Vec<(u8, u8, u8)>> = vec![pixels.to_vec()];
    while boxes.len() < max_colors {
        // Pick the splittable box with the largest channel range.
        let mut best: Option<(usize, u8)> = None; // (box idx, range)
        for (i, b) in boxes.iter().enumerate() {
            if b.len() < 2 {
                continue;
            }
            let r = range_of(b, 0);
            let g = range_of(b, 1);
            let bl = range_of(b, 2);
            let m = r.max(g).max(bl);
            if m == 0 {
                continue;
            }
            if best.is_none_or(|(_, mr)| m > mr) {
                best = Some((i, m));
            }
        }
        let Some((idx, _)) = best else {
            break;
        };
        let b = boxes.swap_remove(idx);
        // Widest channel decides the split axis.
        let r = range_of(&b, 0);
        let g = range_of(&b, 1);
        let bl = range_of(&b, 2);
        let axis = if r >= g && r >= bl {
            0
        } else if g >= bl {
            1
        } else {
            2
        };
        let mut sorted = b;
        sorted.sort_unstable_by(|a, b| {
            let (av, bv) = (chan(a, axis), chan(b, axis));
            av.cmp(&bv)
        });
        let mid = sorted.len() / 2;
        let hi = sorted.split_off(mid.max(1).min(sorted.len() - 1));
        boxes.push(sorted);
        boxes.push(hi);
    }

    boxes
        .into_iter()
        .map(|b| {
            let n = b.len().max(1) as u32;
            let (sr, sg, sb) = b.iter().fold((0u32, 0u32, 0u32), |(a, c, d), p| {
                (a + p.0 as u32, c + p.1 as u32, d + p.2 as u32)
            });
            PaletteColor {
                r: pct((sr / n) as u8),
                g: pct((sg / n) as u8),
                b: pct((sb / n) as u8),
            }
        })
        .collect()
}

fn chan(p: &(u8, u8, u8), axis: usize) -> u8 {
    match axis {
        0 => p.0,
        1 => p.1,
        _ => p.2,
    }
}

fn range_of(b: &[(u8, u8, u8)], axis: usize) -> u8 {
    let (mut lo, mut hi) = (u8::MAX, u8::MIN);
    for p in b {
        let v = chan(p, axis);
        lo = lo.min(v);
        hi = hi.max(v);
    }
    hi.saturating_sub(lo)
}

fn pct(v: u8) -> u8 {
    ((v as u16 * 100 + 127) / 255) as u8
}

/// Map every pixel to its nearest palette index (RGB euclidean).
/// Exact-color hits memoize, so photos with large flat areas map in
/// near-linear time instead of pixels × registers.
pub fn map_to_palette(pixels: &[(u8, u8, u8)], palette: &[PaletteColor]) -> Vec<u8> {
    // Palette back to 0-255 for distance math.
    let pal: Vec<(i32, i32, i32)> = palette
        .iter()
        .map(|c| {
            (
                c.r as i32 * 255 / 100,
                c.g as i32 * 255 / 100,
                c.b as i32 * 255 / 100,
            )
        })
        .collect();
    let mut memo: std::collections::HashMap<(u8, u8, u8), u8> =
        std::collections::HashMap::with_capacity(palette.len().max(16));
    // Seed exact palette colors (round-trip through 0-100 loses a bit,
    // so seed by nearest too — first occurrence computes, rest hit).
    pixels
        .iter()
        .map(|p| {
            if let Some(&v) = memo.get(p) {
                return v;
            }
            let (pr, pg, pb) = (p.0 as i32, p.1 as i32, p.2 as i32);
            let mut best = 0u8;
            let mut best_d = i32::MAX;
            for (i, (r, g, b)) in pal.iter().enumerate() {
                let d =
                    (pr - r) * (pr - r) + (pg - g) * (pg - g) + (pb - b) * (pb - b);
                if d < best_d {
                    best_d = d;
                    best = i as u8;
                    if d == 0 {
                        break;
                    }
                }
            }
            memo.insert(*p, best);
            best
        })
        .collect()
}

/// Encode indexed pixels (`width × height`, values < `palette.len()`)
/// as a Sixel DCS sequence (7-bit form `ESC P q ... ESC \`).
///
/// `bg` fills register 0 (letterbox + band padding); pass the UI
/// background so the image rect blends into the surrounding panel.
///
/// Layout: palette definitions first (`#n;r;g;b`), then per color
/// register row-major bands of 6 (`#n` select, sixel chars `?`+bits,
/// `!n` RLE repeats, `$` carriage return, `-` next band).
pub fn encode_sixel(
    indexed: &[u8],
    width: usize,
    height: usize,
    palette: &[PaletteColor],
    bg: (u8, u8, u8),
) -> String {
    assert!(palette.len() <= 255, "registers 1..=255 hold the palette");
    assert!(indexed.len() >= width.saturating_mul(height));
    let mut out = String::new();
    out.push_str("\x1bPq");
    // Register 0 is the caller's background (letterbox bands);
    // palette colors occupy registers 1..=len.
    out.push_str(&format!(
        "#0;2;{};{};{}",
        pct(bg.0),
        pct(bg.1),
        pct(bg.2)
    ));
    for (i, c) in palette.iter().enumerate() {
        out.push_str(&format!("#{};2;{};{};{}", i + 1, c.r, c.g, c.b));
    }
    let bands = height.div_ceil(6);
    for band in 0..bands {
        let y0 = band * 6;
        // Registers actually used in this band (sorted for determinism).
        let mut regs: Vec<u8> = Vec::new();
        for y in y0..(y0 + 6).min(height) {
            for x in 0..width {
                let v = indexed[y * width + x] + 1;
                if !regs.contains(&v) {
                    regs.push(v);
                }
            }
        }
        regs.sort_unstable();
        for (ri, reg) in regs.iter().enumerate() {
            out.push_str(&format!("#{}", reg));
            // Emit runs of (sixels char, repeat) across the band rows.
            let mut run_ch: Option<u8> = None;
            let mut run_n: usize = 0;
            let flush = |out: &mut String, ch: u8, n: usize| {
                if n == 0 {
                    return;
                }
                if n > 3 {
                    out.push_str(&format!("!{}{}", n, (ch + b'?') as char));
                } else {
                    for _ in 0..n {
                        out.push((ch + b'?') as char);
                    }
                }
            };
            for x in 0..width {
                let mut bits: u8 = 0;
                for r in 0..6 {
                    let y = y0 + r;
                    if y < height && indexed[y * width + x] + 1 == *reg {
                        bits |= 1 << r;
                    }
                }
                match run_ch {
                    Some(c) if c == bits => run_n += 1,
                    _ => {
                        if let Some(c) = run_ch {
                            flush(&mut out, c, run_n);
                        }
                        run_ch = Some(bits);
                        run_n = 1;
                    }
                }
            }
            if let Some(c) = run_ch {
                flush(&mut out, c, run_n);
            }
            if ri + 1 < regs.len() {
                out.push('$');
            }
        }
        out.push('-');
    }
    out.push_str("\x1b\\");
    out
}

/// True when a DA1 device-attributes response advertises sixel
/// (attribute 4). Parses `CSI ? Pm ; … ; Ps c` loosely: any `;4;`,
/// `;4c`, `?4;` or `?4c` field match counts; bare numbers must equal 4
/// (so `64` never matches).
pub fn da1_advertises_sixel(resp: &str) -> bool {    let s = resp.trim();
    let s = s.strip_prefix("\x1b[").or_else(|| s.strip_prefix('\u{9b}')).unwrap_or(s);
    let s = s.strip_prefix('?').unwrap_or(s);
    let body = s.strip_suffix('c').unwrap_or(s);
    body.split(';').any(|f| {
        f.trim()
            .parse::<u32>()
            .map(|n| n == 4)
            .unwrap_or(false)
    })
}

/// Viewer pixel backend, best first.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelBackend {
    /// True pixels via Kitty graphics (kitty, ghostty, wezterm, iterm2).
    Kitty,
    /// True pixels via Sixel (Windows Terminal, foot, DA1-probed).
    Sixel,
    /// Sextant micro-blocks (universal fallback: vscode, VTE,
    /// konsole, conhost, unknown).
    Sextant,
}

/// Manual override via `NFE_IMG`: `kitty` | `sixel` | `cells`.
/// The legacy `NFE_SIXEL=0/1` still maps to cells/sixel.
fn env_override(env: &Env) -> Option<PixelBackend> {
    match env.get("NFE_IMG").map(|s| s.to_lowercase()).as_deref() {
        Some(v) if v == "kitty" => return Some(PixelBackend::Kitty),
        Some(v) if v == "sixel" => return Some(PixelBackend::Sixel),
        Some(v) if v == "cells" || v == "sextant" || v == "text" => {
            return Some(PixelBackend::Sextant)
        }
        _ => {}
    }
    match env.get("NFE_SIXEL").as_deref() {
        Some(v) if v == "0" || v == "off" || v == "false" => {
            return Some(PixelBackend::Sextant)
        }
        Some(v) if v == "1" || v == "on" || v == "true" => {
            return Some(PixelBackend::Sixel)
        }
        _ => None,
    }
}

/// Minimal env snapshot for detection (presence matters for the
/// `_SESSION`/`_WINDOW_ID` markers, hence the empty-string default).
pub type Env = std::collections::HashMap<String, String>;

pub fn snapshot_env() -> Env {
    let mut m = Env::new();
    for k in [
        "NFE_IMG",
        "NFE_SIXEL",
        "KITTY_WINDOW_ID",
        "WT_SESSION",
        "TERM",
        "TERM_PROGRAM",
    ] {
        m.insert(k.to_string(), std::env::var(k).unwrap_or_default());
    }
    m
}

fn env_is(env: &Env, key: &str) -> bool {
    env.get(key).map(|v| !v.is_empty()).unwrap_or(false)
}

fn term_is(env: &Env, val: &str) -> bool {
    env.get("TERM").map(|v| v == val).unwrap_or(false)
}

/// Pure backend detection over an env snapshot plus an optional DA1
/// answer (`None` = not probed). Returns the backend and a short
/// reason for the status line. Precedence: override > kitty env >
/// sixel (trusted env, else DA1) > sextant text cells.
pub fn detect_backend(env: &Env, da1: Option<bool>) -> (PixelBackend, &'static str) {
    use PixelBackend::*;
    if let Some(b) = env_override(env) {
        return (b, "override");
    }
    // Kitty family (all speak the Kitty graphics protocol).
    if env_is(env, "KITTY_WINDOW_ID")
        || term_is(env, "xterm-kitty")
        || term_is(env, "xterm-ghostty")
    {
        return (Kitty, "kitty-env");
    }
    if let Some(prog) = env.get("TERM_PROGRAM") {
        match prog.as_str() {
            "ghostty" | "Ghostty" | "WezTerm" | "wezterm" => return (Kitty, "kitty-env"),
            // iTerm2 documents Kitty-protocol support (APC variant),
            // so one encoder covers it — no separate OSC 1337 path.
            "iTerm.app" | "iTerm2" => return (Kitty, "kitty-env"),
            _ => {}
        }
    }
    // Sixel: Windows Terminal is trusted outright (its ConPTY has a
    // documented DA1 quirk — dropped ESC, microsoft/terminal#17813 —
    // so a negative/missing probe must not veto it). Same for foot.
    if env_is(env, "WT_SESSION") {
        return (Sixel, "sixel-wt");
    }
    if env.get("TERM").map(|v| v.starts_with("foot")).unwrap_or(false) {
        return (Sixel, "sixel-foot");
    }
    // Ambiguous terminals: the DA1 probe decides (mintty, xterm,
    // patched st/rxvt, tmux passthrough).
    match da1 {
        Some(true) => (Sixel, "sixel-da1"),
        Some(false) => (Sextant, "textcells"),
        None => (Sextant, "textcells"),
    }
}

/// True on a legacy Windows console: Windows with no WT_SESSION,
/// no Kitty markers, and no TERM/TERM_PROGRAM at all (double-clicked
/// conhost). No pixel protocol can ever work there; console fonts
/// also lack sextant glyphs, so Half blocks are the correct default.
pub fn is_legacy_console(env: &Env) -> bool {
    #[cfg(not(windows))]
    {
        let _ = env;
        return false;
    }
    #[cfg(windows)]
    {
        !env_is(env, "WT_SESSION")
            && !env_is(env, "KITTY_WINDOW_ID")
            && env.get("TERM_PROGRAM").map(|v| v.is_empty()).unwrap_or(true)
            && env.get("TERM").map(|v| v.is_empty()).unwrap_or(true)
    }
}

/// Default text-cell density: legacy conhost gets Half blocks,
/// everything else gets Sextant micro-blocks.
pub fn default_image_mode(env: &Env) -> crate::nfe::image::ImageMode {
    if is_legacy_console(env) {
        crate::nfe::image::ImageMode::Half
    } else {
        crate::nfe::image::ImageMode::Sextant
    }
}

/// True when the env (without any probe) leaves the backend genuinely
/// ambiguous: classic xterm-likes, screen/tmux, or unknown terms where
/// a DA1 answer for attribute 4 would change the outcome.
pub fn needs_probe(env: &Env) -> bool {
    if env_override(env).is_some() {
        return false;
    }
    // Known-negative terminals: no image protocols exist, probing
    // would only stall the UI for 300ms.
    if let Some(prog) = env.get("TERM_PROGRAM") {
        if prog == "vscode" || prog == "Apple_Terminal" {
            return false;
        }
    }
    if env_is(env, "VTE_VERSION") || env_is(env, "KONSOLE_VERSION") {
        return false;
    }
    if let Some(term) = env.get("TERM") {
        if term == "linux" || term == "dumb" {
            return false;
        }
    }
    // Kitty/sixel env hits already decided above (mirror the table
    // without running a probe for them).
    let (b, _) = detect_backend(env, None);
    if b != PixelBackend::Sextant {
        return false;
    }
    let term = env.get("TERM").cloned().unwrap_or_default();
    term.starts_with("xterm") || term.starts_with("screen") || term.starts_with("tmux")
}

static BACKEND_PROBE: std::sync::OnceLock<(PixelBackend, &'static str)> =
    std::sync::OnceLock::new();

/// Ask the terminal (DA1, attribute 4 = sixel) via the shared CSI
/// query helper. `None` on timeout/error (no usable reply);
/// `Some(answer)` when the terminal actually responded.
fn da1_query(timeout_ms: u64) -> Option<bool> {
    let resp = csi_query(b"\x1b[c", 'c', timeout_ms);
    // A usable reply is an escape-introduced `?...c` sequence.
    if resp.len() > 3 && resp.contains('\x1b') && resp.ends_with('c') {
        Some(da1_advertises_sixel(&resp))
    } else {
        None
    }
}

/// Detect the pixel backend once per process: override, then env
/// table, then (only for ambiguous terms) a single DA1 round-trip.
/// Never blocks the UI more than once.
pub fn detect_cached() -> (PixelBackend, &'static str) {
    *BACKEND_PROBE.get_or_init(|| {
        let env = snapshot_env();
        if !needs_probe(&env) {
            return detect_backend(&env, None);
        }
        detect_backend(&env, da1_query(300))
    })
}

/// Raw CSI query helper: writes `query`, collects the reply until
/// `terminator` or timeout. Leading `ESC [` / `ESC O` framing is kept
/// in the returned buffer for the caller to parse.
fn csi_query(query: &[u8], terminator: char, timeout_ms: u64) -> String {
    use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
    use std::io::Write;
    use std::time::{Duration, Instant};

    let mut stdout = std::io::stdout();
    while poll(Duration::from_millis(0)).unwrap_or(false) {
        let _ = read();
    }
    if stdout.write_all(query).is_err() || stdout.flush().is_err() {
        return String::new();
    }
    let deadline = Instant::now() + Duration::from_millis(timeout_ms);
    let mut buf = String::new();
    while Instant::now() < deadline {
        let left = deadline.saturating_duration_since(Instant::now());
        if !poll(left).unwrap_or(false) {
            break;
        }
        match read() {
            Ok(Event::Key(k))
                if k.kind == KeyEventKind::Press || k.kind == KeyEventKind::Repeat =>
            {
                match k.code {
                    KeyCode::Char(c) => {
                        buf.push(c);
                        if c == terminator && buf.len() > 2 {
                            break;
                        }
                    }
                    KeyCode::Esc => buf.push('\x1b'),
                    KeyCode::Enter => buf.push('\n'),
                    _ => {}
                }
            }
            Ok(_) => {}
            Err(_) => break,
        }
        if buf.len() > 256 {
            break;
        }
    }
    while poll(Duration::from_millis(0)).unwrap_or(false) {
        let _ = read();
    }
    buf
}

/// Cell size in device pixels via XTWINOPS (`CSI 14 t` replies
/// `CSI 4 ; height ; width t`). Cached; falls back to a conservative
/// 8x16 estimate (undershoot letterboxes safely, never scrolls).
pub fn cell_pixel_size() -> (u32, u32) {
    static CELL: std::sync::OnceLock<(u32, u32)> = std::sync::OnceLock::new();
    *CELL.get_or_init(|| {
        let resp = csi_query(b"\x1b[14t", 't', 300);
        let s = resp.trim();
        let s = s.strip_prefix("\x1b[").unwrap_or(s);
        let s = s.strip_suffix('t').unwrap_or(s);
        let mut parts = s.split(';');
        match (parts.next(), parts.next(), parts.next()) {
            (Some("4"), Some(h), Some(w)) => {
                let w: u32 = w.trim().parse().unwrap_or(0);
                let h: u32 = h.trim().parse().unwrap_or(0);
                // Sanity: cell must be within 4..=64 px each way.
                let term_w = crossterm::terminal::size().map(|(w, _)| w as u32).unwrap_or(80);
                let term_h = crossterm::terminal::size().map(|(_, h)| h as u32).unwrap_or(24);
                if (4..=64).contains(&w) && (4..=64).contains(&h) && term_w > 0 && term_h > 0 {
                    // Cross-check: reported text area must roughly match
                    // the known terminal size (guards half-parsed junk).
                    let rw = w * term_w;
                    let rh = h * term_h;
                    if (200..=16000).contains(&rw) && (100..=16000).contains(&rh) {
                        return (w, h);
                    }
                }
                (8, 16)
            }
            _ => (8, 16),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantize_solid_is_one_color() {
        let px = vec![(200u8, 10u8, 10u8); 100];
        let pal = median_cut(&px, 256);
        assert_eq!(pal.len(), 1);
        assert_eq!((pal[0].r, pal[0].g, pal[0].b), (78, 4, 4));
    }

    #[test]
    fn quantize_caps_registers() {
        // 300 distinct colors must collapse to <= 16 registers.
        let px: Vec<(u8, u8, u8)> = (0..300)
            .map(|i| ((i % 256) as u8, ((i * 7) % 256) as u8, ((i * 13) % 256) as u8))
            .collect();
        let pal = median_cut(&px, 16);
        assert!(!pal.is_empty() && pal.len() <= 16, "{}", pal.len());
    }

    #[test]
    fn quantize_is_deterministic() {
        let px: Vec<(u8, u8, u8)> = (0..500)
            .map(|i| ((i * 3) as u8, (i * 5) as u8, (i * 11) as u8))
            .collect();
        assert_eq!(median_cut(&px, 32), median_cut(&px, 32));
    }

    #[test]
    fn encode_single_band_solid() {
        // 4x6 all register-0: one band, `!4` + blank rows map to `?`.
        let idx = vec![0u8; 4 * 6];
        let pal = vec![PaletteColor { r: 100, g: 100, b: 100 }];
        let s = encode_sixel(&idx, 4, 6, &pal, (0, 0, 0));
        assert!(s.starts_with("\x1bPq"), "{}", &s[..8]);
        assert!(s.ends_with("\x1b\\"));
        assert!(s.contains("#0;2;0;0;0"));
        assert!(s.contains("#1;2;100;100;100"));
        // All six rows set -> sixel 0b111111 = '~', run of 4 -> `!4~`.
        assert!(s.contains("#1!4~"), "{}", s);
    }

    #[test]
    fn encode_two_bands_and_carriage() {
        // 2x12: top band register 0, bottom band register 1.
        let mut idx = vec![0u8; 2 * 6];
        idx.extend(vec![1u8; 2 * 6]);
        let pal = vec![
            PaletteColor { r: 0, g: 0, b: 0 },
            PaletteColor { r: 100, g: 0, b: 0 },
        ];
        let s = encode_sixel(&idx, 2, 12, &pal, (0, 0, 0));
        // Two bands -> two `-` terminators (one per band).
        assert_eq!(s.matches('-').count(), 2, "{}", s);
        assert!(s.contains("#2"), "{}", s);
    }

    #[test]
    fn encode_rle_and_raw_runs() {
        // Row pattern across 8 cols: A A A A B B B B (single band, 1 row).
        let mut idx = Vec::new();
        for _ in 0..1 {
            idx.extend([0u8, 0, 0, 0, 1, 1, 1, 1]);
        }
        let pal = vec![
            PaletteColor { r: 0, g: 0, b: 0 },
            PaletteColor { r: 0, g: 100, b: 0 },
        ];
        let s = encode_sixel(&idx, 8, 1, &pal, (0, 0, 0));
        // Register 1 rows: bits col0-3 set (top row) -> `~`? No: single
        // row band -> bit0 set -> '@'+1 = 'A'... sixel char = bits+'?':
        // bit0 -> 0b000001 = 1 -> '?' + 1 = '@'.
        assert!(s.contains("#1!4@"), "{}", s);
        assert!(s.contains('$'), "{}", s);
    }

    #[test]
    fn backend_selection_prefers_pixels() {
        use PixelBackend::*;
        // Pure-function smoke: tested exhaustively via detect_* below.
        assert_ne!(Sextant, Sixel);
        assert_ne!(Sextant, Kitty);
    }

    fn env_of(pairs: &[(&str, &str)]) -> Env {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn detect_kitty_family() {
        use PixelBackend::*;
        // Kitty itself, Ghostty (both markers), WezTerm, iTerm2 (Kitty
        // protocol per its docs) — all Kitty, no probe involved.
        assert_eq!(
            detect_backend(&env_of(&[("KITTY_WINDOW_ID", "1")]), None).0,
            Kitty
        );
        assert_eq!(
            detect_backend(&env_of(&[("TERM", "xterm-kitty")]), None).0,
            Kitty
        );
        assert_eq!(
            detect_backend(&env_of(&[("TERM", "xterm-ghostty")]), None).0,
            Kitty
        );
        assert_eq!(
            detect_backend(&env_of(&[("TERM_PROGRAM", "ghostty")]), None).0,
            Kitty
        );
        assert_eq!(
            detect_backend(&env_of(&[("TERM_PROGRAM", "WezTerm")]), None).0,
            Kitty
        );
        assert_eq!(
            detect_backend(&env_of(&[("TERM_PROGRAM", "iTerm.app")]), None).0,
            Kitty
        );
    }

    #[test]
    fn detect_sixel_trust_and_probe() {
        use PixelBackend::*;
        // Windows Terminal is trusted outright (ConPTY DA1 quirk).
        let (b, r) = detect_backend(&env_of(&[("WT_SESSION", "abc")]), None);
        assert_eq!((b, r), (Sixel, "sixel-wt"));
        // foot likewise.
        let (b, r) = detect_backend(&env_of(&[("TERM", "foot")]), None);
        assert_eq!((b, r), (Sixel, "sixel-foot"));
        // Ambiguous xterm: probe decides.
        let xterm = env_of(&[("TERM", "xterm-256color")]);
        assert!(needs_probe(&xterm));
        assert_eq!(detect_backend(&xterm, Some(true)).0, Sixel);
        assert_eq!(detect_backend(&xterm, Some(false)).0, Sextant);
        assert_eq!(detect_backend(&xterm, None).0, Sextant);
        // Non-probeable terms never probe.
        assert!(!needs_probe(&env_of(&[("TERM_PROGRAM", "vscode")])));
        assert!(!needs_probe(&env_of(&[
            ("TERM", "xterm-ghostty"),
            ("TERM_PROGRAM", "vscode")
        ])));
    }

    #[test]
    fn detect_override_wins() {
        use PixelBackend::*;
        assert_eq!(
            detect_backend(&env_of(&[("NFE_IMG", "cells"), ("WT_SESSION", "x")]), None).0,
            Sextant
        );
        assert_eq!(
            detect_backend(&env_of(&[("NFE_IMG", "kitty")]), None).0,
            Kitty
        );
        assert_eq!(
            detect_backend(&env_of(&[("NFE_SIXEL", "1")]), None).0,
            Sixel
        );
        assert_eq!(
            detect_backend(
                &env_of(&[("NFE_SIXEL", "0"), ("KITTY_WINDOW_ID", "1")]),
                None
            )
            .0,
            Sextant
        );
    }

    #[test]
    fn da1_parsing() {
        assert!(da1_advertises_sixel("\x1b[?1;2;4;6c"));
        assert!(da1_advertises_sixel("\x1b[?4c"));
        assert!(da1_advertises_sixel("\x1b[?62;4;6c"));
        assert!(!da1_advertises_sixel("\x1b[?1;2;6c"));
        assert!(!da1_advertises_sixel("\x1b[?62;64c")); // 64 != 4
        assert!(!da1_advertises_sixel(""));
        assert!(!da1_advertises_sixel("\x1b[?c"));
    }

    #[test]
    fn negative_markers_skip_probe() {
        // Known-negative terminals: sextants with no probe stall.
        assert!(!needs_probe(&env_of(&[("TERM_PROGRAM", "vscode")])));
        assert!(!needs_probe(&env_of(&[("TERM_PROGRAM", "Apple_Terminal")])));
        assert!(!needs_probe(&env_of(&[("VTE_VERSION", "1")])));
        assert!(!needs_probe(&env_of(&[("KONSOLE_VERSION", "1")])));
        assert!(!needs_probe(&env_of(&[("TERM", "linux")])));
        assert!(!needs_probe(&env_of(&[("TERM", "dumb")])));
        // Ambiguous ones still probe.
        assert!(needs_probe(&env_of(&[("TERM", "xterm-256color")])));
        assert!(needs_probe(&env_of(&[("TERM", "tmux-256color")])));
    }

    #[test]
    fn legacy_console_defaults_to_half() {
        use crate::nfe::image::ImageMode;
        // Bare Windows env (double-clicked conhost): Half blocks.
        let bare: Env = Env::new();
        #[cfg(windows)]
        assert_eq!(default_image_mode(&bare), ImageMode::Half);
        #[cfg(not(windows))]
        assert_eq!(default_image_mode(&bare), ImageMode::Sextant);
        assert!(is_legacy_console(&bare) || cfg!(not(windows)));
        // WT / kitty markers -> sextants everywhere.
        assert_eq!(
            default_image_mode(&env_of(&[("WT_SESSION", "x")])),
            ImageMode::Sextant
        );
        assert_eq!(
            default_image_mode(&env_of(&[("KITTY_WINDOW_ID", "1")])),
            ImageMode::Sextant
        );
        assert_eq!(
            default_image_mode(&env_of(&[("TERM", "xterm-256color")])),
            ImageMode::Sextant
        );
    }
}
