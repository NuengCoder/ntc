//! `spec` command: print computer/system specifications.
//!
//! Shows CPU vendor / generation / model, cores & threads, GPU, RAM,
//! storage, OS, manufacturer, model, serial number, hostname, kernel and
//! uptime. Uses `sysinfo` for the parts it supports and platform tools
//! (PowerShell/WMI, sysfs, lspci, system_profiler, ...) for the rest.

use crate::navigator::Navigator;
use crate::output::print_info;
use crate::t;
use anyhow::Result;
use std::process::Command;
use sysinfo::{Disks, System};

const BOX_W: usize = 65;
const VAL_W: usize = 44;

/// Common "not a real value" strings returned by firmware/WMI when a field
/// is left empty by the OEM.
const PLACEHOLDERS: &[&str] = &[
    "default string",
    "to be filled by o.e.m.",
    "system manufacturer",
    "system product name",
    "system serial number",
    "default system serial number",
    "not specified",
    "not applicable",
    "none",
    "o.e.m.",
    "n/a",
    "unknown",
    "0",
];

pub fn cmd_spec(_args: &str, _nav: &mut Navigator) -> Result<bool> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let brand = sys
        .cpus()
        .first()
        .map(|c| c.brand().to_string())
        .unwrap_or_default();
    let (cpu_vendor, cpu_gen, cpu_model) = parse_cpu(&brand);

    let cores = sys.physical_core_count().unwrap_or(0);
    let threads = sys.cpus().len();

    let gpu_list = gpu_info();
    let gpu = if gpu_list.is_empty() {
        t!("cmd.spec.unknown")
    } else {
        gpu_list.join(", ")
    };

    let ram = crate::explorer::human_readable_size(sys.total_memory());

    let disks = Disks::new_with_refreshed_list();
    let mut disk_rows: Vec<(String, String)> = Vec::new();
    let mut total_storage: u64 = 0;
    for d in &disks {
        let size = d.total_space();
        total_storage = total_storage.saturating_add(size);
        let mount = d.mount_point().to_string_lossy().to_string();
        let label = if mount.is_empty() {
            d.name().to_string_lossy().to_string()
        } else {
            mount
        };
        let kind = disk_kind(d);
        let kind_part = if kind.is_empty() {
            String::new()
        } else {
            format!("{} · ", kind)
        };
        disk_rows.push((
            format!("  {}", label),
            format!("{}{}", kind_part, crate::explorer::human_readable_size(size)),
        ));
    }

    let os = format_os();
    let manufacturer = manufacturer_name();
    let computer_model = computer_model_name();
    let serial = serial_number().unwrap_or_else(|| t!("cmd.spec.unknown").to_string());
    let hostname = System::host_name()
        .unwrap_or_else(|| t!("cmd.spec.unknown").to_string());
    let kernel = System::kernel_version()
        .unwrap_or_else(|| t!("cmd.spec.unknown").to_string());
    let uptime = format_uptime(System::uptime());

    // ── render ─────────────────────────────────────────────────────────
    let b = |s: String| crate::utils::terminal::asciify(&s);
    let r = |label: &str, value: &str| {
        format!(
            "│ {:<18} {:<width$} │",
            label,
            truncate(value, VAL_W),
            width = VAL_W
        )
    };

    print_info(&t!("cmd.spec.title"));
    println!("{}", b(format!("┌{}┐", "─".repeat(BOX_W))));
    println!("{}", b(format!("│{:^width$}│", t!("cmd.spec.title"), width = BOX_W)));
    println!("{}", b(format!("├{}┤", "─".repeat(BOX_W))));
    println!("{}", b(r(&t!("cmd.spec.cpu"), &brand)));
    println!("{}", b(r(&t!("cmd.spec.vendor"), &cpu_vendor)));
    println!("{}", b(r(&t!("cmd.spec.generation"), &cpu_gen)));
    println!("{}", b(r(&t!("cmd.spec.model"), &cpu_model)));
    println!("{}", b(r(&t!("cmd.spec.cores"), &format!("{} / {}", cores, threads))));
    println!("{}", b(r(&t!("cmd.spec.gpu"), &gpu)));
    println!("{}", b(r(&t!("cmd.spec.ram"), &ram)));
    println!(
        "{}",
        b(r(
            &t!("cmd.spec.storage"),
            &format!(
                "{} ({})",
                crate::explorer::human_readable_size(total_storage),
                t!("cmd.spec.total")
            )
        ))
    );
    for (label, val) in &disk_rows {
        println!("{}", b(r(label, val)));
    }
    println!("{}", b(r(&t!("cmd.spec.os"), &os)));
    println!("{}", b(r(&t!("cmd.spec.manufacturer"), &manufacturer)));
    println!("{}", b(r(&t!("cmd.spec.computer_model"), &computer_model)));
    println!("{}", b(r(&t!("cmd.spec.serial"), &serial)));
    println!("{}", b(r(&t!("cmd.spec.hostname"), &hostname)));
    println!("{}", b(r(&t!("cmd.spec.kernel"), &kernel)));
    println!("{}", b(r(&t!("cmd.spec.uptime"), &uptime)));
    println!("{}", b(format!("└{}┘", "─".repeat(BOX_W))));

    Ok(false)
}

