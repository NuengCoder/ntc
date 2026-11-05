//! Bot catalog, runtime downloader and UCI engine wrapper for `serve chess`.
//!
//! Stockfish is downloaded from its official GitHub release at runtime (it
//! is GPLv3 and never bundled). Only one install thread per bot can run at
//! a time; every other request observes the shared status map.

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

const STOCKFISH_RELEASES_API: &str =
    "https://api.github.com/repos/official-stockfish/Stockfish/releases/latest";
const STOCKFISH_FALLBACK_TAG: &str = "sf_17.1";
const MAX_DOWNLOAD_BYTES: u64 = 200 * 1024 * 1024;

pub struct BotDef {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub homepage: &'static str,
    pub license: &'static str,
}

pub const BOTS: &[BotDef] = &[BotDef {
    id: "stockfish",
    name: "Stockfish",
    description: "Strong open-source UCI engine. Downloaded from the official release.",
    homepage: "https://stockfishchess.org/",
    license: "GPL-3.0",
}];

pub fn bot_def(id: &str) -> Option<&'static BotDef> {
    BOTS.iter().find(|b| b.id == id)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum BotPhase {
    NotInstalled,
    Downloading,
    Extracting,
    Verifying,
    Ready,
    Error,
}

/// One selectable engine build for a bot: the ntc-managed download, any
/// `ntpkg`-installed version, or a system-wide binary on `PATH`.
#[derive(Clone, Serialize)]
pub struct BotEngine {
    pub id: String,
    pub label: String,
    pub version: Option<String>,
    /// `ntc` | `ntpkg` | `system`
    pub source: &'static str,
    /// The version `ntpkg use` points at (informational only: choosing an
    /// engine never follows it).
    pub active: bool,
    #[serde(skip)]
    pub binary: Option<PathBuf>,
}

#[derive(Clone, Serialize)]
pub struct BotStatus {
    pub id: String,
    pub name: String,
    pub description: &'static str,
    pub version: Option<String>,
    pub phase: BotPhase,
    pub downloaded: u64,
    pub total: u64,
    pub error: Option<String>,
    pub license: &'static str,
    pub homepage: &'static str,
    /// Every engine build the user can pick from.
    pub engines: Vec<BotEngine>,
    /// Id of the selected engine (`ntc`, `ntpkg:19`, `system`).
    pub engine: Option<String>,
    #[serde(skip)]
    pub binary: Option<PathBuf>,
}

