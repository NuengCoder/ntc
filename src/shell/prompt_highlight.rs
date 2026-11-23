//! Syntax highlighting for the interactive shell prompt line.
//!
//! Rustyline calls this on every edit. Colors come from the active theme and
//! are skipped entirely when `cfg color off` (or the terminal cannot render
//! ANSI), so prompting stays plain in monochrome mode.

use crate::config::Config;
use crate::utils::theme::{ansi_enabled, ShellRole, ThemeManager};
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PromptToken {
    NtcCommand,
    AliasName,
    TxtfFile,
    RalName,
    RalCommand,
    SubSubCommand,
    Flag1,
    Flag2,
    TpName,
    TpPath,
}

impl PromptToken {
    fn role(self) -> ShellRole {
        match self {
            Self::NtcCommand => ShellRole::NtcCommand,
            Self::AliasName => ShellRole::AliasName,
            Self::TxtfFile => ShellRole::TxtfFile,
            Self::RalName => ShellRole::RalName,
            Self::RalCommand => ShellRole::RalCommand,
            Self::SubSubCommand => ShellRole::SubSubCommand,
            Self::Flag1 => ShellRole::Flag1,
            Self::Flag2 => ShellRole::Flag2,
            Self::TpName => ShellRole::TpName,
            Self::TpPath => ShellRole::TpPath,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PromptSpan {
    pub range: Range<usize>,
    pub token: PromptToken,
}

struct Word<'a> {
    start: usize,
    end: usize,
    text: &'a str,
}

/// Split `line` into shell-ish words without breaking quoted strings.
fn words(line: &str) -> Vec<Word<'_>> {
    let mut out = Vec::new();
    let mut start: Option<usize> = None;
    let mut quote: Option<char> = None;
    let mut escaped = false;

    for (i, ch) in line.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if quote == Some('"') && ch == '\\' {
            escaped = true;
            start.get_or_insert(i);
            continue;
        }
        if let Some(q) = quote {
            if ch == q {
                quote = None;
            }
            start.get_or_insert(i);
            continue;
        }
        if ch == '\'' || ch == '"' {
            quote = Some(ch);
            start.get_or_insert(i);
        } else if ch.is_whitespace() {
            if let Some(s) = start.take() {
                out.push(Word {
                    start: s,
                    end: i,
                    text: &line[s..i],
                });
            }
        } else {
            start.get_or_insert(i);
        }
    }
    if let Some(s) = start {
        out.push(Word {
            start: s,
            end: line.len(),
            text: &line[s..],
        });
    }
    out
}

fn is_flag(text: &str) -> bool {
    text.starts_with('-') && text.len() > 1 && text != "--"
}

fn is_short_flag(text: &str) -> bool {
    is_flag(text) && !text.starts_with("--")
}

fn is_txtf_value_flag(text: &str) -> bool {
    matches!(
        text,
        "-l" | "--limit"
            | "--head"
            | "--tail"
            | "--betw"
            | "--sort"
            | "-p"
            | "--path"
            | "-ext"
            | "--ext"
            | "-d"
            | "--depth"
    )
}

/// Commands whose second word is a subcommand rather than a path/value.
fn has_subcommands(cmd: &str) -> bool {
    // `ntc` is the shell's own name for nested lines, not a registered command.
    cmd == "ntc"
        || crate::shell::commands::registry_data::registry()
            .resolve(cmd)
            .is_some_and(|spec| spec.subcommand_highlight)
}

fn is_ral_subcommand(text: &str) -> bool {
    matches!(
        text,
        "add" | "edit" | "rm" | "remove" | "rnm" | "rename" | "list" | "ls" | "cls" | "hintn"
    )
}

fn is_tp_subcommand(text: &str) -> bool {
    matches!(
        text,
        "add" | "rm" | "remove" | "list" | "ls" | "jump" | "go" | "show" | "info"
    )
}

fn is_run_alias(text: &str, aliases: &HashMap<String, String>) -> bool {
    let (base, _) = crate::shell::alias::parse_call_syntax(text);
    aliases.contains_key(&base.to_ascii_lowercase())
}

/// Token spans for one prompt line.
pub(crate) fn prompt_spans(line: &str) -> Vec<PromptSpan> {
    if line.is_empty() {
        return Vec::new();
    }
    let aliases = Config::get_run_aliases_merged();
    prompt_spans_with_aliases(line, &aliases)
}

