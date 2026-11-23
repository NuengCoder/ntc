//! Shared `PATH` editing engine behind `usrpath` and `syspath`.
//!
//! * Windows: the registry — `HKCU\Environment` (user) and
//!   `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Environment`
//!   (system).
//! * Linux: `~/.config/ntc/usrpath.sh` for the user level (source it from a
//!   shell profile); for the system level the `PATH` line of
//!   `/etc/environment` — which PAM applies to every session, login or not —
//!   plus an `/etc/profile.d/ntc-syspath.sh` mirror for login shells started
//!   outside a PAM session. Both are root-owned and written through `sudo`
//!   when ntc is not root.
//! * macOS: `~/Library/Application Support/ntc/usrpath.sh` for the user level
//!   (the same `export PATH=` file, sourced from `~/.zprofile`) and
//!   `/etc/paths.d/ntc` for the system level, which `path_helper` reads at
//!   login. `path_helper` only prepends entries, so that file is additive and
//!   never hides `/etc/paths`.
//!
//! Everything else reports that the command is unsupported.

use crate::navigator::Navigator;
use crate::output::print_error;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
use crate::output::{print_info, print_success};
use crate::t;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
use crate::utils::theme::style::ThemeColorize;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
use anyhow::anyhow;
use anyhow::Result;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
use std::path::Path;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::path::PathBuf;

/// Which `PATH` a invocation edits.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Level {
    User,
    System,
}

#[cfg(windows)]
const USER_KEY: &str = "HKCU\\Environment";
#[cfg(windows)]
const SYS_KEY: &str = "HKLM\\SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment";

/// Linux drop-in sourced by login shells on every major distribution.
#[cfg(target_os = "linux")]
const SYSTEM_STORE: &str = "/etc/profile.d/ntc-syspath.sh";

/// Applied by PAM to every session: the system PATH `syspath` edits.
#[cfg(target_os = "linux")]
const SYSTEM_ENV_FILE: &str = "/etc/environment";

/// `usrpath` — edit the per-user `PATH`.
pub(crate) fn run_usrpath(args: &str, nav: &mut Navigator) -> Result<bool> {
    dispatch_path(args, nav, Level::User, "usrpath")
}

/// `syspath` — edit the machine-wide `PATH`.
pub(crate) fn run_syspath(args: &str, nav: &mut Navigator) -> Result<bool> {
    dispatch_path(args, nav, Level::System, "syspath")
}

fn dispatch_path(args: &str, nav: &mut Navigator, level: Level, name: &str) -> Result<bool> {
    #[cfg(any(windows, target_os = "linux", target_os = "macos"))]
    {
        let args = args.trim();
        if args.is_empty() {
            print_usage(name);
            return Ok(false);
        }
        let parts: Vec<&str> = args.splitn(2, ' ').collect();
        let sub = parts[0].to_lowercase();
        let rest = parts.get(1).unwrap_or(&"").trim();
        match sub.as_str() {
            "show" | "ls" => cmd_show(level, name),
            "add" => cmd_add(level, name, rest, nav),
            "rm" | "remove" => cmd_rm(level, rest, name),
            "move" | "mv" => cmd_move(level, rest, name),
            _ => {
                print_usage(name);
                Ok(false)
            }
        }
    }
    #[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
    {
        let _ = (args, nav, level, name);
        print_error(&t!("cmd.pathman.platform_unsupported"));
        Ok(false)
    }
}

#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
fn print_usage(name: &str) {
    println!("{}", t!("cmd.pathman.usage_title").cyan().bold());
    println!("  {}", t!("cmd.pathman.help_show", name));
    println!("  {}", t!("cmd.pathman.help_add", name));
    println!("  {}", t!("cmd.pathman.help_add_cwd", name));
    println!("  {}", t!("cmd.pathman.help_rm", name));
    println!("  {}", t!("cmd.pathman.help_move", name));
    println!();
    println!("{}", t!("cmd.pathman.examples_title").green());
    println!("  {}", t!("cmd.pathman.example_show", name));
    println!("  {}", t!("cmd.pathman.example_add", name));
    println!("  {}", t!("cmd.pathman.example_add_cwd", name));
    println!("  {}", t!("cmd.pathman.example_rm_idx", name));
    println!("  {}", t!("cmd.pathman.example_rm_path", name));
    println!("  {}", t!("cmd.pathman.example_move", name));
}

