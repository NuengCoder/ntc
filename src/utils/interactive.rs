use anyhow::Result;
use colored::*;
use std::io::{self, Write};

pub struct InteractiveMenu<'a> {
    title: &'a str,
    items: Vec<MenuItem>,
    prompt: String,
    show_cancel: bool,
    cancel_label: &'a str,
}

pub struct MenuItem {
    pub label: String,
    pub detail: String,
}

impl MenuItem {
    pub fn simple(label: impl Into<String>) -> Self {
        Self { label: label.into(), detail: String::new() }
    }

    pub fn with_detail(label: impl Into<String>, detail: impl Into<String>) -> Self {
        Self { label: label.into(), detail: detail.into() }
    }
}

impl<'a> InteractiveMenu<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            items: Vec::new(),
            prompt: "Select: ".to_string(),
            show_cancel: true,
            cancel_label: "Cancel",
        }
    }

    pub fn item(mut self, item: MenuItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(mut self, items: Vec<MenuItem>) -> Self {
        self.items.extend(items);
        self
    }

    pub fn prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = prompt.into();
        self
    }

    pub fn no_cancel(mut self) -> Self {
        self.show_cancel = false;
        self
    }

    pub fn cancel_label(mut self, label: &'a str) -> Self {
        self.cancel_label = label;
        self
    }

    pub fn run(&self) -> Result<Option<usize>> {
        println!();
        println!("{}", format!("  {}", self.title).cyan().bold());
        println!("{}", "  ═══════════════════════════════════════".cyan());

        if self.items.is_empty() {
            println!("  {}", "(no items)".dimmed());
        } else {
            for (i, item) in self.items.iter().enumerate() {
                if item.detail.is_empty() {
                    println!("    {} {}", (i + 1).to_string().yellow(), item.label.blue());
                } else {
                    println!(
                        "    {} {} {}",
                        (i + 1).to_string().yellow(),
                        item.label.blue(),
                        item.detail.dimmed()
                    );
                }
            }
        }

        if self.show_cancel {
            println!("    {} {}", "0".yellow(), self.cancel_label.red());
        }
        println!();
        print!("  {} ", self.prompt.green());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if self.show_cancel && (input == "0" || input.is_empty()) {
            println!("  {}", "Cancelled.".dimmed());
            return Ok(None);
        }

        match input.parse::<usize>() {
            Ok(n) if n >= 1 && n <= self.items.len() => Ok(Some(n - 1)),
            Ok(_) => {
                println!("  {}", "Invalid selection.".red());
                Ok(None)
            }
            Err(_) => {
                println!("  {}", "Invalid input.".red());
                Ok(None)
            }
        }
    }
}

pub fn prompt_yes_no(message: &str, default: bool) -> Result<bool> {
    let hint = if default { "[Y/n]" } else { "[y/N]" };
    print!("  {} {} ", message.green(), hint.dimmed());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    match input.as_str() {
        "y" | "yes" => Ok(true),
        "n" | "no" => Ok(false),
        "" => Ok(default),
        _ => Ok(default),
    }
}

pub fn prompt_text(message: &str) -> Result<String> {
    print!("  {} ", message.green());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
