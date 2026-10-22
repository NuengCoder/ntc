//! Fullscreen `nms` player UI: waveform, level meter, progress/countdown and
//! keyboard control. Uses crossterm's alternate screen so the shell is
//! restored on quit (and on panic via [`TerminalGuard`]).

use std::collections::VecDeque;
use std::io::{self, Stdout, Write};
use std::path::Path;
use std::time::Duration;

use anyhow::Result;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor};
use crossterm::terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, queue};

use crate::nms::player::AudioPlayer;

/// How often the UI refreshes (also the waveform sampling interval).
const FRAME: Duration = Duration::from_millis(50);
/// Seek step for the arrow keys.
const SEEK_STEP: f32 = 5.0;
/// Volume step per key press.
const VOLUME_STEP: f32 = 0.05;

/// Restores the terminal on drop (normal exit or panic).
struct TerminalGuard;

impl TerminalGuard {
    fn enter() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(
            io::stdout(),
            EnterAlternateScreen,
            Hide,
            Clear(ClearType::All)
        )?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), LeaveAlternateScreen, Show);
        let _ = terminal::disable_raw_mode();
    }
}

/// Rolling waveform history plus a smoothed level value.
struct View {
    history: VecDeque<f32>,
    level: f32,
}

impl View {
    fn new() -> Self {
        Self {
            history: VecDeque::new(),
            level: 0.0,
        }
    }

    fn update(&mut self, peak: f32, level: f32, width: usize) {
        self.history.push_back(peak.clamp(0.0, 1.0));
        // Smooth the instantaneous level so the meter does not flicker.
        self.level = self.level * 0.6 + level.clamp(0.0, 1.0) * 0.4;
        let cap = width.max(1);
        while self.history.len() > cap {
            self.history.pop_front();
        }
    }
}

