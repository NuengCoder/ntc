// src/shell/commands/git.rs
// Built-in `git` command: pretty-printed status/branch/log, passthrough for
// everything else (commit, add, push, pull, clone, ...).

use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::shell::helpers::run_with_spinner;
use crate::t;
use crate::utils::git as gitmod;

use anyhow::Result;
use colored::*;

pub fn cmd_git(args: &str, nav: &mut Navigator) -> Result<bool> {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    let subcmd = parts.first().copied().unwrap_or("").trim();
    let subargs = parts.get(1).unwrap_or(&"").trim();

    if !gitmod::git_available() {
        print_error(&t!("git.no_git_binary"));
        return Ok(false);
    }

    match subcmd {
        "" | "status" | "st" => cmd_status(nav),
        "branch" | "b" => cmd_branch(nav),
        "log" | "l" => cmd_log(nav),
        "diff" | "d" => passthrough(nav, &format!("git diff {subargs}")),
        "help" | "h" => print_git_help(),
        other => passthrough(nav, &format!("git {other} {subargs}")),
    }
}

fn passthrough(nav: &Navigator, cmd: &str) -> Result<bool> {
    let cmd = cmd.trim_end();
    print_info(&t!("git.executing", cmd));
    println!();
    let status = super::run_system_command(cmd, nav.current_path());
    println!();
    match status {
        Ok(exit_status) => {
            gitmod::invalidate();
            if exit_status.success() {
                print_success(&t!("git.completed"));
            } else {
                match exit_status.code() {
                    Some(code) => print_error(&t!("git.exit_code", code)),
                    None => print_warning(&t!("git.terminated")),
                }
            }
            Ok(false)
        }
        Err(e) => {
            print_error(&t!("git.failed", e));
            Ok(false)
        }
    }
}

fn cmd_status(nav: &Navigator) -> Result<bool> {
    if !gitmod::is_repo(nav.current_path()) {
        print_error(&t!("git.not_repo"));
        return Ok(false);
    }

    let st = match run_with_spinner(&t!("git.checking_status"), || {
        gitmod::status(nav.current_path())
    }) {
        Some(st) => st,
        None => {
            print_error(&t!("git.status_failed"));
            return Ok(false);
        }
    };

    gitmod::invalidate();
    let theme = crate::utils::theme::ThemeManager::current();

    match &st.branch {
        Some(branch) => {
            let mut head = format!(
                "{} {}",
                t!("git.branch_header").cyan().bold(),
                branch.green().bold()
            );
            if let Some(upstream) = &st.upstream {
                head.push_str(&format!(" {}", t!("git.upstream", upstream).dimmed()));
            }
            if st.ahead > 0 || st.behind > 0 {
                head.push_str(&format!(
                    " [{}]",
                    [
                        (st.ahead > 0).then(|| t!("git.ahead", st.ahead)),
                        (st.behind > 0).then(|| t!("git.behind", st.behind)),
                    ]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>()
                    .join(", ")
                ));
            }
            println!("{head}");
        }
        None => {
            println!("{}", t!("git.detached_header").yellow().bold());
        }
    }
    println!();

    #[cfg(not(target_os = "android"))]
    let print_section = |title: String, items: Vec<(char, String)>| {
        if items.is_empty() {
            return;
        }
        println!("{}", title.yellow().bold());
        for (code, path) in items {
            let color = match code {
                'A' | 'C' => theme.shell.git_added.to_colored(),
                'D' => theme.shell.git_deleted.to_colored(),
                'U' => theme.shell.git_conflict.to_colored(),
                '?' => theme.shell.git_untracked.to_colored(),
                _ => theme.shell.git_modified.to_colored(),
            };
            println!(
                "  {} {}",
                code.to_string().bold().color(color),
                path.dimmed()
            );
        }
        println!();
    };

    #[cfg(target_os = "android")]
    let print_section = |title: String, items: Vec<(char, String)>| {
        if items.is_empty() {
            return;
        }
        println!("{}", title.yellow().bold());
        for (code, path) in items {
            let color_str = match code {
                'A' | 'C' => theme.shell.git_added.to_ansi_fg(),
                'D' => theme.shell.git_deleted.to_ansi_fg(),
                'U' => theme.shell.git_conflict.to_ansi_fg(),
                '?' => theme.shell.git_untracked.to_ansi_fg(),
                _ => theme.shell.git_modified.to_ansi_fg(),
            };
            println!(
                "  {}{}{}\x1b[0m {}",
                color_str,
                code.to_string().bold(),
                "",
                path.dimmed()
            );
        }
        println!();
    };

    print_section(
        t!("git.section_conflicts"),
        st.conflicts.iter().map(|p| ('U', p.clone())).collect(),
    );
    print_section(t!("git.section_staged"), st.staged.clone());
    print_section(t!("git.section_unstaged"), st.unstaged.clone());
    print_section(
        t!("git.section_untracked"),
        st.untracked.iter().map(|p| ('?', p.clone())).collect(),
    );

    if !st.is_dirty() {
        #[cfg(not(target_os = "android"))]
        {
            println!(
                "{}",
                t!("git.clean").color(theme.shell.git_added.to_colored())
            );
        }
        #[cfg(target_os = "android")]
        {
            println!(
                "{}{}\x1b[0m",
                theme.shell.git_added.to_ansi_fg(),
                t!("git.clean")
            );
        }
    }

    Ok(false)
}

