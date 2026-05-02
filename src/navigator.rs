use anyhow::{Context, Result};
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

    /// Get display‑ready path string (without `\\?\` prefix)
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

    /// Navigate to a drive root (e.g., "C:" -> "C:\")
    pub fn go_drive(&mut self, drive_letter: char) -> Result<()> {
        let drive_path = format!("{}:\\", drive_letter.to_ascii_uppercase());
        self.go_to(Path::new(&drive_path))
    }

    /// List all available drives on Windows
    pub fn list_drives() -> Vec<char> {
        let mut drives = Vec::new();
        for letter in b'A'..=b'Z' {
            let path = format!("{}:\\", letter as char);
            if Path::new(&path).exists() {
                drives.push(letter as char);
            }
        }
        drives
    }

    /// Go back to the parent directory
    pub fn go_back(&mut self) -> Result<()> {
        let parent = self.current_dir
            .parent()
            .map(|p| p.to_path_buf())
            .ok_or_else(|| anyhow::anyhow!("Already at root, cannot go back further"))?;
        self.go_to(&parent)
    }
}

/// Remove Windows extended‑length prefix (\\?\) for display.
fn strip_extended_prefix(path: &Path) -> String {
    let s = path.to_string_lossy();
    if s.starts_with(r"\\?\") {
        s[4..].to_string()
    } else {
        s.to_string()
    }
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
        let result = nav.go_to(Path::new("Z:\\nonexistent\\path\\12345"));
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
    fn test_list_drives_includes_c() {
        let drives = Navigator::list_drives();
        assert!(drives.contains(&'C'), "C: drive should be available");
    }

    #[test]
    fn test_go_drive_c() -> Result<()> {
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
    fn test_display_path_removes_prefix() {
        let path = Path::new(r"\\?\C:\Users");
        let result = strip_extended_prefix(path);
        assert_eq!(result, r"C:\Users");
    }
}