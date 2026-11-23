use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;
use rustyline::completion::Completer;
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::{Hint, Hinter};
use rustyline::history::DefaultHistory;
use rustyline::validate::Validator;
use rustyline::{Context, Editor, Helper};
use std::borrow::Cow;
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
        Self {
            label: label.into(),
            detail: String::new(),
        }
    }

    pub fn with_detail(label: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            detail: detail.into(),
        }
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
            println!("{}", t!("interactive.no_items").dimmed());
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
        let labels: Vec<&str> = self.items.iter().map(|item| item.label.as_str()).collect();
        let prompt = format!("  {} ", self.prompt);
        let input = match read_line_with_hint(
            &prompt,
            |text| text.green().to_string(),
            |text| numeric_hint(text, &labels),
        )? {
            Some(input) => input,
            None => {
                println!("{}", t!("interactive.cancelled").dimmed());
                return Ok(None);
            }
        };

        if self.show_cancel && (input == "0" || input.is_empty()) {
            println!("{}", t!("interactive.cancelled").dimmed());
            return Ok(None);
        }

        match input.parse::<usize>() {
            Ok(n) if n >= 1 && n <= self.items.len() => Ok(Some(n - 1)),
            Ok(_) => {
                println!("{}", t!("interactive.invalid_selection").red());
                Ok(None)
            }
            Err(_) => {
                println!("{}", t!("interactive.invalid_input").red());
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

/// Hint shown after a number in a selection prompt.
pub(crate) fn numeric_hint<S: AsRef<str>>(input: &str, labels: &[S]) -> Option<String> {
    let number: usize = input.trim().parse().ok()?;
    if number == 0 || number > labels.len() {
        return None;
    }
    Some(format!("  {}", labels[number - 1].as_ref()))
}

/// A hint that can be displayed but never inserted with Right/Tab.
struct MenuHint(String);

impl Hint for MenuHint {
    fn display(&self) -> &str {
        &self.0
    }

    fn completion(&self) -> Option<&str> {
        None
    }
}

struct MenuHelper<P, F> {
    paint_prompt: P,
    hint_for: F,
}

impl<P, F> Completer for MenuHelper<P, F> {
    type Candidate = String;
}

impl<P, F> Hinter for MenuHelper<P, F>
where
    P: Fn(&str) -> String,
    F: Fn(&str) -> Option<String>,
{
    type Hint = MenuHint;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<Self::Hint> {
        if pos < line.len() {
            return None;
        }
        (self.hint_for)(line).map(MenuHint)
    }
}

impl<P, F> Highlighter for MenuHelper<P, F>
where
    P: Fn(&str) -> String,
{
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        _default: bool,
    ) -> Cow<'b, str> {
        Cow::Owned((self.paint_prompt)(prompt))
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        // Same muted role as the idle shell hint (`shell/idle.rs`): advice,
        // never emphasis. Terminals that cannot render ANSI get plain text.
        if crate::output::raw_ansi_allowed() {
            Cow::Owned(hint.dimmed().to_string())
        } else {
            Cow::Borrowed(hint)
        }
    }
}

impl<P, F> Validator for MenuHelper<P, F> {}

impl<P, F> Helper for MenuHelper<P, F>
where
    P: Fn(&str) -> String,
    F: Fn(&str) -> Option<String>,
{
}

/// Read one line with a live hint derived from the text typed so far.
///
/// The prompt is passed to rustyline as plain text (ANSI would break its
/// cursor math) and painted by [`MenuHelper::highlight_prompt`]. Non-TTY
/// input falls back to a plain line read, so scripts keep working.
pub(crate) fn read_line_with_hint<P, F>(
    prompt: &str,
    paint_prompt: P,
    hint_for: F,
) -> Result<Option<String>>
where
    P: Fn(&str) -> String,
    F: Fn(&str) -> Option<String>,
{
    // Painted up front so the non-TTY fallback keeps the same colour.
    let painted_prompt = paint_prompt(prompt);
    let mut editor = match Editor::<MenuHelper<P, F>, DefaultHistory>::new() {
        Ok(editor) => editor,
        Err(_) => return read_line_plain(&painted_prompt),
    };
    editor.set_helper(Some(MenuHelper {
        paint_prompt,
        hint_for,
    }));

    loop {
        match editor.readline(prompt) {
            Ok(line) => return Ok(Some(line.trim().to_string())),
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => return Ok(None),
            Err(ReadlineError::WindowResized) => continue,
            Err(err) => return Err(anyhow::anyhow!("{}", err)),
        }
    }
}

/// Read a menu number and ghost the matching item's label after the caret.
///
/// Used by every numbered picker (`gos`, `gosc`, `godrive`, `fgo`, `fsc`,
/// `backup restore|diff`, `dl --stop|--resume`, …) so typing `3` shows which
/// entry `3` is before Enter is pressed. The hint is display-only: it uses the
/// idle colour and `MenuHint::completion` returns `None`, so Right and Tab can
/// never accept it.
pub(crate) fn read_number_with_hint<P>(
    prompt: &str,
    paint_prompt: P,
    labels: &[String],
) -> Result<Option<String>>
where
    P: Fn(&str) -> String,
{
    read_line_with_hint(prompt, paint_prompt, |text| numeric_hint(text, labels))
}

fn read_line_plain(prompt: &str) -> Result<Option<String>> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    if io::stdin().read_line(&mut input)? == 0 {
        return Ok(None);
    }
    Ok(Some(input.trim().to_string()))
}

#[cfg(test)]
mod tests {
    use super::{numeric_hint, MenuHint};
    use rustyline::hint::Hint;

    #[test]
    fn menu_hint_can_be_displayed_but_never_inserted() {
        let hint = MenuHint("  index.html".to_string());
        assert_eq!(hint.display(), "  index.html");
        assert!(hint.completion().is_none());
    }

    #[test]
    fn numeric_hint_names_the_item_for_a_valid_number() {
        let labels = vec!["file1.txt", "nice.md", "index.html"];
        assert_eq!(numeric_hint("3", &labels).as_deref(), Some("  index.html"));
    }

    #[test]
    fn numeric_hint_ignores_zero_and_out_of_range_numbers() {
        let labels = vec!["file1.txt", "nice.md", "index.html"];
        assert_eq!(numeric_hint("0", &labels), None);
        assert_eq!(numeric_hint("4", &labels), None);
    }

    #[test]
    fn numeric_hint_ignores_empty_and_non_numeric_input() {
        let labels = vec!["file1.txt", "nice.md", "index.html"];
        assert_eq!(numeric_hint("", &labels), None);
        assert_eq!(numeric_hint("x", &labels), None);
        assert_eq!(numeric_hint("1x", &labels), None);
    }

    #[test]
    fn numeric_hint_accepts_surrounding_whitespace() {
        let labels = vec!["file1.txt", "nice.md", "index.html"];
        assert_eq!(numeric_hint(" 2 ", &labels).as_deref(), Some("  nice.md"));
    }
}
