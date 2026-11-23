//! `git status` rendering.

use crate::navigator::Navigator;
use crate::output::print_error;
use crate::shell::helpers::run_with_spinner;
use crate::t;
use crate::utils::git as gitmod;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

pub(super) fn cmd_status(nav: &Navigator) -> Result<bool> {
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

    let print_section = |title: String, items: Vec<(char, String)>| {
        if items.is_empty() {
            return;
        }
        println!(
            "{}",
            theme
                .shell
                .paint_bold(crate::utils::theme::ShellRole::HelpSection, &title)
        );
        for (code, path) in items {
            let color = match code {
                'A' | 'C' => theme.shell.git_added,
                'D' => theme.shell.git_deleted,
                'U' => theme.shell.git_conflict,
                '?' => theme.shell.git_untracked,
                _ => theme.shell.git_modified,
            };
            println!(
                "  {} {}",
                color.paint_ansi_bold(&code.to_string()),
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
        println!("{}", theme.shell.git_added.paint_ansi(&t!("git.clean")));
    }

    Ok(false)
}
