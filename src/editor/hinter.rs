//! Fish-style ghost hints and shell-style Tab completion for the editor's
//! command prompts.
//!
//! The four prompt buffers (`:`, `nfe:`, gmode `g` and query `/`) each carry
//! a caret, so the line can be edited in the middle. Completions work on the
//! token that ends at the caret: the hint bar paints the rest of the first
//! candidate dimmed at the caret, Tab completes as far as the candidates
//! agree (then cycles through them, like a shell), and Right / End / Ctrl+E
//! accept the hint when the caret is at the end of the line.

use std::path::{Path, PathBuf};

use crate::editor::{Editor, Mode};

/// `:` prompt verbs accepted by [`Editor::execute_command`].
const EDITOR_COMMANDS: &[&str] = &[
    "auto off",
    "auto on",
    "back",
    "blame",
    "cd ",
    "color off",
    "color on",
    "e",
    "e!",
    "exit",
    "f",
    "gd",
    "git blame",
    "git diff",
    "gos ",
    "gosc",
    "goto ",
    "gs ",
    "i",
    "l ",
    "line ",
    "mkdir ",
    "mkd ",
    "mkf ",
    "ne ",
    "new ",
    "q",
    "q!",
    "quit",
    "quit!",
    "r",
    "reload",
    "reload!",
    "rmd ",
    "rmf ",
    "run ",
    "syntax off",
    "syntax on",
    "theme ",
    "theme list",
    "theme ls",
    "touch ",
    "tp ",
    "tp-add ",
    "u",
    "w",
    "wq",
    "write",
    "x",
    "B",
    "H",
    "I",
    "V",
];

/// `nfe:` prompt verbs handled by `run_nfe_command`.
const NFE_COMMANDS: &[&str] = &["i", "m ", "n ", "rmd ", "rmf ", "size ", "v", "V", "I"];

/// gmode `g` prompt verbs handled by `execute_gmode`.
const GMODE_COMMANDS: &[&str] = &[
    "aqt", "e", "i", "m ", "n ", "q", "qt", "r", "t", "u", "v", "T",
];

/// Which directory entries a path completion should offer.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Want {
    Any,
    Dir,
    File,
}

/// One token's completion: where it starts in the prompt line (byte offset)
/// and every candidate that can replace it.
pub(crate) struct Completions {
    pub(crate) start: usize,
    pub(crate) tokens: Vec<String>,
}

impl Completions {
    fn new(start: usize, tokens: Vec<String>) -> Self {
        Self { start, tokens }
    }

    fn none() -> Self {
        Self {
            start: 0,
            tokens: Vec::new(),
        }
    }
}

// ── prompt editing (caret + completion) ─────────────────────────────────────

impl Editor {
    /// Buffer of the prompt that is open right now, if any.
    pub(crate) fn prompt_buf(&self) -> Option<&str> {
        match self.mode {
            Mode::Command => Some(&self.cmd_buf),
            Mode::Nfe => self.nfe_cmd.as_deref(),
            Mode::Gmode => Some(&self.gmode_buf),
            Mode::Query => Some(&self.query_buf),
            _ => None,
        }
    }

    pub(crate) fn prompt_buf_mut(&mut self) -> Option<&mut String> {
        match self.mode {
            Mode::Command => Some(&mut self.cmd_buf),
            Mode::Nfe => self.nfe_cmd.as_mut(),
            Mode::Gmode => Some(&mut self.gmode_buf),
            Mode::Query => Some(&mut self.query_buf),
            _ => None,
        }
    }

    /// Caret byte offset, clamped to the buffer length and a char boundary.
    pub(crate) fn prompt_pos(&self) -> usize {
        let Some(buf) = self.prompt_buf() else {
            return 0;
        };
        let mut p = self.prompt_cursor.min(buf.len());
        while p > 0 && !buf.is_char_boundary(p) {
            p -= 1;
        }
        p
    }

