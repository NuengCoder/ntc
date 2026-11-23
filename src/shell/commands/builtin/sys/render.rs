//! Rendering for the `sys` monitor: gauges, disk rows and the full-screen
//! paint pass.

use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{self, Clear, ClearType};
use std::collections::HashMap;
use std::io::Write;
use sysinfo::DiskKind;

use super::monitor::{BodyLine, SortBy, SysMon, STATUS_MS};
use crate::utils::ansi_text::truncate_visible;

pub(super) fn terminal_size() -> (usize, usize) {
    crossterm::terminal::size()
        .map(|(w, h)| (w as usize, h as usize))
        .unwrap_or((80, 24))
}

/// Fixed-width 17-cell gauge: `[---------- 100%]`. The percentage is padded to
/// three cells so bars keep their columns while the number changes.
pub(super) fn render_text_bar(pct: usize) -> String {
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
pub(super) fn cpu_grid_lines(usages: &[usize], cols: usize) -> Vec<String> {
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
pub(super) fn paint(use_color: bool, color: Color) -> Color {
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

impl SysMon {
    pub(super) fn render(&mut self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
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

        // ── CPU (total) ──
        let cpu_pct = self.sys.global_cpu_usage() as usize;
        let cpu_fg = if cpu_pct > 80 {
            fg_err
        } else if cpu_pct > 50 {
            fg_warn
        } else {
            fg_success
        };
        body.push(BodyLine::colored(
            format!(" CPU  {}", render_text_bar(cpu_pct)),
            cpu_fg,
        ));

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
            // ones) so cumulative totals work for collapsed parents too. The
            // flat view has no tree, so it skips this entirely.
            let tree_children: HashMap<sysinfo::Pid, Vec<sysinfo::Pid>> = if self.flat_view {
                HashMap::new()
            } else {
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
                let has_children = !self.flat_view && tree_children.contains_key(&pid);
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

                // Flat view rows carry only the process's own usage.
                let indent = if self.flat_view {
                    String::new()
                } else {
                    "  ".repeat(depth)
                };
                let toggle = if self.flat_view {
                    "  "
                } else if has_children {
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
            ("v", "toggle tree / flat full-list view"),
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
