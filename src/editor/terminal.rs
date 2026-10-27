use std::io::{Read, Write};
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

use crate::t;
use anyhow::Result;

pub(crate) enum TermMessage {
    Output(Vec<u8>),
}

/// A terminal grid cell. `fg`/`bg` are `None` when the child process
/// emitted no explicit color (transparent passthrough): the renderer
/// must not paint any color so the embedded ntc shell looks exactly
/// like the standalone ntc shell in the user's own terminal.
#[derive(Clone, Default, PartialEq, Eq)]
pub(crate) struct TermCell {
    pub ch: char,
    pub fg: Option<(u8, u8, u8)>,
    pub bg: Option<(u8, u8, u8)>,
    /// SGR bold (`\x1b[1m`) active for this cell. The ntc shell emits
    /// bold via `paint_ansi_bold()` / `colored.bold()`; without this the
    /// panel renders those spans in dark/dull variants.
    pub bold: bool,
}

pub(crate) struct TerminalState {
    pub lines: Vec<Vec<TermCell>>,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub width: usize,
    pub height: usize,
    pub rx: mpsc::Receiver<TermMessage>,
    writer: Option<Box<dyn Write + Send>>,
    _child: Option<Box<dyn portable_pty::Child>>,
    pty: Option<Box<dyn portable_pty::MasterPty>>,
    pub exited: bool,
    pub exit_code: Option<i32>,
    pub title: String,
    pub scrollback: Vec<Vec<TermCell>>,
    pub follow_mode: bool,
    pub dirty: bool,
    pub cursor_visible: bool,
    /// App-requested cursor hide (`CSI ? 25 l`), synced from the performer.
    pub app_cursor_hidden: bool,
    /// Last observed alt-screen state; used to drop stale selections and
    /// scroll offsets on fullscreen-app enter/exit.
    was_alt: bool,
    pub last_blink: Instant,
    pub selection_anchor: Option<(usize, usize)>,
    pub selection_active: Option<(usize, usize)>,
    /// True while the embedded shell is executing a command (reported via
    /// the private `OSC 1337;ntc-busy` / `ntc-idle` markers). Esc keeps the
    /// panel visible while this is set.
    pub busy: bool,
    parser: vte::Parser,
    performer: VtePerformer,
}

struct VtePerformer {
    cursor_x: usize,
    cursor_y: usize,
    width: usize,
    height: usize,
    /// Active grid: the main grid, or the alternate screen while a
    /// fullscreen app (DECSET 1047/1049, e.g. opencode) is running.
    lines: Vec<Vec<TermCell>>,
    /// Stashed main grid while the alternate screen is active.
    main_lines: Option<Vec<Vec<TermCell>>>,
    /// Saved main-grid cursor for 1048/1049 restore.
    saved_cursor: (usize, usize),
    current_fg: Option<(u8, u8, u8)>,
    current_bg: Option<(u8, u8, u8)>,
    current_bold: bool,
    /// App-requested cursor hide (`CSI ? 25 l`).
    app_cursor_hidden: bool,
    scrollback: Vec<Vec<TermCell>>,
    /// Set by the embedded shell around each command (`OSC 1337`).
    busy: bool,
}

impl Default for VtePerformer {
    fn default() -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            width: 80,
            height: 24,
            lines: vec![vec![TermCell::default(); 80]; 24],
            main_lines: None,
            saved_cursor: (0, 0),
            // Transparent passthrough: no forced colors.
            current_fg: None,
            current_bg: None,
            current_bold: false,
            app_cursor_hidden: false,
            scrollback: Vec::new(),
            busy: false,
        }
    }
}

impl VtePerformer {
    fn alt_active(&self) -> bool {
        self.main_lines.is_some()
    }

    /// Switch to the alternate screen. With `save_cursor` (1049) the
    /// main-grid cursor is stashed for restore; 1047 skips that.
    fn enter_alt(&mut self) {
        if self.main_lines.is_some() {
            return;
        }
        let cleared = vec![vec![TermCell::default(); self.width]; self.height];
        self.main_lines = Some(std::mem::replace(&mut self.lines, cleared));
        self.cursor_x = 0;
        self.cursor_y = 0;
    }