impl BotStatus {
    fn new(def: &BotDef) -> Self {
        Self {
            id: def.id.to_string(),
            name: def.name.to_string(),
            description: def.description,
            version: None,
            phase: BotPhase::NotInstalled,
            downloaded: 0,
            total: 0,
            error: None,
            license: def.license,
            homepage: def.homepage,
            engines: Vec::new(),
            engine: None,
            binary: None,
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
struct BotManifest {
    #[serde(default)]
    bots: HashMap<String, BotRecord>,
    /// bot id -> selected engine id, so the choice survives restarts.
    #[serde(default)]
    selected: HashMap<String, String>,
}

#[derive(Clone, Serialize, Deserialize)]
struct BotRecord {
    version: String,
    binary: String,
    installed_at: String,
}

pub struct BotManager {
    root: PathBuf,
    states: Mutex<HashMap<String, BotStatus>>,
}

impl BotManager {
    pub fn new() -> Self {
        let root = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ntc")
            .join("chess")
            .join("bots");
        let _ = fs::create_dir_all(&root);

        let states: HashMap<String, BotStatus> = BOTS
            .iter()
            .map(|d| (d.id.to_string(), BotStatus::new(d)))
            .collect();

        let manager = Self {
            root,
            states: Mutex::new(states),
        };
        manager.rescan();
        manager
    }

    /// Rebuild the engine list from disk: the ntc download, every
    /// `ntpkg`-installed version and a `PATH` binary. Runs on every status
    /// read, so a version installed while the server is up shows up without
    /// a restart. Downloaded phases are left alone.
    pub fn rescan(&self) {
        let manifest = load_manifest(&self.root);
        let discovered: Vec<(&'static str, Vec<BotEngine>)> = BOTS
            .iter()
            .map(|def| {
                let active = crate::ntpkg::active_version(def.id);
                (def.id, discover_engines(&manifest, def, &active))
            })
            .collect();

        let mut states = self.lock();
        for (id, engines) in discovered {
            let Some(status) = states.get_mut(id) else {
                continue;
            };
            if matches!(
                status.phase,
                BotPhase::Downloading | BotPhase::Extracting | BotPhase::Verifying
            ) {
                continue;
            }
            let chosen = manifest
                .selected
                .get(id)
                .filter(|want| engines.iter().any(|e| &e.id == *want))
                .cloned()
                .or_else(|| default_engine(&engines))
                .and_then(|want| engines.iter().find(|e| e.id == want).cloned());
            status.engines = engines;
            match chosen {
                Some(engine) => {
                    status.engine = Some(engine.id);
                    status.version = engine.version;
                    status.binary = engine.binary;
                    status.phase = BotPhase::Ready;
                    status.error = None;
                }
                None => {
                    status.engine = None;
                    status.version = None;
                    status.binary = None;
                    if status.phase != BotPhase::Error {
                        status.phase = BotPhase::NotInstalled;
                    }
                }
            }
        }
    }

    /// Pick which installed engine build this bot plays with. The choice is
    /// persisted, so later `ntpkg use` calls never change it.
    pub fn select(&self, id: &str, engine_id: &str) -> Result<()> {
        if bot_def(id).is_none() {
            bail!("unknown bot");
        }
        {
            let states = self.lock();
            let status = states
                .get(id)
                .with_context(|| format!("unknown bot '{}'", id))?;
            if !status.engines.iter().any(|e| e.id == engine_id) {
                bail!("engine '{}' is not installed", engine_id);
            }
        }
        let mut manifest = load_manifest(&self.root);
        manifest
            .selected
            .insert(id.to_string(), engine_id.to_string());
        save_manifest(&self.root, &manifest)?;
        self.rescan();
        Ok(())
    }

    pub fn list(&self) -> Vec<BotStatus> {
        self.rescan();
        let states = self.lock();
        BOTS.iter()
            .filter_map(|d| states.get(d.id).cloned())
            .collect()
    }

    pub fn status(&self, id: &str) -> Option<BotStatus> {
        self.rescan();
        self.lock().get(id).cloned()
    }

    /// Start a background install. Repeated calls while a download is in
    /// flight are no-ops; an already-ready bot is a no-op too.
    pub fn install(self: &Arc<Self>, id: &str) -> Result<()> {
        let def = bot_def(id).context("unknown bot")?;
        {
            let mut states = self.lock();
            let status = states
                .get_mut(id)
                .with_context(|| format!("unknown bot '{}'", id))?;
            match status.phase {
                BotPhase::Downloading | BotPhase::Extracting | BotPhase::Verifying => return Ok(()),
                BotPhase::Ready => return Ok(()),
                _ => {}
            }
            status.phase = BotPhase::Downloading;
            status.downloaded = 0;
            status.total = 0;
            status.error = None;
        }
        println!("[chess] downloading bot: {}", def.name);

        let mgr = Arc::clone(self);
        let id = id.to_string();
        std::thread::spawn(move || {
            if let Err(e) = run_install(&mgr, &id) {
                println!("[chess] bot install failed ({}): {:#}", id, e);
                mgr.set_error(&id, format!("{:#}", e));
            }
        });
        Ok(())
    }

    /// Remove a bot installed under the managed bots directory.
    pub fn uninstall(&self, id: &str) -> Result<()> {
        if bot_def(id).is_none() {
            bail!("unknown bot");
        }
        {
            let states = self.lock();
            let status = states
                .get(id)
                .with_context(|| format!("unknown bot '{}'", id))?;
            if matches!(
                status.phase,
                BotPhase::Downloading | BotPhase::Extracting | BotPhase::Verifying
            ) {
                bail!("bot is busy");
            }
            if !status.engines.iter().any(|e| e.source == "ntc") {
                if status.engines.iter().any(|e| e.source == "ntpkg") {
                    bail!(
                        "engine was installed by ntpkg; run `ntpkg uninstall {}` to remove it",
                        id
                    );
                }
                bail!("engine is not managed by ntc");
            }
        }
        let dir = self.root.join(id);
        if dir.exists() {
            fs::remove_dir_all(&dir).with_context(|| format!("remove {}", dir.display()))?;
        }
        let mut manifest = load_manifest(&self.root);
        manifest.bots.remove(id);
        if manifest.selected.get(id).is_some_and(|e| e == "ntc") {
            manifest.selected.remove(id);
        }
        save_manifest(&self.root, &manifest)?;
        self.rescan();
        Ok(())
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, HashMap<String, BotStatus>> {
        self.states.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn set_phase(&self, id: &str, phase: BotPhase) {
        if let Some(s) = self.lock().get_mut(id) {
            s.phase = phase;
        }
    }

    fn set_version(&self, id: &str, version: &str) {
        if let Some(s) = self.lock().get_mut(id) {
            s.version = Some(version.to_string());
        }
    }

    fn set_progress(&self, id: &str, downloaded: u64, total: u64) {
        if let Some(s) = self.lock().get_mut(id) {
            s.downloaded = downloaded;
            s.total = total;
        }
    }

    fn set_error(&self, id: &str, error: String) {
        if let Some(s) = self.lock().get_mut(id) {
            s.phase = BotPhase::Error;
            s.error = Some(error);
        }
    }

    fn set_ready(&self, id: &str, version: String, binary: PathBuf) {
        if let Some(s) = self.lock().get_mut(id) {
            s.phase = BotPhase::Ready;
            s.version = Some(version);
            s.binary = Some(binary);
            s.error = None;
        }
    }
}

impl Default for BotManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Default pick when the user has not chosen yet: an explicit ntc download,
/// then the newest `ntpkg` version, then a `PATH` binary. Never `ntpkg use`,
/// which is why the active version is only reported, not applied.
fn default_engine(engines: &[BotEngine]) -> Option<String> {
    for source in ["ntc", "ntpkg", "system"] {
        if let Some(engine) = engines.iter().find(|e| e.source == source) {
            return Some(engine.id.clone());
        }
    }
    None
}

fn discover_engines(
    manifest: &BotManifest,
    def: &BotDef,
    active: &Option<String>,
) -> Vec<BotEngine> {
    let mut engines = Vec::new();

    if let Some(rec) = manifest.bots.get(def.id) {
        let path = PathBuf::from(&rec.binary);
        if path.is_file() {
            engines.push(BotEngine {
                id: "ntc".to_string(),
                label: format!("ntc download ({})", rec.version),
                version: Some(rec.version.clone()),
                source: "ntc",
                active: false,
                binary: Some(path),
            });
        }
    }

    for (version, path) in crate::ntpkg::installed_binaries(def.id, def.id) {
        engines.push(BotEngine {
            id: format!("ntpkg:{}", version),
            label: format!("ntpkg {}", version),
            active: active.as_deref() == Some(version.as_str()),
            version: Some(version),
            source: "ntpkg",
            binary: Some(path),
        });
    }

    if let Ok(path) = which::which(def.id) {
        // PATH often resolves to one of the ntpkg copies; list it once.
        let duplicate = engines
            .iter()
            .any(|e| e.binary.as_ref().is_some_and(|b| same_file(b, &path)));
        if !duplicate {
            engines.push(BotEngine {
                id: "system".to_string(),
                label: "system PATH".to_string(),
                version: None,
                source: "system",
                active: false,
                binary: Some(path),
            });
        }
    }

    engines
}

fn same_file(a: &Path, b: &Path) -> bool {
    match (fs::canonicalize(a), fs::canonicalize(b)) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    }
}

// ── Install pipeline ──────────────────────────────────────────────────────

struct Platform {
    os: &'static str,   // windows | linux | macos
    arch: &'static str, // x86_64 | aarch64
    avx2: bool,
    bmi2: bool,
}

impl Platform {
    fn detect() -> Result<Self> {
        #[cfg(target_os = "windows")]
        let os = "windows";
        #[cfg(all(unix, not(target_os = "macos"), not(target_os = "android")))]
        let os = "linux";
        #[cfg(target_os = "macos")]
        let os = "macos";
        #[cfg(target_os = "android")]
        let os = "android";

        let arch = std::env::consts::ARCH;
        let (avx2, bmi2) = cpu_features();
        let p = Self {
            os,
            arch,
            avx2,
            bmi2,
        };
        if p.os == "android" {
            bail!("automatic bot download is not supported on Android/Termux");
        }
        if p.os == "windows" && p.arch != "x86_64" {
            bail!("automatic bot download is only supported on x86_64 Windows");
        }
        if p.os == "linux" && p.arch != "x86_64" {
            bail!("automatic bot download is only supported on x86_64 Linux");
        }
        if p.os == "macos" && p.arch != "x86_64" && p.arch != "aarch64" {
            bail!("automatic bot download is not supported on this Mac");
        }
        Ok(p)
    }

    fn label(&self) -> String {
        format!("{}-{}", self.os, self.arch)
    }
}

#[cfg(target_arch = "x86_64")]
fn cpu_features() -> (bool, bool) {
    (
        std::is_x86_feature_detected!("avx2"),
        std::is_x86_feature_detected!("bmi2"),
    )
}

#[cfg(not(target_arch = "x86_64"))]
fn cpu_features() -> (bool, bool) {
    (false, false)
}

struct ReleaseAsset {
    tag: String,
    name: String,
    url: String,
}

fn resolve_asset(platform: &Platform) -> Result<ReleaseAsset> {
    match fetch_latest_asset(platform) {
        Ok(asset) => Ok(asset),
        Err(e) => {
            let fallback = fallback_asset(platform)
                .with_context(|| format!("latest release failed ({e:#}) and no fallback asset"))?;
            Ok(fallback)
        }
    }
}

fn fetch_latest_asset(platform: &Platform) -> Result<ReleaseAsset> {
    let mut response = ureq::get(STOCKFISH_RELEASES_API)
        .header("User-Agent", "ntc-chess")
        .header("Accept", "application/vnd.github+json")
        .call()
        .context("GitHub release request failed")?;
    let raw = response
        .body_mut()
        .read_to_string()
        .context("read GitHub release response")?;
    let v: serde_json::Value =
        serde_json::from_str(&raw).context("parse GitHub release response")?;
    let tag = v
        .get("tag_name")
        .and_then(|x| x.as_str())
        .context("GitHub release has no tag_name")?
        .to_string();
    let assets = v
        .get("assets")
        .and_then(|x| x.as_array())
        .context("GitHub release has no assets")?;
    let names: Vec<String> = assets
        .iter()
        .filter_map(|a| a.get("name").and_then(|n| n.as_str()).map(str::to_string))
        .collect();
    let idx = select_asset(&names, platform)
        .with_context(|| format!("no Stockfish asset for {}", platform.label()))?;
    let asset = &assets[idx];
    let name = names[idx].clone();
    let url = asset
        .get("browser_download_url")
        .and_then(|x| x.as_str())
        .context("asset has no browser_download_url")?
        .to_string();
    if !url.starts_with("https://github.com/official-stockfish/Stockfish/releases/download/") {
        bail!("unexpected asset URL");
    }
    Ok(ReleaseAsset { tag, name, url })
}

fn select_asset(names: &[String], platform: &Platform) -> Option<usize> {
    let os_needles: &[&str] = match platform.os {
        "windows" => &["windows"],
        "linux" => &["ubuntu", "linux"],
        "macos" => &["macos", "osx"],
        _ => return None,
    };
    let arch_needles: &[&str] = match (platform.os, platform.arch) {
        // Stockfish 19+ ships one universal macOS build for both arches.
        ("macos", "aarch64") => &["apple-silicon", "m1", "arm64", "aarch64", "universal"],
        ("macos", "x86_64") => &["x86-64", "x86_64", "amd64", "universal"],
        (_, "x86_64") => &["x86-64", "x86_64", "amd64"],
        (_, "aarch64") => &["arm64", "aarch64"],
        _ => return None,
    };

    let mut best: Option<(i32, usize)> = None;
    for (i, name) in names.iter().enumerate() {
        let n = name.to_ascii_lowercase();
        if !n.starts_with("stockfish-") {
            continue;
        }
        if !os_needles.iter().any(|s| n.contains(s)) {
            continue;
        }
        if !arch_needles.iter().any(|s| n.contains(s)) {
            continue;
        }
        if !(n.ends_with(".zip") || n.ends_with(".tar") || n.ends_with(".tar.gz")) {
            continue;
        }

        let mut score = 10;
        if n.contains("avx2") {
            score += if platform.avx2 { 30 } else { -20 };
        } else if n.contains("bmi2") {
            score += if platform.bmi2 { 20 } else { -10 };
        } else if n.contains("x86-64") || n.contains("x86_64") {
            score += 5;
        }
        // Prefer a plain archive over never matching one (score stays low
        // when the CPU lacks the advertised instruction set).
        if n.ends_with(".zip") || n.ends_with(".tar") {
            score += 1;
        }
        if best.map(|(bs, _)| score > bs).unwrap_or(true) {
            best = Some((score, i));
        }
    }
    best.map(|(_, i)| i)
}

fn fallback_asset(platform: &Platform) -> Option<ReleaseAsset> {
    let name = match (platform.os, platform.arch) {
        ("windows", "x86_64") if platform.avx2 => "stockfish-windows-x86-64-avx2.zip",
        ("windows", "x86_64") if platform.bmi2 => "stockfish-windows-x86-64-bmi2.zip",
        ("windows", "x86_64") => "stockfish-windows-x86-64.zip",
        ("linux", "x86_64") if platform.avx2 => "stockfish-ubuntu-x86-64-avx2.tar",
        ("linux", "x86_64") if platform.bmi2 => "stockfish-ubuntu-x86-64-bmi2.tar",
        ("linux", "x86_64") => "stockfish-ubuntu-x86-64.tar",
        ("macos", "aarch64") => "stockfish-macos-m1-apple-silicon.tar",
        ("macos", "x86_64") if platform.avx2 => "stockfish-macos-x86-64-avx2.tar",
        ("macos", "x86_64") => "stockfish-macos-x86-64.tar",
        _ => return None,
    };
    Some(ReleaseAsset {
        tag: STOCKFISH_FALLBACK_TAG.to_string(),
        name: name.to_string(),
        url: format!(
            "https://github.com/official-stockfish/Stockfish/releases/download/{}/{}",
            STOCKFISH_FALLBACK_TAG, name
        ),
    })
}

fn run_install(mgr: &Arc<BotManager>, id: &str) -> Result<()> {
    let platform = Platform::detect()?;
    let asset = resolve_asset(&platform)?;
    mgr.set_version(id, &asset.tag);

    let safe_tag: String = asset
        .tag
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    let tmp_dir = mgr.root.join(id).join(format!(
        ".tmp-{}-{}",
        std::process::id(),
        rand::random::<u64>()
    ));
    fs::create_dir_all(&tmp_dir).with_context(|| format!("create {}", tmp_dir.display()))?;
    let part = tmp_dir.join("download.part");

    let result = (|| -> Result<PathBuf> {
        download_stream(mgr, id, &asset.url, &part)?;
        mgr.set_phase(id, BotPhase::Extracting);
        extract_archive(&part, &asset.name, &tmp_dir)?;
        let bin_name = if cfg!(windows) {
            "stockfish.exe"
        } else {
            "stockfish"
        };
        let found = find_engine_binary(&tmp_dir)
            .with_context(|| format!("{} not found in downloaded archive", bin_name))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&found)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&found, perms)?;
        }

        mgr.set_phase(id, BotPhase::Verifying);
        UciEngine::smoke_test(&found)?;

        let final_dir = mgr.root.join(id).join(&safe_tag);
        fs::create_dir_all(&final_dir)
            .with_context(|| format!("create {}", final_dir.display()))?;
        let final_bin = final_dir.join(bin_name);
        if final_bin.exists() {
            let _ = fs::remove_file(&final_bin);
        }
        fs::rename(&found, &final_bin).or_else(|_| {
            fs::copy(&found, &final_bin)?;
            fs::remove_file(&found)?;
            Ok::<(), std::io::Error>(())
        })?;

        let mut manifest = load_manifest(&mgr.root);
        manifest.bots.insert(
            id.to_string(),
            BotRecord {
                version: asset.tag.clone(),
                binary: final_bin.to_string_lossy().to_string(),
                installed_at: chrono::Utc::now().to_rfc3339(),
            },
        );
        save_manifest(&mgr.root, &manifest)?;
        Ok(final_bin)
    })();

    let _ = fs::remove_dir_all(&tmp_dir);
    let final_bin = result?;
    mgr.set_ready(id, asset.tag.clone(), final_bin);
    // A fresh download becomes the user's engine unless they picked another.
    let mut manifest = load_manifest(&mgr.root);
    if !manifest.selected.contains_key(id) {
        manifest.selected.insert(id.to_string(), "ntc".to_string());
        let _ = save_manifest(&mgr.root, &manifest);
    }
    mgr.rescan();
    println!("[chess] bot ready: {} ({})", id, asset.tag);
    Ok(())
}

fn download_stream(mgr: &Arc<BotManager>, id: &str, url: &str, dest: &Path) -> Result<()> {
    let mut response = ureq::get(url)
        .header("User-Agent", "ntc-chess")
        .call()
        .with_context(|| format!("download {}", url))?;
    let total = response
        .headers()
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(0);
    if total > MAX_DOWNLOAD_BYTES {
        bail!("download is too large ({} bytes)", total);
    }
    mgr.set_progress(id, 0, total);

    let mut file = fs::File::create(dest).with_context(|| format!("create {}", dest.display()))?;
    let body = response.body_mut();
    let mut reader = body.as_reader();
    let mut buf = vec![0u8; 64 * 1024];
    let mut downloaded: u64 = 0;
    loop {
        let n = reader.read(&mut buf).context("read download stream")?;
        if n == 0 {
            break;
        }
        downloaded += n as u64;
        if downloaded > MAX_DOWNLOAD_BYTES {
            bail!("download exceeded size limit");
        }
        file.write_all(&buf[..n]).context("write download")?;
        mgr.set_progress(id, downloaded, total);
    }
    file.flush()?;
    if total > 0 && downloaded != total {
        bail!("incomplete download ({} of {} bytes)", downloaded, total);
    }
    Ok(())
}

enum ArchiveKind {
    Zip,
    Tar,
    TarGz,
    TarXz,
}

fn archive_kind(name: &str) -> Result<ArchiveKind> {
    let n = name.to_ascii_lowercase();
    if n.ends_with(".zip") {
        Ok(ArchiveKind::Zip)
    } else if n.ends_with(".tar.gz") || n.ends_with(".tgz") {
        Ok(ArchiveKind::TarGz)
    } else if n.ends_with(".tar.xz") || n.ends_with(".txz") {
        Ok(ArchiveKind::TarXz)
    } else if n.ends_with(".tar") {
        Ok(ArchiveKind::Tar)
    } else {
        bail!("unsupported archive '{}'", name)
    }
}

fn extract_archive(archive: &Path, name: &str, dest: &Path) -> Result<()> {
    let file = fs::File::open(archive).with_context(|| format!("open {}", archive.display()))?;
    match archive_kind(name)? {
        ArchiveKind::Zip => {
            let mut zip = zip::ZipArchive::new(file)
                .with_context(|| format!("read zip {}", archive.display()))?;
            zip.extract(dest)
                .with_context(|| format!("extract zip to {}", dest.display()))?;
        }
        ArchiveKind::Tar => {
            let mut tar = tar::Archive::new(file);
            tar.unpack(dest)
                .with_context(|| format!("extract tar to {}", dest.display()))?;
        }
        ArchiveKind::TarGz => {
            let mut tar = tar::Archive::new(flate2::read::GzDecoder::new(file));
            tar.unpack(dest)
                .with_context(|| format!("extract tar.gz to {}", dest.display()))?;
        }
        ArchiveKind::TarXz => {
            let mut tar = tar::Archive::new(xz2::read::XzDecoder::new(file));
            tar.unpack(dest)
                .with_context(|| format!("extract tar.xz to {}", dest.display()))?;
        }
    }
    Ok(())
}

/// Official archives name the engine after the build
/// (`stockfish-windows-x86-64-universal.exe`), so accept the plain name
/// first and then any `stockfish`/`stockfish-*` executable.
fn find_engine_binary(dir: &Path) -> Option<PathBuf> {
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

fn load_manifest(root: &Path) -> BotManifest {
    fs::read_to_string(root.join("bots.json"))
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn save_manifest(root: &Path, manifest: &BotManifest) -> Result<()> {
    let path = root.join("bots.json");
    let data = serde_json::to_vec_pretty(manifest)?;
    fs::write(&path, data).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}

// ── UCI engine ────────────────────────────────────────────────────────────

pub struct UciEngine {
    child: Child,
    stdin: ChildStdin,
    lines: Receiver<String>,
    _reader: JoinHandle<()>,
}

impl UciEngine {
    pub fn spawn(path: &Path) -> Result<Self> {
        let mut cmd = Command::new(path);
        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }
        let mut child = cmd
            .spawn()
            .with_context(|| format!("spawn engine {}", path.display()))?;
        let stdin = child.stdin.take().context("engine stdin")?;
        let stdout = child.stdout.take().context("engine stdout")?;
        let (tx, rx) = mpsc::channel();
        let reader = std::thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines().map_while(Result::ok) {
                if tx.send(line).is_err() {
                    break;
                }
            }
        });
        Ok(Self {
            child,
            stdin,
            lines: rx,
            _reader: reader,
        })
    }

    pub fn smoke_test(path: &Path) -> Result<()> {
        let mut engine = Self::spawn(path)?;
        engine.handshake()?;
        Ok(())
    }

    pub fn handshake(&mut self) -> Result<()> {
        self.send("uci")?;
        self.wait_line(|l| l.trim() == "uciok", Duration::from_secs(5), "uciok")?;
        self.send("isready")?;
        self.wait_line(|l| l.trim() == "readyok", Duration::from_secs(5), "readyok")?;
        Ok(())
    }

    pub fn configure(&mut self, level: &str) -> Result<()> {
        let skill = level_skill(level);
        self.send("ucinewgame")?;
        self.send("setoption name Threads value 1")?;
        self.send("setoption name Hash value 16")?;
        self.send(&format!("setoption name Skill Level value {}", skill))?;
        self.send("isready")?;
        self.wait_line(|l| l.trim() == "readyok", Duration::from_secs(5), "readyok")?;
        Ok(())
    }

    pub fn best_move(&mut self, fen: &str, level: &str) -> Result<String> {
        let move_ms = level_movetime(level);
        self.send(&format!("position fen {}", fen))?;
        self.send(&format!("go movetime {}", move_ms))?;
        let line = self.wait_line(
            |l| l.trim_start().starts_with("bestmove"),
            Duration::from_millis(move_ms + 5_000),
            "bestmove",
        )?;
        let mv = line
            .split_whitespace()
            .nth(1)
            .unwrap_or_default()
            .to_string();
        if mv.is_empty() || mv == "0000" {
            bail!("engine returned no move");
        }
        Ok(mv)
    }

    fn send(&mut self, cmd: &str) -> Result<()> {
        self.stdin
            .write_all(cmd.as_bytes())
            .context("write engine stdin")?;
        self.stdin.write_all(b"\n").context("write engine stdin")?;
        self.stdin.flush().context("flush engine stdin")?;
        Ok(())
    }

    fn wait_line<F>(&self, pred: F, timeout: Duration, what: &str) -> Result<String>
    where
        F: Fn(&str) -> bool,
    {
        let start = Instant::now();
        loop {
            let remain = timeout
                .checked_sub(start.elapsed())
                .unwrap_or_else(|| Duration::from_millis(0));
            if remain.is_zero() {
                bail!("engine timeout waiting for {}", what);
            }
            match self.lines.recv_timeout(remain) {
                Ok(line) => {
                    if pred(&line) {
                        return Ok(line);
                    }
                }
                Err(RecvTimeoutError::Timeout) => {
                    bail!("engine timeout waiting for {}", what)
                }
                Err(RecvTimeoutError::Disconnected) => {
                    bail!("engine closed stdout while waiting for {}", what)
                }
            }
        }
    }
}

impl Drop for UciEngine {
    fn drop(&mut self) {
        let _ = self.send("quit");
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

pub fn level_valid(level: &str) -> bool {
    matches!(level, "easy" | "normal" | "hard" | "master")
}

pub fn level_skill(level: &str) -> i32 {
    match level {
        "easy" => 0,
        "hard" => 10,
        "master" => 20,
        _ => 5,
    }
}

pub fn level_movetime(level: &str) -> u64 {
    match level {
        "easy" => 150,
        "hard" => 700,
        "master" => 1_500,
        _ => 300,
    }
}

pub fn level_label(level: &str) -> &'static str {
    match level {
        "easy" => "Easy",
        "hard" => "Hard",
        "master" => "Master",
        _ => "Normal",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn selects_avx2_when_supported() {
        let p = Platform {
            os: "windows",
            arch: "x86_64",
            avx2: true,
            bmi2: true,
        };
        let assets = names(&[
            "stockfish-windows-x86-64.zip",
            "stockfish-windows-x86-64-avx2.zip",
            "stockfish-windows-x86-64-bmi2.zip",
        ]);
        assert_eq!(
            select_asset(&assets, &p).map(|i| assets[i].as_str()),
            Some("stockfish-windows-x86-64-avx2.zip")
        );
    }

    #[test]
    fn avoids_avx2_on_old_cpu() {
        let p = Platform {
            os: "linux",
            arch: "x86_64",
            avx2: false,
            bmi2: false,
        };
        let assets = names(&[
            "stockfish-ubuntu-x86-64-avx2.tar",
            "stockfish-ubuntu-x86-64-bmi2.tar",
            "stockfish-ubuntu-x86-64.tar",
        ]);
        assert_eq!(
            select_asset(&assets, &p).map(|i| assets[i].as_str()),
            Some("stockfish-ubuntu-x86-64.tar")
        );
    }

    #[test]
    fn selects_apple_silicon_for_arm_mac() {
        let p = Platform {
            os: "macos",
            arch: "aarch64",
            avx2: false,
            bmi2: false,
        };
        let assets = names(&[
            "stockfish-macos-x86-64-avx2.tar",
            "stockfish-macos-m1-apple-silicon.tar",
        ]);
        assert_eq!(
            select_asset(&assets, &p).map(|i| assets[i].as_str()),
            Some("stockfish-macos-m1-apple-silicon.tar")
        );
    }

    #[test]
    fn archive_kinds_are_detected() {
        assert!(matches!(archive_kind("a.zip"), Ok(ArchiveKind::Zip)));
        assert!(matches!(archive_kind("a.tar"), Ok(ArchiveKind::Tar)));
        assert!(matches!(archive_kind("a.tar.gz"), Ok(ArchiveKind::TarGz)));
        assert!(matches!(archive_kind("a.tar.xz"), Ok(ArchiveKind::TarXz)));
        assert!(archive_kind("a.rar").is_err());
    }

    #[test]
    fn level_presets_are_bounded() {
        for level in ["easy", "normal", "hard", "master"] {
            assert!(level_valid(level));
            assert!((0..=20).contains(&level_skill(level)));
            assert!(level_movetime(level) > 0);
        }
        assert!(!level_valid("impossible"));
    }

    #[test]
    fn engine_defaults_prefer_ntc_then_ntpkg_then_path() {
        let engine = |id: &str, source: &'static str| BotEngine {
            id: id.to_string(),
            label: id.to_string(),
            version: Some(id.to_string()),
            source,
            active: false,
            binary: None,
        };
        let ntpkg = [engine("ntpkg:19", "ntpkg"), engine("system", "system")];
        assert_eq!(default_engine(&ntpkg).as_deref(), Some("ntpkg:19"));

        let with_ntc = [engine("ntpkg:19", "ntpkg"), engine("ntc", "ntc")];
        assert_eq!(default_engine(&with_ntc).as_deref(), Some("ntc"));
        assert_eq!(default_engine(&[]), None);
    }

    #[test]
    fn manifest_roundtrip_uses_temp_root() {
        let dir = tempfile::tempdir().unwrap();
        let mut m = BotManifest::default();
        m.bots.insert(
            "stockfish".into(),
            BotRecord {
                version: "sf_17.1".into(),
                binary: "/tmp/stockfish".into(),
                installed_at: "now".into(),
            },
        );
        save_manifest(dir.path(), &m).unwrap();
        let loaded = load_manifest(dir.path());
        assert_eq!(loaded.bots["stockfish"].version, "sf_17.1");
    }
}