#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
fn print_level_name(level: Level) -> String {
    match level {
        Level::User => t!("cmd.pathman.user_level"),
        Level::System => t!("cmd.pathman.system_level"),
    }
}

#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
fn cmd_show(level: Level, name: &str) -> Result<bool> {
    let current = read_path(level).unwrap_or_default();
    let entries = split_path(&current);

    println!();
    let level_name = print_level_name(level);
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!(
        "║{:^60}",
        format!("{level_name} PATH ({name})").cyan().bold()
    );
    println!("╠══════════════════════════════════════════════════════════════════╣");

    if entries.is_empty() {
        println!("║  {}", t!("cmd.pathman.empty_path").dimmed());
    } else {
        for (i, entry) in entries.iter().enumerate() {
            println!("║  {:<3} {}", format!("{}", i + 1).yellow(), entry);
        }
    }

    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!(
        "  {}",
        t!("cmd.pathman.total_entries", entries.len()).cyan()
    );
    if let Some(note) = store_note(level) {
        println!("{}", t!("cmd.pathman.store_file", note).dimmed());
    }
    println!();
    Ok(false)
}

/// Where the entries live when that is not obvious from the header (Linux
/// writes a file; Windows writes the registry).
#[cfg(target_os = "linux")]
fn store_note(level: Level) -> Option<String> {
    Some(match level {
        Level::User => user_store().to_string_lossy().into_owned(),
        Level::System => SYSTEM_ENV_FILE.to_string(),
    })
}

#[cfg(target_os = "macos")]
fn store_note(level: Level) -> Option<String> {
    Some(match level {
        Level::User => user_store().to_string_lossy().into_owned(),
        Level::System => SYSTEM_PATHS_FILE.to_string(),
    })
}

#[cfg(windows)]
fn store_note(_level: Level) -> Option<String> {
    None
}

#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
fn cmd_add(level: Level, name: &str, args: &str, nav: &Navigator) -> Result<bool> {
    let dir = if args.eq_ignore_ascii_case("--cwd") || args.eq_ignore_ascii_case("--here") {
        nav.current_path().to_string_lossy().to_string()
    } else if args.is_empty() {
        print_usage(name);
        return Ok(false);
    } else {
        crate::utils::paths::expand_path_str(args)
    };

    let dir_path = Path::new(&dir);
    let dir_display = if dir_path.is_absolute() {
        dir.clone()
    } else {
        nav.current_path().join(&dir).to_string_lossy().to_string()
    };

    let current = read_path(level).unwrap_or_default();
    let mut entries = split_path(&current);

    let normalized_new = normalize_path(&dir_display);

    if entries.iter().any(|e| normalize_path(e) == normalized_new) {
        print_info(&t!("cmd.pathman.already_in_path", dir_display));
        return Ok(false);
    }

    entries.push(dir_display.clone());

    let new_path = join_path(&entries);
    write_path(level, &new_path)?;

    print_success(&t!("cmd.pathman.added_to_path", name, dir_display));
    print_restart_note(level);
    Ok(false)
}

#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
fn cmd_rm(level: Level, args: &str, name: &str) -> Result<bool> {
    if args.is_empty() {
        print_error(&t!("cmd.pathman.rm_usage"));
        return Ok(false);
    }

    let current = read_path(level).unwrap_or_default();
    let mut entries = split_path(&current);

    let removed = if let Ok(idx) = args.parse::<usize>() {
        if idx >= 1 && idx <= entries.len() {
            let entry = entries.remove(idx - 1);
            print_success(&t!("cmd.pathman.removed_from_path", name, entry));
            true
        } else {
            print_error(&t!("cmd.pathman.idx_out_of_range", idx, entries.len()));
            return Ok(false);
        }
    } else {
        let target_normalized = normalize_path(args);
        let before = entries.len();
        entries.retain(|e| normalize_path(e) != target_normalized);
        if entries.len() < before {
            print_success(&t!("cmd.pathman.removed_from_path", name, args));
            true
        } else {
            print_error(&t!("cmd.pathman.not_found_in_path", args));
            return Ok(false);
        }
    };

    let new_path = join_path(&entries);
    write_path(level, &new_path)?;

    if removed {
        print_restart_note(level);
    }
    Ok(false)
}

