//! `config` settings: output, depth, toggles, size, and update checks.

use crate::config::Config;
use crate::output::{print_error, print_success, print_warning};
use crate::t;
use crate::utils::paths;
use anyhow::Result;
use std::path::Path;

use super::super::super::support::config_display::on_off;

pub(super) fn config_output(args: &str) -> Result<bool> {
    if args.is_empty() {
        println!(
            "{}",
            t!(
                "msg.setO.status",
                Config::global_get_output_path().display()
            )
        );
    } else {
        Config::global_set_output_path(Path::new(&paths::expand_path_str(args)));
        print_success(&t!(
            "msg.setO.success",
            Config::global_get_output_path().display()
        ));
    }
    Ok(false)
}

pub(super) fn config_depth(args: &str) -> Result<bool> {
    if args.is_empty() {
        println!("{}", t!("msg.setD.status", Config::global_get_max_depth()));
    } else {
        match args.parse::<usize>() {
            Ok(depth) => {
                Config::global_set_max_depth(depth);
                print_success(&t!("msg.setD.success", Config::global_get_max_depth()));
            }
            Err(_) => print_error(&t!("msg.setD.error_parse", args)),
        }
    }
    Ok(false)
}

pub(super) fn config_linenumbers(args: &str) -> Result<bool> {
    if args.is_empty() {
        let state = if Config::global_get_show_line_numbers() {
            "ON"
        } else {
            "OFF"
        };
        println!("{}", t!("msg.setL.status", state));
    } else {
        match Config::parse_on_off(args) {
            Some(state) => {
                Config::global_set_show_line_numbers(state);
                print_success(&t!("msg.setL.success", if state { "ON" } else { "OFF" }));
            }
            None => print_error(&t!("msg.setL.error", args)),
        }
    }
    Ok(false)
}

pub(super) fn config_color(args: &str) -> Result<bool> {
    if args.is_empty() {
        let state = if Config::global_get_color_enabled() {
            "ON"
        } else {
            "OFF"
        };
        println!("{}", t!("msg.setC.status", state));
    } else {
        match Config::parse_on_off(args) {
            Some(state) => {
                Config::global_set_color_enabled(state);
                print_success(&t!("msg.setC.success", if state { "ON" } else { "OFF" }));
            }
            None => print_error(&t!("msg.setC.error", args)),
        }
    }
    Ok(false)
}

pub(super) fn config_autosuggest(args: &str) -> Result<bool> {
    if args.is_empty() {
        let state = if Config::global_get_autosuggest_enabled() {
            "ON"
        } else {
            "OFF"
        };
        println!("{}", t!("msg.setA.status", state));
    } else {
        let upper = args.to_uppercase();
        match upper.as_str() {
            "ON" => {
                Config::global_set_autosuggest_enabled(true);
                print_success(&t!("msg.setA.on"));
            }
            "OFF" => {
                Config::global_set_autosuggest_enabled(false);
                print_warning(&t!("msg.setA.off"));
            }
            _ => print_error(&t!("msg.setA.error", args)),
        }
    }
    Ok(false)
}

/// `config idle [on|off]` (also `ghost_typing`, `gt`): the idle ghost typing
/// switch — what the empty prompt types out after you stop touching it.
pub(super) fn config_idle_hint(args: &str) -> Result<bool> {
    if args.is_empty() {
        let state = if Config::global_get_idle_hint_enabled() {
            "ON"
        } else {
            "OFF"
        };
        println!("{}", t!("msg.setIdle.status", state));
    } else {
        match args.to_uppercase().as_str() {
            "ON" => {
                Config::global_set_idle_hint_enabled(true);
                print_success(&t!("msg.setIdle.on"));
            }
            "OFF" => {
                Config::global_set_idle_hint_enabled(false);
                print_warning(&t!("msg.setIdle.off"));
            }
            _ => print_error(&t!("msg.setIdle.error", args)),
        }
    }
    Ok(false)
}

pub(super) fn config_integration_shell(args: &str) -> Result<bool> {
    if args.is_empty() {
        let state = if Config::global_get_integration_shell_enabled() {
            "ON"
        } else {
            "OFF"
        };
        println!("{}", t!("msg.setIS.status", state));
        print_integration_platform_status();
    } else {
        match args.to_uppercase().as_str() {
            "ON" => {
                Config::global_set_integration_shell_enabled(true);
                print_success(&t!("msg.setIS.on"));
                warn_missing_bash_completion();
            }
            "OFF" => {
                Config::global_set_integration_shell_enabled(false);
                print_warning(&t!("msg.setIS.off"));
            }
            _ => print_error(&t!("msg.setIS.error", args)),
        }
    }
    Ok(false)
}