    /// Completions for the token that ends at the caret.
    pub(crate) fn prompt_completions(&self) -> Option<Completions> {
        let buf = self.prompt_buf()?;
        let seed = &buf[..self.prompt_pos()];
        Some(match self.mode {
            Mode::Command => command_completions(seed),
            Mode::Nfe => nfe_completions(&self.sidebar.root, seed),
            Mode::Gmode => gmode_completions(seed),
            Mode::Query => query_completions(seed, &self.qal_aliases),
            _ => return None,
        })
    }

    /// Candidates that extend the token at the caret: the ghost and the Tab
    /// cycle both work on this list. Completing in the middle of a word
    /// would insert text before characters that no longer match, so the
    /// caret must sit at a word boundary.
    fn prompt_candidates(&self) -> Option<(usize, String, Vec<String>, bool)> {
        let buf = self.prompt_buf()?;
        let pos = self.prompt_pos();
        if !buf[pos..].chars().next().map_or(true, char::is_whitespace) {
            return None;
        }
        let comp = self.prompt_completions()?;
        let start = comp.start.min(pos);
        let typed = buf[start..pos].to_string();
        // A token that is already a complete candidate (`:w`, `gmode q`) has
        // nothing left to type, so it gets no hint.
        let complete = comp.tokens.iter().any(|t| *t == typed);
        let tokens: Vec<String> = comp
            .tokens
            .into_iter()
            .filter(|t| t.len() > typed.len() && t.starts_with(&typed))
            .collect();
        Some((start, typed, tokens, complete))
    }

    /// Index of the candidate the hint shows: the one Tab last cycled to
    /// when that cycle still applies, otherwise the first.
    fn prompt_cycle_pick(&self, start: usize, typed: &str, tokens: &[String]) -> usize {
        if self.prompt_cycle_start == start
            && self.prompt_cycle_tokens == tokens
            && self.prompt_cycle < tokens.len()
            && tokens[self.prompt_cycle].starts_with(typed)
        {
            return self.prompt_cycle;
        }
        0
    }

    /// Whether Tab is currently cycling the candidates of this token.
    fn prompt_cycling(&self, start: usize, tokens: &[String]) -> bool {
        self.prompt_cycle_start == start
            && self.prompt_cycle_tokens == tokens
            && self.prompt_cycle < tokens.len()
    }

    /// Remainder of the hinted candidate, drawn dimmed at the caret.
    /// Right accepts it; Tab cycles to the next candidate.
    pub(crate) fn prompt_ghost(&self) -> Option<String> {
        let (start, typed, tokens, complete) = self.prompt_candidates()?;
        let cycling = self.prompt_cycling(start, &tokens);
        // Tab can still take a complete command to a longer one (`w` →
        // `write`), but the resting hint never suggests it.
        if complete && !cycling {
            return None;
        }
        let idx = self.prompt_cycle_pick(start, &typed, &tokens);
        let token = tokens.get(idx)?;
        Some(token[typed.len()..].to_string())
    }

    /// Insert `text` at the caret (typing, pasting, accepting a hint).
    pub(crate) fn prompt_insert(&mut self, text: &str) {
        let pos = self.prompt_pos();
        if let Some(buf) = self.prompt_buf_mut() {
            buf.insert_str(pos, text);
        }
        self.prompt_cursor = pos + text.len();
        self.prompt_cycle = usize::MAX;
    }

    pub(crate) fn prompt_push(&mut self, c: char) {
        let mut small = [0u8; 4];
        self.prompt_insert(c.encode_utf8(&mut small));
    }

    pub(crate) fn prompt_backspace(&mut self) {
        let pos = self.prompt_pos();
        if pos == 0 {
            return;
        }
        let Some(buf) = self.prompt_buf() else {
            return;
        };
        let mut prev = pos - 1;
        while prev > 0 && !buf.is_char_boundary(prev) {
            prev -= 1;
        }
        if let Some(buf) = self.prompt_buf_mut() {
            buf.replace_range(prev..pos, "");
        }
        self.prompt_cursor = prev;
        self.prompt_cycle = usize::MAX;
    }

