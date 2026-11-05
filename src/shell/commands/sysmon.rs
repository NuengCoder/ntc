use anyhow::Result;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
    KeyModifiers, MouseEventKind,
};
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, queue};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::io::{stdout, Write};
use std::time::{Duration, Instant};
use sysinfo::{DiskKind, Disks, System};

use crate::utils::ansi_text::{expand_tabs, sanitize_control, truncate_visible};

const REFRESH_MS: u64 = 1500;
/// Tab width used when a process name/cmdline contains a tab.
const TAB_STOP: usize = 4;
/// How long an action message stays in the response bar.
const STATUS_MS: u128 = 3000;
/// Rows reserved at the bottom: response bar + hint bar.
const BARS_H: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SortBy {
    Cpu,
    Mem,
    Pid,
    Name,
}

impl SortBy {
    fn label(self) -> &'static str {
        match self {
            SortBy::Cpu => "CPU",
            SortBy::Mem => "MEM",
            SortBy::Pid => "PID",
            SortBy::Name => "NAME",
        }
    }

    /// Next column in the `s` cycle (CPU -> MEM -> PID -> NAME -> CPU).
    fn next(self) -> Self {
        match self {
            SortBy::Cpu => SortBy::Mem,
            SortBy::Mem => SortBy::Pid,
            SortBy::Pid => SortBy::Name,
            SortBy::Name => SortBy::Cpu,
        }
    }
}

/// Order two PIDs by the active column, honouring the direction and falling
/// back to the PID so equal keys stay stable between refreshes.
fn compare_pids(
    sys: &System,
    sort_by: SortBy,
    sort_asc: bool,
    a: &sysinfo::Pid,
    b: &sysinfo::Pid,
) -> Ordering {
    let ord = match sort_by {
        SortBy::Pid => a.cmp(b),
        SortBy::Name => {
            let na = sys
                .process(*a)
                .map(|p| p.name().to_string_lossy().to_lowercase())
                .unwrap_or_default();
            let nb = sys
                .process(*b)
                .map(|p| p.name().to_string_lossy().to_lowercase())
                .unwrap_or_default();
            na.cmp(&nb)
        }
        SortBy::Cpu => {
            let ca = sys.process(*a).map(|p| p.cpu_usage()).unwrap_or(0.0);
            let cb = sys.process(*b).map(|p| p.cpu_usage()).unwrap_or(0.0);
            ca.total_cmp(&cb)
        }
        SortBy::Mem => {
            let ma = sys.process(*a).map(|p| p.memory()).unwrap_or(0);
            let mb = sys.process(*b).map(|p| p.memory()).unwrap_or(0);
            ma.cmp(&mb)
        }
    };
    let ord = if sort_asc { ord } else { ord.reverse() };
    ord.then_with(|| a.cmp(b))
}

/// One row of the scrollable body. Text is sanitised on construction so file,
/// process or disk names containing control bytes can never reach the terminal.
struct BodyLine {
    text: String,
    fg: Option<Color>,
    bg: Option<Color>,
}

impl BodyLine {
    fn new(text: impl Into<String>) -> Self {
        Self {
            text: sanitize_control(&expand_tabs(&text.into(), TAB_STOP)),
            fg: None,
            bg: None,
        }
    }

    fn colored(text: impl Into<String>, fg: Color) -> Self {
        let mut line = Self::new(text);
        line.fg = Some(fg);
        line
    }

    fn selected(text: impl Into<String>, bg: Color) -> Self {
        let mut line = Self::new(text);
        line.bg = Some(bg);
        line
    }
}

/// What the main loop should do after a key press. The handler never touches
/// processes or the terminal, which keeps every binding unit-testable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SysAction {
    None,
    Render,
    Refresh,
    Kill(u32),
    Quit,
}

struct StatusMsg {
    text: String,
    at: Instant,
    err: bool,
}

struct SysMon {
    sys: System,
    disks: Disks,
    sort_by: SortBy,
    sort_asc: bool,
    show_cpu: bool,
    show_disks: bool,
    show_process: bool,
    filter: String,
    filtering: bool,
    help_visible: bool,
    /// Index into `flat_pids`.
    sel: usize,
    /// Selected PID, so the selection survives a re-sort or refresh.
    sel_pid: Option<sysinfo::Pid>,
    /// Process-list scroll (relative to the first process row).
    scroll: usize,
    /// Whole-body scroll.
    tui_scroll: usize,
    term_h: usize,
    term_w: usize,
    flat_pids: Vec<sysinfo::Pid>,
    depth_map: HashMap<sysinfo::Pid, usize>,
    expanded: HashSet<sysinfo::Pid>,
    killed: HashSet<sysinfo::Pid>,
    status_msg: Option<StatusMsg>,
    last_refresh: Instant,
}

