//! `tmprt` command: hardware temperature monitor (`tmprt com`) and
//! OpenWeather current weather (`tmprt weather`).
//!
//! Subcommands:
//! - `tmprt com [secs]`          live HWMonitor-style sensor view
//! - `tmprt weather [city]`      current weather via OpenWeather API
//! - `tmprt key [API_KEY]`       save/display the OpenWeather API key
//! - `tmprt city [name]`         save/display the default city
//! - `tmprt sensors <sub>`       manage hardware sensor backends (LHM)

use crate::config::Config;
use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::t;
use anyhow::Result;
use colored::*;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, queue};
use serde::Deserialize;
use std::io::{stdout, Write};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use sysinfo::{Components, System};

#[cfg(windows)]
use anyhow::anyhow;
#[cfg(windows)]
use indicatif::{ProgressBar, ProgressStyle};
#[cfg(windows)]
use std::io::{IsTerminal, Read};
#[cfg(windows)]
use std::path::Path;

const DEFAULT_REFRESH_MS: u64 = 2000;
const WEATHER_URL: &str = "https://api.openweathermap.org/data/2.5/weather";

#[cfg(windows)]
const LHM_URL: &str = "https://github.com/LibreHardwareMonitor/LibreHardwareMonitor/releases/download/v0.9.6/LibreHardwareMonitor.zip";
#[cfg(windows)]
const LHM_REFRESH_EVERY: Duration = Duration::from_secs(5);
#[cfg(not(windows))]
const LHM_REFRESH_EVERY: Duration = Duration::from_secs(5);
/// PawnIO: the signed, HVCI-compatible kernel driver LHM (>= 0.9.5) uses
/// instead of WinRing0. Installed once (elevated) by `tmprt sensors install`.
#[cfg(windows)]
const PAWNIO_URL: &str =
    "https://github.com/namazso/PawnIO.Setup/releases/download/2.2.0/PawnIO_setup.exe";

pub fn cmd_tmprt(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    let sub = parts.first().copied().unwrap_or("").to_lowercase();
    let rest = parts.get(1).unwrap_or(&"").trim();

    match sub.as_str() {
        "com" | "computer" | "hw" | "hardware" => cmd_tmprt_com(rest),
        "weather" | "w" => cmd_tmprt_weather(rest),
        "key" => cmd_tmprt_key(rest),
        "city" => cmd_tmprt_city(rest),
        "sensors" | "sensor" => cmd_tmprt_sensors(rest),
        "" | "help" | "--help" | "-h" => {
            print_usage();
            Ok(false)
        }
        _ => {
            print_error(&t!("cmd.tmprt.unknown_sub", sub));
            println!();
            print_usage();
            Ok(false)
        }
    }
}

// ─────────────────────────────────────────────────────────────
// tmprt com — interactive hardware temperature monitor
// ─────────────────────────────────────────────────────────────

/// A single temperature reading from any backend.
#[derive(Clone)]
struct SensorReading {
    label: String,
    temp_c: Option<f32>,
    max_c: Option<f32>,
    critical_c: Option<f32>,
}

