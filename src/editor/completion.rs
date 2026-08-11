use super::Editor;
use crate::config::Config;
use crate::syntax::SyntaxLanguage;
use crate::t;

#[derive(Clone, Debug)]
pub(crate) struct CompletionItem {
    pub(crate) label: String,
    pub(crate) detail: String,
    pub(crate) insert_text: String,
    pub(crate) category: u8,
}

fn score_candidate(prefix: &str, label: &str) -> u32 {
    if prefix.is_empty() || label.is_empty() {
        return 0;
    }
    let lower_p = prefix.to_lowercase();
    let lower_l = label.to_lowercase();
    if label == prefix {
        return 100;
    }
    if lower_l == lower_p {
        return 90;
    }
    if lower_l.starts_with(&lower_p) {
        return 80;
    }
    0
}

fn extract_variables(lines: &[String], lang: Option<SyntaxLanguage>) -> Vec<CompletionItem> {
    if lang != Some(SyntaxLanguage::Mql) {
        return Vec::new();
    }
    let infos = crate::mql::extract_variables(lines);
    infos
        .into_iter()
        .map(|v| CompletionItem {
            label: v.name.clone(),
            detail: format!("[Var] {}", v.type_name),
            insert_text: v.name,
            category: 0,
        })
        .collect()
}

fn extract_includes(lines: &[String], lang: Option<SyntaxLanguage>) -> Vec<CompletionItem> {
    if lang != Some(SyntaxLanguage::Mql) {
        return Vec::new();
    }
    let names = crate::mql::extract_includes(lines);
    names
        .into_iter()
        .map(|n| CompletionItem {
            label: n.clone(),
            detail: "[file.mqh]".to_string(),
            insert_text: n,
            category: 4,
        })
        .collect()
}

fn extract_classes(lines: &[String], lang: Option<SyntaxLanguage>) -> Vec<CompletionItem> {
    if lang != Some(SyntaxLanguage::Mql) {
        return Vec::new();
    }
    let classes = crate::mql::extract_classes(lines);
    classes
        .into_iter()
        .map(|(n, _)| CompletionItem {
            label: n.clone(),
            detail: "[class]".to_string(),
            insert_text: n,
            category: 2,
        })
        .collect()
}

fn extract_user_functions(lines: &[String], lang: Option<SyntaxLanguage>) -> Vec<CompletionItem> {
    if lang != Some(SyntaxLanguage::Mql) {
        return Vec::new();
    }
    let funcs = crate::mql::extract_user_functions(lines);
    funcs
        .into_iter()
        .map(|f| {
            let detail = if f.return_type.is_empty() {
                "[Fun]".to_string()
            } else {
                format!("[Fun] {}", f.return_type)
            };
            CompletionItem {
                label: f.name.clone(),
                detail,
                insert_text: f.name,
                category: 1,
            }
        })
        .collect()
}

fn extract_structs(lines: &[String], lang: Option<SyntaxLanguage>) -> Vec<CompletionItem> {
    if lang != Some(SyntaxLanguage::Mql) {
        return Vec::new();
    }
    let structs = crate::mql::extract_structs(lines);
    structs
        .into_iter()
        .map(|(n, _)| CompletionItem {
            label: n.clone(),
            detail: "[struct]".to_string(),
            insert_text: n,
            category: 2,
        })
        .collect()
}