impl SysMon {
    fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        std::thread::sleep(Duration::from_millis(200));
        sys.refresh_all();
        let (w, h) = terminal_size();
        let mut sm = Self::with_system(sys, w, h);
        sm.refresh_filtered();
        sm
    }

    fn with_system(sys: System, w: usize, h: usize) -> Self {
        Self {
            sys,
            disks: Disks::new_with_refreshed_list(),
            sort_by: SortBy::Cpu,
            sort_asc: false,
            show_cpu: true,
            show_disks: true,
            show_process: true,
            filter: String::new(),
            filtering: false,
            help_visible: false,
            sel: 0,
            sel_pid: None,
            scroll: 0,
            tui_scroll: 0,
            term_h: h,
            term_w: w,
            flat_pids: Vec::new(),
            depth_map: HashMap::new(),
            expanded: HashSet::new(),
            killed: HashSet::new(),
            status_msg: None,
            last_refresh: Instant::now(),
        }
    }

    fn refresh(&mut self) {
        self.last_refresh = Instant::now();
        self.sys.refresh_all();
        // Re-enumerate volumes (picks up plugged/unplugged drives)
        // without dropping existing entries.
        self.disks.refresh(false);
        self.refresh_filtered();
    }

    fn build_tree(
        &self,
        all_pids: &[sysinfo::Pid],
    ) -> (Vec<sysinfo::Pid>, HashMap<sysinfo::Pid, usize>) {
        let pid_set: HashSet<sysinfo::Pid> = all_pids.iter().copied().collect();
        let parent_map: HashMap<sysinfo::Pid, sysinfo::Pid> = all_pids
            .iter()
            .filter_map(|pid| {
                self.sys
                    .process(*pid)
                    .and_then(|p| p.parent())
                    // A self-parent (pid 0 on Windows reports itself) would
                    // otherwise be neither a root nor reachable from one.
                    .filter(|ppid| *ppid != *pid && pid_set.contains(ppid))
                    .map(|ppid| (*pid, ppid))
            })
            .collect();

        let mut children: HashMap<sysinfo::Pid, Vec<sysinfo::Pid>> = HashMap::new();
        for (&child, &parent) in &parent_map {
            children.entry(parent).or_default().push(child);
        }

        let mut roots: Vec<sysinfo::Pid> = all_pids
            .iter()
            .filter(|pid| !parent_map.contains_key(pid))
            .copied()
            .collect();

        roots.sort_by(|a, b| compare_pids(&self.sys, self.sort_by, self.sort_asc, a, b));
        // Children follow the same column and direction as the roots.
        for list in children.values_mut() {
            list.sort_by(|a, b| compare_pids(&self.sys, self.sort_by, self.sort_asc, a, b));
        }

        let mut flat = Vec::new();
        let mut depths = HashMap::new();
        fn dfs(
            pid: sysinfo::Pid,
            depth: usize,
            expanded: &HashSet<sysinfo::Pid>,
            children: &HashMap<sysinfo::Pid, Vec<sysinfo::Pid>>,
            flat: &mut Vec<sysinfo::Pid>,
            depths: &mut HashMap<sysinfo::Pid, usize>,
        ) {
            flat.push(pid);
            depths.insert(pid, depth);
            if expanded.contains(&pid) {
                if let Some(kids) = children.get(&pid) {
                    for &child in kids {
                        dfs(child, depth + 1, expanded, children, flat, depths);
                    }
                }
            }
        }
        for &root in &roots {
            dfs(root, 0, &self.expanded, &children, &mut flat, &mut depths);
        }
        (flat, depths)
    }

    fn refresh_filtered(&mut self) {
        // On each full refresh, clear killed PIDs that are no longer in the system
        self.killed
            .retain(|pid| self.sys.processes().contains_key(pid));

        let all: Vec<sysinfo::Pid> = self
            .sys
            .processes()
            .keys()
            .filter(|pid| !self.killed.contains(pid))
            .copied()
            .collect();
        let all = if self.filter.is_empty() {
            all
        } else {
            let lower = self.filter.to_lowercase();
            all.into_iter()
                .filter(|&pid| {
                    self.sys
                        .process(pid)
                        .map(|p| p.name().to_string_lossy().to_lowercase().contains(&lower))
                        .unwrap_or(false)
                })
                .collect()
        };
        let (flat, depths) = self.build_tree(&all);
        self.flat_pids = flat;
        self.depth_map = depths;
        // Keep the selection on the same PID; clamp when it disappeared.
        if let Some(pid) = self.sel_pid {
            if let Some(i) = self.flat_pids.iter().position(|p| *p == pid) {
                self.sel = i;
            }
        }
        self.sel = self.sel.min(self.flat_pids.len().saturating_sub(1));
        self.sel_pid = self.flat_pids.get(self.sel).copied();
    }

    fn selected_pid(&self) -> Option<sysinfo::Pid> {
        self.flat_pids.get(self.sel).copied()
    }

    fn set_sel(&mut self, idx: usize) {
        self.sel = idx.min(self.flat_pids.len().saturating_sub(1));
        self.sel_pid = self.flat_pids.get(self.sel).copied();
    }

    fn flash(&mut self, text: impl Into<String>) {
        self.status_msg = Some(StatusMsg {
            text: text.into(),
            at: Instant::now(),
            err: false,
        });
    }

    fn flash_err(&mut self, text: impl Into<String>) {
        self.status_msg = Some(StatusMsg {
            text: text.into(),
            at: Instant::now(),
            err: true,
        });
    }

    /// Rows available for the body (everything above the two bottom bars).
    fn body_rows(&self) -> usize {
        self.term_h.saturating_sub(BARS_H)
    }

    /// Rows a page key moves.
    fn page(&self) -> usize {
        self.body_rows().saturating_sub(1).max(1)
    }

    fn max_tui_scroll(&self, body_len: usize) -> usize {
        body_len.saturating_sub(self.body_rows())
    }

    /// `j`/`Down` and mouse wheel: selection when the process list is visible,
    /// otherwise the whole-body scroll.
    fn move_selection(&mut self, down: bool) -> SysAction {
        if !self.show_process {
            if down {
                self.tui_scroll = self.tui_scroll.saturating_add(1);
            } else {
                self.tui_scroll = self.tui_scroll.saturating_sub(1);
            }
            return SysAction::Render;
        }
        let last = self.flat_pids.len().saturating_sub(1);
        let next = if down {
            (self.sel + 1).min(last)
        } else {
            self.sel.saturating_sub(1)
        };
        self.set_sel(next);
        SysAction::Render
    }

    /// `PgUp`/`PgDn`: scroll the body only, never the selection.
    fn scroll_view(&mut self, down: bool) -> SysAction {
        let page = self.page();
        if down {
            self.tui_scroll = self.tui_scroll.saturating_add(page);
        } else {
            self.tui_scroll = self.tui_scroll.saturating_sub(page);
        }
        SysAction::Render
    }

    fn kill_selected(&mut self) -> SysAction {
        match self.selected_pid() {
            Some(pid) => SysAction::Kill(pid.as_u32()),
            None => {
                self.flash_err("no process selected");
                SysAction::Render
            }
        }
    }

    /// Drop a killed PID from the list immediately, keeping the selection valid.
    fn forget_pid(&mut self, pid: u32) {
        let pid = sysinfo::Pid::from_u32(pid);
        self.expanded.remove(&pid);
        self.killed.insert(pid);
        self.flat_pids.retain(|p| *p != pid);
        self.depth_map.remove(&pid);
        self.sel = self.sel.min(self.flat_pids.len().saturating_sub(1));
        self.sel_pid = self.flat_pids.get(self.sel).copied();
    }

    /// Pure key handling: decides what to do, performs no side effects beyond
    /// in-memory state (killing is returned as [`SysAction::Kill`]).
    fn handle_key(&mut self, key: KeyEvent) -> SysAction {
        let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

        if self.filtering {
            match key.code {
                KeyCode::Enter | KeyCode::Esc => {
                    self.filtering = false;
                    self.refresh_filtered();
                    SysAction::Render
                }
                KeyCode::Backspace => {
                    self.filter.pop();
                    self.refresh_filtered();
                    SysAction::Render
                }
                KeyCode::Char(c) if !ctrl => {
                    self.filter.push(c);
                    self.refresh_filtered();
                    SysAction::Render
                }
                _ => SysAction::None,
            }
        } else {
            match key.code {
                KeyCode::Char('q') if !ctrl => SysAction::Quit,
                KeyCode::Esc => {
                    if self.filter.is_empty() {
                        SysAction::Quit
                    } else {
                        self.filter.clear();
                        self.refresh_filtered();
                        self.flash("filter cleared");
                        SysAction::Render
                    }
                }
                KeyCode::Char('d') if !ctrl => {
                    self.show_disks = !self.show_disks;
                    self.flash(if self.show_disks {
                        "disks shown"
                    } else {
                        "disks hidden"
                    });
                    SysAction::Render
                }
                KeyCode::Char('p') if !ctrl => {
                    self.show_process = !self.show_process;
                    self.flash(if self.show_process {
                        "processes shown"
                    } else {
                        "processes hidden"
                    });
                    SysAction::Render
                }
                KeyCode::Char('c') if !ctrl => {
                    self.show_cpu = !self.show_cpu;
                    self.flash(if self.show_cpu {
                        "cpu grid shown"
                    } else {
                        "cpu grid hidden"
                    });
                    SysAction::Render
                }
                KeyCode::Char('?') => {
                    self.help_visible = !self.help_visible;
                    SysAction::Render
                }
                KeyCode::Char('j') | KeyCode::Down if !ctrl => self.move_selection(true),
                KeyCode::Char('k') if ctrl => self.kill_selected(),
                KeyCode::Char('k') | KeyCode::Up => self.move_selection(false),
                KeyCode::Char('x') => self.kill_selected(),
                KeyCode::Char('g') if !ctrl => {
                    self.tui_scroll = 0;
                    self.scroll = 0;
                    self.set_sel(0);
                    self.flash("top");
                    SysAction::Render
                }
                KeyCode::Char('G') => {
                    self.set_sel(self.flat_pids.len().saturating_sub(1));
                    self.tui_scroll = usize::MAX;
                    self.flash("bottom");
                    SysAction::Render
                }
                KeyCode::PageUp => self.scroll_view(false),
                KeyCode::PageDown => self.scroll_view(true),
                KeyCode::Char('s') if !ctrl => {
                    self.sort_by = self.sort_by.next();
                    self.reset_view();
                    self.flash(self.sort_msg());
                    SysAction::Refresh
                }
                KeyCode::Char('S') => {
                    self.sort_asc = !self.sort_asc;
                    self.reset_view();
                    self.flash(self.sort_msg());
                    SysAction::Refresh
                }
                KeyCode::Char('F') => {
                    self.filtering = true;
                    SysAction::Render
                }
                KeyCode::Enter => {
                    if let Some(pid) = self.selected_pid() {
                        if !self.expanded.remove(&pid) {
                            self.expanded.insert(pid);
                        }
                        self.refresh_filtered();
                    }
                    SysAction::Render
                }
                _ => SysAction::None,
            }
        }
    }

    fn reset_view(&mut self) {
        self.sel = 0;
        self.sel_pid = None;
        self.scroll = 0;
        self.tui_scroll = 0;
    }

    fn direction(&self) -> &'static str {
        if self.sort_asc {
            "^"
        } else {
            "v"
        }
    }

    fn sort_msg(&self) -> String {
        let word = if self.sort_asc { "asc" } else { "desc" };
        format!("sort: {} {}", self.sort_by.label(), word)
    }

    fn hint_text(&self) -> String {
        let sort_hint = match self.sort_by {
            SortBy::Cpu => "s=sort CPU",
            SortBy::Mem => "s=sort MEM",
            SortBy::Pid => "s=sort PID",
            SortBy::Name => "s=sort NAME",
        };
        format!(
            "?=help  d=disks p=proc {sort_hint} S=rev c=cpu F=filter j/k=move x=kill g=top G=end PgUp/PgDn=scroll q=quit  |  sort: {} {}  disks:{} proc:{}",
            self.sort_by.label(),
            self.direction(),
            if self.show_disks { "on" } else { "off" },
            if self.show_process { "on" } else { "off" },
        )
    }
}

