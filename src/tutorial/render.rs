//! Screen output for the tutorial shell. Each step shows its goal and the
//! command to type; the command is only printed, never executed for the
//! learner. Extra nudges stay behind `tutorial hint`.

use crate::output::print_success;
use crate::t;
use colored::*;
use std::path::Path;

use super::lessons::LESSONS;

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

fn rule(title: &str) {
    let header_c = header_style();
    let line = format!("── {} ", title);
    let fill = 66usize.saturating_sub(title.chars().count() + 3);
    println!(
        "{}{}",
        line.color(header_c).bold(),
        "─".repeat(fill).color(header_c).dimmed()
    );
}

pub fn mode_on(root: &Path) {
    println!();
    rule(&t!("tutorial.menu_title"));
    println!("{}", t!("tutorial.ws_ready", root.display()).dimmed());
    println!("{}", t!("tutorial.mode_on").color(info_style()).bold());
    println!();
    menu();
}

pub fn menu() {
    for (i, lesson) in LESSONS.iter().enumerate() {
        println!(
            "  {}. {}  {}  {}",
            (i + 1).to_string().color(info_style()).bold(),
            format!("{:<6}", lesson.key).bold(),
            lesson.title,
            lesson.summary.dimmed()
        );
    }
    println!();
    println!("{}", t!("tutorial.menu_goal"));
    println!("{}", t!("tutorial.menu_hint").dimmed());
    println!();
}

pub fn lesson(index: usize) {
    let Some(lesson) = LESSONS.get(index) else {
        return;
    };
    println!();
    rule(&format!("{} · {}", lesson.key, lesson.title));
    println!("{}", lesson.summary.dimmed());
    println!();
}

pub fn show_step(index: usize, step: usize) {
    let Some(lesson) = LESSONS.get(index) else {
        return;
    };
    let Some(step_ref) = lesson.steps.get(step) else {
        return;
    };
    println!(
        "{}",
        t!("tutorial.step_label", step + 1, lesson.steps.len())
            .color(info_style())
            .bold()
    );
    println!("{} {}", t!("tutorial.goal_label").bold(), step_ref.goal);
    println!(
        "{} {}",
        t!("tutorial.command_label").bold(),
        step_ref.command.color(info_style()).bold()
    );
    println!("{}", t!("tutorial.command_usage").dimmed());
    println!();
}

pub fn lesson_done(key: &str) {
    println!();
    print_success(&t!("tutorial.lesson_done", key));
    println!("{}", t!("tutorial.menu_hint").dimmed());
    println!();
}

/// One more command to play with, printed right after a step lands.
pub fn also_try(cmd: &str) {
    if cmd.trim().is_empty() {
        return;
    }
    println!("{}", t!("tutorial.also_try_fmt", cmd).dimmed());
}

pub fn usage() {
    println!();
    println!("{}", t!("tutorial.usage"));
    println!();
}
