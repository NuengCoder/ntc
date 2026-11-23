//! sqlite — version listing, download source and installer.

use super::super::download::download_with_progress;
use super::super::download::extract_archive;
use super::super::download::find_binary;
use super::super::manager::load_manifest;
use super::super::manager::save_manifest;
use super::super::types::ArchiveType;
use super::super::types::PackageSource;
use super::common::copy_recursive;
use super::common::platform_key;
use super::Pkg;
use crate::output::print_info;
use crate::output::print_success;
use crate::t;
use anyhow::Context;
use anyhow::Result;
use std::fs;
use std::io::Read;
use std::path::Path;

fn version_to_sqlite_code(version: &str) -> Option<u32> {
    let parts: Vec<u32> = version.split('.').filter_map(|s| s.parse().ok()).collect();
    if parts.len() < 3 {
        return None;
    }
    Some(parts[0] * 1_000_000 + parts[1] * 10_000 + parts[2] * 100)
}

fn fetch_sqlite_versions() -> Result<Vec<String>> {
    let url = "https://www.sqlite.org/download.html";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch SQLite download page")?;

    let mut body = response.into_body();
    let mut html = String::new();
    body.as_reader()
        .read_to_string(&mut html)
        .context("Failed to read SQLite download page")?;

    let mut seen = std::collections::HashSet::new();
    let mut versions: Vec<String> = Vec::new();

    // Pattern: sqlite-tools-{platform}-{versioncode}.zip
    // Platform may contain hyphens (e.g. "linux-x64"), so find from the right
    for (i, _) in html.match_indices("sqlite-tools-") {
        let fragment = &html[i..];
        let zip_end = match fragment.find(".zip") {
            Some(pos) => pos,
            None => continue,
        };
        let before_zip = &fragment[..zip_end];
        // Find the last '-' before .zip — that's where the version code starts
        let code_start = match before_zip.rfind('-') {
            Some(pos) => pos + 1,
            None => continue,
        };
        let code_str = &before_zip[code_start..];
        let code: u32 = match code_str.parse() {
            Ok(c) => c,
            Err(_) => continue,
        };

        let major = code / 1_000_000;
        let minor = (code % 1_000_000) / 10_000;
        let patch = (code % 10_000) / 100;
        let version = format!("{major}.{minor}.{patch}");

        if seen.insert(version.clone()) {
            versions.push(version);
        }
    }

    versions.sort_by(|a, b| {
        let a_parts: Vec<u32> = a.split('.').filter_map(|p| p.parse().ok()).collect();
        let b_parts: Vec<u32> = b.split('.').filter_map(|p| p.parse().ok()).collect();
        b_parts.cmp(&a_parts)
    });

    Ok(versions)
}

fn sqlite_source(version: &str) -> Option<PackageSource> {
    let code = version_to_sqlite_code(version)?;

    // Find the year from the download page
    let year = find_sqlite_year_for_code(code).unwrap_or_else(current_year);

    let platform_map: [(&str, &[&str]); 5] = [
        ("linux-x86_64", &["linux-x64"]),
        ("windows-x86_64", &["win-x64"]),
        ("windows-aarch64", &["win-arm64"]),
        ("macos-x86_64", &["osx-x64"]),
        ("macos-aarch64", &["osx-arm64"]),
    ];

    let mut urls = std::collections::HashMap::new();
    for (key, _aliases) in &platform_map {
        let platform = _aliases[0];
        let url = format!("https://www.sqlite.org/{year}/sqlite-tools-{platform}-{code}.zip");
        urls.insert(key.to_string(), url);
    }

    Some(PackageSource {
        name: "sqlite".to_string(),
        version: version.to_string(),
        urls,
        sha256: std::collections::HashMap::new(),
        archive_type: ArchiveType::Zip,
    })
}

fn find_sqlite_year_for_code(code: u32) -> Option<u32> {
    let url = "https://www.sqlite.org/download.html";
    let response = ureq::get(url).header("User-Agent", "ntc").call().ok()?;

    let mut body = response.into_body();
    let mut html = String::new();
    body.as_reader().read_to_string(&mut html).ok()?;

    // CSV lines: PRODUCT,version,year/sqlite-tools-{platform}-{code}.zip,...
    let target = format!("-{code}.zip");
    for (i, _) in html.match_indices(&target) {
        let before = &html[..i];
        // Look backwards for '/' which separates the year
        if let Some(slash_pos) = before.rfind('/') {
            let _after_slash = &before[slash_pos + 1..];
            // after_slash should be "sqlite-tools-..." — look before the slash
            let before_slash = &before[..slash_pos];
            // The year is the last segment before the slash
            if let Some(comma_pos) = before_slash.rfind(',') {
                let year_str = before_slash[comma_pos + 1..].trim();
                if let Ok(year) = year_str.parse::<u32>() {
                    if year > 2000 && year < 2100 {
                        return Some(year);
                    }
                }
            }
        }
    }

    None
}

fn current_year() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let dur = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    (1970 + dur.as_secs() / 31_536_000) as u32
}

pub(crate) fn install_sqlite(
    name: &str,
    version: &str,
    source: &PackageSource,
    vdir: &Path,
) -> Result<()> {
    let platform_key = platform_key();
    let url = source
        .urls
        .get(&platform_key)
        .ok_or_else(|| anyhow::anyhow!("No download URL for platform '{platform_key}'"))?;

    let tmp_dir = std::env::temp_dir().join(format!("ntc-install-{name}-{version}"));
    if tmp_dir.exists() {
        let _ = fs::remove_dir_all(&tmp_dir);
    }
    fs::create_dir_all(&tmp_dir)?;

    let ext = match source.archive_type {
        super::super::types::ArchiveType::Zip => "zip",
        _ => "zip",
    };
    let archive_name = format!("{name}-tools-{version}.{ext}");
    let archive_path = tmp_dir.join(&archive_name);

    print_info(&format!("Downloading {name} {version}..."));
    download_with_progress(url, &archive_path)?;

    print_info(&t!("ntpkg.extracting"));
    extract_archive(&archive_path, &tmp_dir, source.archive_type)?;

    let sqlite3_bin = find_binary(&tmp_dir, "sqlite3")
        .ok_or_else(|| anyhow::anyhow!("Could not find sqlite3 binary in archive"))?;
    let sqlite3_dir = sqlite3_bin
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid SQLite directory structure"))?;

    fs::create_dir_all(vdir)?;
    copy_recursive(sqlite3_dir, &vdir.join("bin"))?;

    fs::write(
        vdir.join(".real_path"),
        vdir.join("bin").to_string_lossy().as_bytes(),
    )?;

    let _ = fs::remove_dir_all(&tmp_dir);

    print_success(&format!("{name} {version} installed at {}", vdir.display()));

    let mut manifest = load_manifest()?;
    manifest.add_version(name, version);
    manifest.activate(name, version);
    save_manifest(&manifest)?;

    Ok(())
}

pub(crate) static PKG: Pkg = Pkg {
    name: "sqlite",
    termux: Some("sqlite"),
    description_key: "ntpkg.src_sqlite",
    versions: fetch_sqlite_versions,
    source: sqlite_source,
    install: install_sqlite,
};