    /// Forward delete (`Delete` key) at the caret.
    pub(crate) fn prompt_delete(&mut self) {
        let pos = self.prompt_pos();
        let Some(buf) = self.prompt_buf() else {
            return;
        };
        if pos >= buf.len() {
            return;
        }
        let mut next = pos + 1;
        while next < buf.len() && !buf.is_char_boundary(next) {
            next += 1;
        }
        if let Some(buf) = self.prompt_buf_mut() {
            buf.replace_range(pos..next, "");
        }
        self.prompt_cycle = usize::MAX;
    }

    pub(crate) fn prompt_move_left(&mut self) {
        let pos = self.prompt_pos();
        if pos == 0 {
            return;
        }
        let Some(buf) = self.prompt_buf() else {
            return;
        };
        let mut prev = pos - 1;
        while prev > 0 && !buf.is_char_boundary(prev) {
            prev -= 1;
        }
        self.prompt_cursor = prev;
        self.prompt_cycle = usize::MAX;
    }

    pub(crate) fn prompt_move_right(&mut self) {
        let pos = self.prompt_pos();
        let Some(buf) = self.prompt_buf() else {
            return;
        };
        if pos >= buf.len() {
            return;
        }
        let mut next = pos + 1;
        while next < buf.len() && !buf.is_char_boundary(next) {
            next += 1;
        }
        self.prompt_cursor = next;
        self.prompt_cycle = usize::MAX;
    }

    pub(crate) fn prompt_home(&mut self) {
        self.prompt_cursor = 0;
        self.prompt_cycle = usize::MAX;
    }

    pub(crate) fn prompt_end(&mut self) {
        self.prompt_cursor = self.prompt_buf().map(|b| b.len()).unwrap_or(0);
        self.prompt_cycle = usize::MAX;
    }

    /// Right arrow: accept the hint when the caret is at the end of the
    /// line, otherwise move one character right.
    pub(crate) fn prompt_accept_or_right(&mut self) {
        let len = self.prompt_buf().map(|b| b.len()).unwrap_or(0);
        if self.prompt_pos() == len {
            if let Some(ghost) = self.prompt_ghost() {
                self.prompt_insert(&ghost);
                return;
            }
        }
        self.prompt_move_right();
    }

    /// End / Ctrl+E: accept the hint when the caret is at the end of the
    /// line, otherwise jump the caret to the end like End always did.
    pub(crate) fn prompt_accept_or_end(&mut self) {
        let len = self.prompt_buf().map(|b| b.len()).unwrap_or(0);
        if self.prompt_pos() == len {
            if let Some(ghost) = self.prompt_ghost() {
                self.prompt_insert(&ghost);
                return;
            }
        }
        self.prompt_end();
    }

    /// Shell-style Tab: step the hint to the next candidate for the token at
    /// the caret, wrapping around. The buffer is left alone — the hint is
    /// what changes — and Right accepts whatever is hinted.
    pub(crate) fn prompt_tab(&mut self) {
        let Some((start, typed, tokens, _)) = self.prompt_candidates() else {
            return;
        };
        if tokens.is_empty() {
            return;
        }
        let idx = (self.prompt_cycle_pick(start, &typed, &tokens) + 1) % tokens.len();
        self.prompt_cycle = idx;
        self.prompt_cycle_start = start;
        self.prompt_cycle_tokens = tokens;
    }
}

// ── candidate sources ───────────────────────────────────────────────────────

/// Locate the last token of a prompt line and split off what precedes it.
fn split_last_token(input: &str) -> (&str, &str) {
    match input.rfind(char::is_whitespace) {
        Some(i) => {
            let split = i + input[i..].chars().next().map_or(1, char::len_utf8);
            (&input[..split], &input[split..])
        }
        None => ("", input),
    }
}

