use std::collections::HashMap;
use std::io::Read;
use std::sync::Mutex;

use anyhow::{Context, Result};

use super::manager;
use super::types::{ArchiveType, PackageSource};
use crate::t;

/// In-memory cache: runtime name -> sorted list of available versions
static VERSION_CACHE: Mutex<Option<HashMap<String, Vec<String>>>> = Mutex::new(None);

/// Caches full git tag for a given version (e.g., "2.45.0" -> "v2.45.0.windows.1")
static GIT_TAG_CACHE: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);

/// Caches Java release data keyed by feature version (e.g., "21" -> release JSON array)
static JAVA_RELEASE_CACHE: Mutex<Option<HashMap<String, Vec<serde_json::Value>>>> =
    Mutex::new(None);

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
        "python" => fetch_python_versions()?,
        "node" => fetch_node_versions()?,
        "cloudflare" => vec!["latest".to_string()],
        "git" => fetch_git_versions()?,
        "gh" => fetch_gh_versions()?,
        "java" => fetch_java_versions()?,
        "kotlin" => fetch_kotlin_versions()?,
        "swift" => fetch_swift_versions()?,
        "php" => fetch_php_versions()?,
        "zig" => fetch_zig_versions()?,
        "go" => fetch_go_versions()?,
        "rust" => fetch_rust_versions()?,
        "msys2" => fetch_msys2_versions()?,
        "ruby" => fetch_ruby_versions()?,
        "lua" => fetch_lua_versions()?,
        "nim" => fetch_nim_versions()?,
        "dotnet" => fetch_dotnet_versions()?,
        "flutter" => fetch_flutter_versions()?,
        "kubectl" => fetch_kubectl_versions()?,
        "helm" => fetch_helm_versions()?,
        "kind" => fetch_kind_versions()?,
        "kustomize" => fetch_kustomize_versions()?,
        "docker" => fetch_docker_versions()?,
        "docker-compose" => fetch_docker_compose_versions()?,
        "act" => fetch_act_versions()?,
        "devops-tool" => vec!["latest".to_string()],
        "scala" => fetch_scala_versions()?,
        "opencode" => vec!["latest".to_string()],
        "mimocode" => vec!["latest".to_string()],
        "kilocode" => vec!["latest".to_string()],
        "omniroute" => vec!["latest".to_string()],
        "vscode" => vec!["latest".to_string()],
        "sqlite" => fetch_sqlite_versions()?,
        "sqlitestudio" => fetch_sqlitestudio_versions()?,
        "ffmpeg" => vec!["latest".to_string()],
        "imagemagick" => vec!["latest".to_string()],
        "tesseract" => vec!["latest".to_string()],
        "freebuff" => vec!["latest".to_string()],
        "nginx" => fetch_nginx_versions()?,
        _ => vec![],
    };

    if let Ok(mut cache) = VERSION_CACHE.lock() {
        cache
            .get_or_insert_with(HashMap::new)
            .insert(name.to_string(), versions.clone());
    }

    Ok(versions)
}

/// Return available sources for a given language + version.
pub fn get_source(name: &str, version: &str) -> Option<PackageSource> {
    match name {
        "python" => python_source(version),
        "node" => node_source(version),
        "cloudflare" => cloudflare_source(version),
        "git" => git_source(version),
        "gh" => gh_source(version),
        "java" => java_source(version),
        "kotlin" => kotlin_source(version),
        "swift" => swift_source(version),
        "php" => php_source(version),
        "zig" => zig_source(version),
        "go" => go_source(version),
        "rust" => rust_source(version),
        "msys2" => msys2_source(version),
        "ruby" => ruby_source(version),
        "lua" => lua_source(version),
        "nim" => nim_source(version),
        "dotnet" => dotnet_source(version),
        "flutter" => flutter_source(version),
        "kubectl" => kubectl_source(version),
        "helm" => helm_source(version),
        "kind" => kind_source(version),
        "kustomize" => kustomize_source(version),
        "docker" => docker_source(version),
        "docker-compose" => docker_compose_source(version),
        "act" => act_source(version),
        "devops-tool" => Some(PackageSource {
            name: "devops-tool".to_string(),
            version: version.to_string(),
            urls: HashMap::new(),
            sha256: HashMap::new(),
            archive_type: ArchiveType::Zip,
        }),
        "scala" => scala_source(version),
        "sqlite" => sqlite_source(version),
        "sqlitestudio" => sqlitestudio_source(version),
        "ffmpeg" => ffmpeg_source(version),
        "imagemagick" => imagemagick_source(version),
        "tesseract" => tesseract_source(version),
        "nginx" => nginx_source(version),
        "opencode" | "mimocode" | "kilocode" | "omniroute" | "vscode" | "freebuff" => Some(PackageSource {
            name: name.to_string(),
            version: version.to_string(),
            urls: HashMap::new(),
            sha256: HashMap::new(),
            archive_type: ArchiveType::Zip,
        }),
        _ => None,
    }
}

/// List all supported languages.
pub fn list_supported_languages() -> Vec<&'static str> {
    vec![
        "python",
        "node",
        "cloudflare",
        "git",
        "gh",
        "java",
        "kotlin",
        "swift",
        "php",
        "zig",
        "go",
        "rust",
        "msys2",
        "ruby",
        "lua",
        "nim",
        "dotnet",
        "flutter",
        "kubectl",
        "helm",
        "kind",
        "kustomize",
        "docker",
        "docker-compose",
        "act",
        "devops-tool",
        "scala",
        "opencode",
        "mimocode",
        "kilocode",
        "freebuff",
        "omniroute",
        "vscode",
        "sqlite",
        "sqlitestudio",
        "ffmpeg",
        "imagemagick",
        "tesseract",
        "nginx",
    ]
}

/// List all supported packages with short descriptions.
pub fn list_packages_with_descriptions() -> Vec<(&'static str, String)> {
    vec![
        ("python", t!("ntpkg.src_python")),
        ("node", t!("ntpkg.src_node")),
        ("cloudflare", t!("ntpkg.src_cloudflare")),
        ("git", t!("ntpkg.src_git")),
        ("gh", t!("ntpkg.src_gh")),
        ("java", t!("ntpkg.src_java")),
        ("kotlin", t!("ntpkg.src_kotlin")),
        ("swift", t!("ntpkg.src_swift")),
        ("php", t!("ntpkg.src_php")),
        ("zig", t!("ntpkg.src_zig")),
        ("go", t!("ntpkg.src_go")),
        ("rust", t!("ntpkg.src_rust")),
        ("msys2", t!("ntpkg.src_msys2")),
        ("ruby", t!("ntpkg.src_ruby")),
        ("lua", t!("ntpkg.src_lua")),
        ("nim", t!("ntpkg.src_nim")),
        ("dotnet", t!("ntpkg.src_dotnet")),
        ("flutter", t!("ntpkg.src_flutter")),
        ("kubectl", t!("ntpkg.src_kubectl")),
        ("helm", t!("ntpkg.src_helm")),
        ("kind", t!("ntpkg.src_kind")),
        ("kustomize", t!("ntpkg.src_kustomize")),
        ("docker", t!("ntpkg.src_docker")),
        ("docker-compose", t!("ntpkg.src_docker_compose")),
        ("act", t!("ntpkg.src_act")),
        ("devops-tool", t!("ntpkg.src_devops_tool")),
        ("scala", t!("ntpkg.src_scala")),
        ("opencode", t!("ntpkg.src_opencode")),
        ("mimocode", t!("ntpkg.src_mimocode")),
        ("kilocode", t!("ntpkg.src_kilocode")),
        ("freebuff", t!("ntpkg.src_freebuff")),
        ("omniroute", t!("ntpkg.src_omniroute")),
        ("vscode", t!("ntpkg.src_vscode")),
        ("sqlite", t!("ntpkg.src_sqlite")),
        ("sqlitestudio", t!("ntpkg.src_sqlitestudio")),
        ("ffmpeg", t!("ntpkg.src_ffmpeg")),
        ("imagemagick", t!("ntpkg.src_imagemagick")),
        ("tesseract", t!("ntpkg.src_tesseract")),
        ("nginx", t!("ntpkg.src_nginx")),
    ]
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

// ── nginx ────────────────────────────────────────────────────────────

fn fetch_nginx_versions() -> Result<Vec<String>> {
    let url = "https://nginx.org/download/";
    let response = ureq::get(url)
        .call()
        .context("Failed to fetch nginx version list from nginx.org")?;

    let mut body = response.into_body();
    let mut html = String::new();
    body.as_reader()
        .read_to_string(&mut html)
        .context("Failed to read nginx.org download page")?;

    let mut versions: Vec<String> = Vec::new();
    let mut search_start = 0;

    // Windows binaries ship as `nginx-{version}.zip`.
    while let Some(href_start) = html[search_start..].find("nginx-") {
        let start = search_start + href_start + 6;
        if let Some(zip_end_rel) = html[start..].find(".zip") {
            let version_str = &html[start..start + zip_end_rel];
            if is_valid_version(version_str) && !versions.iter().any(|v| v == version_str) {
                versions.push(version_str.to_string());
            }
            search_start = start + zip_end_rel + 4;
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

fn nginx_source(version: &str) -> Option<PackageSource> {
    let base = format!("https://nginx.org/download/nginx-{version}");

    let mut urls = HashMap::new();
    // Windows: official prebuilt zip. ARM64 Windows runs the x64 build under emulation.
    urls.insert(
        "windows-x86_64".to_string(),
        format!("{base}.zip"),
    );
    urls.insert(
        "windows-aarch64".to_string(),
        format!("{base}.zip"),
    );
    // Linux/macOS: official source tarball (built by ntpkg when requested).
    urls.insert("linux-x86_64".to_string(), format!("{base}.tar.gz"));
    urls.insert("linux-aarch64".to_string(), format!("{base}.tar.gz"));
    urls.insert("macos-x86_64".to_string(), format!("{base}.tar.gz"));
    urls.insert("macos-aarch64".to_string(), format!("{base}.tar.gz"));

    let archive_type = platform_nginx_archive();

    Some(PackageSource {
        name: "nginx".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type,
    })
}

fn platform_nginx_archive() -> ArchiveType {
    #[cfg(target_os = "windows")]
    {
        ArchiveType::Zip
    }

    #[cfg(not(target_os = "windows"))]
    {
        ArchiveType::TarGz
    }
}

fn python_source(version: &str) -> Option<PackageSource> {
    let base = format!("https://www.python.org/ftp/python/{version}/");

    let (platform_key, filename, archive_type) = platform_python_archive(version)?;

    let url = format!("{base}{filename}");
    let mut urls = HashMap::new();
    urls.insert(platform_key.to_string(), url.clone());
    // The Linux source tarball is arch-independent — register it for both
    // arch keys so aarch64 Linux doesn't hit "No download URL for platform".
    if platform_key.starts_with("linux-") {
        urls.insert("linux-x86_64".to_string(), url.clone());
        urls.insert("linux-aarch64".to_string(), url);
    }

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
        // ARM64 Windows has its own installer (verified on python.org FTP);
        // x64 emulation works but the native build is the right default.
        if cfg!(target_arch = "aarch64") {
            let f = format!("python-{version}-arm64.exe");
            Some(("windows-aarch64", f, ArchiveType::Msi))
        } else {
            let f = format!("python-{version}-amd64.exe");
            Some(("windows-x86_64", f, ArchiveType::Msi))
        }
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        let f = format!("Python-{version}.tar.xz");
        Some(("linux-x86_64", f, ArchiveType::TarXz))
    }

    #[cfg(target_os = "macos")]
    {
        let f = format!("python-{version}-macos11.pkg");
        Some(("macos-universal", f, ArchiveType::Pkg))
    }

    #[cfg(not(any(
        target_os = "windows",
        target_os = "linux",
        target_os = "android",
        target_os = "macos"
    )))]
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

    let entries: Vec<serde_json::Value> =
        serde_json::from_str(&raw).context("Failed to parse Node.js version JSON")?;

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
        Some((
            "windows-x86_64",
            "cloudflared-windows-amd64.exe".to_string(),
            ArchiveType::Zip,
        ))
    }

    #[cfg(all(
        any(target_os = "linux", target_os = "android"),
        target_arch = "x86_64"
    ))]
    {
        Some((
            "linux-x86_64",
            "cloudflared-linux-amd64".to_string(),
            ArchiveType::TarGz,
        ))
    }

    #[cfg(all(
        any(target_os = "linux", target_os = "android"),
        target_arch = "aarch64"
    ))]
    {
        Some((
            "linux-aarch64",
            "cloudflared-linux-arm64".to_string(),
            ArchiveType::TarGz,
        ))
    }

    #[cfg(all(
        target_os = "macos",
        any(target_arch = "x86_64", target_arch = "aarch64")
    ))]
    {
        Some((
            "macos-universal",
            "cloudflared-darwin-amd64.tgz".to_string(),
            ArchiveType::TarGz,
        ))
    }

    #[cfg(not(any(
        all(target_os = "windows", target_arch = "x86_64"),
        all(
            any(target_os = "linux", target_os = "android"),
            any(target_arch = "x86_64", target_arch = "aarch64")
        ),
        all(
            target_os = "macos",
            any(target_arch = "x86_64", target_arch = "aarch64")
        ),
    )))]
    {
        None
    }
}

