use anyhow::Result;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind,
};
use crossterm::style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, queue};
use std::collections::{HashMap, HashSet};
use std::io::{stdout, Write};
use std::time::Instant;
use sysinfo::{DiskKind, Disks, System};

const REFRESH_MS: u64 = 1500;

#[derive(Debug)]
enum SortBy {
    Cpu,
    Mem,
    Pid,
    Name,
}

struct SysMon {
    sys: System,
    disks: Disks,
    sort_by: SortBy,
    sort_asc: bool,
    show_cpu: bool,
    filter: String,
    filtering: bool,
    sel: usize,
    scroll: usize,
    term_h: usize,
    term_w: usize,
    flat_pids: Vec<sysinfo::Pid>,
    depth_map: HashMap<sysinfo::Pid, usize>,
    expanded: HashSet<sysinfo::Pid>,
    killed: HashSet<sysinfo::Pid>,
    last_refresh: Instant,
}

impl SysMon {
    fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        std::thread::sleep(std::time::Duration::from_millis(200));
        sys.refresh_all();
        let disks = Disks::new_with_refreshed_list();
        let (w, h) = terminal_size();
        let mut sm = Self {
            sys,
            disks,
            sort_by: SortBy::Cpu,
            sort_asc: false,
            show_cpu: true,
            filter: String::new(),
            filtering: false,
            sel: 0,
            scroll: 0,
            term_h: h,
            term_w: w,
            flat_pids: Vec::new(),
            depth_map: HashMap::new(),
            expanded: HashSet::new(),
            killed: HashSet::new(),
            last_refresh: Instant::now(),
        };
        sm.refresh_filtered();
        sm
    }

    fn refresh(&mut self) {
        let now = Instant::now();
        self.last_refresh = now;
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
                    .filter(|ppid| pid_set.contains(ppid))
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

        let sort_desc = !self.sort_asc;
        match self.sort_by {
            SortBy::Pid => roots.sort_by(|a, b| if sort_desc { b.cmp(a) } else { a.cmp(b) }),
            SortBy::Name => {
                roots.sort_by(|a, b| {
                    let na = self
                        .sys
                        .process(*a)
                        .map(|p| p.name().to_string_lossy().to_string())
                        .unwrap_or_default();
                    let nb = self
                        .sys
                        .process(*b)
                        .map(|p| p.name().to_string_lossy().to_string())
                        .unwrap_or_default();
                    if sort_desc {
                        nb.cmp(&na)
                    } else {
                        na.cmp(&nb)
                    }
                });
            }
            SortBy::Cpu => {
                roots.sort_by(|a, b| {
                    let ca = self.sys.process(*a).map(|p| p.cpu_usage()).unwrap_or(0.0);
                    let cb = self.sys.process(*b).map(|p| p.cpu_usage()).unwrap_or(0.0);
                    if sort_desc {
                        cb.total_cmp(&ca)
                    } else {
                        ca.total_cmp(&cb)
                    }
                });
            }
            SortBy::Mem => {
                roots.sort_by(|a, b| {
                    let ma = self.sys.process(*a).map(|p| p.memory()).unwrap_or(0);
                    let mb = self.sys.process(*b).map(|p| p.memory()).unwrap_or(0);
                    if sort_desc {
                        mb.cmp(&ma)
                    } else {
                        ma.cmp(&mb)
                    }
                });
            }
        }
        for list in children.values_mut() {
            list.sort_by(|a, b| {
                let na = self
                    .sys
                    .process(*a)
                    .map(|p| p.name().to_string_lossy().to_string())
                    .unwrap_or_default();
                let nb = self
                    .sys
                    .process(*b)
                    .map(|p| p.name().to_string_lossy().to_string())
                    .unwrap_or_default();
                na.cmp(&nb)
            });
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
        self.sel = self.sel.min(self.flat_pids.len().saturating_sub(1));
    }

    fn proc_rows(&self) -> usize {
        (self.term_h as u16)
            .saturating_sub(self.used_rows() as u16)
            .max(3) as usize
    }

    fn used_rows(&self) -> usize {
        // sep-after-header + MEM + SWAP + disks + per-core-cpu-grid + sep-after-metrics
        let cpu_rows = if self.show_cpu {
            let cpus = self.sys.cpus().len();
            let cell_w = 22; // "C 0 [---------- 100%]" ~22 chars
            let num_cols = (self.term_w / cell_w).max(1);
            (cpus + num_cols - 1) / num_cols
        } else {
            0
        };
        1 + 1 + 1 + self.disks.len() + cpu_rows + 1
    }

    fn render(&mut self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        let cols = self.term_w;
        let fg_info = theme.shell.info.to_crossterm();
        let fg_success = theme.shell.success.to_crossterm();
        let fg_warn = theme.shell.warning.to_crossterm();
        let fg_err = theme.shell.error.to_crossterm();
        let bg_status = theme.editor.status_bg.to_crossterm();
        let bg_sel = theme.editor.selection_bg.to_crossterm();

        queue!(stdout, terminal::Clear(ClearType::All))?;

        let sort_name = match self.sort_by {
            SortBy::Cpu => "CPU",
            SortBy::Mem => "MEM",
            SortBy::Pid => "PID",
            SortBy::Name => "NAME",
        };
        let cpu_label = if self.show_cpu { "hide" } else { "show" };
        let header = format!(
            " sys  |  q=quit  s=sort  arrows=nav  k=kill  c={cpu_label}CPU  F=filter  Enter=expand  |  sort: {sort_name}"
        );
        queue!(
            stdout,
            MoveTo(0, 0),
            SetForegroundColor(fg_info),
            SetBackgroundColor(bg_status),
            Print(&header[..header.len().min(cols)]),
            ResetColor,
        )?;

        let mut row: u16 = 1;
        queue!(
            stdout,
            MoveTo(0, row),
            Print(&"\u{2500}".repeat(cols)),
            ResetColor
        )?;
        row += 1;

        // ── Memory ──
        let total_mem = self.sys.total_memory();
        let used_mem = self.sys.used_memory();
        let mem_pct = used_mem
            .checked_mul(100)
            .and_then(|v| v.checked_div(total_mem))
            .unwrap_or(0) as usize;
        let bar = render_text_bar(mem_pct);
        let fg = if mem_pct > 80 {
            fg_err
        } else if mem_pct > 50 {
            fg_warn
        } else {
            fg_success
        };
        queue!(
            stdout,
            MoveTo(0, row),
            SetForegroundColor(fg),
            Print(&format!(
                " MEM  {bar}  {} / {}",
                format_size(used_mem),
                format_size(total_mem)
            )),
            ResetColor,
        )?;
        row += 1;

        // ── Swap ──
        let total_swap = self.sys.total_swap();
        let used_swap = self.sys.used_swap();
        let swap_pct = used_swap
            .checked_mul(100)
            .and_then(|v| v.checked_div(total_swap))
            .unwrap_or(0) as usize;
        let bar = render_text_bar(swap_pct);
        let fg = if swap_pct > 80 {
            fg_err
        } else if swap_pct > 50 {
            fg_warn
        } else {
            fg_success
        };
        queue!(
            stdout,
            MoveTo(0, row),
            SetForegroundColor(fg),
            Print(&format!(
                " SWAP {bar}  {} / {}",
                format_size(used_swap),
                format_size(total_swap)
            )),
            ResetColor,
        )?;
        row += 1;

        // ── Disks (every volume sysinfo lists: C:\, D:\, USB, …) ──
        for disk in &self.disks {
            if row >= (self.term_h as u16).saturating_sub(1) {
                break;
            }
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
            queue!(
                stdout,
                MoveTo(0, row),
                SetForegroundColor(fg),
                Print(&format_disk_line(
                    &disk.mount_point().to_string_lossy(),
                    &disk.name().to_string_lossy(),
                    &disk.file_system().to_string_lossy(),
                    disk.kind(),
                    disk.is_removable(),
                    disk.is_read_only(),
                    used,
                    total,
                )),
                ResetColor,
            )?;
            row += 1;
        }

        // ── Per-core CPU grid (toggled by 'c') ──
        if self.show_cpu {
            let cpus = self.sys.cpus();
            let core_label_w = 4; // "C 0 " or "C10 "
            let bar16 = render_text_bar(100); // max width bar for column sizing
            let cell_w = core_label_w + 1 + bar16.len(); // label + space + bar
            let num_cols = ((cols as usize) / cell_w).max(1);
            let num_rows = (cpus.len() + num_cols - 1) / num_cols;

            for ri in 0..num_rows {
                if row >= (self.term_h as u16).saturating_sub(1) {
                    break;
                }
                let mut line_x = 2u16;
                for ci in 0..num_cols {
                    let idx = ri * num_cols + ci;
                    if idx >= cpus.len() {
                        break;
                    }
                    let pct = cpus[idx].cpu_usage() as usize;
                    let bar = render_text_bar(pct);
                    let fg = if pct > 80 {
                        fg_err
                    } else if pct > 50 {
                        fg_warn
                    } else {
                        fg_success
                    };
                    queue!(
                        stdout,
                        MoveTo(line_x, row),
                        SetForegroundColor(fg),
                        Print(&format!("C{:>2} {}", idx, bar)),
                        ResetColor,
                    )?;
                    line_x += cell_w as u16;
                }
                row += 1;
            }
        }

        // Separator after metrics
        if row < self.term_h as u16 {
            queue!(
                stdout,
                MoveTo(0, row),
                Print(&"\u{2500}".repeat(cols)),
                ResetColor
            )?;
            row += 1;
        }

        // ── Filter bar ──
        if self.filtering {
            let filter_display = format!(" filter: {}_", self.filter);
            queue!(
                stdout,
                MoveTo(0, row),
                SetForegroundColor(fg_info),
                SetBackgroundColor(bg_status),
                Print(&filter_display[..filter_display.len().min(cols)]),
                ResetColor,
            )?;
            row += 1;
        } else if !self.filter.is_empty() {
            let filter_display = format!(" filter: {}  (F to edit, Esc to clear)", self.filter);
            queue!(
                stdout,
                MoveTo(0, row),
                SetForegroundColor(fg_warn),
                Print(&filter_display[..filter_display.len().min(cols)]),
                ResetColor,
            )?;
            row += 1;
        }

        // ── Process list header ──
        let proc_h = (self.term_h as u16).saturating_sub(row).max(3) as usize;
        let sort_label = match self.sort_by {
            SortBy::Cpu => "CPU%",
            SortBy::Mem => "MEM%",
            SortBy::Pid => "PID",
            SortBy::Name => "NAME",
        };
        let header_str = format!(
            " {:>6} {:>6} {:>7}  {}",
            "PID", sort_label, "CPU%", "Command"
        );
        queue!(
            stdout,
            MoveTo(0, row),
            SetForegroundColor(fg_info),
            Print(&header_str[..header_str.len().min(cols)]),
            ResetColor,
        )?;
        row += 1;

        let total_procs = self.flat_pids.len();
        if self.sel >= self.scroll + proc_h {
            self.scroll = self.sel.saturating_add(1).saturating_sub(proc_h);
        }
        if self.sel < self.scroll {
            self.scroll = self.sel;
        }

        // Build children map from ALL system processes (not just flat_pids)
        // so cumulative totals work even for collapsed parents.
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
            let own = sys
                .process(pid)
                .map(|p| (p.cpu_usage(), p.memory()))
                .unwrap_or((0.0, 0));
            let (mut tcpu, mut tmem) = own;
            if let Some(kids) = children.get(&pid) {
                for &k in kids {
                    let (c, m) = sum_tree(k, children, sys);
                    tcpu += c;
                    tmem += m;
                }
            }
            (tcpu, tmem)
        }

        for i in 0..proc_h {
            let idx = self.scroll + i;
            let r = row + i as u16;
            if r >= self.term_h as u16 {
                break;
            }
            if idx >= total_procs {
                queue!(stdout, MoveTo(0, r), Clear(ClearType::UntilNewLine))?;
                continue;
            }
            let pid = self.flat_pids[idx];
            let depth = self.depth_map.get(&pid).copied().unwrap_or(0);
            let has_children = tree_children.contains_key(&pid);
            let is_expanded = self.expanded.contains(&pid);

            let proc_info = match self.sys.process(pid) {
                Some(p) => p,
                None => continue,
            };
            let name = proc_info.name().to_string_lossy();
            let (cpu, mem) = if has_children {
                sum_tree(pid, &tree_children, &self.sys)
            } else {
                (proc_info.cpu_usage(), proc_info.memory())
            };
            let mem_mb = mem as f64 / 1024.0 / 1024.0;
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

            let is_sel = idx == self.sel;
            let line = format!("{indent}{toggle}{pid:>6} {mem_mb:>6.1}M {cpu:>6.1}  {name} {cmd}");
            let line_ref: &str = &line;
            let truncated = if line.len() > cols {
                &line_ref[..cols]
            } else {
                line_ref
            };

            if is_sel {
                queue!(
                    stdout,
                    MoveTo(0, r),
                    SetBackgroundColor(bg_sel),
                    Print(truncated),
                    ResetColor,
                )?;
            } else {
                queue!(stdout, MoveTo(0, r), Print(truncated), ResetColor)?;
            }
        }

        stdout.flush()?;
        Ok(())
    }
}