impl SensorReading {
    fn live(
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
    fn unavailable(label: impl Into<String>) -> Self {
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

fn cmd_tmprt_com(args: &str) -> Result<bool> {
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

/// Whether the current process has administrator privileges (Windows).
///
/// The LibreHardwareMonitorLib WinRing0 kernel driver only loads elevated, so
/// CPU temperatures are unreadable from a standard (non-admin) process.
#[cfg(windows)]
fn is_elevated() -> bool {
    std::process::Command::new("whoami")
        .args(["/groups"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).contains("S-1-16-12288"))
        .unwrap_or(false)
}

#[cfg(not(windows))]
fn is_elevated() -> bool {
    false
}

#[cfg(not(windows))]
fn pawnio_installed() -> bool {
    false
}

/// Whether the LibreHardwareMonitorLib DLLs are installed.
fn lhm_installed() -> bool {
    lhm_dir().join("LibreHardwareMonitorLib.dll").exists()
}

fn sane_temp(t: f32) -> bool {
    !t.is_nan() && !t.is_infinite() && (-40.0..150.0).contains(&t)
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

/// Storage temperatures via PowerShell (Windows only).
#[cfg(windows)]
fn disk_temps() -> Vec<SensorReading> {
    const SCRIPT: &str = r#"
Get-PhysicalDisk | ForEach-Object {
  $r = Get-StorageReliabilityCounter -PhysicalDisk $_ -ErrorAction SilentlyContinue
  if ($r -and $r.Temperature) {
    Write-Output ('{0}:{1}' -f $_.FriendlyName, [Math]::Round(($r.Temperature - 273.15), 1))
  }
}
"#;
    let mut out = Vec::new();
    if let Ok(stdout) = run_ps(SCRIPT, &[]) {
        for line in stdout.lines() {
            if let Some((name, val)) = line.split_once(':') {
                if let Ok(t) = val.trim().parse::<f32>() {
                    if sane_temp(t) {
                        out.push(SensorReading::live(format!("DISK {name}"), t, None, None));
                    }
                }
            }
        }
    }
    out
}

/// CPU/GPU/board/storage temperatures via LibreHardwareMonitorLib (Windows only).
#[cfg(windows)]
fn lhm_temps() -> Vec<SensorReading> {
    let libs = lhm_dir();
    if !libs.join("LibreHardwareMonitorLib.dll").exists() {
        return Vec::new();
    }
    const SCRIPT: &str = r#"
param([string]$dir)
Unblock-File -Path (Join-Path $dir '*.*') -ErrorAction SilentlyContinue
# Load dependencies first so assembly references (e.g. System.Memory) resolve.
Get-ChildItem -Path $dir -Filter '*.dll' | Where-Object { $_.Name -ne 'LibreHardwareMonitorLib.dll' } | ForEach-Object { Add-Type -Path $_.FullName -ErrorAction SilentlyContinue }
Add-Type -Path (Join-Path $dir 'LibreHardwareMonitorLib.dll')
$computer = New-Object LibreHardwareMonitor.Hardware.Computer
$computer.IsCpuEnabled = $true
$computer.IsGpuEnabled = $true
$computer.IsMotherboardEnabled = $true
$computer.IsStorageEnabled = $true
$computer.IsBatteryEnabled = $true
$computer.IsControllerEnabled = $true
$computer.Open()
try {
  # Warm up the sensors so values populate (first read is often 0/null).
  foreach ($hardware in $computer.Hardware) {
    foreach ($h in @($hardware) + $hardware.SubHardware) { $h.Update() }
  }
  Start-Sleep -Milliseconds 400
  foreach ($hardware in $computer.Hardware) {
    $hardware.Update()
    foreach ($h in @($hardware) + $hardware.SubHardware) {
      $h.Update()
      $temps = @($h.Sensors | Where-Object { $_.SensorType -eq 'Temperature' -and $_.Name -notlike '*Distance to TjMax*' } | ForEach-Object {
        $v = if ($null -ne $_.Value) { [Math]::Round($_.Value, 1) } else { 0 }
        ('{0}:{1}' -f $_.Name, $v)
      })
      if ($temps.Count -gt 0) {
        Write-Output ('{0}>{1}>{2}' -f $h.HardwareType, $h.Name, ($temps -join '|'))
      }
    }
  }
}
finally {
  $computer.Close()
}
"#;
    let mut out = Vec::new();
    let libs_str = libs.to_string_lossy().to_string();
    if let Ok(stdout) = run_ps(SCRIPT, &["-dir", &libs_str]) {
        for line in stdout.lines() {
            let mut it = line.splitn(3, '>');
            let htype = it.next().unwrap_or("").trim();
            let hname = it.next().unwrap_or("").trim();
            let temps = it.next().unwrap_or("").trim();
            if temps.is_empty() {
                continue;
            }
            let short = if htype.starts_with("Cpu") {
                "CPU"
            } else if htype.starts_with("Gpu") {
                "GPU"
            } else if htype == "Motherboard" {
                "MB"
            } else if htype.starts_with("Storage") {
                "DISK"
            } else if htype == "Battery" {
                "BATT"
            } else if htype == "SuperIO" {
                "IO"
            } else {
                htype
            };
            for pair in temps.split('|') {
                if let Some((sname, val)) = pair.split_once(':') {
                    let label = format!("{short} {hname} {sname}");
                    // LHM reports 0.0 / null when it cannot read a sensor
                    // (e.g. no admin rights); those render as `--` rows.
                    let t = val.trim().parse::<f32>().unwrap_or(0.0);
                    if t == 0.0 || !sane_temp(t) {
                        out.push(SensorReading::unavailable(label));
                    } else {
                        out.push(SensorReading::live(label, t, None, None));
                    }
                }
            }
        }
    }
    out
}

/// Verbose LHM diagnostic dump: every hardware device and temperature sensor
/// with its raw value (null means the WinRing0 driver could not be loaded).
#[cfg(windows)]
fn lhm_debug() -> Result<String> {
    let libs = lhm_dir();
    if !libs.join("LibreHardwareMonitorLib.dll").exists() {
        return Err(anyhow!("not installed"));
    }
    const SCRIPT: &str = r#"
param([string]$dir)
Unblock-File -Path (Join-Path $dir '*.*') -ErrorAction SilentlyContinue
Get-ChildItem -Path $dir -Filter '*.dll' | Where-Object { $_.Name -ne 'LibreHardwareMonitorLib.dll' } | ForEach-Object { Add-Type -Path $_.FullName -ErrorAction SilentlyContinue }
Add-Type -Path (Join-Path $dir 'LibreHardwareMonitorLib.dll')
$computer = New-Object LibreHardwareMonitor.Hardware.Computer
$computer.IsCpuEnabled = $true
$computer.IsGpuEnabled = $true
$computer.IsMotherboardEnabled = $true
$computer.IsStorageEnabled = $true
$computer.IsBatteryEnabled = $true
$computer.IsControllerEnabled = $true
$computer.Open()
try {
  foreach ($hardware in $computer.Hardware) {
    $hardware.Update()
    Write-Output ('[{0}] {1}' -f $hardware.HardwareType, $hardware.Name)
    foreach ($h in @($hardware) + $hardware.SubHardware) {
      $h.Update()
      foreach ($s in $h.Sensors) {
        if ($s.SensorType -eq 'Temperature') {
          $v = if ($null -ne $s.Value) { [Math]::Round($s.Value, 1) } else { '<null>' }
          Write-Output ('    {0} = {1}' -f $s.Name, $v)
        }
      }
    }
  }
}
finally {
  $computer.Close()
}
"#;
    let libs_str = libs.to_string_lossy().to_string();
    run_ps(SCRIPT, &["-dir", &libs_str])
}

/// Run a PowerShell script file and return trimmed stdout (Windows only).
#[cfg(windows)]
fn run_ps(script: &str, args: &[&str]) -> Result<String> {
    let script_path = std::env::temp_dir().join(format!("ntc_tmprt_{}.ps1", std::process::id()));
    std::fs::write(&script_path, script)?;
    let out = std::process::Command::new("powershell")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
        ])
        .arg(&script_path)
        .args(args)
        .output()?;
    let _ = std::fs::remove_file(&script_path);
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(anyhow!("powershell failed: {}", stderr.trim()));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Directory holding downloaded LibreHardwareMonitor DLLs.
fn lhm_dir() -> PathBuf {
    dirs::config_dir()
        .map(|d| d.join("ntc").join("libs"))
        .unwrap_or_else(|| std::env::temp_dir().join("ntc_libs"))
}

/// Whether the PawnIO kernel driver is installed on the system.
///
/// PawnIO is a signed, HVCI/Memory-Integrity-compatible replacement for the
/// WinRing0 driver that LibreHardwareMonitor >= 0.9.5 relies on to read CPU
/// temperatures. It must be installed once (elevated) by `tmprt sensors
/// install`; merely having the LHM DLLs is not enough.
#[cfg(windows)]
fn pawnio_installed() -> bool {
    const SCRIPT: &str = r#"
if (Test-Path 'C:\Program Files\PawnIO') { Write-Output 'yes' }
elseif (Get-ItemProperty -Path 'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\PawnIO','HKLM:\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\PawnIO' -ErrorAction SilentlyContinue) { Write-Output 'yes' }
"#;
    run_ps(SCRIPT, &[])
        .map(|o| o.contains("yes"))
        .unwrap_or(false)
}

/// Download and silently install the PawnIO driver. Must be run elevated.
/// Returns the installer exit code (0 = ok, 3010 = reboot required).
#[cfg(windows)]
fn install_pawnio() -> Result<i32> {
    let setup = std::env::temp_dir().join("ntc_PawnIO_setup.exe");
    download_file(
        PAWNIO_URL,
        &setup,
        &t!("cmd.tmprt.sensors.pawnio_downloading"),
    )?;
    const SCRIPT: &str = r#"
param([string]$setup)
$p = Start-Process -FilePath $setup -ArgumentList '-install','-silent' -Wait -PassThru -NoNewWindow
if ($p) { Write-Output $p.ExitCode } else { Write-Output '1' }
"#;
    let setup_str = setup.to_string_lossy().to_string();
    let out = run_ps(SCRIPT, &["-setup", &setup_str])?;
    let _ = std::fs::remove_file(&setup);
    let code = out.trim().parse::<i32>().unwrap_or(1);
    // 183 = ERROR_ALREADY_EXISTS (driver already present): treat as success.
    Ok(if code == 183 { 0 } else { code })
}

/// Relaunch ntc elevated to run the PawnIO install (UAC prompt), wait for the
/// child to finish, and return its exit code. Used when the user runs
/// `tmprt sensors install` from a standard (non-admin) shell.
#[cfg(windows)]
fn self_elevate_pawnio_install() -> Result<i32> {
    let exe = std::env::current_exe()?;
    let exe_str = exe.to_string_lossy().replace('\'', "''");
    const SCRIPT: &str = r#"
param([string]$exe)
$p = Start-Process -Verb RunAs -FilePath $exe -ArgumentList 'tmprt','sensors','pawnio-install' -Wait -PassThru
if ($p) { Write-Output $p.ExitCode }
"#;
    let out = run_ps(SCRIPT, &["-exe", &exe_str])?;
    Ok(out.trim().parse::<i32>().unwrap_or(1))
}

/// Called after the LHM DLLs are copied: installs PawnIO when missing
/// (auto-elevating via UAC when required) and reports the outcome.
#[cfg(windows)]
fn install_pawnio_if_needed() {
    if pawnio_installed() {
        print_info(&t!("cmd.tmprt.sensors.pawnio_present"));
        return;
    }
    let code = if is_elevated() {
        install_pawnio()
    } else {
        print_info(&t!("cmd.tmprt.sensors.pawnio_elevating"));
        self_elevate_pawnio_install()
    };
    match code {
        Ok(0) => print_success(&t!("cmd.tmprt.sensors.pawnio_installed")),
        Ok(3010) => print_warning(&t!("cmd.tmprt.sensors.pawnio_reboot")),
        Ok(other) => print_error(&t!("cmd.tmprt.sensors.pawnio_failed", other)),
        Err(e) => print_error(&t!("cmd.tmprt.sensors.pawnio_failed", e)),
    }
}

/// Download and unpack the LibreHardwareMonitor DLLs (Windows only).
#[cfg(windows)]
fn install_lhm() -> Result<()> {
    let libs = lhm_dir();
    std::fs::create_dir_all(&libs)?;

    let zip_path = std::env::temp_dir().join("ntc_lhm_download.zip");
    download_file(LHM_URL, &zip_path, &t!("cmd.tmprt.sensors.downloading"))?;

    let extract_dir = std::env::temp_dir().join("ntc_lhm_extract");
    let _ = std::fs::remove_dir_all(&extract_dir);
    std::fs::create_dir_all(&extract_dir)?;
    extract_zip(&zip_path, &extract_dir)?;
    let _ = std::fs::remove_file(&zip_path);

    let mut copied = 0;
    if let Ok(entries) = std::fs::read_dir(&extract_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("dll") {
                continue;
            }
            let Some(name) = path.file_name() else {
                continue;
            };
            if std::fs::copy(&path, libs.join(name)).is_ok() {
                copied += 1;
            }
        }
    }
    let _ = std::fs::remove_dir_all(&extract_dir);
    if copied == 0 || !libs.join("LibreHardwareMonitorLib.dll").exists() {
        return Err(anyhow!("no required DLLs found in the downloaded archive"));
    }
    Ok(())
}

/// Download a URL to a file with a progress bar (Windows only).
#[cfg(windows)]
fn download_file(url: &str, dest: &Path, msg: &str) -> Result<()> {
    let pb = ProgressBar::new(0);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg}\n{wide_bar} {bytes}/{total_bytes}")?
            .progress_chars("##-"),
    );
    pb.set_message(msg.to_string());

    let response = ureq::get(url)
        .call()
        .map_err(|e| anyhow!("download failed: {e}"))?;
    let total = response
        .headers()
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(0);
    pb.set_length(total);

    let mut buf = Vec::new();
    let mut body = response.into_body();
    let mut reader = body.as_reader();
    let mut chunk = vec![0u8; 8192];
    loop {
        let n = reader.read(&mut chunk)?;
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&chunk[..n]);
        pb.set_position(buf.len() as u64);
    }
    pb.finish_and_clear();

