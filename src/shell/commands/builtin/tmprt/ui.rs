//! Shared formatting helpers for the `tmprt` views.

/// Terminal size, falling back to 80x24 when the OS does not report one.
pub(super) fn terminal_size() -> (usize, usize) {
    crossterm::terminal::size()
        .map(|(w, h)| (w as usize, h as usize))
        .unwrap_or((80, 24))
}

/// Truncate to `max` bytes on a char boundary.
pub(super) fn truncate(s: &str, max: usize) -> &str {
    if s.len() <= max {
        return s;
    }
    if max == 0 {
        return "";
    }
    let mut end = max;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

/// 12-cell usage gauge: `[------    ]`.
pub(super) fn render_text_bar(pct: usize) -> String {
    let fill = (pct * 10 / 100).min(10);
    let empty = 10usize.saturating_sub(fill);
    format!("[{}{}]", "-".repeat(fill), " ".repeat(empty))
}

pub(super) fn pct_color(
    pct: usize,
    fg_success: crossterm::style::Color,
    fg_warn: crossterm::style::Color,
    fg_err: crossterm::style::Color,
) -> crossterm::style::Color {
    if pct > 80 {
        fg_err
    } else if pct > 50 {
        fg_warn
    } else {
        fg_success
    }
}

pub(super) fn temp_color(
    temp_c: f32,
    critical: Option<f32>,
    fg_success: crossterm::style::Color,
    fg_warn: crossterm::style::Color,
    fg_err: crossterm::style::Color,
) -> crossterm::style::Color {
    if let Some(crit) = critical {
        if temp_c >= crit * 0.85 {
            fg_err
        } else if temp_c >= crit * 0.6 {
            fg_warn
        } else {
            fg_success
        }
    } else if temp_c >= 75.0 {
        fg_err
    } else if temp_c >= 55.0 {
        fg_warn
    } else {
        fg_success
    }
}

pub(super) fn to_unit(c: f32, celsius: bool) -> f32 {
    if celsius {
        c
    } else {
        c * 9.0 / 5.0 + 32.0
    }
}

pub(super) fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "K", "M", "G", "T"];
    let mut value = bytes as f64;
    let mut unit_idx = 0;
    while value >= 1024.0 && unit_idx + 1 < UNITS.len() {
        value /= 1024.0;
        unit_idx += 1;
    }
    if value < 10.0 {
        format!("{:.1}{}", value, UNITS[unit_idx])
    } else {
        format!("{:.0}{}", value, UNITS[unit_idx])
    }
}
