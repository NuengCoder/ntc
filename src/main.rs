use ntc::cli;
use ntc::shell;
use ntc::Config;
use ntc::utils;
use anyhow::Result;
use colored::*;

fn run() -> Result<bool> {
    crate::utils::theme::ThemeManager::ensure_default_themes();
    let _ = Config::global();
    cli::run_cli()
}

fn main() -> Result<()> {
    let launch_interactive = run()?;
    if launch_interactive {
        println!();
        println!("  {}  {}  {}",
            "N".blue().bold(),
            "T".blue().bold(),
            "C".blue().bold()
        );
        println!("  {} {} {}",
            "Navigate".red(),
            "Toolkit".green(),
            "Center".blue()
        );
        println!();
        shell::run_shell()?;
    }
    Config::save_global();
    Ok(())
}