// ── Git ───────────────────────────────────────────────────────────────

fn fetch_git_versions() -> Result<Vec<String>> {
    let url = "https://api.github.com/repos/git-for-windows/git/releases?per_page=100";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch Git versions from GitHub")?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read GitHub response body")?;

    let releases: Vec<serde_json::Value> =
        serde_json::from_str(&raw).context("Failed to parse GitHub releases JSON")?;

    let mut versions: Vec<String> = Vec::new();
    let mut tag_map: HashMap<String, String> = HashMap::new();

    for release in releases {
        let tag = release["tag_name"].as_str().unwrap_or("");
        if let Some(ver) = parse_git_tag(tag) {
            if !versions.iter().any(|v| v == &ver) {
                versions.push(ver.clone());
                tag_map.insert(ver, tag.to_string());
            }
        }
    }

    versions.sort_by(|a, b| {
        let a_parts: Vec<u32> = a.split('.').filter_map(|p| p.parse().ok()).collect();
        let b_parts: Vec<u32> = b.split('.').filter_map(|p| p.parse().ok()).collect();
        b_parts.cmp(&a_parts)
    });

    if let Ok(mut cache) = GIT_TAG_CACHE.lock() {
        *cache = Some(tag_map);
    }

    Ok(versions)
}

fn parse_git_tag(tag: &str) -> Option<String> {
    let stripped = tag.strip_prefix('v')?;
    let dot_windows = stripped.find(".windows.")?;
    let ver = &stripped[..dot_windows];
    if is_valid_version(ver) {
        Some(ver.to_string())
    } else {
        None
    }
}

fn git_source(version: &str) -> Option<PackageSource> {
    #[cfg(windows)]
    let full_tag = GIT_TAG_CACHE
        .lock()
        .ok()
        .and_then(|c| c.as_ref()?.get(version).cloned())
        .unwrap_or_else(|| format!("v{}.windows.1", version));

    #[cfg(windows)]
    {
        let url = format!(
            "https://github.com/git-for-windows/git/releases/download/{full_tag}/MinGit-{version}-64-bit.zip"
        );
        let mut urls = HashMap::new();
        urls.insert("windows-x86_64".to_string(), url);
        Some(PackageSource {
            name: "git".to_string(),
            version: version.to_string(),
            urls,
            sha256: HashMap::new(),
            archive_type: ArchiveType::Zip,
        })
    }

    #[cfg(not(windows))]
    {
        let mut urls = HashMap::new();
        urls.insert("unix".to_string(), String::new());
        Some(PackageSource {
            name: "git".to_string(),
            version: version.to_string(),
            urls,
            sha256: HashMap::new(),
            archive_type: ArchiveType::TarGz,
        })
    }
}

// ── GitHub CLI (gh) ────────────────────────────────────────────────────

fn fetch_gh_versions() -> Result<Vec<String>> {
    let url = "https://api.github.com/repos/cli/cli/releases?per_page=100";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch gh versions from GitHub")?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read GitHub response body")?;

    let releases: Vec<serde_json::Value> =
        serde_json::from_str(&raw).context("Failed to parse GitHub releases JSON")?;

    let mut versions: Vec<String> = Vec::new();
    for release in releases {
        let tag = release["tag_name"].as_str().unwrap_or("");
        let stripped = tag.strip_prefix('v').unwrap_or(tag);
        if is_gh_valid_version(stripped) && !versions.iter().any(|v| v == stripped) {
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

fn is_gh_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() < 2 || parts.len() > 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn gh_source(version: &str) -> Option<PackageSource> {
    let base = format!("https://github.com/cli/cli/releases/download/v{version}");

    let mut urls = HashMap::new();
    urls.insert(
        "linux-x86_64".to_string(),
        format!("{base}/gh_{version}_linux_amd64.tar.gz"),
    );
    urls.insert(
        "linux-aarch64".to_string(),
        format!("{base}/gh_{version}_linux_arm64.tar.gz"),
    );
    urls.insert(
        "macos-x86_64".to_string(),
        format!("{base}/gh_{version}_macOS_amd64.zip"),
    );
    urls.insert(
        "macos-aarch64".to_string(),
        format!("{base}/gh_{version}_macOS_arm64.zip"),
    );
    urls.insert(
        "windows-x86_64".to_string(),
        format!("{base}/gh_{version}_windows_amd64.zip"),
    );

    Some(PackageSource {
        name: "gh".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type: ArchiveType::Zip,
    })
}

fn node_source(version: &str) -> Option<PackageSource> {
    let base = format!("https://nodejs.org/dist/v{version}/");

    let entry = platform_node_archive(version)?;
    let url = format!("{base}{}", entry.1);

    let mut urls = HashMap::new();
    urls.insert(entry.0.to_string(), url);
    urls.insert(
        "linux-x86_64".to_string(),
        format!("{}node-v{version}-linux-x64.tar.gz", base),
    );
    urls.insert(
        "linux-aarch64".to_string(),
        format!("{}node-v{version}-linux-arm64.tar.gz", base),
    );
    urls.insert(
        "macos-x86_64".to_string(),
        format!("{}node-v{version}-darwin-x64.tar.gz", base),
    );
    urls.insert(
        "macos-aarch64".to_string(),
        format!("{}node-v{version}-darwin-arm64.tar.gz", base),
    );
    urls.insert(
        "windows-x86_64".to_string(),
        format!("{}node-v{version}-win-x64.zip", base),
    );
    urls.insert(
        "windows-aarch64".to_string(),
        format!("{}node-v{version}-win-arm64.zip", base),
    );

    Some(PackageSource {
        name: "node".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type: entry.2,
    })
}

fn platform_node_archive(version: &str) -> Option<(&'static str, String, ArchiveType)> {
    #[cfg(all(
        any(target_os = "linux", target_os = "android"),
        target_arch = "x86_64"
    ))]
    {
        let f = format!("node-v{version}-linux-x64.tar.gz");
        Some(("linux-x86_64", f, ArchiveType::TarGz))
    }

    #[cfg(all(
        any(target_os = "linux", target_os = "android"),
        target_arch = "aarch64"
    ))]
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
        all(
            any(target_os = "linux", target_os = "android"),
            any(target_arch = "x86_64", target_arch = "aarch64")
        ),
        all(
            target_os = "macos",
            any(target_arch = "x86_64", target_arch = "aarch64")
        ),
        all(
            target_os = "windows",
            any(target_arch = "x86_64", target_arch = "aarch64")
        ),
    )))]
    {
        None
    }
}

