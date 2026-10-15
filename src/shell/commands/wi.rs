//! `wi` — live watch interface (fullscreen file-change feed).
//!
//! Opens its own temporary recursive watcher on the target directory and
//! streams Created/Modified/Deleted events. Observe-only: watch trigger
//! aliases never fire while `wi` is open.

use crate::navigator::Navigator;
use crate::output::print_error;
use crate::t;
use crate::watcher::{start_watcher, ChangeKind, WatcherHandle};
use anyhow::Result;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind,
};
use crossterm::style::{
    Color as CxColor, Print, ResetColor, SetBackgroundColor, SetForegroundColor,
};
use crossterm::terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, queue};
use std::io::{stdout, Write};
use std::path::PathBuf;

/// Max live rows kept (oldest dropped from the top).
const MAX_ROWS: usize = 200;

struct LiveEvent {
    kind: ChangeKind,
    name: String,
    file_type: String,
    time: String,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Filter {
    All,
    Created,
    Modified,
    Deleted,
}

impl Filter {
    fn next(self) -> Self {
        match self {
            Filter::All => Filter::Created,
            Filter::Created => Filter::Modified,
            Filter::Modified => Filter::Deleted,
            Filter::Deleted => Filter::All,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Filter::All => "all",
            Filter::Created => "created",
            Filter::Modified => "modified",
            Filter::Deleted => "deleted",
        }
    }

    fn matches(self, kind: &ChangeKind) -> bool {
        match self {
            Filter::All => true,
            Filter::Created => *kind == ChangeKind::Created,
            Filter::Modified => *kind == ChangeKind::Modified,
            Filter::Deleted => *kind == ChangeKind::Deleted,
        }
    }
}

fn kind_color(kind: &ChangeKind) -> CxColor {
    match kind {
        ChangeKind::Created => CxColor::Green,
        ChangeKind::Modified => CxColor::Yellow,
        ChangeKind::Deleted => CxColor::Red,
        ChangeKind::Other => CxColor::Cyan,
    }
}

fn kind_label(kind: &ChangeKind) -> &'static str {
    match kind {
        ChangeKind::Created => "created",
        ChangeKind::Modified => "modified",
        ChangeKind::Deleted => "deleted",
        ChangeKind::Other => "changed",
    }
}

fn now_hms() -> String {
    chrono::Local::now().format("%H:%M:%S").to_string()
}

struct Wi {
    events: Vec<LiveEvent>,
    /// Index into `events` of the highlighted row.
    selected: usize,
    /// When true the view sticks to the newest row.
    follow: bool,
    paused: bool,
    /// Events drained while paused (shown in header, not listed).
    skipped: usize,
    filter: Filter,
    created: usize,
    modified: usize,
    deleted: usize,
    term_h: usize,
    term_w: usize,
    root_display: String,
}

impl Wi {
    fn list_h(&self) -> usize {
        // Header(1) + sep(1) + footer(1) + counters(1).
        self.term_h.saturating_sub(4).max(1)
    }

    fn push(&mut self, ev: LiveEvent) {
        // Count everything even when filtered/paused.
        match ev.kind {
            ChangeKind::Created => self.created += 1,
            ChangeKind::Modified => self.modified += 1,
            ChangeKind::Deleted => self.deleted += 1,
            ChangeKind::Other => {}
        }
        if self.paused {
            self.skipped += 1;
            return;
        }
        if !self.filter.matches(&ev.kind) {
            return;
        }
        self.events.push(ev);
        if self.events.len() > MAX_ROWS {
            let overflow = self.events.len() - MAX_ROWS;
            self.events.drain(..overflow);
        }
        if self.follow {
            self.selected = self.events.len().saturating_sub(1);
        } else if self.selected >= self.events.len() {
            self.selected = self.events.len().saturating_sub(1);
        }
    }

    fn visible_range(&self) -> (usize, usize) {
        let lh = self.list_h();
        if self.events.is_empty() {
            return (0, 0);
        }
        if self.follow {
            let end = self.events.len();
            (end.saturating_sub(lh), end)
        } else {
            let start = self
                .selected
                .saturating_sub(lh.saturating_sub(1))
                .min(self.events.len().saturating_sub(lh.min(self.events.len())));
            (start, (start + lh).min(self.events.len()))
        }
    }