    std::fs::write(dest, &buf)?;
    Ok(())
}

/// Flatten a zip archive into `dest` using only each entry's file name
/// (avoids path traversal and nested folders) (Windows only).
#[cfg(windows)]
fn extract_zip(zip_path: &Path, dest: &Path) -> Result<()> {
    let file = std::fs::File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        if entry.is_dir() {
            continue;
        }
        let name = entry.name().to_string();
        let fname = Path::new(&name)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        if fname.is_empty() {
            continue;
        }
        let out_path = dest.join(fname);
        let mut out = std::fs::File::create(&out_path)?;
        std::io::copy(&mut entry, &mut out)?;
    }
    Ok(())
}

// ─────────────────────────────────────────────────────────────
// tmprt sensors — backend management
// ─────────────────────────────────────────────────────────────

fn cmd_tmprt_sensors(args: &str) -> Result<bool> {
    let sub = args.trim().to_lowercase();
    match sub.as_str() {
        "install" => {
            #[cfg(windows)]
            {
                match install_lhm() {
                    Ok(()) => {
                        print_success(&t!("cmd.tmprt.sensors.installed"));
                        install_pawnio_if_needed();
                    }
                    Err(e) => print_error(&t!("cmd.tmprt.sensors.install_failed", e)),
                }
            }
            #[cfg(not(windows))]
            {
                print_error(&t!("cmd.tmprt.sensors.windows_only"));
            }
            Ok(false)
        }
        "pawnio-install" => {
            // Internal: relaunched elevated by `install` to install the PawnIO
            // driver. Exits with the installer's code so the parent can react.
            #[cfg(windows)]
            {
                let code = match install_pawnio() {
                    Ok(code) => code,
                    Err(e) => {
                        eprintln!("{e}");
                        1
                    }
                };
                std::process::exit(code);
            }
            #[cfg(not(windows))]
            {
                unreachable!("pawnio-install is Windows-only");
            }
        }
        "uninstall" => {
            let libs = lhm_dir();
            if libs.exists() {
                match std::fs::remove_dir_all(&libs) {
                    Ok(()) => print_success(&t!("cmd.tmprt.sensors.uninstalled")),
                    Err(e) => print_error(&t!("cmd.tmprt.sensors.uninstall_failed", e)),
                }
            } else {
                print_info(&t!("cmd.tmprt.sensors.not_installed"));
            }
            Ok(false)
        }
        "status" => {
            print_sensors_status();
            Ok(false)
        }
        "debug" => {
            print_sensors_debug();
            Ok(false)
        }
        _ => {
            print_sensors_help();
            Ok(false)
        }
    }
}

