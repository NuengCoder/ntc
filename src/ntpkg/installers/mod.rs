use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::output::print_error;

mod act;
mod cloudflared;
mod devops_tool;
mod docker;
mod docker_compose;
mod dotnet;
mod flutter;
mod gh;
mod git;
mod go;
mod helm;
mod java;
mod kind;
mod kotlin;
mod kubectl;
mod kustomize;
mod kilocode;
mod lua;
mod mimocode;
mod msys2;
mod npm_guide;
mod omniroute;
mod opencode;
mod node;
mod php;
pub(crate) mod python;
mod ruby;
mod rust;
mod scala;
mod sqlite;
mod sqlitestudio;
mod swift;
mod zig;

pub(crate) use act::install_act;
pub(crate) use cloudflared::install_cloudflared;
pub(crate) use devops_tool::install_devops_tool;
pub(crate) use docker::install_docker;
pub(crate) use docker_compose::install_docker_compose;
pub(crate) use dotnet::install_dotnet;
pub(crate) use flutter::install_flutter;
pub(crate) use gh::install_gh;
pub(crate) use git::install_git;
pub(crate) use go::install_go;
pub(crate) use helm::install_helm;
pub(crate) use java::install_java;
pub(crate) use kind::install_kind;
pub(crate) use kotlin::install_kotlin;
pub(crate) use kubectl::install_kubectl;
pub(crate) use kustomize::install_kustomize;
pub(crate) use kilocode::install_kilocode;
pub(crate) use lua::install_lua;
pub(crate) use mimocode::install_mimocode;
pub(crate) use msys2::install_msys2;
pub(crate) use omniroute::install_omniroute;
pub(crate) use opencode::install_opencode;
pub(crate) use node::install_prebuilt_node;
pub(crate) use php::install_php;
pub(crate) use ruby::install_ruby;
pub(crate) use rust::install_rust;
pub(crate) use scala::install_scala;
pub(crate) use sqlite::install_sqlite;
pub(crate) use sqlitestudio::install_sqlitestudio;
pub(crate) use swift::install_swift;
pub(crate) use zig::install_zig;

/// Launch an installer (.exe/.pkg) and wait for the user to finish.
/// On Windows, uses ShellExecute (via PowerShell) to request administrator elevation.
pub(crate) fn launch_installer_and_wait(installer: &Path, name: &str, _version: &str) -> Result<()> {
    #[cfg(windows)]
    {
        let installer_disp = installer.display();
        let script = format!(
            "Start-Process -FilePath '{}' -Wait -Verb RunAs",
            installer_disp.to_string().replace('\'', "''")
        );
        let status = Command::new("powershell")
            .args(["-NoProfile", "-Command", &script])
            .status()
            .with_context(|| format!("Failed to launch {} installer (elevated)", name))?;

        if let Some(code) = status.code() {
            if code != 0 && code != 1 {
                print_error(&format!("{} installer exited with code {:?}", name, status.code()));
            }
        }
    }

    #[cfg(not(windows))]
    {
        let status = Command::new(installer)
            .status()
            .with_context(|| format!("Failed to launch {} installer", name))?;

        if !status.success() {
            print_error(&format!("{} installer exited with code {:?}", name, status.code()));
        }
    }

    Ok(())
}

/// Find an installer (.exe/.pkg) in the given directory.
pub(crate) fn find_installer_in_dir(dir: &Path, name: &str, _version: &str) -> Option<PathBuf> {
    if !dir.is_dir() {
        return None;
    }
    let ext = if cfg!(windows) { "exe" } else { "pkg" };

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(fname) = path.file_name().and_then(|n| n.to_str()) {
                    if fname.starts_with(&format!("{name}-")) && fname.ends_with(&format!(".{ext}")) {
                        return Some(path);
                    }
                }
            }
        }
    }
    None
}
