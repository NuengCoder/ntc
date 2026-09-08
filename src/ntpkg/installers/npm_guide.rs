use anyhow::Result;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::super::manager::load_manifest;
use super::super::types::version_dir;
use crate::output::{print_info, print_success};
use crate::t;

pub(crate) fn print_npm_guide(name: &str, npm_pkg: &str) -> Result<()> {
    let node_path = find_node();
    let npm_path = find_npm();

    let (node_ok, npm_ok) = (node_path.is_some(), npm_path.is_some());

    if !node_ok || !npm_ok {
        print_info(&t!("ntpkg.npm_not_detected"));
        println!();
        println!("{}", t!("ntpkg.npm_section_node_first"));
        println!();

        if let Some(ref np) = node_path {
            println!("{}", t!("ntpkg.npm_node_found_at", np.display()));
        } else {
            println!("{}", t!("ntpkg.npm_node_not_found"));
            println!();
            if super::super::manager::is_termux() {
                println!("{}", t!("ntpkg.npm_install_via_pkg"));
                print_info(&t!("ntpkg.termux_cmd", "nodejs"));
            } else {
                println!("{}", t!("ntpkg.npm_install_via_ntpkg"));
                println!("{}", t!("ntpkg.npm_ntpkg_cmd", "node"));
                println!();
                println!("{}", t!("ntpkg.npm_or_download_nodejs"));
                println!();
                println!("{}", t!("ntpkg.npm_or_fnm_nvm_volta"));
            }
        }

        if let Some(ref np) = npm_path {
            println!("{}", t!("ntpkg.npm_found_at", np.display()));
        } else {
            println!("{}", t!("ntpkg.npm_not_found"));
        }

        println!();
        if super::super::manager::is_termux() {
            println!("{}", t!("ntpkg.npm_then_run"));
            println!("{}", t!("ntpkg.npm_cmd", npm_pkg));
        } else {
            println!("{}", t!("ntpkg.npm_then_rerun"));
            println!("{}", t!("ntpkg.npm_ntpkg_cmd", name));
        }
        println!();
    } else {
        print_success(&t!("ntpkg.npm_detected"));
        println!();

        let npm_bin = detect_npm_bin_dir();
        if let Some(ref bin_dir) = npm_bin {
            if !is_in_path(bin_dir) {
                print_info(&t!(
                    "ntpkg.npm_bin_not_in_path",
                    bin_dir.display()
                ));
                println!("{}", t!("ntpkg.path_add_label"));
                println!("    {}", bin_dir.display());
                println!();
            } else {
                print_success(&t!("ntpkg.npm_bin_in_path"));
                println!();
            }
        }

        println!("{}", t!("ntpkg.npm_install_section", name));
        println!();
        println!("{}", t!("ntpkg.npm_run_cmd_label"));
        println!("{}", t!("ntpkg.npm_cmd", npm_pkg));
        println!();
    }

    Ok(())
}

fn find_node() -> Option<PathBuf> {
    if let Some(p) = find_on_path("node") {
        return Some(p);
    }
    if let Some(p) = find_ntpkg_node_bin() {
        return Some(p);
    }
    find_node_common_locations()
}

fn find_npm() -> Option<PathBuf> {
    if let Some(p) = find_on_path("npm") {
        return Some(p);
    }
    if let Some(p) = find_ntpkg_npm_bin() {
        return Some(p);
    }
    find_npm_common_locations()
}

fn find_on_path(cmd: &str) -> Option<PathBuf> {
    #[cfg(windows)]
    {
        let output = Command::new("where").arg(cmd).output().ok()?;
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let first = stdout.lines().next()?.trim().to_string();
            if !first.is_empty() {
                return Some(PathBuf::from(first));
            }
        }
        None
    }
    #[cfg(not(windows))]
    {
        let output = Command::new("which").arg(cmd).output().ok()?;
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return Some(PathBuf::from(path));
            }
        }
        None
    }
}

fn find_ntpkg_node_bin() -> Option<PathBuf> {
    let manifest = load_manifest().ok()?;
    let active_ver = manifest.active.get("node")?;
    if !manifest.is_installed("node", active_ver) {
        return None;
    }
    let dir = version_dir("node", active_ver);
    let node_exe = if cfg!(windows) { "node.exe" } else { "node" };
    let candidate = dir.join(node_exe);
    if candidate.exists() {
        Some(candidate)
    } else {
        None
    }
}