fn cmd_branch(nav: &Navigator) -> Result<bool> {
    if !gitmod::is_repo(nav.current_path()) {
        print_error(&t!("git.not_repo"));
        return Ok(false);
    }
    let out = match gitmod::git_output(nav.current_path(), &["branch"]) {
        Some(o) => o,
        None => {
            print_error(&t!("git.branch_failed"));
            return Ok(false);
        }
    };
    let theme = crate::utils::theme::ThemeManager::current();
    for line in out.lines() {
        let is_current = line.starts_with('*');
        let name = line.trim().trim_start_matches("* ").trim();
        if name.is_empty() {
            continue;
        }
        if is_current {
            #[cfg(not(target_os = "android"))]
            {
                println!(
                    "  {} {}",
                    "*".bold().color(theme.shell.git_branch.to_colored()),
                    name.green().bold()
                );
            }
            #[cfg(target_os = "android")]
            {
                println!(
                    "  {}{}\x1b[0m {}",
                    theme.shell.git_branch.to_ansi_fg(),
                    "*".bold(),
                    name.green().bold()
                );
            }
        } else {
            println!("  {}", name.dimmed());
        }
    }
    Ok(false)
}

fn cmd_log(nav: &Navigator) -> Result<bool> {
    if !gitmod::is_repo(nav.current_path()) {
        print_error(&t!("git.not_repo"));
        return Ok(false);
    }
    let out = match gitmod::git_output(nav.current_path(), &["log", "--pretty=format:%h %s", "-20"]) {
        Some(o) => o,
        None => {
            print_error(&t!("git.log_failed"));
            return Ok(false);
        }
    };
    if out.trim().is_empty() {
        print_warning(&t!("git.log_empty"));
        return Ok(false);
    }
    let theme = crate::utils::theme::ThemeManager::current();
    for line in out.lines() {
        if let Some(rest) = line.split_once(' ') {
            #[cfg(not(target_os = "android"))]
            {
                println!(
                    "  {} {}",
                    rest.0.green().bold().color(theme.shell.git_branch.to_colored()),
                    rest.1.dimmed()
                );
            }
            #[cfg(target_os = "android")]
            {
                println!(
                    "  {}{}\x1b[0m {}",
                    theme.shell.git_branch.to_ansi_fg(),
                    rest.0.green().bold(),
                    rest.1.dimmed()
                );
            }
        } else {
            println!("  {}", line.dimmed());
        }
    }
    Ok(false)
}

fn print_git_help() -> Result<bool> {
    println!("{}", t!("git.help_title").cyan().bold());
    println!();
    println!("  {}", t!("git.help_status"));
    println!("  {}", t!("git.help_branch"));
    println!("  {}", t!("git.help_log"));
    println!("  {}", t!("git.help_diff"));
    println!("  {}", t!("git.help_passthrough"));
    println!();
    println!("{}", t!("git.examples_header").green());
    println!("  {}", t!("git.help_example1"));
    println!("  {}", t!("git.help_example2"));
    Ok(false)
}