fn print_sensors_status() {
    println!("{}", t!("cmd.tmprt.sensors.status_title").cyan().bold());
    println!();

    let mut components = Components::new_with_refreshed_list();
    components.refresh(false);
    let n_acpi = components
        .list()
        .iter()
        .filter(|c| c.temperature().is_some_and(|t| !t.is_nan()))
        .count();
    println!("  {}: {}", t!("cmd.tmprt.sensors.src_acpi").cyan(), n_acpi);

    let nvidia = std::process::Command::new("nvidia-smi")
        .args(["--query-gpu=name", "--format=csv,noheader"])
        .output()
        .ok()
        .filter(|r| r.status.success())
        .map(|r| {
            String::from_utf8_lossy(&r.stdout)
                .lines()
                .next()
                .unwrap_or("")
                .trim()
                .to_string()
        });
    match nvidia {
        Some(name) if !name.is_empty() => {
            println!("  {}: {}", t!("cmd.tmprt.sensors.src_nvidia").cyan(), name)
        }
        _ => println!(
            "  {}: {}",
            t!("cmd.tmprt.sensors.src_nvidia").cyan(),
            t!("cmd.tmprt.sensors.src_unavailable")
        ),
    }

    #[cfg(windows)]
    {
        let installed = lhm_installed();
        let pawnio = pawnio_installed();
        println!(
            "  {}: {}",
            t!("cmd.tmprt.sensors.src_lhm").cyan(),
            if installed {
                t!("cmd.tmprt.sensors.src_installed")
            } else {
                t!("cmd.tmprt.sensors.src_not_installed")
            }
        );
        println!(
            "  {}: {}",
            t!("cmd.tmprt.sensors.src_pawnio").cyan(),
            if pawnio {
                t!("cmd.tmprt.sensors.src_installed")
            } else {
                t!("cmd.tmprt.sensors.src_not_installed")
            }
        );
        println!(
            "  {}: {}",
            t!("cmd.tmprt.sensors.elevation").cyan(),
            if is_elevated() {
                t!("cmd.tmprt.sensors.admin_yes")
            } else {
                t!("cmd.tmprt.sensors.admin_no")
            }
        );
        if installed && !pawnio {
            println!();
            print_warning(&t!("cmd.tmprt.sensors.pawnio_missing"));
        } else if installed && pawnio && !is_elevated() {
            println!();
            print_warning(&t!("cmd.tmprt.sensors.need_admin"));
        }
    }

    if !lhm_installed() {
        println!();
        println!("{}", t!("cmd.tmprt.sensors.status_hint"));
    }
}

