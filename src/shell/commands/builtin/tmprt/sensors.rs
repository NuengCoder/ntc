//! Sensor backends: sysinfo ACPI, NVIDIA, disk SMART and LibreHardwareMonitor
//! (LHM) plus the PawnIO driver plumbing, and the `tmprt sensors` subcommand.

#[cfg(windows)]
use crate::output::print_warning;
use crate::output::{print_error, print_info, print_success};
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;
use std::path::PathBuf;
use std::time::Duration;
use sysinfo::Components;

#[cfg(windows)]
use anyhow::anyhow;
#[cfg(windows)]
use indicatif::{ProgressBar, ProgressStyle};
#[cfg(windows)]
use std::io::Read;
#[cfg(windows)]
use std::path::Path;

#[cfg(windows)]
const LHM_URL: &str = "https://github.com/LibreHardwareMonitor/LibreHardwareMonitor/releases/download/v0.9.6/LibreHardwareMonitor.zip";
pub(super) const LHM_REFRESH_EVERY: Duration = Duration::from_secs(5);
/// PawnIO: the signed, HVCI-compatible kernel driver LHM (>= 0.9.5) uses
/// instead of WinRing0. Installed once (elevated) by `tmprt sensors install`.
#[cfg(windows)]
const PAWNIO_URL: &str =
    "https://github.com/namazso/PawnIO.Setup/releases/download/2.2.0/PawnIO_setup.exe";

/// Whether the current process has administrator privileges (Windows).
///
/// The LibreHardwareMonitorLib WinRing0 kernel driver only loads elevated, so
/// CPU temperatures are unreadable from a standard (non-admin) process.
#[cfg(windows)]
pub(super) fn is_elevated() -> bool {
    std::process::Command::new("whoami")
        .args(["/groups"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).contains("S-1-16-12288"))
        .unwrap_or(false)
}

#[cfg(not(windows))]
pub(super) fn is_elevated() -> bool {
    false
}

#[cfg(not(windows))]
pub(super) fn pawnio_installed() -> bool {
    false
}

/// Whether the LibreHardwareMonitorLib DLLs are installed.
pub(super) fn lhm_installed() -> bool {
    lhm_dir().join("LibreHardwareMonitorLib.dll").exists()
}

pub(super) fn sane_temp(t: f32) -> bool {
    !t.is_nan() && !t.is_infinite() && (-40.0..150.0).contains(&t)
}

/// Storage temperatures via PowerShell (Windows only).
#[cfg(windows)]
pub(super) fn disk_temps() -> Vec<super::com::SensorReading> {
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
                        out.push(super::com::SensorReading::live(
                            format!("DISK {name}"),
                            t,
                            None,
                            None,
                        ));
                    }
                }
            }
        }
    }
    out
}

/// CPU/GPU/board/storage temperatures via LibreHardwareMonitorLib (Windows only).
#[cfg(windows)]
pub(super) fn lhm_temps() -> Vec<super::com::SensorReading> {
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
                        out.push(super::com::SensorReading::unavailable(label));
                    } else {
                        out.push(super::com::SensorReading::live(label, t, None, None));
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
pub(super) fn lhm_debug() -> Result<String> {
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
pub(super) fn lhm_dir() -> PathBuf {
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
pub(super) fn pawnio_installed() -> bool {
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
pub(super) fn install_lhm() -> Result<()> {
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

pub(super) fn cmd_tmprt_sensors(args: &str) -> Result<bool> {
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