    /// Leave the alternate screen, discarding it. With `restore_cursor`
    /// (1049/1048) the stashed main-grid cursor is restored.
    fn exit_alt(&mut self, restore_cursor: bool) {
        if let Some(main) = self.main_lines.take() {
            self.lines = main;
            if restore_cursor {
                self.cursor_x = self.saved_cursor.0.min(self.width.saturating_sub(1));
                self.cursor_y = self.saved_cursor.1.min(self.height.saturating_sub(1));
            }
        }
    }
}

impl vte::Perform for VtePerformer {
    fn print(&mut self, ch: char) {
        if self.cursor_x < self.width && self.cursor_y < self.height {
            self.lines[self.cursor_y][self.cursor_x] = TermCell {
                ch,
                fg: self.current_fg,
                bg: self.current_bg,
                bold: self.current_bold,
            };
            self.cursor_x += 1;
        }
    }

    fn execute(&mut self, byte: u8) {
        match byte {
            b'\r' => self.cursor_x = 0,
            b'\n' => {
                self.cursor_y += 1;
                if self.cursor_y >= self.height {
                    self.scroll_up();
                    self.cursor_y = self.height - 1;
                }
            }
            b'\x08' if self.cursor_x > 0 => {
                self.cursor_x -= 1;
            }
            b'\t' => {
                self.cursor_x = (self.cursor_x + 8) & !7;
                if self.cursor_x >= self.width {
                    self.cursor_x = self.width - 1;
                }
            }
            b'\x07' => {}
            _ => {}
        }
    }