// ── Java (Adoptium Temurin) ──────────────────────────────────────────

/// Returns supported Java feature versions (JDK 8, 11, 17, 21, 26).
fn fetch_java_versions() -> Result<Vec<String>> {
    Ok(vec![
        "8".to_string(),
        "11".to_string(),
        "17".to_string(),
        "21".to_string(),
        "26".to_string(),
    ])
}

/// Fetch available minor releases for a Java feature version.
pub(crate) fn list_java_minor_versions(feature: &str) -> Vec<String> {
    let url = format!("https://api.adoptium.net/v3/assets/feature_releases/{feature}/ga");

    let response = match ureq::get(&url).header("User-Agent", "ntc").call() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("ntpkg: failed to fetch Java {feature} versions: {e}");
            return vec![];
        }
    };

    let mut body = response.into_body();
    let mut raw = String::new();
    if body.as_reader().read_to_string(&mut raw).is_err() {
        return vec![];
    }

    let releases: Vec<serde_json::Value> = match serde_json::from_str(&raw) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("ntpkg: failed to parse Java {feature} releases JSON: {e}");
            return vec![];
        }
    };

    let mut versions: Vec<String> = Vec::new();
    for release in &releases {
        if let Some(name) = release["release_name"].as_str() {
            if !versions.iter().any(|v| v == name) {
                versions.push(name.to_string());
            }
        }
    }

    if let Ok(mut cache) = JAVA_RELEASE_CACHE.lock() {
        cache
            .get_or_insert_with(HashMap::new)
            .insert(feature.to_string(), releases);
    }

    versions
}

fn java_source(version: &str) -> Option<PackageSource> {
    let (os_key, arch_key) = adoptium_platform()?;

    // Try cache first
    let found_release = if let Ok(cache) = JAVA_RELEASE_CACHE.lock() {
        if let Some(ref map) = *cache {
            map.values()
                .flatten()
                .find(|r| r["release_name"].as_str() == Some(version))
                .cloned()
        } else {
            None
        }
    } else {
        None
    };

    let release = match found_release {
        Some(r) => r,
        None => {
            // Cache miss: extract feature version and fetch feature releases
            let feature = java_feature_from_version(version)?;
            let url = format!("https://api.adoptium.net/v3/assets/feature_releases/{feature}/ga");
            let response = ureq::get(&url).header("User-Agent", "ntc").call().ok()?;
            let mut body = response.into_body();
            let mut raw = String::new();
            body.as_reader().read_to_string(&mut raw).ok()?;
            let releases: Vec<serde_json::Value> = serde_json::from_str(&raw).ok()?;

            if let Ok(mut cache) = JAVA_RELEASE_CACHE.lock() {
                cache
                    .get_or_insert_with(HashMap::new)
                    .insert(feature.to_string(), releases.clone());
            }

            releases
                .into_iter()
                .find(|r| r["release_name"].as_str() == Some(version))?
        }
    };

    let binaries = release["binaries"].as_array()?;

    for binary in binaries {
        let b_os = binary["os"].as_str()?;
        let b_arch = binary["architecture"].as_str()?;
        let b_type = binary["image_type"].as_str()?;
        if b_os == os_key && b_arch == arch_key && b_type == "jdk" {
            let pkg = &binary["package"];
            let link = pkg["link"].as_str()?;
            let mut urls = HashMap::new();
            urls.insert(manager::platform_key(), link.to_string());

            let archive_type = if link.ends_with(".zip") {
                ArchiveType::Zip
            } else {
                ArchiveType::TarGz
            };

            return Some(PackageSource {
                name: "java".to_string(),
                version: version.to_string(),
                urls,
                sha256: HashMap::new(),
                archive_type,
            });
        }
    }

    None
}

/// Extract feature version from a release name.
/// "jdk-21.0.11+10" -> "21", "jdk8u432-b06" -> "8"
fn java_feature_from_version(version: &str) -> Option<String> {
    let stripped = version
        .strip_prefix("jdk-")
        .or_else(|| version.strip_prefix("jdk"))?;
    if let Some(dot) = stripped.find('.') {
        Some(stripped[..dot].to_string())
    } else if let Some(u) = stripped.find('u') {
        Some(stripped[..u].to_string())
    } else if let Some(plus) = stripped.find('+') {
        Some(stripped[..plus].to_string())
    } else {
        // Bare number like "21"
        Some(stripped.to_string())
    }
}

// ── Kotlin ────────────────────────────────────────────────────────────

