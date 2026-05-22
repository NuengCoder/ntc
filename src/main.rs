use ntc::cli;
use ntc::shell;
use ntc::Config;
use anyhow::Result;
use colored::*;


fn main() -> Result<()> {
    let _ = Config::global();

    let launch_interactive = cli::run_cli()?;
    if launch_interactive {
        // Logo is now printed inside cli.rs for @ shortcut
        // For regular interactive mode, we need to print it here
        println!();
        println!("  {}  {}  {}", 
            "N".blue().bold(), 
            "T".blue().bold(), 
            "C".blue().bold()
        );
        println!("  {} {} {}", 
            "Navigate".red(), 
            "Tree".green(), 
            "Cat".blue()
        );
        println!();
        shell::run_shell()?;
    }

    Config::save_global();
    Ok(())
}