fn prompt_spans_with_aliases(line: &str, aliases: &HashMap<String, String>) -> Vec<PromptSpan> {
    let words = words(line);
    let Some(root) = words.first() else {
        return Vec::new();
    };
    let cmd = root.text.to_ascii_lowercase();
    let mut spans = Vec::new();
    let mut subcommand = String::new();
    let mut ral_subcommand = String::new();
    let mut txtf_file_seen = false;
    let mut txtf_skip_value = false;

    if root.text.starts_with('@') && root.text.len() > 1 {
        if let Some(relative) = root.text[1..].find('/') {
            let split = root.start + 1 + relative;
            spans.push(PromptSpan {
                range: root.start..split,
                token: PromptToken::TpName,
            });
            spans.push(PromptSpan {
                range: split..root.end,
                token: PromptToken::TpPath,
            });
        } else {
            spans.push(PromptSpan {
                range: root.start..root.end,
                token: PromptToken::TpName,
            });
        }
    } else {
        spans.push(PromptSpan {
            range: root.start..root.end,
            token: if is_run_alias(root.text, aliases) {
                PromptToken::AliasName
            } else {
                PromptToken::NtcCommand
            },
        });
    }

    for (idx, word) in words.iter().enumerate().skip(1) {
        let text = word.text;
        if text.is_empty() {
            continue;
        }

        if text.starts_with('@') && text.len() > 1 {
            if let Some(relative) = text[1..].find('/') {
                let split = word.start + 1 + relative;
                spans.push(PromptSpan {
                    range: word.start..split,
                    token: PromptToken::TpName,
                });
                spans.push(PromptSpan {
                    range: split..word.end,
                    token: PromptToken::TpPath,
                });
            } else {
                spans.push(PromptSpan {
                    range: word.start..word.end,
                    token: PromptToken::TpName,
                });
            }
            continue;
        }

        if is_flag(text) {
            spans.push(PromptSpan {
                range: word.start..word.end,
                token: if is_short_flag(text) {
                    PromptToken::Flag1
                } else {
                    PromptToken::Flag2
                },
            });
            txtf_skip_value = is_txtf_value_flag(text);
            continue;
        }

        if txtf_skip_value {
            txtf_skip_value = false;
            continue;
        }

        if (cmd == "txtf"
            || cmd == "txtc"
            || cmd == "view"
            || cmd == "viewf"
            || cmd == "viewd"
            || cmd == "views")
            && !txtf_file_seen
        {
            spans.push(PromptSpan {
                range: word.start..word.end,
                token: PromptToken::TxtfFile,
            });
            txtf_file_seen = true;
            continue;
        }

        if idx == 1 && is_run_alias(text, aliases) {
            spans.push(PromptSpan {
                range: word.start..word.end,
                token: PromptToken::AliasName,
            });
            continue;
        }

        if cmd == "ral" {
            if idx == 1 && !is_ral_subcommand(text) {
                spans.push(PromptSpan {
                    range: word.start..word.end,
                    token: PromptToken::RalName,
                });
                continue;
            }
            if idx == 1 {
                subcommand = text.to_ascii_lowercase();
                ral_subcommand = subcommand.clone();
            } else if matches!(
                ral_subcommand.as_str(),
                "add" | "edit" | "rm" | "remove" | "rnm" | "rename" | "hintn"
            ) {
                let token = if idx == 2 {
                    PromptToken::RalName
                } else {
                    PromptToken::RalCommand
                };
                spans.push(PromptSpan {
                    range: word.start..word.end,
                    token,
                });
                continue;
            }
        }

        if cmd == "run" || cmd == "r" {
            if idx == 1 {
                spans.push(PromptSpan {
                    range: word.start..word.end,
                    token: PromptToken::RalName,
                });
                continue;
            }
        }

        if cmd == "tp" || cmd == "tpb" {
            if idx == 1 {
                if cmd == "tpb" || !is_tp_subcommand(text) {
                    spans.push(PromptSpan {
                        range: word.start..word.end,
                        token: PromptToken::TpName,
                    });
                    continue;
                }
                subcommand = text.to_ascii_lowercase();
            } else if idx == 2 {
                spans.push(PromptSpan {
                    range: word.start..word.end,
                    token: PromptToken::TpName,
                });
                continue;
            } else if idx == 3 && subcommand == "add" {
                spans.push(PromptSpan {
                    range: word.start..word.end,
                    token: PromptToken::TpPath,
                });
                continue;
            }
        }

        if idx == 1 && has_subcommands(&cmd) {
            subcommand = text.to_ascii_lowercase();
            spans.push(PromptSpan {
                range: word.start..word.end,
                token: PromptToken::NtcCommand,
            });
            continue;
        }

        if idx == 2 {
            spans.push(PromptSpan {
                range: word.start..word.end,
                token: PromptToken::SubSubCommand,
            });
        }
    }

    spans
}

