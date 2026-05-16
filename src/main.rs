use ntc::cli;
use ntc::shell;
use ntc::Config;
use anyhow::Result;
use colored::*;

fn print_logo() {
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
}

fn main() -> Result<()> {
    let _ = Config::global();

    let launch_interactive = cli::run_cli()?;
    if launch_interactive {
        print_logo();
        shell::run_shell()?;
    }

    Config::save_global();
    Ok(())
}