    fn csi_dispatch(
        &mut self,
        params: &vte::Params,
        intermediates: &[u8],
        _ignore: bool,
        action: char,
    ) {
        // DEC private modes (`CSI ? ... h/l`): alternate screen for
        // fullscreen TUIs (opencode), app cursor hide/show. All other
        // private modes (mouse tracking, bracketed paste, …) are parsed
        // and ignored — the editor handles mouse itself.
        if intermediates == [b'?'] && (action == 'h' || action == 'l') {
            let enable = action == 'h';
            for p in params.iter() {
                match p[0] {
                    25 => self.app_cursor_hidden = !enable,
                    // 1047: alt screen without save; 1049: with save.
                    1047 => {
                        if enable {
                            self.enter_alt();
                        } else {
                            self.exit_alt(false);
                        }
                    }
                    1048 => {
                        if enable {
                            self.saved_cursor = (self.cursor_x, self.cursor_y);
                        } else {
                            self.cursor_x = self.saved_cursor.0.min(self.width.saturating_sub(1));
                            self.cursor_y = self.saved_cursor.1.min(self.height.saturating_sub(1));
                        }
                    }
                    1049 => {
                        if enable {
                            self.saved_cursor = (self.cursor_x, self.cursor_y);
                            self.enter_alt();
                        } else {
                            self.exit_alt(true);
                        }
                    }
                    _ => {}
                }
            }
            return;
        }
        let pv: Vec<u16> = params.iter().map(|p| p[0]).collect();
        match action {
            'A' => {
                let n = pv.first().copied().unwrap_or(1) as usize;
                self.cursor_y = self.cursor_y.saturating_sub(n);
            }
            'B' => {
                let n = pv.first().copied().unwrap_or(1) as usize;
                self.cursor_y = (self.cursor_y + n).min(self.height - 1);
            }
            'C' => {
                let n = pv.first().copied().unwrap_or(1) as usize;
                self.cursor_x = (self.cursor_x + n).min(self.width - 1);
            }
            'D' => {
                let n = pv.first().copied().unwrap_or(1) as usize;
                self.cursor_x = self.cursor_x.saturating_sub(n);
            }
            'H' | 'f' => {
                let row = pv.first().copied().unwrap_or(1) as usize;
                let col = pv.get(1).copied().unwrap_or(1) as usize;
                let max_row = self.height.saturating_sub(1);
                let max_col = self.width.saturating_sub(1);
                self.cursor_y = row.saturating_sub(1).min(max_row);
                self.cursor_x = col.saturating_sub(1).min(max_col);
            }
            'J' => {
                let mode = pv.first().copied().unwrap_or(0);
                match mode {
                    0 => {
                        for x in self.cursor_x..self.width {
                            self.lines[self.cursor_y][x] = TermCell::default();
                        }
                        for y in (self.cursor_y + 1)..self.height {
                            self.lines[y].fill(TermCell::default());
                        }
                    }
                    1 => {
                        for y in 0..self.cursor_y {
                            self.lines[y].fill(TermCell::default());
                        }
                        for x in 0..=self.cursor_x {
                            self.lines[self.cursor_y][x] = TermCell::default();
                        }
                    }
                    2 | 3 => {
                        self.lines.fill(vec![TermCell::default(); self.width]);
                        // Never let a fullscreen app wipe the main scrollback.
                        if !self.alt_active() {
                            self.scrollback.clear();
                            self.cursor_x = 0;
                            self.cursor_y = 0;
                        }
                    }
                    _ => {}
                }
            }
            'K' => {
                let mode = pv.first().copied().unwrap_or(0);
                match mode {
                    0 => {
                        for x in self.cursor_x..self.width {
                            self.lines[self.cursor_y][x] = TermCell::default();
                        }
                    }
                    1 => {
                        for x in 0..=self.cursor_x {
                            self.lines[self.cursor_y][x] = TermCell::default();
                        }
                    }
                    2 => {
                        self.lines[self.cursor_y].fill(TermCell::default());
                    }
                    _ => {}
                }
            }
            'S' => {
                let n = pv.first().copied().unwrap_or(1) as usize;
                for _ in 0..n {
                    self.scroll_up();
                }
            }
            'T' => {
                let n = pv.first().copied().unwrap_or(1) as usize;
                for _ in 0..n {
                    self.scroll_down();
                }
            }
            'm' => {
                let mut i = 0;
                while i < pv.len() {
                    match pv[i] {
                        // Reset → transparent passthrough (no forced colors).
                        0 => {
                            self.current_fg = None;
                            self.current_bg = None;
                            self.current_bold = false;
                        }
                        // Bold on/off (ntc shell `paint_ansi_bold` / `.bold()`).
                        1 => {
                            self.current_bold = true;
                        }
                        22 => {
                            self.current_bold = false;
                        }
                        30..=37 => {
                            let idx = (pv[i] - 30) as usize;
                            let colors = [
                                (0, 0, 0),
                                (170, 0, 0),
                                (0, 170, 0),
                                (170, 85, 0),
                                (0, 0, 170),
                                (170, 0, 170),
                                (0, 170, 170),
                                (170, 170, 170),
                            ];
                            self.current_fg = Some(colors[idx]);
                        }
                        // Default foreground → passthrough.
                        39 => {
                            self.current_fg = None;
                        }
                        40..=47 => {
                            let idx = (pv[i] - 40) as usize;
                            let colors = [
                                (0, 0, 0),
                                (170, 0, 0),
                                (0, 170, 0),
                                (170, 85, 0),
                                (0, 0, 170),
                                (170, 0, 170),
                                (0, 170, 170),
                                (170, 170, 170),
                            ];
                            self.current_bg = Some(colors[idx]);
                        }
                        // Default background → passthrough.
                        49 => {
                            self.current_bg = None;
                        }
                        90..=97 => {
                            let idx = (pv[i] - 90) as usize;
                            let colors = [
                                (85, 85, 85),
                                (255, 85, 85),
                                (85, 255, 85),
                                (255, 255, 85),
                                (85, 85, 255),
                                (255, 85, 255),
                                (85, 255, 255),
                                (255, 255, 255),
                            ];
                            self.current_fg = Some(colors[idx]);
                        }
                        100..=107 => {
                            let idx = (pv[i] - 100) as usize;
                            let colors = [
                                (85, 85, 85),
                                (255, 85, 85),
                                (85, 255, 85),
                                (255, 255, 85),
                                (85, 85, 255),
                                (255, 85, 255),
                                (85, 255, 255),
                                (255, 255, 255),
                            ];
                            self.current_bg = Some(colors[idx]);
                        }
                        38 if i + 2 < pv.len() && pv[i + 1] == 5 => {
                            self.current_fg = Some(ansi256_to_rgb(pv[i + 2] as u8));
                            i += 2;
                        }
                        // Truecolor `38;2;r;g;b` (e.g. theme `Rgb` tree colors).
                        38 if i + 4 < pv.len() && pv[i + 1] == 2 => {
                            self.current_fg =
                                Some((pv[i + 2] as u8, pv[i + 3] as u8, pv[i + 4] as u8));
                            i += 4;
                        }
                        48 if i + 2 < pv.len() && pv[i + 1] == 5 => {
                            self.current_bg = Some(ansi256_to_rgb(pv[i + 2] as u8));
                            i += 2;
                        }
                        // Truecolor `48;2;r;g;b` background.
                        48 if i + 4 < pv.len() && pv[i + 1] == 2 => {
                            self.current_bg =
                                Some((pv[i + 2] as u8, pv[i + 3] as u8, pv[i + 4] as u8));
                            i += 4;
                        }
                        _ => {}
                    }
                    i += 1;
                }
            }
            _ => {}
        }
    }

