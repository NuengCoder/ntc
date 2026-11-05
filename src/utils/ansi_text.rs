// src/utils/ansi_text.rs
// Terminal-safe text helpers shared by the shell TUI, the editor and reports.
//
// Everything here is intentionally allocation-light and never slices a string
// in the middle of a UTF-8 character or an ANSI escape sequence.

use crate::editor::char_col_width;

/// Byte length of the ANSI escape sequence starting at `bytes[i]` (which must
/// be `0x1B`), or `1` when the sequence is incomplete / unknown.
fn ansi_len(bytes: &[u8], i: usize) -> usize {
    debug_assert_eq!(bytes[i], 0x1b);
    let Some(&kind) = bytes.get(i + 1) else {
        return 1;
    };
    match kind {
        b'[' => {
            // CSI: parameter/intermediate bytes, then one final byte.
            let mut j = i + 2;
            while j < bytes.len() && !(0x40..=0x7e).contains(&bytes[j]) {
                j += 1;
            }
            if j < bytes.len() {
                j += 1;
            }
            j - i
        }
        b']' | b'P' | b'X' | b'^' | b'_' => {
            // OSC / DCS / SOS / PM / APC: BEL or ST (ESC \) terminated.
            let mut j = i + 2;
            while j < bytes.len() {
                if bytes[j] == 0x07 {
                    return j + 1 - i;
                }
                if bytes[j] == 0x1b && bytes.get(j + 1) == Some(&b'\\') {
                    return j + 2 - i;
                }
                j += 1;
            }
            bytes.len() - i
        }
        // Two-byte escape (e.g. ESC ( B) — consume the introducer too.
        _ => 2.min(bytes.len() - i),
    }
}

/// Visible terminal width of `s`, ignoring ANSI escapes.
pub(crate) fn visible_width(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut width = 0usize;
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == 0x1b {
            i += ansi_len(bytes, i);
            continue;
        }
        // ASCII fast path keeps the common case cheap.
        if bytes[i] < 0x80 {
            if bytes[i] >= 0x20 && bytes[i] != 0x7f {
                width += 1;
            }
            i += 1;
            continue;
        }
        let Some(ch) = s[i..].chars().next() else {
            break;
        };
        width += char_col_width(ch);
        i += ch.len_utf8();
    }
    width
}

/// Truncate `s` to at most `max` visible columns.
///
/// ANSI escapes that appear before the cut are preserved and never split;
/// a wide character that would cross the boundary is dropped entirely.
pub(crate) fn truncate_visible(s: &str, max: usize) -> &str {
    let bytes = s.as_bytes();
    let mut width = 0usize;
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == 0x1b {
            let len = ansi_len(bytes, i);
            if i + len >= bytes.len() {
                break;
            }
            i += len;
            continue;
        }
        let Some(ch) = s[i..].chars().next() else {
            break;
        };
        let w = if (ch as u32) < 0x80 {
            if (ch as u32) < 0x20 || ch == '\x7f' {
                0
            } else {
                1
            }
        } else {
            char_col_width(ch)
        };
        if width + w > max {
            break;
        }
        width += w;
        i += ch.len_utf8();
    }
    &s[..i]
}

/// Escape C0/C1 control characters so file or child-process output can be
/// printed literally without the terminal interpreting it.
///
/// `\n` becomes a space (callers print one line at a time), `\t` is kept so
/// [`expand_tabs`] can align it, and `ESC` becomes `^[`.
pub(crate) fn sanitize_control(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '\n' | '\r' => out.push(' '),
            '\t' => out.push('\t'),
            '\x7f' => out.push_str("^?"),
            c if (c as u32) < 0x20 => {
                out.push('^');
                out.push((b'@' + c as u8) as char);
            }
            c => out.push(c),
        }
    }
    out
}

/// Expand tabs to spaces using the visible column (ANSI escapes do not count).
pub(crate) fn expand_tabs(s: &str, tabstop: usize) -> String {
    let tabstop = tabstop.max(1);
    let mut out = String::with_capacity(s.len());
    let mut col = 0usize;
    let bytes = s.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == 0x1b {
            let len = ansi_len(bytes, i);
            out.push_str(&s[i..i + len]);
            i += len;
            continue;
        }
        let Some(ch) = s[i..].chars().next() else {
            break;
        };
        if ch == '\t' {
            let next = (col / tabstop + 1) * tabstop;
            out.push_str(&" ".repeat(next - col));
            col = next;
        } else {
            out.push(ch);
            col += char_col_width(ch);
        }
        i += ch.len_utf8();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn width_ignores_ansi() {
        assert_eq!(visible_width("\x1b[31mred\x1b[0m"), 3);
        assert_eq!(visible_width("plain"), 5);
        assert_eq!(visible_width(""), 0);
    }

    #[test]
    fn width_counts_wide_chars() {
        assert_eq!(visible_width("日本"), 4);
        assert_eq!(visible_width("\x1b[1m日本\x1b[0m"), 4);
    }

    #[test]
    fn truncate_keeps_ansi_and_never_splits() {
        let s = "\x1b[31mabcdef\x1b[0m";
        assert_eq!(truncate_visible(s, 3), "\x1b[31mabc");
        assert_eq!(truncate_visible("日本語", 3), "日");
        assert_eq!(truncate_visible("abc", 10), "abc");
    }

    #[test]
    fn sanitize_escapes_control_bytes() {
        assert_eq!(sanitize_control("a\x1b[31mb"), "a^[[31mb");
        assert_eq!(sanitize_control("a\tb"), "a\tb");
        assert_eq!(sanitize_control("a\nb"), "a b");
        assert_eq!(sanitize_control("a\x7fb"), "a^?b");
    }

    #[test]
    fn expand_tabs_uses_visible_columns() {
        assert_eq!(expand_tabs("\tx", 4), "    x");
        assert_eq!(expand_tabs("ab\tx", 4), "ab  x");
        assert_eq!(
            expand_tabs("\x1b[31mab\x1b[0m\tx", 4),
            "\x1b[31mab\x1b[0m  x"
        );
    }
}
