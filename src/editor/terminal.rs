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

#[derive(Clone, Default)]
pub(crate) struct TermCell {
    pub ch: char,
    pub fg: (u8, u8, u8),
    pub bg: (u8, u8, u8),
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
    pub last_blink: Instant,
    pub selection_anchor: Option<(usize, usize)>,
    pub selection_active: Option<(usize, usize)>,
    parser: vte::Parser,
    performer: VtePerformer,
}

struct VtePerformer {
    cursor_x: usize,
    cursor_y: usize,
    width: usize,
    height: usize,
    lines: Vec<Vec<TermCell>>,
    current_fg: (u8, u8, u8),
    current_bg: (u8, u8, u8),
    scrollback: Vec<Vec<TermCell>>,
}

impl Default for VtePerformer {
    fn default() -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            width: 80,
            height: 24,
            lines: vec![vec![TermCell::default(); 80]; 24],
            current_fg: (204, 204, 204),
            current_bg: (30, 30, 30),
            scrollback: Vec::new(),
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
        _intermediates: &[u8],
        _ignore: bool,
        action: char,
    ) {
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
                self.cursor_y = (row - 1).min(self.height - 1);
                self.cursor_x = (col - 1).min(self.width - 1);
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
                        self.scrollback.clear();
                        self.cursor_x = 0;
                        self.cursor_y = 0;
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
                        0 => {
                            self.current_fg = (204, 204, 204);
                            self.current_bg = (30, 30, 30);
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
                            self.current_fg = colors[idx];
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
                            self.current_bg = colors[idx];
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
                            self.current_fg = colors[idx];
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
                            self.current_bg = colors[idx];
                        }
                        38 if i + 2 < pv.len() && pv[i + 1] == 5 => {
                            self.current_fg = ansi256_to_rgb(pv[i + 2] as u8);
                            i += 2;
                        }
                        48 if i + 2 < pv.len() && pv[i + 1] == 5 => {
                            self.current_bg = ansi256_to_rgb(pv[i + 2] as u8);
                            i += 2;
                        }
                        _ => {}
                    }
                    i += 1;
                }
            }
            _ => {}
        }
    }

    fn osc_dispatch(&mut self, _params: &[&[u8]], _bell_terminated: bool) {}
    fn hook(&mut self, _params: &vte::Params, _intermediates: &[u8], _ignore: bool, _action: char) {
    }
    fn put(&mut self, _byte: u8) {}
    fn unhook(&mut self) {}
}

impl VtePerformer {
    fn scroll_up(&mut self) {
        if !self.lines.is_empty() {
            let first = self.lines.remove(0);
            self.scrollback.push(first);
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
    pub fn spawn(shell: &str, width: u16, height: u16, cwd: &Path) -> Result<Self> {
        let pty_system = portable_pty::native_pty_system();

        let pair = pty_system.openpty(portable_pty::PtySize {
            rows: height,
            cols: width,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        let mut cmd = portable_pty::CommandBuilder::new(shell);
        cmd.cwd(cwd);

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
                shell,
                "\u{2014}",
                title_cwd
            ),
            scrollback: Vec::new(),
            follow_mode: true,
            dirty: true,
            cursor_visible: true,
            last_blink: Instant::now(),
            selection_anchor: None,
            selection_active: None,
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
            self.dirty = true;
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
        for line in &mut self.performer.lines {
            line.resize(width as usize, TermCell::default());
        }
        self.performer
            .lines
            .resize(height as usize, vec![TermCell::default(); width as usize]);
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

    /// Total number of lines in the scrollback buffer + visible screen.
    pub fn total_lines(&self) -> usize {
        self.scrollback.len() + self.lines.len()
    }

    /// Get a line by index from the combined scrollback + screen buffer.
    /// Index 0 = oldest scrollback line, total_lines()-1 = bottom of screen.
    pub fn get_line(&self, idx: usize) -> &Vec<TermCell> {
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