fn fetch_kotlin_versions() -> Result<Vec<String>> {
    let url = "https://api.github.com/repos/JetBrains/kotlin/releases?per_page=100";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch Kotlin versions from GitHub")?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read GitHub response body")?;

    let releases: Vec<serde_json::Value> =
        serde_json::from_str(&raw).context("Failed to parse GitHub releases JSON")?;

    let mut versions: Vec<String> = Vec::new();
    for release in releases {
        let tag = release["tag_name"].as_str().unwrap_or("");
        let stripped = tag.strip_prefix('v').unwrap_or(tag);
        if is_kotlin_valid_version(stripped) && !versions.iter().any(|v| v == stripped) {
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

fn is_kotlin_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() < 2 || parts.len() > 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn kotlin_source(version: &str) -> Option<PackageSource> {
    let url = format!(
        "https://github.com/JetBrains/kotlin/releases/download/v{version}/kotlin-compiler-{version}.zip"
    );
    let mut urls = HashMap::new();
    urls.insert("linux-x86_64".to_string(), url.clone());
    urls.insert("linux-aarch64".to_string(), url.clone());
    urls.insert("macos-x86_64".to_string(), url.clone());
    urls.insert("macos-aarch64".to_string(), url.clone());
    urls.insert("windows-x86_64".to_string(), url.clone());
    urls.insert("windows-aarch64".to_string(), url);

    Some(PackageSource {
        name: "kotlin".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type: ArchiveType::Zip,
    })
}

// ── Swift ────────────────────────────────────────────────────────────

fn fetch_swift_versions() -> Result<Vec<String>> {
    let url = "https://api.github.com/repos/apple/swift/releases?per_page=100";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch Swift versions from GitHub")?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read GitHub response body")?;

    let releases: Vec<serde_json::Value> =
        serde_json::from_str(&raw).context("Failed to parse GitHub releases JSON")?;

    let mut versions: Vec<String> = Vec::new();
    for release in releases {
        let tag = release["tag_name"].as_str().unwrap_or("");
        if let Some(ver) = parse_swift_tag(tag) {
            if !versions.iter().any(|v| v == &ver) {
                versions.push(ver);
            }
        }
    }

    versions.sort_by(|a, b| {
        let a_parts: Vec<u32> = a.split('.').filter_map(|p| p.parse().ok()).collect();
        let b_parts: Vec<u32> = b.split('.').filter_map(|p| p.parse().ok()).collect();
        b_parts.cmp(&a_parts)
    });

    Ok(versions)
}

fn parse_swift_tag(tag: &str) -> Option<String> {
    let stripped = tag.strip_prefix("swift-")?;
    let ver = stripped.strip_suffix("-RELEASE")?;
    if is_swift_valid_version(ver) {
        Some(ver.to_string())
    } else {
        None
    }
}

fn is_swift_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() < 2 || parts.len() > 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn swift_source(version: &str) -> Option<PackageSource> {
    let branch = format!("swift-{version}-release");
    let release_dir = format!("swift-{version}-RELEASE");

    let mut urls = HashMap::new();

    // ── Linux: Ubuntu 24.04 (default) ──
    urls.insert(
        "linux-x86_64".to_string(),
        format!(
            "https://download.swift.org/{branch}/ubuntu2404/{release_dir}/{release_dir}-ubuntu24.04.tar.gz"
        ),
    );
    urls.insert(
        "linux-aarch64".to_string(),
        format!(
            "https://download.swift.org/{branch}/ubuntu2404-aarch64/{release_dir}/{release_dir}-ubuntu24.04-aarch64.tar.gz"
        ),
    );

    // ── macOS ──
    urls.insert(
        "macos-x86_64".to_string(),
        format!("https://download.swift.org/{branch}/xcode/{release_dir}/{release_dir}-osx.pkg"),
    );
    urls.insert(
        "macos-aarch64".to_string(),
        format!("https://download.swift.org/{branch}/xcode/{release_dir}/{release_dir}-osx.pkg"),
    );

    // ── Windows ──
    urls.insert(
        "windows-x86_64".to_string(),
        format!(
            "https://download.swift.org/{branch}/windows10/{release_dir}/{release_dir}-windows10.exe"
        ),
    );
    urls.insert(
        "windows-aarch64".to_string(),
        format!(
            "https://download.swift.org/{branch}/windows10-arm64/{release_dir}/{release_dir}-windows10-arm64.exe"
        ),
    );

    Some(PackageSource {
        name: "swift".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type: ArchiveType::TarGz,
    })
}

// ── PHP ──────────────────────────────────────────────────────────────

fn fetch_php_versions() -> Result<Vec<String>> {
    let url = "https://api.github.com/repos/php/php-src/releases?per_page=100";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch PHP versions from GitHub")?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read GitHub response body")?;

    let releases: Vec<serde_json::Value> =
        serde_json::from_str(&raw).context("Failed to parse GitHub releases JSON")?;

    let mut versions: Vec<String> = Vec::new();
    for release in releases {
        let tag = release["tag_name"].as_str().unwrap_or("");
        let stripped = tag.strip_prefix("php-").unwrap_or(tag);
        if is_php_valid_version(stripped) && !versions.iter().any(|v| v == stripped) {
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

fn is_php_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() < 2 || parts.len() > 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn php_source(version: &str) -> Option<PackageSource> {
    let mut urls = HashMap::new();

    // ── Windows: prebuilt zip ──
    let vs = if php_uses_vs17(version) {
        "vs17"
    } else {
        "vs16"
    };
    urls.insert(
        "windows-x86_64".to_string(),
        format!("https://downloads.php.net/~windows/releases/php-{version}-nts-Win32-{vs}-x64.zip"),
    );

    // ── Linux / macOS: source tarball ──
    let source_url = format!("https://www.php.net/distributions/php-{version}.tar.gz");
    urls.insert("linux-x86_64".to_string(), source_url.clone());
    urls.insert("linux-aarch64".to_string(), source_url.clone());
    urls.insert("macos-x86_64".to_string(), source_url.clone());
    urls.insert("macos-aarch64".to_string(), source_url);

    Some(PackageSource {
        name: "php".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type: ArchiveType::TarGz,
    })
}

fn php_uses_vs17(version: &str) -> bool {
    let parts: Vec<u32> = version.split('.').filter_map(|p| p.parse().ok()).collect();
    if parts.len() >= 2 {
        // PHP 8.4+ uses vs17, older versions use vs16
        parts[0] > 8 || (parts[0] == 8 && parts[1] >= 4)
    } else {
        false
    }
}

// ── Zig ─────────────────────────────────────────────────────────────

fn fetch_zig_versions() -> Result<Vec<String>> {
    let url = "https://api.github.com/repos/ziglang/zig/releases?per_page=100";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch Zig versions from GitHub")?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read GitHub response body")?;

    let releases: Vec<serde_json::Value> =
        serde_json::from_str(&raw).context("Failed to parse GitHub releases JSON")?;

    let mut versions: Vec<String> = Vec::new();
    for release in releases {
        let tag = release["tag_name"].as_str().unwrap_or("");
        if is_zig_valid_version(tag) && !versions.iter().any(|v| v == tag) {
            versions.push(tag.to_string());
        }
    }

    versions.sort_by(|a, b| {
        let a_parts: Vec<u32> = a.split('.').filter_map(|p| p.parse().ok()).collect();
        let b_parts: Vec<u32> = b.split('.').filter_map(|p| p.parse().ok()).collect();
        b_parts.cmp(&a_parts)
    });

    Ok(versions)
}

fn is_zig_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() < 2 || parts.len() > 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn zig_source(version: &str) -> Option<PackageSource> {
    let base = format!("https://ziglang.org/download/{version}");

    let mut urls = HashMap::new();

    let platforms = [
        ("linux-x86_64", false),
        ("linux-aarch64", false),
        ("macos-x86_64", false),
        ("macos-aarch64", false),
        ("windows-x86_64", true),
        ("windows-aarch64", true),
    ];

    for (platform, is_zip) in &platforms {
        let ext = if *is_zip { "zip" } else { "tar.xz" };
        let filename = zig_filename(version, platform, ext);
        urls.insert(platform.to_string(), format!("{base}/{filename}"));
    }

    Some(PackageSource {
        name: "zig".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type: ArchiveType::TarXz,
    })
}

fn zig_filename(version: &str, platform: &str, ext: &str) -> String {
    let (os, arch) = platform.split_once('-').unwrap_or((platform, ""));
    let prefix = if uses_new_zig_naming(version) {
        format!("{arch}-{os}")
    } else {
        format!("{os}-{arch}")
    };
    format!("zig-{prefix}-{version}.{ext}")
}

fn uses_new_zig_naming(version: &str) -> bool {
    let parts: Vec<u32> = version.split('.').filter_map(|p| p.parse().ok()).collect();
    if parts.len() >= 2 {
        let major = parts[0];
        let minor = parts[1];
        let patch = parts.get(2).copied().unwrap_or(0);
        (major > 0) || (minor > 14) || (minor == 14 && patch >= 1)
    } else {
        false
    }
}

// ── Go ─────────────────────────────────────────────────────────────

fn fetch_go_versions() -> Result<Vec<String>> {
    let url = "https://go.dev/dl/?mode=json";
    let response = ureq::get(url)
        .call()
        .context("Failed to fetch Go version list from go.dev")?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read go.dev response body")?;

    let releases: Vec<serde_json::Value> =
        serde_json::from_str(&raw).context("Failed to parse go.dev JSON")?;

    let mut versions: Vec<String> = Vec::new();
    for release in releases {
        if release["stable"].as_bool().unwrap_or(false) {
            if let Some(v) = release["version"].as_str() {
                let cleaned = v.strip_prefix("go").unwrap_or(v).to_string();
                if !versions.iter().any(|x| x == &cleaned) {
                    versions.push(cleaned);
                }
            }
        }
    }

    versions.sort_by(|a, b| {
        let a_parts: Vec<u32> = a.split('.').filter_map(|p| p.parse().ok()).collect();
        let b_parts: Vec<u32> = b.split('.').filter_map(|p| p.parse().ok()).collect();
        b_parts.cmp(&a_parts)
    });

    Ok(versions)
}

fn go_source(version: &str) -> Option<PackageSource> {
    let base = format!("https://go.dev/dl/go{version}");

    let mut urls = HashMap::new();
    urls.insert(
        "linux-x86_64".to_string(),
        format!("{base}.linux-amd64.tar.gz"),
    );
    urls.insert(
        "linux-aarch64".to_string(),
        format!("{base}.linux-arm64.tar.gz"),
    );
    urls.insert(
        "macos-x86_64".to_string(),
        format!("{base}.darwin-amd64.tar.gz"),
    );
    urls.insert(
        "macos-aarch64".to_string(),
        format!("{base}.darwin-arm64.tar.gz"),
    );
    urls.insert(
        "windows-x86_64".to_string(),
        format!("{base}.windows-amd64.zip"),
    );
    urls.insert(
        "windows-aarch64".to_string(),
        format!("{base}.windows-arm64.zip"),
    );

    let archive_type = platform_go_archive();

    Some(PackageSource {
        name: "go".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type,
    })
}

fn platform_go_archive() -> ArchiveType {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        ArchiveType::TarGz
    }

    #[cfg(target_os = "windows")]
    {
        ArchiveType::Zip
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        ArchiveType::TarGz
    }
}

// ── Rust ─────────────────────────────────────────────────────────

fn fetch_rust_versions() -> Result<Vec<String>> {
    let url = "https://api.github.com/repos/rust-lang/rust/tags?per_page=100";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch Rust versions from GitHub")?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read GitHub response body")?;

    let tags: Vec<serde_json::Value> =
        serde_json::from_str(&raw).context("Failed to parse GitHub tags JSON")?;

    let mut versions: Vec<String> = Vec::new();
    for tag in tags {
        if let Some(name) = tag["name"].as_str() {
            if is_rust_version(name) && !versions.iter().any(|v| v == name) {
                versions.push(name.to_string());
            }
        }
    }

    versions.sort_by(|a, b| {
        let a_parts: Vec<u32> = a.split('.').filter_map(|p| p.parse().ok()).collect();
        let b_parts: Vec<u32> = b.split('.').filter_map(|p| p.parse().ok()).collect();
        b_parts.cmp(&a_parts)
    });

    Ok(versions)
}

