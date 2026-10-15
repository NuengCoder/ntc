use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::editor::{ci_find, sel_range, Editor, Mode};
use crate::t;

impl Editor {
    pub(crate) fn handle_query_event(&mut self, ev: Event) -> Result<()> {
        match ev {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => {
                let ctrl = modifiers.contains(KeyModifiers::CONTROL);
                match (ctrl, code) {
                    (true, KeyCode::Char('f')) => {
                        self.mode = Mode::Search;
                        if self.has_selection() {
                            self.search.query = self.selected_text();
                        } else {
                            self.search.query.clear();
                        }
                        self.search.matches.clear();
                        self.search.match_idx = 0;
                        self.rebuild_search_matches();
                        if !self.search.matches.is_empty() {
                            self.jump_to_match(0);
                        }
                        self.mark_all_dirty();
                    }
                    (true, KeyCode::Char('j')) => {
                        self.prev_mode = Mode::Normal;
                        self.mode = Mode::Jump;
                        self.jump_buf.clear();
                        self.status_msg = None;
                    }
                    (true, KeyCode::Char('Q')) => {
                        self.close_all_tabs();
                        self.mode = Mode::Normal;
                    }
                    _ => match code {
                        // Esc alone exits: plain `q` must stay typeable
                        // (`qal …`, `/quit`). The result message goes away
                        // too so Normal doesn't inherit a stale `✦`.
                        KeyCode::Esc => {
                            self.query_buf.clear();
                            self.status_msg = None;
                            self.mode = self.prev_mode.clone();
                            self.mark_all_dirty();
                        }
                        KeyCode::Enter => {
                            let q = self.query_buf.clone();
                            self.query_buf.clear();
                            self.execute_query(&q);
                            self.mark_all_dirty();
                        }
                        KeyCode::Backspace => {
                            self.query_buf.pop();
                            // Composing the next command retires the
                            // previous result message.
                            self.status_msg = None;
                        }
                        KeyCode::Up if !self.qal_aliases.is_empty() => {
                            self.query_buf.clear();
                            self.status_msg = None;
                        }
                        KeyCode::Char(c) => {
                            self.query_buf.push(c);
                            self.status_msg = None;
                        }
                        _ => {}
                    },
                }
            }
            Event::Resize(w, h) => {
                self.term_w = w as usize;
                self.term_h = h as usize;
            }
            _ => {}
        }
        Ok(())
    }

    fn execute_query(&mut self, q: &str) {
        let trimmed = q.trim();
        if trimmed.is_empty() {
            return;
        }

        let commands: Vec<&str> = trimmed
            .split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        for cmd in commands {
            if !self.exec_one(cmd) {
                break;
            }
        }
    }

    /// Verbs that can never become qal aliases (case-insensitive).
    /// Bare words matching these always dispatch as commands, so the
    /// leading `/` is optional everywhere.
    const RESERVED_VERBS: &[&str] = &[
        "s", "l", "sa", "sal", "sl", "rp", "c", "cs", "add", "rm", "set", "z", "upper", "up",
        "lower", "low", "undo", "redo", "exit", "quit", "help", "qal",
    ];

    fn exec_one(&mut self, cmd: &str) -> bool {
        // The leading `/` is optional: `/s "hi"` and `s "hi"` are the same.
        // Reserved verbs always dispatch as commands; anything else is a
        // qal alias invocation (`name [args]`, `/name [args]`).
        let stripped = cmd.strip_prefix('/').unwrap_or(cmd);
        let parts: Vec<&str> = stripped.splitn(2, ' ').collect();
        let verb = parts[0];
        let args = parts.get(1).unwrap_or(&"").trim();
        match verb {
            "s" => {
                self.cmd_select_next(args);
                true
            }
            "l" => {
                self.cmd_goto_line(args);
                true
            }
            "sa" => {
                self.cmd_select_all_occ(args);
                true
            }
            "sal" => {
                self.clear_extra_cursors();
                self.select_all();
                self.status_msg = Some(t!("editor.query_selected_all_lines"));
                true
            }
            "sl" => {
                self.cmd_select_lines_new(args);
                true
            }
            "rp" => {
                self.cmd_replace_sel(args);
                true
            }
            "c" | "cs" => {
                self.clear_selection();
                self.clear_extra_cursors();
                self.status_msg = Some(t!("editor.query_selection_cleared"));
                true
            }
            "qal" => {
                self.cmd_qal(args);
                true
            }
            "exit" | "quit" => {
                self.query_buf.clear();
                self.mode = self.prev_mode.clone();
                self.mark_all_dirty();
                false
            }
            "help" => {
                self.mode = Mode::Help;
                self.status_msg = None;
                false
            }
            "undo" => {
                self.undo();
                true
            }
            "redo" => {
                self.redo();
                true
            }
            "upper" | "up" => {
                self.cmd_upper_lower(true);
                true
            }
            "lower" | "low" => {
                self.cmd_upper_lower(false);
                true
            }
            "z" => {
                self.cmd_add_cursors(args);
                true
            }
            "add" => {
                self.cmd_add(args);
                true
            }
            "rm" => {
                self.cmd_rm(args);
                true
            }
            "set" => {
                self.cmd_set(args);
                true
            }
            _ => {
                self.try_alias_or_unknown(stripped);
                true
            }
        }
    }

    /// True when `name` is reserved and can never become a qal alias.
    fn is_reserved(name: &str) -> bool {
        Self::RESERVED_VERBS
            .iter()
            .any(|v| v.eq_ignore_ascii_case(name))
    }

    /// Split `s` on whitespace, keeping `"quoted groups"` together
    /// (quotes stripped). Used by `/rp` (`/rp "foo bar" "baz"`).
    fn split_args(s: &str) -> Vec<String> {
        let mut out = Vec::new();
        let mut cur = String::new();
        let mut in_quotes = false;
        let mut has_token = false;
        for ch in s.chars() {
            if ch == '"' {
                in_quotes = !in_quotes;
                has_token = true;
                continue;
            }
            if ch.is_whitespace() && !in_quotes {
                if has_token {
                    out.push(std::mem::take(&mut cur));
                    has_token = false;
                }
                continue;
            }
            cur.push(ch);
            has_token = true;
        }
        if has_token {
            out.push(cur);
        }
        out
    }

    /// Strip one pair of surrounding double quotes, if present.
    fn unquote(s: &str) -> String {
        let t = s.trim();
        if t.len() >= 2 && t.starts_with('"') && t.ends_with('"') {
            t[1..t.len() - 1].to_string()
        } else {
            t.to_string()
        }
    }