pub(crate) fn completions_for(
    lang: Option<SyntaxLanguage>,
    prefix: &str,
    vars: &[CompletionItem],
    includes: &[CompletionItem],
    classes: &[CompletionItem],
    structs: &[CompletionItem],
    user_funcs: &[CompletionItem],
) -> Vec<CompletionItem> {
    let mut scored: Vec<(u32, CompletionItem)> = Vec::new();
    let lower = prefix.to_lowercase();

    fn push(
        scored: &mut Vec<(u32, CompletionItem)>,
        prefix: &str,
        label: &str,
        detail: &str,
        insert_text: &str,
        category: u8,
    ) {
        let s = score_candidate(prefix, label);
        if s > 0 {
            scored.push((
                s,
                CompletionItem {
                    label: label.to_string(),
                    detail: detail.to_string(),
                    insert_text: insert_text.to_string(),
                    category,
                },
            ));
        }
    }

    if let Some(lang) = lang {
        match lang {
            SyntaxLanguage::NtcMath => {
                for name in crate::math::builtin_function_names() {
                    if name == "print" {
                        continue;
                    }
                    let detail = match name {
                        "sin" => t!("editor.compl_math_sin"),
                        "cos" => t!("editor.compl_math_cos"),
                        "tan" => t!("editor.compl_math_tan"),
                        "cot" => t!("editor.compl_math_cot"),
                        "sec" => t!("editor.compl_math_sec"),
                        "csc" => t!("editor.compl_math_csc"),
                        "asin" | "arcsin" => t!("editor.compl_math_asin"),
                        "acos" | "arccos" => t!("editor.compl_math_acos"),
                        "atan" | "arctan" => t!("editor.compl_math_atan"),
                        "acot" | "arccot" => t!("editor.compl_math_acot"),
                        "asec" | "arcsec" => t!("editor.compl_math_asec"),
                        "acsc" | "arccsc" => t!("editor.compl_math_acsc"),
                        "sqrt" => t!("editor.compl_math_sqrt"),
                        "pow" => t!("editor.compl_math_pow"),
                        "abs" => t!("editor.compl_math_abs"),
                        "floor" => t!("editor.compl_math_floor"),
                        "ceil" | "ceiling" => t!("editor.compl_math_ceil"),
                        "round" => t!("editor.compl_math_round"),
                        "ln" | "log" => t!("editor.compl_math_ln"),
                        "log2" => t!("editor.compl_math_log2"),
                        "log10" => t!("editor.compl_math_log10"),
                        "sum" => t!("editor.compl_math_sum"),
                        "min" => t!("editor.compl_math_min"),
                        "max" => t!("editor.compl_math_max"),
                        "avg" | "average" | "mean" => t!("editor.compl_math_avg"),
                        "rand" | "random" => t!("editor.compl_math_rand"),
                        "tobinary" => t!("editor.compl_math_tobinary"),
                        "tohex" => t!("editor.compl_math_tohex"),
                        "to8" | "tooctal" => t!("editor.compl_math_tooctal"),
                        "todecimal" => t!("editor.compl_math_todecimal"),
                        "tohb" | "tohumanbytes" | "tohumanreadable" => t!("editor.compl_math_tohb"),
                        _ => "".to_string(),
                    };
                    push(&mut scored, prefix, name, &detail, name, 1);
                }
                for name in crate::math::constant_names() {
                    let val = match name {
                        "PI" | "pi" => std::f64::consts::PI,
                        "E" | "e" | "EXP" | "exp" => std::f64::consts::E,
                        "PHI" | "phi" => 1.618_033_988_749_895,
                        "TAU" | "tau" => std::f64::consts::TAU,
                        _ => 0.0,
                    };
                    push(&mut scored, prefix, name, &format!("= {}", val), name, 3);
                }
                for kw in &["return"] {
                    push(
                        &mut scored,
                        prefix,
                        kw,
                        &t!("editor.compl_return_value"),
                        kw,
                        5,
                    );
                }
                if "print".starts_with(&lower) {
                    push(
                        &mut scored,
                        prefix,
                        "print",
                        &t!("editor.compl_print_values"),
                        "print()",
                        1,
                    );
                }
                let cfg = Config::global();
                if let Ok(cfg_guard) = cfg.read() {
                    for (name, def) in &cfg_guard.math_functions {
                        let arrow = def.find("=>").unwrap_or(0);
                        let params = def[..arrow].trim();
                        push(
                            &mut scored,
                            prefix,
                            name,
                            &format!("fn({})", params),
                            name,
                            1,
                        );
                    }
                }
            }

            SyntaxLanguage::Mql => {
                for v in vars {
                    let s = score_candidate(prefix, &v.label);
                    if s > 0 {
                        scored.push((s, v.clone()));
                    }
                }
                for v in includes {
                    let s = score_candidate(prefix, &v.label);
                    if s > 0 {
                        scored.push((s, v.clone()));
                    }
                }
                for v in classes {
                    let s = score_candidate(prefix, &v.label);
                    if s > 0 {
                        scored.push((s, v.clone()));
                    }
                }
                for v in structs {
                    let s = score_candidate(prefix, &v.label);
                    if s > 0 {
                        scored.push((s, v.clone()));
                    }
                }
                for v in user_funcs {
                    let s = score_candidate(prefix, &v.label);
                    if s > 0 {
                        scored.push((s, v.clone()));
                    }
                }
                for name in crate::mql::builtin_function_names() {
                    push(&mut scored, prefix, name, "[Fun]", name, 1);
                }
                for name in crate::mql::type_names() {
                    push(
                        &mut scored,
                        prefix,
                        name,
                        &t!("editor.compl_builtin"),
                        name,
                        2,
                    );
                }
                for name in crate::mql::constant_names() {
                    push(&mut scored, prefix, name, "[constant]", name, 3);
                }
                for name in crate::mql::keyword_names() {
                    push(
                        &mut scored,
                        prefix,
                        name,
                        &t!("editor.compl_other"),
                        name,
                        5,
                    );
                }
            }

            _ => {}
        }
    }

    // Group by category, sort each group by score descending
    let mut grouped: std::collections::HashMap<u8, Vec<(u32, CompletionItem)>> =
        std::collections::HashMap::new();
    for item in scored {
        grouped.entry(item.1.category).or_default().push(item);
    }
    for items in grouped.values_mut() {
        items.sort_by(|(sa, _), (sb, _)| sb.cmp(sa));
    }

    let mut final_items: Vec<(u32, CompletionItem)> = Vec::new();
    // Phase 1: take top 2 from every category
    for cat in 0..=5u8 {
        if let Some(mut items) = grouped.remove(&cat) {
            let take = items.len().min(2);
            for item in items.drain(..take) {
                final_items.push(item);
            }
            if !items.is_empty() {
                grouped.entry(cat).or_default().extend(items);
            }
        }
    }

    // Phase 2: redistribute remaining budget
    let mut budget = 12usize.saturating_sub(final_items.len());
    if budget > 0 {
        // 2a: [file.mqh] (cat=4) gets priority for all extras
        if let Some(items) = grouped.remove(&4) {
            for item in items {
                if budget == 0 {
                    break;
                }
                final_items.push(item);
                budget -= 1;
            }
        }
        // 2b: each remaining category gets +1 (max total = 3)
        for cat in [0, 1, 2, 3, 5] {
            if budget == 0 {
                break;
            }
            if let Some(items) = grouped.remove(&cat) {
                if let Some(item) = items.into_iter().next() {
                    final_items.push(item);
                    budget -= 1;
                }
            }
        }
        // 2c: any leftover budget — drain whatever is left
        if budget > 0 {
            for cat in 0..=5u8 {
                if budget == 0 {
                    break;
                }
                if let Some(items) = grouped.remove(&cat) {
                    for item in items {
                        if budget == 0 {
                            break;
                        }
                        final_items.push(item);
                        budget -= 1;
                    }
                }
            }
        }
    }

    final_items.truncate(12);
    final_items.into_iter().map(|(_, item)| item).collect()
}