// ── CPU parsing ────────────────────────────────────────────────────────

/// Best-effort parse of a CPU brand string into (vendor, generation, model id).
fn parse_cpu(brand: &str) -> (String, String, String) {
    let upper = brand.to_uppercase();
    if upper.contains("AMD") {
        let (gen, model) = parse_amd(brand);
        return ("AMD".to_string(), gen, model);
    }
    if upper.contains("INTEL") {
        let (gen, model) = parse_intel(brand);
        return ("Intel".to_string(), gen, model);
    }
    (
        t!("cmd.spec.other").to_string(),
        t!("cmd.spec.unknown").to_string(),
        t!("cmd.spec.unknown").to_string(),
    )
}

fn parse_intel(brand: &str) -> (String, String) {
    // Core Ultra: "Intel(R) Core(TM) Ultra 7 155H" → 1st Gen
    let upper = brand.to_uppercase();
    if let Some(idx) = upper.find("ULTRA") {
        let rest = &brand[idx + "ULTRA".len()..];
        let toks: Vec<&str> = rest.split_whitespace().collect();
        if toks.len() >= 2 {
            if let Ok(sku) = toks[1].parse::<u32>() {
                let gen = (sku / 100) as usize;
                if (1..=3).contains(&gen) {
                    let model = format!("Ultra {} {}", toks[0], toks[1]);
                    return (format!("{} Gen", ordinal(gen)), model);
                }
            }
        }
    }

    // Classic: "Intel(R) Core(TM) i7-10700K CPU @ 3.80GHz"
    let chars: Vec<(usize, char)> = upper.char_indices().collect();
    for (i, (_, c)) in chars.iter().enumerate() {
        if (*c == 'I' || *c == 'C')
            && i + 1 < chars.len()
            && chars[i + 1].1.is_ascii_digit()
        {
            let mut j = i + 1;
            while j < chars.len() && chars[j].1.is_ascii_digit() {
                j += 1;
            }
            if j < chars.len() && chars[j].1 == '-' {
                let prefix_end = chars[j].0;
                let num_start = chars[j].0 + 1;
                let prefix = &brand[..prefix_end];
                let rest = &brand[num_start..];
                let model_num: String = rest
                    .chars()
                    .take_while(|ch| ch.is_ascii_alphanumeric())
                    .collect();
                if !model_num.is_empty() {
                    let core_name = prefix
                        .rsplit(' ')
                        .next()
                        .unwrap_or("")
                        .to_string();
                    let model = format!("{}-{}", core_name, model_num);
                    let digits: String = model_num
                        .chars()
                        .take_while(|ch| ch.is_ascii_digit())
                        .collect();
                    let gen = if digits.len() >= 5 && digits.starts_with('1') {
                        digits[..2].parse::<u32>().unwrap_or(0)
                    } else {
                        digits
                            .chars()
                            .next()
                            .and_then(|ch| ch.to_digit(10))
                            .unwrap_or(0)
                    };
                    if (2..=14).contains(&gen) {
                        return (format!("{} Gen", ordinal(gen as usize)), model);
                    }
                    return (
                        t!("cmd.spec.unknown").to_string(),
                        t!("cmd.spec.unknown").to_string(),
                    );
                }
            }
        }
    }
    (
        t!("cmd.spec.unknown").to_string(),
        t!("cmd.spec.unknown").to_string(),
    )
}