    fn try_alias_or_unknown(&mut self, cmd: &str) {
        let parts: Vec<&str> = cmd.splitn(2, ' ').collect();
        let name = parts[0];
        let rest_args = parts.get(1).unwrap_or(&"");

        if self.qal_aliases.contains_key(name) {
            let stored = self.qal_aliases[name].clone();
            let expanded = self.expand_alias(&stored, rest_args);
            self.execute_query(&expanded);
        } else {
            self.status_msg = Some(t!("editor.query_unknown", cmd));
        }
    }

    fn cmd_alias_invoke(&mut self, args: &str) {
        let parts: Vec<&str> = args.splitn(2, ' ').collect();
        let name = parts[0];
        let rest_args = parts.get(1).unwrap_or(&"");

        if self.qal_aliases.contains_key(name) {
            let stored = self.qal_aliases[name].clone();
            let expanded = self.expand_alias(&stored, rest_args);
            self.execute_query(&expanded);
        } else {
            self.status_msg = Some(t!("editor.query_alias_not_found", name));
        }
    }

    fn expand_alias(&self, template: &str, args_str: &str) -> String {
        let args: Vec<&str> = args_str.split_whitespace().collect();
        let mut result = template.to_string();

        // First pass: simple ${N} replacement
        for (i, arg) in args.iter().enumerate() {
            let idx = i + 1;
            let var = format!("${{{}}}", idx);
            result = result.replace(&var, arg);
        }

        // Second pass: handle ${N:-default} and ${N[]}. Guarantees progress:
        // an unresolvable placeholder is kept literally and skipped past.
        let mut rest: &str = &result;
        let mut out = String::new();
        while let Some(start) = rest.find("${") {
            out.push_str(&rest[..start]);
            let after = &rest[start + 2..];
            let close = match after.find('}') {
                Some(c) => c,
                None => {
                    out.push_str(&rest[start..]);
                    return out;
                }
            };
            let var_content = &after[..close];
            let full_expr = format!("${{{}}}", var_content);

            let replacement: Option<String> = if let Some(dash_idx) = var_content.find(":-") {
                let num_str = &var_content[..dash_idx];
                let default = &var_content[dash_idx + 2..];
                match num_str.parse::<usize>() {
                    Ok(n) if n > 0 && n <= args.len() => Some(args[n - 1].to_string()),
                    Ok(_) => Some(default.to_string()),
                    Err(_) => None,
                }
            } else if let Some(bracket) = var_content.find("[]") {
                let num_str = &var_content[..bracket];
                match num_str.parse::<usize>() {
                    Ok(n) if n > 0 && n <= args.len() => Some(args[n - 1..].join(" ")),
                    Ok(_) => Some(String::new()),
                    Err(_) => None,
                }
            } else {
                match var_content.parse::<usize>() {
                    Ok(n) if n > 0 && n <= args.len() => Some(args[n - 1].to_string()),
                    _ => None,
                }
            };

            match replacement {
                Some(value) => out.push_str(&value),
                None => out.push_str(&full_expr),
            }
            rest = &after[close + 1..];
        }
        out.push_str(rest);
        out
    }

    /// Place the primary selection on `(y, s)..(y, e)` after clearing
    /// any existing selection and extra cursors.
    fn select_match_at(&mut self, y: usize, s: usize, e: usize) {
        self.clear_selection();
        self.clear_extra_cursors();
        self.cursor.y = y;
        self.cursor.byte = e;
        self.cursor.selection_anchor = Some((y, s));
    }

    /// `/s "text"`: select the next occurrence after the cursor,
    /// wrapping to the top. Bare `/s` selects the word under the cursor.
    fn cmd_select_next(&mut self, args: &str) {
        let pat = Self::unquote(args);
        if pat.is_empty() {
            match self.select_word_under_cursor() {
                Some(_) => {
                    self.status_msg = Some(t!("editor.query_selected_word"));
                    return;
                }
                None => {
                    self.status_msg = Some(t!("editor.query_s_usage"));
                    return;
                }
            }
        }

        if self.lines.is_empty() {
            self.status_msg = Some(t!("editor.query_no_matches", pat));
            return;
        }
        // Repeat invocations keep moving forward: start after the end of
        // the current same-line selection, else at the cursor.
        let (cy, start_b) = if self.has_selection() {
            let (ay, ab) = self.cursor.selection_anchor.unwrap();
            let ((sy, _), (ey, eb)) = sel_range(self.cursor.y, self.cursor.byte, ay, ab);
            if sy == ey && sy == self.cursor.y {
                (ey, eb.min(self.lines[ey].len()))
            } else {
                (
                    self.cursor.y,
                    self.cursor.byte.min(self.lines[self.cursor.y].len()),
                )
            }
        } else {
            (
                self.cursor.y,
                self.cursor.byte.min(self.lines[self.cursor.y].len()),
            )
        };

        // Current line from start, then lines below, then wrap to the top.
        let mut found: Option<(usize, usize, usize)> = None;
        for li in cy..self.lines.len() {
            let from = if li == cy { start_b } else { 0 };
            if let Some((s, e)) = ci_find(&self.lines[li], &pat, from) {
                found = Some((li, s, e));
                break;
            }
        }
        if found.is_none() {
            for li in 0..cy {
                if let Some((s, e)) = ci_find(&self.lines[li], &pat, 0) {
                    found = Some((li, s, e));
                    break;
                }
            }
        }
        match found {
            Some((y, s, e)) => {
                self.select_match_at(y, s, e);
                self.status_msg = Some(t!("editor.query_selected_next", pat));
            }
            None => {
                self.status_msg = Some(t!("editor.query_no_matches", pat));
            }
        }
    }

    /// `/l 10`: move the cursor to the start of line 10 (1-based, clamped).
    fn cmd_goto_line(&mut self, args: &str) {
        let n: usize = match args.split_whitespace().next().map(|t| t.parse()) {
            Some(Ok(n)) => n,
            _ => {
                self.status_msg = Some(t!("editor.query_l_usage"));
                return;
            }
        };
        if self.lines.is_empty() {
            return;
        }
        let y = n
            .max(1)
            .saturating_sub(1)
            .min(self.lines.len().saturating_sub(1));
        self.clear_selection();
        self.clear_extra_cursors();
        self.cursor.y = y;
        self.cursor.byte = 0;
        self.status_msg = Some(t!("editor.query_selected_line", y + 1));
    }

    /// `/sa "text"`: select every occurrence in the whole buffer as
    /// multi-cursors (primary on the first).
    fn cmd_select_all_occ(&mut self, args: &str) {
        let pat = Self::unquote(args);
        if pat.is_empty() {
            self.status_msg = Some(t!("editor.query_sa_usage"));
            return;
        }
        let mut matches: Vec<(usize, usize, usize)> = Vec::new();
        for (li, line) in self.lines.iter().enumerate() {
            let mut start = 0;
            while let Some((s, e)) = ci_find(line, &pat, start) {
                matches.push((li, s, e));
                start = s + 1;
                if start >= line.len() {
                    break;
                }
            }
        }
        if matches.is_empty() {
            self.status_msg = Some(t!("editor.query_no_matches", pat));
            return;
        }
        let (first_y, first_s, first_e) = matches[0];
        self.select_match_at(first_y, first_s, first_e);
        for &(my, ms, me) in &matches[1..] {
            self.cursor.extra_cursors.push(crate::editor::CursorPos {
                y: my,
                byte: me,
                anchor: Some((my, ms)),
            });
        }
        self.status_msg = Some(t!("editor.query_selected_occurrences", matches.len(), pat));
    }