#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
fn cmd_move(level: Level, args: &str, name: &str) -> Result<bool> {
    let parts: Vec<&str> = args.split_whitespace().collect();
    if parts.len() < 2 {
        print_error(&t!("cmd.pathman.move_usage", name));
        return Ok(false);
    }

    let select = parts[0].parse::<usize>();
    let target = parts[1].parse::<usize>();

    let (select, target) = match (select, target) {
        (Ok(s), Ok(t)) => (s, t),
        _ => {
            print_error(&t!("cmd.pathman.indices_numbers"));
            return Ok(false);
        }
    };

    let current = read_path(level).unwrap_or_default();
    let mut entries = split_path(&current);

    if select < 1 || select > entries.len() {
        print_error(&t!("cmd.pathman.idx_out_of_range", select, entries.len()));
        return Ok(false);
    }
    if target < 1 || target > entries.len() {
        print_error(&t!("cmd.pathman.idx_out_of_range", target, entries.len()));
        return Ok(false);
    }

    if select == target {
        print_info(&t!("cmd.pathman.same_source_target"));
        return Ok(false);
    }

    let select_idx = select - 1;
    let mut target_idx = target - 1;

    let entry = entries.remove(select_idx);
    if select_idx < target_idx {
        target_idx -= 1;
    }
    entries.insert(target_idx, entry.clone());

    let new_path = join_path(&entries);
    write_path(level, &new_path)?;

    print_success(&t!("cmd.pathman.moved", target, name, entry));
    print_restart_note(level);
    Ok(false)
}

#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
fn print_restart_note(level: Level) {
    println!();
    #[cfg(windows)]
    {
        let _ = level;
        print_info(&t!("cmd.pathman.new_terminal"));
    }
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        match level {
            Level::User => print_info(&t!(
                "cmd.pathman.shell_profile_hint",
                format!(". \"{}\"", user_store().to_string_lossy())
            )),
            Level::System => print_info(&t!("cmd.pathman.next_login")),
        }
    }
}

// ============================================================================
// Windows: registry
// ============================================================================

#[cfg(windows)]
fn windows_key(level: Level) -> &'static str {
    match level {
        Level::User => USER_KEY,
        Level::System => SYS_KEY,
    }
}

#[cfg(windows)]
fn read_path(level: Level) -> Result<String> {
    let key = windows_key(level);
    let output = std::process::Command::new("reg")
        .args(["query", key, "/v", "PATH"])
        .output()
        .map_err(|e| anyhow!("{}", t!("cmd.pathman.reg_query_fail", e)))?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        if err.contains("does not exist") || err.contains("not found") {
            return Ok(String::new());
        }
        return Err(anyhow!("{}", t!("cmd.pathman.reg_query_fail", err.trim())));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("PATH") || trimmed.starts_with("path") {
            if let Some(val) = parse_reg_value(trimmed) {
                return Ok(val);
            }
        }
    }

    Ok(String::new())
}

#[cfg(windows)]
fn parse_reg_value(line: &str) -> Option<String> {
    let after_name = line.find(char::is_whitespace)?;
    let rest = line[after_name..].trim_start();
    let after_type = rest.find(char::is_whitespace)?;
    let val = rest[after_type..].trim_start();
    if val.is_empty() {
        None
    } else {
        Some(val.to_string())
    }
}