/// Run the player until the user quits.
pub fn run(mut player: AudioPlayer, path: &Path) -> Result<()> {
    let mut stdout = io::stdout();
    let _guard = TerminalGuard::enter()?;

    let is_video = crate::nms::is_video(path);
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.display().to_string());

    let mut view = View::new();

    loop {
        let (cols, rows) = terminal::size().unwrap_or((80, 24));
        let cols = cols.max(24) as usize;
        let rows = rows.max(10) as usize;

        if player.finished() {
            // Playback ended: stop drawing the waveform (no `O`s) and freeze the
            // animation instead of spinning. Wait for a replay or quit.
            view.level = 0.0;
        } else {
            view.update(player.take_peak(), player.level(), cols);
        }
        draw(&mut stdout, &player, &view, &name, is_video, cols, rows)?;

        if player.finished() {
            loop {
                if let Event::Key(key) = event::read()? {
                    if key.kind != KeyEventKind::Press {
                        continue;
                    }
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char('a') => {
                            if player.restart().is_ok() {
                                view = View::new();
                            }
                            break;
                        }
                        _ => {}
                    }
                }
            }
            continue;
        }

        if event::poll(FRAME)? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('a') => {
                        if player.restart().is_ok() {
                            view = View::new();
                        }
                    }
                    KeyCode::Char(' ') => player.toggle_pause(),
                    KeyCode::Left => player.seek_relative(-SEEK_STEP),
                    KeyCode::Right => player.seek_relative(SEEK_STEP),
                    KeyCode::Up | KeyCode::Char('+') | KeyCode::Char('=') => {
                        player.adjust_volume(VOLUME_STEP)
                    }
                    KeyCode::Down | KeyCode::Char('-') => player.adjust_volume(-VOLUME_STEP),
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn draw(
    out: &mut Stdout,
    player: &AudioPlayer,
    view: &View,
    name: &str,
    is_video: bool,
    cols: usize,
    rows: usize,
) -> io::Result<()> {
    const WAVE_TOP: usize = 3;
    const FOOTER: usize = 3;
    let wave_h = rows.saturating_sub(WAVE_TOP + FOOTER).max(3);
    let level_row = WAVE_TOP + wave_h;
    let progress_row = level_row + 1;
    let keys_row = progress_row + 1;

    queue!(out, Clear(ClearType::All))?;

    // ── Title / header ────────────────────────────────────────────────
    put(out, 0, 0, " NTC MUSIC ", Color::Black, false)?;
    // Redraw the title with a colored background look by using a bright color.
    put(out, 0, 0, "♪ NTC MUSIC", Color::Cyan, true)?;

    let tag = if is_video {
        "VIDEO · audio only"
    } else {
        "AUDIO"
    };
    let header = format!("  {}  [{}]", name, tag);
    put(out, 1, 0, &truncate(&header, cols), Color::White, true)?;

    // ── Waveform ──────────────────────────────────────────────────────
    // Bottom-anchored "candle" columns of `O`, one per sample, colored by the
    // sample's level (green low / yellow mid / red high). Rendered row by row.
    let wave_w = cols.saturating_sub(2);
    let grid = wave_cells(&view.history, wave_w, wave_h, player.finished());
    for (r, row) in grid.iter().enumerate() {
        let y = (WAVE_TOP + r) as u16;
        queue!(out, MoveTo(1, y), ResetColor)?;
        let mut i = 0;
        while i < row.len() {
            match row[i] {
                Some((_, color)) => {
                    // Coalesce an unbroken run of same-colored candles.
                    let mut j = i;
                    while j < row.len() && matches!(row[j], Some((_, c)) if c == color) {
                        j += 1;
                    }
                    queue!(
                        out,
                        SetForegroundColor(color),
                        Print("O".repeat(j - i)),
                        ResetColor
                    )?;
                    i = j;
                }
                None => {
                    // Advance the cursor over the empty cells so later runs
                    // keep their column positions.
                    let mut j = i;
                    while j < row.len() && row[j].is_none() {
                        j += 1;
                    }
                    queue!(out, Print(" ".repeat(j - i)))?;
                    i = j;
                }
            }
        }
    }

    // ── Level meter ───────────────────────────────────────────────────
    let meter_w = cols.saturating_sub(24).clamp(10, 40);
    let filled = (view.level * meter_w as f32).round() as usize;
    let filled = filled.min(meter_w);
    let meter = format!("{}{}", "█".repeat(filled), "░".repeat(meter_w - filled));
    let level_line = format!("  LEVEL [{:>5.2}] {}", view.level, meter);
    put(
        out,
        level_row as u16,
        0,
        &truncate(&level_line, cols),
        Color::Magenta,
        false,
    )?;

    // ── Progress + countdown ──────────────────────────────────────────
    let pos = player.position();
    let dur = player.duration();
    let frac = dur
        .map(|d| {
            if d.as_secs_f64() > 0.0 {
                (pos.as_secs_f64() / d.as_secs_f64()).clamp(0.0, 1.0)
            } else {
                0.0
            }
        })
        .unwrap_or(0.0);
    let remaining = dur
        .map(|d| format_time(d.saturating_sub(pos)))
        .unwrap_or_else(|| "--:--".to_string());
    let times = format!(
        "{} / {}  -{}",
        format_time(pos),
        dur.map(format_time).unwrap_or_else(|| "--:--".to_string()),
        remaining
    );
    let bar_w = cols.saturating_sub(times.len() + 6).clamp(8, 80);
    let done = (frac * bar_w as f64).round() as usize;
    let done = done.min(bar_w);
    let bar = format!("{}{}", "█".repeat(done), "─".repeat(bar_w - done));
    let status = if player.finished() {
        "■ FINISHED"
    } else if player.is_paused() {
        "⏸ PAUSED"
    } else {
        "▶ PLAYING"
    };
    let progress_line = format!("  [{}] {}  {}", bar, times, status);
    put(
        out,
        progress_row as u16,
        0,
        &truncate(&progress_line, cols),
        Color::White,
        false,
    )?;

    // ── Key hints ─────────────────────────────────────────────────────
    let keys = format!(
        "  space pause/resume · ←/→ seek {}s · ↑/↓ volume {}% · a replay · q quit",
        SEEK_STEP as i32,
        (player.volume() * 100.0).round() as i32
    );
    put(
        out,
        keys_row as u16,
        0,
        &truncate(&keys, cols),
        Color::DarkGrey,
        false,
    )?;

    out.flush()
}

fn put(
    out: &mut Stdout,
    row: u16,
    col: u16,
    text: &str,
    color: Color,
    bold: bool,
) -> io::Result<()> {
    queue!(out, MoveTo(col, row), SetForegroundColor(color))?;
    if bold {
        queue!(out, SetAttribute(Attribute::Bold))?;
    }
    queue!(out, Print(text), SetAttribute(Attribute::Reset), ResetColor)?;
    Ok(())
}

/// Truncate to `cols` characters (char-safe) to avoid line wrapping.
fn truncate(s: &str, cols: usize) -> String {
    s.chars().take(cols).collect()
}

/// Format a duration as `M:SS` (or `H:MM:SS` past an hour).
pub fn format_time(d: Duration) -> String {
    let total = d.as_secs();
    let (h, m, s) = (total / 3600, (total % 3600) / 60, total % 60);
    if h > 0 {
        format!("{}:{:02}:{:02}", h, m, s)
    } else {
        format!("{}:{:02}", m, s)
    }
}

