//! Delegated completion for other shells (`powershell`, `bash`, `fish`).
//!
//! The launcher's *argument* is treated as a shell command line and handed
//! to that shell's own completion engine, then the answers replace the last
//! token of the line ntc already has. Launcher flags (`bash -`, `cmd /C`,
//! `powershell -NoProfile`) are completed from a static table instead.
//!
//! Providers degrade to "no candidates" instead of erroring: an unfamiliar
//! distro should never make the prompt unresponsive. `sh`/`cmd` have no
//! argument engine — their engines need a real console session; ntc's file
//! completion covers everything after the launcher. `zsh` is hinted on macOS
//! only and is served by the integrated `ntc zsh` host instead; here it still
//! completes its launcher switches.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use super::{nonempty_lines, platform, tokens_to_lines};

const SHELL_CACHE_TTL: Duration = Duration::from_secs(2);

/// Foreign shells controlled by `config integration_shell`. Only names that
/// are actually installed on `PATH` are hinted.
pub(crate) const NAMES: &[&str] = &["powershell", "pwsh", "cmd", "bash", "sh", "zsh", "fish"];

const POWERSHELL_FLAGS: &[&str] = &[
    "-NoProfile",
    "-NoLogo",
    "-NoExit",
    "-NonInteractive",
    "-ExecutionPolicy",
    "-Command",
    "-File",
    "-EncodedCommand",
    "-WindowStyle",
    "-WorkingDirectory",
    "-InputFormat",
    "-OutputFormat",
    "-ConfigurationName",
    "-Version",
    "-Sta",
    "-Mta",
    "-Login",
    "-Help",
    "-?",
];

const BASH_FLAGS: &[&str] = &[
    "-c",
    "-i",
    "-l",
    "--login",
    "-e",
    "-x",
    "-u",
    "-v",
    "-n",
    "-o",
    "-r",
    "--norc",
    "--noprofile",
    "--version",
    "--help",
];

const SH_FLAGS: &[&str] = &[
    "-c",
    "-i",
    "-l",
    "-e",
    "-x",
    "-u",
    "-v",
    "-n",
    "--help",
    "--version",
];

const ZSH_FLAGS: &[&str] = &[
    "-c",
    "-i",
    "-l",
    "-d",
    "-f",
    "-e",
    "-x",
    "-v",
    "-n",
    "--help",
    "--version",
];

const FISH_FLAGS: &[&str] = &[
    "-c",
    "-i",
    "-l",
    "-P",
    "-C",
    "-d",
    "-f",
    "-n",
    "-p",
    "-v",
    "--help",
    "--version",
];

/// Windows `cmd` uses `/` for its switches.
const CMD_FLAGS: &[&str] = &[
    "/C", "/K", "/S", "/Q", "/D", "/A", "/U", "/E:ON", "/E:OFF", "/F:ON", "/F:OFF", "/V:ON",
    "/V:OFF", "/T:", "/HELP",
];

pub(crate) fn owns(cmd: &str) -> bool {
    NAMES.contains(&cmd) && platform::supports_shell(cmd)
}

/// Curated shell names present on `PATH` and supported on this platform, in
/// registry order.
pub(crate) fn installed_names() -> Vec<String> {
    NAMES
        .iter()
        .filter(|name| platform::supports_shell(name) && super::is_installed(name))
        .map(|name| name.to_string())
        .collect()
}

pub(crate) fn invalidate() {
    if let Ok(mut c) = cache().lock() {
        c.clear();
    }
}

fn cache() -> &'static Mutex<HashMap<(String, usize), (Instant, Vec<String>)>> {
    static CACHE: OnceLock<Mutex<HashMap<(String, usize), (Instant, Vec<String>)>>> =
        OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// True when the bash-completion scripts are installed. The standard entry
/// points are the same ones bash itself sources (and the ones the delegation
/// script below tries); Termux keeps them under `$PREFIX`. Not cached: a few
/// `stat` calls are cheap and this way a package installed while ntc is
/// running is picked up immediately.
pub(crate) fn bash_completion_available() -> bool {
    let mut paths: Vec<std::path::PathBuf> = vec![
        "/usr/share/bash-completion/bash_completion".into(),
        "/etc/bash_completion".into(),
        "/usr/local/share/bash-completion/bash_completion".into(),
    ];
    if let Some(prefix) = std::env::var_os("PREFIX") {
        paths.push(std::path::Path::new(&prefix).join("share/bash-completion/bash_completion"));
    }
    paths.iter().any(|path| path.is_file())
}

