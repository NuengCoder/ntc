use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Tracks which package versions are installed and active
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageManifest {
    /// lang -> active_version
    pub active: HashMap<String, String>,
    /// lang -> list of installed versions
    pub installed: HashMap<String, Vec<String>>,
}

impl PackageManifest {
    pub fn new() -> Self {
        Self {
            active: HashMap::new(),
            installed: HashMap::new(),
        }
    }

    pub fn is_installed(&self, name: &str, version: &str) -> bool {
        self.installed
            .get(name)
            .is_some_and(|versions| versions.contains(&version.to_string()))
    }

    pub fn add_version(&mut self, name: &str, version: &str) {
        self.installed
            .entry(name.to_string())
            .or_default()
            .push(version.to_string());
    }

    pub fn remove_version(&mut self, name: &str, version: &str) {
        if let Some(versions) = self.installed.get_mut(name) {
            versions.retain(|v| v != version);
            if versions.is_empty() {
                self.installed.remove(name);
                self.active.remove(name);
            }
        }
    }

    pub fn activate(&mut self, name: &str, version: &str) {
        self.active.insert(name.to_string(), version.to_string());
    }
}

/// Metadata for a single package version
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub install_dir: PathBuf,
}

/// Describes where to download a runtime from
#[derive(Debug, Clone)]
pub struct PackageSource {
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub version: String,
    pub urls: HashMap<String, String>,
    #[allow(dead_code)]
    pub sha256: HashMap<String, String>,
    pub archive_type: ArchiveType,
}

/// Binaries to find inside the archive
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RuntimeBinaries {
    pub bins: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveType {
    #[cfg_attr(windows, allow(dead_code))]
    TarGz,
    #[cfg_attr(any(windows, target_os = "macos"), allow(dead_code))]
    TarXz,
    Zip,
    #[allow(dead_code)]
    Msi,
    #[allow(dead_code)]
    Pkg,
}

pub fn package_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ntc")
        .join("packages")
}

pub fn manifest_path() -> PathBuf {
    package_dir().join("packages.toml")
}

pub fn version_dir(name: &str, version: &str) -> PathBuf {
    let ver = version.replace('.', "");
    package_dir().join(name).join(ver)
}

pub fn bin_dir(name: &str, version: &str) -> PathBuf {
    #[cfg(windows)]
    {
        version_dir(name, version)
    }
    #[cfg(not(windows))]
    {
        version_dir(name, version).join("bin")
    }
}
