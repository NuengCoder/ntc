//! Platform gating for the foreign-shell integration.
//!
//! PowerShell hints and `ntc ps` are Windows-only; bash hints and argument
//! delegation need a real bash — desktop Linux or Termux on Android (an
//! Android userland without Termux markers, e.g. a bare proot container, is
//! rejected); zsh hints and `ntc zsh` are macOS-only. `cmd`, `sh` and `fish`
//! keep their existing installed-only behavior everywhere.
//!
//! `supports_shell` runs on every keystroke, so every probe here is cached.

use std::collections::HashMap;
use std::path::Path;
use std::sync::OnceLock;

/// Package-manager families ntc can name an install command for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PackageManager {
    Pacman,
    Dnf,
    Apt,
    Apk,
    /// Termux on Android.
    Pkg,
    Zypper,
    Xbps,
    Emerge,
    Unknown,
}

impl PackageManager {
    /// The package-manager command name shown in status output.
    pub(crate) fn command(self) -> &'static str {
        match self {
            PackageManager::Pacman => "pacman",
            PackageManager::Dnf => "dnf",
            PackageManager::Apt => "apt",
            PackageManager::Apk => "apk",
            PackageManager::Pkg => "pkg",
            PackageManager::Zypper => "zypper",
            PackageManager::Xbps => "xbps-install",
            PackageManager::Emerge => "emerge",
            PackageManager::Unknown => "unknown",
        }
    }

    /// How to install `bash-completion` on this distro family.
    pub(crate) fn install_bash_completion(self) -> Option<&'static str> {
        Some(match self {
            PackageManager::Pacman => "sudo pacman -S bash-completion",
            PackageManager::Dnf => "sudo dnf install bash-completion",
            PackageManager::Apt => "sudo apt install bash-completion",
            PackageManager::Apk => "sudo apk add bash-completion",
            PackageManager::Pkg => "pkg install bash-completion",
            PackageManager::Zypper => "sudo zypper install bash-completion",
            PackageManager::Xbps => "sudo xbps-install -S bash-completion",
            PackageManager::Emerge => "sudo emerge -av app-shells/bash-completion",
            PackageManager::Unknown => return None,
        })
    }
}

/// A detected desktop-Linux distribution.
#[derive(Debug, Clone)]
pub(crate) struct Distro {
    /// `PRETTY_NAME` from `os-release`, falling back to `NAME` then `ID`.
    pub name: String,
    /// Lowercase `ID` (`arch`, `ubuntu`, `alpine`, …), `linux` when absent.
    pub id: String,
    pub package_manager: PackageManager,
}

pub(crate) fn is_windows() -> bool {
    cfg!(windows)
}

/// Desktop Linux means a Linux build that is not an Android/Termux shell.
pub(crate) fn is_desktop_linux() -> bool {
    static DESKTOP: OnceLock<bool> = OnceLock::new();
    *DESKTOP.get_or_init(|| cfg!(target_os = "linux") && !android_markers())
}

/// Termux on Android: a real bash, a POSIX filesystem and `pkg`.
pub(crate) fn is_termux() -> bool {
    static TERMUX: OnceLock<bool> = OnceLock::new();
    *TERMUX
        .get_or_init(|| cfg!(any(target_os = "linux", target_os = "android")) && android_markers())
}

/// Platforms whose bash is one ntc can hint and host: desktop Linux and
/// Termux. Both get the same integration; only the prompt label and package
/// manager differ.
pub(crate) fn is_bash_platform() -> bool {
    is_desktop_linux() || is_termux()
}

/// macOS is the only platform where zsh is the integrated shell and `brew`
/// belongs to the shell integration.
pub(crate) fn is_macos() -> bool {
    cfg!(target_os = "macos")
}

/// The detected distro, or `None` off desktop Linux.
pub(crate) fn distro() -> Option<&'static Distro> {
    static DISTRO: OnceLock<Option<Distro>> = OnceLock::new();
    DISTRO.get_or_init(detect_distro).as_ref()
}

/// Prompt label for the integrated bash mode: `ntc arch [dir]>`,
/// `ntc ubuntu [dir]>`, `ntc alpine [dir]>`, and `linux` as a fallback.
pub(crate) fn prompt_label() -> &'static str {
    distro().map(|d| d.id.as_str()).unwrap_or("linux")
}

/// True when `name` may be hinted, delegated to, or enter a host mode
/// (`ntc ps`, `ntc <distro>`, `ntc zsh`) on this platform. Unknown names are
/// left to the caller's registry.
pub(crate) fn supports_shell(name: &str) -> bool {
    if super::is_powershell(name) {
        return is_windows();
    }
    if name.eq_ignore_ascii_case("bash") {
        return is_bash_platform();
    }
    if name.eq_ignore_ascii_case("zsh") {
        return is_macos();
    }
    true
}