fn print_sensors_debug() {
    println!("{}", t!("cmd.tmprt.sensors.debug_title").cyan().bold());
    println!();

    #[cfg(windows)]
    {
        println!(
            "  {}: {}",
            t!("cmd.tmprt.sensors.elevation").cyan(),
            if is_elevated() {
                t!("cmd.tmprt.sensors.admin_yes")
            } else {
                t!("cmd.tmprt.sensors.admin_no")
            }
        );
        println!();
        match lhm_debug() {
            Ok(out) => print!("{}", out),
            Err(_) => {
                print_warning(&t!("cmd.tmprt.sensors.debug_none"));
            }
        }
    }
    #[cfg(not(windows))]
    {
        println!("  {}", t!("cmd.tmprt.sensors.windows_only"));
    }
}

fn print_sensors_help() {
    println!("{}", t!("cmd.tmprt.sensors.help_title").cyan().bold());
    println!("  {}", t!("cmd.tmprt.sensors.help_status"));
    println!("  {}", t!("cmd.tmprt.sensors.help_install"));
    println!("  {}", t!("cmd.tmprt.sensors.help_uninstall"));
    println!("  {}", t!("cmd.tmprt.sensors.help_debug"));
}

// ─────────────────────────────────────────────────────────────
// tmprt weather — OpenWeather current weather
// ─────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct WeatherResp {
    #[serde(default)]
    coord: Coord,
    #[serde(default)]
    weather: Vec<Weather>,
    #[serde(default)]
    main: Main,
    #[serde(default)]
    wind: Wind,
    #[serde(default)]
    clouds: Clouds,
    #[serde(default)]
    sys: Sys,
    #[serde(default)]
    name: String,
    #[serde(default)]
    visibility: Option<i64>,
}