/// Platform details for the integrated shell: the macOS package manager, or
/// the detected distro (Termux included) plus whether bash argument hints
/// have `bash-completion`. Silent on other platforms.
fn print_integration_platform_status() {
    use crate::shell::ext_complete::{bash_completion_available, is_installed, platform};

    if platform::is_macos() {
        let package_manager = if is_installed("brew") {
            "brew"
        } else {
            "unknown"
        };
        println!("{}", t!("msg.setIS.platform", "macOS", package_manager));
        return;
    }
    if !platform::is_bash_platform() {
        return;
    }
    if let Some(distro) = platform::distro() {
        println!(
            "{}",
            t!(
                "msg.setIS.platform",
                distro.name,
                distro.package_manager.command()
            )
        );
    }
    if !is_installed("bash") {
        return;
    }
    if bash_completion_available() {
        println!("{}", t!("msg.setIS.bash_completion_ok"));
        return;
    }
    print_warning(&t!("msg.setIS.bash_completion_missing"));
    if let Some(command) = crate::shell::ext_complete::bash_completion_install_hint() {
        println!("  {}", t!("msg.setIS.bash_completion_install", command));
    }
}

/// On `config integration_shell ON`, explain why bash arguments will not
/// complete on this machine and how to fix it.
fn warn_missing_bash_completion() {
    let Some(command) = crate::shell::ext_complete::bash_completion_install_hint() else {
        return;
    };
    print_warning(&t!("msg.setIS.bash_completion_missing"));
    println!("  {}", t!("msg.setIS.bash_completion_install", command));
}

pub(super) fn config_code_tools(args: &str) -> Result<bool> {
    if args.is_empty() {
        let state = if Config::global_get_code_tools_enabled() {
            "ON"
        } else {
            "OFF"
        };
        println!("{}", t!("msg.setCT.status", state));
    } else {
        match args.to_uppercase().as_str() {
            "ON" => {
                Config::global_set_code_tools_enabled(true);
                print_success(&t!("msg.setCT.on"));
            }
            "OFF" => {
                Config::global_set_code_tools_enabled(false);
                print_warning(&t!("msg.setCT.off"));
            }
            _ => print_error(&t!("msg.setCT.error", args)),
        }
    }
    Ok(false)
}

pub(super) fn config_git(args: &str) -> Result<bool> {
    if args.is_empty() {
        let state = if Config::global_get_git_integration_enabled() {
            "ON"
        } else {
            "OFF"
        };
        println!("{}", t!("msg.setGit.status", state));
    } else {
        let upper = args.to_uppercase();
        match upper.as_str() {
            "ON" => {
                Config::global_set_git_integration_enabled(true);
                print_success(&t!("msg.setGit.on"));
            }
            "OFF" => {
                Config::global_set_git_integration_enabled(false);
                print_warning(&t!("msg.setGit.off"));
            }
            _ => print_error(&t!("msg.setGit.error", args)),
        }
    }
    Ok(false)
}

pub(super) fn config_intro_view(args: &str) -> Result<bool> {
    if args.trim().is_empty() {
        let state = if Config::global_get_intro_view_enabled() {
            "ON"
        } else {
            "OFF"
        };
        println!("{}", t!("msg.setIV.status", state));
    } else {
        match Config::parse_on_off(args) {
            Some(state) => {
                Config::global_set_intro_view_enabled(state);
                if state {
                    print_success(&t!("msg.setIV.on"));
                } else {
                    print_warning(&t!("msg.setIV.off"));
                }
            }
            None => print_error(&t!("msg.setIV.error")),
        }
    }
    Ok(false)
}

pub(super) fn config_dir_view(args: &str) -> Result<bool> {
    if args.is_empty() {
        let state = if Config::global_get_dir_view_enabled() {
            "ON"
        } else {
            "OFF"
        };
        println!("{}", t!("msg.setDV.status", state));
    } else {
        let upper = args.to_uppercase();
        match upper.as_str() {
            "ON" => {
                Config::global_set_dir_view_enabled(true);
                print_success(&t!("msg.setDV.on"));
            }
            "OFF" => {
                Config::global_set_dir_view_enabled(false);
                print_warning(&t!("msg.setDV.off"));
            }
            _ => print_error(&t!("msg.setDV.error")),
        }
    }
    Ok(false)
}

pub(super) fn config_file_view(args: &str) -> Result<bool> {
    if args.trim().is_empty() {
        println!(
            "{}",
            t!(
                "msg.setFV.status",
                on_off(Config::global_get_file_view_enabled())
            )
        );
    } else {
        match Config::parse_on_off(args.trim()) {
            Some(state) => {
                Config::global_set_file_view_enabled(state);
                if state {
                    print_success(&t!("msg.setFV.on"));
                } else {
                    print_warning(&t!("msg.setFV.off"));
                }
            }
            None => print_error(&t!("msg.setFV.error")),
        }
    }
    Ok(false)
}