    /// Select a whole line (anchor at col 0, cursor at EOL) without
    /// disturbing other cursors. Caller sets the primary first.
    fn push_line_cursor(&mut self, y: usize, primary: bool) {
        let end = self.lines[y].len();
        if primary {
            self.cursor.y = y;
            self.cursor.byte = end;
            self.cursor.selection_anchor = Some((y, 0));
        } else {
            self.cursor.extra_cursors.push(crate::editor::CursorPos {
                y,
                byte: end,
                anchor: Some((y, 0)),
            });
        }
    }

    /// `/sl 5` (one line), `/sl 5 10` or `/sl 10 5` (range, any order),
    /// `/sl 1 , 3 , 5` (discrete lines, any count).
    fn cmd_select_lines_new(&mut self, args: &str) {
        // Space out commas so `5,10` and `5 , 10` parse identically.
        let spaced = args.replace(',', " , ");
        let tokens: Vec<&str> = spaced.split_whitespace().collect();
        if tokens.is_empty() {
            self.status_msg = Some(t!("editor.query_sl_usage"));
            return;
        }
        let has_comma = tokens.iter().any(|t| *t == ",");
        let nums: Option<Vec<usize>> = tokens
            .iter()
            .filter(|t| **t != ",")
            .map(|t| t.parse::<usize>().ok())
            .collect();
        let Some(nums) = nums else {
            self.status_msg = Some(t!("editor.query_sl_usage"));
            return;
        };
        if nums.is_empty() || nums.iter().any(|n| *n == 0) {
            self.status_msg = Some(t!("editor.query_sl_usage"));
            return;
        }
        if self.lines.is_empty() {
            return;
        }
        let max_idx = self.lines.len().saturating_sub(1);
        let clamp = |n: usize| n.max(1).saturating_sub(1).min(max_idx);

        self.clear_selection();
        self.clear_extra_cursors();
        if has_comma {
            // Discrete lines: primary on the first, extras on the rest.
            let mut ys: Vec<usize> = nums.iter().map(|n| clamp(*n)).collect();
            ys.sort();
            ys.dedup();
            for (i, y) in ys.iter().enumerate() {
                self.push_line_cursor(*y, i == 0);
            }
            self.status_msg = Some(t!("editor.query_selected_lines", ys.len()));
        } else if nums.len() == 1 {
            let y = clamp(nums[0]);
            self.push_line_cursor(y, true);
            self.status_msg = Some(t!("editor.query_selected_line", y + 1));
        } else if nums.len() == 2 {
            let (mut a, mut b) = (clamp(nums[0]), clamp(nums[1]));
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }
            self.cursor.selection_anchor = Some((a, 0));
            self.cursor.y = b;
            self.cursor.byte = self.lines[b].len();
            self.status_msg = Some(t!("editor.query_selected_lines_range", a + 1, b + 1));
        } else {
            self.status_msg = Some(t!("editor.query_sl_usage"));
        }
    }

    /// `/rp <find> <replace>`: case-insensitive replace scoped to the
    /// current selection (selection is required). Quote multi-word
    /// patterns: `/rp "foo bar" "baz"`.
    fn cmd_replace_sel(&mut self, args: &str) {
        let parts = Self::split_args(args);
        if parts.len() != 2 {
            self.status_msg = Some(t!("editor.query_rp_usage"));
            return;
        }
        let (find, replace) = (parts[0].as_str(), parts[1].as_str());
        if find.is_empty() {
            self.status_msg = Some(t!("editor.query_empty_pattern"));
            return;
        }
        if !self.has_selection() {
            self.status_msg = Some(t!("editor.query_selection_required"));
            return;
        }

        let (ay, ab) = self.cursor.selection_anchor.unwrap();
        let ((sy, sb), (ey, eb)) = sel_range(self.cursor.y, self.cursor.byte, ay, ab);
        let (row_start, row_end, col_start, col_end) = (sy, ey, sb, eb);

        self.snapshot();
        let mut count = 0usize;

        for li in row_start..=row_end {
            let line = self.lines[li].clone();
            let line_start = if li == row_start {
                col_start.min(line.len())
            } else {
                0
            };
            let line_end = if li == row_end && col_end != usize::MAX {
                col_end.min(line.len())
            } else {
                line.len()
            };
            if line_start > line_end {
                continue;
            }
            let searchable = &line[line_start..line_end];

            let mut result = String::new();
            let mut pos = line_start;
            let mut found = false;
            loop {
                match ci_find(searchable, find, pos - line_start) {
                    Some((rel_s, rel_e)) => {
                        let abs_s = line_start + rel_s;
                        let abs_e = line_start + rel_e;
                        result.push_str(&line[pos..abs_s]);
                        result.push_str(replace);
                        pos = abs_e;
                        count += 1;
                        found = true;
                        if pos >= line_end {
                            break;
                        }
                    }
                    None => {
                        result.push_str(&line[pos..]);
                        break;
                    }
                }
            }

            if found {
                self.lines[li] = result;
                self.syntax.invalidate_line(li);
                self.mark_dirty(li);
            }
        }

        self.modified = true;
        self.search.mark_dirty();
        self.status_msg = Some(t!("editor.query_replaced_count", count));
    }

    fn cmd_qal(&mut self, args: &str) {
        let parts: Vec<&str> = args.splitn(2, ' ').collect();
        let subcmd = parts[0];
        let subargs = parts.get(1).unwrap_or(&"").trim();

        match subcmd {
            "add" => {
                let aparts: Vec<&str> = subargs.splitn(2, ' ').collect();
                if aparts.len() < 2 {
                    self.status_msg = Some(t!("editor.query_qal_add_usage"));
                    return;
                }
                let name = aparts[0];
                let query = aparts[1];
                if Self::is_reserved(name) {
                    self.status_msg = Some(t!("editor.query_qal_reserved", name));
                    return;
                }
                if self.qal_aliases.contains_key(name) {
                    self.status_msg = Some(t!("editor.query_qal_exists", name));
                    return;
                }
                self.qal_aliases.insert(name.to_string(), query.to_string());
                self.status_msg = Some(t!("editor.query_qal_added", name));
            }
            "rm" | "remove" => {
                if subargs.is_empty() {
                    self.status_msg = Some(t!("editor.query_qal_rm_usage"));
                    return;
                }
                if self.qal_aliases.remove(subargs).is_some() {
                    self.status_msg = Some(t!("editor.query_qal_removed", subargs));
                } else {
                    self.status_msg = Some(t!("editor.query_qal_not_found", subargs));
                }
            }
            "info" => {
                if subargs.is_empty() {
                    self.status_msg = Some(t!("editor.query_qal_info_usage"));
                    return;
                }
                match self.qal_aliases.get(subargs) {
                    Some(query) => {
                        self.status_msg = Some(format!("'{}' -> {}", subargs, query));
                    }
                    None => {
                        self.status_msg = Some(t!("editor.query_qal_not_found", subargs));
                    }
                }
            }
            "edit" => {
                let aparts: Vec<&str> = subargs.splitn(2, ' ').collect();
                if aparts.len() < 2 {
                    self.status_msg = Some(t!("editor.query_qal_edit_usage"));
                    return;
                }
                let name = aparts[0];
                let query = aparts[1];
                if self.qal_aliases.contains_key(name) {
                    self.qal_aliases.insert(name.to_string(), query.to_string());
                    self.status_msg = Some(t!("editor.query_qal_updated", name));
                } else {
                    self.status_msg = Some(t!("editor.query_qal_not_found", name));
                }
            }
            "rnm" | "rename" => {
                let rparts: Vec<&str> = subargs.splitn(3, ' ').collect();
                if rparts.len() < 3 || rparts[1] != "to" {
                    self.status_msg = Some(t!("editor.query_qal_rnm_usage"));
                    return;
                }
                let old = rparts[0];
                let new = rparts[2];
                if Self::is_reserved(new) {
                    self.status_msg = Some(t!("editor.query_qal_reserved", new));
                    return;
                }
                if self.qal_aliases.contains_key(new) {
                    self.status_msg = Some(t!("editor.query_qal_exists", new));
                    return;
                }
                if let Some(query) = self.qal_aliases.remove(old) {
                    self.qal_aliases.insert(new.to_string(), query);
                    self.status_msg = Some(t!("editor.query_qal_renamed", old, new));
                } else {
                    self.status_msg = Some(t!("editor.query_qal_not_found", old));
                }
            }
            "list" | "ls" => {
                if self.qal_aliases.is_empty() {
                    self.status_msg = Some(t!("editor.query_qal_no_aliases"));
                } else {
                    let mut sorted: Vec<_> = self.qal_aliases.iter().collect();
                    sorted.sort_by(|a, b| a.0.cmp(b.0));
                    let lines: Vec<String> = sorted
                        .iter()
                        .map(|(n, q)| format!("{} → {}", n, q))
                        .collect();
                    self.status_msg = Some(t!("editor.query_qal_list", lines.join(" | ")));
                }
            }
            "run" => {
                self.cmd_alias_invoke(subargs);
            }
            _ => {
                self.status_msg = Some(t!("editor.query_qal_help"));
            }
        }
    }

    fn cmd_add(&mut self, args: &str) {
        let args = args.trim();
        if args.is_empty() {
            self.status_msg = Some(t!("editor.query_add_usage"));
            return;
        }
        let sep = " to ";
        let pos = match args.rfind(sep) {
            Some(p) => p,
            None => {
                self.status_msg = Some(t!("editor.query_add_format"));
                return;
            }
        };
        let text = &args[..pos];
        let target = args[pos + sep.len()..].trim();
        if text.is_empty() || target.is_empty() {
            self.status_msg = Some(t!("editor.query_add_format"));
            return;
        }

        let is_end = target.eq_ignore_ascii_case("#end");
        let is_start = target.eq_ignore_ascii_case("#start") || target.eq_ignore_ascii_case("#hm");
        if !is_end && !is_start {
            self.status_msg = Some(t!("editor.query_add_position"));
            return;
        }

        if !self.has_selection() {
            self.status_msg = Some(t!("editor.query_selection_required"));
            return;
        }
        let (ay, ab) = self.cursor.selection_anchor.unwrap();
        let ((sy, _), (ey, _)) = sel_range(self.cursor.y, self.cursor.byte, ay, ab);
        let (row_start, row_end) = (sy, ey);

        self.snapshot();
        for li in row_start..=row_end {
            if is_end {
                self.lines[li].push_str(text);
            } else {
                self.lines[li].insert_str(0, text);
            }
            self.syntax.invalidate_line(li);
            self.mark_dirty(li);
        }
        self.modified = true;
        self.search.mark_dirty();
        self.status_msg = Some(t!("editor.query_added_text", row_end - row_start + 1));
    }

    /// `/rm <text> from #start|#hm` / `/rm <text> from #end`: exact inverse
    /// of `/add`. Strips one anchored occurrence per selected line (prefix
    /// for #start/#hm, suffix for #end); non-matching lines are skipped.
    /// Case-sensitive and literal. Selection is required; undo-able.
    fn cmd_rm(&mut self, args: &str) {
        let args = args.trim();
        if args.is_empty() {
            self.status_msg = Some(t!("editor.query_rm_usage"));
            return;
        }
        let sep = " from ";
        let pos = match args.rfind(sep) {
            Some(p) => p,
            None => {
                self.status_msg = Some(t!("editor.query_rm_format"));
                return;
            }
        };
        let text = &args[..pos];
        let target = args[pos + sep.len()..].trim();
        if text.is_empty() || target.is_empty() {
            self.status_msg = Some(t!("editor.query_rm_format"));
            return;
        }

        let is_end = target.eq_ignore_ascii_case("#end");
        let is_start = target.eq_ignore_ascii_case("#start") || target.eq_ignore_ascii_case("#hm");
        if !is_end && !is_start {
            self.status_msg = Some(t!("editor.query_add_position"));
            return;
        }

        if !self.has_selection() {
            self.status_msg = Some(t!("editor.query_selection_required"));
            return;
        }
        let (ay, ab) = self.cursor.selection_anchor.unwrap();
        let ((sy, _), (ey, _)) = sel_range(self.cursor.y, self.cursor.byte, ay, ab);
        let (row_start, row_end) = (sy, ey);

        self.snapshot();
        let mut removed = 0usize;
        for li in row_start..=row_end {
            let stripped: Option<String> = if is_end {
                self.lines[li].strip_suffix(text).map(|s| s.to_string())
            } else {
                self.lines[li].strip_prefix(text).map(|s| s.to_string())
            };
            if let Some(next) = stripped {
                self.lines[li] = next;
                self.syntax.invalidate_line(li);
                self.mark_dirty(li);
                removed += 1;
            }
        }
        self.modified = true;
        self.search.mark_dirty();
        let skipped = (row_end - row_start + 1).saturating_sub(removed);
        self.status_msg = Some(t!("editor.query_removed_text", removed, skipped));
    }

    fn cmd_set(&mut self, args: &str) {
        let args = args.trim();
        if args.is_empty() {
            self.status_msg = Some(t!("editor.query_set_usage"));
            return;
        }
        let sep = " to ";
        let pos = match args.rfind(sep) {
            Some(p) => p,
            None => {
                self.status_msg = Some(t!("editor.query_set_format"));
                return;
            }
        };
        let pattern = &args[..pos];
        let target = args[pos + sep.len()..].trim();
        if pattern.is_empty() || target.is_empty() {
            self.status_msg = Some(t!("editor.query_set_format"));
            return;
        }

        let is_end = target.eq_ignore_ascii_case("#end");
        let is_start = target.eq_ignore_ascii_case("#start") || target.eq_ignore_ascii_case("#hm");
        if !is_end && !is_start {
            self.status_msg = Some(t!("editor.query_add_position"));
            return;
        }

        if !self.has_selection() {
            self.status_msg = Some(t!("editor.query_selection_required"));
            return;
        }
        let (ay, ab) = self.cursor.selection_anchor.unwrap();
        let ((sy, _), (ey, _)) = sel_range(self.cursor.y, self.cursor.byte, ay, ab);
        let (row_start, row_end) = (sy, ey);

        // Collect (line_index, first_occurrence_byte) for all matching lines
        let pat_lower = pattern.to_lowercase();
        let mut matches: Vec<(usize, usize)> = Vec::new();
        for li in row_start..=row_end {
            let lower = self.lines[li].to_lowercase();
            if let Some(offset) = lower.find(&pat_lower) {
                matches.push((li, offset));
            }
        }

        if matches.is_empty() {
            self.status_msg = Some(t!("editor.query_no_pattern_matches", pattern));
            return;
        }

        self.snapshot();

        if is_end {
            let max_pos = matches.iter().map(|&(_, p)| p).max().unwrap();
            for &(li, offset) in &matches {
                if offset < max_pos {
                    let pad = max_pos - offset;
                    self.lines[li].insert_str(offset, &" ".repeat(pad));
                }
                self.syntax.invalidate_line(li);
                self.mark_dirty(li);
            }
        } else {
            let min_pos = matches.iter().map(|&(_, p)| p).min().unwrap();
            for &(li, offset) in &matches {
                if offset > min_pos {
                    let excess = offset - min_pos;
                    // Remove `excess` characters before the first occurrence,
                    // but only if they are spaces
                    let line = &self.lines[li];
                    let before = &line[..offset];
                    let space_count = before.chars().rev().take_while(|c| *c == ' ').count();
                    let to_remove = space_count.min(excess);
                    if to_remove > 0 {
                        let cut = offset - to_remove;
                        let mut new_line = String::with_capacity(line.len() - to_remove);
                        new_line.push_str(&line[..cut]);
                        new_line.push_str(&line[offset..]);
                        self.lines[li] = new_line;
                    }
                }
                self.syntax.invalidate_line(li);
                self.mark_dirty(li);
            }
        }

        self.modified = true;
        self.search.mark_dirty();
        self.status_msg = Some(t!("editor.query_set_alignment", matches.len()));
    }

    /// `/z [#hm|#end|#start]`: add one cursor per selected line at the
    /// given column (#hm = first non-blank = default, #end = EOL,
    /// #start = col 0), then enter Insert mode so typing hits every
    /// cursor. Without a selection, adds a cursor at #hm of the current
    /// line. Cursor-only: no undo entry.
    fn cmd_add_cursors(&mut self, args: &str) {
        let target = args.trim().to_lowercase();
        enum Pos {
            Hm,
            End,
            Start,
        }
        let pos = match target.as_str() {
            "" | "#hm" => Pos::Hm,
            "#end" => Pos::End,
            "#start" => Pos::Start,
            _ => {
                self.status_msg = Some(t!("editor.query_z_usage"));
                return;
            }
        };
        let rows: Vec<usize> = if self.has_selection() {
            let (ay, ab) = self.cursor.selection_anchor.unwrap();
            let ((sy, _), (ey, _)) = sel_range(self.cursor.y, self.cursor.byte, ay, ab);
            (sy..=ey).collect()
        } else {
            vec![self.cursor.y]
        };
        let cols: Vec<usize> = rows
            .iter()
            .map(|&row| {
                let line = &self.lines[row];
                let c = match pos {
                    Pos::Start => 0,
                    Pos::End => line.len(),
                    Pos::Hm => line
                        .char_indices()
                        .find(|(_, c)| !c.is_whitespace())
                        .map(|(b, _)| b)
                        .unwrap_or(0),
                };
                c.min(line.len())
            })
            .collect();
        self.clear_extra_cursors();
        let first = rows[0];
        self.cursor.y = first;
        self.cursor.byte = cols[0];
        self.cursor.selection_anchor = Some((first, cols[0]));
        for (i, &row) in rows.iter().enumerate().skip(1) {
            let c = cols[i];
            self.cursor.extra_cursors.push(crate::editor::CursorPos {
                y: row,
                byte: c,
                anchor: Some((row, c)),
            });
        }
        self.mark_all_dirty();
        self.status_msg = Some(t!("editor.query_cursors_added", rows.len()));
        // Land in Insert with the fresh cursors so typing makes sense.
        self.mode = Mode::Insert;
        self.completion = None;
        self.sighelp = None;
    }

    fn cmd_upper_lower(&mut self, is_upper: bool) {
        if !self.has_selection() {
            self.status_msg = Some(t!(
                "editor.query_upper_lower_required",
                if is_upper { "upper" } else { "lower" }
            ));
            return;
        }
        let text = self.selected_text();
        let transformed = if is_upper {
            text.to_uppercase()
        } else {
            text.to_lowercase()
        };
        self.snapshot();
        let (ay, ab) = self.cursor.selection_anchor.unwrap();
        let ((sy, sb), (ey, eb)) = sel_range(self.cursor.y, self.cursor.byte, ay, ab);

        if sy == ey {
            let line = &self.lines[sy];
            let mut new_line = String::with_capacity(line.len());
            new_line.push_str(&line[..sb]);
            new_line.push_str(&transformed);
            new_line.push_str(&line[eb.min(line.len())..]);
            self.lines[sy] = new_line;
            self.syntax.invalidate_line(sy);
            self.mark_dirty(sy);
        } else {
            // First line
            let first = &self.lines[sy];
            let mut new_first = String::with_capacity(first.len());
            new_first.push_str(&first[..sb]);
            let lines_vec: Vec<&str> = transformed.split('\n').collect();
            new_first.push_str(lines_vec.first().unwrap_or(&""));
            self.lines[sy] = new_first;
            self.syntax.invalidate_line(sy);
            self.mark_dirty(sy);

            for (i, li) in (sy + 1..ey).enumerate() {
                if i + 1 < lines_vec.len() {
                    self.lines[li] = lines_vec[i + 1].to_string();
                    self.syntax.invalidate_line(li);
                    self.mark_dirty(li);
                }
            }

            // Last line
            let last = &self.lines[ey];
            let mut new_last = String::with_capacity(last.len());
            if let Some(last_part) = lines_vec.last() {
                new_last.push_str(last_part);
            }
            new_last.push_str(&last[eb.min(last.len())..]);
            self.lines[ey] = new_last;
            self.syntax.invalidate_line(ey);
            self.mark_dirty(ey);
        }

        self.modified = true;
        self.search.mark_dirty();
        self.status_msg = Some(if is_upper {
            t!("editor.query_uppercased")
        } else {
            t!("editor.query_lowercased")
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::editor::{Editor, Mode};
    use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

    fn test_editor(content: &[&str]) -> Editor {
        let dir = std::env::temp_dir().join("ntc_editor_test");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("test_query.txt");
        let lines: Vec<String> = content.iter().map(|s| s.to_string()).collect();
        std::fs::write(&path, lines.join("\n")).unwrap();
        let mut editor = Editor::new(&path).unwrap();
        editor.lines = lines;
        editor
    }

    fn press(code: KeyCode) -> Event {
        Event::Key(KeyEvent {
            code,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::empty(),
        })
    }

    #[test]
    fn typing_q_stays_in_query_mode() {
        // Regression: plain `q` used to exit Query mode, making
        // `qal …` and `/quit` impossible to type.
        let mut editor = test_editor(&["hello"]);
        editor.prev_mode = Mode::Normal;
        editor.mode = Mode::Query;
        for c in ['q', 'a', 'l'] {
            editor.handle_query_event(press(KeyCode::Char(c))).unwrap();
            assert_eq!(editor.mode, Mode::Query);
        }
        assert_eq!(editor.query_buf, "qal");
        // `/quit` is typeable too (slash + q + rest).
        editor.query_buf.clear();
        for c in ['/', 'q', 'u', 'i', 't'] {
            editor.handle_query_event(press(KeyCode::Char(c))).unwrap();
            assert_eq!(editor.mode, Mode::Query);
        }
        assert_eq!(editor.query_buf, "/quit");
        // Esc still exits and clears the buffer.
        editor.handle_query_event(press(KeyCode::Esc)).unwrap();
        assert_eq!(editor.mode, Mode::Normal);
        assert!(editor.query_buf.is_empty());
    }

    #[test]
    fn query_result_visible_until_next_keystroke() {
        // State contract for the hint bar: a successful query leaves a
        // `status_msg` behind (painted as `✦ …`); composing the next
        // command or leaving the mode retires it.
        let mut editor = test_editor(&["foo bar", "baz"]);
        editor.prev_mode = Mode::Normal;
        editor.mode = Mode::Query;
        assert!(editor.exec_one("/s \"foo\""));
        assert!(editor.status_msg.is_some());
        // Typing the next command clears the previous result.
        editor
            .handle_query_event(press(KeyCode::Char('/')))
            .unwrap();
        assert!(editor.status_msg.is_none());
        // ... as does Backspace.
        assert!(editor.exec_one("/s \"foo\""));
        assert!(editor.status_msg.is_some());
        editor
            .handle_query_event(press(KeyCode::Backspace))
            .unwrap();
        assert!(editor.status_msg.is_none());
        // ... and Esc clears it so Normal inherits nothing stale.
        assert!(editor.exec_one("/s \"foo\""));
        assert!(editor.status_msg.is_some());
        editor.handle_query_event(press(KeyCode::Esc)).unwrap();
        assert_eq!(editor.mode, Mode::Normal);
        assert!(editor.status_msg.is_none());
    }

    #[test]
    fn expand_alias_terminates_on_unresolved_placeholders() {
        let editor = test_editor(&["hello"]);
        // ${9} (out of range), ${0}, ${x} (non-numeric): previously infinite loop.
        let out = editor.expand_alias("${9} ${0} ${x} ${1} ${2:-d} ${3[]}", "a");
        assert_eq!(out, "${9} ${0} ${x} a d ");
    }

    #[test]
    fn expand_alias_resolves_ranges_and_defaults() {
        let editor = test_editor(&["hello"]);
        // Only ${1} resolves in pass 1; ${2:-fallback} falls back to its default,
        // and an out-of-range ${3[]} expands to empty.
        let out = editor.expand_alias("${1} ${2:-fallback} ${3[]}", "x");
        assert_eq!(out, "x fallback ");
    }

    fn select_line(editor: &mut Editor, y: usize) {
        editor.cursor.y = y;
        editor.cursor.byte = editor.lines[y].len();
        editor.cursor.selection_anchor = Some((y, 0));
    }

    #[test]
    fn rp_requires_selection() {
        let mut editor = test_editor(&["foo foo"]);
        editor.cmd_replace_sel("foo X");
        assert_eq!(editor.lines[0], "foo foo");
        assert!(editor.status_msg.is_some());
    }

    #[test]
    fn rp_replaces_within_selection_only() {
        let mut editor = test_editor(&["one", "two one", "one"]);
        select_line(&mut editor, 1);
        editor.cmd_replace_sel("one 1");
        assert_eq!(editor.lines[0], "one");
        assert_eq!(editor.lines[1], "two 1");
        assert_eq!(editor.lines[2], "one");
    }

    #[test]
    fn rp_quoted_multiword_patterns() {
        let mut editor = test_editor(&["foo bar foo", "keep"]);
        select_line(&mut editor, 0);
        editor.cmd_replace_sel("\"foo bar\" X");
        // split_args: ["foo bar", "X"]
        assert_eq!(editor.lines[0], "X foo");
        assert_eq!(editor.lines[1], "keep");
    }

    #[test]
    fn rp_case_insensitive_with_multibyte() {
        let mut editor = test_editor(&["aẞb ß x", "keep"]);
        // `ẞ` (3 bytes) lowercases to `ß` (2 bytes) — offsets must not drift.
        select_line(&mut editor, 0);
        editor.cmd_replace_sel("ß S");
        assert_eq!(editor.lines[0], "aSb S x");
        assert_eq!(editor.lines[1], "keep");
    }

    #[test]
    fn s_selects_next_and_wraps() {
        let mut editor = test_editor(&["foo bar", "baz foo", "qux"]);
        editor.cursor.y = 0;
        editor.cursor.byte = 0;
        // First "foo" is at (0, 0..3) — cursor sits at its start, so the
        // next occurrence after byte 0... ci_find from 0 finds (0,0..3).
        editor.cmd_select_next("foo");
        assert_eq!((editor.cursor.y, editor.cursor.byte), (0, 3));
        assert_eq!(editor.cursor.selection_anchor, Some((0, 0)));
        // Repeat: moves past the current selection to line 1.
        editor.cmd_select_next("foo");
        assert_eq!(editor.cursor.y, 1);
        assert_eq!(editor.cursor.selection_anchor, Some((1, 4)));
        // Repeat: wraps back to line 0.
        editor.cmd_select_next("foo");
        assert_eq!((editor.cursor.y, editor.cursor.byte), (0, 3));
    }

    #[test]
    fn s_bare_selects_word_under_cursor() {
        let mut editor = test_editor(&["hello world"]);
        editor.cursor.y = 0;
        editor.cursor.byte = 1;
        editor.cmd_select_next("");
        assert_eq!(editor.selected_text(), "hello");
    }

    #[test]
    fn s_no_match_keeps_state() {
        let mut editor = test_editor(&["hello"]);
        editor.cursor.y = 0;
        editor.cursor.byte = 0;
        editor.cmd_select_next("zzz");
        assert!(!editor.has_selection());
        assert!(editor.status_msg.is_some());
    }

    #[test]
    fn l_moves_cursor_to_line_start() {
        let mut editor = test_editor(&["a", "bb", "ccc"]);
        editor.cursor.y = 0;
        editor.cursor.byte = 1;
        editor.cursor.selection_anchor = Some((0, 0));
        editor.cmd_goto_line("3");
        assert_eq!((editor.cursor.y, editor.cursor.byte), (2, 0));
        assert!(!editor.has_selection());
        // Out of range clamps to the last line.
        editor.cmd_goto_line("99");
        assert_eq!(editor.cursor.y, 2);
        // Garbage shows usage.
        editor.cmd_goto_line("abc");
        assert!(editor.status_msg.is_some());
    }

    #[test]
    fn sa_selects_all_occurrences() {
        let mut editor = test_editor(&["foo bar", "baz foo foo"]);
        editor.cmd_select_all_occ("foo");
        assert_eq!((editor.cursor.y, editor.cursor.byte), (0, 3));
        assert_eq!(editor.cursor.selection_anchor, Some((0, 0)));
        assert_eq!(editor.cursor.extra_cursors.len(), 2);
        // Empty pattern shows usage, selects nothing.
        let mut editor = test_editor(&["foo"]);
        editor.cmd_select_all_occ("");
        assert!(!editor.has_selection());
    }

    #[test]
    fn sl_single_range_and_discrete() {
        // Single line.
        let mut editor = test_editor(&["a", "bb", "ccc", "dddd"]);
        editor.cmd_select_lines_new("2");
        assert_eq!(editor.cursor.selection_anchor, Some((1, 0)));
        assert_eq!((editor.cursor.y, editor.cursor.byte), (1, 2));
        // Range, either order.
        editor.cmd_select_lines_new("2 4");
        assert_eq!(editor.cursor.selection_anchor, Some((1, 0)));
        assert_eq!(editor.cursor.y, 3);
        editor.cmd_select_lines_new("4 2");
        assert_eq!(editor.cursor.selection_anchor, Some((1, 0)));
        assert_eq!(editor.cursor.y, 3);
        // Discrete lines, any count.
        editor.cmd_select_lines_new("1 , 3 , 4");
        assert_eq!(editor.cursor.selection_anchor, Some((0, 0)));
        assert_eq!(editor.cursor.extra_cursors.len(), 2);
        // Attached commas parse identically.
        let mut editor = test_editor(&["a", "bb", "ccc", "dddd"]);
        editor.cmd_select_lines_new("1,3");
        assert_eq!(editor.cursor.extra_cursors.len(), 1);
        // Garbage shows usage.
        editor.cmd_select_lines_new("x");
        assert!(editor.status_msg.is_some());
        editor.cmd_select_lines_new("");
        assert!(editor.status_msg.is_some());
    }

    #[test]
    fn c_clears_selection_and_cursors() {
        let mut editor = test_editor(&["foo foo", "foo"]);
        editor.cmd_select_all_occ("foo");
        assert!(editor.has_selection());
        assert!(editor.exec_one("/c"));
        assert!(!editor.has_selection());
        assert!(editor.cursor.extra_cursors.is_empty());
        assert!(editor.exec_one("cs"));
    }

    #[test]
    fn verbs_work_without_slash_and_old_verbs_retired() {
        let mut editor = test_editor(&["foo", "bar"]);
        // Bare verbs dispatch (no `/` needed).
        assert!(editor.exec_one("l 2"));
        assert_eq!(editor.cursor.y, 1);
        assert!(editor.exec_one("sal"));
        assert!(editor.has_selection());
        assert!(editor.exec_one("c"));
        assert!(!editor.has_selection());
        // Retired verbs are unknown now (no alias named `p` either).
        editor.exec_one("/p foo to X");
        assert_eq!(editor.lines, vec!["foo".to_string(), "bar".to_string()]);
        editor.exec_one("/q something");
        assert!(editor.status_msg.is_some());
    }

    #[test]
    fn qal_add_rejects_reserved_and_duplicates() {
        let mut editor = test_editor(&["x"]);
        // Reserved: the verb set + `qal` itself.
        for name in ["qal", "s", "sl", "rp", "c", "add", "rm", "S"] {
            editor.cmd_qal(&format!("add {} /sal", name));
            assert!(
                !editor.qal_aliases.contains_key(name),
                "reserved name accepted: {}",
                name
            );
        }
        // Duplicates need `/qal edit`.
        editor.cmd_qal("add mine /sal");
        assert!(editor.qal_aliases.contains_key("mine"));
        editor.cmd_qal("add mine /c");
        assert_eq!(editor.qal_aliases["mine"], "/sal");
        // Rename onto reserved/existing is blocked too.
        editor.cmd_qal("add other /c");
        editor.cmd_qal("rnm other to s");
        assert!(editor.qal_aliases.contains_key("other"));
        editor.cmd_qal("rnm other to mine");
        assert!(editor.qal_aliases.contains_key("other"));
        editor.cmd_qal("rnm other to fresh");
        assert!(editor.qal_aliases.contains_key("fresh"));
    }

    #[test]
    fn bare_alias_invokes_without_slash() {
        let mut editor = test_editor(&["a", "bb"]);
        editor.cmd_qal("add all /sal");
        assert!(editor.exec_one("all"));
        assert!(editor.has_selection());
        assert!(editor.exec_one("/c"));
        assert!(editor.exec_one("/all"));
        assert!(editor.has_selection());
    }

    #[test]
    fn split_args_respects_quotes() {
        assert_eq!(
            Editor::split_args("\"foo bar\" baz"),
            vec!["foo bar".to_string(), "baz".to_string()]
        );
        assert_eq!(
            Editor::split_args("a b c"),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
        assert!(Editor::split_args("").is_empty());
        assert_eq!(Editor::unquote("\"hi\""), "hi");
        assert_eq!(Editor::unquote("hi"), "hi");
    }

    #[test]
    fn rm_requires_selection() {
        let mut editor = test_editor(&["# ab", "# cd"]);
        editor.cmd_rm("#  from #hm");
        assert_eq!(editor.lines, vec!["# ab".to_string(), "# cd".to_string()]);
        assert!(editor.status_msg.is_some());
    }

    #[test]
    fn rm_strips_prefix_and_hm_alias() {
        let mut editor = test_editor(&["# def foo(a):", "#     do_something()", "plain"]);
        // Select rows 0..=1 only (multi-line range like /sl 1 2).
        editor.cmd_select_lines_new("1 2");
        editor.cmd_rm("#  from #hm");
        assert_eq!(editor.lines[0], "def foo(a):");
        assert_eq!(editor.lines[1], "    do_something()");
        assert_eq!(editor.lines[2], "plain");
        // #start behaves the same as #hm.
        let mut editor = test_editor(&["# x", "y"]);
        editor.cmd_select_lines_new("1 2");
        editor.cmd_rm("#  from #start");
        assert_eq!(editor.lines[0], "x");
        assert_eq!(editor.lines[1], "y");
    }

    #[test]
    fn rm_strips_suffix_once_and_skips_nonmatching() {
        let mut editor = test_editor(&["x;;", "y", "z;;extra"]);
        editor.cmd_select_lines_new("1 3");
        editor.cmd_rm(";; from #end");
        assert_eq!(editor.lines[0], "x");
        assert_eq!(editor.lines[1], "y");
        assert_eq!(editor.lines[2], "z;;extra");
        assert!(editor.status_msg.is_some());
    }

    #[test]
    fn rm_roundtrips_add_and_undo_restores() {
        let mut editor = test_editor(&["def foo(a):", "    do_something()"]);
        editor.cmd_select_lines_new("1 2");
        editor.cmd_add("#  to #hm");
        assert_eq!(editor.lines[0], "# def foo(a):");
        editor.cmd_rm("#  from #hm");
        assert_eq!(editor.lines[0], "def foo(a):");
        assert_eq!(editor.lines[1], "    do_something()");
        editor.undo();
        assert_eq!(editor.lines[0], "# def foo(a):");
    }

    #[test]
    fn rm_usage_errors() {
        let mut editor = test_editor(&["# ab"]);
        editor.cmd_select_lines_new("1");
        editor.cmd_rm("");
        assert_eq!(editor.lines[0], "# ab");
        editor.cmd_rm("#  to #hm");
        assert_eq!(editor.lines[0], "# ab");
        editor.cmd_rm(" from #hm");
        assert_eq!(editor.lines[0], "# ab");
        editor.cmd_rm("#  from #mid");
        assert_eq!(editor.lines[0], "# ab");
        // Case-sensitive: lowercase prefix does not match uppercase line.
        let mut editor = test_editor(&["# AB"]);
        editor.cmd_select_lines_new("1");
        editor.cmd_rm("# ab from #hm");
        assert_eq!(editor.lines[0], "# AB");
    }

    #[test]
    fn z_adds_cursor_per_selected_line() {
        let mut editor = test_editor(&["  ab", "    cd", "ef"]);
        editor.cmd_select_lines_new("1 3");
        editor.cmd_add_cursors("#hm");
        assert_eq!(editor.cursor.extra_cursors.len(), 2);
        assert_eq!(editor.mode, Mode::Insert);
        // #hm = first non-blank: row 0 col 2, row 1 col 4, row 2 col 0.
        assert_eq!((editor.cursor.y, editor.cursor.byte), (0, 2));
        let extras = &editor.cursor.extra_cursors;
        assert_eq!((extras[0].y, extras[0].byte), (1, 4));
        assert_eq!((extras[1].y, extras[1].byte), (2, 0));
    }

    #[test]
    fn z_positions_end_start_and_default() {
        let mut editor = test_editor(&["  ab", "cd"]);
        editor.cmd_select_lines_new("1 2");
        editor.cmd_add_cursors("#end");
        assert_eq!((editor.cursor.y, editor.cursor.byte), (0, 4));
        assert_eq!(editor.cursor.extra_cursors[0].byte, 2);
        editor.cmd_select_lines_new("1 2");
        editor.cmd_add_cursors("#start");
        assert_eq!(editor.cursor.byte, 0);
        assert_eq!(editor.cursor.extra_cursors[0].byte, 0);
        // Bare /z defaults to #hm.
        editor.cmd_select_lines_new("1");
        editor.cmd_add_cursors("");
        assert_eq!((editor.cursor.y, editor.cursor.byte), (0, 2));
        // Bad position shows usage and adds nothing.
        let mut editor = test_editor(&["ab"]);
        editor.cmd_select_lines_new("1");
        editor.cmd_add_cursors("#mid");
        assert!(editor.cursor.extra_cursors.is_empty());
        assert!(editor.status_msg.is_some());
    }

    #[test]
    fn z_standalone_uses_current_line() {
        let mut editor = test_editor(&["  ab", "cd"]);
        editor.cursor.y = 1;
        editor.cursor.byte = 0;
        assert!(editor.exec_one("/z"));
        assert_eq!((editor.cursor.y, editor.cursor.byte), (1, 0));
        assert!(editor.cursor.extra_cursors.is_empty());
    }

    #[test]
    fn up_low_alias_upper_lower() {
        let mut editor = test_editor(&["ab"]);
        select_line(&mut editor, 0);
        assert!(editor.exec_one("/up"));
        assert_eq!(editor.lines[0], "AB");
        assert!(editor.exec_one("/low"));
        assert_eq!(editor.lines[0], "ab");
        assert!(editor.exec_one("/upper"));
        assert_eq!(editor.lines[0], "AB");
        assert!(editor.exec_one("/lower"));
        assert_eq!(editor.lines[0], "ab");
        // Reserved: cannot become qal aliases.
        editor.cmd_qal("add up /sal");
        assert!(!editor.qal_aliases.contains_key("up"));
        editor.cmd_qal("add low /sal");
        assert!(!editor.qal_aliases.contains_key("low"));
        editor.cmd_qal("add z /sal");
        assert!(!editor.qal_aliases.contains_key("z"));
    }

    #[test]
    fn add_and_set_require_selection_now() {
        let mut editor = test_editor(&["ab", "cd"]);
        editor.cmd_add("X to #end");
        assert_eq!(editor.lines, vec!["ab".to_string(), "cd".to_string()]);
        editor.cmd_set("b to #end");
        assert_eq!(editor.lines, vec!["ab".to_string(), "cd".to_string()]);
        // With a selection both apply to the selected rows only.
        select_line(&mut editor, 0);
        editor.cmd_add("X to #end");
        assert_eq!(editor.lines[0], "abX");
        assert_eq!(editor.lines[1], "cd");
    }
}