#[cfg(windows)]
fn write_path(level: Level, path_value: &str) -> Result<()> {
    let output = std::process::Command::new("reg")
        .args([
            "add",
            windows_key(level),
            "/v",
            "PATH",
            "/t",
            "REG_EXPAND_SZ",
            "/d",
            path_value,
            "/f",
        ])
        .output()
        .map_err(|e| anyhow!("{}", t!("cmd.pathman.reg_write_fail", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let err_lower = stderr.to_lowercase();
        if err_lower.contains("access is denied") || err_lower.contains("access denied") {
            println!();
            print_info(&t!("cmd.pathman.run_as_admin"));
            print_info(&t!("cmd.pathman.mode_admin_hint"));
            return Err(anyhow::anyhow!("{}", t!("cmd.pathman.access_denied")));
        }
        return Err(anyhow!(
            "{}",
            t!("cmd.pathman.path_update_fail", stderr.trim())
        ));
    }

    Ok(())
}

// ============================================================================
// Unix: `export PATH=...` files and per-platform system stores
// ============================================================================

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn user_store() -> PathBuf {
    crate::utils::paths::config_home()
        .join("ntc")
        .join("usrpath.sh")
}

#[cfg(target_os = "linux")]
fn store_path(level: Level) -> PathBuf {
    match level {
        Level::User => user_store(),
        Level::System => PathBuf::from(SYSTEM_STORE),
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn read_file(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}

/// Value of the `PATH=` / `export PATH=` line in `content`, empty when absent.
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn parse_path_line(content: &str) -> String {
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('#') {
            continue;
        }
        let line = line.strip_prefix("export ").unwrap_or(line);
        let Some(rest) = line.strip_prefix("PATH=") else {
            continue;
        };
        return unquote(rest.trim());
    }
    String::new()
}

/// `syspath` reads `/etc/environment` first — PAM applies that to every
/// session, so it is the system PATH a shell actually inherits. The
/// `/etc/profile.d` drop-in covers login shells started outside a PAM session,
/// and the PATH already in effect seeds the list while neither file defines
/// one, so the first `add` extends the PATH instead of replacing it.
#[cfg(target_os = "linux")]
fn read_path(level: Level) -> Result<String> {
    match level {
        Level::User => Ok(parse_path_line(&read_file(&user_store()))),
        Level::System => {
            let from_env = parse_path_line(&read_file(Path::new(SYSTEM_ENV_FILE)));
            if !from_env.is_empty() {
                return Ok(from_env);
            }
            let from_drop_in = parse_path_line(&read_file(&store_path(Level::System)));
            if !from_drop_in.is_empty() {
                return Ok(from_drop_in);
            }
            Ok(std::env::var("PATH").unwrap_or_default())
        }
    }
}

/// Reverse [`quote`] for both quoting styles an `export PATH=` line can use.
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn unquote(value: &str) -> String {
    if let Some(inner) = value.strip_prefix('\'').and_then(|v| v.strip_suffix('\'')) {
        return inner.replace("'\\''", "'");
    }
    if let Some(inner) = value.strip_prefix('"').and_then(|v| v.strip_suffix('"')) {
        return inner.replace("\\\"", "\"");
    }
    value.to_string()
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn write_user_store(path_value: &str) -> Result<()> {
    let path = user_store();
    let content = format!(
        "# managed by ntc — `usrpath`\nexport PATH={}\n",
        quote(path_value)
    );
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| anyhow!("{}", t!("cmd.pathman.path_update_fail", e)))?;
    }
    std::fs::write(&path, content).map_err(|e| {
        anyhow!(
            "{}",
            t!(
                "cmd.pathman.path_update_fail",
                format!("{}: {}", path.to_string_lossy(), e)
            )
        )
    })
}

#[cfg(target_os = "linux")]
fn write_path(level: Level, path_value: &str) -> Result<()> {
    if level == Level::User {
        return write_user_store(path_value);
    }

    // A system edit lands in both files: `/etc/environment` for PAM sessions
    // and the profile.d drop-in for login shells started outside one.
    let env_content = with_path_line(&read_file(Path::new(SYSTEM_ENV_FILE)), path_value);
    let drop_in = format!(
        "# managed by ntc — `syspath`\nexport PATH={}\n",
        quote(path_value)
    );

    if !is_root() {
        print_info(&t!(
            "cmd.pathman.sudo_write",
            format!("{SYSTEM_ENV_FILE}, {SYSTEM_STORE}")
        ));
    }
    write_privileged(SYSTEM_ENV_FILE, &env_content)?;
    write_privileged(SYSTEM_STORE, &drop_in)?;
    Ok(())
}

/// Rewrite only the `PATH=` line of `/etc/environment`, or append one. Every
/// other line, comments included, is preserved verbatim.
#[cfg(target_os = "linux")]
fn with_path_line(content: &str, path_value: &str) -> String {
    let line = format!(
        "PATH=\"{}\"",
        path_value.replace('\\', "\\\\").replace('"', "\\\"")
    );
    let mut out = String::new();
    let mut replaced = false;
    for raw in content.lines() {
        let trimmed = raw.trim();
        let unexported = trimmed.strip_prefix("export ").unwrap_or(trimmed);
        if !replaced && !trimmed.starts_with('#') && unexported.starts_with("PATH=") {
            out.push_str(&line);
            replaced = true;
        } else {
            out.push_str(raw);
        }
        out.push('\n');
    }
    if !replaced {
        out.push_str(&line);
        out.push('\n');
    }
    out
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn write_privileged(target: &str, content: &str) -> Result<()> {
    if !is_root() {
        return write_with_sudo(Path::new(target), content);
    }
    if let Some(parent) = Path::new(target).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| anyhow!("{}", t!("cmd.pathman.path_update_fail", e)))?;
    }
    std::fs::write(target, content).map_err(|e| {
        anyhow!(
            "{}",
            t!("cmd.pathman.path_update_fail", format!("{target}: {e}"))
        )
    })
}

/// `sudo tee <path>` — stdin carries the file body, stdout is dropped so the
/// content is not echoed back to the terminal.
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn write_with_sudo(path: &Path, content: &str) -> Result<()> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let mut child = Command::new("sudo")
        .arg("tee")
        .arg(path)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| anyhow!("{}", t!("cmd.pathman.sudo_fail", e)))?;

    if let Some(mut stdin) = child.stdin.take() {
        // A failed write surfaces through the exit status below; writing to a
        // sudo that already exited just yields EPIPE.
        let _ = stdin.write_all(content.as_bytes());
    }

    let output = child
        .wait_with_output()
        .map_err(|e| anyhow!("{}", t!("cmd.pathman.path_update_fail", e)))?;
    if !output.status.success() {
        return Err(anyhow!(
            "{}",
            t!(
                "cmd.pathman.path_update_fail",
                String::from_utf8_lossy(&output.stderr).trim()
            )
        ));
    }
    Ok(())
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn is_root() -> bool {
    // SAFETY: `geteuid` takes no arguments, cannot fail and has no
    // preconditions, so calling it is always sound.
    unsafe { libc::geteuid() == 0 }
}

