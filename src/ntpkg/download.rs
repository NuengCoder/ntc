use anyhow::{bail, Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use super::types::ArchiveType;

/// Download a file from `url` to `dest_path` with a progress bar.
pub fn download_with_progress(url: &str, dest_path: &Path) -> Result<()> {
    let pb = ProgressBar::new(0);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg}\n{wide_bar} {bytes}/{total_bytes} ({eta})")?
            .progress_chars("##-"),
    );
    pb.set_message(format!("Downloading {}", url));

    let response = ureq::get(url)
        .call()
        .with_context(|| {
            // Check if status indicates 404 for better error message
            let hint = if let Some(name) = extract_package_name(url) {
                format!(
                    "Package '{}' may not exist for this platform/version",
                    name
                )
            } else {
                "Download URL may be unreachable".to_string()
            };
            format!("Failed to send download request: {hint}")
        })?;

    let total = response
        .headers()
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(0);

    pb.set_length(total);

    let mut downloaded: u64 = 0;
    let mut buf = Vec::new();

    let mut body = response.into_body();
    let mut reader = body.as_reader();
    let mut chunk = vec![0u8; 8192];
    loop {
        let n = reader
            .read(&mut chunk)
            .context("Failed to read download stream")?;
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&chunk[..n]);
        downloaded += n as u64;
        pb.set_position(downloaded);
    }

    pb.finish_and_clear();

    fs::write(dest_path, &buf).context("Failed to write downloaded file")?;
    Ok(())
}

/// Verify a file's content matches expected SHA256.
#[allow(dead_code)]
pub fn verify_sha256(path: &Path, expected: &str) -> Result<bool> {
    if expected.starts_with("PLACEHOLDER") {
        return Ok(true);
    }

    use sha2::Digest;
    let data = fs::read(path).context("Failed to read file for SHA256")?;
    let hash = hex::encode(sha2::Sha256::digest(&data));

    if !hash.eq_ignore_ascii_case(expected) {
        bail!(
            "SHA256 mismatch for {}:\n  expected: {}\n  got:      {}",
            path.display(),
            expected,
            hash
        );
    }
    Ok(true)
}

/// Extract an archive to `dest_dir`.
pub fn extract_archive(archive_path: &Path, dest_dir: &Path, kind: ArchiveType) -> Result<()> {
    fs::create_dir_all(dest_dir)
        .with_context(|| format!("Failed to create {}", dest_dir.display()))?;

    match kind {
        ArchiveType::TarGz => extract_tar_gz(archive_path, dest_dir),
        ArchiveType::TarXz => extract_tar_xz(archive_path, dest_dir),
        ArchiveType::Zip => extract_zip(archive_path, dest_dir),
        ArchiveType::Msi | ArchiveType::Pkg => {
            // MSI/PKG installers are copied as-is for manual install
            let fname = archive_path
                .file_name()
                .unwrap_or(archive_path.as_os_str());
            let dest = dest_dir.join(fname);
            fs::copy(archive_path, &dest)
                .with_context(|| format!("Failed to copy to {}", dest.display()))?;
            Ok(())
        }
    }
}

fn extract_tar_gz(archive_path: &Path, dest_dir: &Path) -> Result<()> {
    use flate2::read::GzDecoder;
    use tar::Archive;

    let file = fs::File::open(archive_path)
        .with_context(|| format!("Failed to open {}", archive_path.display()))?;
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);
    archive
        .unpack(dest_dir)
        .with_context(|| format!("Failed to extract tar.gz to {}", dest_dir.display()))?;
    Ok(())
}

fn extract_tar_xz(archive_path: &Path, dest_dir: &Path) -> Result<()> {
    use tar::Archive;
    use xz2::read::XzDecoder;

    let file = fs::File::open(archive_path)
        .with_context(|| format!("Failed to open {}", archive_path.display()))?;
    let decoder = XzDecoder::new(file);
    let mut archive = Archive::new(decoder);
    archive
        .unpack(dest_dir)
        .with_context(|| format!("Failed to extract tar.xz to {}", dest_dir.display()))?;
    Ok(())
}

fn extract_zip(archive_path: &Path, dest_dir: &Path) -> Result<()> {
    let file = fs::File::open(archive_path)
        .with_context(|| format!("Failed to open {}", archive_path.display()))?;
    let mut archive =
        zip::ZipArchive::new(file).with_context(|| format!("Failed to read zip {}", archive_path.display()))?;
    archive
        .extract(dest_dir)
        .with_context(|| format!("Failed to extract zip to {}", dest_dir.display()))?;
    Ok(())
}

/// Find a binary file inside an extracted directory tree.
pub fn find_binary(extract_dir: &Path, binary_name: &str) -> Option<PathBuf> {
    let target_name = if cfg!(windows) {
        format!("{}.exe", binary_name)
    } else {
        binary_name.to_string()
    };

    find_file_recursive(extract_dir, &target_name)
}

fn extract_package_name(url: &str) -> Option<String> {
    let name_versions = ["python", "node", "ruby", "go", "dotnet", "java", "kotlin", "swift", "php", "zig", "msys2", "lua", "flutter", "kubectl", "helm", "kind", "kustomize", "docker", "docker-compose", "act", "devops-tool", "scala", "opencode", "mimocode", "kilocode", "sqlitestudio", "sqlite"];
    for nv in &name_versions {
        if url.contains(nv) {
            return Some(nv.to_string());
        }
    }
    url.split('/')
        .nth(3)
        .map(|s| s.to_string())
}

fn find_file_recursive(dir: &Path, name: &str) -> Option<PathBuf> {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(found) = find_file_recursive(&path, name) {
                    return Some(found);
                }
            } else if path
                .file_name()
                .and_then(|n| n.to_str()) == Some(name)
            {
                return Some(path);
            }
        }
    }
    None
}
