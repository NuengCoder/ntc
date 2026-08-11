use anyhow::Result;
use colored::*;
use ntc::cli;
use ntc::shell;
use ntc::utils;
use ntc::Config;

fn run() -> Result<bool> {
    utils::theme::ThemeManager::ensure_default_themes();
    let _ = Config::global();
    cli::run_cli()
}

const RAINBOW: [(u8, u8, u8); 7] = [
    (255, 77, 77),
    (255, 165, 0),
    (255, 230, 50),
    (77, 255, 77),
    (50, 200, 255),
    (77, 77, 255),
    (200, 77, 255),
];

fn rainbow(s: &str, offset: usize) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            format!(
                "{}",
                c.to_string().truecolor(
                    RAINBOW[(offset + i) % 7].0,
                    RAINBOW[(offset + i) % 7].1,
                    RAINBOW[(offset + i) % 7].2,
                )
            )
        })
        .collect()
}

fn print_logo() {
    let logo = [
        "███╗   ██╗████████╗ ██████╗",
        "████╗  ██║╚══██╔══╝██╔════╝",
        "██╔██╗ ██║   ██║   ██║     ",
        "██║╚██╗██║   ██║   ██║     ",
        "██║ ╚████║   ██║   ╚██████╗",
        "╚═╝  ╚═══╝   ╚═╝    ╚═════╝",
    ];
    for line in &logo {
        println!(" {}", rainbow(line, 0));
    }
    println!(
        " {}  {}  {}",
        rainbow("Navigate", 0),
        rainbow("Toolkit", 0),
        rainbow("Center", 0),
    );
}

fn main() -> Result<()> {
    let launch_interactive = run()?;
    if launch_interactive {
        println!();
        print_logo();
        println!();
        shell::run_shell()?;
    }
    Config::save_global();
    Ok(())
}
