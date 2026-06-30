use std::collections::HashMap;
use std::io::Read;
use std::sync::Mutex;

use anyhow::{Context, Result};

use super::types::{ArchiveType, PackageSource};

/// In-memory cache: runtime name -> sorted list of available versions
static VERSION_CACHE: Mutex<Option<HashMap<String, Vec<String>>>> = Mutex::new(None);

/// Return cached versions only (no HTTP fetch). Used by the hinter on every keystroke.
pub fn peek_cached_versions(name: &str) -> Vec<String> {
    if let Ok(cache) = VERSION_CACHE.lock() {
        if let Some(ref map) = *cache {
            if let Some(versions) = map.get(name) {
                return versions.clone();
            }
        }
    }
    vec![]
}

/// Return available versions for a language by fetching from the upstream index.
pub fn list_available_versions(name: &str) -> Vec<String> {
    match fetch_available_versions(name) {
        Ok(versions) => versions,
        Err(e) => {
            eprintln!("ntpkg: failed to fetch versions for '{name}': {e}");
            vec![]
        }
    }
}

fn fetch_available_versions(name: &str) -> Result<Vec<String>> {
    if let Ok(cache) = VERSION_CACHE.lock() {
        if let Some(ref map) = *cache {
            if let Some(versions) = map.get(name) {
                return Ok(versions.clone());
            }
        }
    }

    let versions = match name {
        "python"    => fetch_python_versions()?,
        "node"      => fetch_node_versions()?,
        "cloudflare" => vec!["latest".to_string()],
        _ => vec![],
    };

    if let Ok(mut cache) = VERSION_CACHE.lock() {
        cache.get_or_insert_with(HashMap::new).insert(name.to_string(), versions.clone());
    }

    Ok(versions)
}

/// Return available sources for a given language + version.
pub fn get_source(name: &str, version: &str) -> Option<PackageSource> {
    match name {
        "python"    => python_source(version),
        "node"      => node_source(version),
        "cloudflare" => cloudflare_source(version),
        _ => None,
    }
}

/// List all supported languages.
#[allow(dead_code)]
pub fn list_supported_languages() -> Vec<&'static str> {
    vec!["python", "node", "cloudflare"]
}

// ── Python ───────────────────────────────────────────────────────────

fn fetch_python_versions() -> Result<Vec<String>> {
    let url = "https://www.python.org/ftp/python/";
    let response = ureq::get(url)
        .call()
        .context("Failed to fetch Python version list from python.org")?;

    let mut body = response.into_body();
    let mut html = String::new();
    body.as_reader()
        .read_to_string(&mut html)
        .context("Failed to read python.org response body")?;

    let mut versions: Vec<String> = Vec::new();
    let mut search_start = 0;

    while let Some(href_start) = html[search_start..].find("<a href=\"") {
        let start = search_start + href_start + 9; // after <a href="
        if let Some(quote_end) = html[start..].find('"') {
            let url_target = &html[start..start + quote_end];

            // Look for version-like directory entries ending with /
            if url_target.ends_with('/') {
                let candidate = url_target.trim_end_matches('/');
                if is_valid_version(candidate) && !versions.iter().any(|v| v == candidate) {
                    versions.push(candidate.to_string());
                }
            }

            search_start = start + quote_end + 1;
        } else {
            break;
        }
    }

    versions.sort_by(|a, b| {
        let a_parts: Vec<u32> = a.split('.').filter_map(|p| p.parse().ok()).collect();
        let b_parts: Vec<u32> = b.split('.').filter_map(|p| p.parse().ok()).collect();
        b_parts.cmp(&a_parts)
    });

    Ok(versions)
}

fn is_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    if !parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit())) {
        return false;
    }
    true
}

fn python_source(version: &str) -> Option<PackageSource> {
    let base = format!("https://www.python.org/ftp/python/{version}/");

    let (platform_key, filename, archive_type) = platform_python_archive(version)?;

    let url = format!("{base}{filename}");
    let mut urls = HashMap::new();
    urls.insert(platform_key.to_string(), url);

    Some(PackageSource {
        name: "python".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type,
    })
}

#[allow(unused_variables)]
fn platform_python_archive(version: &str) -> Option<(&'static str, String, ArchiveType)> {
    #[cfg(target_os = "windows")]
    {
        let f = format!("python-{version}-amd64.exe");
        Some(("windows-x86_64", f, ArchiveType::Msi))
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        let f = format!("Python-{version}.tar.xz");
        Some(("linux-x86_64", f, ArchiveType::TarGz))
    }

    #[cfg(target_os = "macos")]
    {
        let f = format!("python-{version}-macos11.pkg");
        Some(("macos-universal", f, ArchiveType::Pkg))
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "android", target_os = "macos")))]
    {
        None
    }
}

// ── Node.js ──────────────────────────────────────────────────────────

