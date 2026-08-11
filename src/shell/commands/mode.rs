use crate::navigator::Navigator;
use crate::output::print_error;
#[cfg(windows)]
use crate::output::print_info;
use crate::t;
use anyhow::Result;
#[cfg(windows)]
use colored::*;

pub fn cmd_mode(args: &str, _nav: &mut Navigator) -> Result<bool> {
    #[cfg(windows)]
    {
        let args = args.trim().to_lowercase();
        match args.as_str() {
            "admin" => {
                let exe = std::env::current_exe()
                    .map_err(|e| anyhow::anyhow!("{}", t!("cmd.mode.exe_fail", e)))?;
                let exe_str = exe.to_string_lossy().replace('\'', "''");
                let status = std::process::Command::new("powershell")
                    .args([
                        "-NoProfile",
                        "-Command",
                        &format!("Start-Process -Verb RunAs -FilePath '{exe_str}' -Wait"),
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
                        &format!("Start-Process -FilePath '{exe_str}'"),
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
        let _ = (args, _nav);
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