    fn render(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        let cols = self.term_w.max(20);

        queue!(stdout, terminal::Clear(ClearType::All))?;

        // ── Header ──
        let state = if self.paused {
            format!("PAUSED (+{} skipped)", self.skipped)
        } else {
            "LIVE".to_string()
        };
        let header = format!(
            " wi: {}  |  {}  |  C:{} M:{} D:{}  |  filter:{} ",
            truncate_cols(&self.root_display, cols.saturating_sub(60).max(10)),
            state,
            self.created,
            self.modified,
            self.deleted,
            self.filter.label(),
        );
        queue!(
            stdout,
            MoveTo(0, 0),
            SetForegroundColor(theme.shell.info.to_crossterm()),
            SetBackgroundColor(theme.editor.status_bg.to_crossterm()),
            Print(truncate_cols(&header, cols)),
            ResetColor,
        )?;

        let sep = "\u{2500}".repeat(cols);
        queue!(stdout, MoveTo(0, 1), Print(&sep), ResetColor)?;

        // ── Event list ──
        let lh = self.list_h();
        let (start, end) = self.visible_range();
        if self.events.is_empty() {
            let msg = if self.paused {
                " paused — press SPACE to resume "
            } else {
                " no changes yet — create, modify or delete a file to see it flow "
            };
            queue!(
                stdout,
                MoveTo(0, 2),
                Print(truncate_cols(msg, cols)),
                ResetColor
            )?;
            for row in 3..lh + 2 {
                if row < self.term_h {
                    queue!(
                        stdout,
                        MoveTo(0, row as u16),
                        Clear(ClearType::UntilNewLine)
                    )?;
                }
            }
        } else {
            for (i, idx) in (start..end).enumerate() {
                let row = 2 + i;
                if row >= self.term_h {
                    break;
                }
                let ev = &self.events[idx];
                let prefix = format!("[{}] {}  {}  ", kind_label(&ev.kind), ev.time, ev.name);
                let avail = cols.saturating_sub(prefix.len() + ev.file_type.len() + 2);
                let _ = avail;
                let line = format!("{}{}", prefix, ev.file_type);
                let shown = truncate_cols(&line, cols);
                if idx == self.selected {
                    queue!(
                        stdout,
                        MoveTo(0, row as u16),
                        SetForegroundColor(theme.editor.selection_bg.to_crossterm()),
                        SetBackgroundColor(theme.editor.selection_bg.to_crossterm()),
                        Print(shown),
                        ResetColor,
                    )?;
                } else {
                    queue!(stdout, MoveTo(0, row as u16))?;
                    queue!(
                        stdout,
                        SetForegroundColor(kind_color(&ev.kind)),
                        Print(format!("[{}]", kind_label(&ev.kind))),
                        ResetColor,
                        Print(format!(" {}  {}  ", ev.time, ev.name)),
                    )?;
                    // file_type dimmed is skipped for width simplicity; plain print:
                    queue!(
                        stdout,
                        Print(truncate_cols(&ev.file_type, cols)),
                        ResetColor
                    )?;
                    queue!(stdout, Clear(ClearType::UntilNewLine))?;
                }
            }
            // Clear leftover rows when the list shrank (filter change / clear).
            for row in 2 + (end - start)..2 + lh {
                if row < self.term_h {
                    queue!(
                        stdout,
                        MoveTo(0, row as u16),
                        Clear(ClearType::UntilNewLine)
                    )?;
                }
            }
        }

        // ── Footer ──
        let footer_row = lh + 2;
        if footer_row < self.term_h {
            let footer = " j/k navigate · End=follow · f=filter · SPACE=pause · c=clear · q=quit ";
            queue!(
                stdout,
                MoveTo(0, footer_row as u16),
                Print(truncate_cols(footer, cols)),
                ResetColor
            )?;
        }

        stdout.flush()?;
        Ok(())
    }
}

fn truncate_cols(s: &str, max: usize) -> &str {
    if s.len() <= max {
        return s;
    }
    let mut end = max.min(s.len());
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

fn terminal_size() -> (usize, usize) {
    let (w, h) = crossterm::terminal::size().unwrap_or((80, 24));
    (w as usize, h as usize)
}

struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
        let _ = execute!(
            std::io::stdout(),
            DisableMouseCapture,
            Show,
            LeaveAlternateScreen
        );
    }
}

fn resolve_root(args: &str, nav: &Navigator) -> Option<PathBuf> {
    let a = args.trim();
    if a.is_empty() {
        return Some(nav.current_path().to_path_buf());
    }
    // Strip surrounding quotes if the user quoted a path with spaces.
    let a = a
        .strip_prefix('"')
        .and_then(|s| s.strip_suffix('"'))
        .unwrap_or(a);
    let p = PathBuf::from(a);
    let full = if p.is_absolute() {
        p
    } else {
        nav.current_path().join(p)
    };
    if full.is_dir() {
        Some(full)
    } else {
        None
    }
}

