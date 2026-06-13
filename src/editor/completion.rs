use crate::syntax::SyntaxLanguage;
use crate::config::Config;
use super::Editor;

#[derive(Clone, Debug)]
pub(crate) struct CompletionItem {
    pub(crate) label: String,
    pub(crate) detail: String,
    pub(crate) insert_text: String,
}

pub(crate) fn completions_for(lang: Option<SyntaxLanguage>, prefix: &str) -> Vec<CompletionItem> {
    let lower = prefix.to_lowercase();
    let mut items = Vec::new();

    if let Some(SyntaxLanguage::NtcMath) = lang {
        // Built-in functions
        for name in crate::math::builtin_function_names() {
            if name == "print" { continue; } // handled below with parens
            if name.to_lowercase().starts_with(&lower) {
                let detail = match name {
                    "sin" => "sine (radians)",
                    "cos" => "cosine (radians)",
                    "tan" => "tangent (radians)",
                    "cot" => "cotangent (radians)",
                    "sec" => "secant (radians)",
                    "csc" => "cosecant (radians)",
                    "asin" | "arcsin" => "inverse sine",
                    "acos" | "arccos" => "inverse cosine",
                    "atan" | "arctan" => "inverse tangent",
                    "acot" | "arccot" => "inverse cotangent",
                    "asec" | "arcsec" => "inverse secant",
                    "acsc" | "arccsc" => "inverse cosecant",
                    "sqrt" => "square root",
                    "pow" => "power (x, y)",
                    "abs" => "absolute value",
                    "floor" => "round down",
                    "ceil" | "ceiling" => "round up",
                    "round" => "nearest integer",
                    "ln" | "log" => "natural log",
                    "log2" => "base-2 log",
                    "log10" => "base-10 log",
                    "sum" => "sum of values",
                    "min" => "minimum value",
                    "max" => "maximum value",
                    "avg" | "average" | "mean" => "arithmetic mean",
                    "rand" | "random" => "random integer in [min, max]",
                    "tobinary" => "to binary string",
                    "tohex" => "to hex string",
                    "to8" | "tooctal" => "to octal string",
                    "todecimal" => "to decimal number",
                    "tohb" | "tohumanbytes" | "tohumanreadable" => "human-readable bytes",
                    _ => "",
                };
                items.push(CompletionItem {
                    label: name.to_string(),
                    detail: detail.to_string(),
                    insert_text: name.to_string(),
                });
            }
        }

        // Constants
        for name in crate::math::constant_names() {
            if name.to_lowercase().starts_with(&lower) {
                let val = match name {
                    "PI" | "pi" => std::f64::consts::PI,
                    "E" | "e" | "EXP" | "exp" => std::f64::consts::E,
                    "PHI" | "phi" => 1.618_033_988_749_895,
                    "TAU" | "tau" => std::f64::consts::TAU,
                    _ => 0.0,
                };
                items.push(CompletionItem {
                    label: name.to_string(),
                    detail: format!("= {}", val),
                    insert_text: name.to_string(),
                });
            }
        }

        // Keywords
        for kw in &["return"] {
            if kw.starts_with(&lower) {
                items.push(CompletionItem {
                    label: kw.to_string(),
                    detail: "return a value".to_string(),
                    insert_text: kw.to_string(),
                });
            }
        }

        // print() — insert with parentheses
        if "print".starts_with(&lower) {
            items.push(CompletionItem {
                label: "print".to_string(),
                detail: "print value(s)".to_string(),
                insert_text: "print()".to_string(),
            });
        }

        // User-defined functions from config
        let cfg = Config::global();
        if let Ok(cfg_guard) = cfg.read() {
            for (name, def) in &cfg_guard.math_functions {
                if name.starts_with(&lower) && !items.iter().any(|i| i.label == *name) {
                    let arrow = def.find("=>").unwrap_or(0);
                    let params = def[..arrow].trim();
                    let detail = format!("fn({})", params);
                    items.push(CompletionItem {
                        label: name.clone(),
                        detail,
                        insert_text: name.clone(),
                    });
                }
            }
        }
    }

    // Sort: exact match first, then alphabetical
    items.sort_by(|a, b| {
        let a_exact = a.label.to_lowercase() == lower;
        let b_exact = b.label.to_lowercase() == lower;
        if a_exact != b_exact {
            return b_exact.cmp(&a_exact);
        }
        a.label.cmp(&b.label)
    });

    items
}

impl Editor {
    pub(crate) fn trigger_completion(&mut self) {
        let line = self.current();
        if let Some((word, _start_byte)) = word_before_cursor(line, self.cursor_byte) {
            let lang = self.syntax.language;
            let items = completions_for(lang, &word);
            if !items.is_empty() {
                self.completion_items = items;
                self.completion_idx = 0;
                self.completion_visible = true;
                self.completion_prefix = word.clone();
                self.mark_all_dirty();
            }
        }
    }

    pub(crate) fn dismiss_completion(&mut self) {
        if self.completion_visible {
            self.completion_visible = false;
            self.completion_items.clear();
            self.completion_idx = 0;
            self.completion_prefix.clear();
            self.mark_all_dirty();
        }
    }

    pub(crate) fn select_next_completion(&mut self) {
        if self.completion_visible && !self.completion_items.is_empty() {
            self.completion_idx = (self.completion_idx + 1) % self.completion_items.len();
            self.mark_all_dirty();
        }
    }

    pub(crate) fn select_prev_completion(&mut self) {
        if self.completion_visible && !self.completion_items.is_empty() {
            if self.completion_idx == 0 {
                self.completion_idx = self.completion_items.len() - 1;
            } else {
                self.completion_idx -= 1;
            }
            self.mark_all_dirty();
        }
    }

    pub(crate) fn apply_completion(&mut self) {
        if !self.completion_visible || self.completion_items.is_empty() {
            return;
        }
        let item = &self.completion_items[self.completion_idx];
        let line = self.current();
        if let Some((_, start_byte)) = word_before_cursor(line, self.cursor_byte) {
            let end_byte = self.cursor_byte;
            // Replace the word with the completion
            let new_line = format!(
                "{}{}{}",
                &line[..start_byte],
                item.insert_text,
                &line[end_byte..]
            );
            self.lines[self.cursor_y] = new_line;
            self.cursor_byte = start_byte + item.insert_text.len();
            self.modified = true;
            self.mark_dirty(self.cursor_y);
            self.syntax.invalidate_line(self.cursor_y);
        }
        self.dismiss_completion();
    }
}

/// Get the word (identifier) under or before the cursor. Returns the word and its start byte.
pub(crate) fn word_before_cursor(line: &str, cursor_byte: usize) -> Option<(String, usize)> {
    let byte = cursor_byte.min(line.len());
    let byte = byte.min(line.floor_char_boundary(byte));
    let prefix = &line[..byte];
    let start = prefix
        .chars()
        .rev()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .map(|c| c.len_utf8())
        .sum::<usize>();

    if start == 0 {
        return None;
    }

    let word = line[byte - start..byte].to_string();
    Some((word, byte - start))
}
