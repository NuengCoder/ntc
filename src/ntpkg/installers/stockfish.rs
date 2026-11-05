use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use super::super::download::{download_with_progress, extract_archive};
use super::super::manager::{load_manifest, platform_key, save_manifest};
use super::super::types::{bin_dir, ArchiveType, PackageSource};
use crate::output::{print_info, print_success};
use crate::t;

pub(crate) fn install_stockfish(
    name: &str,
    version: &str,
    source: &PackageSource,
    vdir: &Path,
) -> Result<()> {
    let platform_key = platform_key();
    let url = source.urls.get(&platform_key).ok_or_else(|| {
        anyhow::anyhow!("Stockfish has no prebuilt binary for the '{platform_key}' platform")
    })?;

    let tmp_dir = std::env::temp_dir().join(format!("ntc-install-{name}-{version}"));
    if tmp_dir.exists() {
        let _ = fs::remove_dir_all(&tmp_dir);
    }
    fs::create_dir_all(&tmp_dir)?;

    let fname = url.rsplit_once('/').map(|(_, f)| f).unwrap_or("archive");
    let archive_path = tmp_dir.join(fname);

    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &archive_path)?;

    print_info(&t!("ntpkg.extracting"));
    extract_archive(&archive_path, &tmp_dir, archive_kind(url))?;

    let found = find_stockfish_binary(&tmp_dir).ok_or_else(|| {
        anyhow::anyhow!("Could not find the Stockfish binary in the downloaded archive")
    })?;

    let target_dir = bin_dir(name, version);
    fs::create_dir_all(&target_dir)?;
    let dest = target_dir.join(if cfg!(windows) {
        "stockfish.exe"
    } else {
        "stockfish"
    });
    fs::copy(&found, &dest)
        .with_context(|| format!("Failed to copy Stockfish binary to {}", dest.display()))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&dest, fs::Permissions::from_mode(0o755))?;
    }

    fs::write(
        vdir.join(".real_path"),
        target_dir.to_string_lossy().as_bytes(),
    )
    .context("Failed to write .real_path")?;

    let _ = fs::remove_dir_all(&tmp_dir);

    print_success(&format!("{name} {version} installed at {}", vdir.display()));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

fn archive_kind(url: &str) -> ArchiveType {
    let u = url.to_ascii_lowercase();
    if u.ends_with(".zip") {
        ArchiveType::Zip
    } else if u.ends_with(".tar.gz") || u.ends_with(".tgz") {
        ArchiveType::TarGz
    } else {
        ArchiveType::Tar
    }
}

/// Official archives name the engine after the build
/// (`stockfish-windows-x86-64-universal.exe`, `stockfish-ubuntu-x86-64`),
/// so accept any `stockfish`/`stockfish-*` executable as well as the
/// plain name some builds use.
fn find_stockfish_binary(dir: &Path) -> Option<PathBuf> {
    let mut files = Vec::new();
    collect_files(dir, &mut files);

    let exact = if cfg!(windows) {
        "stockfish.exe"
    } else {
        "stockfish"
    };
    if let Some(hit) = files
        .iter()
        .find(|p| p.file_name().and_then(|n| n.to_str()) == Some(exact))
    {
        return Some(hit.clone());
    }

    files.into_iter().find(|p| {
        let Some(name) = p.file_name().and_then(|n| n.to_str()) else {
            return false;
        };
        let n = name.to_ascii_lowercase();
        if cfg!(windows) {
            n.starts_with("stockfish-") && n.ends_with(".exe")
        } else {
            n.starts_with("stockfish-") && !n.contains('.')
        }
    })
}

fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_files(&path, out);
            } else {
                out.push(path);
            }
        }
    }
}
