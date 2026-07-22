use crate::navigator::Navigator;
use crate::output::print_info;
use crate::shell::commands::execute_command;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use super::render::{self, PromptAction};
use super::steps::STEPS;

fn tutorial_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ntc")
        .join("tutorial")
}

fn prepare_tutorial_dir() -> Result<PathBuf> {
    let dir = tutorial_dir();

    fs::create_dir_all(dir.join("projects").join("web"))?;
    fs::create_dir_all(dir.join("projects").join("cli"))?;
    fs::create_dir_all(dir.join("docs"))?;
    fs::create_dir_all(dir.join("images"))?;

    fs::write(
        dir.join("hello.txt"),
        "Hello from NTC tutorial!\n\nThis is a sample file you can view, edit, or search.",
    )?;
    fs::write(
        dir.join("notes.txt"),
        "Meeting notes:\n- Discuss Q4 roadmap\n- Review project timeline\n- Assign team leads\n- Schedule follow-up\n",
    )?;
    fs::write(
        dir.join("data.csv"),
        "name,age,role\nAlice,30,Engineer\nBob,25,Designer\nCarol,35,Manager\n",
    )?;
    fs::write(
        dir.join("projects").join("web").join("index.html"),
        "<!DOCTYPE html>\n<html>\n<head><title>My Project</title></head>\n<body>\n  <h1>Welcome</h1>\n</body>\n</html>\n",
    )?;
    fs::write(
        dir.join("projects").join("cli").join("main.rs"),
        "fn main() {\n    println!(\"Hello from NTC!\");\n}\n",
    )?;
    fs::write(
        dir.join("docs").join("readme.md"),
        "# Tutorial Project\n\nThis is a sample project for the NTC tutorial.\n",
    )?;

    Ok(dir)
}

pub fn run_tutorial(nav: &mut Navigator) -> Result<()> {
    let dir = prepare_tutorial_dir()?;
    nav.go_to(&dir)?;

    print_info(&format!("Tutorial workspace: {}", dir.display()));
    println!();

    let total = STEPS.len();

    for (i, step) in STEPS.iter().enumerate() {
        render::clear();
        render::header(i + 1, total);
        render::step(step);

        if let Some(cmd) = step.demo {
            render::demo_header();
            println!("  ${}", cmd);
            println!();
            if let Err(e) = execute_command(cmd, nav, &None::<Arc<crate::watcher::WatcherHandle>>) {
                use crate::output::print_warning;
                print_warning(&format!("Demo command failed: {}", e));
            }
        }

        match render::prompt() {
            PromptAction::Exit => break,
            PromptAction::Skip => continue,
            PromptAction::Next => continue,
        }
    }

    render::completion();
    Ok(())
}
