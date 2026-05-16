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
        let canonical = path
            .canonicalize()
            .with_context(|| format!("Cannot navigate to: {}", path.display()))?;

        if canonical.is_dir() {
            env::set_current_dir(&canonical)?;
            self.current_dir = canonical;
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
        // ANSI: move cursor to top-left and clear screen
        print!("\x1B[2J\x1B[1;1H");
        let _ = std::io::Write::flush(&mut std::io::stdout());
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::TempDir;

    #[test]
    fn test_navigator_starts_at_current_dir() -> Result<()> {
        let nav = Navigator::new()?;
        assert_eq!(nav.current_path(), env::current_dir()?);
        Ok(())
    }

    #[test]
    fn test_go_to_valid_directory() -> Result<()> {
        let temp = TempDir::new()?;
        let mut nav = Navigator::new()?;
        nav.go_to(temp.path())?;
        assert_eq!(nav.current_path().canonicalize()?, temp.path().canonicalize()?);
        Ok(())
    }

    #[test]
    fn test_go_to_nonexistent_directory_fails() {
        let mut nav = Navigator::new().unwrap();
        // Use a path that doesn't exist on any platform
        let result = nav.go_to(Path::new("/nonexistent/ntc/test/path/xyz"));
        assert!(result.is_err());
    }

    #[test]
    fn test_go_back() -> Result<()> {
        let temp = TempDir::new()?;
        let sub_dir = temp.path().join("sub");
        std::fs::create_dir(&sub_dir)?;

        let mut nav = Navigator::new()?;
        nav.go_to(&sub_dir)?;
        assert!(nav.current_path().ends_with("sub"));

        nav.go_back()?;
        Ok(())
    }

    #[test]
    #[cfg(windows)]
    fn test_list_drives_includes_c_on_windows() {
        let drives = Navigator::list_drives();
        assert!(drives.contains(&'C'), "C: drive should be available");
    }

    #[test]
    #[cfg(not(windows))]
    fn test_list_drives_empty_on_non_windows() {
        let drives = Navigator::list_drives();
        assert!(drives.is_empty(), "list_drives() should return empty on non-Windows");
    }

    #[test]
    #[cfg(windows)]
    fn test_go_drive_c_on_windows() -> Result<()> {
        let mut nav = Navigator::new()?;
        nav.go_drive('C')?;
        let path_str = nav.display_path();
        assert!(
            path_str.contains("C:") || path_str.contains("c:"),
            "Expected path to contain C:, got: {}",
            path_str
        );
        Ok(())
    }

    #[test]
    #[cfg(not(windows))]
    fn test_go_drive_errors_on_non_windows() {
        let mut nav = Navigator::new().unwrap();
        assert!(nav.go_drive('C').is_err());
    }

    #[test]
    #[cfg(windows)]
    fn test_display_path_removes_windows_prefix() {
        let path = Path::new(r"\\?\C:\Users");
        let result = strip_extended_prefix(path);
        assert_eq!(result, r"C:\Users");
    }

    #[test]
    #[cfg(not(windows))]
    fn test_display_path_passthrough_on_non_windows() {
        let path = Path::new("/home/user/projects");
        let result = strip_extended_prefix(path);
        assert_eq!(result, "/home/user/projects");
    }

    #[test]
    fn test_list_subdirs() -> Result<()> {
        let temp = TempDir::new()?;
        std::fs::create_dir(temp.path().join("aaa"))?;
        std::fs::create_dir(temp.path().join("bbb"))?;
        std::fs::create_dir(temp.path().join("ccc"))?;
        // Create a file (should not appear)
        std::fs::write(temp.path().join("file.txt"), "test")?;
        // Create an ignored directory (should not appear)
        std::fs::create_dir(temp.path().join("node_modules"))?;

        let mut nav = Navigator::new()?;
        nav.go_to(temp.path())?;

        let dirs = nav.list_subdirs()?;
        let names: Vec<&str> = dirs.iter().map(|(_, n)| n.as_str()).collect();

        assert_eq!(dirs.len(), 3, "Should list 3 subdirectories (ignoring node_modules and file)");
        assert_eq!(dirs[0], (1, "aaa".to_string()));
        assert_eq!(dirs[1], (2, "bbb".to_string()));
        assert_eq!(dirs[2], (3, "ccc".to_string()));
        assert!(!names.contains(&"node_modules"), "node_modules should be filtered out");
        Ok(())
    }
}