    /// `OSC 1337;ntc-busy` / `OSC 1337;ntc-idle` come from the embedded
    /// ntc shell and mark whether a command is currently running.
    fn osc_dispatch(&mut self, params: &[&[u8]], _bell_terminated: bool) {
        if params.first() != Some(&b"1337".as_slice()) {
            return;
        }
        match params.get(1) {
            Some(&b"ntc-busy") => self.busy = true,
            Some(&b"ntc-idle") => self.busy = false,
            _ => {}
        }
    }
    fn hook(&mut self, _params: &vte::Params, _intermediates: &[u8], _ignore: bool, _action: char) {
    }
    fn put(&mut self, _byte: u8) {}
    fn unhook(&mut self) {}
}

impl VtePerformer {
    fn scroll_up(&mut self) {
        if !self.lines.is_empty() {
            let first = self.lines.remove(0);
            // The alternate screen has no scrollback: scrolled-off lines
            // are discarded so TUIs can't pollute the main history.
            if !self.alt_active() {
                self.scrollback.push(first);
            }
            self.lines.push(vec![TermCell::default(); self.width]);
        }
    }

    fn scroll_down(&mut self) {
        if !self.scrollback.is_empty() {
            let last = self.scrollback.pop().unwrap();
            self.lines.insert(0, last);
            if self.lines.len() > self.height {
                self.lines.pop();
            }
        }
    }
}

fn ansi256_to_rgb(idx: u8) -> (u8, u8, u8) {
    if idx < 16 {
        let colors = [
            (0, 0, 0),
            (128, 0, 0),
            (0, 128, 0),
            (128, 128, 0),
            (0, 0, 128),
            (128, 0, 128),
            (0, 128, 128),
            (192, 192, 192),
            (128, 128, 128),
            (255, 0, 0),
            (0, 255, 0),
            (255, 255, 0),
            (0, 0, 255),
            (255, 0, 255),
            (0, 255, 255),
            (255, 255, 255),
        ];
        colors[idx as usize]
    } else if idx < 232 {
        let idx = idx - 16;
        let r = idx / 36;
        let g = (idx % 36) / 6;
        let b = idx % 6;
        (
            if r == 0 { 0 } else { 55 + r * 40 },
            if g == 0 { 0 } else { 55 + g * 40 },
            if b == 0 { 0 } else { 55 + b * 40 },
        )
    } else {
        let gray = (idx - 232) * 10 + 8;
        (gray, gray, gray)
    }
}

impl TerminalState {
    /// Spawn the embedded terminal running the ntc shell only
    /// (`std::env::current_exe()` with no args → logo + interactive shell).
    /// Foreign shells (powershell / sh / fish / zsh) are never used.
    pub fn spawn_ntc_shell(width: u16, height: u16, cwd: &Path) -> Result<Self> {
        let exe = std::env::current_exe()?;
        Self::spawn_cmd(
            exe.to_string_lossy().as_ref(),
            &[],
            width,
            height,
            cwd,
            "ntc",
        )
    }

