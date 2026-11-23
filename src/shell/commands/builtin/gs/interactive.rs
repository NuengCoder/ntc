use crate::navigator::Navigator;
use crate::search::{display_content_path, ContentMatch, ContextSpec, MatchMode, MatchOptions};
use crate::shell::commands::support::search::GsSort;
use crate::t;
use anyhow::Result;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers,
};
use crossterm::style::{
    Color as CxColor, Print, ResetColor, SetBackgroundColor, SetForegroundColor,
};
use crossterm::terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, queue};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, stdout, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

/// Lines of context shown around the selected match on each side.
const DEFAULT_CONTEXT: usize = 5;
/// Upper bound accepted by `-A`/`-B` and the `c` prompt.
const MAX_CONTEXT: usize = 10;
const MAX_CACHED_FILE_LINES: usize = 50_000;
/// Re-searches from the pager are capped like `gs` itself.
const MAX_MATCHES: usize = 20_000;
/// Each interactive page holds exactly 10 matches, keyed to `0`-`9`.
pub const GS_PAGE_SIZE: usize = 10;

/// What the pager re-runs when the pattern changes (`gs -i`, `f`).
pub(crate) struct SearchRequest {
    pub root: PathBuf,
    pub max_depth: usize,
    pub paths: Vec<String>,
    pub sort: Option<GsSort>,
    pub context: ContextSpec,
    /// `-A`/`-B` was given, so `0/0` means "match line only" rather than
    /// "no context flags → use the default".
    pub context_requested: bool,
    /// `-x` / `-h` / `-g` / `-w` / `-z`; the pager toggles these live.
    pub opts: MatchOptions,
}

/// Colours used by the pager. Every field is `None` when `cfg color off`
/// (or the terminal cannot render ANSI), in which case no colour escape is
/// written at all and the terminal's default fg/bg is used.
#[derive(Clone, Copy)]
struct Palette {
    header_fg: Option<CxColor>,
    header_bg: Option<CxColor>,
    separator: Option<CxColor>,
    selected_fg: Option<CxColor>,
    selected_bg: Option<CxColor>,
    hit_fg: Option<CxColor>,
    muted: Option<CxColor>,
    prompt_fg: Option<CxColor>,
    prompt_bg: Option<CxColor>,
}

impl Palette {
    fn current() -> Self {
        Self::for_enabled(crate::utils::theme::ansi_enabled())
    }

    fn for_enabled(enabled: bool) -> Self {
        if !enabled {
            return Self {
                header_fg: None,
                header_bg: None,
                separator: None,
                selected_fg: None,
                selected_bg: None,
                hit_fg: None,
                muted: None,
                prompt_fg: None,
                prompt_bg: None,
            };
        }
        let theme = crate::utils::theme::ThemeManager::current();
        Self {
            header_fg: Some(theme.shell.info.to_crossterm()),
            header_bg: Some(theme.editor.status_bg.to_crossterm()),
            separator: Some(theme.editor.border.to_crossterm()),
            // Foreground must differ from the selection background or the
            // selected row / target line paints itself invisible.
            selected_fg: Some(theme.editor.status_text.to_crossterm()),
            selected_bg: Some(theme.editor.selection_bg.to_crossterm()),
            hit_fg: Some(CxColor::Red),
            muted: Some(theme.editor.gutter_text.to_crossterm()),
            prompt_fg: Some(theme.shell.info.to_crossterm()),
            prompt_bg: Some(theme.editor.status_bg.to_crossterm()),
        }
    }
}

fn set_fg<W: Write>(out: &mut W, color: Option<CxColor>) -> io::Result<()> {
    if let Some(color) = color {
        queue!(out, SetForegroundColor(color))?;
    }
    Ok(())
}

fn set_bg<W: Write>(out: &mut W, color: Option<CxColor>) -> io::Result<()> {
    if let Some(color) = color {
        queue!(out, SetBackgroundColor(color))?;
    }
    Ok(())
}

struct GsInteractive {
    matches: Vec<ContentMatch>,
    filtered: Vec<usize>,
    selected: usize,
    term_h: usize,
    term_w: usize,
    file_cache: HashMap<PathBuf, Vec<String>>,
    pattern: String,
    request: SearchRequest,
    /// Lines of context previewed before / after the selected match.
    ctx_before: usize,
    ctx_after: usize,
    /// Active `c` context prompt buffer (None = no prompt).
    context_prompt: Option<String>,
    /// Active pattern editor buffer (None = not editing).
    pattern_buf: Option<String>,
    /// A re-search is running (draws a centered status row).
    busy: bool,
    /// The `?` key-help overlay is open.
    show_help: bool,
}

impl GsInteractive {
    fn total(&self) -> usize {
        self.filtered.len()
    }