pub fn cmd_wi(args: &str, nav: &mut Navigator) -> Result<bool> {
    let root = match resolve_root(args, nav) {
        Some(p) => p,
        None => {
            print_error(&t!("misc.wi.not_dir", args.trim()));
            return Ok(false);
        }
    };

    let handle: WatcherHandle = match start_watcher(&root) {
        Ok(h) => h,
        Err(e) => {
            print_error(&t!("misc.wi.start_error", e));
            return Ok(false);
        }
    };

    let mut stdout = stdout();
    let _guard = TerminalGuard;
    execute!(stdout, EnterAlternateScreen, Hide, EnableMouseCapture)?;
    terminal::enable_raw_mode()?;

    let (w, h) = terminal_size();
    let mut wi = Wi {
        events: Vec::new(),
        selected: 0,
        follow: true,
        paused: false,
        skipped: 0,
        filter: Filter::All,
        created: 0,
        modified: 0,
        deleted: 0,
        term_h: h,
        term_w: w,
        root_display: root.to_string_lossy().replace('\\', "/"),
    };
    wi.render(&mut stdout)?;

    loop {
        // Watcher batches on a ~400ms debounce; 150ms tick keeps UI snappy.
        if event::poll(std::time::Duration::from_millis(150))? {
            let ev = event::read()?;
            match ev {
                Event::Key(ke) => {
                    if ke.kind != KeyEventKind::Press {
                        continue;
                    }
                    match ke.code {
                        KeyCode::Esc | KeyCode::Char('q') => break,
                        KeyCode::Char('f') => {
                            wi.filter = wi.filter.next();
                            // Re-anchor selection into the (possibly shorter) view.
                            if !wi.events.is_empty() {
                                wi.selected = wi.selected.min(wi.events.len() - 1);
                            }
                            wi.render(&mut stdout)?;
                        }
                        KeyCode::Char(' ') => {
                            wi.paused = !wi.paused;
                            if !wi.paused {
                                wi.skipped = 0;
                            }
                            wi.render(&mut stdout)?;
                        }
                        KeyCode::Char('c') => {
                            wi.events.clear();
                            wi.selected = 0;
                            wi.follow = true;
                            wi.render(&mut stdout)?;
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            if wi.selected > 0 {
                                wi.selected -= 1;
                            }
                            wi.follow = wi.selected + 1 >= wi.events.len();
                            wi.render(&mut stdout)?;
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if wi.selected + 1 < wi.events.len() {
                                wi.selected += 1;
                            }
                            wi.follow = wi.selected + 1 >= wi.events.len();
                            wi.render(&mut stdout)?;
                        }
                        KeyCode::PageUp => {
                            let step = wi.list_h().max(1);
                            wi.selected = wi.selected.saturating_sub(step);
                            wi.follow = false;
                            wi.render(&mut stdout)?;
                        }
                        KeyCode::PageDown => {
                            let step = wi.list_h().max(1);
                            wi.selected =
                                (wi.selected + step).min(wi.events.len().saturating_sub(1));
                            wi.follow = wi.selected + 1 >= wi.events.len();
                            wi.render(&mut stdout)?;
                        }
                        KeyCode::Home => {
                            wi.selected = 0;
                            wi.follow = false;
                            wi.render(&mut stdout)?;
                        }
                        KeyCode::End | KeyCode::Char('G') => {
                            wi.follow = true;
                            wi.selected = wi.events.len().saturating_sub(1);
                            wi.render(&mut stdout)?;
                        }
                        _ => {}
                    }
                }
                Event::Resize(nw, nh) => {
                    wi.term_w = nw as usize;
                    wi.term_h = nh as usize;
                    wi.render(&mut stdout)?;
                }
                Event::Mouse(m) => match m.kind {
                    crossterm::event::MouseEventKind::ScrollUp => {
                        if wi.selected > 0 {
                            wi.selected -= 1;
                        }
                        wi.follow = false;
                        wi.render(&mut stdout)?;
                    }
                    crossterm::event::MouseEventKind::ScrollDown => {
                        if wi.selected + 1 < wi.events.len() {
                            wi.selected += 1;
                        }
                        wi.follow = wi.selected + 1 >= wi.events.len();
                        wi.render(&mut stdout)?;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // Drain watcher batches (observe-only: trigger_alias deliberately ignored).
        if let Some(summary) = handle.poll() {
            if !summary.events.is_empty() {
                for ev in summary.events {
                    wi.push(LiveEvent {
                        kind: ev.kind,
                        name: ev.name,
                        file_type: ev.file_type,
                        time: now_hms(),
                    });
                }
                wi.render(&mut stdout)?;
            }
        }
    }

    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_cycles_all_kinds_and_back() {
        let mut f = Filter::All;
        f = f.next();
        assert_eq!(f, Filter::Created);
        f = f.next();
        assert_eq!(f, Filter::Modified);
        f = f.next();
        assert_eq!(f, Filter::Deleted);
        f = f.next();
        assert_eq!(f, Filter::All);
    }

    #[test]
    fn filter_matches_expected_kinds() {
        assert!(Filter::All.matches(&ChangeKind::Created));
        assert!(Filter::All.matches(&ChangeKind::Other));
        assert!(Filter::Created.matches(&ChangeKind::Created));
        assert!(!Filter::Created.matches(&ChangeKind::Deleted));
        assert!(Filter::Modified.matches(&ChangeKind::Modified));
        assert!(!Filter::Modified.matches(&ChangeKind::Created));
        assert!(Filter::Deleted.matches(&ChangeKind::Deleted));
        assert!(!Filter::Deleted.matches(&ChangeKind::Modified));
    }
}
