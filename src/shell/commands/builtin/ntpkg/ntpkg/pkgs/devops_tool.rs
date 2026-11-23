//! devops-tool — version listing, download source and installer.

use super::super::download::download_with_progress;
use super::super::download::extract_archive;
use super::super::download::find_binary;
use super::super::manager::load_manifest;
use super::super::manager::save_manifest;
use super::super::registry;
use super::super::types::ArchiveType;
use super::super::types::PackageSource;
use super::common::empty_source;
use super::common::platform_key;
use super::Pkg;
use crate::output::print_info;
use crate::output::print_success;
use crate::t;
use anyhow::Context;
use anyhow::Result;
use std::fs;
use std::path::Path;

fn latest_versions() -> Result<Vec<String>> {
    Ok(vec!["latest".to_string()])
}

fn npm_only_source(version: &str) -> Option<PackageSource> {
    empty_source("devops-tool", version)
}

const BUNDLED_TOOLS: &[&str] = &[
    "act",
    "kubectl",
    "helm",
    "kind",
    "kustomize",
    "docker",
    "docker-compose",
    "gh",
];

pub(crate) fn install_devops_tool(
    name: &str,
    version: &str,
    _source: &PackageSource,
    vdir: &Path,
) -> Result<()> {
    let combined_bin = vdir.join("bin");
    fs::create_dir_all(&combined_bin)?;

    for &tool_name in BUNDLED_TOOLS {
        print_info(&format!("  Installing {tool_name}..."));
        if let Err(e) = install_single_tool(tool_name, &combined_bin, vdir) {
            print_info(&format!("  Failed to install {tool_name}: {e}"));
            return Err(e);
        }
    }

    fs::write(
        vdir.join(".real_path"),
        combined_bin.to_string_lossy().as_bytes(),
    )
    .context("Failed to write .real_path")?;

    print_success(&format!("{name} {version} installed at {}", vdir.display()));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

fn install_single_tool(tool_name: &str, combined_bin: &Path, _vdir: &Path) -> Result<()> {
    let versions = registry::list_available_versions(tool_name);
    if versions.is_empty() {
        anyhow::bail!("No versions available for '{tool_name}'");
    }
    let latest = &versions[0];

    let source = registry::get_source(tool_name, latest)
        .ok_or_else(|| anyhow::anyhow!("No source for '{tool_name}'"))?;

    let platform_key = platform_key();
    let url = source.urls.get(&platform_key).ok_or_else(|| {
        anyhow::anyhow!("No download URL for '{tool_name}' on platform '{platform_key}'")
    })?;

    let tmp_dir = std::env::temp_dir().join(format!("ntc-bundle-{tool_name}"));
    if tmp_dir.exists() {
        let _ = fs::remove_dir_all(&tmp_dir);
    }
    fs::create_dir_all(&tmp_dir)?;

    let fname = url.rsplit_once('/').map(|(_, f)| f).unwrap_or("archive");
    let download_path = tmp_dir.join(fname);

    download_with_progress(url, &download_path)?;

    if is_archive_url(url) {
        let archive_type = if url.ends_with(".zip") {
            ArchiveType::Zip
        } else {
            ArchiveType::TarGz
        };
        print_info(&t!("ntpkg.extracting_indent"));
        extract_archive(&download_path, &tmp_dir, archive_type)?;

        let binary_name = if cfg!(windows) {
            format!("{}.exe", tool_name)
        } else {
            tool_name.to_string()
        };

        let found = find_binary(&tmp_dir, tool_name)
            .ok_or_else(|| anyhow::anyhow!("Could not find {tool_name} binary in archive"))?;

        let dest = combined_bin.join(&binary_name);
        fs::copy(&found, &dest)
            .with_context(|| format!("Failed to copy {tool_name} to {}", dest.display()))?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(&dest, fs::Permissions::from_mode(0o755));
        }
    } else {
        let binary_name = if cfg!(windows) {
            format!("{}.exe", tool_name)
        } else {
            tool_name.to_string()
        };
        let target = combined_bin.join(&binary_name);
        if fs::rename(&download_path, &target).is_err() {
            fs::copy(&download_path, &target)
                .with_context(|| format!("Failed to copy {tool_name} to {}", target.display()))?;
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(&target, fs::Permissions::from_mode(0o755));
        }
    }

    let _ = fs::remove_dir_all(&tmp_dir);

    Ok(())
}

fn is_archive_url(url: &str) -> bool {
    url.ends_with(".tar.gz")
        || url.ends_with(".tgz")
        || url.ends_with(".tar.xz")
        || url.ends_with(".zip")
}

pub(crate) static PKG: Pkg = Pkg {
    name: "devops-tool",
    termux: None,
    description_key: "ntpkg.src_devops_tool",
    versions: latest_versions,
    source: npm_only_source,
    install: install_devops_tool,
};