fn is_rust_version(s: &str) -> bool {
    if s.is_empty() || s.starts_with('0') {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn rust_source(version: &str) -> Option<PackageSource> {
    let manifest_url = format!("https://static.rust-lang.org/dist/channel-rust-{version}.toml");
    let response = ureq::get(&manifest_url).call().ok()?;
    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader().read_to_string(&mut raw).ok()?;

    let manifest: toml::Value = toml::from_str(&raw).ok()?;
    let date = manifest.get("date")?.as_str()?;

    let base = format!("https://static.rust-lang.org/dist/{date}");

    let target_map: [(&str, &str); 6] = [
        ("linux-x86_64", "x86_64-unknown-linux-gnu"),
        ("linux-aarch64", "aarch64-unknown-linux-gnu"),
        ("macos-x86_64", "x86_64-apple-darwin"),
        ("macos-aarch64", "aarch64-apple-darwin"),
        ("windows-x86_64", "x86_64-pc-windows-msvc"),
        ("windows-aarch64", "aarch64-pc-windows-msvc"),
    ];

    let mut urls = HashMap::new();
    for (key, target) in &target_map {
        let url = format!("{base}/rust-{version}-{target}.tar.gz");
        urls.insert(key.to_string(), url);
    }

    Some(PackageSource {
        name: "rust".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type: ArchiveType::TarGz,
    })
}

// ── MSYS2 ───────────────────────────────────────────────────────

fn fetch_msys2_versions() -> Result<Vec<String>> {
    let url = "https://api.github.com/repos/msys2/msys2-installer/releases?per_page=30";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch MSYS2 releases from GitHub")?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read GitHub response")?;

    let releases: Vec<serde_json::Value> =
        serde_json::from_str(&raw).context("Failed to parse MSYS2 releases JSON")?;

    let mut versions: Vec<String> = Vec::new();
    for release in releases {
        if let Some(tag) = release["tag_name"].as_str() {
            if !tag.contains("nightly") && !versions.iter().any(|v| v == tag) {
                versions.push(tag.to_string());
            }
        }
    }

    versions.sort_by(|a, b| b.cmp(a));
    let mut all = vec!["latest".to_string()];
    all.extend(versions);
    Ok(all)
}

fn msys2_source(version: &str) -> Option<PackageSource> {
    // For "latest", fetch the actual tag from the latest release
    let tag = if version == "latest" {
        let url = "https://api.github.com/repos/msys2/msys2-installer/releases/latest";
        let resp = ureq::get(url).header("User-Agent", "ntc").call().ok()?;
        let mut body = resp.into_body();
        let mut raw = String::new();
        body.as_reader().read_to_string(&mut raw).ok()?;
        let v: serde_json::Value = serde_json::from_str(&raw).ok()?;
        v["tag_name"].as_str()?.to_string()
    } else {
        version.to_string()
    };

    let date_str = tag.replace('-', "");
    let url = format!(
        "https://github.com/msys2/msys2-installer/releases/download/{tag}/msys2-x86_64-{date_str}.exe"
    );

    let mut urls = HashMap::new();
    urls.insert("windows-x86_64".to_string(), url);

    Some(PackageSource {
        name: "msys2".to_string(),
        version: tag,
        urls,
        sha256: HashMap::new(),
        archive_type: ArchiveType::Msi,
    })
}

// ── Ruby ─────────────────────────────────────────────────────────────

fn fetch_ruby_versions() -> Result<Vec<String>> {
    let url = "https://ftp.ruby-lang.org/pub/ruby/index.txt";
    let response = ureq::get(url)
        .call()
        .context("Failed to fetch Ruby version list from ruby-lang.org")?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read ruby-lang.org index.txt")?;

    let mut versions: Vec<String> = Vec::new();
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let name = trimmed.split('\t').next().unwrap_or("");
        let ver = name.strip_prefix("ruby-").unwrap_or("");
        if is_valid_version(ver) && !versions.iter().any(|v| v == ver) {
            versions.push(ver.to_string());
        }
    }

    versions.sort_by(|a, b| {
        let a_parts: Vec<u32> = a.split('.').filter_map(|p| p.parse().ok()).collect();
        let b_parts: Vec<u32> = b.split('.').filter_map(|p| p.parse().ok()).collect();
        b_parts.cmp(&a_parts)
    });

    Ok(versions)
}

fn ruby_source(version: &str) -> Option<PackageSource> {
    let short = ruby_short_version(version);
    let url = format!("https://cache.ruby-lang.org/pub/ruby/{short}/ruby-{version}.tar.gz");

    let mut urls = HashMap::new();
    urls.insert("linux-x86_64".to_string(), url.clone());
    urls.insert("linux-aarch64".to_string(), url.clone());
    urls.insert("macos-x86_64".to_string(), url.clone());
    urls.insert("macos-aarch64".to_string(), url.clone());

    #[cfg(windows)]
    {
        let win_url = format!(
            "https://github.com/oneclick/rubyinstaller2/releases/download/RubyInstaller-{version}-1/rubyinstaller-{version}-1-x64.exe"
        );
        urls.insert("windows-x86_64".to_string(), win_url);
    }

    Some(PackageSource {
        name: "ruby".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type: ArchiveType::TarGz,
    })
}

fn ruby_short_version(version: &str) -> String {
    let parts: Vec<&str> = version.splitn(3, '.').collect();
    format!("{}.{}", parts[0], parts[1])
}

// ── Lua ──────────────────────────────────────────────────────────────

fn fetch_lua_versions() -> Result<Vec<String>> {
    let url = "https://www.lua.org/ftp/";
    let response = ureq::get(url)
        .call()
        .context("Failed to fetch Lua version list from lua.org")?;

    let mut body = response.into_body();
    let mut html = String::new();
    body.as_reader()
        .read_to_string(&mut html)
        .context("Failed to read lua.org response body")?;

    let html_lower = html.to_lowercase();

    let mut versions: Vec<String> = Vec::new();
    let mut search_start = 0;

    while let Some(href_start) = html_lower[search_start..].find("href=\"") {
        let start = search_start + href_start + 6;
        if let Some(quote_end) = html_lower[start..].find('"') {
            let url_target = &html_lower[start..start + quote_end];

            if let Some(rest) = url_target.strip_prefix("lua-") {
                if let Some(ver) = rest.strip_suffix(".tar.gz") {
                    if is_valid_version(ver) && !versions.iter().any(|v| v == ver) {
                        versions.push(ver.to_string());
                    }
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

fn lua_source(version: &str) -> Option<PackageSource> {
    let url = format!("https://www.lua.org/ftp/lua-{version}.tar.gz");

    let mut urls = HashMap::new();
    urls.insert("linux-x86_64".to_string(), url.clone());
    urls.insert("linux-aarch64".to_string(), url.clone());
    urls.insert("macos-x86_64".to_string(), url.clone());
    urls.insert("macos-aarch64".to_string(), url.clone());
    urls.insert("windows-x86_64".to_string(), url);

    Some(PackageSource {
        name: "lua".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type: ArchiveType::TarGz,
    })
}

// ── Nim ──────────────────────────────────────────────────────────────

fn fetch_nim_versions() -> Result<Vec<String>> {
    let url = "https://api.github.com/repos/nim-lang/Nim/tags?per_page=100";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch Nim versions from GitHub")?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read GitHub response body")?;

    let tags: Vec<serde_json::Value> =
        serde_json::from_str(&raw).context("Failed to parse GitHub tags JSON")?;

    let mut versions: Vec<String> = Vec::new();
    for tag_entry in tags {
        let tag = tag_entry["name"].as_str().unwrap_or("");
        let stripped = tag.strip_prefix('v').unwrap_or(tag);
        if is_nim_valid_version(stripped) && !versions.iter().any(|v| v == stripped) {
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

fn is_nim_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn nim_source(version: &str) -> Option<PackageSource> {
    let base = format!("https://nim-lang.org/download/nim-{version}");

    let mut urls = HashMap::new();
    urls.insert(
        "linux-x86_64".to_string(),
        format!("{base}-linux_x64.tar.xz"),
    );
    urls.insert(
        "linux-aarch64".to_string(),
        format!("{base}-linux_arm64.tar.xz"),
    );
    urls.insert(
        "macos-x86_64".to_string(),
        format!("{base}-macosx_x64.tar.xz"),
    );
    urls.insert(
        "macos-aarch64".to_string(),
        format!("{base}-macosx_arm64.tar.xz"),
    );
    urls.insert("windows-x86_64".to_string(), format!("{base}_x64.zip"));

    let archive_type = platform_nim_archive();

    Some(PackageSource {
        name: "nim".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type,
    })
}

fn platform_nim_archive() -> ArchiveType {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        ArchiveType::TarXz
    }

    #[cfg(target_os = "windows")]
    {
        ArchiveType::Zip
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        ArchiveType::TarXz
    }
}

// ── .NET SDK ─────────────────────────────────────────────────────────

fn fetch_dotnet_versions() -> Result<Vec<String>> {
    let url = "https://api.github.com/repos/dotnet/sdk/releases?per_page=100";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch .NET SDK versions from GitHub")?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read GitHub response body")?;

    let releases: Vec<serde_json::Value> =
        serde_json::from_str(&raw).context("Failed to parse GitHub releases JSON")?;

    let mut versions: Vec<String> = Vec::new();
    for release in releases {
        let tag = release["tag_name"].as_str().unwrap_or("");
        let stripped = tag.strip_prefix('v').unwrap_or(tag);
        if is_dotnet_valid_version(stripped) && !versions.iter().any(|v| v == stripped) {
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

fn is_dotnet_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn dotnet_source(version: &str) -> Option<PackageSource> {
    let base =
        format!("https://builds.dotnet.microsoft.com/dotnet/Sdk/{version}/dotnet-sdk-{version}");

    let mut urls = HashMap::new();
    urls.insert(
        "linux-x86_64".to_string(),
        format!("{base}-linux-x64.tar.gz"),
    );
    urls.insert(
        "linux-aarch64".to_string(),
        format!("{base}-linux-arm64.tar.gz"),
    );
    urls.insert("macos-x86_64".to_string(), format!("{base}-osx-x64.tar.gz"));
    urls.insert(
        "macos-aarch64".to_string(),
        format!("{base}-osx-arm64.tar.gz"),
    );
    urls.insert("windows-x86_64".to_string(), format!("{base}-win-x64.zip"));
    urls.insert(
        "windows-aarch64".to_string(),
        format!("{base}-win-arm64.zip"),
    );

    let archive_type = platform_dotnet_archive();

    Some(PackageSource {
        name: "dotnet".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type,
    })
}

fn platform_dotnet_archive() -> ArchiveType {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        ArchiveType::TarGz
    }

    #[cfg(target_os = "windows")]
    {
        ArchiveType::Zip
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        ArchiveType::TarGz
    }
}

// ── Flutter ──────────────────────────────────────────────────────────

fn fetch_flutter_versions() -> Result<Vec<String>> {
    let url = "https://storage.googleapis.com/flutter_infra_release/releases/releases_linux.json";
    let response = ureq::get(url)
        .call()
        .context("Failed to fetch Flutter versions from Google Storage")?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read Flutter releases JSON")?;

    let releases: serde_json::Value =
        serde_json::from_str(&raw).context("Failed to parse Flutter releases JSON")?;

    let mut versions: Vec<String> = Vec::new();
    if let Some(arr) = releases["releases"].as_array() {
        for entry in arr {
            if let Some(ver) = entry["version"].as_str() {
                if is_flutter_valid_version(ver) && !versions.iter().any(|v| v == ver) {
                    versions.push(ver.to_string());
                }
            }
        }
    }

    versions.sort_by(|a, b| {
        let a_parts: Vec<u32> = a.split('.').filter_map(|p| p.parse().ok()).collect();
        let b_parts: Vec<u32> = b.split('.').filter_map(|p| p.parse().ok()).collect();
        b_parts.cmp(&a_parts)
    });

    Ok(versions)
}

fn is_flutter_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn flutter_source(version: &str) -> Option<PackageSource> {
    let mut urls = HashMap::new();
    urls.insert(
        "windows-x86_64".to_string(),
        format!("https://storage.googleapis.com/flutter_infra_release/releases/stable/windows/flutter_windows_{version}-stable.zip"),
    );
    urls.insert(
        "linux-x86_64".to_string(),
        format!("https://storage.googleapis.com/flutter_infra_release/releases/stable/linux/flutter_linux_{version}-stable.tar.xz"),
    );
    let mac_url = format!("https://storage.googleapis.com/flutter_infra_release/releases/stable/macos/flutter_macos_{version}-stable.zip");
    urls.insert("macos-x86_64".to_string(), mac_url.clone());
    urls.insert("macos-aarch64".to_string(), mac_url);

    let archive_type = platform_flutter_archive();

    Some(PackageSource {
        name: "flutter".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type,
    })
}

fn platform_flutter_archive() -> ArchiveType {
    #[cfg(any(target_os = "linux"))]
    {
        ArchiveType::TarXz
    }

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    {
        ArchiveType::Zip
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        ArchiveType::Zip
    }
}

fn adoptium_platform() -> Option<(&'static str, &'static str)> {
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    {
        Some(("windows", "x64"))
    }

    #[cfg(all(
        any(target_os = "linux", target_os = "android"),
        target_arch = "x86_64"
    ))]
    {
        Some(("linux", "x64"))
    }

    #[cfg(all(
        any(target_os = "linux", target_os = "android"),
        target_arch = "aarch64"
    ))]
    {
        Some(("linux", "aarch64"))
    }

    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    {
        Some(("mac", "x64"))
    }

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        Some(("mac", "aarch64"))
    }

    #[cfg(not(any(
        all(target_os = "windows", target_arch = "x86_64"),
        all(
            any(target_os = "linux", target_os = "android"),
            any(target_arch = "x86_64", target_arch = "aarch64")
        ),
        all(
            target_os = "macos",
            any(target_arch = "x86_64", target_arch = "aarch64")
        ),
    )))]
    {
        None
    }
}