/// Bottom-anchored candle height for a level in `0.0..=1.0`.
///
/// Always at least `1` so every column keeps a baseline `O` at the very bottom
/// even when the sample is silent.
fn bar_height(level: f32, wave_h: usize) -> usize {
    let scaled = (level.clamp(0.0, 1.0) * wave_h as f32).round() as usize;
    scaled.clamp(1, wave_h.max(1))
}

/// Color for a sample: green (low), yellow (mid) or red (high).
fn level_color(level: f32) -> Color {
    if level < 0.33 {
        Color::Green
    } else if level < 0.66 {
        Color::Yellow
    } else {
        Color::Red
    }
}

/// Build the waveform as `wave_h` rows of `wave_w` cells, bottom-anchored.
///
/// `history` runs oldest → newest; the newest sample lands in the rightmost
/// column. Each cell is `Some(('O', color))` when that column's candle reaches
/// it, or `None` for background.
fn wave_grid(
    history: &VecDeque<f32>,
    wave_w: usize,
    wave_h: usize,
) -> Vec<Vec<Option<(char, Color)>>> {
    let mut grid = vec![vec![None; wave_w]; wave_h];
    if wave_h == 0 || wave_w == 0 {
        return grid;
    }
    // Keep the newest `wave_w` samples, then lay them out left(older)→right(newer).
    let tail: Vec<f32> = history.iter().rev().take(wave_w).copied().collect();
    for (i, &level) in tail.iter().rev().enumerate() {
        let color = level_color(level);
        let top = wave_h - bar_height(level, wave_h);
        for row in grid.iter_mut().take(wave_h).skip(top) {
            row[i] = Some(('O', color));
        }
    }
    grid
}

/// Waveform cells for the current frame.
///
/// Returns nothing once the track has finished, so the frozen final frame shows
/// no `O`s; otherwise it is exactly [`wave_grid`].
fn wave_cells(
    history: &VecDeque<f32>,
    wave_w: usize,
    wave_h: usize,
    finished: bool,
) -> Vec<Vec<Option<(char, Color)>>> {
    if finished {
        Vec::new()
    } else {
        wave_grid(history, wave_w, wave_h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_times() {
        assert_eq!(format_time(Duration::from_secs(0)), "0:00");
        assert_eq!(format_time(Duration::from_secs(65)), "1:05");
        assert_eq!(format_time(Duration::from_secs(3661)), "1:01:01");
    }

    #[test]
    fn truncate_is_char_safe() {
        assert_eq!(truncate("abcdef", 3), "abc");
        assert_eq!(truncate("hello", 10), "hello");
    }

    #[test]
    fn view_caps_history_to_width() {
        let mut v = View::new();
        for _ in 0..100 {
            v.update(0.5, 0.5, 10);
        }
        assert_eq!(v.history.len(), 10);
    }

    #[test]
    fn bar_height_is_bottom_anchored_with_baseline() {
        assert_eq!(bar_height(0.0, 5), 1); // silent still shows a baseline O
        assert_eq!(bar_height(0.5, 10), 5);
        assert_eq!(bar_height(1.0, 5), 5);
        assert_eq!(bar_height(2.0, 5), 5); // clamped to the area
    }

    #[test]
    fn level_color_thresholds() {
        assert_eq!(level_color(0.0), Color::Green);
        assert_eq!(level_color(0.32), Color::Green);
        assert_eq!(level_color(0.33), Color::Yellow);
        assert_eq!(level_color(0.65), Color::Yellow);
        assert_eq!(level_color(0.66), Color::Red);
        assert_eq!(level_color(1.0), Color::Red);
    }

    #[test]
    fn wave_grid_draws_bottom_anchored_candles() {
        let mut h = VecDeque::new();
        for v in [0.2f32, 0.4, 0.6, 0.8, 1.0] {
            h.push_back(v);
        }
        let grid = wave_grid(&h, 5, 5);
        let art: Vec<String> = grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| if c.is_some() { 'O' } else { ' ' })
                    .collect()
            })
            .collect();
        assert_eq!(art, vec!["    O", "   OO", "  OOO", " OOOO", "OOOOO"]);

        // Every column is filled from the bottom with no gaps.
        for x in 0..5 {
            let mut seen_fill = false;
            for row in &grid {
                match row[x] {
                    Some(_) => seen_fill = true,
                    None => assert!(!seen_fill, "gap in column {x}"),
                }
            }
        }
    }

    #[test]
    fn wave_cells_empty_when_finished() {
        let mut h = VecDeque::new();
        for v in [0.5f32, 0.9] {
            h.push_back(v);
        }
        // Finished: no `O`s are drawn at all.
        assert!(wave_cells(&h, 4, 3, true).is_empty());
        // Playing: the candle grid is produced.
        assert_eq!(wave_cells(&h, 4, 3, false).len(), 3);
    }
}