fn terminal_size() -> (usize, usize) {
    crossterm::terminal::size()
        .map(|(w, h)| (w as usize, h as usize))
        .unwrap_or((80, 24))
}

/// Fixed-width 17-cell gauge: `[---------- 100%]`. The percentage is padded to
/// three cells so bars keep their columns while the number changes.
fn render_text_bar(pct: usize) -> String {
    let pct = pct.min(100);
    let fill = pct / 10;
    let empty = 10usize.saturating_sub(fill);
    format!("[{}{} {:>3}%]", "-".repeat(fill), " ".repeat(empty), pct)
}

/// Cell width of one core in the CPU grid: `C 1 ` + bar + two-space gap.
const CPU_CORE_LABEL_W: usize = 4;
const CPU_CELL_GAP: usize = 2;

/// Per-core grid rows: `C 1 [  3%]  C 2 [ 47%]`, wrapped to `cols`. Cores are
/// numbered from 1 (what the OS reports as core 1) and every cell keeps the
/// same width, so the gauges line up in columns.
fn cpu_grid_lines(usages: &[usize], cols: usize) -> Vec<String> {
    let bar_w = render_text_bar(100).len();
    let cell_w = CPU_CORE_LABEL_W + 1 + bar_w + CPU_CELL_GAP;
    let per_row = (cols / cell_w).max(1);
    let mut lines = Vec::new();
    for (row, chunk) in usages.chunks(per_row).enumerate() {
        let mut line = String::from(" ");
        for (i, &pct) in chunk.iter().enumerate() {
            let core_no = row * per_row + i + 1;
            line.push_str(&format!(
                "C{:>2} {}{}",
                core_no,
                render_text_bar(pct),
                " ".repeat(CPU_CELL_GAP)
            ));
        }
        lines.push(line.trim_end().to_string());
    }
    lines
}