fn parse_amd(brand: &str) -> (String, String) {
    let upper = brand.to_uppercase();
    let Some(idx) = upper.find("RYZEN") else {
        return (
            t!("cmd.spec.unknown").to_string(),
            t!("cmd.spec.unknown").to_string(),
        );
    };
    let rest = &brand[idx + "RYZEN".len()..];
    let toks: Vec<&str> = rest.split_whitespace().collect();
    if toks.is_empty() {
        return (
            t!("cmd.spec.unknown").to_string(),
            t!("cmd.spec.unknown").to_string(),
        );
    }

    // "Ryzen AI 9 HX 370" → series 300, model 370
    if toks[0].eq_ignore_ascii_case("ai") {
        if let Some(last) = toks.last() {
            if let Ok(n) = last.parse::<u32>() {
                let series = (n / 100) * 100;
                return (format!("Ryzen {} series", series), last.to_string());
            }
        }
        return (
            t!("cmd.spec.unknown").to_string(),
            t!("cmd.spec.unknown").to_string(),
        );
    }

    // "Ryzen Threadripper 1950X"
    if toks[0].eq_ignore_ascii_case("threadripper") && toks.len() >= 2 {
        if let Ok(n) = toks[1].parse::<u32>() {
            let series = (n / 100) * 100;
            return (
                format!("Ryzen Threadripper {} series", series),
                toks[1].to_string(),
            );
        }
        return (
            t!("cmd.spec.unknown").to_string(),
            t!("cmd.spec.unknown").to_string(),
        );
    }

    // "Ryzen 7 5800X 8-Core Processor" → series 5000, model 5800X
    if toks[0].parse::<u32>().is_ok() && toks.len() >= 2 {
        let model = toks[1].to_string();
        let digits: String = model
            .chars()
            .take_while(|ch| ch.is_ascii_digit())
            .collect();
        if digits.len() >= 4 {
            let series = digits[..1].parse::<u32>().unwrap_or(0) * 1000;
            return (format!("Ryzen {} series", series), model);
        }
        return (t!("cmd.spec.unknown").to_string(), model);
    }

    (
        t!("cmd.spec.unknown").to_string(),
        t!("cmd.spec.unknown").to_string(),
    )
}

fn ordinal(n: usize) -> String {
    let suffix = match n % 100 {
        11..=13 => "th",
        _ => match n % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    };
    format!("{}{}", n, suffix)
}

// ── OS ─────────────────────────────────────────────────────────────────

fn os_family() -> &'static str {
    #[cfg(windows)]
    {
        "Windows"
    }
    #[cfg(target_os = "macos")]
    {
        "macOS"
    }
    #[cfg(target_os = "linux")]
    {
        "Linux"
    }
    #[cfg(target_os = "android")]
    {
        "Android"
    }
    #[cfg(not(any(
        windows,
        target_os = "macos",
        target_os = "linux",
        target_os = "android"
    )))]
    {
        "Other"
    }
}

fn format_os() -> String {
    let family = os_family();
    let detail = System::long_os_version()
        .or_else(System::os_version)
        .unwrap_or_default()
        .to_string();
    if detail.is_empty() {
        family.to_string()
    } else if detail.to_lowercase().contains(&family.to_lowercase()) {
        detail
    } else {
        format!("{} {}", family, detail)
    }
}

fn format_uptime(secs: u64) -> String {
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let mins = (secs % 3600) / 60;
    if days > 0 {
        format!("{}d {}h {}m", days, hours, mins)
    } else if hours > 0 {
        format!("{}h {}m", hours, mins)
    } else {
        format!("{}m", mins)
    }
}

// ── platform helpers ───────────────────────────────────────────────────

