use ntc::cli;
use ntc::shell;
use ntc::Config;
use anyhow::Result;

fn main() -> Result<()> {
    // Load persisted config (already called when accessing global, but we ensure it's initialized)
    let _ = Config::global(); // trigger static init, which calls Config::load()

    let launch_interactive = cli::run_cli()?;
    if launch_interactive {
        shell::run_shell()?;
    }

    // Final save
    Config::save_global();
    Ok(())
}