fn android_markers() -> bool {
    android_like(
        std::env::var_os("TERMUX_VERSION").is_some(),
        std::env::var("PREFIX").ok().as_deref(),
        std::env::var_os("ANDROID_ROOT").is_some(),
        std::env::var_os("ANDROID_DATA").is_some(),
        Path::new("/system/build.prop").exists(),
    )
}

/// Pure Android detection so it can be tested without touching the process
/// environment. `PREFIX` catches Termux even inside a proot Linux userland.
fn android_like(
    termux: bool,
    prefix: Option<&str>,
    android_root: bool,
    android_data: bool,
    build_prop: bool,
) -> bool {
    termux
        || android_root
        || android_data
        || build_prop
        || prefix
            .map(|prefix| prefix.contains("com.termux"))
            .unwrap_or(false)
}

fn detect_distro() -> Option<Distro> {
    if is_termux() {
        return Some(Distro {
            name: "Termux".to_string(),
            id: "termux".to_string(),
            package_manager: PackageManager::Pkg,
        });
    }
    if !is_desktop_linux() {
        return None;
    }
    let text = std::fs::read_to_string("/etc/os-release")
        .or_else(|_| std::fs::read_to_string("/usr/lib/os-release"))
        .ok()?;
    let fields = parse_os_release(&text);
    let id = fields.get("ID").map(String::as_str).unwrap_or_default();
    let id_like = fields
        .get("ID_LIKE")
        .map(String::as_str)
        .unwrap_or_default();
    let name = ["PRETTY_NAME", "NAME"]
        .iter()
        .filter_map(|key| fields.get(*key))
        .map(|value| value.trim())
        .find(|value| !value.is_empty())
        .map(str::to_string)
        .or_else(|| {
            let id = id.trim();
            (!id.is_empty()).then(|| id.to_string())
        })
        .unwrap_or_else(|| "Linux".to_string());
    let id = normalize_id(id);
    let package_manager =
        ids_to_manager(id.as_str(), id_like).unwrap_or_else(probe_package_manager);
    Some(Distro {
        name,
        id,
        package_manager,
    })
}

/// Lowercase `ID` for the prompt; `linux` when `os-release` has none.
fn normalize_id(raw: &str) -> String {
    let id = raw.trim().to_ascii_lowercase();
    if id.is_empty() {
        "linux".to_string()
    } else {
        id
    }
}

/// Parse the `KEY=value` lines of an `os-release` file.
fn parse_os_release(text: &str) -> HashMap<String, String> {
    let mut fields = HashMap::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        if key.is_empty() {
            continue;
        }
        fields.insert(key.to_string(), unquote(value));
    }
    fields
}

fn unquote(value: &str) -> String {
    let value = value.trim();
    let bytes = value.as_bytes();
    if value.len() >= 2
        && ((bytes[0] == b'"' && bytes[bytes.len() - 1] == b'"')
            || (bytes[0] == b'\'' && bytes[bytes.len() - 1] == b'\''))
    {
        return value[1..value.len() - 1].to_string();
    }
    value.to_string()
}

fn ids_to_manager(id: &str, id_like: &str) -> Option<PackageManager> {
    std::iter::once(id)
        .chain(id_like.split_ascii_whitespace())
        .filter_map(|token| id_to_manager(token))
        .next()
}

fn id_to_manager(id: &str) -> Option<PackageManager> {
    let id = id.trim().to_ascii_lowercase();
    Some(match id.as_str() {
        "arch" | "manjaro" | "endeavouros" | "garuda" | "cachyos" | "artix" | "arcolinux" => {
            PackageManager::Pacman
        }
        "fedora" | "rhel" | "centos" | "rocky" | "almalinux" | "alma" | "nobara" | "ol"
        | "amzn" => PackageManager::Dnf,
        "debian" | "ubuntu" | "kali" | "linuxmint" | "mint" | "pop" | "raspbian" | "neon"
        | "elementary" | "zorin" | "mx" | "deepin" => PackageManager::Apt,
        "alpine" | "postmarketos" => PackageManager::Apk,
        "opensuse" | "opensuse-leap" | "opensuse-tumbleweed" | "sles" | "suse" | "suse-microos" => {
            PackageManager::Zypper
        }
        "void" => PackageManager::Xbps,
        "gentoo" | "funtoo" => PackageManager::Emerge,
        _ => return None,
    })
}

