use crate::filetype::strip_unc_prefix;
use crate::shell::venv::{detect_active_venv, has_venv_in_dir};
use crate::t;
use anyhow::{Context, Result};
use std::env;
use std::path::{Path, PathBuf};

/// Manages the current working directory state
pub struct Navigator {
    current_dir: PathBuf,
    venv_name: Option<String>,
}

impl Navigator {
    /// Create a new Navigator starting from the current directory
    pub fn new() -> Result<Self> {
        let current_dir = env::current_dir().context("Failed to get current directory")?;
        Ok(Self {
            current_dir,
            venv_name: detect_active_venv(),
        })
    }

    pub fn update_venv_name(&mut self, venv_name: Option<String>) {
        self.venv_name = venv_name;
    }

    /// Test-only constructor pointing at `dir` without touching the
    /// process-global CWD (unlike `go_to`, which chdirs). Prevents parallel
    /// tests from observing each other's working directory.
    #[cfg(test)]
    pub(crate) fn for_test(dir: &Path) -> Self {
        Self {
            current_dir: dir.to_path_buf(),
            venv_name: detect_active_venv(),
        }
    }

    /// Get the current directory path (internal, may contain extended prefix)
    pub fn current_path(&self) -> &Path {
        &self.current_dir
    }

    /// Get display‑ready path string (without `\\?\` prefix on Windows)
    pub fn display_path(&self) -> String {
        strip_extended_prefix(&self.current_dir)
    }

    /// Get just the current directory name for compact prompt display
    pub fn current_dir_name(&self) -> String {
        self.current_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("~")
            .to_string()
    }

    /// Get the venv prefix for display
    pub fn get_venv_prefix(&self) -> String {
        match &self.venv_name {
            Some(name) => format!("({}) ", name),
            None => String::new(),
        }
    }

    /// Get colored venv prefix using theme
    pub fn get_colored_venv_prefix(&self) -> String {
        match &self.venv_name {
            Some(name) => {
                format!("({}) ", name)
            }
            None => String::new(),
        }
    }

    /// Navigate to a specific directory
    pub fn go_to(&mut self, path: &Path) -> Result<()> {
        // Try to canonicalize, but if it fails with extended path, try without
        let canonical = match path.canonicalize() {
            Ok(p) => p,
            Err(_) => {
                // If canonicalize fails, try to clean the path first
                let path_str = path.to_string_lossy();
                let clean_path =
                    Path::new(path_str.strip_prefix(r"\\?\").unwrap_or(&path_str)).to_path_buf();
                clean_path
                    .canonicalize()
                    .with_context(|| t!("navigator.cannot_navigate_to", path.display()))?
            }
        };

        if canonical.is_dir() {
            let has_local_config = canonical.join("ntconfig.toml").exists();
            env::set_current_dir(&canonical)?;
            self.current_dir = canonical;

            // Re-detect venv after directory change
            self.venv_name = detect_active_venv();

            // Optional: Show hint if venv exists but not activated
            if self.venv_name.is_none() && has_venv_in_dir(&self.current_dir) {
                #[cfg(not(target_os = "android"))]
                {
                    use colored::Colorize;
                    println!();
                    println!("{}", t!("navigator.venv_tip").cyan());
                    #[cfg(windows)]
                    println!("{}", t!("navigator.venv_activate_win"));
                    #[cfg(not(windows))]
                    println!("{}", t!("navigator.venv_activate_unix"));
                }
            }

            if has_local_config {
                crate::config::Config::reload_global();
            }

            Ok(())
        } else {
            anyhow::bail!(t!("navigator.not_a_directory", path.display()))
        }
    }

    /// Navigate to a drive root on Windows (e.g., 'C' -> "C:\").
    /// On non-Windows platforms this is a no-op that returns an error.
    pub fn go_drive(&mut self, drive_letter: char) -> Result<()> {
        #[cfg(windows)]
        {
            let drive_path = format!("{}:\\", drive_letter.to_ascii_uppercase());
            self.go_to(Path::new(&drive_path))
        }
        #[cfg(not(windows))]
        {
            let _ = drive_letter;
            anyhow::bail!(t!("navigator.drive_windows_only"))
        }
    }

    /// List all available drives.
    /// On Windows, scans A–Z for existing drive roots.
    /// On other platforms, returns an empty Vec.
    pub fn list_drives() -> Vec<char> {
        #[cfg(windows)]
        {
            let mut drives = Vec::new();
            for letter in b'A'..=b'Z' {
                let path = format!("{}:\\", letter as char);
                if Path::new(&path).exists() {
                    drives.push(letter as char);
                }
            }
            drives
        }
        #[cfg(not(windows))]
        {
            Vec::new()
        }
    }

    /// Go back to the parent directory
    pub fn go_back(&mut self) -> Result<()> {
        let parent = self
            .current_dir
            .parent()
            .map(|p| p.to_path_buf())
            .ok_or_else(|| anyhow::anyhow!(t!("navigator.already_root")))?;
        self.go_to(&parent)
    }

    /// List subdirectories in the current directory (sorted alphabetically),
    /// respecting the same ignored-directory config as tree/reports.
    /// Returns vector of (index, name) tuples.
    pub fn list_subdirs(&self) -> Result<Vec<(usize, String)>> {
        let mut dirs = Vec::new();
        match std::fs::read_dir(&self.current_dir) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            dirs.push(name.to_string());
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "{}",
                    t!("navigator.cannot_read_dir", self.current_dir.display(), e)
                );
            }
        }
        dirs.sort_by_cached_key(|a| a.to_lowercase());
        Ok(dirs
            .into_iter()
            .enumerate()
            .map(|(i, n)| (i + 1, n))
            .collect())
    }
}

/// Clear the terminal screen using ANSI escape codes.
pub fn clear_screen() {
    print!("\x1B[2J\x1B[3J\x1B[1;1H");
    let _ = std::io::Write::flush(&mut std::io::stdout());
}

/// Remove Windows extended‑length prefix (\\?\) for display.
fn strip_extended_prefix(path: &Path) -> String {
    strip_unc_prefix(path)
}
