//! `doctor` — check the parts of an ntc setup that break silently.
//!
//! Every check is local and read-only: no network calls and no files are
//! created outside a one-file probe that is removed again. The report is a
//! flat list so it stays readable in a terminal and useful in a bug report.

pub(crate) mod help;

use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;
use std::path::Path;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) fn cmd_doctor(_args: &str, nav: &mut Navigator) -> Result<bool> {
    println!("{}", t!("doctor.title").cyan().bold());
    println!();

    // Informational header: what is running, where.
    print_info(&t!(
        "doctor.platform",
        env!("CARGO_PKG_VERSION"),
        env!("BUILD_GIT_HASH"),
        std::env::consts::OS,
        std::env::consts::ARCH
    ));

    let mut issues = 0;
    issues += check_config();
    issues += check_install();
    issues += check_ran(nav);
    issues += check_ntpkg();

    println!();
    if issues == 0 {
        print_success(&t!("doctor.ok"));
    } else {
        print_warning(&t!("doctor.issues", issues));
    }
    Ok(false)
}

/// Config directory: exists, writable, and which language/theme it selects.
fn check_config() -> usize {
    let Some(dir) = dirs::config_dir().map(|d| d.join("ntc")) else {
        print_error(&t!("doctor.no_config_dir"));
        return 1;
    };
    let config_file = dir.join("config.toml");

    if !dir.is_dir() {
        print_info(&t!("doctor.config_new", dir.display()));
        return 0;
    }

    // A probe file is the only reliable writability test on Windows, where a
    // read-only attribute or ACL shows up as a failed write, not as metadata.
    let probe = dir.join(".ntc_doctor_probe");
    match std::fs::write(&probe, b"") {
        Ok(_) => {
            let _ = std::fs::remove_file(&probe);
            print_success(&t!("doctor.config_ok", config_file.display()));
            print_info(&t!(
                "doctor.settings",
                crate::utils::config::Config::global_get_language(),
                crate::utils::theme::ThemeManager::current().name.clone()
            ));
            0
        }
        Err(e) => {
            print_error(&t!("doctor.config_unwritable", dir.display(), e));
            1
        }
    }
}

/// Is this `ntc` reachable as a shell command?
fn check_install() -> usize {
    match which::which("ntc") {
        Ok(path) => {
            print_success(&t!("doctor.install", "ntc", path.display()));
            0
        }
        Err(_) => {
            print_warning(&t!("doctor.install_missing", "ntc"));
            1
        }
    }
}

/// The project task runner: a ranfile here, and does it validate?
fn check_ran(nav: &Navigator) -> usize {
    use super::ran::ranfile::{parser, types::RanFormat};

    let cwd = nav.current_path();
    let Some((path, fmt)) = parser::resolve(cwd) else {
        print_info(&t!("doctor.ran_none", cwd.display()));
        return 0;
    };
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.display().to_string());

    match parser::parse_file(&path) {
        Ok(ranfile) => {
            let validator: fn(&str) -> Vec<super::super::support::check::Diagnostic> = match fmt {
                RanFormat::Toml => super::super::support::check::formats::ran,
                RanFormat::Json => super::super::support::check::formats::ran_json,
                RanFormat::Yaml => super::super::support::check::formats::ran_yaml,
            };
            let source = std::fs::read_to_string(&path).unwrap_or_default();
            let problems = validator(&source).len();
            if problems == 0 {
                print_success(&t!("doctor.ran_ok", name, ranfile.targets.len()));
                print_info(&t!("doctor.ran_env", ranfile.env.len(), ranfile.vars.len()));
                0
            } else {
                print_warning(&t!("doctor.ran_problems", name, problems, "ran check"));
                1
            }
        }
        Err(e) => {
            print_error(&t!("doctor.ran_error", name, e));
            1
        }
    }
}

/// Installed runtime versions, and whether their bin dirs are on PATH.
fn check_ntpkg() -> usize {
    use super::ntpkg::ntpkg::{manager, types};

    if !types::package_dir().is_dir() {
        print_info(&t!("doctor.ntpkg_none"));
        return 0;
    }
    let manifest = match manager::load_manifest() {
        Ok(manifest) => manifest,
        Err(e) => {
            print_error(&t!(
                "doctor.ntpkg_error",
                types::manifest_path().display(),
                e
            ));
            return 1;
        }
    };
    let installed: usize = manifest.installed.values().map(Vec::len).sum();
    if installed == 0 {
        print_info(&t!("doctor.ntpkg_none"));
        return 0;
    }
    print_success(&t!("doctor.ntpkg_ok", installed, manifest.active.len()));

    let missing = manager::active_bin_dirs()
        .unwrap_or_default()
        .into_iter()
        .filter(|dir| !on_path(dir))
        .count();
    if missing == 0 {
        0
    } else {
        print_warning(&t!("doctor.ntpkg_path", missing));
        1
    }
}

/// Whether `dir` is one of the entries in PATH.
fn on_path(dir: &Path) -> bool {
    let Ok(path) = std::env::var("PATH") else {
        return false;
    };
    let sep = if cfg!(windows) { ';' } else { ':' };
    let wanted = trim_separators(dir);
    path.split(sep).any(|entry| {
        let entry = entry.trim().trim_matches('"');
        if entry.is_empty() {
            return false;
        }
        let candidate = trim_separators(Path::new(entry));
        if cfg!(windows) {
            candidate.eq_ignore_ascii_case(&wanted)
        } else {
            candidate == wanted
        }
    })
}

/// Path text without a trailing separator, for comparison.
fn trim_separators(path: &Path) -> String {
    let text = path.to_string_lossy();
    text.trim_end_matches(['/', '\\']).to_string()
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "doctor",
    aliases: &[],
    help_topic: Some("doctor"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_doctor),
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_membership_is_checked_against_real_entries() {
        let path = std::env::var("PATH").unwrap_or_default();
        let sep = if cfg!(windows) { ';' } else { ':' };
        let Some(first) = path.split(sep).find(|e| !e.trim().is_empty()) else {
            return;
        };
        let first = first.trim().trim_matches('"');
        assert!(on_path(Path::new(first)), "PATH entry {first} not found");
    }

    #[test]
    fn trailing_separators_do_not_hide_a_match() {
        assert_eq!(trim_separators(Path::new("C:/x/y/")), "C:/x/y");
        assert_eq!(trim_separators(Path::new("C:\\x\\y\\\\")), "C:\\x\\y");
    }
}