/// Prompt spans with the shell-theme field name that colors each one. Used
/// by `serve theme` so its editable prompt line highlights exactly like the
/// terminal one.
pub(crate) fn themed_field_spans(line: &str) -> Vec<(Range<usize>, &'static str)> {
    prompt_spans(line)
        .into_iter()
        .map(|span| {
            let field = match span.token {
                PromptToken::NtcCommand => "ntc_command",
                PromptToken::AliasName => "alias_name",
                PromptToken::TxtfFile => "txtf_file",
                PromptToken::RalName => "ral_name",
                PromptToken::RalCommand => "ral_command",
                PromptToken::SubSubCommand => "sub_sub_command",
                PromptToken::Flag1 => "flag1",
                PromptToken::Flag2 => "flag2",
                PromptToken::TpName => "tp_name",
                PromptToken::TpPath => "tp_path",
            };
            (span.range, field)
        })
        .collect()
}

/// Highlight one editable prompt line. Returns the original string untouched
/// when color is off or nothing is worth coloring.
pub(crate) fn highlight_line(line: &str) -> Cow<'_, str> {
    if line.is_empty() || !ansi_enabled() {
        return Cow::Borrowed(line);
    }
    let spans = prompt_spans(line);
    if spans.is_empty() {
        return Cow::Borrowed(line);
    }
    let theme = ThemeManager::current();
    let colors: Vec<_> = spans
        .iter()
        .map(|span| theme.shell.role(span.token.role()))
        .collect();
    drop(theme);

    let mut out = String::with_capacity(line.len() + spans.len() * 12);
    let mut cursor = 0;
    for (span, color) in spans.iter().zip(colors) {
        out.push_str(&line[cursor..span.range.start]);
        out.push_str(&color.paint_ansi(&line[span.range.clone()]));
        cursor = span.range.end;
    }
    out.push_str(&line[cursor..]);
    Cow::Owned(out)
}

/// Whether `highlight_line` would color any word in `line`.
pub(crate) fn has_highlights(line: &str) -> bool {
    ansi_enabled() && !prompt_spans(line).is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds_with_aliases(line: &str, aliases: &HashMap<String, String>) -> Vec<&'static str> {
        prompt_spans_with_aliases(line, aliases)
            .into_iter()
            .map(|span| match span.token {
                PromptToken::NtcCommand => "command",
                PromptToken::AliasName => "alias",
                PromptToken::TxtfFile => "txtf_file",
                PromptToken::RalName => "ral_name",
                PromptToken::RalCommand => "ral_command",
                PromptToken::SubSubCommand => "sub_sub",
                PromptToken::Flag1 => "flag1",
                PromptToken::Flag2 => "flag2",
                PromptToken::TpName => "tp_name",
                PromptToken::TpPath => "tp_path",
            })
            .collect()
    }

    fn kinds(line: &str) -> Vec<&'static str> {
        kinds_with_aliases(line, &HashMap::new())
    }

    #[test]
    fn colors_txtf_file_and_flags() {
        assert_eq!(
            kinds("txtf --head 20 src/main.rs"),
            vec!["command", "flag2", "txtf_file"]
        );
        assert_eq!(
            kinds("txtf src/main.rs -l"),
            vec!["command", "txtf_file", "flag1"]
        );
    }

    #[test]
    fn colors_ral_name_and_command() {
        assert_eq!(
            kinds("ral add build \"cargo build --release\""),
            vec!["command", "command", "ral_name", "ral_command"]
        );
        assert_eq!(kinds("run build"), vec!["command", "ral_name"]);
    }

    #[test]
    fn colors_teleports_and_subcommands() {
        assert_eq!(
            kinds("tp add web ./site"),
            vec!["command", "command", "tp_name", "tp_path"]
        );
        assert_eq!(kinds("@web/src"), vec!["tp_name", "tp_path"]);
        assert_eq!(kinds("cfg color on"), vec!["command", "command", "sub_sub"]);
        assert_eq!(
            kinds("theme edit default"),
            vec!["command", "command", "sub_sub"]
        );
    }

    #[test]
    fn aliases_color_the_root_and_first_argument() {
        let aliases = HashMap::from([("build".to_string(), "cargo build".to_string())]);
        assert_eq!(
            kinds_with_aliases("build --release", &aliases),
            vec!["alias", "flag2"]
        );
        assert_eq!(
            kinds_with_aliases("fake build -n", &aliases),
            vec!["command", "alias", "flag1"]
        );
        assert_eq!(
            kinds_with_aliases("build(release)", &aliases),
            vec!["alias"]
        );
    }
}
