//! `godrive` — switch drives on Windows, or `/mnt` mounts on Linux.

pub(crate) mod help;

use crate::navigator::{clear_screen, DriveRoot, Navigator};
use crate::output::print_error;
use crate::shell::helpers::show_nav_view;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

/// Accept `c`, `C:`, `c/`, `/mnt/c` and compare by the last path segment.
fn wanted_label(arg: &str) -> String {
    let trimmed = arg.trim().trim_end_matches('/');
    let name = trimmed.rsplit('/').next().unwrap_or(trimmed);
    name.trim_end_matches(':').to_ascii_lowercase()
}

fn go_to_root(nav: &mut Navigator, root: &DriveRoot) -> Result<()> {
    nav.go_to(&root.path)?;
    clear_screen();
    show_nav_view(nav);
    Ok(())
}

pub(crate) fn cmd_godrive(args: &str, nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();
    let roots = Navigator::drive_roots();

    if args.is_empty() {
        println!(
            "{}",
            if cfg!(windows) {
                t!("nav.available_drives")
            } else {
                t!("nav.available_mounts")
            }
            .cyan()
            .bold()
        );
        for (i, root) in roots.iter().enumerate() {
            println!("  {}: {}", i + 1, root.path.display());
        }
        println!();
        let prompt = if cfg!(windows) {
            t!("nav.drive_prompt")
        } else {
            t!("nav.mount_prompt")
        };
        let labels: Vec<String> = roots
            .iter()
            .map(|root| format!("{} ({})", root.label, root.path.display()))
            .collect();
        let Some(choice) = crate::utils::interactive::read_number_with_hint(
            &prompt,
            |text| text.green().to_string(),
            &labels,
        )?
        else {
            println!("{}", t!("nav.cancelled"));
            return Ok(false);
        };
        let choice = choice.to_lowercase();

        if choice == "cancel" || choice.is_empty() {
            println!("{}", t!("nav.cancelled"));
        } else if let Ok(num) = choice.parse::<usize>() {
            if num > 0 && num <= roots.len() {
                go_to_root(nav, &roots[num - 1])?;
            } else {
                print_error(&if cfg!(windows) {
                    t!("cmd.nav.invalid_drive")
                } else {
                    t!("cmd.nav.invalid_selection")
                });
            }
        } else if let Some(root) = roots
            .iter()
            .find(|r| r.label.eq_ignore_ascii_case(&choice))
        {
            go_to_root(nav, root)?;
        } else {
            print_error(&t!("cmd.nav.invalid_input"));
        }
    } else {
        let wanted = wanted_label(args);
        match roots
            .iter()
            .find(|r| r.label.eq_ignore_ascii_case(&wanted))
        {
            Some(root) => go_to_root(nav, root)?,
            None => print_error(&t!("cmd.godrive.unknown", args)),
        }
    }
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "godrive",
    aliases: &["god"],
    help_topic: Some("godrive"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_godrive),
};

#[cfg(test)]
mod tests {
    use super::wanted_label;

    #[test]
    fn drive_letters_and_mount_paths_normalize_to_a_label() {
        assert_eq!(wanted_label("c"), "c");
        assert_eq!(wanted_label("C"), "c");
        assert_eq!(wanted_label("C:"), "c");
        assert_eq!(wanted_label(" d "), "d");
        assert_eq!(wanted_label("/mnt/c"), "c");
        assert_eq!(wanted_label("/mnt/c/"), "c");
        assert_eq!(wanted_label("wsl/"), "wsl");
    }
}
