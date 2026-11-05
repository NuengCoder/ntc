//! The explorer's command prompt (`g` or `:`).
//!
//! Only three verbs exist, all relative to the directory being browsed:
//!
//! ```text
//! file <file_name>     select a file (its directory is opened first)
//! dir  <dir_name>      enter a directory
//! line <+N|-N|N>       move the selection by N rows / to row N
//! ```
//!
//! The first word is completed from that verb list, and the argument of
//! `file` / `dir` is completed against the current directory (a trailing
//! `/` lets the next component be completed). `line` takes a number, so it
//! gets no hint. `Tab` cycles the hint, `Right` / `Ctrl+E` / `End` accept
//! it, `Enter` runs the line.

use crate::editor::hinter::{path_completions, Want};
use crate::nfe::state::{NfeMode, NfeState};

/// Prompt verbs in hint order. The trailing space lets `file` accept
/// straight into `file ` with the argument slot ready.
pub(crate) const PROMPT_VERBS: &[&str] = &["dir ", "file ", "line "];

/// Usage line shown when the prompt needs an argument (or the verb is
/// unknown).
pub(crate) const PROMPT_USAGE: &str =
    "Commands: file <file_name> | dir <dir_name> | line <+N|-N|N>";

/// Split the last whitespace-separated token off `input`: (head, token).
fn split_last_token(input: &str) -> (&str, &str) {
    match input.rfind(char::is_whitespace) {
        Some(i) => {
            let split = i + input[i..].chars().next().map_or(1, char::len_utf8);
            (&input[..split], &input[split..])
        }
        None => ("", input),
    }
}

impl NfeState {
    /// Open the prompt, optionally prefilled (`l` starts it with `line `).
    pub fn start_prompt(&mut self, prefill: &str) {
        self.mode = NfeMode::Prompt;
        self.prompt_buf = prefill.to_string();
        self.prompt_cursor = self.prompt_buf.len();
        self.prompt_cycle = usize::MAX;
        self.prompt_cycle_tokens.clear();
        self.prompt_cycle_start = 0;
    }

    /// Close the prompt without running anything.
    pub fn cancel_prompt(&mut self) {
        self.mode = self.listing_mode();
        self.prompt_buf.clear();
        self.prompt_cursor = 0;
        self.prompt_cycle = usize::MAX;
        self.prompt_cycle_tokens.clear();
    }

    /// Caret byte offset, clamped to the buffer and a char boundary.
    pub fn prompt_pos(&self) -> usize {
        let mut p = self.prompt_cursor.min(self.prompt_buf.len());
        while p > 0 && !self.prompt_buf.is_char_boundary(p) {
            p -= 1;
        }
        p
    }

    pub fn prompt_insert(&mut self, text: &str) {
        let pos = self.prompt_pos();
        self.prompt_buf.insert_str(pos, text);
        self.prompt_cursor = pos + text.len();
        self.prompt_cycle = usize::MAX;
    }

    pub fn prompt_push(&mut self, c: char) {
        let mut small = [0u8; 4];
        self.prompt_insert(c.encode_utf8(&mut small));
    }

    pub fn prompt_backspace(&mut self) {
        let pos = self.prompt_pos();
        if pos == 0 {
            return;
        }
        let mut prev = pos - 1;
        while prev > 0 && !self.prompt_buf.is_char_boundary(prev) {
            prev -= 1;
        }
        self.prompt_buf.replace_range(prev..pos, "");
        self.prompt_cursor = prev;
        self.prompt_cycle = usize::MAX;
    }

    pub fn prompt_delete(&mut self) {
        let pos = self.prompt_pos();
        if pos >= self.prompt_buf.len() {
            return;
        }
        let mut next = pos + 1;
        while next < self.prompt_buf.len() && !self.prompt_buf.is_char_boundary(next) {
            next += 1;
        }
        self.prompt_buf.replace_range(pos..next, "");
        self.prompt_cycle = usize::MAX;
    }

