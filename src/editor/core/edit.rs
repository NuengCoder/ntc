use crate::editor::complete::enclosing_call;
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
    }

    /// Single-param hover line for the enclosing call at the cursor.
    /// Shares its call-site logic with `refresh_sighelp` (past-close and
    /// mid-word positions resolve to "no call" and clear). Repaints when
    /// the visible state changes so a dismissed line never lingers as
    /// ghost pixels — text underneath is only repainted on dirty marks.
    pub(crate) fn trigger_function_signature(&mut self) {
        let (next_active, next_text) = self.signature_hover_state();
        if next_active != self.hover_sig_active || next_text != self.hover_text {
            self.hover_sig_active = next_active;
            self.hover_text = next_text;
            self.mark_all_dirty();
        }
    }

    fn signature_hover_state(&self) -> (bool, Option<String>) {
        if !matches!(self.syntax.language, Some(SyntaxLanguage::Mql)) {
            return (false, None);
        }
        // Single source of truth with `refresh_sighelp`: the enclosing
        // call at the cursor (multi-line, string/comment-masked,
        // past-close aware). When there is none — cursor before the
        // paren, past the closing paren (`fun()|`), mid-word (`fu|n`),
        // or outside any call — clear instead of showing stale state.
        let call = match enclosing_call(&self.lines, self.cursor.y, self.cursor.byte) {
            Some(c) => c,
            None => return (false, None),
        };
        let name = call.name;
        let param_idx = call.active;

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
            let text = if param_idx < param_items.len() {
                Some(param_items[param_idx].clone())
            } else if !param_items.is_empty() {
                Some(param_items[0].clone())
            } else {
                Some(doc)
            };
            return (true, text);
        }

        // Look up builtin simple info
        if let Some(doc) = crate::mql::hover_info(&name) {
            return (true, Some(doc.to_string()));
        }

        // Look up user-defined function
        for func in crate::mql::extract_user_functions(&self.lines) {
            if func.name == name {
                let mut text = if func.params.is_empty() {
                    Some(format!("{} {}", func.return_type, func.name))
                } else {
                    let param_list: Vec<String> = func.params.split(',').map(fmt_one).collect();
                    if param_idx < param_list.len() {
                        Some(param_list[param_idx].clone())
                    } else if !param_list.is_empty() {
                        Some(param_list[0].clone())
                    } else {
                        None
                    }
                };
                if let Some(mut display) = text.take() {
                    if let Some(box_content) =
                        crate::mql::extract_comment_box(&self.lines, func.line)
                    {
                        display.push_str(&format!("\n\n{}", box_content));
                    }
                    text = Some(display);
                }
                let active = text.is_some();
                return (active, text);
            }
        }

        (false, None)
    }
}
