use anyhow::{Context, Result};
use crate::config::Config;
use std::env;
use std::path::{Path, PathBuf};

/// Manages the current working directory state
pub struct Navigator {
    current_dir: PathBuf,
}

impl Navigator {
    /// Create a new Navigator starting from the current directory
    pub fn new() -> Result<Self> {
        let current_dir = env::current_dir().context("Failed to get current directory")?;
        Ok(Self { current_dir })
    }

    /// Get the current directory path (internal, may contain extended prefix)
    pub fn current_path(&self) -> &Path {
        &self.current_dir
    }

    /// Get display‑ready path string (without `\\?\` prefix on Windows)
    pub fn display_path(&self) -> String {
        strip_extended_prefix(&self.current_dir)
    }

    /// Navigate to a specific directory
    pub fn go_to(&mut self, path: &Path) -> Result<()> {
        // Try to canonicalize, but if it fails with extended path, try without
        let canonical = match path.canonicalize() {
            Ok(p) => p,
            Err(_) => {
                // If canonicalize fails, try to clean the path first
                let path_str = path.to_string_lossy();
                let clean_path = if path_str.starts_with(r"\\?\") {
                    Path::new(&path_str[4..]).to_path_buf()
                } else {
                    path.to_path_buf()
                };
                clean_path.canonicalize()
                    .with_context(|| format!("Cannot navigate to: {}", path.display()))?
            }
        };
        
        if canonical.is_dir() {
            env::set_current_dir(&canonical)?;
            self.current_dir = canonical;
            
            // Reload config to pick up any ntconfig.toml in the new directory
            crate::config::Config::reload_global();
            
            Ok(())
        } else {
            anyhow::bail!("{} is not a directory", path.display())
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
            anyhow::bail!("Drive navigation is only supported on Windows")
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
        let parent = self.current_dir
            .parent()
            .map(|p| p.to_path_buf())
            .ok_or_else(|| anyhow::anyhow!("Already at root, cannot go back further"))?;
        self.go_to(&parent)
    }

    /// List subdirectories in the current directory (sorted alphabetically),
    /// respecting the same ignored-directory config as tree/reports.
    /// Returns vector of (index, name) tuples.
    pub fn list_subdirs(&self) -> Result<Vec<(usize, String)>> {
        let ignored = Config::global_get_ignored_dirs();
        let mut dirs = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&self.current_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if !ignored.contains(&name.to_lowercase()) {
                            dirs.push(name.to_string());
                        }
                    }
                }
            }
        }
        dirs.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        Ok(dirs.into_iter().enumerate().map(|(i, n)| (i + 1, n)).collect())
    }
}

/// Clear the terminal screen in a cross-platform way.
pub fn clear_screen() {
    #[cfg(windows)]
    {
        let _ = std::process::Command::new("cmd").args(["/c", "cls"]).status();
    }
    #[cfg(not(windows))]
    {
        // Try system clear command first (works better in WSL)
        let status = std::process::Command::new("clear").status();
        if status.is_err() {
            // Fallback to ANSI if clear command not available
            print!("\x1B[2J\x1B[1;1H");
            let _ = std::io::Write::flush(&mut std::io::stdout());
        }
    }
}

/// Remove Windows extended‑length prefix (\\?\) for display.
/// On non-Windows platforms this is a straight string conversion.
fn strip_extended_prefix(path: &Path) -> String {
    let s = path.to_string_lossy();
    #[cfg(windows)]
    if s.starts_with(r"\\?\") {
        return s[4..].to_string();
    }
    s.to_string()
}

