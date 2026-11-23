//! `tmprt com` — live hardware temperature monitor.

use anyhow::Result;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, queue};
use std::io::stdout;
#[cfg(windows)]
use std::io::IsTerminal;
use std::io::Write;
use std::time::{Duration, Instant};
use sysinfo::{Components, System};

#[cfg(windows)]
use super::sensors::{disk_temps, install_lhm, lhm_dir, lhm_temps};
use super::sensors::{is_elevated, lhm_installed, pawnio_installed, sane_temp, LHM_REFRESH_EVERY};
use super::ui::{
    format_size, pct_color, render_text_bar, temp_color, terminal_size, to_unit, truncate,
};
#[cfg(windows)]
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::t;

const DEFAULT_REFRESH_MS: u64 = 2000;

/// A single temperature reading from any backend.
#[derive(Clone)]
pub(super) struct SensorReading {
    label: String,
    temp_c: Option<f32>,
    max_c: Option<f32>,
    critical_c: Option<f32>,
}

impl SensorReading {
    pub(super) fn live(
        label: impl Into<String>,
        temp_c: f32,
        max_c: Option<f32>,
        critical_c: Option<f32>,
    ) -> Self {
        Self {
            label: label.into(),
            temp_c: Some(temp_c),
            max_c,
            critical_c,
        }
    }

    /// A sensor that was detected but could not be read (renders as `--`).
    #[cfg(windows)]
    pub(super) fn unavailable(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            temp_c: None,
            max_c: None,
            critical_c: None,
        }
    }
}

struct TempMon {
    sys: System,
    components: Components,
    readings: Vec<SensorReading>,
    /// Cached LHM + disk readings, refreshed on the slow (5 s) cadence.
    heavy_readings: Vec<SensorReading>,
    term_w: usize,
    term_h: usize,
    celsius: bool,
    frozen: bool,
    last_heavy: Instant,
    elevated: bool,
    pawnio: bool,
}