#[derive(Default, Deserialize)]
struct Coord {
    #[serde(default)]
    lon: f64,
    #[serde(default)]
    lat: f64,
}

#[derive(Default, Deserialize)]
struct Weather {
    #[serde(default)]
    main: String,
    #[serde(default)]
    description: String,
}

#[derive(Default, Deserialize)]
struct Main {
    #[serde(default)]
    temp: f64,
    #[serde(default)]
    feels_like: f64,
    #[serde(default)]
    temp_min: f64,
    #[serde(default)]
    temp_max: f64,
    #[serde(default)]
    pressure: f64,
    #[serde(default)]
    humidity: f64,
}

#[derive(Default, Deserialize)]
struct Wind {
    #[serde(default)]
    speed: f64,
    #[serde(default)]
    deg: i64,
}

#[derive(Default, Deserialize)]
struct Clouds {
    #[serde(default)]
    all: i64,
}

#[derive(Default, Deserialize)]
struct Sys {
    #[serde(default)]
    country: String,
    #[serde(default)]
    sunrise: i64,
    #[serde(default)]
    sunset: i64,
}

fn resolve_api_key() -> Option<String> {
    if let Ok(k) = std::env::var("OPENWEATHER_API_KEY") {
        let k = k.trim().to_string();
        if !k.is_empty() {
            return Some(k);
        }
    }
    Config::global_get_openweather_api_key().filter(|k| !k.trim().is_empty())
}

fn resolve_default_city() -> Option<String> {
    Config::global_get_openweather_default_city().filter(|c| !c.trim().is_empty())
}

fn cmd_tmprt_weather(args: &str) -> Result<bool> {
    let (city_arg, units) = parse_weather_args(args);

    let key = match resolve_api_key() {
        Some(k) => k,
        None => {
            print_error(&t!("cmd.tmprt.weather.no_key"));
            print_info(&t!("cmd.tmprt.weather.no_key_hint"));
            return Ok(false);
        }
    };

    let city = match city_arg {
        Some(c) => c,
        None => match resolve_default_city() {
            Some(c) => c,
            None => {
                print_error(&t!("cmd.tmprt.weather.no_city"));
                print_info(&t!("cmd.tmprt.weather.no_city_hint"));
                return Ok(false);
            }
        },
    };

    let agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(15)))
        .http_status_as_error(false)
        .build()
        .new_agent();

    let response = match agent
        .get(WEATHER_URL)
        .query("q", &city)
        .query("appid", &key)
        .query("units", units.as_str())
        .call()
    {
        Ok(r) => r,
        Err(e) => {
            print_error(&t!("cmd.tmprt.weather.request_failed", e));
            return Ok(false);
        }
    };

    let status = response.status();
    let body = response.into_body().read_to_string().unwrap_or_default();

    if status != 200 {
        match status.as_u16() {
            401 => print_error(&t!("cmd.tmprt.weather.bad_key")),
            404 => print_error(&t!("cmd.tmprt.weather.city_not_found", city)),
            _ => {
                let msg = serde_json::from_str::<serde_json::Value>(&body)
                    .ok()
                    .and_then(|v| {
                        v.get("message")
                            .and_then(|m| m.as_str())
                            .map(|s| s.to_string())
                    })
                    .unwrap_or_else(|| "<no detail>".to_string());
                print_error(&t!("cmd.tmprt.weather.http_error", status.as_u16(), msg));
            }
        }
        return Ok(false);
    }

    let w: WeatherResp = match serde_json::from_str(&body) {
        Ok(w) => w,
        Err(e) => {
            print_error(&t!("cmd.tmprt.weather.parse_failed", e));
            return Ok(false);
        }
    };

    print_weather(&w, &units);
    Ok(false)
}