/// Candidates for a bare word token (verbs, names, aliases).
fn word_completions<'a>(input: &str, words: impl IntoIterator<Item = &'a str>) -> Completions {
    let (head, token) = split_last_token(input);
    if token.is_empty() {
        return Completions::none();
    }
    let tokens = words
        .into_iter()
        .filter(|w| w.starts_with(token))
        .map(str::to_string)
        .collect();
    Completions::new(head.len(), tokens)
}

/// Complete the last token against directory entry names under `root`.
/// Candidates are full tokens (directory part included); a directory gets
/// a trailing `/` so the next component can be completed right away.
fn path_completions(root: &Path, token: &str, want: Want) -> Vec<String> {
    let (dir_part, name_part) = match token.rfind(['/', '\\']) {
        Some(i) => token.split_at(i + 1),
        None => ("", token),
    };
    let dir: PathBuf = if dir_part.is_empty() {
        root.to_path_buf()
    } else if dir_part.starts_with('/') || dir_part.starts_with('\\') || dir_part.contains(':') {
        PathBuf::from(dir_part)
    } else {
        root.join(dir_part)
    };
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return Vec::new();
    };
    let mut names: Vec<(String, bool)> = entries
        .flatten()
        .filter_map(|entry| {
            let name = entry.file_name().to_string_lossy().to_string();
            if !name.starts_with(name_part) {
                return None;
            }
            let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
            // A file that is already fully typed has nothing left to complete.
            if name == name_part && !name_part.is_empty() && !is_dir {
                return None;
            }
            match want {
                Want::Dir if !is_dir => return None,
                Want::File if is_dir => return None,
                _ => {}
            }
            Some((name, is_dir))
        })
        .collect();
    names.sort_by_key(|(name, _)| name.to_lowercase());
    names
        .into_iter()
        .map(|(name, is_dir)| {
            let mut full = format!("{}{}", dir_part, name);
            if is_dir {
                full.push('/');
            }
            full
        })
        .collect()
}

fn cwd() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn command_completions(input: &str) -> Completions {
    if input.is_empty() {
        return Completions::none();
    }
    if !input.contains(char::is_whitespace) {
        // Verbs are completed from the start of the line so multi-word
        // commands (`syntax off`, `git diff`) stay one token.
        let tokens: Vec<String> = EDITOR_COMMANDS
            .iter()
            .filter(|w| w.starts_with(input))
            .map(|w| w.to_string())
            .collect();
        return Completions::new(0, tokens);
    }
    // Still inside a multi-word verb (`syntax o`, `git d`): complete it
    // against the whole line. Once the line *is* the verb (`cd `), the
    // argument token takes over instead.
    let phrases: Vec<String> = EDITOR_COMMANDS
        .iter()
        .filter(|w| w.contains(' ') && w.starts_with(input) && **w != input)
        .map(|w| w.to_string())
        .collect();
    if !phrases.is_empty() {
        return Completions::new(0, phrases);
    }
    let Some((verb, args)) = input.split_once(char::is_whitespace) else {
        return Completions::none();
    };
    let args = args.trim_start();
    let tokens = match verb {
        "git" => {
            let token = tail_token(args);
            ["blame", "diff"]
                .into_iter()
                .filter(|w| w.starts_with(token))
                .map(str::to_string)
                .collect()
        }
        "cd" | "gos" => path_completions(&cwd(), tail_token(args), Want::Dir),
        "theme" => {
            let names = crate::utils::theme::ThemeManager::list_themes();
            named_completions(args, names.iter().map(String::as_str))
        }
        "tp" => {
            let names = crate::teleport::TeleportManager::get_all();
            named_completions(args, names.keys().map(String::as_str))
        }
        "run" | "ne" | "touch" | "new" | "mkf" | "rmf" => {
            path_completions(&cwd(), tail_token(args), Want::Any)
        }
        "mkdir" | "mkd" | "rmd" => path_completions(&cwd(), tail_token(args), Want::Dir),
        _ => Vec::new(),
    };
    let start = input.len().saturating_sub(tail_token(args).len());
    Completions::new(start, tokens)
}