    fn spawn_cmd(
        prog: &str,
        args: &[&str],
        width: u16,
        height: u16,
        cwd: &Path,
        title_shell: &str,
    ) -> Result<Self> {
        let pty_system = portable_pty::native_pty_system();

        let pair = pty_system.openpty(portable_pty::PtySize {
            rows: height,
            cols: width,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        let mut cmd = portable_pty::CommandBuilder::new(prog);
        cmd.args(args);
        cmd.cwd(cwd);
        // Mark the child as running inside the editor: the embedded ntc
        // shell skips session-directory restore/save and refuses nested
        // `ne` / `ntceditor` invocations when this is set.
        cmd.env("NTC_IN_EDITOR", "1");
        // Deterministic child rendering: without an explicit TERM the
        // shell can degrade to ASCII-fallback boxes / plain output that
        // never appear in the standalone shell.
        let term_unset = std::env::var("TERM")
            .map(|t| t.is_empty() || t == "dumb")
            .unwrap_or(true);
        if term_unset {
            cmd.env("TERM", "xterm-256color");
        }

        let child = pair.slave.spawn_command(cmd)?;

        let mut reader = pair.master.try_clone_reader()?;
        let writer = pair.master.take_writer()?;

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let mut buf = [0u8; 4096];
            loop {
                match reader.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        let data = buf[..n].to_vec();
                        if tx.send(TermMessage::Output(data)).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        let performer = VtePerformer {
            width: width as usize,
            height: height as usize,
            lines: vec![vec![TermCell::default(); width as usize]; height as usize],
            ..Default::default()
        };

        let title_cwd = cwd
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| cwd.display().to_string());

        Ok(Self {
            lines: performer.lines.clone(),
            cursor_x: 0,
            cursor_y: 0,
            width: width as usize,
            height: height as usize,
            rx,
            writer: Some(Box::new(writer)),
            _child: Some(child),
            pty: Some(pair.master),
            exited: false,
            exit_code: None,
            title: format!(
                "{} {} {} {} {}",
                t!("editor.term_title"),
                "\u{2014}",
                title_shell,
                "\u{2014}",
                title_cwd
            ),
            scrollback: Vec::new(),
            follow_mode: true,
            dirty: true,
            cursor_visible: true,
            app_cursor_hidden: false,
            was_alt: false,
            last_blink: Instant::now(),
            selection_anchor: None,
            selection_active: None,
            busy: false,
            parser: vte::Parser::new(),
            performer,
        })
    }

    pub fn poll_output(&mut self) {
        let mut had_output = false;
        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                TermMessage::Output(data) => {
                    for byte in &data {
                        self.parser.advance(&mut self.performer, *byte);
                    }
                    had_output = true;
                }
            }
        }
        if had_output {
            self.lines = self.performer.lines.clone();
            self.cursor_x = self.performer.cursor_x;
            self.cursor_y = self.performer.cursor_y;
            self.scrollback = self.performer.scrollback.clone();
            self.app_cursor_hidden = self.performer.app_cursor_hidden;
            self.busy = self.performer.busy;
            // Fullscreen-app enter/exit invalidates scroll + selection
            // (alt grid has no scrollback; indices don't transfer).
            let is_alt = self.performer.alt_active();
            if is_alt != self.was_alt {
                self.was_alt = is_alt;
                self.selection_anchor = None;
                self.selection_active = None;
                self.follow_mode = true;
            }
            self.dirty = true;
        }
    }

    /// True while a fullscreen app owns the alternate screen.
    pub fn is_alt_screen(&self) -> bool {
        self.performer.alt_active()
    }

    /// True while the embedded shell is running a command.
    pub fn is_busy(&self) -> bool {
        self.busy
    }

    /// Absolute cursor line index in `total_lines()`/`get_line()` space:
    /// grid-relative on the alternate screen (no scrollback there),
    /// scrollback-relative otherwise.
    pub fn cursor_abs_y(&self) -> usize {
        if self.is_alt_screen() {
            self.cursor_y
        } else {
            self.scrollback.len() + self.cursor_y
        }
    }

    pub fn send_input(&mut self, data: &[u8]) {
        if let Some(ref mut writer) = self.writer {
            let _ = writer.write_all(data);
            let _ = writer.flush();
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width as usize;
        self.height = height as usize;
        self.performer.width = width as usize;
        self.performer.height = height as usize;
        Self::resize_grid(&mut self.performer.lines, width as usize, height as usize);
        // A stashed main grid must track the size too so alt-exit
        // restores a correctly shaped grid.
        if let Some(ref mut main) = self.performer.main_lines {
            Self::resize_grid(main, width as usize, height as usize);
        }
        self.lines = self.performer.lines.clone();
        self.dirty = true;

        if let Some(ref pty) = self.pty {
            let _ = pty.resize(portable_pty::PtySize {
                rows: height,
                cols: width,
                pixel_width: 0,
                pixel_height: 0,
            });
        }
    }

    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }

    pub fn tick_cursor(&mut self, blink_rate_ms: u64) {
        if blink_rate_ms > 0 && self.last_blink.elapsed().as_millis() as u64 >= blink_rate_ms {
            self.cursor_visible = !self.cursor_visible;
            self.last_blink = Instant::now();
            self.dirty = true;
        }
    }

