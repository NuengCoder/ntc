//! Process/disk state and pure key handling for `sys`.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Color;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::time::{Duration, Instant};
use sysinfo::{Disks, System};

use super::logging::SysLogger;
use super::render::terminal_size;
use crate::utils::ansi_text::{expand_tabs, sanitize_control};

pub(super) const REFRESH_MS: u64 = 1500;
/// Tab width used when a process name/cmdline contains a tab.
const TAB_STOP: usize = 4;
/// How long an action message stays in the response bar.
pub(super) const STATUS_MS: u128 = 3000;
/// Rows reserved at the bottom: response bar + hint bar.
pub(super) const BARS_H: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SortBy {
    Cpu,
    Mem,
    Pid,
    Name,
}

impl SortBy {
    pub(super) fn label(self) -> &'static str {
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
pub(super) fn compare_pids(
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
pub(super) struct BodyLine {
    pub(super) text: String,
    pub(super) fg: Option<Color>,
    pub(super) bg: Option<Color>,
}

impl BodyLine {
    pub(super) fn new(text: impl Into<String>) -> Self {
        Self {
            text: sanitize_control(&expand_tabs(&text.into(), TAB_STOP)),
            fg: None,
            bg: None,
        }
    }

    pub(super) fn colored(text: impl Into<String>, fg: Color) -> Self {
        let mut line = Self::new(text);
        line.fg = Some(fg);
        line
    }

    pub(super) fn selected(text: impl Into<String>, bg: Color) -> Self {
        let mut line = Self::new(text);
        line.bg = Some(bg);
        line
    }
}

/// What the main loop should do after a key press. The handler never touches
/// processes or the terminal, which keeps every binding unit-testable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SysAction {
    None,
    Render,
    Refresh,
    Kill(u32),
    Quit,
}

pub(super) struct StatusMsg {
    pub(super) text: String,
    pub(super) at: Instant,
    pub(super) err: bool,
}

pub(super) struct SysMon {
    pub(super) sys: System,
    pub(super) disks: Disks,
    pub(super) sort_by: SortBy,
    pub(super) sort_asc: bool,
    pub(super) show_cpu: bool,
    pub(super) show_disks: bool,
    pub(super) show_process: bool,
    /// Flat "full" list: every process on one level, no expand/collapse.
    pub(super) flat_view: bool,
    pub(super) filter: String,
    pub(super) filtering: bool,
    pub(super) help_visible: bool,
    /// Index into `flat_pids`.
    pub(super) sel: usize,
    /// Selected PID, so the selection survives a re-sort or refresh.
    pub(super) sel_pid: Option<sysinfo::Pid>,
    /// Process-list scroll (relative to the first process row).
    pub(super) scroll: usize,
    /// Whole-body scroll.
    pub(super) tui_scroll: usize,
    pub(super) term_h: usize,
    pub(super) term_w: usize,
    pub(super) flat_pids: Vec<sysinfo::Pid>,
    pub(super) depth_map: HashMap<sysinfo::Pid, usize>,
    pub(super) expanded: HashSet<sysinfo::Pid>,
    pub(super) killed: HashSet<sysinfo::Pid>,
    pub(super) status_msg: Option<StatusMsg>,
    pub(super) logger: Option<SysLogger>,
    last_refresh: Instant,
}

impl SysMon {
    pub(super) fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        std::thread::sleep(Duration::from_millis(200));
        sys.refresh_all();
        let (w, h) = terminal_size();
        let mut sm = Self::with_system(sys, w, h);
        sm.refresh_filtered();
        sm
    }

    pub(super) fn with_system(sys: System, w: usize, h: usize) -> Self {
        Self {
            sys,
            disks: Disks::new_with_refreshed_list(),
            sort_by: SortBy::Cpu,
            sort_asc: false,
            show_cpu: true,
            show_disks: true,
            show_process: true,
            flat_view: false,
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
            logger: None,
            last_refresh: Instant::now(),
        }
    }

    /// Start `--log`: open `<cwd>/log/ntc_sys.log` and write the first
    /// snapshot. Later snapshots are written once every `interval`.
    pub(super) fn start_logging(&mut self, cwd: &Path, interval: Duration) -> std::io::Result<()> {
        let mut logger = SysLogger::open(cwd, interval)?;
        logger.append(&self.sys, &self.disks)?;
        self.logger = Some(logger);
        Ok(())
    }

    /// True when the next snapshot is due. The event loop refreshes early so
    /// the requested logging interval is not rounded up to a UI refresh.
    pub(super) fn log_due(&self) -> bool {
        self.logger
            .as_ref()
            .is_some_and(|logger| logger.is_due(Instant::now()))
    }