    fn page_count(&self) -> usize {
        (self.total() + GS_PAGE_SIZE - 1) / GS_PAGE_SIZE.max(1)
    }

    fn page(&self) -> usize {
        if self.total() == 0 {
            0
        } else {
            self.selected / GS_PAGE_SIZE
        }
    }

    fn page_range(&self) -> (usize, usize) {
        let start = self.page() * GS_PAGE_SIZE;
        let end = (start + GS_PAGE_SIZE).min(self.total());
        (start, end)
    }

    fn result_h(&self) -> usize {
        // header + separator, preview separator, and the hint bar are fixed;
        // the rest of the screen goes to results, capped at one page.
        let preview_rows = self.ctx_before + self.ctx_after + 1;
        let fixed = 2 + 1 + 1;
        let avail = self.term_h.saturating_sub(fixed + preview_rows);
        avail.clamp(1, GS_PAGE_SIZE)
    }

    /// Apply a typed context size to both sides, clamped to the allowed range.
    fn apply_context_prompt(&mut self) {
        if let Some(buf) = self.context_prompt.take() {
            if let Ok(n) = buf.parse::<usize>() {
                let n = n.min(MAX_CONTEXT);
                self.ctx_before = n;
                self.ctx_after = n;
            }
        }
    }

    fn selected_match(&self) -> Option<&ContentMatch> {
        self.filtered.get(self.selected).map(|&i| &self.matches[i])
    }

    fn get_file_lines(&mut self, path: &Path) -> Vec<String> {
        if !self.file_cache.contains_key(path) {
            let mut lines = Vec::new();
            if let Ok(file) = File::open(path) {
                let reader = BufReader::new(file);
                for line in reader.lines().take(MAX_CACHED_FILE_LINES) {
                    match line {
                        Ok(l) => lines.push(l),
                        Err(_) => break,
                    }
                }
            }
            self.file_cache.insert(path.to_path_buf(), lines);
        }
        self.file_cache.get(path).cloned().unwrap_or_default()
    }

    fn goto_page(&mut self, page: usize) {
        let pc = self.page_count();
        if pc == 0 {
            self.selected = 0;
            return;
        }
        let p = page.min(pc - 1);
        self.selected = (p * GS_PAGE_SIZE).min(self.total().saturating_sub(1));
    }

    fn next_page(&mut self) {
        self.goto_page(self.page() + 1);
    }

    fn prev_page(&mut self) {
        self.goto_page(self.page().saturating_sub(1));
    }

    fn ensure_visible(&mut self) {
        if self.total() == 0 {
            self.selected = 0;
            return;
        }
        self.selected = self.selected.min(self.total() - 1);
    }

    /// Jump to the row numbered `digit` on the current page (`0` = first),
    /// returning whether the selection moved.
    fn jump_to_row(&mut self, digit: usize) -> bool {
        let (page_start, page_end) = self.page_range();
        let target = page_start + digit;
        if target < page_end {
            self.selected = target;
            true
        } else {
            false
        }
    }

    /// Re-run the search for `pattern` and select the first hit.
    fn search(&mut self, pattern: &str) {
        let mut found = crate::shell::commands::builtin::gs::search_matches(&self.request, pattern);
        found.truncate(MAX_MATCHES);
        self.matches = found;
        self.pattern = pattern.to_string();
        self.selected = 0;
        self.filtered = (0..self.matches.len()).collect();
    }

