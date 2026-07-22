use crate::navigator::Navigator;
use crate::output::print_error;
#[cfg(windows)]
use crate::output::print_info;
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
                    .map_err(|e| anyhow::anyhow!("Failed to get executable path: {e}"))?;
                let exe_str = exe.to_string_lossy().replace('\'', "''");
                let status = std::process::Command::new("powershell")
                    .args([
                        "-NoProfile", "-Command",
                        &format!("Start-Process -Verb RunAs -FilePath '{exe_str}' -Wait")
                    ])
                    .spawn();
                match status {
                    Ok(_) => {
                        print_info("Re-launching ntc as Administrator...");
                        Ok(true)
                    }
                    Err(e) => {
                        print_error(&format!("Failed to launch elevated process: {e}"));
                        Ok(false)
                    }
                }
            }
            "normal" => {
                let exe = std::env::current_exe()
                    .map_err(|e| anyhow::anyhow!("Failed to get executable path: {e}"))?;
                let exe_str = exe.to_string_lossy().replace('\'', "''");
                let status = std::process::Command::new("powershell")
                    .args([
                        "-NoProfile", "-Command",
                        &format!("Start-Process -FilePath '{exe_str}'")
                    ])
                    .spawn();
                match status {
                    Ok(_) => {
                        print_info("Re-launching ntc in normal mode...");
                        Ok(true)
                    }
                    Err(e) => {
                        print_error(&format!("Failed to launch process: {e}"));
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
        print_error("mode is a Windows-only feature.");
        Ok(false)
    }
}

#[cfg(windows)]
fn print_usage() {
    println!("{}", "Usage:".cyan().bold());
    println!("  mode admin               Exit and re-launch ntc as Administrator");
    println!("  mode normal              Exit and re-launch ntc normally");
    println!();
    println!("{}", "Examples:".green());
    println!("  mode admin                (UAC prompt will appear)");
    println!("  mode normal");
}
