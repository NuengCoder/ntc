use anyhow::Result;
use colored::Colorize;
use ntc::cli;
use ntc::shell;
use ntc::utils;
use ntc::Config;

fn run() -> Result<(bool, i32)> {
    utils::theme::ThemeManager::ensure_default_themes();
    let _ = Config::global();
    // rayon's default pool is one worker per CPU. On a small VPS that means a
    // dozen threads (each with its own stack and malloc arena) for scans,
    // sizes and views. Honor `config threads`, which was previously unused.
    // Setting the env var (rather than building the pool here) keeps the pool
    // lazy: commands that never use rayon still pay for no idle workers.
    if std::env::var_os("RAYON_NUM_THREADS").is_none() {
        std::env::set_var(
            "RAYON_NUM_THREADS",
            Config::global_get_num_threads().max(1).to_string(),
        );
    }
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
    // Legacy conhost (Win7/8, Admin legacy mode) can't render even basic
    // ANSI — emit plain text so users never see `[91m` garbage.
    // Plain white logo fallback per user choice.
    if !colored::control::SHOULD_COLORIZE.should_colorize() {
        return s.to_string();
    }
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            // Spaces carry no color codes: avoids width miscounts and bloat.
            if c == ' ' {
                return " ".to_string();
            }
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
    // Enable VT processing BEFORE any colored output. On Win11 Terminal this
    // is a no-op; on Win10/Admin conhost it turns `[91m` garbage into real
    // colors; on Win7/8 legacy it fails and Config::load() strips colors.
    #[cfg(windows)]
    {
        ntc::utils::terminal::init();
    }
    let (launch_interactive, exit_code) = run()?;
    if launch_interactive {
        // `config intro_view off` suppresses the whole startup intro
        // (logo here; ntpkg/watcher notices in the shell entry).
        if Config::global_get_intro_view_enabled() {
            println!();
            print_logo();
            println!();
        }
        shell::run_shell()?;
    }
    Config::save_global();
    // One-shot commands: hand the shell's success/failure back to the OS so
    // scripts and CI can gate on `ntc <command>`.
    if exit_code != 0 {
        std::process::exit(exit_code);
    }
    Ok(())
}