/// Colours collapse to the terminal default when the user ran `setc OFF`
/// (or set `NO_COLOR` / `TERM=dumb`), so `sys` honours the same switch as the
/// rest of the shell.
fn paint(use_color: bool, color: Color) -> Color {
    if use_color {
        color
    } else {
        Color::Reset
    }
}

/// Render one disk row: ` C:\ [Label] (NTFS SSD) [USB] [bar] used / total`.
/// Empty labels / filesystems are omitted, unknown kinds are skipped,
/// and a zero total renders `0 / 0` without dividing by zero.
pub(crate) fn format_disk_line(
    mount: &str,
    label: &str,
    fs: &str,
    kind: DiskKind,
    removable: bool,
    readonly: bool,
    used: u64,
    total: u64,
) -> String {
    let pct = used
        .checked_mul(100)
        .and_then(|v| v.checked_div(total))
        .unwrap_or(0) as usize;
    let mut s = format!(" {}", mount);
    let label = label.trim();
    if !label.is_empty() {
        s.push_str(&format!(" [{}]", label));
    }
    let mut paren: Vec<&str> = Vec::new();
    let fs = fs.trim();
    if !fs.is_empty() {
        paren.push(fs);
    }
    match kind {
        DiskKind::HDD => paren.push("HDD"),
        DiskKind::SSD => paren.push("SSD"),
        DiskKind::Unknown(_) => {}
    }
    if !paren.is_empty() {
        s.push_str(&format!(" ({})", paren.join(" ")));
    }
    if removable {
        s.push_str(" [USB]");
    }
    if readonly {
        s.push_str(" [RO]");
    }
    s.push_str(&format!(
        " {}  {} / {}",
        render_text_bar(pct),
        format_size(used),
        format_size(total)
    ));
    s
}

