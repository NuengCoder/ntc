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
                    (true, KeyCode::Char('g')) => {
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
                        KeyCode::Esc | KeyCode::Char('q') => {
                            self.query_buf.clear();
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
                        }
                        KeyCode::Up if !self.qal_aliases.is_empty() => {
                            self.query_buf.clear();
                        }
                        KeyCode::Char(c) => {
                            self.query_buf.push(c);
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

    fn exec_one(&mut self, cmd: &str) -> bool {
        if let Some(rest) = cmd.strip_prefix('/') {
            let parts: Vec<&str> = rest.splitn(2, ' ').collect();
            let verb = parts[0];
            let args = parts.get(1).unwrap_or(&"").trim();
            match verb {
                "s" | "sl" | "s/l" => {
                    self.cmd_select_lines(args);
                    true
                }
                "p" => {
                    self.cmd_replace(args);
                    true
                }
                "qal" => {
                    self.cmd_qal(args);
                    true
                }
                "q" | "qal/run" => {
                    self.cmd_alias_invoke(args);
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
                "upper" => {
                    self.cmd_upper_lower(true);
                    true
                }
                "lower" => {
                    self.cmd_upper_lower(false);
                    true
                }
                "add" => {
                    self.cmd_add(args);
                    true
                }
                "set" => {
                    self.cmd_set(args);
                    true
                }
                _ => {
                    // Try alias lookup with /name [args]
                    self.try_alias_or_unknown(cmd);
                    true
                }
            }
        } else {
            self.try_alias_or_unknown(cmd);
            true
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

    fn cmd_select_lines(&mut self, args: &str) {
        let args = args.trim();
        if args.is_empty() {
            self.clear_selection();
            self.clear_extra_cursors();
            self.status_msg = Some(t!("editor.query_selection_cleared"));
            return;
        }

        let mut do_all = false;
        let mut range_start: Option<usize> = None;
        let mut range_end: Option<usize> = None;
        let mut filter_str: Option<String> = None;

        let tokens: Vec<&str> = args.split_whitespace().collect();
        let mut i = 0;
        while i < tokens.len() {
            match tokens[i] {
                "/a" => {
                    do_all = true;
                }
                "to" | "," => {}
                "/e" => {
                    i += 1;
                    if i < tokens.len() {
                        filter_str = Some(tokens[i].to_string());
                    }
                }
                n if n.parse::<usize>().is_ok() => {
                    let num: usize = n.parse().unwrap();
                    if range_start.is_none() {
                        range_start = Some(num);
                    } else if range_end.is_none() {
                        range_end = Some(num);
                    }
                }
                _ => {}
            }
            i += 1;
        }

        if let Some(ref pat) = filter_str {
            // Multi-cursor mode: select each occurrence of pattern
            let mut matches: Vec<(usize, usize, usize)> = Vec::new();

            let (search_s, search_e) = if let (Some(s), Some(e)) = (range_start, range_end) {
                (
                    s.max(1).saturating_sub(1),
                    e.max(1)
                        .saturating_sub(1)
                        .min(self.lines.len().saturating_sub(1)),
                )
            } else if do_all || range_start.is_none() {
                (0, self.lines.len().saturating_sub(1))
            } else {
                let n = range_start
                    .map(|n| {
                        n.max(1)
                            .saturating_sub(1)
                            .min(self.lines.len().saturating_sub(1))
                    })
                    .unwrap_or(0);
                (n, n)
            };

            for li in search_s..=search_e {
                let line = &self.lines[li];
                let mut start = 0;
                while let Some((s, e)) = ci_find(line, pat, start) {
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

            self.clear_selection();
            self.clear_extra_cursors();

            // Primary cursor at first match
            let (first_y, first_s, first_e) = matches[0];
            self.cursor.y = first_y;
            self.cursor.byte = first_e;
            self.cursor.selection_anchor = Some((first_y, first_s));

            // Extra cursors at rest
            for &(my, ms, me) in &matches[1..] {
                self.cursor.extra_cursors.push(crate::editor::CursorPos {
                    y: my,
                    byte: me,
                    anchor: Some((my, ms)),
                });
            }
            self.status_msg = Some(t!("editor.query_selected_occurrences", matches.len(), pat));
            return;
        }

        if do_all {
            self.select_all();
            self.status_msg = Some(t!("editor.query_selected_all_lines"));
        } else if let (Some(s), Some(e)) = (range_start, range_end) {
            let s = s.max(1).saturating_sub(1);
            let e = e.max(1).saturating_sub(1);
            let max_idx = self.lines.len().saturating_sub(1);
            if s > max_idx {
                self.status_msg = Some(t!("editor.query_start_out_of_range", s + 1, max_idx + 1));
                return;
            }
            let ey = e.min(max_idx);
            self.cursor.selection_anchor = Some((s, 0));
            self.cursor.y = ey;
            self.cursor.byte = self.lines[ey].len();
            self.status_msg = Some(t!("editor.query_selected_lines_range", s + 1, ey + 1));
        } else if let Some(n) = range_start {
            let n = n
                .max(1)
                .saturating_sub(1)
                .min(self.lines.len().saturating_sub(1));
            self.cursor.selection_anchor = Some((n, 0));
            self.cursor.byte = self.lines[n].len();
            self.status_msg = Some(t!("editor.query_selected_line", n + 1));
        }
    }

    fn cmd_replace(&mut self, args: &str) {
        let args = args.trim();
        if args.is_empty() {
            self.status_msg = Some(t!("editor.query_replace_usage"));
            return;
        }
        let sep = " to ";
        let pos = match args.rfind(sep) {
            Some(p) => p,
            None => {
                self.status_msg = Some(t!("editor.query_replace_format"));
                return;
            }
        };
        let find = &args[..pos];
        let replace = &args[pos + sep.len()..];
        if find.is_empty() {
            self.status_msg = Some(t!("editor.query_empty_pattern"));
            return;
        }

        let (row_start, row_end, col_start, col_end) = if self.has_selection() {
            let (ay, ab) = self.cursor.selection_anchor.unwrap();
            let ((sy, sb), (ey, eb)) = sel_range(self.cursor.y, self.cursor.byte, ay, ab);
            (sy, ey, sb, eb)
        } else {
            (0, self.lines.len().saturating_sub(1), 0, usize::MAX)
        };

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

        let (row_start, row_end) = if self.has_selection() {
            let (ay, ab) = self.cursor.selection_anchor.unwrap();
            let ((sy, _), (ey, _)) = sel_range(self.cursor.y, self.cursor.byte, ay, ab);
            (sy, ey)
        } else {
            if self.lines.is_empty() {
                return;
            }
            (0, self.lines.len().saturating_sub(1))
        };

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

        let (row_start, row_end) = if self.has_selection() {
            let (ay, ab) = self.cursor.selection_anchor.unwrap();
            let ((sy, _), (ey, _)) = sel_range(self.cursor.y, self.cursor.byte, ay, ab);
            (sy, ey)
        } else {
            if self.lines.is_empty() {
                return;
            }
            (0, self.lines.len().saturating_sub(1))
        };

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
    use crate::editor::Editor;

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

    #[test]
    fn cmd_replace_last_line_no_selection_does_not_panic() {
        let mut editor = test_editor(&["no match here", "foo bar foo"]);
        editor.cmd_replace("foo to X");
        assert_eq!(editor.lines[1], "X bar X");
        assert_eq!(editor.lines[0], "no match here");
    }

    #[test]
    fn cmd_replace_case_insensitive_with_multibyte() {
        let mut editor = test_editor(&["aẞb ß x", "keep"]);
        // `ẞ` (3 bytes) lowercases to `ß` (2 bytes) — offsets must not drift.
        editor.cmd_replace("ß to S");
        assert_eq!(editor.lines[0], "aSb S x");
        assert_eq!(editor.lines[1], "keep");
    }

    #[test]
    fn cmd_replace_respects_selection_bounds() {
        let mut editor = test_editor(&["one", "two one", "one"]);
        // Select "one" on line 1 only.
        editor.cursor.y = 1;
        editor.cursor.byte = 7;
        editor.cursor.selection_anchor = Some((1, 0));
        editor.cmd_replace("one to 1");
        assert_eq!(editor.lines[0], "one");
        assert_eq!(editor.lines[1], "two 1");
        assert_eq!(editor.lines[2], "one");
    }
}