fn tail_token(args: &str) -> &str {
    split_last_token(args).1
}

/// Names that extend the argument token. An empty token hints every name,
/// so `theme ` / `tp ` show their first entry before anything is typed.
fn named_completions<'a>(args: &str, names: impl IntoIterator<Item = &'a str>) -> Vec<String> {
    let token = tail_token(args);
    names
        .into_iter()
        .filter(|n| n.starts_with(token))
        .map(str::to_string)
        .collect()
}

fn nfe_completions(root: &Path, input: &str) -> Completions {
    if input.is_empty() {
        return Completions::none();
    }
    if !input.contains(char::is_whitespace) {
        return word_completions(input, NFE_COMMANDS.iter().copied());
    }
    let Some((verb, args)) = input.split_once(char::is_whitespace) else {
        return Completions::none();
    };
    let args = args.trim_start();
    let tokens = match verb {
        "size" => ["default", "def", "full"]
            .into_iter()
            .filter(|w| w.starts_with(tail_token(args)))
            .map(str::to_string)
            .collect(),
        "n" => path_completions(root, tail_token(args), Want::Any),
        "m" | "rmd" => path_completions(root, tail_token(args), Want::Dir),
        "rmf" => path_completions(root, tail_token(args), Want::File),
        _ => Vec::new(),
    };
    let start = input.len().saturating_sub(tail_token(args).len());
    Completions::new(start, tokens)
}

fn gmode_completions(input: &str) -> Completions {
    if input.is_empty() {
        return Completions::none();
    }
    if !input.contains(char::is_whitespace) {
        return word_completions(input, GMODE_COMMANDS.iter().copied());
    }
    let Some((verb, args)) = input.split_once(char::is_whitespace) else {
        return Completions::none();
    };
    let args = args.trim_start();
    let tokens = match verb {
        "n" => path_completions(&cwd(), tail_token(args), Want::Any),
        "m" => path_completions(&cwd(), tail_token(args), Want::Dir),
        _ => Vec::new(),
    };
    let start = input.len().saturating_sub(tail_token(args).len());
    Completions::new(start, tokens)
}