fn run_capture(program: &str, args: &[&str]) -> Option<String> {
    let out = Command::new(program).args(args).output().ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

fn is_placeholder(s: &str) -> bool {
    let lower = s.trim().to_lowercase();
    lower.is_empty() || PLACEHOLDERS.iter().any(|p| lower == *p)
}

fn disk_kind(d: &sysinfo::Disk) -> String {
    use sysinfo::DiskKind;
    match d.kind() {
        DiskKind::HDD => "HDD".to_string(),
        DiskKind::SSD => "SSD".to_string(),
        DiskKind::Unknown(_) => String::new(),
    }
}

fn manufacturer_name() -> String {
    #[cfg(windows)]
    {
        if let Some(m) = run_capture(
            "powershell",
            &["-NoProfile", "-Command", "(Get-CimInstance Win32_ComputerSystem).Manufacturer"],
        ) {
            if !is_placeholder(&m) {
                return m;
            }
        }
        t!("cmd.spec.custom").to_string()
    }
    #[cfg(target_os = "linux")]
    {
        if let Ok(s) = std::fs::read_to_string("/sys/class/dmi/id/sys_vendor") {
            let s = s.trim().to_string();
            if !is_placeholder(&s) {
                return s;
            }
        }
        if let Some(s) = run_capture("dmidecode", &["-s", "system-manufacturer"]) {
            if !is_placeholder(&s) {
                return s;
            }
        }
        t!("cmd.spec.custom").to_string()
    }
    #[cfg(target_os = "macos")]
    {
        "Apple Inc.".to_string()
    }
    #[cfg(target_os = "android")]
    {
        if let Some(s) = run_capture("getprop", &["ro.product.manufacturer"]) {
            if !is_placeholder(&s) {
                return s;
            }
        }
        t!("cmd.spec.custom").to_string()
    }
}

fn computer_model_name() -> String {
    #[cfg(windows)]
    {
        if let Some(m) = run_capture(
            "powershell",
            &["-NoProfile", "-Command", "(Get-CimInstance Win32_ComputerSystem).Model"],
        ) {
            if !is_placeholder(&m) {
                return m;
            }
        }
        t!("cmd.spec.custom").to_string()
    }
    #[cfg(target_os = "linux")]
    {
        if let Ok(s) = std::fs::read_to_string("/sys/class/dmi/id/product_name") {
            let s = s.trim().to_string();
            if !is_placeholder(&s) {
                return s;
            }
        }
        t!("cmd.spec.custom").to_string()
    }
    #[cfg(target_os = "macos")]
    {
        if let Some(out) = run_capture("system_profiler", &["SPHardwareDataType"]) {
            for line in out.lines() {
                let tr = line.trim();
                if let Some(idx) = tr.find(':') {
                    let key = tr[..idx].trim();
                    if key.eq_ignore_ascii_case("Model Name")
                        || key.eq_ignore_ascii_case("Model Identifier")
                    {
                        let val = tr[idx + 1..].trim().to_string();
                        if !is_placeholder(&val) {
                            return val;
                        }
                    }
                }
            }
        }
        t!("cmd.spec.custom").to_string()
    }
    #[cfg(target_os = "android")]
    {
        if let Some(s) = run_capture("getprop", &["ro.product.model"]) {
            if !is_placeholder(&s) {
                return s;
            }
        }
        t!("cmd.spec.custom").to_string()
    }
}

fn serial_number() -> Option<String> {
    #[cfg(windows)]
    {
        let s = run_capture(
            "powershell",
            &["-NoProfile", "-Command", "(Get-CimInstance Win32_BIOS).SerialNumber"],
        )?;
        if is_placeholder(&s) {
            None
        } else {
            Some(s)
        }
    }
    #[cfg(target_os = "linux")]
    {
        for p in ["/sys/class/dmi/id/product_serial", "/sys/class/dmi/id/board_serial"] {
            if let Ok(s) = std::fs::read_to_string(p) {
                let s = s.trim().to_string();
                if !is_placeholder(&s) {
                    return Some(s);
                }
            }
        }
        None
    }
    #[cfg(target_os = "macos")]
    {
        if let Some(out) = run_capture("system_profiler", &["SPHardwareDataType"]) {
            for line in out.lines() {
                let tr = line.trim();
                if let Some(idx) = tr.find(':') {
                    let key = tr[..idx].trim();
                    if key.eq_ignore_ascii_case("Serial Number (system)")
                        || key.eq_ignore_ascii_case("Serial number (system)")
                        || key.eq_ignore_ascii_case("Serial Number")
                    {
                        let val = tr[idx + 1..].trim().to_string();
                        if !is_placeholder(&val) {
                            return Some(val);
                        }
                    }
                }
            }
        }
        None
    }
    #[cfg(target_os = "android")]
    {
        for p in ["ro.boot.serialno", "ro.serialno"] {
            if let Some(s) = run_capture("getprop", &[p]) {
                if !is_placeholder(&s) {
                    return Some(s);
                }
            }
        }
        None
    }
}

fn gpu_info() -> Vec<String> {
    #[cfg(windows)]
    {
        let mut gpus: Vec<String> = Vec::new();
        if let Some(out) = run_capture(
            "powershell",
            &["-NoProfile", "-Command", "(Get-CimInstance Win32_VideoController).Name"],
        ) {
            for line in out.lines() {
                let l = line.trim().to_string();
                if !l.is_empty() && !gpus.iter().any(|g| g == &l) {
                    gpus.push(l);
                }
            }
        }
        if gpus.is_empty() {
            if let Some(out) = run_capture(
                "nvidia-smi",
                &["--query-gpu=name", "--format=csv,noheader"],
            ) {
                gpus.extend(
                    out.lines()
                        .map(|l| l.trim().to_string())
                        .filter(|l| !l.is_empty()),
                );
            }
        }
        gpus
    }
    #[cfg(target_os = "linux")]
    {
        let mut gpus: Vec<String> = Vec::new();
        if let Some(out) = run_capture("lspci", &["-nn"]) {
            for line in out.lines() {
                let lower = line.to_lowercase();
                if lower.contains("vga compatible")
                    || lower.contains("3d controller")
                    || lower.contains("display controller")
                {
                    let mut s = line.trim().to_string();
                    if let Some(pos) = s.find(": ") {
                        s = s[pos + 2..].to_string();
                    }
                    if let Some(rev) = s.find(" (rev") {
                        s.truncate(rev);
                    }
                    s = s.trim().to_string();
                    if !s.is_empty() && !gpus.iter().any(|g| g == &s) {
                        gpus.push(s);
                    }
                }
            }
        }
        if gpus.is_empty() {
            if let Some(out) = run_capture(
                "nvidia-smi",
                &["--query-gpu=name", "--format=csv,noheader"],
            ) {
                gpus.extend(
                    out.lines()
                        .map(|l| l.trim().to_string())
                        .filter(|l| !l.is_empty()),
                );
            }
        }
        gpus
    }
    #[cfg(target_os = "macos")]
    {
        let mut gpus: Vec<String> = Vec::new();
        if let Some(out) = run_capture("system_profiler", &["SPDisplaysDataType"]) {
            for line in out.lines() {
                let tr = line.trim();
                if let Some(idx) = tr.find(':') {
                    let key = tr[..idx].trim();
                    if key.eq_ignore_ascii_case("Chipset Model") {
                        let val = tr[idx + 1..].trim().to_string();
                        if !val.is_empty() && !gpus.iter().any(|g| g == &val) {
                            gpus.push(val);
                        }
                    }
                }
            }
        }
        gpus
    }
    #[cfg(target_os = "android")]
    {
        let mut gpus: Vec<String> = Vec::new();
        if let Some(out) = run_capture(
            "nvidia-smi",
            &["--query-gpu=name", "--format=csv,noheader"],
        ) {
            gpus.extend(
                out.lines()
                    .map(|l| l.trim().to_string())
                    .filter(|l| !l.is_empty()),
            );
        }
        gpus
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let mut out = String::new();
    for c in s.chars() {
        if out.chars().count() >= max {
            break;
        }
        out.push(c);
    }
    out
}