    /// Split `text` into (segment, is_match) for red pattern highlight.
    ///
    /// Shares [`crate::utils::search::highlight_segments_opts`] with `gs`
    /// output, so every alternative of an OR pattern (`a\|b`) is highlighted
    /// here too, and the `x`/`h`/`g`/`w`/`z` toggles mark what they match.
    fn match_segments<'a>(&self, text: &'a str) -> Vec<(&'a str, bool)> {
        crate::utils::search::highlight_segments_opts(text, &self.pattern, self.request.opts)
    }

    /// Active match flags, e.g. `-h -z` (empty when everything is default).
    fn flags(&self) -> String {
        let o = self.request.opts;
        let mut flags: Vec<&str> = Vec::new();
        match o.mode {
            MatchMode::StartsWith => flags.push("-h"),
            MatchMode::EndsWith => flags.push("-g"),
            MatchMode::Whole => flags.push("-w"),
            MatchMode::Contains => {}
        }
        if o.literal {
            flags.push("-x");
        }
        if o.case_sensitive {
            flags.push("-z");
        }
        flags.join(" ")
    }

    /// Toggle an anchored mode (`h`/`g`/`w`): the active one turns off, and
    /// the three stay exclusive.
    fn toggle_mode(&mut self, mode: MatchMode) {
        self.request.opts.mode = if self.request.opts.mode == mode {
            MatchMode::Contains
        } else {
            mode
        };
    }

    fn toggle_literal(&mut self) {
        self.request.opts.literal = !self.request.opts.literal;
    }

    fn toggle_case(&mut self) {
        self.request.opts.case_sensitive = !self.request.opts.case_sensitive;
    }

    fn truncate_chars(s: &str, max: usize) -> &str {
        if s.len() <= max {
            return s;
        }
        let mut end = max.min(s.len());
        while end > 0 && !s.is_char_boundary(end) {
            end -= 1;
        }
        &s[..end]
    }

    /// Keep the tail of `s` when it is wider than `max` (pattern editor).
    fn fit_tail(s: &str, max: usize) -> String {
        if max == 0 {
            return String::new();
        }
        if s.chars().count() <= max {
            return s.to_string();
        }
        let keep = max.saturating_sub(1);
        let tail: String = s
            .chars()
            .rev()
            .take(keep)
            .collect::<Vec<char>>()
            .into_iter()
            .rev()
            .collect();
        format!("…{}", tail)
    }

    fn list_hint(&self) -> String {
        [
            t!("misc.gs.pager.hint_help"),
            t!("misc.gs.pager.hint_pattern"),
            t!("misc.gs.pager.hint_filters"),
            t!("misc.gs.pager.hint_open"),
            t!("misc.gs.pager.hint_row"),
            t!("misc.gs.pager.hint_move"),
            t!("misc.gs.pager.hint_page"),
            t!(
                "misc.gs.pager.hint_context",
                self.ctx_before,
                self.ctx_after
            ),
            t!("misc.gs.pager.hint_quit"),
        ]
        .join(" | ")
    }

    fn edit_hint(&self) -> String {
        [
            t!("misc.gs.pager.hint_help"),
            t!("misc.gs.pager.hint_type"),
            t!("misc.gs.pager.hint_accept"),
            t!("misc.gs.pager.hint_cancel"),
        ]
        .join(" | ")
    }

    fn render_row<W: Write>(
        &self,
        out: &mut W,
        p: &Palette,
        row: usize,
        index: &str,
        prefix: &str,
        content: &str,
        selected: bool,
    ) -> io::Result<()> {
        queue!(out, MoveTo(0, row as u16))?;
        let base_fg = if selected { p.selected_fg } else { None };
        if selected {
            set_bg(out, p.selected_bg)?;
            set_fg(out, p.selected_fg)?;
        }
        set_fg(out, p.muted)?;
        queue!(out, Print(index))?;
        set_fg(out, base_fg)?;
        queue!(out, Print(prefix))?;
        for (seg, is_hit) in self.match_segments(content) {
            if is_hit {
                set_fg(out, p.hit_fg)?;
                queue!(out, Print(seg))?;
                set_fg(out, base_fg)?;
            } else {
                queue!(out, Print(seg))?;
            }
        }
        queue!(out, ResetColor, Clear(ClearType::UntilNewLine))?;
        Ok(())
    }

    fn render_header<W: Write>(&self, out: &mut W, p: &Palette) -> io::Result<()> {
        let total = self.total();
        let mut header = t!(
            "misc.gs.pager.header",
            if total == 0 { 0 } else { self.selected + 1 },
            total,
            if total == 0 { 0 } else { self.page() + 1 },
            self.page_count().max(1)
        );
        let flags = self.flags();
        if !flags.is_empty() {
            header.push_str(" | ");
            header.push_str(&flags);
        }
        let truncated = Self::truncate_chars(&header, self.term_w);
        queue!(out, MoveTo(0, 0))?;
        set_bg(out, p.header_bg)?;
        set_fg(out, p.header_fg)?;
        queue!(
            out,
            Print(truncated),
            Clear(ClearType::UntilNewLine),
            ResetColor
        )?;
        Ok(())
    }

    fn render_separator<W: Write>(&self, out: &mut W, p: &Palette, row: usize) -> io::Result<()> {
        let sep = "─".repeat(self.term_w);
        queue!(out, MoveTo(0, row as u16))?;
        set_fg(out, p.separator)?;
        queue!(out, Print(&sep), ResetColor)?;
        Ok(())
    }

    fn render_preview<W: Write>(&mut self, out: &mut W, p: &Palette, top: usize) -> io::Result<()> {
        let Some((file_path, line_num)) = self
            .selected_match()
            .map(|m| (m.file_path.clone(), m.line_number))
        else {
            return Ok(());
        };
        let start = line_num.saturating_sub(self.ctx_before);
        let lines = self.get_file_lines(&file_path);
        let last_row = self.term_h.saturating_sub(1);

        for i in 0..=self.ctx_before + self.ctx_after {
            let row = top + 1 + i;
            if row >= last_row {
                break;
            }
            let src_line = start + i;
            let prefix = format!("{:>6} ", src_line);

            let display = if src_line >= 1 && src_line <= lines.len() {
                let text = &lines[src_line - 1];
                let available = self.term_w.saturating_sub(7);
                let text = Self::truncate_chars(text, available);
                format!("{}{}", prefix, text)
            } else {
                format!("{}~", prefix)
            };

            let truncated = Self::truncate_chars(&display, self.term_w);

            queue!(out, MoveTo(0, row as u16))?;
            if src_line == line_num {
                set_bg(out, p.selected_bg)?;
                set_fg(out, p.selected_fg)?;
            }
            queue!(
                out,
                Print(truncated),
                ResetColor,
                Clear(ClearType::UntilNewLine)
            )?;
        }
        Ok(())
    }

    fn render_results<W: Write>(&mut self, out: &mut W, p: &Palette) -> io::Result<()> {
        let rh = self.result_h().max(1);
        let (page_start, page_end) = self.page_range();

        self.render_header(out, p)?;
        self.render_separator(out, p, 1)?;

        for i in 0..rh {
            let idx = page_start + i;
            let row = 2 + i;
            if row >= self.term_h.saturating_sub(1) {
                break;
            }
            if idx >= page_end {
                queue!(out, MoveTo(0, row as u16), Clear(ClearType::UntilNewLine))?;
                continue;
            }
            let m = &self.matches[self.filtered[idx]];
            let index = format!("{} ", idx - page_start);
            let rel = display_content_path(&m.file_path, &self.request.root);
            let prefix = format!("{}:{}: ", rel, m.line_number);
            let content = m.line.trim();
            let avail = self.term_w.saturating_sub(index.len() + prefix.len() + 1);
            let shown = Self::truncate_chars(content, avail.max(10));
            self.render_row(out, p, row, &index, &prefix, shown, idx == self.selected)?;
        }

        let preview_top = 2 + rh;
        if preview_top < self.term_h.saturating_sub(1) {
            self.render_separator(out, p, preview_top)?;
            self.render_preview(out, p, preview_top)?;
        }
        Ok(())
    }

    fn render_pattern_box<W: Write>(&self, out: &mut W, p: &Palette) -> io::Result<()> {
        let label = t!("misc.gs.pager.pattern_box");
        let raw = format!(
            "[{}{}█]",
            label,
            self.pattern_buf.as_deref().unwrap_or_default()
        );
        let max = self.term_w.saturating_sub(2).max(8);
        let text = Self::fit_tail(&raw, max);
        let width = text.chars().count();
        let col = self.term_w.saturating_sub(width) / 2;
        let row = (self.term_h / 2).min(self.term_h.saturating_sub(2));

        queue!(out, MoveTo(col as u16, row as u16))?;
        set_bg(out, p.prompt_bg)?;
        set_fg(out, p.prompt_fg)?;
        queue!(
            out,
            Print(&text),
            ResetColor,
            Clear(ClearType::UntilNewLine)
        )?;
        Ok(())
    }

    fn render_center<W: Write>(&self, out: &mut W, text: &str, row: usize) -> io::Result<()> {
        let text = Self::truncate_chars(text, self.term_w);
        let width = text.chars().count();
        let col = self.term_w.saturating_sub(width) / 2;
        queue!(out, MoveTo(col as u16, row as u16), Print(text))?;
        Ok(())
    }

    fn render_hint<W: Write>(&self, out: &mut W, p: &Palette, text: &str) -> io::Result<()> {
        let row = self.term_h.saturating_sub(1);
        let shown = Self::truncate_chars(text, self.term_w);
        queue!(out, MoveTo(0, row as u16))?;
        set_bg(out, p.prompt_bg)?;
        set_fg(out, p.prompt_fg)?;
        queue!(
            out,
            Print(shown),
            Clear(ClearType::UntilNewLine),
            ResetColor
        )?;
        Ok(())
    }

    /// The `?` overlay: key reference centred over the TUI. Esc/q closes it
    /// and returns to the same result list.
    fn render_help<W: Write>(&self, out: &mut W, p: &Palette) -> io::Result<()> {
        let title = t!("misc.gs.pager.help_title");
        let mut lines: Vec<String> = vec![title.clone()];
        lines.push("─".repeat(title.chars().count().max(20)));
        lines.extend(t!("misc.gs.pager.help_body").lines().map(str::to_string));
        lines.push(String::new());
        lines.push(t!("misc.gs.pager.help_close"));

        queue!(out, Clear(ClearType::All))?;
        let width = lines
            .iter()
            .map(|l| l.chars().count())
            .max()
            .unwrap_or(0)
            .min(self.term_w);
        let left = self.term_w.saturating_sub(width) / 2;
        let top = self.term_h.saturating_sub(lines.len()) / 2;
        for (i, line) in lines.iter().enumerate() {
            let row = top + i;
            if row >= self.term_h {
                break;
            }
            let shown = Self::truncate_chars(line, width);
            queue!(out, MoveTo(left as u16, row as u16))?;
            set_bg(out, p.prompt_bg)?;
            set_fg(out, p.prompt_fg)?;
            queue!(
                out,
                Print(shown),
                ResetColor,
                Clear(ClearType::UntilNewLine)
            )?;
        }
        Ok(())
    }

    fn render<W: Write>(&mut self, out: &mut W) -> io::Result<()> {
        let p = Palette::current();
        queue!(out, Clear(ClearType::All))?;

        if self.show_help {
            self.render_help(out, &p)?;
            out.flush()?;
            return Ok(());
        }

        let editing = self.pattern_buf.is_some();
        if editing || self.busy {
            if !self.matches.is_empty() {
                self.render_results(out, &p)?;
            }
            if self.busy {
                self.render_center(out, &t!("misc.gs.pager.searching"), self.term_h / 2)?;
            } else {
                self.render_pattern_box(out, &p)?;
            }
            let hint = if self.busy {
                self.list_hint()
            } else {
                self.edit_hint()
            };
            self.render_hint(out, &p, &hint)?;
            out.flush()?;
            return Ok(());
        }

        if self.matches.is_empty() {
            self.render_header(out, &p)?;
            self.render_center(out, &t!("cmd.gs.no_matches"), self.term_h / 2)?;
            self.render_hint(out, &p, &self.list_hint())?;
            out.flush()?;
            return Ok(());
        }

        self.render_results(out, &p)?;
        self.render_hint(out, &p, &self.list_hint())?;
        out.flush()?;
        Ok(())
    }
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