// ============================================================================
// macOS: /etc/paths.d
// ============================================================================

/// One directory per line, read by `path_helper` at login.
#[cfg(target_os = "macos")]
const SYSTEM_PATHS_FILE: &str = "/etc/paths.d/ntc";

/// `/etc/paths.d` files list one directory per line, while the rest of the
/// engine works on a platform-separated path string.
#[cfg(target_os = "macos")]
fn parse_paths_d(content: &str) -> String {
    content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect::<Vec<_>>()
        .join(":")
}

#[cfg(target_os = "macos")]
fn paths_d_content(path_value: &str) -> String {
    let mut out = String::new();
    for entry in split_path(path_value) {
        out.push_str(&entry);
        out.push('\n');
    }
    out
}

#[cfg(target_os = "macos")]
fn read_path(level: Level) -> Result<String> {
    match level {
        Level::User => Ok(parse_path_line(&read_file(&user_store()))),
        Level::System => Ok(parse_paths_d(&read_file(Path::new(SYSTEM_PATHS_FILE)))),
    }
}

#[cfg(target_os = "macos")]
fn write_path(level: Level, path_value: &str) -> Result<()> {
    if level == Level::User {
        return write_user_store(path_value);
    }
    if !is_root() {
        print_info(&t!("cmd.pathman.sudo_write", SYSTEM_PATHS_FILE));
    }
    write_privileged(SYSTEM_PATHS_FILE, &paths_d_content(path_value))
}

// ============================================================================
// Shared helpers
// ============================================================================

// Only the Windows/Linux/macOS backends build path strings, so the separator
// is scoped to them; other targets (e.g. Android) leave it unused.
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
const PATH_SEP: char = if cfg!(windows) { ';' } else { ':' };

