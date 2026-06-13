use crate::navigator::Navigator;
use crate::shell::commands::execute_command;
use anyhow::Result;
use std::sync::Arc;

use super::render::{self, PromptAction};
use super::steps::STEPS;

pub fn run_tutorial(nav: &mut Navigator) -> Result<()> {
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