fn fetch_node_versions() -> Result<Vec<String>> {
    let url = "https://nodejs.org/dist/index.json";
    let response = ureq::get(url)
        .call()
        .context("Failed to fetch Node.js version list")?;

    let mut body = response.into_body();
    let mut reader = body.as_reader();
    let mut raw = String::new();
    reader
        .read_to_string(&mut raw)
        .context("Failed to read Node.js version response")?;

    let entries: Vec<serde_json::Value> = serde_json::from_str(&raw)
        .context("Failed to parse Node.js version JSON")?;

    let mut versions: Vec<String> = Vec::new();
    for entry in entries {
        let ver_str = match entry.get("version").and_then(|v| v.as_str()) {
            Some(v) => v,
            None => continue,
        };
        let stripped = ver_str.strip_prefix('v').unwrap_or(ver_str);
        if !is_node_valid_version(stripped) {
            continue;
        }
        if !versions.iter().any(|v| v == stripped) {
            versions.push(stripped.to_string());
        }
    }

    versions.sort_by(|a, b| {
        let a_parts: Vec<u32> = a.split('.').filter_map(|p| p.parse().ok()).collect();
        let b_parts: Vec<u32> = b.split('.').filter_map(|p| p.parse().ok()).collect();
        b_parts.cmp(&a_parts)
    });

    Ok(versions)
}

fn is_node_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

// ── Cloudflare / cloudflared ─────────────────────────────────────────

fn cloudflare_source(version: &str) -> Option<PackageSource> {
    let base = "https://github.com/cloudflare/cloudflared/releases/latest/download";

    let (platform_key, filename, archive_type) = platform_cloudflare_archive()?;

    let url = format!("{base}/{filename}");
    let mut urls = std::collections::HashMap::new();
    urls.insert(platform_key.to_string(), url);

    Some(PackageSource {
        name: "cloudflare".to_string(),
        version: version.to_string(),
        urls,
        sha256: std::collections::HashMap::new(),
        archive_type,
    })
}

fn platform_cloudflare_archive() -> Option<(&'static str, String, ArchiveType)> {
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    {
        Some(("windows-x86_64", "cloudflared-windows-amd64.exe".to_string(), ArchiveType::Zip))
    }

    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        Some(("linux-x86_64", "cloudflared-linux-amd64".to_string(), ArchiveType::TarGz))
    }

    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    {
        Some(("linux-aarch64", "cloudflared-linux-arm64".to_string(), ArchiveType::TarGz))
    }

    #[cfg(all(target_os = "macos", any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        Some(("macos-universal", "cloudflared-darwin-amd64.tgz".to_string(), ArchiveType::TarGz))
    }

    #[cfg(not(any(
        all(target_os = "windows", target_arch = "x86_64"),
        all(target_os = "linux", any(target_arch = "x86_64", target_arch = "aarch64")),
        all(target_os = "macos", any(target_arch = "x86_64", target_arch = "aarch64")),
    )))]
    {
        None
    }
}

fn node_source(version: &str) -> Option<PackageSource> {
    let base = format!("https://nodejs.org/dist/v{version}/");

    let entry = platform_node_archive(version)?;
    let url = format!("{base}{}", entry.1);

    let mut urls = HashMap::new();
    urls.insert(entry.0.to_string(), url);
    urls.insert("linux-x86_64".to_string(), format!("{}node-v{version}-linux-x64.tar.gz", base));
    urls.insert("linux-aarch64".to_string(), format!("{}node-v{version}-linux-arm64.tar.gz", base));
    urls.insert("macos-x86_64".to_string(), format!("{}node-v{version}-darwin-x64.tar.gz", base));
    urls.insert("macos-aarch64".to_string(), format!("{}node-v{version}-darwin-arm64.tar.gz", base));
    urls.insert("windows-x86_64".to_string(), format!("{}node-v{version}-win-x64.zip", base));
    urls.insert("windows-aarch64".to_string(), format!("{}node-v{version}-win-arm64.zip", base));

    Some(PackageSource {
        name: "node".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type: entry.2,
    })
}

fn platform_node_archive(version: &str) -> Option<(&'static str, String, ArchiveType)> {
    #[cfg(all(any(target_os = "linux", target_os = "android"), target_arch = "x86_64"))]
    {
        let f = format!("node-v{version}-linux-x64.tar.gz");
        Some(("linux-x86_64", f, ArchiveType::TarGz))
    }

    #[cfg(all(any(target_os = "linux", target_os = "android"), target_arch = "aarch64"))]
    {
        let f = format!("node-v{version}-linux-arm64.tar.gz");
        Some(("linux-aarch64", f, ArchiveType::TarGz))
    }

    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    {
        let f = format!("node-v{version}-darwin-x64.tar.gz");
        Some(("macos-x86_64", f, ArchiveType::TarGz))
    }

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        let f = format!("node-v{version}-darwin-arm64.tar.gz");
        Some(("macos-aarch64", f, ArchiveType::TarGz))
    }

    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    {
        let f = format!("node-v{version}-win-x64.zip");
        Some(("windows-x86_64", f, ArchiveType::Zip))
    }

    #[cfg(all(target_os = "windows", target_arch = "aarch64"))]
    {
        let f = format!("node-v{version}-win-arm64.zip");
        Some(("windows-aarch64", f, ArchiveType::Zip))
    }

    #[cfg(not(any(
        all(any(target_os = "linux", target_os = "android"), any(target_arch = "x86_64", target_arch = "aarch64")),
        all(target_os = "macos", any(target_arch = "x86_64", target_arch = "aarch64")),
        all(target_os = "windows", any(target_arch = "x86_64", target_arch = "aarch64")),
    )))]
    {
        None
    }
}