impl Editor {
    pub(crate) fn trigger_completion(&mut self) {
        let line = self.current();
        if let Some((word, _start_byte)) = word_before_cursor(line, self.cursor.byte) {
            let lang = self.syntax.language;
            let vars = extract_variables(&self.lines, lang);
            let includes = extract_includes(&self.lines, lang);
            let classes = extract_classes(&self.lines, lang);
            let structs = extract_structs(&self.lines, lang);
            let user_funcs = extract_user_functions(&self.lines, lang);
            let items = completions_for(
                lang,
                &word,
                &vars,
                &includes,
                &classes,
                &structs,
                &user_funcs,
            );
            if !items.is_empty() {
                self.completion_items = items;
                self.completion_idx = 0;
                self.completion_visible = true;
                self.completion_prefix = word.clone();
                self.mark_all_dirty();
            } else {
                self.dismiss_completion();
            }
        } else {
            self.dismiss_completion();
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
        if let Some((_, start_byte)) = word_before_cursor(line, self.cursor.byte) {
            let end_byte = self.cursor.byte;
            let new_line = format!(
                "{}{}{}",
                &line[..start_byte],
                item.insert_text,
                &line[end_byte..]
            );
            self.lines[self.cursor.y] = new_line;
            self.cursor.byte = start_byte + item.insert_text.len();
            self.modified = true;
            self.mark_dirty(self.cursor.y);
            self.syntax.invalidate_line(self.cursor.y);
        }
        self.dismiss_completion();
    }
}

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