fn find_ntpkg_npm_bin() -> Option<PathBuf> {
    let manifest = load_manifest().ok()?;
    let active_ver = manifest.active.get("node")?;
    if !manifest.is_installed("node", active_ver) {
        return None;
    }
    let dir = version_dir("node", active_ver);
    let npm_exe = if cfg!(windows) { "npm.cmd" } else { "npm" };

    let candidate = dir.join(npm_exe);
    if candidate.exists() {
        return Some(candidate);
    }

    let candidate2 = dir.join("bin").join("npm");
    if candidate2.exists() {
        return Some(candidate2);
    }

    None
}

fn find_node_common_locations() -> Option<PathBuf> {
    #[cfg(windows)]
    {
        let candidates = vec![
            PathBuf::from(r"C:\Program Files\nodejs\node.exe"),
            PathBuf::from(r"C:\Program Files (x86)\nodejs\node.exe"),
        ];
        for c in candidates {
            if c.exists() {
                return Some(c);
            }
        }

        if let Ok(localappdata) = std::env::var("LOCALAPPDATA") {
            let p = PathBuf::from(localappdata)
                .join("fnm")
                .join("nodejs")
                .join("node.exe");
            if p.exists() {
                return Some(p);
            }
        }

        if let Ok(userprofile) = std::env::var("USERPROFILE") {
            let nvm_root = PathBuf::from(&userprofile).join(r"AppData\Roaming\nvm");
            if let Ok(entries) = std::fs::read_dir(&nvm_root) {
                for entry in entries.flatten() {
                    let p = entry.path().join("node.exe");
                    if p.exists() {
                        return Some(p);
                    }
                }
            }
        }

        None
    }
    #[cfg(not(windows))]
    {
        let candidates = vec![
            PathBuf::from("/usr/local/bin/node"),
            PathBuf::from("/usr/bin/node"),
            PathBuf::from("/opt/homebrew/bin/node"),
        ];
        candidates.into_iter().find(|c| c.exists())
    }
}

fn find_npm_common_locations() -> Option<PathBuf> {
    #[cfg(windows)]
    {
        let candidates = vec![
            PathBuf::from(r"C:\Program Files\nodejs\npm.cmd"),
            PathBuf::from(r"C:\Program Files (x86)\nodejs\npm.cmd"),
        ];
        for c in candidates {
            if c.exists() {
                return Some(c);
            }
        }

        if let Ok(localappdata) = std::env::var("LOCALAPPDATA") {
            let p = PathBuf::from(localappdata)
                .join("fnm")
                .join("nodejs")
                .join("npm.cmd");
            if p.exists() {
                return Some(p);
            }
        }

        if let Ok(userprofile) = std::env::var("USERPROFILE") {
            let nvm_root = PathBuf::from(&userprofile).join(r"AppData\Roaming\nvm");
            if let Ok(entries) = std::fs::read_dir(&nvm_root) {
                for entry in entries.flatten() {
                    let p = entry.path().join("npm.cmd");
                    if p.exists() {
                        return Some(p);
                    }
                }
            }
        }

        None
    }
    #[cfg(not(windows))]
    {
        let candidates = vec![
            PathBuf::from("/usr/local/bin/npm"),
            PathBuf::from("/usr/bin/npm"),
            PathBuf::from("/opt/homebrew/bin/npm"),
        ];
        candidates.into_iter().find(|c| c.exists())
    }
}

fn detect_npm_bin_dir() -> Option<PathBuf> {
    let output = Command::new("npm").args(["root", "-g"]).output().ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let root_dir = stdout.trim();
    if root_dir.is_empty() {
        return None;
    }

    let root_path = Path::new(root_dir);
    root_path.parent().map(|p| p.to_path_buf())
}

fn is_in_path(dir: &Path) -> bool {
    let path = std::env::var_os("PATH").unwrap_or(OsString::new());
    let dir_str = dir.to_string_lossy().to_lowercase();
    std::env::split_paths(&path).any(|p| p.to_string_lossy().to_lowercase() == dir_str)
}