fn format_size(bytes: u64) -> String {
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

impl SysMon {
    fn render(&mut self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        let use_color = crate::utils::theme::ansi_enabled();
        let cols = self.term_w;
        let body_h = self.body_rows();
        let fg_info = paint(use_color, theme.shell.info.to_crossterm());
        let fg_success = paint(use_color, theme.shell.success.to_crossterm());
        let fg_warn = paint(use_color, theme.shell.warning.to_crossterm());
        let fg_err = paint(use_color, theme.shell.error.to_crossterm());
        let bg_status = paint(use_color, theme.editor.status_bg.to_crossterm());
        let bg_sel = paint(use_color, theme.editor.selection_bg.to_crossterm());

        queue!(stdout, terminal::Clear(ClearType::All))?;

        let mut body: Vec<BodyLine> = Vec::new();

        // ── Memory ──
        let total_mem = self.sys.total_memory();
        let used_mem = self.sys.used_memory();
        let mem_pct = used_mem
            .checked_mul(100)
            .and_then(|v| v.checked_div(total_mem))
            .unwrap_or(0) as usize;
        let mem_fg = if mem_pct > 80 {
            fg_err
        } else if mem_pct > 50 {
            fg_warn
        } else {
            fg_success
        };
        body.push(BodyLine::colored(
            format!(
                " MEM  {}  {} / {}",
                render_text_bar(mem_pct),
                format_size(used_mem),
                format_size(total_mem)
            ),
            mem_fg,
        ));

        // ── Swap ──
        let total_swap = self.sys.total_swap();
        let used_swap = self.sys.used_swap();
        let swap_pct = used_swap
            .checked_mul(100)
            .and_then(|v| v.checked_div(total_swap))
            .unwrap_or(0) as usize;
        let swap_fg = if swap_pct > 80 {
            fg_err
        } else if swap_pct > 50 {
            fg_warn
        } else {
            fg_success
        };
        body.push(BodyLine::colored(
            format!(
                " SWAP {}  {} / {}",
                render_text_bar(swap_pct),
                format_size(used_swap),
                format_size(total_swap)
            ),
            swap_fg,
        ));

        // ── Disks (toggled by 'd') ──
        if self.show_disks {
            for disk in &self.disks {
                let total = disk.total_space();
                let avail = disk.available_space();
                let used = total.saturating_sub(avail);
                let pct = used
                    .checked_mul(100)
                    .and_then(|v| v.checked_div(total))
                    .unwrap_or(0) as usize;
                let fg = if pct > 85 {
                    fg_err
                } else if pct > 60 {
                    fg_warn
                } else {
                    fg_success
                };
                body.push(BodyLine::colored(
                    format_disk_line(
                        &disk.mount_point().to_string_lossy(),
                        &disk.name().to_string_lossy(),
                        &disk.file_system().to_string_lossy(),
                        disk.kind(),
                        disk.is_removable(),
                        disk.is_read_only(),
                        used,
                        total,
                    ),
                    fg,
                ));
            }
        }

        // ── Per-core CPU grid (toggled by 'c') ──
        if self.show_cpu {
            let usages: Vec<usize> = self
                .sys
                .cpus()
                .iter()
                .map(|c| c.cpu_usage() as usize)
                .collect();
            for line in cpu_grid_lines(&usages, cols) {
                body.push(BodyLine::colored(line, fg_success));
            }
        }

        body.push(BodyLine::new("\u{2500}".repeat(cols)));

        // ── Filter bar ──
        if self.filtering {
            body.push(BodyLine::colored(
                format!(" filter: {}_", self.filter),
                fg_info,
            ));
        } else if !self.filter.is_empty() {
            body.push(BodyLine::colored(
                format!(" filter: {}  (F to edit, Esc to clear)", self.filter),
                fg_warn,
            ));
        }

        // ── Process list (toggled by 'p') ──
        let mut proc_start = None;
        if self.show_process {
            let proc_h = body_h.saturating_sub(body.len() + 1).max(1);
            if self.sel >= self.scroll + proc_h {
                self.scroll = self.sel.saturating_add(1).saturating_sub(proc_h);
            }
            if self.sel < self.scroll {
                self.scroll = self.sel;
            }

            let dir = self.direction();
            let mark = |label: &str, by: SortBy| {
                if self.sort_by == by {
                    format!("{label}{dir}")
                } else {
                    label.to_string()
                }
            };
            body.push(BodyLine::colored(
                format!(
                    "  {:>6} {:>6} {:>8}  {}",
                    mark("PID", SortBy::Pid),
                    mark("CPU%", SortBy::Cpu),
                    mark("MEM", SortBy::Mem),
                    mark("COMMAND", SortBy::Name)
                ),
                fg_info,
            ));
            proc_start = Some(body.len());

            // Build the children map from ALL processes (not just the visible
            // ones) so cumulative totals work for collapsed parents too.
            let tree_children: HashMap<sysinfo::Pid, Vec<sysinfo::Pid>> = {
                let mut map: HashMap<sysinfo::Pid, Vec<sysinfo::Pid>> = HashMap::new();
                for (p, proc) in self.sys.processes() {
                    if self.killed.contains(p) {
                        continue;
                    }
                    if let Some(ppid) = proc.parent() {
                        if ppid != *p
                            && self.sys.processes().contains_key(&ppid)
                            && !self.killed.contains(&ppid)
                        {
                            map.entry(ppid).or_default().push(*p);
                        }
                    }
                }
                map
            };

            fn sum_tree(
                pid: sysinfo::Pid,
                children: &HashMap<sysinfo::Pid, Vec<sysinfo::Pid>>,
                sys: &sysinfo::System,
            ) -> (f32, u64) {
                let (mut tcpu, mut tmem) = sys
                    .process(pid)
                    .map(|p| (p.cpu_usage(), p.memory()))
                    .unwrap_or((0.0, 0));
                if let Some(kids) = children.get(&pid) {
                    for &k in kids {
                        let (c, m) = sum_tree(k, children, sys);
                        tcpu += c;
                        tmem += m;
                    }
                }
                (tcpu, tmem)
            }

            let total_procs = self.flat_pids.len();
            for i in 0..proc_h {
                let idx = self.scroll + i;
                if idx >= total_procs {
                    break;
                }
                let pid = self.flat_pids[idx];
                let depth = self.depth_map.get(&pid).copied().unwrap_or(0);
                let has_children = tree_children.contains_key(&pid);
                let is_expanded = self.expanded.contains(&pid);

                let Some(proc_info) = self.sys.process(pid) else {
                    continue;
                };
                let (cpu, mem) = if has_children {
                    sum_tree(pid, &tree_children, &self.sys)
                } else {
                    (proc_info.cpu_usage(), proc_info.memory())
                };
                let mem_mb = mem as f64 / 1024.0 / 1024.0;
                let name = proc_info.name().to_string_lossy();
                let cmd = proc_info
                    .cmd()
                    .first()
                    .map(|s| s.to_string_lossy())
                    .unwrap_or_default();

                let indent = "  ".repeat(depth);
                let toggle = if has_children {
                    if is_expanded {
                        "\u{25bc} "
                    } else {
                        "\u{25b6} "
                    }
                } else {
                    "  "
                };

                let text =
                    format!("{indent}{toggle}{pid:>6} {cpu:>6.1} {mem_mb:>7.1}M  {name} {cmd}");
                if idx == self.sel {
                    body.push(BodyLine::selected(text, bg_sel));
                } else {
                    body.push(BodyLine::new(text));
                }
            }
        } else if !self.show_disks && !self.show_cpu {
            body.push(BodyLine::colored(
                " all panels hidden - press d for disks, p for processes",
                fg_warn,
            ));
        }

        // `setc OFF` must stay plain: drop every colour the body collected,
        // including the selection background.
        if !use_color {
            for line in &mut body {
                line.fg = None;
                line.bg = None;
            }
        }

        // Keep the selected process row inside the viewport.
        if let Some(start) = proc_start {
            let abs = start + self.sel.saturating_sub(self.scroll);
            if abs < self.tui_scroll {
                self.tui_scroll = abs;
            } else if body_h > 0 && abs >= self.tui_scroll + body_h {
                self.tui_scroll = abs + 1 - body_h;
            }
        }
        self.tui_scroll = self.tui_scroll.min(self.max_tui_scroll(body.len()));

        // ── Body (windowed by `tui_scroll`) ──
        for i in 0..body_h {
            let y = i as u16;
            queue!(stdout, MoveTo(0, y), Clear(ClearType::UntilNewLine))?;
            let Some(line) = body.get(self.tui_scroll + i) else {
                continue;
            };
            if let Some(bg) = line.bg {
                queue!(stdout, SetBackgroundColor(bg))?;
            }
            if let Some(fg) = line.fg {
                queue!(stdout, SetForegroundColor(fg))?;
            }
            queue!(
                stdout,
                Print(truncate_visible(&line.text, cols)),
                ResetColor
            )?;
        }

        // ── Response bar ──
        let resp_y = body_h as u16;
        queue!(
            stdout,
            MoveTo(0, resp_y),
            SetBackgroundColor(bg_status),
            Clear(ClearType::UntilNewLine)
        )?;
        if let Some(msg) = self
            .status_msg
            .as_ref()
            .filter(|m| m.at.elapsed().as_millis() < STATUS_MS)
        {
            queue!(
                stdout,
                SetForegroundColor(if msg.err { fg_err } else { fg_info }),
                Print(truncate_visible(&format!(" {}", msg.text), cols)),
            )?;
        }
        queue!(stdout, ResetColor)?;

        // ── Hint bar (last row, `?=help` first) ──
        let hint_y = (body_h + 1) as u16;
        queue!(
            stdout,
            MoveTo(0, hint_y),
            SetBackgroundColor(bg_status),
            SetForegroundColor(fg_info),
            Clear(ClearType::UntilNewLine),
            Print(truncate_visible(&self.hint_text(), cols)),
            ResetColor
        )?;

        if self.help_visible {
            self.render_help(stdout, bg_status, fg_info, fg_warn)?;
        }

        stdout.flush()?;
        Ok(())
    }

    fn render_help(
        &self,
        stdout: &mut std::io::Stdout,
        bg: Color,
        fg: Color,
        fg_key: Color,
    ) -> std::io::Result<()> {
        const KEYS: &[(&str, &str)] = &[
            ("d", "show / hide disks"),
            ("p", "show / hide the process list"),
            ("j / Down", "selection down (view scroll when list hidden)"),
            ("k / Up", "selection up"),
            ("g", "first row"),
            ("G", "last row"),
            ("PgUp / PgDn", "scroll the view one page"),
            ("s", "next sort column"),
            ("S", "reverse sort direction"),
            ("c", "show / hide the CPU grid"),
            ("F", "filter processes by name"),
            ("x / Ctrl+K", "kill the selected process"),
            ("Enter", "expand / collapse a process tree"),
            ("?", "close this help"),
            ("q / Esc", "quit (Esc clears the filter first)"),
        ];
        let pad = 12;
        let widest = KEYS
            .iter()
            .map(|(k, d)| k.chars().count() + d.chars().count())
            .max()
            .unwrap_or(20);
        let w = (widest + pad).min(self.term_w.saturating_sub(2)).max(24);
        let h = KEYS.len() + 2;
        let left = (self.term_w.saturating_sub(w)) / 2;
        let top = (self.term_h.saturating_sub(h)) / 2;

        for r in 0..h {
            queue!(
                stdout,
                MoveTo(left as u16, (top + r) as u16),
                SetBackgroundColor(bg),
                Clear(ClearType::UntilNewLine),
                Print(" ".repeat(w)),
            )?;
        }
        let title = " help ";
        let fill = w.saturating_sub(title.chars().count() + 2);
        queue!(
            stdout,
            MoveTo(left as u16, top as u16),
            SetBackgroundColor(bg),
            SetForegroundColor(fg),
            Print(format!("{}{}", title, "".repeat(fill))),
        )?;
        for (i, (key, desc)) in KEYS.iter().enumerate() {
            queue!(
                stdout,
                MoveTo(left as u16, (top + 1 + i) as u16),
                SetBackgroundColor(bg),
                SetForegroundColor(fg_key),
                Print(truncate_visible(
                    &format!(" {key:<11}"),
                    w.saturating_sub(1)
                )),
                SetForegroundColor(fg),
                Print(truncate_visible(desc, w.saturating_sub(14))),
            )?;
        }
        queue!(
            stdout,
            MoveTo(left as u16, (top + h - 1) as u16),
            SetBackgroundColor(bg),
            SetForegroundColor(fg),
            Print(format!("{}", "".repeat(w - 2))),
            ResetColor
        )?;
        Ok(())
    }
}

fn kill_process(pid: u32) {
    let pid_str = pid.to_string();
    #[cfg(windows)]
    let _ = std::process::Command::new("taskkill")
        .args(["/F", "/PID", &pid_str])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();
    #[cfg(not(windows))]
    let _ = std::process::Command::new("kill")
        .args(["-9", &pid_str])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();
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

pub fn cmd_sys(_args: &str, _nav: &mut crate::navigator::Navigator) -> Result<bool> {
    let mut sm = SysMon::new();
    let mut stdout = stdout();
    let _guard = TerminalGuard;

    execute!(stdout, EnterAlternateScreen, Hide, EnableMouseCapture)?;
    execute!(stdout, Clear(ClearType::All))?;
    terminal::enable_raw_mode()?;

    sm.render(&mut stdout)?;
    let mut last_refresh = Instant::now();

    loop {
        let poll_ms = REFRESH_MS.min(200);
        if !event::poll(Duration::from_millis(poll_ms))? {
            if last_refresh.elapsed().as_millis() as u64 >= REFRESH_MS {
                sm.refresh();
                sm.render(&mut stdout)?;
                last_refresh = Instant::now();
            }
            continue;
        }

        let ev = event::read()?;
        let action = match ev {
            Event::Key(ke) => {
                if ke.kind != KeyEventKind::Press {
                    SysAction::None
                } else {
                    sm.handle_key(ke)
                }
            }
            Event::Resize(w, h) => {
                sm.term_w = w as usize;
                sm.term_h = h as usize;
                SysAction::Render
            }
            Event::Mouse(m) => match m.kind {
                MouseEventKind::ScrollUp => sm.move_selection(false),
                MouseEventKind::ScrollDown => sm.move_selection(true),
                _ => SysAction::None,
            },
            _ => SysAction::None,
        };

        match action {
            SysAction::None => {}
            SysAction::Render => sm.render(&mut stdout)?,
            SysAction::Refresh => {
                sm.refresh();
                sm.render(&mut stdout)?;
                last_refresh = Instant::now();
            }
            SysAction::Kill(pid) => {
                kill_process(pid);
                sm.forget_pid(pid);
                sm.flash(format!("killed PID {pid}"));
                sm.render(&mut stdout)?;
            }
            SysAction::Quit => break,
        }

        if last_refresh.elapsed().as_millis() as u64 >= REFRESH_MS {
            sm.refresh();
            sm.render(&mut stdout)?;
            last_refresh = Instant::now();
        }
    }

    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    fn ctrl(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::CONTROL)
    }

    /// Monitor with a fake process list (no OS query) for key tests.
    fn bare() -> SysMon {
        let mut sm = SysMon::with_system(System::new(), 100, 40);
        sm.disks = Disks::new();
        sm.flat_pids = (0..5).map(|i| sysinfo::Pid::from_u32(100 + i)).collect();
        sm.sel_pid = sm.flat_pids.first().copied();
        sm
    }

    /// Monitor backed by the real process table (sorting tests).
    fn real() -> SysMon {
        let mut sys = System::new_all();
        sys.refresh_all();
        let mut sm = SysMon::with_system(sys, 100, 40);
        sm.disks = Disks::new();
        sm.refresh_filtered();
        sm
    }

    #[test]
    fn toggles_hide_and_show_panels() {
        let mut sm = bare();
        assert_eq!(sm.handle_key(key(KeyCode::Char('d'))), SysAction::Render);
        assert!(!sm.show_disks);
        assert_eq!(
            sm.status_msg.as_ref().map(|m| m.text.as_str()),
            Some("disks hidden")
        );
        sm.handle_key(key(KeyCode::Char('d')));
        assert!(sm.show_disks);

        sm.handle_key(key(KeyCode::Char('p')));
        assert!(!sm.show_process);
        sm.handle_key(key(KeyCode::Char('p')));
        assert!(sm.show_process);

        sm.handle_key(key(KeyCode::Char('c')));
        assert!(!sm.show_cpu);
    }

    #[test]
    fn jk_moves_selection_when_process_list_is_shown() {
        let mut sm = bare();
        sm.handle_key(key(KeyCode::Char('j')));
        assert_eq!(sm.sel, 1);
        assert_eq!(sm.tui_scroll, 0);
        sm.handle_key(key(KeyCode::Char('j')));
        sm.handle_key(key(KeyCode::Up));
        assert_eq!(sm.sel, 1);
        // Never past the ends.
        for _ in 0..10 {
            sm.handle_key(key(KeyCode::Char('k')));
        }
        assert_eq!(sm.sel, 0);
    }

    #[test]
    fn jk_scrolls_the_view_when_process_list_is_hidden() {
        let mut sm = bare();
        sm.show_process = false;
        sm.handle_key(key(KeyCode::Char('j')));
        assert_eq!(sm.sel, 0);
        assert_eq!(sm.tui_scroll, 1);
        sm.handle_key(key(KeyCode::Down));
        assert_eq!(sm.tui_scroll, 2);
        sm.handle_key(key(KeyCode::Char('k')));
        assert_eq!(sm.tui_scroll, 1);
    }

    #[test]
    fn kill_bindings_do_not_fire_on_plain_k() {
        let mut sm = bare();
        sm.set_sel(2);
        assert_eq!(sm.handle_key(key(KeyCode::Char('x'))), SysAction::Kill(102));
        assert_eq!(
            sm.handle_key(ctrl(KeyCode::Char('k'))),
            SysAction::Kill(102)
        );
        // Plain `k` is navigation, never a kill.
        assert_eq!(sm.handle_key(key(KeyCode::Char('k'))), SysAction::Render);
        assert_eq!(sm.sel, 1);
    }

    #[test]
    fn home_and_end_jump_to_the_edges() {
        let mut sm = bare();
        sm.set_sel(3);
        sm.tui_scroll = 7;
        sm.handle_key(key(KeyCode::Char('g')));
        assert_eq!(sm.sel, 0);
        assert_eq!(sm.tui_scroll, 0);
        sm.handle_key(key(KeyCode::Char('G')));
        assert_eq!(sm.sel, 4);
        // Clamped against the body length during render.
        assert_eq!(sm.tui_scroll, usize::MAX);
    }

    #[test]
    fn page_keys_scroll_without_moving_selection() {
        let mut sm = bare();
        sm.set_sel(2);
        let page = sm.page();
        sm.handle_key(key(KeyCode::PageDown));
        assert_eq!(sm.sel, 2, "PgDn must not move the selection");
        assert_eq!(sm.tui_scroll, page);
        sm.handle_key(key(KeyCode::PageUp));
        assert_eq!(sm.tui_scroll, 0);
    }

    #[test]
    fn esc_clears_the_filter_before_quitting() {
        let mut sm = bare();
        sm.filter = "rust".into();
        assert_eq!(sm.handle_key(key(KeyCode::Esc)), SysAction::Render);
        assert!(sm.filter.is_empty());
        assert_eq!(sm.handle_key(key(KeyCode::Esc)), SysAction::Quit);
        assert_eq!(sm.handle_key(key(KeyCode::Char('q'))), SysAction::Quit);
    }

    #[test]
    fn filter_mode_swallows_navigation_keys() {
        let mut sm = bare();
        sm.handle_key(key(KeyCode::Char('F')));
        assert!(sm.filtering);
        // `q` types instead of quitting, `?` types instead of toggling help.
        sm.handle_key(key(KeyCode::Char('q')));
        sm.handle_key(key(KeyCode::Char('?')));
        assert_eq!(sm.filter, "q?");
        assert!(!sm.help_visible);
        sm.handle_key(key(KeyCode::Backspace));
        assert_eq!(sm.filter, "q");
        sm.handle_key(key(KeyCode::Enter));
        assert!(!sm.filtering);
    }

    #[test]
    fn sort_cycle_resets_the_view_and_reports_direction() {
        let mut sm = bare();
        sm.set_sel(3);
        sm.tui_scroll = 4;
        assert_eq!(sm.handle_key(key(KeyCode::Char('s'))), SysAction::Refresh);
        assert_eq!(sm.sort_by, SortBy::Mem);
        assert_eq!(sm.sel, 0);
        assert_eq!(sm.tui_scroll, 0);
        assert_eq!(
            sm.status_msg.as_ref().map(|m| m.text.as_str()),
            Some("sort: MEM desc")
        );
        sm.handle_key(key(KeyCode::Char('S')));
        assert!(sm.sort_asc);
        assert_eq!(
            sm.status_msg.as_ref().map(|m| m.text.as_str()),
            Some("sort: MEM asc")
        );
    }

    #[test]
    fn help_overlay_toggles_with_question_mark() {
        let mut sm = bare();
        sm.handle_key(key(KeyCode::Char('?')));
        assert!(sm.help_visible);
        assert!(sm.hint_text().starts_with("?=help"));
        sm.handle_key(key(KeyCode::Char('?')));
        assert!(!sm.help_visible);
    }

    #[test]
    fn comparator_reverses_with_the_direction() {
        let mut sys = System::new_all();
        sys.refresh_all();
        let mut pids: Vec<sysinfo::Pid> = sys.processes().keys().copied().collect();
        assert!(pids.len() >= 2, "need at least two live processes");

        let mut asc = pids.clone();
        asc.sort_by(|a, b| compare_pids(&sys, SortBy::Pid, true, a, b));
        assert!(asc.windows(2).all(|w| w[0] <= w[1]));

        pids.sort_by(|a, b| compare_pids(&sys, SortBy::Pid, false, a, b));
        assert!(pids.windows(2).all(|w| w[0] >= w[1]));
    }

    #[test]
    fn children_follow_the_sort_direction() {
        let mut sm = real();
        // Find a parent with at least two visible children.
        let mut by_parent: HashMap<sysinfo::Pid, Vec<sysinfo::Pid>> = HashMap::new();
        for (pid, proc) in sm.sys.processes() {
            if let Some(ppid) = proc.parent() {
                if ppid != *pid && sm.sys.processes().contains_key(&ppid) {
                    by_parent.entry(ppid).or_default().push(*pid);
                }
            }
        }
        sm.sort_by = SortBy::Name;
        sm.sort_asc = false;
        let all: Vec<sysinfo::Pid> = sm.sys.processes().keys().copied().collect();
        // Pick a parent the tree can actually reach (collapsed view first).
        let (roots_only, _) = sm.build_tree(&all);
        let Some(parent) = roots_only
            .iter()
            .copied()
            .find(|p| by_parent.get(p).is_some_and(|kids| kids.len() >= 2))
        else {
            return; // The live process table has no branching tree right now.
        };
        sm.expanded.insert(parent);
        let (flat, depths) = sm.build_tree(&all);
        let pos = flat
            .iter()
            .position(|p| *p == parent)
            .expect("parent listed");
        let Some(&first_child) = flat.get(pos + 1) else {
            return;
        };
        let child_depth = depths[&first_child];
        assert_eq!(child_depth, depths[&parent] + 1);
        let kids: Vec<sysinfo::Pid> = flat[pos + 1..]
            .iter()
            .take_while(|p| depths[p] == child_depth)
            .copied()
            .collect();
        assert!(kids.len() >= 2, "expanded parent must list its children");
        assert!(
            kids.windows(2)
                .all(|w| compare_pids(&sm.sys, SortBy::Name, false, &w[0], &w[1])
                    != Ordering::Greater),
            "children must follow the root sort order: {kids:?}"
        );
    }

    #[test]
    fn selection_survives_a_refresh() {
        let mut sm = bare();
        sm.set_sel(3);
        let pid = sm.selected_pid();
        // A refresh rebuilds the list from the (empty) fake system.
        sm.refresh_filtered();
        assert!(sm.sel_pid.is_none() || sm.sel_pid == pid);
        assert!(sm.sel < sm.flat_pids.len().max(1));
    }

    #[test]
    fn killed_pid_is_dropped_immediately() {
        let mut sm = bare();
        sm.set_sel(2);
        sm.forget_pid(102);
        assert!(!sm.flat_pids.contains(&sysinfo::Pid::from_u32(102)));
        assert_eq!(sm.sel, 2);
        assert!(sm.killed.contains(&sysinfo::Pid::from_u32(102)));
    }

    #[test]
    fn body_lines_carry_no_control_bytes() {
        let line = BodyLine::new("na\u{1b}[31mme\tx");
        assert!(
            !line.text.chars().any(|c| c.is_control()),
            "{:?}",
            line.text
        );
        assert!(line.text.contains("^["), "{:?}", line.text);
        assert!(line.text.starts_with("na^[[31mme"), "{:?}", line.text);
    }

    #[test]
    fn cpu_grid_numbers_cores_from_one_with_a_gap() {
        let rows = cpu_grid_lines(&[3, 47, 100], 200);
        assert_eq!(rows.len(), 1, "three cores fit on one row: {rows:?}");
        let row = &rows[0];
        assert!(row.starts_with(" C 1 ["), "got: {row}");
        assert!(!row.contains("C 0"), "cores must start at 1: {row}");
        assert!(
            row.contains("]  C 2 ["),
            "cells need a two-space gap: {row}"
        );
        assert!(
            row.contains("]  C 3 ["),
            "cells need a two-space gap: {row}"
        );
        // Fixed cell width keeps the gauges in columns.
        let first = row.find('[').expect("gauge");
        let second = row[first + 1..].find('[').expect("second gauge") + first + 1;
        let third = row[second + 1..].find('[').expect("third gauge") + second + 1;
        assert_eq!(second - first, third - second, "got: {row}");
    }

    #[test]
    fn cpu_grid_wraps_and_keeps_numbering() {
        // 24-cell cells + one leading space: 49 columns fit two cores.
        let usages: Vec<usize> = (0..5).map(|i| i * 10).collect();
        let rows = cpu_grid_lines(&usages, 49);
        assert_eq!(rows.len(), 3, "got: {rows:?}");
        assert!(
            rows[0].contains("C 1 ") && rows[0].contains("C 2 "),
            "{rows:?}"
        );
        assert!(rows[1].starts_with(" C 3 "), "{rows:?}");
        assert!(rows[2].starts_with(" C 5 "), "{rows:?}");
    }

    #[test]
    fn text_bar_width_is_constant() {
        for pct in [0, 3, 9, 10, 55, 99, 100, 250] {
            assert_eq!(
                render_text_bar(pct).chars().count(),
                17,
                "pct {pct}: {:?}",
                render_text_bar(pct)
            );
        }
        assert_eq!(render_text_bar(100), "[---------- 100%]");
        assert_eq!(render_text_bar(0), "[             0%]");
        assert!(render_text_bar(3).ends_with("  3%]"));
    }

    #[test]
    fn paint_honours_setc_off() {
        assert_eq!(paint(true, Color::Red), Color::Red);
        assert_eq!(paint(false, Color::Red), Color::Reset);
    }

    #[test]
    fn disk_line_full_detail() {
        let line = format_disk_line(
            "C:\\",
            "Windows",
            "NTFS",
            DiskKind::SSD,
            false,
            false,
            120 * 1024 * 1024 * 1024,
            500 * 1024 * 1024 * 1024,
        );
        assert!(
            line.starts_with(" C:\\ [Windows] (NTFS SSD) "),
            "got: {}",
            line
        );
        assert!(line.contains("120"), "got: {}", line);
        assert!(!line.contains("USB"), "got: {}", line);
    }

    #[test]
    fn disk_line_tags_removable_and_readonly() {
        let line = format_disk_line("E:\\", "", "FAT32", DiskKind::HDD, true, true, 0, 100);
        assert!(line.contains(" [USB]"), "got: {}", line);
        assert!(line.contains(" [RO]"), "got: {}", line);
        // Empty label omitted, no stray brackets.
        assert!(!line.contains("[]"), "got: {}", line);
    }

    #[test]
    fn disk_line_unknown_kind_and_zero_total() {
        let line = format_disk_line("/", "", "", DiskKind::Unknown(-1), false, false, 0, 0);
        assert!(!line.contains('('), "got: {}", line);
        assert!(line.ends_with("0.0B / 0.0B"), "got: {}", line);
    }
}