fn parse_weather_args(args: &str) -> (Option<String>, String) {
    let mut city = None;
    let mut units = "metric".to_string();
    for token in args.split_whitespace() {
        if let Some(v) = token.strip_prefix("--units=") {
            units = if v.eq_ignore_ascii_case("imperial") {
                "imperial".to_string()
            } else {
                "metric".to_string()
            };
        } else if token == "--imperial" || token == "-i" {
            units = "imperial".to_string();
        } else if token == "--metric" || token == "-m" {
            units = "metric".to_string();
        } else if city.is_none() {
            city = Some(token.to_string());
        }
    }
    (city, units)
}

fn print_weather(w: &WeatherResp, units: &str) {
    let unit_sym = if units == "imperial" { "'oF" } else { "'oC" };
    let wind_unit = if units == "imperial" { "mph" } else { "m/s" };
    let temp = |c: f64| format!("{:.1}{unit_sym}", c);
    let desc = w
        .weather
        .first()
        .map(|wd| wd.description.as_str())
        .unwrap_or("");

    let location = format!("{}, {}", w.name, w.sys.country);
    println!();
    println!("{}", "=".repeat(46).cyan());
    println!(
        "  {}  {}",
        t!("cmd.tmprt.weather.title").cyan().bold(),
        location.bold()
    );
    println!("{}", "=".repeat(46).cyan());

    if !desc.is_empty() {
        println!(
            "  {:<12} {} {}",
            t!("cmd.tmprt.weather.condition").cyan(),
            w.weather
                .first()
                .map(|x| x.main.as_str())
                .unwrap_or("")
                .bold(),
            format!("({})", desc).dimmed()
        );
    }
    println!(
        "  {:<12} {}  ({} {})",
        t!("cmd.tmprt.weather.temp").cyan(),
        temp(w.main.temp).bold(),
        t!("cmd.tmprt.weather.feels"),
        temp(w.main.feels_like).dimmed()
    );
    println!(
        "  {:<12} {} / {}",
        t!("cmd.tmprt.weather.range").cyan(),
        temp(w.main.temp_min),
        temp(w.main.temp_max)
    );
    println!(
        "  {:<12} {}%",
        t!("cmd.tmprt.weather.humidity").cyan(),
        w.main.humidity.round()
    );
    println!(
        "  {:<12} {} hPa",
        t!("cmd.tmprt.weather.pressure").cyan(),
        w.main.pressure.round()
    );
    println!(
        "  {:<12} {} {wind_unit} ({}, {})'o",
        t!("cmd.tmprt.weather.wind").cyan(),
        w.wind.speed,
        compass_dir(w.wind.deg),
        w.wind.deg
    );
    if let Some(vis) = w.visibility {
        let vis_km = if units == "imperial" {
            (vis as f64 / 1609.344).round() as i64
        } else {
            vis / 1000
        };
        println!(
            "  {:<12} {} {}",
            t!("cmd.tmprt.weather.visibility").cyan(),
            vis_km,
            if units == "imperial" { "mi" } else { "km" }
        );
    }
    println!(
        "  {:<12} {}%",
        t!("cmd.tmprt.weather.clouds").cyan(),
        w.clouds.all
    );
    let sunrise = fmt_unix_time(w.sys.sunrise);
    let sunset = fmt_unix_time(w.sys.sunset);
    if let (Some(rise), Some(set)) = (sunrise, sunset) {
        println!(
            "  {:<12} {}   {} {}",
            t!("cmd.tmprt.weather.sunrise").cyan(),
            rise,
            t!("cmd.tmprt.weather.sunset").cyan(),
            set
        );
    }
    println!(
        "  {:<12} {:.2}'o, {:.2}'o",
        t!("cmd.tmprt.weather.coords").cyan(),
        w.coord.lat,
        w.coord.lon
    );
    println!("{}", "=".repeat(46).cyan());
    println!();
}

fn compass_dir(deg: i64) -> &'static str {
    const DIRS: [&str; 16] = [
        "N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", "S", "SSW", "SW", "WSW", "W", "WNW",
        "NW", "NNW",
    ];
    let idx = ((deg as f64 / 22.5).round() as usize) % 16;
    DIRS[idx]
}

fn fmt_unix_time(ts: i64) -> Option<String> {
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.with_timezone(&chrono::Local).format("%H:%M").to_string())
}

