use crate::editor::{auto_pair, Editor};
use crate::syntax::SyntaxLanguage;

impl Editor {
    pub(crate) fn handle_char_input(&mut self, c: char) {
        if self.has_multiple_cursors() {
            self.multi_insert_at(c);
        } else {
            self.snapshot();
            if let Some(closing) = auto_pair(c) {
                self.insert_at(c);
                self.insert_at_raw(closing);
                self.cursor.byte = self.cursor.byte.saturating_sub(closing.len_utf8());
            } else {
                self.insert_at(c);
            }
        }
        if matches!(self.syntax.language, Some(SyntaxLanguage::Mql)) {
            self.trigger_function_signature();
        }
        if matches!(
            self.syntax.language,
            Some(SyntaxLanguage::NtcMath | SyntaxLanguage::Mql)
        ) {
            self.trigger_completion();
        }
    }

    pub(crate) fn trigger_function_signature(&mut self) {
        if !matches!(self.syntax.language, Some(SyntaxLanguage::Mql)) {
            self.hover_sig_active = false;
            self.hover_text = None;
            return;
        }
        let line = self.current();
        let byte = self.cursor.byte.min(line.len());
        let prefix = &line[..byte];
        let paren = prefix.rfind('(');

        let (paren_pos, name) = match paren {
            Some(p) if p > 0 => {
                let before_paren = &prefix[..p];
                let start = before_paren
                    .chars()
                    .rev()
                    .take_while(|c| c.is_alphanumeric() || *c == '_')
                    .map(|c| c.len_utf8())
                    .sum::<usize>();
                if start == 0 {
                    self.hover_sig_active = false;
                    self.hover_text = None;
                    return;
                }
                let name = &before_paren[before_paren.len() - start..];
                if name.is_empty() {
                    self.hover_sig_active = false;
                    self.hover_text = None;
                    return;
                }
                (p, name.to_string())
            }
            _ => {
                self.hover_sig_active = false;
                self.hover_text = None;
                return;
            }
        };

        // Compute current param index: count non-nested commas between ( and cursor
        let inside = &prefix[paren_pos + 1..];
        let mut depth = 0u32;
        #[allow(clippy::match_single_binding)]
        let param_idx = inside
            .chars()
            .filter(|&c| match c {
                '(' => {
                    depth += 1;
                    false
                }
                ')' => {
                    if depth > 0 {
                        depth -= 1;
                    }
                    false
                }
                ',' if depth == 0 => true,
                _ => false,
            })
            .count();

        let fmt_one = |p: &str| -> String {
            let p = p.trim();
            let t: Vec<&str> = p.split_whitespace().collect();
            if t.len() >= 2 {
                format!("{}: {}", t[t.len() - 1], t[..t.len() - 1].join(" "))
            } else {
                p.to_string()
            }
        };

        // Look up builtin with detailed info
        if let Some(doc) = crate::mql::hover_info_detailed(&name) {
            let doc_lines: Vec<&str> = doc.lines().collect();
            let param_items: Vec<String> = doc_lines
                .iter()
                .skip_while(|l| l.contains('—'))
                .skip_while(|l| l.trim().is_empty())
                .take_while(|l| l.contains(':') && !l.starts_with("Returns"))
                .map(|l| l.trim().to_string())
                .collect();
            self.hover_text = if param_idx < param_items.len() {
                Some(param_items[param_idx].clone())
            } else if !param_items.is_empty() {
                Some(param_items[0].clone())
            } else {
                Some(doc)
            };
            self.hover_sig_active = true;
            return;
        }

        // Look up builtin simple info
        if let Some(doc) = crate::mql::hover_info(&name) {
            self.hover_text = Some(doc.to_string());
            self.hover_sig_active = true;
            return;
        }

        // Look up user-defined function
        for func in crate::mql::extract_user_functions(&self.lines) {
            if func.name == name {
                if func.params.is_empty() {
                    self.hover_text = Some(format!("{} {}", func.return_type, func.name));
                } else {
                    let param_list: Vec<String> = func.params.split(',').map(fmt_one).collect();
                    self.hover_text = if param_idx < param_list.len() {
                        Some(param_list[param_idx].clone())
                    } else if !param_list.is_empty() {
                        Some(param_list[0].clone())
                    } else {
                        None
                    };
                }
                if let Some(mut display) = self.hover_text.take() {
                    if let Some(box_content) =
                        crate::mql::extract_comment_box(&self.lines, func.line)
                    {
                        display.push_str(&format!("\n\n{}", box_content));
                    }
                    self.hover_text = Some(display);
                }
                self.hover_sig_active = self.hover_text.is_some();
                return;
            }
        }

        self.hover_sig_active = false;
        self.hover_text = None;
    }
}
