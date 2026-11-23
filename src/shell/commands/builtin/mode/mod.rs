pub(crate) mod completion;
pub(crate) mod help;
use crate::navigator::Navigator;
use crate::output::print_error;
#[cfg(windows)]
use crate::output::print_info;
use crate::t;
#[cfg(windows)]
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

pub fn cmd_mode(args: &str, nav: &mut Navigator) -> Result<bool> {
    #[cfg(windows)]
    {
        let args = args.trim().to_lowercase();
        // `Start-Process` otherwise lets the new shell restore its last
        // session directory, which can leave a project's ntconfig.toml (and
        // its ral/igcare hints) behind. Pin the relaunch to this directory.
        let cwd = nav.current_path().to_string_lossy().replace('\'', "''");
        match args.as_str() {
            "admin" => {
                let exe = std::env::current_exe()
                    .map_err(|e| anyhow::anyhow!("{}", t!("cmd.mode.exe_fail", e)))?;
                let exe_str = exe.to_string_lossy().replace('\'', "''");
                let status = std::process::Command::new("powershell")
                    .args([
                        "-NoProfile",
                        "-Command",
                        &format!(
                            "Start-Process -Verb RunAs -FilePath '{exe_str}' -ArgumentList '--here' -WorkingDirectory '{cwd}' -Wait"
                        ),
                    ])
                    .spawn();
                match status {
                    Ok(_) => {
                        print_info(&t!("cmd.mode.admin_relaunch"));
                        Ok(true)
                    }
                    Err(e) => {
                        print_error(&t!("cmd.mode.elevate_fail", e));
                        Ok(false)
                    }
                }
            }
            "normal" => {
                let exe = std::env::current_exe()
                    .map_err(|e| anyhow::anyhow!("{}", t!("cmd.mode.exe_fail", e)))?;
                let exe_str = exe.to_string_lossy().replace('\'', "''");
                let status = std::process::Command::new("powershell")
                    .args([
                        "-NoProfile",
                        "-Command",
                        &format!(
                            "Start-Process -FilePath '{exe_str}' -ArgumentList '--here' -WorkingDirectory '{cwd}'"
                        ),
                    ])
                    .spawn();
                match status {
                    Ok(_) => {
                        print_info(&t!("cmd.mode.normal_relaunch"));
                        Ok(true)
                    }
                    Err(e) => {
                        print_error(&t!("cmd.mode.launch_fail", e));
                        Ok(false)
                    }
                }
            }
            _ => {
                print_usage();
                Ok(false)
            }
        }
    }
    #[cfg(not(windows))]
    {
        let _ = (args, nav);
        print_error(&t!("cmd.mode.windows_only"));
        Ok(false)
    }
}

#[cfg(windows)]
fn print_usage() {
    println!("{}", t!("mode.help_usage").cyan().bold());
    println!("  {}", t!("mode.help_admin"));
    println!("  {}", t!("mode.help_normal"));
    println!();
    println!("{}", t!("mode.help_examples").green());
    println!("  {}", t!("mode.help_example_admin"));
    println!("  {}", t!("mode.help_example_normal"));
}

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "mode",
    aliases: &[],
    help_topic: Some("mode"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_mode),
};