// ── kubectl ─────────────────────────────────────────────────────────────

fn fetch_kubectl_versions() -> Result<Vec<String>> {
    let url = "https://api.github.com/repos/kubernetes/kubernetes/releases?per_page=100";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch kubectl versions from GitHub")?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read GitHub response body")?;

    let releases: Vec<serde_json::Value> =
        serde_json::from_str(&raw).context("Failed to parse GitHub releases JSON")?;

    let mut versions: Vec<String> = Vec::new();
    for release in releases {
        let tag = release["tag_name"].as_str().unwrap_or("");
        let stripped = tag.strip_prefix('v').unwrap_or(tag);
        if is_kubectl_valid_version(stripped) && !versions.iter().any(|v| v == stripped) {
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

fn is_kubectl_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    // kubernetes tags look like "1.32.0" (3 parts) or "1.32.0-alpha.1" etc
    // we only take stable 3-part semver
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn kubectl_source(version: &str) -> Option<PackageSource> {
    let base = format!("https://dl.k8s.io/release/v{version}");

    let mut urls = HashMap::new();
    urls.insert(
        "linux-x86_64".to_string(),
        format!("{base}/bin/linux/amd64/kubectl"),
    );
    urls.insert(
        "linux-aarch64".to_string(),
        format!("{base}/bin/linux/arm64/kubectl"),
    );
    urls.insert(
        "macos-x86_64".to_string(),
        format!("{base}/bin/darwin/amd64/kubectl"),
    );
    urls.insert(
        "macos-aarch64".to_string(),
        format!("{base}/bin/darwin/arm64/kubectl"),
    );
    urls.insert(
        "windows-x86_64".to_string(),
        format!("{base}/bin/windows/amd64/kubectl.exe"),
    );

    Some(PackageSource {
        name: "kubectl".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type: ArchiveType::Zip,
    })
}

// ── Helm ────────────────────────────────────────────────────────────────

fn fetch_helm_versions() -> Result<Vec<String>> {
    let url = "https://api.github.com/repos/helm/helm/releases?per_page=100";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch Helm versions from GitHub")?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read GitHub response body")?;

    let releases: Vec<serde_json::Value> =
        serde_json::from_str(&raw).context("Failed to parse GitHub releases JSON")?;

    let mut versions: Vec<String> = Vec::new();
    for release in releases {
        let tag = release["tag_name"].as_str().unwrap_or("");
        let stripped = tag.strip_prefix('v').unwrap_or(tag);
        if is_helm_valid_version(stripped) && !versions.iter().any(|v| v == stripped) {
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

fn is_helm_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn helm_source(version: &str) -> Option<PackageSource> {
    let base = format!("https://get.helm.sh/helm-v{version}");

    let mut urls = HashMap::new();
    urls.insert(
        "linux-x86_64".to_string(),
        format!("{base}-linux-amd64.tar.gz"),
    );
    urls.insert(
        "linux-aarch64".to_string(),
        format!("{base}-linux-arm64.tar.gz"),
    );
    urls.insert(
        "macos-x86_64".to_string(),
        format!("{base}-darwin-amd64.tar.gz"),
    );
    urls.insert(
        "macos-aarch64".to_string(),
        format!("{base}-darwin-arm64.tar.gz"),
    );
    urls.insert(
        "windows-x86_64".to_string(),
        format!("{base}-windows-amd64.zip"),
    );

    Some(PackageSource {
        name: "helm".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type: ArchiveType::TarGz,
    })
}

// ── kind ────────────────────────────────────────────────────────────────

fn fetch_kind_versions() -> Result<Vec<String>> {
    let url = "https://api.github.com/repos/kubernetes-sigs/kind/releases?per_page=100";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch kind versions from GitHub")?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read GitHub response body")?;

    let releases: Vec<serde_json::Value> =
        serde_json::from_str(&raw).context("Failed to parse GitHub releases JSON")?;

    let mut versions: Vec<String> = Vec::new();
    for release in releases {
        let tag = release["tag_name"].as_str().unwrap_or("");
        let stripped = tag.strip_prefix('v').unwrap_or(tag);
        if is_kind_valid_version(stripped) && !versions.iter().any(|v| v == stripped) {
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

fn is_kind_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn kind_source(version: &str) -> Option<PackageSource> {
    let base = format!("https://kind.sigs.k8s.io/dl/v{version}");

    let mut urls = HashMap::new();
    urls.insert(
        "linux-x86_64".to_string(),
        format!("{base}/kind-linux-amd64"),
    );
    urls.insert(
        "linux-aarch64".to_string(),
        format!("{base}/kind-linux-arm64"),
    );
    urls.insert(
        "macos-x86_64".to_string(),
        format!("{base}/kind-darwin-amd64"),
    );
    urls.insert(
        "macos-aarch64".to_string(),
        format!("{base}/kind-darwin-arm64"),
    );
    urls.insert(
        "windows-x86_64".to_string(),
        format!("{base}/kind-windows-amd64"),
    );

    Some(PackageSource {
        name: "kind".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type: ArchiveType::Zip,
    })
}

// ── kustomize ───────────────────────────────────────────────────────────

fn fetch_kustomize_versions() -> Result<Vec<String>> {
    let url = "https://api.github.com/repos/kubernetes-sigs/kustomize/releases?per_page=100";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch kustomize versions from GitHub")?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read GitHub response body")?;

    let releases: Vec<serde_json::Value> =
        serde_json::from_str(&raw).context("Failed to parse GitHub releases JSON")?;

    let mut versions: Vec<String> = Vec::new();
    for release in releases {
        let tag = release["tag_name"].as_str().unwrap_or("");
        if let Some(stripped) = tag.strip_prefix("kustomize/v") {
            if is_kustomize_valid_version(stripped) && !versions.iter().any(|v| v == stripped) {
                versions.push(stripped.to_string());
            }
        }
    }

    versions.sort_by(|a, b| {
        let a_parts: Vec<u32> = a.split('.').filter_map(|p| p.parse().ok()).collect();
        let b_parts: Vec<u32> = b.split('.').filter_map(|p| p.parse().ok()).collect();
        b_parts.cmp(&a_parts)
    });

    Ok(versions)
}

fn is_kustomize_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn kustomize_source(version: &str) -> Option<PackageSource> {
    let base = format!(
        "https://github.com/kubernetes-sigs/kustomize/releases/download/kustomize/v{version}"
    );

    let mut urls = HashMap::new();
    urls.insert(
        "linux-x86_64".to_string(),
        format!("{base}/kustomize_v{version}_linux_amd64.tar.gz"),
    );
    urls.insert(
        "linux-aarch64".to_string(),
        format!("{base}/kustomize_v{version}_linux_arm64.tar.gz"),
    );
    urls.insert(
        "macos-x86_64".to_string(),
        format!("{base}/kustomize_v{version}_darwin_amd64.tar.gz"),
    );
    urls.insert(
        "macos-aarch64".to_string(),
        format!("{base}/kustomize_v{version}_darwin_arm64.tar.gz"),
    );
    urls.insert(
        "windows-x86_64".to_string(),
        format!("{base}/kustomize_v{version}_windows_amd64.zip"),
    );

    Some(PackageSource {
        name: "kustomize".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type: ArchiveType::Zip,
    })
}

// ── Docker CLI ──────────────────────────────────────────────────────────

fn fetch_docker_versions() -> Result<Vec<String>> {
    let url = "https://download.docker.com/linux/static/stable/x86_64/";
    let response = ureq::get(url)
        .call()
        .context("Failed to fetch Docker versions from download.docker.com")?;

    let mut body = response.into_body();
    let mut html = String::new();
    body.as_reader()
        .read_to_string(&mut html)
        .context("Failed to read download.docker.com response body")?;

    let mut versions: Vec<String> = Vec::new();
    let mut search_start = 0;

    while let Some(href_start) = html[search_start..].find("<a href=\"") {
        let start = search_start + href_start + 9;
        if let Some(quote_end) = html[start..].find('"') {
            let url_target = &html[start..start + quote_end];

            if let Some(rest) = url_target.strip_prefix("docker-") {
                if let Some(ver) = rest.strip_suffix(".tgz") {
                    if is_docker_valid_version(ver) && !versions.iter().any(|v| v == ver) {
                        versions.push(ver.to_string());
                    }
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

fn is_docker_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    // Accept semver like 27.0.0 or 24.0.8
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn docker_source(version: &str) -> Option<PackageSource> {
    let mut urls = HashMap::new();

    urls.insert(
        "linux-x86_64".to_string(),
        format!("https://download.docker.com/linux/static/stable/x86_64/docker-{version}.tgz"),
    );
    urls.insert(
        "linux-aarch64".to_string(),
        format!("https://download.docker.com/linux/static/stable/aarch64/docker-{version}.tgz"),
    );
    urls.insert(
        "macos-x86_64".to_string(),
        format!("https://download.docker.com/mac/static/stable/x86_64/docker-{version}.tgz"),
    );
    urls.insert(
        "macos-aarch64".to_string(),
        format!("https://download.docker.com/mac/static/stable/aarch64/docker-{version}.tgz"),
    );
    urls.insert(
        "windows-x86_64".to_string(),
        format!("https://download.docker.com/win/static/stable/x86_64/docker-{version}.zip"),
    );

    Some(PackageSource {
        name: "docker".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type: ArchiveType::TarGz,
    })
}

// ── Docker Compose ──────────────────────────────────────────────────────

fn fetch_docker_compose_versions() -> Result<Vec<String>> {
    let url = "https://api.github.com/repos/docker/compose/releases?per_page=100";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch Docker Compose versions from GitHub")?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read GitHub response body")?;

    let releases: Vec<serde_json::Value> =
        serde_json::from_str(&raw).context("Failed to parse GitHub releases JSON")?;

    let mut versions: Vec<String> = Vec::new();
    for release in releases {
        let tag = release["tag_name"].as_str().unwrap_or("");
        let stripped = tag.strip_prefix('v').unwrap_or(tag);
        if is_docker_compose_valid_version(stripped) && !versions.iter().any(|v| v == stripped) {
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

fn is_docker_compose_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn docker_compose_source(version: &str) -> Option<PackageSource> {
    let base = format!("https://github.com/docker/compose/releases/download/v{version}");

    let mut urls = HashMap::new();
    urls.insert(
        "linux-x86_64".to_string(),
        format!("{base}/docker-compose-linux-x86_64"),
    );
    urls.insert(
        "linux-aarch64".to_string(),
        format!("{base}/docker-compose-linux-aarch64"),
    );
    urls.insert(
        "macos-x86_64".to_string(),
        format!("{base}/docker-compose-darwin-x86_64"),
    );
    urls.insert(
        "macos-aarch64".to_string(),
        format!("{base}/docker-compose-darwin-aarch64"),
    );
    urls.insert(
        "windows-x86_64".to_string(),
        format!("{base}/docker-compose-windows-x86_64.exe"),
    );

    Some(PackageSource {
        name: "docker-compose".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type: ArchiveType::Zip,
    })
}

// ── act ─────────────────────────────────────────────────────────────────

fn fetch_act_versions() -> Result<Vec<String>> {
    let url = "https://api.github.com/repos/nektos/act/releases?per_page=100";
    let response = ureq::get(url)
        .header("User-Agent", "ntc")
        .call()
        .context("Failed to fetch act versions from GitHub")?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read GitHub response body")?;

    let releases: Vec<serde_json::Value> =
        serde_json::from_str(&raw).context("Failed to parse GitHub releases JSON")?;

    let mut versions: Vec<String> = Vec::new();
    for release in releases {
        let tag = release["tag_name"].as_str().unwrap_or("");
        let stripped = tag.strip_prefix('v').unwrap_or(tag);
        if is_act_valid_version(stripped) && !versions.iter().any(|v| v == stripped) {
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

fn is_act_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn act_source(version: &str) -> Option<PackageSource> {
    let base = format!("https://github.com/nektos/act/releases/download/v{version}");

    let mut urls = HashMap::new();
    urls.insert(
        "linux-x86_64".to_string(),
        format!("{base}/act_Linux_x86_64.tar.gz"),
    );
    urls.insert(
        "linux-aarch64".to_string(),
        format!("{base}/act_Linux_arm64.tar.gz"),
    );
    urls.insert(
        "macos-x86_64".to_string(),
        format!("{base}/act_Darwin_x86_64.tar.gz"),
    );
    urls.insert(
        "macos-aarch64".to_string(),
        format!("{base}/act_Darwin_arm64.tar.gz"),
    );
    urls.insert(
        "windows-x86_64".to_string(),
        format!("{base}/act_Windows_x86_64.zip"),
    );

    Some(PackageSource {
        name: "act".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type: ArchiveType::TarGz,
    })
}

// ── Scala (2.x + 3.x) ─────────────────────────────────────────────────

fn fetch_scala_versions() -> Result<Vec<String>> {
    let mut versions: Vec<String> = Vec::new();

    // Fetch Scala 2 from scala/scala (tags v2.x.x)
    if let Ok(v2) = fetch_github_releases("scala", "scala") {
        for v in v2 {
            if v.starts_with('2') && is_scala_valid_version(&v) {
                if !versions.contains(&v) {
                    versions.push(v);
                }
            }
        }
    }

    // Fetch Scala 3 from scala/scala3 (tags 3.x.x)
    if let Ok(v3) = fetch_github_releases("scala", "scala3") {
        for v in v3 {
            if v.starts_with('3') && is_scala3_valid_version(&v) {
                if !versions.contains(&v) {
                    versions.push(v);
                }
            }
        }
    }

    versions.sort_by(|a, b| {
        let a_parts: Vec<u32> = a
            .split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .filter_map(|s| s.parse().ok())
            .collect();
        let b_parts: Vec<u32> = b
            .split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .filter_map(|s| s.parse().ok())
            .collect();
        b_parts.cmp(&a_parts)
    });

    Ok(versions)
}

fn is_scala_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

fn is_scala3_valid_version(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() < 2 || parts.len() > 4 {
        return false;
    }
    // Must start with digit
    if !parts[0].chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    // Allow suffixes like -RC1, -M1 in the last part
    let last = parts.last().copied().unwrap_or("");
    let base = last.split('-').next().unwrap_or("");
    if !base.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    true
}

fn scala_source(version: &str) -> Option<PackageSource> {
    let is_scala2 = version.starts_with('2');
    let (ext, archive_type) = if cfg!(windows) {
        ("zip", ArchiveType::Zip)
    } else if is_scala2 {
        ("tgz", ArchiveType::TarGz)
    } else {
        ("tar.gz", ArchiveType::TarGz)
    };

    let (repo, tag_prefix, archive_prefix) = if is_scala2 {
        ("scala/scala", "v", "scala")
    } else {
        ("scala/scala3", "", "scala3")
    };

    let url = format!(
        "https://github.com/{repo}/releases/download/{tag_prefix}{version}/{archive_prefix}-{version}.{ext}"
    );

    let mut urls = HashMap::new();
    urls.insert("linux-x86_64".to_string(), url.clone());
    urls.insert("linux-aarch64".to_string(), url.clone());
    urls.insert("macos-x86_64".to_string(), url.clone());
    urls.insert("macos-aarch64".to_string(), url.clone());
    urls.insert("windows-x86_64".to_string(), url.clone());
    urls.insert("windows-aarch64".to_string(), url);

    Some(PackageSource {
        name: "scala".to_string(),
        version: version.to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type,
    })
}

// ── SQLite ──────────────────────────────────────────────────────────────

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

// ── SQLiteStudio ────────────────────────────────────────────────────────

fn fetch_sqlitestudio_versions() -> Result<Vec<String>> {
    let mut versions = fetch_github_releases("pawelsalawa", "sqlitestudio")?;
    // Filter out versions >= 4.0.0 which use "letos" asset naming
    versions.retain(|v| {
        let parts: Vec<u32> = v.split('.').filter_map(|s| s.parse().ok()).collect();
        parts.first().copied().unwrap_or(0) < 4
    });
    let mut all = vec!["latest".to_string()];
    all.extend(versions);
    Ok(all)
}

fn sqlitestudio_source(version: &str) -> Option<PackageSource> {
    let ver = if version == "latest" {
        let versions = fetch_sqlitestudio_versions().ok()?;
        // First entry beyond "latest" is the newest version
        versions.get(1)?.clone()
    } else {
        version.to_string()
    };

    let base = format!(
        "https://github.com/pawelsalawa/sqlitestudio/releases/download/{ver}/sqlitestudio-{ver}"
    );

    let mut urls = std::collections::HashMap::new();

    if cfg!(windows) {
        urls.insert(
            "windows-x86_64".to_string(),
            format!("{base}-windows-x64.zip"),
        );
        urls.insert(
            "windows-aarch64".to_string(),
            format!("{base}-windows-x64.zip"),
        );
    } else if cfg!(target_os = "linux") {
        urls.insert(
            "linux-x86_64".to_string(),
            format!("{base}-linux-x64.tar.xz"),
        );
        // No linux-arm64 binary provided upstream
    } else if cfg!(target_os = "macos") {
        urls.insert("macos-x86_64".to_string(), format!("{base}-macos-x64.dmg"));
        urls.insert("macos-aarch64".to_string(), format!("{base}-macos-x64.dmg"));
    }

    Some(PackageSource {
        name: "sqlitestudio".to_string(),
        version: ver,
        urls,
        sha256: std::collections::HashMap::new(),
        archive_type: ArchiveType::Zip,
    })
}

// ── FFmpeg ────────────────────────────────────────────────────────────

fn ffmpeg_source(_version: &str) -> Option<PackageSource> {
    let mut urls = HashMap::new();
    urls.insert(
        "windows-x86_64".to_string(),
        "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-gpl.zip"
            .to_string(),
    );
    urls.insert(
        "linux-x86_64".to_string(),
        "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-linux64-gpl.tar.xz"
            .to_string(),
    );
    urls.insert(
        "linux-aarch64".to_string(),
        "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-linuxarm64-gpl.tar.xz"
            .to_string(),
    );
    urls.insert(
        "macos-x86_64".to_string(),
        "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-macos64-gpl.tar.xz"
            .to_string(),
    );
    urls.insert(
        "macos-aarch64".to_string(),
        "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-macosarm64-gpl.tar.xz"
            .to_string(),
    );

    let archive_type = platform_ffmpeg_archive();

    Some(PackageSource {
        name: "ffmpeg".to_string(),
        version: "latest".to_string(),
        urls,
        sha256: HashMap::new(),
        archive_type,
    })
}

fn platform_ffmpeg_archive() -> ArchiveType {
    #[cfg(target_os = "windows")]
    {
        ArchiveType::Zip
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        ArchiveType::TarXz
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        ArchiveType::TarXz
    }
}

// ── ImageMagick ───────────────────────────────────────────────────────

fn imagemagick_source(version: &str) -> Option<PackageSource> {
    let resolved_ver = if version == "latest" {
        fetch_latest_imagemagick_version()?
    } else {
        version.to_string()
    };

    let mut urls = HashMap::new();
    // Windows: .exe installer
    urls.insert(
        "windows-x86_64".to_string(),
        format!(
            "https://github.com/ImageMagick/ImageMagick/releases/download/{resolved_ver}/ImageMagick-{resolved_ver}-Q16-HDRI-x64-dll.exe"
        ),
    );
    // Linux: AppImage (gcc build)
    urls.insert(
        "linux-x86_64".to_string(),
        format!(
            "https://github.com/ImageMagick/ImageMagick/releases/download/{resolved_ver}/ImageMagick-{resolved_ver}-gcc-x86_64.AppImage"
        ),
    );
    // macOS: not directly supported; site recommends Homebrew

    let archive_type = platform_imagemagick_archive();

    Some(PackageSource {
        name: "imagemagick".to_string(),
        version: resolved_ver,
        urls,
        sha256: HashMap::new(),
        archive_type,
    })
}

fn fetch_latest_imagemagick_version() -> Option<String> {
    let url = "https://api.github.com/repos/ImageMagick/ImageMagick/releases?per_page=10";
    let response = ureq::get(url).header("User-Agent", "ntc").call().ok()?;
    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader().read_to_string(&mut raw).ok()?;

    let releases: Vec<serde_json::Value> = serde_json::from_str(&raw).ok()?;

    for release in &releases {
        if let Some(tag) = release["tag_name"].as_str() {
            if tag.starts_with("7.") || tag.starts_with("6.") {
                return Some(tag.to_string());
            }
        }
    }

    None
}

fn platform_imagemagick_archive() -> ArchiveType {
    #[cfg(target_os = "windows")]
    {
        ArchiveType::Pkg
    }

    #[cfg(target_os = "linux")]
    {
        ArchiveType::Pkg
    }

    #[cfg(target_os = "macos")]
    {
        ArchiveType::Pkg
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        ArchiveType::Pkg
    }
}

fn current_year() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let dur = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    (1970 + dur.as_secs() / 31_536_000) as u32
}

fn fetch_github_releases(owner: &str, repo: &str) -> Result<Vec<String>> {
    let url = format!("https://api.github.com/repos/{owner}/{repo}/releases?per_page=100");
    let response = ureq::get(&url)
        .header("User-Agent", "ntc")
        .call()
        .with_context(|| format!("Failed to fetch {owner}/{repo} versions from GitHub"))?;

    let mut body = response.into_body();
    let mut raw = String::new();
    body.as_reader()
        .read_to_string(&mut raw)
        .context("Failed to read GitHub response body")?;

    let releases: Vec<serde_json::Value> =
        serde_json::from_str(&raw).context("Failed to parse GitHub releases JSON")?;

    let mut versions: Vec<String> = Vec::new();
    for release in releases {
        let tag = release["tag_name"].as_str().unwrap_or("");
        let stripped = tag.strip_prefix('v').unwrap_or(tag);
        if !stripped.is_empty() && !versions.iter().any(|v| v == stripped) {
            versions.push(stripped.to_string());
        }
    }

    Ok(versions)
}

// ── Tesseract ─────────────────────────────────────────────────────────

fn tesseract_source(_version: &str) -> Option<PackageSource> {
    let resolved_ver = fetch_latest_tesseract_version();

    let mut urls = HashMap::new();
    // Windows: UB-Mannheim NSIS installer (silent-installable).
    urls.insert(
        "windows-x86_64".to_string(),
        format!(
            "https://github.com/UB-Mannheim/tesseract/releases/download/v{resolved_ver}/tesseract-ocr-w64-setup-{resolved_ver}.exe"
        ),
    );
    // Linux/macOS: no bundled download; the installer prints apt/brew hints.

    Some(PackageSource {
        name: "tesseract".to_string(),
        version: resolved_ver,
        urls,
        sha256: HashMap::new(),
        archive_type: ArchiveType::Pkg,
    })
}

/// Resolve the latest UB-Mannheim tesseract release tag, falling back to a
/// known-good pinned version if the GitHub API is unreachable.
fn fetch_latest_tesseract_version() -> String {
    const FALLBACK: &str = "5.4.0.20240606";

    let url = "https://api.github.com/repos/UB-Mannheim/tesseract/releases/latest";
    let response = match ureq::get(url).header("User-Agent", "ntc").call() {
        Ok(r) => r,
        Err(_) => return FALLBACK.to_string(),
    };
    let mut body = response.into_body();
    let mut raw = String::new();
    if body.as_reader().read_to_string(&mut raw).is_err() {
        return FALLBACK.to_string();
    }

    let release: serde_json::Value = match serde_json::from_str(&raw) {
        Ok(v) => v,
        Err(_) => return FALLBACK.to_string(),
    };
    let tag = match release["tag_name"].as_str() {
        Some(t) => t,
        None => return FALLBACK.to_string(),
    };
    let stripped = tag.strip_prefix('v').unwrap_or(tag);
    if stripped.is_empty() {
        FALLBACK.to_string()
    } else {
        stripped.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Regression test for `ntpkg install python 3.14.5` failing with
    /// "Failed to write downloaded file": the source itself must resolve to
    /// a real python.org URL with the matching archive type.
    #[test]
    fn python_source_matches_platform_archive() {
        let src = python_source("3.14.5").expect("python source should resolve");
        if cfg!(target_os = "windows") {
            assert_eq!(src.archive_type, ArchiveType::Msi);
            let (key, suffix) = if cfg!(target_arch = "aarch64") {
                ("windows-aarch64", "python-3.14.5-arm64.exe")
            } else {
                ("windows-x86_64", "python-3.14.5-amd64.exe")
            };
            let url = src.urls.get(key).expect("windows python URL missing");
            assert!(
                url.ends_with(suffix),
                "unexpected python URL for {key}: {url}"
            );
        } else if cfg!(any(target_os = "linux", target_os = "android")) {
            // Source tarball is .tar.xz and must be extracted as TarXz
            // (a TarGz decoder on xz data fails).
            assert_eq!(src.archive_type, ArchiveType::TarXz);
            for key in ["linux-x86_64", "linux-aarch64"] {
                let url = src.urls.get(key).expect("linux python URL missing");
                assert!(
                    url.ends_with("Python-3.14.5.tar.xz"),
                    "unexpected python URL for {key}: {url}"
                );
            }
        } else if cfg!(target_os = "macos") {
            assert_eq!(src.archive_type, ArchiveType::Pkg);
        }
    }
}