/// Starting context: explicit `-A`/`-B` values (even `0/0`) win; otherwise the
/// pager's own default applies.
fn initial_context(request: &SearchRequest) -> (usize, usize) {
    if request.context_requested {
        (
            request.context.before.min(MAX_CONTEXT),
            request.context.after.min(MAX_CONTEXT),
        )
    } else {
        (DEFAULT_CONTEXT, DEFAULT_CONTEXT)
    }
}

fn normalize_pattern(buf: &str) -> Option<String> {
    let trimmed = buf.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

pub fn run_gs_interactive(
    matches: Vec<ContentMatch>,
    request: &SearchRequest,
    pattern: &str,
) -> Result<bool> {
    let mut stdout = stdout();
    let _guard = TerminalGuard;

    execute!(stdout, EnterAlternateScreen, Hide, EnableMouseCapture)?;
    terminal::enable_raw_mode()?;

    // Re-query terminal size after entering alternate screen
    let (w, h) = terminal_size();
    let (ctx_before, ctx_after) = initial_context(request);
    let filtered: Vec<usize> = (0..matches.len()).collect();
    let mut gs = GsInteractive {
        matches,
        filtered,
        selected: 0,
        term_h: h,
        term_w: w,
        file_cache: HashMap::new(),
        pattern: pattern.to_string(),
        request: SearchRequest {
            root: request.root.clone(),
            max_depth: request.max_depth,
            paths: request.paths.clone(),
            sort: request.sort,
            context: request.context,
            context_requested: request.context_requested,
            opts: request.opts,
        },
        ctx_before,
        ctx_after,
        context_prompt: None,
        // `gs -I` with no pattern opens straight into the pattern editor.
        pattern_buf: if pattern.is_empty() {
            Some(String::new())
        } else {
            None
        },
        busy: false,
        show_help: false,
    };

    gs.ensure_visible();
    gs.render(&mut stdout)?;

    let result = loop {
        if !event::poll(std::time::Duration::from_millis(100))? {
            continue;
        }
        let ev = event::read()?;
        match ev {
            Event::Key(ke) => {
                if ke.kind != KeyEventKind::Press {
                    continue;
                }
                let ctrl_or_alt = ke
                    .modifiers
                    .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT);

                // The `?` overlay swallows every key except the ones that
                // close it; the result list underneath is untouched.
                if gs.show_help {
                    if matches!(
                        ke.code,
                        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('?')
                    ) {
                        gs.show_help = false;
                    }
                    gs.render(&mut stdout)?;
                    continue;
                }

                // Pattern editor: typing edits the buffer, Enter searches.
                if gs.pattern_buf.is_some() {
                    match ke.code {
                        KeyCode::Esc => {
                            gs.pattern_buf = None;
                            if gs.pattern.is_empty() && gs.matches.is_empty() {
                                break Ok(false);
                            }
                        }
                        KeyCode::Enter => {
                            let buf = gs.pattern_buf.take().unwrap_or_default();
                            match normalize_pattern(&buf) {
                                Some(pat) => {
                                    gs.busy = true;
                                    gs.render(&mut stdout)?;
                                    gs.search(&pat);
                                    gs.busy = false;
                                }
                                None => {
                                    if gs.pattern.is_empty() && gs.matches.is_empty() {
                                        break Ok(false);
                                    }
                                }
                            }
                        }
                        KeyCode::Backspace => {
                            if let Some(buf) = gs.pattern_buf.as_mut() {
                                buf.pop();
                            }
                        }
                        KeyCode::Char(ch) if !ctrl_or_alt => {
                            if let Some(buf) = gs.pattern_buf.as_mut() {
                                buf.push(ch);
                            }
                        }
                        _ => {}
                    }
                    gs.ensure_visible();
                    gs.render(&mut stdout)?;
                    continue;
                }

                // Context prompt (`c`): digits edit it, Enter applies.
                if gs.context_prompt.is_some() {
                    match ke.code {
                        KeyCode::Esc => gs.context_prompt = None,
                        KeyCode::Enter => gs.apply_context_prompt(),
                        KeyCode::Backspace => {
                            if let Some(buf) = gs.context_prompt.as_mut() {
                                buf.pop();
                            }
                        }
                        KeyCode::Char(ch) if ch.is_ascii_digit() => {
                            if let Some(buf) = gs.context_prompt.as_mut() {
                                if buf.len() < 2 {
                                    buf.push(ch);
                                }
                            }
                        }
                        _ => {}
                    }
                    gs.render(&mut stdout)?;
                    continue;
                }

                let mut redraw = false;
                let mut research = false;
                match ke.code {
                    KeyCode::Esc | KeyCode::Char('q') => break Ok(false),
                    KeyCode::Enter | KeyCode::Char('e') => {
                        let Some(m) = gs.selected_match() else {
                            continue;
                        };
                        let path = m.file_path.clone();
                        let line = m.line_number;
                        let pat = gs.pattern.clone();
                        drop(_guard);
                        // Flag editor to center + transiently highlight target line.
                        crate::shell::commands::builtin::ne::editor::GOTO_HIGHLIGHT
                            .store(true, std::sync::atomic::Ordering::SeqCst);
                        crate::shell::commands::builtin::ne::editor::set_goto_pattern(&pat);
                        // Open ntcEditor at the file and line
                        crate::shell::commands::builtin::ne::cmd_ne(
                            &format!("{}:{}", path.display(), line),
                            &mut Navigator::new()?,
                        )?;
                        break Ok(false);
                    }
                    KeyCode::Char('f') => {
                        gs.pattern_buf = Some(gs.pattern.clone());
                        redraw = true;
                    }
                    KeyCode::Char('c') => {
                        gs.context_prompt = Some(String::new());
                        redraw = true;
                    }
                    KeyCode::Char(ch) if ch.is_ascii_digit() => {
                        redraw = gs.jump_to_row(ch.to_digit(10).unwrap_or(0) as usize);
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if gs.selected > 0 {
                            gs.selected -= 1;
                        }
                        gs.ensure_visible();
                        redraw = true;
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if gs.selected + 1 < gs.total() {
                            gs.selected += 1;
                        }
                        gs.ensure_visible();
                        redraw = true;
                    }
                    KeyCode::PageUp | KeyCode::Char('p') | KeyCode::Left => {
                        gs.prev_page();
                        gs.ensure_visible();
                        redraw = true;
                    }
                    KeyCode::PageDown
                    | KeyCode::Char(' ')
                    | KeyCode::Char('n')
                    | KeyCode::Right => {
                        gs.next_page();
                        gs.ensure_visible();
                        redraw = true;
                    }
                    KeyCode::Home => {
                        gs.selected = 0;
                        gs.ensure_visible();
                        redraw = true;
                    }
                    KeyCode::End => {
                        let total = gs.total();
                        if total > 0 {
                            gs.selected = total - 1;
                        }
                        gs.ensure_visible();
                        redraw = true;
                    }
                    KeyCode::Char('?') => {
                        gs.show_help = true;
                        redraw = true;
                    }
                    KeyCode::Char('x') => {
                        gs.toggle_literal();
                        research = true;
                    }
                    KeyCode::Char('z') => {
                        gs.toggle_case();
                        research = true;
                    }
                    KeyCode::Char('h') => {
                        gs.toggle_mode(MatchMode::StartsWith);
                        research = true;
                    }
                    KeyCode::Char('g') => {
                        gs.toggle_mode(MatchMode::EndsWith);
                        research = true;
                    }
                    KeyCode::Char('w') => {
                        gs.toggle_mode(MatchMode::Whole);
                        research = true;
                    }
                    _ => {}
                }
                // A match toggle re-runs the current pattern so the list
                // always reflects the flags shown in the header.
                if research && !gs.pattern.is_empty() {
                    gs.busy = true;
                    gs.render(&mut stdout)?;
                    let pat = gs.pattern.clone();
                    gs.search(&pat);
                    gs.busy = false;
                    redraw = true;
                }
                if redraw {
                    gs.render(&mut stdout)?;
                }
            }
            Event::Resize(w, h) => {
                gs.term_w = w as usize;
                gs.term_h = h as usize;
                gs.ensure_visible();
                gs.render(&mut stdout)?;
            }
            Event::Mouse(m) => {
                let mut redraw = false;
                match m.kind {
                    crossterm::event::MouseEventKind::ScrollUp => {
                        if gs.selected > 0 {
                            gs.selected -= 1;
                            redraw = true;
                        }
                    }
                    crossterm::event::MouseEventKind::ScrollDown => {
                        if gs.selected + 1 < gs.total() {
                            gs.selected += 1;
                            redraw = true;
                        }
                    }
                    _ => {}
                }
                gs.ensure_visible();
                if redraw {
                    gs.render(&mut stdout)?;
                }
            }
            _ => {}
        }
    };

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(root: &Path) -> SearchRequest {
        SearchRequest {
            root: root.to_path_buf(),
            max_depth: 10,
            paths: Vec::new(),
            sort: None,
            context: ContextSpec {
                before: 0,
                after: 0,
            },
            context_requested: false,
            opts: MatchOptions::default(),
        }
    }

    fn match_at(path: &Path, line: usize) -> ContentMatch {
        ContentMatch {
            file_path: path.to_path_buf(),
            line_number: line,
            line: "hit".to_string(),
        }
    }

    fn pager(dir: &Path, matches: Vec<ContentMatch>) -> GsInteractive {
        let filtered = (0..matches.len()).collect();
        GsInteractive {
            matches,
            filtered,
            selected: 0,
            term_h: 40,
            term_w: 100,
            file_cache: HashMap::new(),
            pattern: "hit".to_string(),
            request: request(dir),
            ctx_before: DEFAULT_CONTEXT,
            ctx_after: DEFAULT_CONTEXT,
            context_prompt: None,
            pattern_buf: None,
            busy: false,
            show_help: false,
        }
    }

    #[test]
    fn context_prompt_sets_both_sides_and_clamps() {
        let dir = tempfile::tempdir().unwrap();
        let mut gs = pager(dir.path(), Vec::new());

        gs.context_prompt = Some("7".to_string());
        gs.apply_context_prompt();
        assert_eq!((gs.ctx_before, gs.ctx_after), (7, 7));
        assert!(gs.context_prompt.is_none());

        // Cap is 10, not 20.
        gs.context_prompt = Some("99".to_string());
        gs.apply_context_prompt();
        assert_eq!((gs.ctx_before, gs.ctx_after), (MAX_CONTEXT, MAX_CONTEXT));

        // Anything unparseable keeps the current context.
        gs.context_prompt = Some("x".to_string());
        gs.apply_context_prompt();
        assert_eq!((gs.ctx_before, gs.ctx_after), (MAX_CONTEXT, MAX_CONTEXT));
    }

    #[test]
    fn digits_jump_to_their_row_on_the_current_page() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("f.txt");
        let matches = (1..=25).map(|n| match_at(&file, n)).collect();
        let mut gs = pager(dir.path(), matches);

        assert!(gs.jump_to_row(3));
        assert_eq!(gs.selected, 3);

        // Page 2 starts at index 10.
        gs.selected = 12;
        assert!(gs.jump_to_row(5));
        assert_eq!(gs.selected, 15);

        // Last page has 5 rows (20..25): 20 + 9 is out of range.
        gs.selected = 20;
        assert!(!gs.jump_to_row(9));
        assert_eq!(gs.selected, 20);
        assert!(gs.jump_to_row(4));
        assert_eq!(gs.selected, 24);
    }

    #[test]
    fn initial_context_uses_flags_even_when_zero() {
        let dir = tempfile::tempdir().unwrap();
        let mut req = request(dir.path());
        assert_eq!(initial_context(&req), (DEFAULT_CONTEXT, DEFAULT_CONTEXT));

        req.context_requested = true;
        assert_eq!(initial_context(&req), (0, 0));

        req.context.before = 20;
        req.context.after = 3;
        assert_eq!(initial_context(&req), (MAX_CONTEXT, 3));
    }

    #[test]
    fn pattern_buffer_trims_and_rejects_empty() {
        assert_eq!(
            normalize_pattern("  main\\|wow "),
            Some("main\\|wow".into())
        );
        assert_eq!(normalize_pattern("   "), None);
        assert_eq!(normalize_pattern(""), None);
    }

    #[test]
    fn selected_row_foreground_differs_from_its_background() {
        let p = Palette::for_enabled(true);
        assert_ne!(p.selected_fg, p.selected_bg);
        assert!(p.hit_fg.is_some());

        let plain = Palette::for_enabled(false);
        assert!(plain.selected_fg.is_none());
        assert!(plain.selected_bg.is_none());
        assert!(plain.hit_fg.is_none());
    }

    fn contains_color_sgr(s: &str) -> bool {
        let mut rest = s;
        while let Some(start) = rest.find("\x1b[") {
            rest = &rest[start + 2..];
            let Some(end) = rest.find(|c: char| c.is_ascii_alphabetic()) else {
                break;
            };
            if &rest[end..end + 1] == "m" {
                let params = &rest[..end];
                if params.split(';').any(|p| {
                    p.parse::<u16>().is_ok_and(|n| {
                        matches!(
                            n,
                            30..=37 | 90..=97 | 40..=47 | 100..=107 | 38 | 39 | 48 | 49
                        )
                    })
                }) {
                    return true;
                }
            }
            rest = &rest[end + 1..];
        }
        false
    }

    #[test]
    fn render_emits_no_colour_when_disabled() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("f.txt");
        std::fs::write(&file, "a\nhit\nc\n").unwrap();
        let gs_matches = vec![match_at(&file, 2)];

        let mut cfg = crate::config::Config::new();
        cfg.color_enabled = false;
        crate::config::Config::with_test_config(cfg, || {
            let mut gs = pager(dir.path(), gs_matches);
            let mut out = Vec::new();
            gs.render(&mut out).unwrap();
            let rendered = String::from_utf8_lossy(&out);
            assert!(!contains_color_sgr(&rendered));
        });
    }

    #[test]
    fn help_overlay_replaces_the_list_and_returns_to_it() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("f.txt");
        std::fs::write(&file, "hit\n").unwrap();
        let mut gs = pager(dir.path(), vec![match_at(&file, 1)]);
        gs.show_help = true;

        let mut cfg = crate::config::Config::new();
        cfg.color_enabled = false;
        crate::config::Config::with_test_config(cfg, || {
            let mut out = Vec::new();
            gs.render(&mut out).unwrap();
            let rendered = String::from_utf8_lossy(&out);
            assert!(rendered.contains("GS INTERACTIVE"));
            assert!(rendered.contains("x  literal"));
            // Overlay, not the result list.
            assert!(!rendered.contains("gs: 1/1 matches"));

            // Esc/q clears the flag and the list comes back unchanged.
            gs.show_help = false;
            let mut out = Vec::new();
            gs.render(&mut out).unwrap();
            let rendered = String::from_utf8_lossy(&out);
            assert!(rendered.contains("gs: 1/1 matches"));
            assert!(!rendered.contains("GS INTERACTIVE"));
        });
    }

    #[test]
    fn flags_label_and_toggles_track_the_request() {
        let dir = tempfile::tempdir().unwrap();
        let mut gs = pager(dir.path(), Vec::new());
        assert_eq!(gs.flags(), "");

        gs.toggle_literal();
        gs.toggle_case();
        gs.toggle_mode(MatchMode::StartsWith);
        assert_eq!(gs.flags(), "-h -x -z");

        // `h`/`g`/`w` stay exclusive, and pressing the active one clears it
        // back to the plain substring mode.
        gs.toggle_mode(MatchMode::EndsWith);
        assert_eq!(gs.request.opts.mode, MatchMode::EndsWith);
        gs.toggle_mode(MatchMode::EndsWith);
        assert_eq!(gs.request.opts.mode, MatchMode::Contains);
        assert_eq!(gs.flags(), "-x -z");
    }

    #[test]
    fn empty_launch_shows_pattern_box_and_hint() {
        let dir = tempfile::tempdir().unwrap();
        let mut gs = pager(dir.path(), Vec::new());
        gs.pattern.clear();
        gs.pattern_buf = Some(String::new());

        let mut cfg = crate::config::Config::new();
        cfg.color_enabled = false;
        crate::config::Config::with_test_config(cfg, || {
            let mut out = Vec::new();
            gs.render(&mut out).unwrap();
            let rendered = String::from_utf8_lossy(&out);
            assert!(rendered.contains("pattern:"));
            assert!(!rendered.contains("gs: 0 of 0"));
        });
    }
}