/// Fallback for distros whose `os-release` ID ntc does not know.
fn probe_package_manager() -> PackageManager {
    for (binary, manager) in [
        ("pacman", PackageManager::Pacman),
        ("dnf", PackageManager::Dnf),
        ("apt", PackageManager::Apt),
        ("apk", PackageManager::Apk),
        ("zypper", PackageManager::Zypper),
        ("xbps-install", PackageManager::Xbps),
        ("emerge", PackageManager::Emerge),
    ] {
        if super::is_installed(binary) {
            return manager;
        }
    }
    PackageManager::Unknown
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supports_only_platform_native_shells() {
        assert_eq!(supports_shell("powershell"), cfg!(windows));
        assert_eq!(supports_shell("pwsh"), cfg!(windows));
        assert_eq!(supports_shell("bash"), is_bash_platform());
        assert_eq!(supports_shell("zsh"), cfg!(target_os = "macos"));
        assert_eq!(supports_shell("zsh"), is_macos());
        // Everything else keeps the old installed-only behavior.
        for name in ["cmd", "sh", "fish", "git"] {
            assert!(supports_shell(name), "{name}");
        }
    }

    #[test]
    fn termux_is_a_bash_platform_with_pkg() {
        assert_eq!(is_bash_platform(), is_desktop_linux() || is_termux());
        if is_termux() {
            // Termux is not desktop Linux but has bash, so the prompt shows
            // `ntc termux [dir]>` and installs come from `pkg`.
            assert!(!is_desktop_linux());
            assert_eq!(prompt_label(), "termux");
            let distro = distro().expect("termux distro");
            assert_eq!(distro.package_manager.command(), "pkg");
        }
    }

    #[test]
    fn android_markers_reject_termux_and_android() {
        assert!(!android_like(false, None, false, false, false));
        assert!(android_like(true, None, false, false, false));
        assert!(android_like(
            false,
            Some("/data/data/com.termux/files/usr"),
            false,
            false,
            false
        ));
        assert!(android_like(false, None, true, false, false));
        assert!(android_like(false, None, false, true, false));
        assert!(android_like(false, None, false, false, true));
        assert!(!android_like(false, Some("/usr"), false, false, false));
    }

    #[test]
    fn os_release_parsing_handles_quotes_and_comments() {
        let fields = parse_os_release(
            "# comment\nNAME=\"Kali GNU/Linux\"\nID=kali\nID_LIKE=\"debian\"\nPRETTY_NAME='Kali GNU/Linux 2026.1'\n",
        );
        assert_eq!(
            fields.get("NAME").map(String::as_str),
            Some("Kali GNU/Linux")
        );
        assert_eq!(fields.get("ID").map(String::as_str), Some("kali"));
        assert_eq!(fields.get("ID_LIKE").map(String::as_str), Some("debian"));
        assert_eq!(
            fields.get("PRETTY_NAME").map(String::as_str),
            Some("Kali GNU/Linux 2026.1")
        );
    }

    #[test]
    fn distro_ids_map_to_package_managers() {
        let cases = [
            ("arch", "", PackageManager::Pacman),
            ("manjaro", "arch", PackageManager::Pacman),
            ("fedora", "", PackageManager::Dnf),
            ("centos", "rhel fedora", PackageManager::Dnf),
            ("debian", "", PackageManager::Apt),
            ("ubuntu", "debian", PackageManager::Apt),
            ("kali", "debian", PackageManager::Apt),
            ("alpine", "", PackageManager::Apk),
            (
                "opensuse-tumbleweed",
                "opensuse suse",
                PackageManager::Zypper,
            ),
            ("void", "", PackageManager::Xbps),
            ("gentoo", "", PackageManager::Emerge),
        ];
        for (id, id_like, expected) in cases {
            assert_eq!(ids_to_manager(id, id_like), Some(expected), "{id}");
        }
        assert_eq!(ids_to_manager("plan9", ""), None);
        // A known ID_LIKE wins when the ID itself is unfamiliar.
        assert_eq!(
            ids_to_manager("unknown", "debian"),
            Some(PackageManager::Apt)
        );
    }

    #[test]
    fn prompt_label_is_a_lowercase_distro_id() {
        assert_eq!(normalize_id("Arch"), "arch");
        assert_eq!(normalize_id(" ubuntu "), "ubuntu");
        assert_eq!(normalize_id(""), "linux");
        let label = prompt_label();
        assert!(!label.is_empty());
        assert_eq!(label, label.to_ascii_lowercase());
        if is_desktop_linux() {
            assert_eq!(label, distro().map(|d| d.id.as_str()).unwrap_or("linux"));
        }
    }

    #[test]
    fn bash_completion_commands_cover_known_managers() {
        assert_eq!(
            PackageManager::Pacman.install_bash_completion(),
            Some("sudo pacman -S bash-completion")
        );
        assert_eq!(
            PackageManager::Apt.install_bash_completion(),
            Some("sudo apt install bash-completion")
        );
        assert_eq!(
            PackageManager::Apk.install_bash_completion(),
            Some("sudo apk add bash-completion")
        );
        assert_eq!(
            PackageManager::Pkg.install_bash_completion(),
            Some("pkg install bash-completion")
        );
        assert_eq!(PackageManager::Pkg.command(), "pkg");
        assert_eq!(PackageManager::Unknown.install_bash_completion(), None);
        assert_eq!(PackageManager::Unknown.command(), "unknown");
    }
}