    /// Adjust horizontal scroll offset so cursor_x stays within visible width.
    pub fn ensure_cursor_visible_x(&self, scroll_x: &mut usize, visible_width: usize) {
        let max_scroll = self.width.saturating_sub(visible_width);
        if self.cursor_x < *scroll_x {
            *scroll_x = self.cursor_x;
        } else if *scroll_x > 0 && self.cursor_x >= *scroll_x + visible_width {
            *scroll_x = self.cursor_x.saturating_sub(visible_width) + 1;
        }
        *scroll_x = (*scroll_x).min(max_scroll);
    }

    fn resize_grid(grid: &mut Vec<Vec<TermCell>>, width: usize, height: usize) {
        for line in grid.iter_mut() {
            line.resize(width, TermCell::default());
        }
        grid.resize(height, vec![TermCell::default(); width]);
    }

    /// Total number of lines in the scrollback buffer + visible screen.
    /// On the alternate screen there is no scrollback: just the grid.
    pub fn total_lines(&self) -> usize {
        if self.is_alt_screen() {
            self.lines.len()
        } else {
            self.scrollback.len() + self.lines.len()
        }
    }

    /// Get a line by index from the combined scrollback + screen buffer.
    /// Index 0 = oldest scrollback line, total_lines()-1 = bottom of screen.
    /// On the alternate screen the index addresses the grid directly.
    pub fn get_line(&self, idx: usize) -> &Vec<TermCell> {
        if self.is_alt_screen() {
            return self
                .lines
                .get(idx)
                .unwrap_or_else(|| &self.lines[self.lines.len().saturating_sub(1)]);
        }
        let sb_len = self.scrollback.len();
        if idx < sb_len {
            &self.scrollback[idx]
        } else {
            let screen_idx = idx - sb_len;
            if screen_idx < self.lines.len() {
                &self.lines[screen_idx]
            } else {
                &self.lines[self.lines.len().saturating_sub(1)]
            }
        }
    }

    pub fn has_selection(&self) -> bool {
        self.selection_anchor.is_some() && self.selection_active.is_some()
    }

    pub fn clear_selection(&mut self) {
        self.selection_anchor = None;
        self.selection_active = None;
        self.dirty = true;
    }

    pub fn selection_range(&self) -> Option<((usize, usize), (usize, usize))> {
        match (self.selection_anchor, self.selection_active) {
            (Some(a), Some(b)) => {
                let start = if a <= b { a } else { b };
                let end = if a <= b { b } else { a };
                Some((start, end))
            }
            _ => None,
        }
    }

    /// Extract text within the selection range.
    pub fn selected_text(&self) -> Option<String> {
        let ((sy, sx), (ey, ex)) = self.selection_range()?;
        let mut result = String::new();
        if sy == ey {
            let line = self.get_line(sy);
            let s: String = line
                .iter()
                .skip(sx)
                .take(ex.saturating_sub(sx))
                .map(|c| c.ch)
                .collect();
            result.push_str(&s);
        } else {
            // First line
            let first = self.get_line(sy);
            let s: String = first.iter().skip(sx).map(|c| c.ch).collect();
            result.push_str(s.trim_end());
            result.push('\n');
            // Middle lines
            for ly in (sy + 1)..ey {
                let line = self.get_line(ly);
                let s: String = line.iter().map(|c| c.ch).collect();
                result.push_str(s.trim_end());
                result.push('\n');
            }
            // Last line
            let last = self.get_line(ey);
            let s: String = last.iter().take(ex).map(|c| c.ch).collect();
            result.push_str(s.trim_end());
        }
        Some(result)
    }

