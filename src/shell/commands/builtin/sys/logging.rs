//! Append-only snapshot logging for `sys --log`.
//!
//! Snapshots are written on the requested interval to `log/ntc_sys.log` under
//! the shell's current directory. The file is opened in append mode, so
//! earlier runs are preserved.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::time::{Duration, Instant};
use sysinfo::{Disks, System};

use super::render::format_size;

pub(super) const LOG_DIR: &str = "log";
pub(super) const LOG_FILE: &str = "ntc_sys.log";
/// `sys -l` writes a snapshot at the `-d`/`--delay` interval, 15 seconds
/// unless one is given.
pub(super) const DEFAULT_INTERVAL_SECS: u64 = 15;
/// Value `-d`/`--delay` falls back to when it is given without a number:
/// "use [`DEFAULT_INTERVAL_SECS`]".
pub(super) const DEFAULT_DELAY: i64 = -1;

pub(super) struct SysLogger {
    file: File,
    first_record: bool,
    interval: Duration,
    last_write: Instant,
}

impl SysLogger {
    /// Open (or create) `<cwd>/log/ntc_sys.log` without truncating it.
    pub(super) fn open(cwd: &Path, interval: Duration) -> io::Result<Self> {
        let dir = cwd.join(LOG_DIR);
        fs::create_dir_all(&dir)?;
        let path = dir.join(LOG_FILE);
        let file = OpenOptions::new().create(true).append(true).open(&path)?;
        let first_record = file.metadata().map(|m| m.len() == 0).unwrap_or(true);
        Ok(Self {
            file,
            first_record,
            interval,
            last_write: Instant::now(),
        })
    }

    pub(super) fn interval_secs(&self) -> u64 {
        self.interval.as_secs()
    }

    /// Append one snapshot. A blank line separates records when the file
    /// already holds data.
    pub(super) fn append(&mut self, sys: &System, disks: &Disks) -> io::Result<()> {
        self.append_at(sys, disks, Instant::now())
    }

    pub(super) fn is_due(&self, now: Instant) -> bool {
        now.saturating_duration_since(self.last_write) >= self.interval
    }

    /// Append only when at least one logging interval has passed.
    pub(super) fn append_if_due(
        &mut self,
        sys: &System,
        disks: &Disks,
        now: Instant,
    ) -> io::Result<bool> {
        if !self.is_due(now) {
            return Ok(false);
        }
        self.append_at(sys, disks, now)?;
        Ok(true)
    }

    fn append_at(&mut self, sys: &System, disks: &Disks, now: Instant) -> io::Result<()> {
        let mut out = String::new();
        if !self.first_record {
            out.push('\n');
        }
        out.push_str(&snapshot(sys, disks));
        self.file.write_all(out.as_bytes())?;
        self.file.flush()?;
        self.first_record = false;
        self.last_write = now;
        Ok(())
    }
}

fn snapshot(sys: &System, disks: &Disks) -> String {
    let mut out = String::new();
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    out.push_str(&format!("[{now}]\n"));

    out.push_str(&format!("CPU: {:.1}%\n", sys.global_cpu_usage()));
    for (i, cpu) in sys.cpus().iter().enumerate() {
        out.push_str(&format!("  core {}: {:.1}%\n", i + 1, cpu.cpu_usage()));
    }

    let mem_total = sys.total_memory();
    let mem_used = sys.used_memory();
    out.push_str(&format!(
        "MEM: {} / {} ({:.1}%)\n",
        format_size(mem_used),
        format_size(mem_total),
        percent(mem_used, mem_total)
    ));

    let swap_total = sys.total_swap();
    let swap_used = sys.used_swap();
    out.push_str(&format!(
        "SWAP: {} / {} ({:.1}%)\n",
        format_size(swap_used),
        format_size(swap_total),
        percent(swap_used, swap_total)
    ));

    for disk in disks {
        let total = disk.total_space();
        let avail = disk.available_space();
        let used = total.saturating_sub(avail);
        out.push_str(&disk_line(
            &disk.mount_point().to_string_lossy(),
            &disk.name().to_string_lossy(),
            used,
            total,
            avail,
        ));
    }
    out
}

fn disk_line(mount: &str, label: &str, used: u64, total: u64, avail: u64) -> String {
    let label = label.trim();
    let label = if label.is_empty() {
        String::new()
    } else {
        format!(" [{label}]")
    };
    format!(
        "DISK {mount}{label}: {} / {} ({:.1}%), {} free\n",
        format_size(used),
        format_size(total),
        percent(used, total),
        format_size(avail)
    )
}

fn percent(used: u64, total: u64) -> f64 {
    if total == 0 {
        0.0
    } else {
        used as f64 * 100.0 / total as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appends_without_truncating_existing_records() {
        let dir = tempfile::tempdir().unwrap();
        let sys = System::new();
        let disks = Disks::new();

        let mut logger = SysLogger::open(dir.path(), Duration::from_secs(15)).unwrap();
        logger.append(&sys, &disks).unwrap();
        let path = dir.path().join(LOG_DIR).join(LOG_FILE);
        let first = fs::read_to_string(&path).unwrap();
        assert!(first.starts_with('['), "got: {first}");
        assert!(first.contains("CPU:"), "got: {first}");
        assert!(first.contains("MEM:"), "got: {first}");
        assert!(first.contains("SWAP:"), "got: {first}");

        // A second session opens the same file and appends a new block.
        let mut logger = SysLogger::open(dir.path(), Duration::from_secs(15)).unwrap();
        logger.append(&sys, &disks).unwrap();
        let second = fs::read_to_string(&path).unwrap();
        assert!(
            second.starts_with(&first),
            "old records must be preserved:\n{second}"
        );
        assert_eq!(second.matches("MEM:").count(), 2);
        assert!(second.contains("\n\n["), "records need a blank separator");
    }

    #[test]
    fn appends_only_after_the_interval_elapses() {
        let dir = tempfile::tempdir().unwrap();
        let sys = System::new();
        let disks = Disks::new();
        let start = Instant::now();
        let mut logger = SysLogger::open(dir.path(), Duration::from_secs(5)).unwrap();
        logger.append_at(&sys, &disks, start).unwrap();

        assert!(!logger
            .append_if_due(&sys, &disks, start + Duration::from_secs(4))
            .unwrap());
        assert!(logger
            .append_if_due(&sys, &disks, start + Duration::from_secs(5))
            .unwrap());
        assert!(!logger
            .append_if_due(&sys, &disks, start + Duration::from_secs(6))
            .unwrap());

        let path = dir.path().join(LOG_DIR).join(LOG_FILE);
        let log = fs::read_to_string(path).unwrap();
        assert_eq!(log.matches("MEM:").count(), 2);
    }

    #[test]
    fn disk_line_reports_usage_and_free_space() {
        let gb = 1024 * 1024 * 1024;
        let line = disk_line("C:\\", "Windows", 120 * gb, 500 * gb, 380 * gb);
        assert!(line.starts_with("DISK C:\\ [Windows]: "), "got: {line}");
        assert!(line.contains("(24.0%)"), "got: {line}");
        assert!(line.contains("380G free"), "got: {line}");
        assert!(line.ends_with('\n'));
    }

    #[test]
    fn zero_total_does_not_divide_by_zero() {
        assert_eq!(percent(5, 0), 0.0);
        assert_eq!(
            disk_line("/", "", 0, 0, 0),
            "DISK /: 0.0B / 0.0B (0.0%), 0.0B free\n"
        );
    }
}