    fn log_snapshot(&mut self) {
        let result = match self.logger.as_mut() {
            Some(logger) => Some(logger.append_if_due(&self.sys, &self.disks, Instant::now())),
            None => None,
        };
        if let Some(Err(e)) = result {
            self.logger = None;
            self.flash_err(format!("log disabled: {e}"));
        }
    }

    pub(super) fn refresh(&mut self) {
        self.last_refresh = Instant::now();
        self.sys.refresh_all();
        // Re-enumerate volumes (picks up plugged/unplugged drives)
        // without dropping existing entries.
        self.disks.refresh(false);
        self.log_snapshot();
        self.refresh_filtered();
    }

    pub(super) fn build_tree(
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

    /// Every process on one level, ordered by the active column. Used by the
    /// `v` flat view, where the parent/child tree and its collapse state do
    /// not apply.
    fn build_flat(&self, all_pids: &[sysinfo::Pid]) -> Vec<sysinfo::Pid> {
        let mut flat = all_pids.to_vec();
        flat.sort_by(|a, b| compare_pids(&self.sys, self.sort_by, self.sort_asc, a, b));
        flat
    }

    pub(super) fn refresh_filtered(&mut self) {
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
        let (flat, depths) = if self.flat_view {
            let flat = self.build_flat(&all);
            let depths = flat.iter().map(|pid| (*pid, 0)).collect();
            (flat, depths)
        } else {
            self.build_tree(&all)
        };
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

    pub(super) fn selected_pid(&self) -> Option<sysinfo::Pid> {
        self.flat_pids.get(self.sel).copied()
    }

    pub(super) fn set_sel(&mut self, idx: usize) {
        self.sel = idx.min(self.flat_pids.len().saturating_sub(1));
        self.sel_pid = self.flat_pids.get(self.sel).copied();
    }

    pub(super) fn flash(&mut self, text: impl Into<String>) {
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
    pub(super) fn body_rows(&self) -> usize {
        self.term_h.saturating_sub(BARS_H)
    }

    /// Rows a page key moves.
    pub(super) fn page(&self) -> usize {
        self.body_rows().saturating_sub(1).max(1)
    }

    pub(super) fn max_tui_scroll(&self, body_len: usize) -> usize {
        body_len.saturating_sub(self.body_rows())
    }

    /// `j`/`Down` and mouse wheel: selection when the process list is visible,
    /// otherwise the whole-body scroll.
    pub(super) fn move_selection(&mut self, down: bool) -> SysAction {
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
    pub(super) fn forget_pid(&mut self, pid: u32) {
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
    pub(super) fn handle_key(&mut self, key: KeyEvent) -> SysAction {
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
                KeyCode::Char('v') if !ctrl => {
                    self.flat_view = !self.flat_view;
                    self.refresh_filtered();
                    self.flash(if self.flat_view {
                        "flat view (all processes)"
                    } else {
                        "tree view"
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
                // Nothing to expand while the flat view is active.
                KeyCode::Enter if !self.flat_view => {
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

    pub(super) fn direction(&self) -> &'static str {
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

    pub(super) fn hint_text(&self) -> String {
        let sort_hint = match self.sort_by {
            SortBy::Cpu => "s=sort CPU",
            SortBy::Mem => "s=sort MEM",
            SortBy::Pid => "s=sort PID",
            SortBy::Name => "s=sort NAME",
        };
        let view_hint = if self.flat_view { "v=tree" } else { "v=flat" };
        // Without a filter the count is simply every live process; a filter
        // makes the matched rows meaningful next to the total.
        let total_procs = self.sys.processes().len();
        let procs_hint = if self.filter.is_empty() {
            format!("procs:{total_procs}")
        } else {
            format!("procs:{}/{total_procs}", self.flat_pids.len())
        };
        format!(
            "?=help  d=disks p=proc {view_hint} {sort_hint} S=rev c=cpu F=filter j/k=move x=kill g=top G=end PgUp/PgDn=scroll q=quit  |  {procs_hint}  sort: {} {}  view:{} disks:{} proc:{}{}",
            self.sort_by.label(),
            self.direction(),
            if self.flat_view { "flat" } else { "tree" },
            if self.show_disks { "on" } else { "off" },
            if self.show_process { "on" } else { "off" },
            match &self.logger {
                Some(logger) => format!("  log:on/{}s", logger.interval_secs()),
                None => String::new(),
            },
        )
    }
}

/// Kill a process with the platform's own tool.
pub(super) fn kill_process(pid: u32) {
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