pub(super) fn config_todo(args: &str) -> Result<bool> {
    let mut parts = args.split_whitespace();
    let action = parts.next().unwrap_or("");
    if action.is_empty() {
        println!(
            "{}",
            t!("msg.setTD.status", Config::global_get_todo_format())
        );
        return Ok(false);
    }
    if !matches!(action, "format" | "fmt") {
        print_error(&t!("msg.setTD.error"));
        return Ok(false);
    }
    let Some(fmt) = parts.next() else {
        print_error(&t!("msg.setTD.error"));
        return Ok(false);
    };
    if parts.next().is_some() {
        print_error(&t!("msg.setTD.error"));
        return Ok(false);
    }
    let fmt = fmt.to_lowercase();
    if !crate::config::VALID_TODO_FORMATS.contains(&fmt.as_str()) {
        print_error(&t!("cmd.todo.unknown_format", fmt));
        return Ok(false);
    }
    Config::global_set_todo_format(&fmt);
    print_success(&t!("msg.setTD.set", fmt, fmt));
    Ok(false)
}

pub(super) fn config_size(args: &str) -> Result<bool> {
    let mut files = Config::global_get_auto_size_files();
    let mut dirs = Config::global_get_auto_size_dirs();

    match parse_size_action(args) {
        Some(SizeAction::ShowAll) => {
            println!("{}", t!("msg.setSZ.status", on_off(files), on_off(dirs)));
            println!("  {}", t!("msg.setSZ.usage"));
        }
        Some(SizeAction::ShowTarget(target)) => {
            let state = match target {
                SizeTarget::All => files && dirs,
                SizeTarget::Files => files,
                SizeTarget::Dirs => dirs,
            };
            println!(
                "{}",
                t!("msg.setSZ.target_status", target.label(), on_off(state))
            );
        }
        Some(SizeAction::Set { target, state }) => {
            let prev_files = files;
            let prev_dirs = dirs;
            let enabled_now = match target {
                SizeTarget::All => {
                    files = state;
                    dirs = state;
                    state && (!prev_files || !prev_dirs)
                }
                SizeTarget::Files => {
                    files = state;
                    state && !prev_files
                }
                SizeTarget::Dirs => {
                    dirs = state;
                    state && !prev_dirs
                }
            };
            Config::global_set_auto_size(files, dirs);
            if state {
                print_success(&t!("msg.setSZ.on", target.label()));
            } else {
                print_warning(&t!("msg.setSZ.off", target.label()));
            }
            if enabled_now {
                print_warning(&t!("msg.setSZ.slow_warn"));
            }
        }
        None => print_error(&t!("msg.setSZ.error")),
    }
    Ok(false)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SizeTarget {
    All,
    Files,
    Dirs,
}

impl SizeTarget {
    fn label(self) -> &'static str {
        match self {
            SizeTarget::All => "all",
            SizeTarget::Files => "file",
            SizeTarget::Dirs => "dir",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SizeAction {
    ShowAll,
    ShowTarget(SizeTarget),
    Set { target: SizeTarget, state: bool },
}

fn parse_size_target(token: &str) -> Option<SizeTarget> {
    match token.to_lowercase().as_str() {
        "all" | "both" => Some(SizeTarget::All),
        "file" | "files" => Some(SizeTarget::Files),
        "dir" | "dirs" | "directory" | "directories" => Some(SizeTarget::Dirs),
        _ => None,
    }
}

pub(super) fn parse_size_action(args: &str) -> Option<SizeAction> {
    let tokens: Vec<String> = args
        .split_whitespace()
        .map(|token| token.to_lowercase())
        .collect();
    match tokens.as_slice() {
        [] => Some(SizeAction::ShowAll),
        [one] => {
            if let Some(state) = Config::parse_on_off(one) {
                Some(SizeAction::Set {
                    target: SizeTarget::All,
                    state,
                })
            } else {
                parse_size_target(one).map(SizeAction::ShowTarget)
            }
        }
        [target, state] => {
            let target = parse_size_target(target)?;
            let state = Config::parse_on_off(state)?;
            Some(SizeAction::Set { target, state })
        }
        _ => None,
    }
}

pub(super) fn config_update_check(args: &str) -> Result<bool> {
    if args.is_empty() {
        let state = if Config::global_get_update_check_enabled() {
            "ON"
        } else {
            "OFF"
        };
        println!("{}", t!("msg.setUC.status", state));
    } else {
        let upper = args.to_uppercase();
        match upper.as_str() {
            "ON" => {
                Config::global_set_update_check_enabled(true);
                print_success(&t!("msg.setUC.on"));
            }
            "OFF" => {
                Config::global_set_update_check_enabled(false);
                print_warning(&t!("msg.setUC.off"));
            }
            _ => print_error(&t!("msg.setUC.error")),
        }
    }
    Ok(false)
}