#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
fn split_path(value: &str) -> Vec<String> {
    value
        .split(PATH_SEP)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
fn join_path(entries: &[String]) -> String {
    entries.join(&PATH_SEP.to_string())
}

/// Windows paths compare case-insensitively and may carry either separator;
/// Unix paths are case-sensitive.
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
fn normalize_path(s: &str) -> String {
    let trimmed = s.trim_end_matches('\\').trim_end_matches('/');
    #[cfg(windows)]
    {
        trimmed.to_lowercase()
    }
    #[cfg(not(windows))]
    {
        trimmed.to_string()
    }
}

#[cfg(all(test, windows))]
mod windows_tests {
    use super::*;

    #[test]
    fn splits_and_joins_on_semicolons() {
        let entries = split_path("C:\\a;;C:\\b;");
        assert_eq!(entries, vec!["C:\\a", "C:\\b"]);
        assert_eq!(join_path(&entries), "C:\\a;C:\\b");
    }

    #[test]
    fn normalization_ignores_case_and_trailing_separators() {
        assert_eq!(normalize_path("C:\\Tools\\"), normalize_path("c:\\tools"));
    }
}

#[cfg(all(test, target_os = "macos"))]
mod macos_tests {
    use super::*;

    #[test]
    fn parses_paths_d_lines_into_a_path_string() {
        let content = "# comment\n/opt/homebrew/bin\n\n/usr/local/bin\n";
        assert_eq!(parse_paths_d(content), "/opt/homebrew/bin:/usr/local/bin");
        assert_eq!(parse_paths_d(""), "");
    }

    #[test]
    fn writes_one_entry_per_line() {
        assert_eq!(
            paths_d_content("/opt/homebrew/bin:/usr/local/bin"),
            "/opt/homebrew/bin\n/usr/local/bin\n"
        );
        assert_eq!(paths_d_content(""), "");
        let value = "/opt/a:/opt/b c";
        assert_eq!(parse_paths_d(&paths_d_content(value)), value);
    }
}

#[cfg(all(test, target_os = "linux"))]
mod linux_tests {
    use super::*;

    #[test]
    fn splits_and_joins_on_colons() {
        let entries = split_path("/usr/bin::/opt/bin:");
        assert_eq!(entries, vec!["/usr/bin", "/opt/bin"]);
        assert_eq!(join_path(&entries), "/usr/bin:/opt/bin");
    }

    #[test]
    fn normalization_keeps_case_and_trims_trailing_separator() {
        assert_eq!(normalize_path("/opt/Bin/"), "/opt/Bin");
        assert_ne!(normalize_path("/opt/Bin"), normalize_path("/opt/bin"));
    }

    #[test]
    fn quotes_and_unquotes_round_trip() {
        for value in ["/usr/bin:/bin", "/opt/my tools/bin", "/opt/it's/bin"] {
            assert_eq!(unquote(&quote(value)), value);
        }
        assert_eq!(unquote("\"/opt/x\""), "/opt/x");
    }

    #[test]
    fn user_level_round_trips_through_its_file() {
        let _home = crate::utils::test_support::IsolatedConfigHome::new();
        let entries = vec!["/opt/first".to_string(), "/opt/it's second".to_string()];
        let joined = join_path(&entries);

        write_path(Level::User, &joined).unwrap();

        assert_eq!(read_path(Level::User).unwrap(), joined);
        assert_eq!(split_path(&read_path(Level::User).unwrap()), entries);
        assert!(user_store().exists());
        let hint = std::fs::read_to_string(user_store()).unwrap();
        assert!(hint.contains("`usrpath`"), "unexpected header: {hint}");
    }

    #[test]
    fn missing_store_reads_as_empty() {
        let _home = crate::utils::test_support::IsolatedConfigHome::new();
        assert_eq!(read_path(Level::User).unwrap(), "");
    }

    #[test]
    fn system_env_rewrite_replaces_only_the_path_line() {
        let original = "# managed by hand\nLANG=en_US.UTF-8\nPATH=\"/usr/bin:/bin\"\nEDITOR=vim\n";
        let out = with_path_line(original, "/usr/bin:/opt/x");
        assert_eq!(
            out,
            "# managed by hand\nLANG=en_US.UTF-8\nPATH=\"/usr/bin:/opt/x\"\nEDITOR=vim\n"
        );
        assert_eq!(parse_path_line(&out), "/usr/bin:/opt/x");
    }

    #[test]
    fn system_env_rewrite_appends_when_missing() {
        let out = with_path_line("# parsed by pam_env\n", "/opt/x");
        assert_eq!(out, "# parsed by pam_env\nPATH=\"/opt/x\"\n");
        assert_eq!(parse_path_line(&out), "/opt/x");
    }

    #[test]
    fn path_line_parsing_ignores_comments_and_export() {
        assert_eq!(parse_path_line("export PATH='/a:/bin'"), "/a:/bin");
        assert_eq!(parse_path_line("PATH=\"/a:/bin\"\n"), "/a:/bin");
        assert_eq!(parse_path_line("# PATH=/nope\n"), "");
        assert_eq!(parse_path_line("LANG=C\n"), "");
    }
}