impl TempMon {
    fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        std::thread::sleep(Duration::from_millis(200));
        sys.refresh_all();
        let mut components = Components::new_with_refreshed_list();
        let heavy_readings = collect_heavy_temps();
        let readings = collect_temps(&mut components, &heavy_readings);
        let (w, h) = terminal_size();
        Self {
            sys,
            components,
            readings,
            heavy_readings,
            term_w: w,
            term_h: h,
            celsius: true,
            frozen: false,
            last_heavy: Instant::now(),
            elevated: is_elevated(),
            pawnio: pawnio_installed(),
        }
    }

    fn refresh(&mut self) {
        if self.frozen {
            return;
        }
        self.sys.refresh_all();
        if self.last_heavy.elapsed() >= LHM_REFRESH_EVERY {
            self.last_heavy = Instant::now();
            self.heavy_readings = collect_heavy_temps();
        }
        self.readings = collect_temps(&mut self.components, &self.heavy_readings);
    }

    fn render(&mut self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let theme = crate::utils::theme::ThemeManager::current();
        let fg_info = theme.shell.info.to_crossterm();
        let fg_success = theme.shell.success.to_crossterm();
        let fg_warn = theme.shell.warning.to_crossterm();
        let fg_err = theme.shell.error.to_crossterm();
        let bg_status = theme.editor.status_bg.to_crossterm();
        let cols = self.term_w;

        queue!(stdout, terminal::Clear(ClearType::All))?;

        let state = if self.frozen { "frozen" } else { "live" };
        let unit = if self.celsius { 'C' } else { 'F' };
        let header =
            format!(" tmprt  |  q=quit  s=freeze  c='oC/'oF  |  state: {state}  |  unit: 'o{unit}");
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

        // ── CPU usage ──
        let cpu_usage = self.sys.global_cpu_usage() as usize;
        let bar = render_text_bar(cpu_usage);
        let fg = pct_color(cpu_usage, fg_success, fg_warn, fg_err);
        queue!(
            stdout,
            MoveTo(0, row),
            SetForegroundColor(fg),
            Print(&format!(" CPU  {bar}  {cpu_usage}%")),
            ResetColor,
        )?;
        row += 1;

        // ── CPU details ──
        let brand = self
            .sys
            .cpus()
            .first()
            .map(|c| c.brand().to_string())
            .unwrap_or_default();
        let freq_mhz = self.sys.cpus().first().map(|c| c.frequency()).unwrap_or(0);
        let cores = self.sys.cpus().len();
        let cpu_line = format!(
            " {:.2} GHz  {cores} cores  {brand}",
            freq_mhz as f64 / 1000.0
        );
        queue!(
            stdout,
            MoveTo(0, row),
            SetForegroundColor(fg_info),
            Print(&cpu_line[..cpu_line.len().min(cols)]),
            ResetColor,
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
        let fg = pct_color(mem_pct, fg_success, fg_warn, fg_err);
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

        queue!(
            stdout,
            MoveTo(0, row),
            Print(&"\u{2500}".repeat(cols)),
            ResetColor
        )?;
        row += 1;

        // ── Temperature sensors ──
        if self.readings.is_empty() {
            if row < self.term_h as u16 {
                queue!(
                    stdout,
                    MoveTo(0, row),
                    SetForegroundColor(fg_warn),
                    Print(&truncate(&t!("cmd.tmprt.com.no_sensors"), cols)),
                    ResetColor,
                )?;
                row += 1;
                if row < self.term_h as u16 {
                    queue!(
                        stdout,
                        MoveTo(0, row),
                        SetForegroundColor(fg_info),
                        Print(&truncate(&t!("cmd.tmprt.com.no_sensors_hint"), cols)),
                        ResetColor,
                    )?;
                    row += 1;
                }
            }
        } else {
            queue!(
                stdout,
                MoveTo(0, row),
                SetForegroundColor(fg_info),
                Print(" TEMPERATURE SENSORS"),
                ResetColor,
            )?;
            row += 1;
            let head = format!(
                " {:width$}  {:>10}  {:>10}  {:>10}",
                "SENSOR",
                "NOW",
                "MAX",
                "CRIT",
                width = 22
            );
            queue!(
                stdout,
                MoveTo(0, row),
                SetForegroundColor(fg_info),
                Print(&head[..head.len().min(cols)]),
                ResetColor,
            )?;
            row += 1;

            for sensor in &self.readings {
                if row >= (self.term_h as u16).saturating_sub(1) {
                    break;
                }
                let (now_str, max_str, crit_str, fg) = match sensor.temp_c {
                    Some(c) => {
                        let fg = temp_color(c, sensor.critical_c, fg_success, fg_warn, fg_err);
                        (
                            format!("{:.1}'o", to_unit(c, self.celsius)),
                            sensor.max_c.map_or_else(
                                || "--".to_string(),
                                |m| format!("{:.1}'o", to_unit(m, self.celsius)),
                            ),
                            sensor.critical_c.map_or_else(
                                || "--".to_string(),
                                |cr| format!("{:.1}'o", to_unit(cr, self.celsius)),
                            ),
                            fg,
                        )
                    }
                    _ => {
                        let fg = fg_info;
                        ("--".to_string(), "--".to_string(), "--".to_string(), fg)
                    }
                };
                let line = format!(
                    " {label:<22}  {now_str:>10}  {max_str:>10}  {crit_str:>10}",
                    label = sensor.label
                );
                queue!(
                    stdout,
                    MoveTo(0, row),
                    SetForegroundColor(fg),
                    Print(&line[..line.len().min(cols)]),
                    ResetColor,
                )?;
                row += 1;
            }
        }

        // ── Hint: LHM CPU sensors need the PawnIO driver + elevation ──
        if lhm_installed() && !self.elevated && row < self.term_h as u16 {
            let msg = if self.pawnio {
                t!("cmd.tmprt.com.admin_hint")
            } else {
                t!("cmd.tmprt.com.pawnio_hint")
            };
            queue!(
                stdout,
                MoveTo(0, row),
                SetForegroundColor(fg_warn),
                Print(&truncate(&msg, cols)),
                ResetColor,
            )?;
            row += 1;
        }

        // ── Footer hint ──
        if row < self.term_h as u16 {
            queue!(
                stdout,
                MoveTo(0, row),
                SetForegroundColor(fg_info),
                Print(&truncate(&t!("cmd.tmprt.com.footer"), cols)),
                ResetColor,
            )?;
        }

        stdout.flush()?;
        Ok(())
    }
}

