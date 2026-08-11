use crate::navigator::clear_screen;
use crate::output::print_info;
use crate::t;
use colored::*;
use std::io::{self, Write};

use super::steps::TutorialStep;

const BOX_WIDTH: usize = 68;

fn header_style() -> colored::Color {
    #[cfg(not(target_os = "android"))]
    {
        crate::utils::theme::ThemeManager::current()
            .shell
            .help_header
            .to_colored()
    }
    #[cfg(target_os = "android")]
    {
        colored::Color::White
    }
}

fn success_style() -> colored::Color {
    #[cfg(not(target_os = "android"))]
    {
        crate::utils::theme::ThemeManager::current()
            .shell
            .success
            .to_colored()
    }
    #[cfg(target_os = "android")]
    {
        colored::Color::White
    }
}

fn info_style() -> colored::Color {
    #[cfg(not(target_os = "android"))]
    {
        crate::utils::theme::ThemeManager::current()
            .shell
            .info
            .to_colored()
    }
    #[cfg(target_os = "android")]
    {
        colored::Color::White
    }
}

pub fn clear() {
    clear_screen();
}

pub fn header(current: usize, total: usize) {
    let header_c = header_style();
    let success_c = success_style();
    let progress = progress_bar(current, total);

    println!(
        "{}",
        "╔══════════════════════════════════════════════════════════════════╗".color(header_c)
    );

    let title = t!("tutorial.step_title", current, total);
    let title_pad = BOX_WIDTH.saturating_sub(title.len());
    let left = title_pad / 2;
    let right = title_pad - left;
    println!(
        "║{}{}{}║",
        " ".repeat(left),
        title.color(success_c).bold(),
        " ".repeat(right)
    );

    let prog_pad = BOX_WIDTH.saturating_sub(progress.len());
    let pleft = prog_pad / 2;
    let pright = prog_pad - pleft;
    println!("║{}{}{}║", " ".repeat(pleft), progress, " ".repeat(pright));

    println!(
        "{}",
        "╚══════════════════════════════════════════════════════════════════╝".color(header_c)
    );
    println!();
}

fn progress_bar(current: usize, total: usize) -> String {
    let bar_width = 16;
    let filled = (current * bar_width).checked_div(total).unwrap_or(0);
    let empty = bar_width.saturating_sub(filled);
    let mut bar = String::with_capacity(bar_width + 2);
    bar.push('[');
    for _ in 0..filled {
        bar.push('█');
    }
    for _ in 0..empty {
        bar.push('░');
    }
    bar.push(']');
    bar.white().to_string()
}

pub fn step(step: &TutorialStep) {
    let info_c = info_style();

    let sep = format!("── {} ", step.title);
    let sep_fill = BOX_WIDTH.saturating_sub(sep.len() + 2);
    println!(
        "{}{}{}",
        sep.color(info_c).bold(),
        "─".repeat(sep_fill).color(info_c).dimmed(),
        "─".color(info_c).dimmed()
    );
    println!();

    for line in step.lines {
        if line.is_empty() {
            println!();
        } else if line.starts_with("  ") {
            println!("{}", line.dimmed());
        } else {
            println!("{}", line);
        }
    }

    if let Some(try_it) = step.try_it {
        println!();
        println!(
            "{} {}",
            t!("tutorial.tip_label").yellow().bold(),
            try_it.dimmed()
        );
    }
}

pub fn demo_header() {
    println!();
    print_info(&t!("tutorial.demonstrating"));
}

pub fn prompt() -> PromptAction {
    println!();
    print!("{}", t!("tutorial.prompt_continue").dimmed());
    print!("{}", " | ".dimmed());
    print!("{}", t!("tutorial.prompt_skip_label").yellow());
    print!("{}", " | ".dimmed());
    print!("{}", t!("tutorial.prompt_exit_label").red());
    print!("{}", ": ".dimmed());
    io::stdout().flush().ok();

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    let input = input.trim().to_lowercase();

    match input.as_str() {
        "exit" | "quit" | "q" => PromptAction::Exit,
        "skip" | "s" => PromptAction::Skip,
        _ => PromptAction::Next,
    }
}

pub enum PromptAction {
    Next,
    Skip,
    Exit,
}

pub fn completion() {
    let header_c = header_style();
    let success_c = success_style();
    let info_c = info_style();
    clear();

    println!(
        "{}",
        "╔══════════════════════════════════════════════════════════════════╗".color(header_c)
    );
    let msg = t!("tutorial.complete_title");
    let msg_pad = BOX_WIDTH.saturating_sub(msg.len());
    let mleft = msg_pad / 2;
    let mright = msg_pad - mleft;
    println!(
        "║{}{}{}║",
        " ".repeat(mleft),
        msg.color(success_c).bold(),
        " ".repeat(mright)
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════════════════════════════╝".color(header_c)
    );
    println!();

    println!("{}", t!("tutorial.complete_msg"));
    println!();
    let highlights = [
        ("Navigation", "cd, back, gosc, where"),
        ("Teleport", "tp add/jump, @name, tpb"),
        ("Aliases", "ral add/list/run, ran"),
        ("View/Search", "view, fs, ds, locate, gs"),
        ("Reports", "report txt/html/json/md/..."),
        ("Config", "config depth/color/watch, showcg"),
        ("Backups", "backup create/restore/undo/diff"),
        ("File Ops", "mkf, mkd, rmf, rmd, cp, txtc/txtf"),
        ("Themes", "theme list/switch/add/rm"),
        ("Hotkey", "config qsc, qsc listen"),
        ("Network", "sys/top, serve"),
        ("Editor", "ne — Normal/Insert/Command/Search modes"),
    ];
    for (feature, cmds) in &highlights {
        println!(
            "  {}  {}",
            format!("{:<12}", feature).color(info_c).bold(),
            cmds.dimmed()
        );
    }
    println!();
    println!("{}", t!("tutorial.help_hint").dimmed());
    println!("{}", t!("tutorial.exit_hint").dimmed());
    println!();
    pause(&t!("tutorial.return_hint"));
}

fn pause(msg: &str) {
    print!("{}", msg.dimmed());
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
}