fn query_completions(
    input: &str,
    aliases: &std::collections::HashMap<String, String>,
) -> Completions {
    // Only the verb of the command last in a `;` chain is completed.
    let seg_off = input.rfind(';').map(|i| i + 1).unwrap_or(0);
    let segment = &input[seg_off..];
    let lead = segment.len() - segment.trim_start().len();
    let body = segment.trim_start();
    let stripped = body.strip_prefix('/').unwrap_or(body);
    let start = seg_off + lead + (body.len() - stripped.len());
    if stripped.is_empty() || stripped.contains(char::is_whitespace) {
        return Completions::none();
    }
    let mut names: Vec<&str> = Editor::RESERVED_VERBS.to_vec();
    names.extend(aliases.keys().map(String::as_str));
    let tokens = names
        .into_iter()
        .filter(|n| n.starts_with(stripped))
        .map(str::to_string)
        .collect();
    Completions::new(start, tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn temp_dir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "ntc_hinter_{}_{}",
            tag,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("src")).unwrap();
        std::fs::create_dir_all(dir.join("assets")).unwrap();
        std::fs::write(dir.join("main.rs"), "fn main() {}").unwrap();
        dir
    }

    /// Mirrors `Editor::prompt_ghost`: a complete token gets no hint, else
    /// the first candidate that is strictly longer than what is typed.
    fn ghost(input: &str, comp: Completions) -> Option<String> {
        let typed = &input[comp.start.min(input.len())..];
        if comp.tokens.iter().any(|t| t == typed) {
            return None;
        }
        comp.tokens
            .iter()
            .find(|t| t.len() > typed.len() && t.starts_with(typed))
            .map(|t| t[typed.len()..].to_string())
    }

    #[test]
    fn command_verbs_and_arguments_get_hints() {
        assert_eq!(
            ghost("wr", command_completions("wr")),
            Some("ite".to_string())
        );
        assert_eq!(
            ghost("sy", command_completions("sy")),
            Some("ntax off".to_string())
        );
        assert_eq!(
            ghost("th", command_completions("th")),
            Some("eme ".to_string())
        );
        assert_eq!(
            ghost("git ", command_completions("git ")),
            Some("blame".to_string())
        );
        assert_eq!(
            ghost("git d", command_completions("git d")),
            Some("iff".to_string())
        );
        // A complete command gets no hint, even when a longer verb shares
        // its prefix (`w` / `write`).
        assert_eq!(ghost("w", command_completions("w")), None);
        assert_eq!(ghost("e", command_completions("e")), None);
        // Nothing typed after the verb still hints the first argument, the
        // way a shell shows a directory once `cd ` is on the line.
        assert!(ghost("cd ", command_completions("cd ")).is_some());
        assert_eq!(ghost("", command_completions("")), None);
        assert_eq!(ghost("nonsense", command_completions("nonsense")), None);
    }

    #[test]
    fn path_hints_respect_file_and_dir_wants() {
        let dir = temp_dir("paths");
        let any = path_completions(&dir, "ma", Want::Any);
        assert_eq!(any, vec!["main.rs".to_string()]);
        let dirs = path_completions(&dir, "sr", Want::Dir);
        assert_eq!(dirs, vec!["src/".to_string()]);
        assert!(path_completions(&dir, "sr", Want::File).is_empty());
        assert!(path_completions(&dir, "main.rs", Want::Any).is_empty());
        assert!(path_completions(&dir, "missing", Want::Any).is_empty());
        // An empty token lists everything (dirs first in sort order), so a
        // bare `cd ` / `theme ` hints an entry before anything is typed.
        assert_eq!(
            path_completions(&dir, "", Want::Any),
            vec![
                "assets/".to_string(),
                "main.rs".to_string(),
                "src/".to_string()
            ]
        );
        assert_eq!(
            path_completions(&dir, "", Want::Dir),
            vec!["assets/".to_string(), "src/".to_string()]
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn nfe_hints_cover_verbs_args_and_paths() {
        let dir = temp_dir("nfe");
        assert_eq!(
            ghost("si", nfe_completions(&dir, "si")),
            Some("ze ".to_string())
        );
        assert_eq!(
            ghost("rm", nfe_completions(&dir, "rm")),
            Some("d ".to_string())
        );
        assert_eq!(
            ghost("n ma", nfe_completions(&dir, "n ma")),
            Some("in.rs".to_string())
        );
        assert_eq!(
            ghost("m sr", nfe_completions(&dir, "m sr")),
            Some("c/".to_string())
        );
        // The trailing space of `n ` is itself the hint for the argument.
        assert_eq!(
            ghost("n", nfe_completions(&dir, "n")),
            Some(" ".to_string())
        );
        assert_eq!(
            ghost("m", nfe_completions(&dir, "m")),
            Some(" ".to_string())
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn gmode_hints_cover_verbs_and_new_entries() {
        assert_eq!(ghost("a", gmode_completions("a")), Some("qt".to_string()));
        assert_eq!(ghost("q", gmode_completions("q")), None);
        assert_eq!(ghost("t", gmode_completions("t")), None);
        assert_eq!(ghost("zz", gmode_completions("zz")), None);
    }

    #[test]
    fn query_hints_cover_verbs_and_aliases() {
        let mut aliases = HashMap::new();
        aliases.insert("mine".to_string(), "/s \"x\"".to_string());
        assert_eq!(
            ghost("upp", query_completions("upp", &aliases)),
            Some("er".to_string())
        );
        assert_eq!(
            ghost("/upp", query_completions("/upp", &aliases)),
            Some("er".to_string())
        );
        assert_eq!(
            ghost("mi", query_completions("mi", &aliases)),
            Some("ne".to_string())
        );
        assert_eq!(ghost("sa", query_completions("sa", &aliases)), None);
        assert_eq!(ghost("s a", query_completions("s a", &aliases)), None);
    }

    fn prompt_editor(tag: &str) -> (PathBuf, Editor) {
        let dir = temp_dir(tag);
        let path = dir.join("main.rs");
        std::fs::write(&path, "fn main() {}").unwrap();
        let editor = Editor::new(&path).unwrap();
        (dir, editor)
    }

    #[test]
    fn prompt_ghost_follows_the_active_prompt() {
        let (dir, mut editor) = prompt_editor("dispatch");
        editor.mode = Mode::Command;
        editor.cmd_buf = "wr".to_string();
        editor.prompt_cursor = 2;
        assert_eq!(editor.prompt_ghost(), Some("ite".to_string()));
        editor.mode = Mode::Gmode;
        editor.gmode_buf = "a".to_string();
        editor.prompt_cursor = 1;
        assert_eq!(editor.prompt_ghost(), Some("qt".to_string()));
        editor.mode = Mode::Nfe;
        editor.nfe_cmd = Some("si".to_string());
        editor.prompt_cursor = 2;
        assert_eq!(editor.prompt_ghost(), Some("ze ".to_string()));
        editor.nfe_cmd = None;
        assert_eq!(editor.prompt_ghost(), None);
        editor.mode = Mode::Query;
        editor.query_buf = "upp".to_string();
        editor.prompt_cursor = 3;
        assert_eq!(editor.prompt_ghost(), Some("er".to_string()));
        editor.mode = Mode::Normal;
        assert_eq!(editor.prompt_ghost(), None);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn prompt_ghost_ignores_the_middle_of_a_word() {
        let (dir, mut editor) = prompt_editor("midword");
        editor.mode = Mode::Command;
        editor.cmd_buf = "writes".to_string();
        editor.prompt_cursor = 2;
        assert_eq!(editor.prompt_ghost(), None);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn prompt_editing_moves_the_caret() {
        let (dir, mut editor) = prompt_editor("caret");
        editor.mode = Mode::Command;
        editor.prompt_insert("write");
        assert_eq!(editor.cmd_buf, "write");
        assert_eq!(editor.prompt_cursor, 5);
        editor.prompt_move_left();
        editor.prompt_move_left();
        editor.prompt_insert("s");
        assert_eq!(editor.cmd_buf, "wriste");
        assert_eq!(editor.prompt_cursor, 4);
        editor.prompt_backspace();
        assert_eq!(editor.cmd_buf, "write");
        editor.prompt_end();
        editor.prompt_backspace();
        assert_eq!(editor.cmd_buf, "writ");
        editor.prompt_home();
        editor.prompt_delete();
        assert_eq!(editor.cmd_buf, "rit");
        editor.prompt_move_right();
        assert_eq!(editor.prompt_pos(), 1);
        // Multi-byte characters step by whole characters.
        editor.cmd_buf = "αβγ".to_string();
        editor.prompt_cursor = 4;
        editor.prompt_move_left();
        assert_eq!(editor.prompt_pos(), 2);
        editor.prompt_backspace();
        assert_eq!(editor.cmd_buf, "βγ");
        editor.prompt_end();
        editor.prompt_backspace();
        assert_eq!(editor.cmd_buf, "β");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn tab_cycles_the_hint_without_touching_the_buffer() {
        let (dir, mut editor) = prompt_editor("tab");
        editor.mode = Mode::Command;
        editor.prompt_insert("sy");
        assert_eq!(editor.prompt_ghost(), Some("ntax off".to_string()));
        // Tab walks the candidates; the line itself never changes.
        editor.prompt_tab();
        assert_eq!(editor.cmd_buf, "sy");
        assert_eq!(editor.prompt_ghost(), Some("ntax on".to_string()));
        editor.prompt_tab();
        assert_eq!(editor.cmd_buf, "sy");
        assert_eq!(editor.prompt_ghost(), Some("ntax off".to_string()));
        // Right commits whatever the hint shows.
        editor.prompt_tab();
        editor.prompt_accept_or_right();
        assert_eq!(editor.cmd_buf, "syntax on");
        // A single candidate has nothing to cycle to, but Right accepts it.
        editor.cmd_buf.clear();
        editor.prompt_cursor = 0;
        editor.prompt_insert("wr");
        editor.prompt_tab();
        assert_eq!(editor.cmd_buf, "wr");
        assert_eq!(editor.prompt_ghost(), Some("ite".to_string()));
        editor.prompt_accept_or_right();
        assert_eq!(editor.cmd_buf, "write");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn typing_restarts_the_hint_cycle() {
        let (dir, mut editor) = prompt_editor("tab-reset");
        editor.mode = Mode::Command;
        editor.prompt_insert("sy");
        editor.prompt_tab();
        assert_eq!(editor.prompt_ghost(), Some("ntax on".to_string()));
        editor.prompt_push('n');
        assert_eq!(editor.cmd_buf, "syn");
        assert_eq!(editor.prompt_ghost(), Some("tax off".to_string()));
        // `:theme` cycles through the three theme verbs.
        editor.cmd_buf.clear();
        editor.prompt_cursor = 0;
        editor.prompt_insert("th");
        assert_eq!(editor.prompt_ghost(), Some("eme ".to_string()));
        editor.prompt_tab();
        assert_eq!(editor.prompt_ghost(), Some("eme list".to_string()));
        editor.prompt_tab();
        assert_eq!(editor.prompt_ghost(), Some("eme ls".to_string()));
        editor.prompt_tab();
        assert_eq!(editor.prompt_ghost(), Some("eme ".to_string()));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn right_accepts_the_hint_only_at_the_end() {
        let (dir, mut editor) = prompt_editor("right");
        editor.mode = Mode::Command;
        editor.prompt_insert("wr");
        editor.prompt_accept_or_right();
        assert_eq!(editor.cmd_buf, "write");
        assert_eq!(editor.prompt_cursor, 5);
        // Mid-line Right just moves.
        editor.prompt_home();
        editor.prompt_accept_or_right();
        assert_eq!(editor.cmd_buf, "write");
        assert_eq!(editor.prompt_cursor, 1);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn end_and_ctrl_e_accept_the_hint_like_right() {
        let (dir, mut editor) = prompt_editor("end-accept");
        editor.mode = Mode::Command;
        editor.prompt_insert("wr");
        assert_eq!(editor.prompt_ghost(), Some("ite".to_string()));
        editor.prompt_accept_or_end();
        assert_eq!(editor.cmd_buf, "write");
        assert_eq!(editor.prompt_cursor, 5);
        // With nothing to complete End still just jumps to the end.
        editor.cmd_buf.clear();
        editor.prompt_cursor = 0;
        editor.prompt_insert("quit");
        editor.prompt_home();
        editor.prompt_accept_or_end();
        assert_eq!(editor.prompt_cursor, 4);
        // Mid-line End still just jumps to the end of the line.
        editor.cmd_buf.clear();
        editor.prompt_cursor = 0;
        editor.prompt_insert("wr");
        editor.prompt_home();
        editor.prompt_accept_or_end();
        assert_eq!(editor.cmd_buf, "wr");
        assert_eq!(editor.prompt_cursor, 2);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