    pub fn prompt_move_left(&mut self) {
        let pos = self.prompt_pos();
        if pos == 0 {
            return;
        }
        let mut prev = pos - 1;
        while prev > 0 && !self.prompt_buf.is_char_boundary(prev) {
            prev -= 1;
        }
        self.prompt_cursor = prev;
        self.prompt_cycle = usize::MAX;
    }

    pub fn prompt_move_right(&mut self) {
        let pos = self.prompt_pos();
        if pos >= self.prompt_buf.len() {
            return;
        }
        let mut next = pos + 1;
        while next < self.prompt_buf.len() && !self.prompt_buf.is_char_boundary(next) {
            next += 1;
        }
        self.prompt_cursor = next;
        self.prompt_cycle = usize::MAX;
    }

    pub fn prompt_home(&mut self) {
        self.prompt_cursor = 0;
        self.prompt_cycle = usize::MAX;
    }

    pub fn prompt_end(&mut self) {
        self.prompt_cursor = self.prompt_buf.len();
        self.prompt_cycle = usize::MAX;
    }

    /// Candidates that extend the token at the caret:
    /// `(token start, typed text, candidates, is_already_complete)`.
    /// `None` when the caret sits inside a word.
    pub fn prompt_candidates(&self) -> Option<(usize, String, Vec<String>, bool)> {
        let pos = self.prompt_pos();
        if !self.prompt_buf[pos..]
            .chars()
            .next()
            .map_or(true, char::is_whitespace)
        {
            return None;
        }
        let (head, typed) = split_last_token(&self.prompt_buf[..pos]);
        let candidates: Vec<String> = if head.trim().is_empty() {
            PROMPT_VERBS
                .iter()
                .filter(|v| v.starts_with(&typed))
                .map(|v| v.to_string())
                .collect()
        } else {
            let verb = head.split_whitespace().next().unwrap_or("");
            match verb {
                "file" => path_completions(&self.current_dir, typed, Want::Any),
                "dir" => path_completions(&self.current_dir, typed, Want::Dir),
                // `line` takes a number: nothing to complete.
                _ => Vec::new(),
            }
        };
        // A token that already is a candidate has nothing left to type.
        let complete = candidates.iter().any(|t| *t == typed);
        let tokens = candidates
            .into_iter()
            .filter(|t| t.len() > typed.len() && t.starts_with(&typed))
            .collect();
        Some((head.len(), typed.to_string(), tokens, complete))
    }

    /// Remainder of the hinted candidate, painted dimmed at the caret.
    pub fn prompt_ghost(&self) -> Option<String> {
        let (start, typed, tokens, complete) = self.prompt_candidates()?;
        let cycling = self.prompt_cycle_start == start
            && self.prompt_cycle_tokens == tokens
            && self.prompt_cycle < tokens.len();
        // Tab can still extend a complete verb (`file` -> `file `), but the
        // resting hint never suggests it.
        if complete && !cycling {
            return None;
        }
        let idx = if cycling && self.prompt_cycle_tokens[self.prompt_cycle].starts_with(&typed) {
            self.prompt_cycle
        } else {
            0
        };
        let token = tokens.get(idx)?;
        Some(token[typed.len()..].to_string())
    }

    /// Accept the hinted text into the buffer.
    pub fn prompt_accept(&mut self) {
        if let Some(ghost) = self.prompt_ghost() {
            self.prompt_insert(&ghost);
        }
    }

    /// Right arrow: accept the hint at the end of the line, else move right.
    pub fn prompt_accept_or_right(&mut self) {
        if self.prompt_pos() == self.prompt_buf.len() && self.prompt_ghost().is_some() {
            self.prompt_accept();
            return;
        }
        self.prompt_move_right();
    }