fn terminal_size() -> (usize, usize) {
    crossterm::terminal::size()
        .map(|(w, h)| (w as usize, h as usize))
        .unwrap_or((80, 24))
}

fn render_text_bar(pct: usize) -> String {
    let pct_str = pct.to_string();
    let fill = (pct * 10 / 100).min(10);
    let empty = 10usize.saturating_sub(fill);
    format!("[{}{} {}%]", "-".repeat(fill), " ".repeat(empty), pct_str)
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

#[cfg(test)]
mod tests {
    use super::*;

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

pub fn cmd_sys(_args: &str, _nav: &mut crate::navigator::Navigator) -> Result<bool> {
    let mut sm = SysMon::new();
    let mut stdout = stdout();
    let _guard = TerminalGuard;

    execute!(stdout, EnterAlternateScreen, Hide, EnableMouseCapture)?;
    terminal::enable_raw_mode()?;

    sm.render(&mut stdout)?;
    let mut last_refresh = Instant::now();

    loop {
        let poll_ms = REFRESH_MS.min(200);
        if !event::poll(std::time::Duration::from_millis(poll_ms))? {
            if last_refresh.elapsed().as_millis() as u64 >= REFRESH_MS {
                sm.refresh();
                sm.render(&mut stdout)?;
                last_refresh = Instant::now();
            }
            continue;
        }

        let ev = event::read()?;
        match ev {
            Event::Key(ke) => {
                if ke.kind != KeyEventKind::Press {
                    continue;
                }
                match ke.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        if sm.filtering {
                            sm.filtering = false;
                            sm.render(&mut stdout)?;
                            continue;
                        }
                        break;
                    }
                    _ if sm.filtering => {
                        match ke.code {
                            KeyCode::Enter => {
                                sm.filtering = false;
                                sm.refresh_filtered();
                            }
                            KeyCode::Backspace => {
                                sm.filter.pop();
                                sm.refresh_filtered();
                            }
                            KeyCode::Char(c) => {
                                sm.filter.push(c);
                                sm.refresh_filtered();
                            }
                            _ => {}
                        }
                        sm.render(&mut stdout)?;
                    }
                    KeyCode::Enter => {
                        if let Some(&pid) = sm.flat_pids.get(sm.sel) {
                            if sm.expanded.contains(&pid) {
                                sm.expanded.remove(&pid);
                            } else {
                                sm.expanded.insert(pid);
                            }
                            sm.refresh_filtered();
                            sm.render(&mut stdout)?;
                        }
                    }
                    KeyCode::Up => {
                        if sm.sel > 0 {
                            sm.sel -= 1;
                        }
                        sm.render(&mut stdout)?;
                    }
                    KeyCode::Down => {
                        if sm.sel + 1 < sm.flat_pids.len() {
                            sm.sel += 1;
                        }
                        sm.render(&mut stdout)?;
                    }
                    KeyCode::PageUp => {
                        let page = sm.proc_rows().max(1);
                        sm.sel = sm.sel.saturating_sub(page);
                        sm.render(&mut stdout)?;
                    }
                    KeyCode::PageDown => {
                        let page = sm.proc_rows().max(1);
                        sm.sel = (sm.sel + page).min(sm.flat_pids.len().saturating_sub(1));
                        sm.render(&mut stdout)?;
                    }
                    KeyCode::Char('s') => {
                        sm.sort_by = match sm.sort_by {
                            SortBy::Cpu => SortBy::Mem,
                            SortBy::Mem => SortBy::Pid,
                            SortBy::Pid => SortBy::Name,
                            SortBy::Name => SortBy::Cpu,
                        };
                        sm.expanded.clear();
                        sm.refresh_filtered();
                        sm.sel = 0;
                        sm.scroll = 0;
                        sm.render(&mut stdout)?;
                    }
                    KeyCode::Char('S') => {
                        sm.sort_asc = !sm.sort_asc;
                        sm.expanded.clear();
                        sm.refresh_filtered();
                        sm.render(&mut stdout)?;
                    }
                    KeyCode::Char('c') => {
                        sm.show_cpu = !sm.show_cpu;
                        sm.render(&mut stdout)?;
                    }
                    KeyCode::Char('F') => {
                        sm.filtering = !sm.filtering;
                        if !sm.filtering {
                            sm.refresh_filtered();
                        }
                        sm.render(&mut stdout)?;
                    }
                    KeyCode::Char('k') => {
                        if let Some(&pid) = sm.flat_pids.get(sm.sel) {
                            let pid_str = pid.to_string();
                            #[cfg(windows)]
                            let _ = std::process::Command::new("taskkill")
                                .args(["/F", "/PID", &pid_str])
                                .status();
                            #[cfg(not(windows))]
                            let _ = std::process::Command::new("kill")
                                .args(["-9", &pid_str])
                                .status();
                            sm.expanded.remove(&pid);
                            sm.killed.insert(pid);
                            // Directly remove from flat list so it vanishes immediately
                            if sm.sel > 0 && sm.sel >= sm.flat_pids.len().saturating_sub(1) {
                                sm.sel -= 1;
                            }
                            sm.flat_pids.retain(|p| *p != pid);
                            sm.depth_map.remove(&pid);
                            sm.render(&mut stdout)?;
                        }
                    }
                    _ => {}
                }
            }
            Event::Resize(w, h) => {
                sm.term_w = w as usize;
                sm.term_h = h as usize;
                sm.render(&mut stdout)?;
            }
            Event::Mouse(m) => match m.kind {
                crossterm::event::MouseEventKind::ScrollUp => {
                    if sm.sel > 0 {
                        sm.sel -= 1;
                    }
                    sm.render(&mut stdout)?;
                }
                crossterm::event::MouseEventKind::ScrollDown => {
                    if sm.sel + 1 < sm.flat_pids.len() {
                        sm.sel += 1;
                    }
                    sm.render(&mut stdout)?;
                }
                _ => {}
            },
            _ => {}
        }

        if last_refresh.elapsed().as_millis() as u64 >= REFRESH_MS {
            sm.refresh();
            last_refresh = Instant::now();
        }
    }

    Ok(false)
}