// ─────────────────────────────────────────────────────────────
// tmprt key / city — config helpers
// ─────────────────────────────────────────────────────────────

fn cmd_tmprt_key(args: &str) -> Result<bool> {
    let key = args.trim();
    if key.is_empty() {
        match resolve_api_key() {
            Some(k) => {
                let masked = mask_key(&k);
                if std::env::var("OPENWEATHER_API_KEY")
                    .ok()
                    .is_some_and(|v| !v.trim().is_empty())
                {
                    print_info(&t!("cmd.tmprt.key.env_source", masked));
                } else {
                    print_info(&t!("cmd.tmprt.key.status", masked));
                }
            }
            None => {
                print_warning(&t!("cmd.tmprt.key.not_set"));
                print_info(&t!("cmd.tmprt.weather.no_key_hint"));
            }
        }
        return Ok(false);
    }

    Config::global_set_openweather_api_key(Some(key.to_string()));
    print_success(&t!("cmd.tmprt.key.saved", mask_key(key)));
    Ok(false)
}

fn cmd_tmprt_city(args: &str) -> Result<bool> {
    let city = args.trim();
    if city.is_empty() {
        match resolve_default_city() {
            Some(c) => print_info(&t!("cmd.tmprt.city.status", c)),
            None => {
                print_warning(&t!("cmd.tmprt.city.not_set"));
                print_info(&t!("cmd.tmprt.weather.no_city_hint"));
            }
        }
        return Ok(false);
    }

    Config::global_set_openweather_default_city(Some(city.to_string()));
    print_success(&t!("cmd.tmprt.city.saved", city));
    Ok(false)
}

fn mask_key(key: &str) -> String {
    let chars: Vec<char> = key.chars().collect();
    if chars.len() <= 8 {
        let head: String = chars.iter().take(4).collect();
        return format!("{}****", head);
    }
    let start: String = chars.iter().take(4).collect();
    let end: String = chars[chars.len() - 4..].iter().collect();
    format!("{}****{}", start, end)
}

// ─────────────────────────────────────────────────────────────
// shared helpers
// ─────────────────────────────────────────────────────────────

fn print_usage() {
    println!("{}", t!("cmd.tmprt.usage_title").cyan().bold());
    println!("  {}", t!("cmd.tmprt.usage_com"));
    println!("  {}", t!("cmd.tmprt.usage_weather"));
    println!("  {}", t!("cmd.tmprt.usage_key"));
    println!("  {}", t!("cmd.tmprt.usage_city"));
    println!("  {}", t!("cmd.tmprt.usage_sensors"));
    println!();
    println!("{}", t!("cmd.tmprt.examples_title").green());
    println!("  {}", t!("cmd.tmprt.example1"));
    println!("  {}", t!("cmd.tmprt.example2"));
    println!("  {}", t!("cmd.tmprt.example3"));
    println!("  {}", t!("cmd.tmprt.example4"));
}

fn terminal_size() -> (usize, usize) {
    crossterm::terminal::size()
        .map(|(w, h)| (w as usize, h as usize))
        .unwrap_or((80, 24))
}

fn truncate(s: &str, max: usize) -> &str {
    if s.len() <= max {
        return s;
    }
    if max == 0 {
        return "";
    }
    let mut end = max;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

fn render_text_bar(pct: usize) -> String {
    let fill = (pct * 10 / 100).min(10);
    let empty = 10usize.saturating_sub(fill);
    format!("[{}{}]", "-".repeat(fill), " ".repeat(empty))
}

fn pct_color(
    pct: usize,
    fg_success: crossterm::style::Color,
    fg_warn: crossterm::style::Color,
    fg_err: crossterm::style::Color,
) -> crossterm::style::Color {
    if pct > 80 {
        fg_err
    } else if pct > 50 {
        fg_warn
    } else {
        fg_success
    }
}

fn temp_color(
    temp_c: f32,
    critical: Option<f32>,
    fg_success: crossterm::style::Color,
    fg_warn: crossterm::style::Color,
    fg_err: crossterm::style::Color,
) -> crossterm::style::Color {
    if let Some(crit) = critical {
        if temp_c >= crit * 0.85 {
            fg_err
        } else if temp_c >= crit * 0.6 {
            fg_warn
        } else {
            fg_success
        }
    } else if temp_c >= 75.0 {
        fg_err
    } else if temp_c >= 55.0 {
        fg_warn
    } else {
        fg_success
    }
}

fn to_unit(c: f32, celsius: bool) -> f32 {
    if celsius {
        c
    } else {
        c * 9.0 / 5.0 + 32.0
    }
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
        let _ = execute!(std::io::stdout(), Show, LeaveAlternateScreen);
    }
}