    /// End / Ctrl+E: accept the hint at the end of the line, else jump to
    /// the end like End always did.
    pub fn prompt_accept_or_end(&mut self) {
        if self.prompt_pos() == self.prompt_buf.len() && self.prompt_ghost().is_some() {
            self.prompt_accept();
            return;
        }
        self.prompt_end();
    }

    /// Shell-style Tab: step the hint to the next candidate, wrapping.
    /// The buffer is untouched — Right accepts whatever is hinted.
    pub fn prompt_tab(&mut self) {
        let Some((start, typed, tokens, _)) = self.prompt_candidates() else {
            return;
        };
        if tokens.is_empty() {
            return;
        }
        let current = if self.prompt_cycle_start == start
            && self.prompt_cycle_tokens == tokens
            && self.prompt_cycle < tokens.len()
            && tokens[self.prompt_cycle].starts_with(&typed)
        {
            self.prompt_cycle
        } else {
            usize::MAX
        };
        let next = if current == usize::MAX {
            0
        } else {
            (current + 1) % tokens.len()
        };
        self.prompt_cycle = next;
        self.prompt_cycle_start = start;
        self.prompt_cycle_tokens = tokens;
    }

    /// Run the prompt line. A good line closes the prompt; a bad one keeps
    /// it open (with the text intact) so the command can be fixed.
    pub fn run_prompt(&mut self) {
        let line = self.prompt_buf.trim().to_string();
        if line.is_empty() {
            self.cancel_prompt();
            return;
        }
        let (verb, args) = match line.split_once(char::is_whitespace) {
            Some((v, a)) => (v, a.trim()),
            None => (line.as_str(), ""),
        };
        let done = match verb {
            "file" => self.prompt_file(args),
            "dir" => self.prompt_dir(args),
            "line" => self.prompt_line(args),
            other => {
                self.set_status(format!("Unknown command '{}' — {}", other, PROMPT_USAGE));
                false
            }
        };
        if done {
            self.mode = self.listing_mode();
            self.prompt_buf.clear();
            self.prompt_cursor = 0;
            self.prompt_cycle = usize::MAX;
            self.prompt_cycle_tokens.clear();
        }
    }

    fn prompt_file(&mut self, args: &str) -> bool {
        if args.is_empty() {
            self.set_status(format!("Usage: file <file_name> — {}", PROMPT_USAGE));
            return false;
        }
        let target = self.resolve_goto(args);
        if target.is_dir() {
            self.set_status(format!("'{}' is a directory — use: dir {}", args, args));
            return false;
        }
        let found = if target.is_file() {
            Some(target)
        } else {
            self.find_match(args, Some(false))
        };
        match found {
            Some(path) => {
                self.reveal_file(&path);
                true
            }
            None => {
                self.set_status(format!("No such file: {}", args));
                false
            }
        }
    }

    fn prompt_dir(&mut self, args: &str) -> bool {
        if args.is_empty() {
            self.set_status(format!("Usage: dir <dir_name> — {}", PROMPT_USAGE));
            return false;
        }
        let target = self.resolve_goto(args);
        if target.is_dir() {
            self.go_to_dir(&target);
            return true;
        }
        // Teleport bookmarks still resolve here, then a name lookup under
        // the current directory.
        if let Some(path) = crate::teleport::TeleportManager::get_path(args) {
            if path.is_dir() {
                self.go_to_dir(&path);
                return true;
            }
        }
        match self.find_match(args, Some(true)) {
            Some(path) => {
                self.go_to_dir(&path);
                true
            }
            None => {
                self.set_status(format!("No such directory: {}", args));
                false
            }
        }
    }

    fn prompt_line(&mut self, args: &str) -> bool {
        if args.is_empty() {
            self.set_status(format!("Usage: line <+N|-N|N> — {}", PROMPT_USAGE));
            return false;
        }
        if self.jump_line(args) {
            return true;
        }
        self.set_status(format!("Invalid line target: {}", args));
        false
    }
}