/// The distro-specific command that installs `bash-completion`, when bash is
/// installed on desktop Linux or Termux and the scripts are missing.
pub(crate) fn bash_completion_install_hint() -> Option<&'static str> {
    if !platform::is_bash_platform() || !super::is_installed("bash") {
        return None;
    }
    if bash_completion_available() {
        return None;
    }
    platform::distro()?
        .package_manager
        .install_bash_completion()
}

/// `before` is the whole active segment (`fish git ch` included).
pub(crate) fn complete(shell: &str, before: &str, pos: usize) -> Option<Vec<String>> {
    if !platform::supports_shell(shell) {
        return None;
    }
    // Launcher switches are cheap and never reach the shell engine.
    if let Some(flags) = launcher_flags(shell, before) {
        return Some(flags);
    }
    if !matches!(shell, "bash" | "fish") {
        return None;
    }

    let key = (before.to_string(), pos);
    if let Ok(c) = cache().lock() {
        if let Some((at, lines)) = c.get(&key) {
            if at.elapsed() < SHELL_CACHE_TTL {
                return Some(lines.clone());
            }
        }
    }
    let inner = argument_line(before)?;
    if inner.is_empty() {
        return None;
    }
    let tokens = match shell {
        "fish" => fish_tokens(inner)?,
        "bash" => bash_tokens(inner)?,
        _ => return None,
    };
    let lines = tokens_to_lines(before, tokens);
    if let Ok(mut c) = cache().lock() {
        if c.len() > 64 {
            c.clear();
        }
        c.insert(key, (Instant::now(), lines.clone()));
    }
    Some(lines)
}

/// Static launcher switches for `shell`, offered while the fragment being
/// typed starts with `-` (or `/` for `cmd`). `None` when the fragment is not
/// a switch, so value-taking flags keep delegating (`powershell -Command
/// Get-Chi` still completes the command text).
fn launcher_flags(shell: &str, before: &str) -> Option<Vec<String>> {
    let tail = argument_tail(before)?;
    if before.ends_with(char::is_whitespace) {
        return None;
    }
    let frag = tail.rsplit(char::is_whitespace).next().unwrap_or(tail);
    let flag_char = if shell == "cmd" { '/' } else { '-' };
    if !frag.starts_with(flag_char) {
        return None;
    }
    let flags: &[&str] = match shell {
        "powershell" | "pwsh" => POWERSHELL_FLAGS,
        "bash" => BASH_FLAGS,
        "sh" => SH_FLAGS,
        "zsh" => ZSH_FLAGS,
        "fish" => FISH_FLAGS,
        "cmd" => CMD_FLAGS,
        _ => return None,
    };
    Some(tokens_to_lines(
        before,
        flags.iter().map(|s| s.to_string()).collect(),
    ))
}

/// Text after the launcher word, trimmed (flags included).
fn argument_tail(before: &str) -> Option<&str> {
    let bytes = before.as_bytes();
    let first = bytes.iter().position(|b| !b.is_ascii_whitespace())?;
    let end = before[first..]
        .find(char::is_whitespace)
        .map(|i| first + i)
        .unwrap_or(before.len());
    let tail = before[end..].trim_start();
    if tail.is_empty() {
        None
    } else {
        Some(tail)
    }
}

/// Text after the launcher word, trimmed. `None` when there is nothing to
/// delegate (flags belong to the launcher, e.g. `bash -c …`).
fn argument_line(before: &str) -> Option<&str> {
    let tail = argument_tail(before)?;
    if tail.starts_with('-') {
        return None;
    }
    Some(tail)
}

