//! Harness path table plus the copy/remove/hash helpers used by the command.
//!
//! Skills are always copied, never symlinked: Windows often lacks the
//! privilege, and a copy keeps each harness's folder reviewable.

use anyhow::{bail, Context, Result};
use sha2::Digest;
use std::fs;
use std::path::{Path, PathBuf};

pub const HARNESSES: [&str; 9] = [
    "opencode",
    "claude",
    "codex",
    "agents",
    "freebuff",
    "kilocode",
    "mimocode",
    "workbuddy",
    "trae",
];

pub fn is_harness(name: &str) -> bool {
    HARNESSES.iter().any(|h| h.eq_ignore_ascii_case(name))
}

pub fn canonical_harness(name: &str) -> Result<&'static str> {
    HARNESSES
        .iter()
        .copied()
        .find(|h| h.eq_ignore_ascii_case(name.trim()))
        .with_context(|| format!("unknown harness '{}' (use {})", name, HARNESSES.join(", ")))
}

/// Every harness path depends only on these bases, so tests can fake them.
struct PathBase<'a> {
    project: &'a Path,
    home: &'a Path,
    /// Resolved OpenCode config dir (`OPENCODE_CONFIG_DIR` / `%APPDATA%`).
    opencode: Option<&'a Path>,
    /// XDG config root (`~/.config` unless `XDG_CONFIG_HOME` is set).
    xdg: &'a Path,
}

fn resolve_target(harness: &str, global: bool, base: &PathBase) -> Result<PathBuf> {
    let harness = canonical_harness(harness)?;
    if !global {
        return Ok(base.project.join(format!(".{}", harness)).join("skills"));
    }
    let root = match harness {
        "opencode" => match base.opencode {
            Some(dir) => dir.to_path_buf(),
            None => base.xdg.join("opencode"),
        },
        _ => base.home.join(format!(".{}", harness)),
    };
    Ok(root.join("skills"))
}

/// `<cwd>/.<harness>/skills` or the personal folder with `global`.
pub fn target_root(harness: &str, global: bool) -> Result<PathBuf> {
    let project = std::env::current_dir().context("current directory")?;
    let home = dirs::home_dir().context("home directory")?;
    let xdg = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .unwrap_or_else(|| home.join(".config"));
    let opencode = opencode_config_dir(&home);
    resolve_target(
        harness,
        global,
        &PathBase {
            project: &project,
            home: &home,
            opencode: opencode.as_deref(),
            xdg: &xdg,
        },
    )
}

/// `OPENCODE_CONFIG_DIR`, then `%APPDATA%/opencode` when it exists, then the
/// XDG location (`None` means "use XDG").
fn opencode_config_dir(home: &Path) -> Option<PathBuf> {
    if let Some(dir) = std::env::var_os("OPENCODE_CONFIG_DIR") {
        let path = PathBuf::from(dir);
        if !path.as_os_str().is_empty() {
            return Some(path);
        }
    }
    #[cfg(windows)]
    {
        let _ = home;
        if let Some(base) = dirs::config_dir() {
            let path = base.join("opencode");
            if path.is_dir() {
                return Some(path);
            }
        }
    }
    #[cfg(not(windows))]
    {
        let _ = home;
    }
    None
}

pub fn sha256_file(path: &Path) -> Result<String> {
    let data = fs::read(path).with_context(|| format!("read {}", path.display()))?;
    Ok(hex::encode(sha2::Sha256::digest(&data)))
}

/// Recursively copy text/assets; symlinks are skipped rather than followed.
pub fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst).with_context(|| format!("create {}", dst.display()))?;
    for entry in walkdir::WalkDir::new(src).follow_links(false) {
        let entry = entry.with_context(|| format!("walk {}", src.display()))?;
        let rel = entry
            .path()
            .strip_prefix(src)
            .with_context(|| format!("relative path for {}", entry.path().display()))?;
        if rel.as_os_str().is_empty() {
            continue;
        }
        let target = dst.join(rel);
        let file_type = entry.file_type();
        if file_type.is_symlink() {
            continue;
        }
        if file_type.is_dir() {
            fs::create_dir_all(&target)?;
        } else if file_type.is_file() {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(entry.path(), &target).with_context(|| {
                format!("copy {} -> {}", entry.path().display(), target.display())
            })?;
        }
    }
    Ok(())
}

pub fn remove_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }
    if !path.is_dir() {
        bail!("{} is not a directory", path.display());
    }
    fs::remove_dir_all(path).with_context(|| format!("remove {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base<'a>(project: &'a Path, home: &'a Path) -> PathBase<'a> {
        PathBase {
            project,
            home,
            opencode: None,
            xdg: home,
        }
    }

    #[test]
    fn project_targets_use_the_harness_dot_folder() {
        let dir = tempfile::tempdir().unwrap();
        let project = dir.path().join("work");
        let home = dir.path().join("home");
        for harness in HARNESSES {
            let path = resolve_target(harness, false, &base(&project, &home)).unwrap();
            assert_eq!(
                path,
                project.join(format!(".{harness}")).join("skills"),
                "{harness}"
            );
        }
    }

    #[test]
    fn global_targets_use_each_tools_home_folder() {
        let dir = tempfile::tempdir().unwrap();
        let project = dir.path().join("work");
        let home = dir.path().join("home");
        for harness in ["claude", "codex", "agents"] {
            let path = resolve_target(harness, true, &base(&project, &home)).unwrap();
            assert_eq!(path, home.join(format!(".{harness}")).join("skills"));
        }
        // OpenCode without an explicit config dir falls back to XDG.
        let path = resolve_target("opencode", true, &base(&project, &home)).unwrap();
        assert_eq!(path, home.join("opencode").join("skills"));
    }

    #[test]
    fn opencode_config_dir_wins_when_present() {
        let dir = tempfile::tempdir().unwrap();
        let project = dir.path().join("work");
        let home = dir.path().join("home");
        let opencode = dir.path().join("appdata/opencode");
        let path = resolve_target(
            "opencode",
            true,
            &PathBase {
                project: &project,
                home: &home,
                opencode: Some(&opencode),
                xdg: &home,
            },
        )
        .unwrap();
        assert_eq!(path, opencode.join("skills"));
    }

    #[test]
    fn unknown_harness_lists_valid_names() {
        let err = canonical_harness("cursor").unwrap_err().to_string();
        for harness in HARNESSES {
            assert!(err.contains(harness), "{err}");
        }
        assert!(canonical_harness("CLAUDE").is_ok());
    }

    #[test]
    fn copy_dir_skips_symlinks_and_keeps_nested_files() {
        let dir = tempfile::tempdir().unwrap();
        let src = dir.path().join("src");
        fs::create_dir_all(src.join("scripts")).unwrap();
        fs::write(src.join("SKILL.md"), "body").unwrap();
        fs::write(src.join("scripts/run.sh"), "echo hi").unwrap();
        let dst = dir.path().join("dst");
        copy_dir(&src, &dst).unwrap();
        assert_eq!(fs::read_to_string(dst.join("SKILL.md")).unwrap(), "body");
        assert!(dst.join("scripts/run.sh").is_file());
        assert!(sha256_file(&dst.join("SKILL.md")).unwrap().len() == 64);
    }
}