    /// Copy visible text as a string (for clipboard).
    pub fn visible_text(&self, scroll: usize, panel_h: usize) -> String {
        let total = self.total_lines();
        let mut result = String::new();
        for i in 0..panel_h {
            let idx = scroll + i;
            if idx >= total {
                break;
            }
            let line = self.get_line(idx);
            let s: String = line.iter().map(|c| c.ch).collect();
            let trimmed = s.trim_end();
            result.push_str(trimmed);
            result.push('\n');
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn feed(p: &mut VtePerformer, parser: &mut vte::Parser, s: &[u8]) {
        for b in s {
            parser.advance(p, *b);
        }
    }

    #[test]
    fn truecolor_fg_parses() {
        let mut p = VtePerformer::default();
        let mut parser = vte::Parser::new();
        // Theme `Rgb(30,220,220)` tree-branch teal, as emitted by the shell.
        feed(&mut p, &mut parser, b"\x1b[38;2;30;220;220mA");
        assert_eq!(p.lines[0][0].fg, Some((30, 220, 220)));
        assert_eq!(p.lines[0][0].ch, 'A');
    }

    #[test]
    fn truecolor_bg_parses() {
        let mut p = VtePerformer::default();
        let mut parser = vte::Parser::new();
        feed(&mut p, &mut parser, b"\x1b[48;2;1;2;3mA");
        assert_eq!(p.lines[0][0].bg, Some((1, 2, 3)));
    }

    #[test]
    fn bold_sets_and_reset_clears() {
        let mut p = VtePerformer::default();
        let mut parser = vte::Parser::new();
        feed(&mut p, &mut parser, b"\x1b[1mB");
        assert!(p.lines[0][0].bold);
        assert_eq!(p.lines[0][0].ch, 'B');
        // SGR 0 clears bold + colors back to passthrough.
        feed(&mut p, &mut parser, b"\x1b[0m");
        assert!(!p.current_bold);
        assert_eq!(p.current_fg, None);
        assert_eq!(p.current_bg, None);
    }

    #[test]
    fn default_reset_codes_are_passthrough() {
        let mut p = VtePerformer::default();
        let mut parser = vte::Parser::new();
        feed(&mut p, &mut parser, b"\x1b[31m\x1b[39m\x1b[49mA");
        assert_eq!(p.lines[0][0].fg, None);
        assert_eq!(p.lines[0][0].bg, None);
    }

    #[test]
    fn alt_screen_isolates_and_restores() {
        let mut p = VtePerformer::default();
        let mut parser = vte::Parser::new();
        feed(&mut p, &mut parser, b"main");
        assert!(!p.alt_active());
        // Enter alt (1049): grid cleared, main stashed.
        feed(&mut p, &mut parser, b"\x1b[?1049h");
        assert!(p.alt_active());
        assert_eq!(p.lines[0][0].ch, '\0');
        feed(&mut p, &mut parser, b"tui");
        assert_eq!(p.lines[0][0].ch, 't');
        // Scrolling on alt must not grow the main scrollback.
        for _ in 0..30 {
            feed(&mut p, &mut parser, b"\n");
        }
        assert!(p.scrollback.is_empty());
        // Exit alt: main grid restored, alt discarded.
        feed(&mut p, &mut parser, b"\x1b[?1049l");
        assert!(!p.alt_active());
        assert_eq!(p.lines[0][0].ch, 'm');
    }

    #[test]
    fn alt_clear_does_not_wipe_scrollback() {
        let mut p = VtePerformer::default();
        let mut parser = vte::Parser::new();
        for _ in 0..30 {
            feed(&mut p, &mut parser, b"x\n");
        }
        assert!(!p.scrollback.is_empty());
        let sb_len = p.scrollback.len();
        feed(&mut p, &mut parser, b"\x1b[?1049h\x1b[2J\x1b[?1049l");
        assert_eq!(p.scrollback.len(), sb_len);
    }

    #[test]
    fn shell_busy_markers_toggle_busy() {
        let mut p = VtePerformer::default();
        let mut parser = vte::Parser::new();
        assert!(!p.busy);
        feed(&mut p, &mut parser, b"\x1b]1337;ntc-busy\x07");
        assert!(p.busy);
        feed(&mut p, &mut parser, b"\x1b]1337;ntc-idle\x07");
        assert!(!p.busy);
        // Other OSC titles/params must not touch the flag.
        feed(&mut p, &mut parser, b"\x1b]1337;ntc-busy\x07");
        feed(&mut p, &mut parser, b"\x1b]0;some title\x07");
        assert!(p.busy);
    }

    #[test]
    fn app_cursor_hide_show() {
        let mut p = VtePerformer::default();
        let mut parser = vte::Parser::new();
        assert!(!p.app_cursor_hidden);
        feed(&mut p, &mut parser, b"\x1b[?25l");
        assert!(p.app_cursor_hidden);
        feed(&mut p, &mut parser, b"\x1b[?25h");
        assert!(!p.app_cursor_hidden);
    }
}