pub(super) fn cmd_tmprt_com(args: &str) -> Result<bool> {
    let interval_ms = args
        .trim()
        .parse::<u64>()
        .ok()
        .map(|s| s.clamp(1, 60) * 1000)
        .unwrap_or(DEFAULT_REFRESH_MS);

    #[cfg(windows)]
    {
        let libs = lhm_dir();
        if !libs.join("LibreHardwareMonitorLib.dll").exists() && stdout().is_terminal() {
            print_warning(&t!("cmd.tmprt.com.lhm_missing"));
            print_info(&t!("cmd.tmprt.com.lhm_prompt"));
            print!("{} ", t!("cmd.tmprt.com.lhm_confirm"));
            let _ = stdout().flush();
            let mut line = String::new();
            let _ = std::io::stdin().read_line(&mut line);
            let answer = line.trim().to_lowercase();
            if answer.is_empty() || answer == "y" || answer == "yes" {
                print_info(&t!("cmd.tmprt.com.lhm_installing"));
                match install_lhm() {
                    Ok(()) => print_success(&t!("cmd.tmprt.com.lhm_installed")),
                    Err(e) => print_error(&t!("cmd.tmprt.com.lhm_install_failed", e)),
                }
            }
        }
    }

    let mut tm = TempMon::new();
    let mut stdout = stdout();
    let _guard = TerminalGuard;

    execute!(stdout, EnterAlternateScreen, Hide)?;
    terminal::enable_raw_mode()?;

    tm.render(&mut stdout)?;
    let mut last_refresh = Instant::now();

    loop {
        let poll_ms = interval_ms.min(200);
        if !event::poll(Duration::from_millis(poll_ms))? {
            if last_refresh.elapsed().as_millis() as u64 >= interval_ms {
                tm.refresh();
                tm.render(&mut stdout)?;
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
                    KeyCode::Esc | KeyCode::Char('q') => break,
                    KeyCode::Char('s') => {
                        tm.frozen = !tm.frozen;
                        tm.render(&mut stdout)?;
                    }
                    KeyCode::Char('c') => {
                        tm.celsius = !tm.celsius;
                        tm.render(&mut stdout)?;
                    }
                    _ => {}
                }
            }
            Event::Resize(w, h) => {
                tm.term_w = w as usize;
                tm.term_h = h as usize;
                tm.render(&mut stdout)?;
            }
            _ => {}
        }

        if last_refresh.elapsed().as_millis() as u64 >= interval_ms {
            tm.refresh();
            last_refresh = Instant::now();
        }
    }

    Ok(false)
}

// ─────────────────────────────────────────────────────────────
// Sensor collection — multi-source aggregation
// ─────────────────────────────────────────────────────────────

fn collect_temps(components: &mut Components, heavy: &[SensorReading]) -> Vec<SensorReading> {
    let mut out = Vec::new();
    components.refresh(false);
    out.extend(sysinfo_temps(components));
    #[cfg(windows)]
    {
        // Prefer LHM GPU readings when they are real; otherwise fall back to
        // nvidia-smi and hide LHM's unreadable GPU rows (CPU rows stay as `--`).
        let lhm_gpu_ok = heavy
            .iter()
            .any(|r| r.temp_c.is_some() && r.label.starts_with("GPU "));
        if lhm_gpu_ok {
            out.extend(heavy.iter().cloned());
        } else {
            out.extend(
                heavy
                    .iter()
                    .filter(|r| !r.label.starts_with("GPU "))
                    .cloned(),
            );
            out.extend(nvidia_smi_temps());
        }
    }
    #[cfg(not(windows))]
    {
        out.extend(heavy.iter().cloned());
        out.extend(nvidia_smi_temps());
    }
    out
}

/// Slow (5 s cadence) probes: LibreHardwareMonitorLib + disk SMART.
#[cfg(windows)]
fn collect_heavy_temps() -> Vec<SensorReading> {
    let mut out = disk_temps();
    out.extend(lhm_temps());
    out
}

#[cfg(not(windows))]
fn collect_heavy_temps() -> Vec<SensorReading> {
    Vec::new()
}

/// ACPI/hwmon sensors exposed through sysinfo.
fn sysinfo_temps(components: &Components) -> Vec<SensorReading> {
    components
        .list()
        .iter()
        .filter_map(|c| {
            let t = c.temperature().filter(|t| !t.is_nan())?;
            if !sane_temp(t) {
                return None;
            }
            Some(SensorReading::live(
                c.label().to_string(),
                t,
                c.max().filter(|m| !m.is_nan() && *m > 0.0),
                c.critical().filter(|m| !m.is_nan() && *m > 0.0),
            ))
        })
        .collect()
}

/// NVIDIA GPU temperatures via `nvidia-smi` (works on any OS with the driver).
fn nvidia_smi_temps() -> Vec<SensorReading> {
    let mut out = Vec::new();
    let Ok(res) = std::process::Command::new("nvidia-smi")
        .args([
            "--query-gpu=name,temperature.gpu,fan.speed",
            "--format=csv,noheader,nounits",
        ])
        .output()
    else {
        return out;
    };
    if !res.status.success() {
        return out;
    }
    for line in String::from_utf8_lossy(&res.stdout).lines() {
        let mut parts = line.split(',').map(|s| s.trim());
        let name = parts.next().unwrap_or("NVIDIA").to_string();
        let gpu = parts.next().and_then(|s| s.parse::<f32>().ok());
        if let Some(t) = gpu {
            if sane_temp(t) {
                out.push(SensorReading::live(format!("GPU {name}"), t, None, None));
            }
        }
    }
    out
}

struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
        let _ = execute!(std::io::stdout(), Show, LeaveAlternateScreen);
    }
}