fn bash_tokens(inner: &str) -> Option<Vec<String>> {
    if !bash_completion_available() {
        return None;
    }
    const SCRIPT: &str = r#"
source /usr/share/bash-completion/bash_completion 2>/dev/null || source /etc/bash_completion 2>/dev/null
COMP_LINE="$NTC_LINE"
COMP_POINT=${#COMP_LINE}
read -ra COMP_WORDS <<< "$COMP_LINE"
# A trailing space means an empty word is being completed (`git `), which
# `read` cannot represent.
[ -n "$COMP_LINE" ] && [ "${COMP_LINE: -1}" = " " ] && COMP_WORDS+=("")
cmd=${COMP_WORDS[0]}
spec=$(complete -p "$cmd" 2>/dev/null) || exit 0
fn=$(printf '%s\n' "$spec" | sed -n 's/.*-F[[:space:]]\{1,\}\([^[:space:]]*\).*/\1/p')
[ -z "$fn" ] && exit 0
n=${#COMP_WORDS[@]}
cur=${COMP_WORDS[$((n-1))]}
prev=""
[ "$n" -ge 2 ] && prev=${COMP_WORDS[$((n-2))]}
COMPREPLY=()
"$fn" "$cmd" "$cur" "$prev" >/dev/null 2>&1
printf '%s\n' "${COMPREPLY[@]}"
"#;
    let out = run_with_line("bash", &["-c", SCRIPT], inner)?;
    Some(nonempty_lines(&out))
}

fn fish_tokens(inner: &str) -> Option<Vec<String>> {
    const SCRIPT: &str = r#"complete -C "$NTC_LINE""#;
    let out = run_with_line("fish", &["-c", SCRIPT], inner)?;
    let mut tokens = Vec::new();
    for line in out.lines() {
        let line = line.trim_end();
        if line.is_empty() {
            continue;
        }
        // fish separates the completion from its description with a tab.
        let token = line.split('\t').next().unwrap_or(line);
        if !token.is_empty() {
            tokens.push(token.to_string());
        }
    }
    if tokens.is_empty() {
        return None;
    }
    Some(tokens)
}

fn run_with_line(program: &str, args: &[&str], line: &str) -> Option<String> {
    let out = std::process::Command::new(program)
        .args(args)
        .env("NTC_LINE", line)
        .stdin(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    String::from_utf8(out.stdout).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_covers_all_supported_shells() {
        for name in ["powershell", "pwsh", "cmd", "bash", "sh", "zsh", "fish"] {
            assert_eq!(owns(name), platform::supports_shell(name), "{name}");
        }
        assert!(!owns("git"));
        assert!(!owns("powershellx"));
        // Platform gates: PowerShell is Windows-only, bash needs a bash
        // platform (desktop Linux or Termux) and zsh is macOS-only.
        assert_eq!(owns("powershell"), cfg!(windows));
        assert_eq!(owns("bash"), platform::is_bash_platform());
        assert_eq!(owns("zsh"), platform::is_macos());
        assert!(owns("sh"));
    }

    #[test]
    fn bash_hints_need_a_bash_platform() {
        if platform::is_bash_platform() {
            return;
        }
        assert!(!owns("bash"));
        assert!(complete("bash", "bash git ch", 11).is_none());
        assert!(bash_completion_install_hint().is_none());
    }

    #[test]
    fn launcher_flags_use_the_right_switch_character() {
        let flags = launcher_flags("cmd", "cmd /").expect("cmd switches");
        assert!(flags.contains(&"cmd /C".to_string()), "{flags:?}");
        let flags = launcher_flags("bash", "bash --").expect("bash flags");
        assert!(flags.contains(&"bash --norc".to_string()), "{flags:?}");
        let flags = launcher_flags("powershell", "powershell -NoP").expect("ps flags");
        assert!(
            flags.contains(&"powershell -NoProfile".to_string()),
            "{flags:?}"
        );
        // Non-switch arguments belong to the engine, not the flag table.
        assert!(launcher_flags("bash", "bash git").is_none());
        assert!(launcher_flags("cmd", "cmd echo").is_none());
        // A value-taking launcher flag must keep delegating its value.
        assert!(launcher_flags("powershell", "powershell -Command Get-Chi").is_none());
        assert!(launcher_flags("bash", "bash -c git").is_none());
    }

    #[test]
    fn zsh_sh_and_cmd_do_not_delegate_arguments() {
        // zsh is served by the integrated host on macOS, not by this engine.
        assert!(complete("zsh", "zsh ls", 6).is_none());
        assert!(complete("sh", "sh ls", 5).is_none());
        assert!(complete("cmd", "cmd echo", 8).is_none());
    }

    #[test]
    fn zsh_launcher_switches_still_complete() {
        if !platform::is_macos() {
            return;
        }
        let flags = launcher_flags("zsh", "zsh -").expect("zsh switches");
        assert!(flags.contains(&"zsh -c".to_string()), "{flags:?}");
    }
}
