// ── unicode helpers ──────────────────────────────────────────────────────────

/// Visual width of a character in a terminal (1 or 2 for CJK / fullwidth).
///
/// Only the East Asian **Wide/Fullwidth** blocks are two cells. Box drawing
/// (`╔═╗`), arrows, geometric shapes and other East Asian *Ambiguous*
/// characters stay single-width, because that is how terminals draw them —
/// counting them as double left an untouched cell between every glyph, which
/// then kept whatever the previous frame had painted there.
pub(crate) fn char_col_width(c: char) -> usize {
    let cp = c as u32;
    let wide = matches!(
        cp,
        0x1100..=0x115F // Hangul Jamo (leading)
            | 0x2E80..=0x303E // CJK radicals, Kangxi, CJK symbols/punctuation
            | 0x3041..=0x33FF // Hiragana … CJK compatibility
            | 0x3400..=0x4DBF // CJK ext A
            | 0x4E00..=0x9FFF // CJK unified ideographs
            | 0xA000..=0xA4CF // Yi
            | 0xA960..=0xA97F // Hangul Jamo extended A
            | 0xAC00..=0xD7A3 // Hangul syllables
            | 0xF900..=0xFAFF // CJK compatibility ideographs
            | 0xFE10..=0xFE19 // vertical forms
            | 0xFE30..=0xFE6F // CJK compatibility forms
            | 0xFF00..=0xFF60 // fullwidth forms
            | 0xFFE0..=0xFFE6 // fullwidth signs
            | 0x1B000..=0x1B2FF // Kana supplement / extended
            | 0x1F200..=0x1F2FF // enclosed ideographic supplement
            | 0x1F300..=0x1F64F // emoji
            | 0x1F900..=0x1F9FF // emoji (supplemental symbols)
            | 0x20000..=0x2FFFD // CJK ext B–F
            | 0x30000..=0x3FFFD // CJK ext G
    );
    if wide {
        2
    } else {
        1
    }
}

/// Number of visual columns occupied by the first `byte_end` bytes of `s`.
pub(crate) fn byte_to_col(s: &str, byte_end: usize) -> usize {
    s[..byte_end.min(s.len())].chars().map(char_col_width).sum()
}

/// Two-cell visible stand-in for a control character (`ESC` -> `^[`,
/// `DEL` -> `^?`). Printing the raw byte would reach the terminal.
pub(crate) fn ctrl_placeholder(c: char) -> [char; 2] {
    let second = if c == '\x7f' {
        '?'
    } else {
        (b'@' + (c as u8)) as char
    };
    ['^', second]
}

/// Terminal-safe display cells for one buffer line.
///
/// Returns `(glyph, byte offset)` pairs. The byte offset always points at the
/// source character the glyph came from, so syntax colours and selection
/// ranges keep working. Tabs expand to the next `tabstop` boundary and control
/// characters become their two-cell placeholder, which guarantees the renderer
/// never hands a raw `ESC` (or any other control byte) to `Print`.
pub(crate) fn display_cells(line: &str, tabstop: usize) -> Vec<(char, usize)> {
    let tabstop = tabstop.max(1);
    let mut out: Vec<(char, usize)> = Vec::with_capacity(line.len() + 8);
    let mut col = 0usize;
    for (b, ch) in line.char_indices() {
        match ch {
            '\t' => {
                let next = (col / tabstop + 1) * tabstop;
                for _ in col..next {
                    out.push((' ', b));
                }
                col = next;
            }
            c if is_ctrl_char(c) => {
                let [a, d] = ctrl_placeholder(c);
                out.push((a, b));
                out.push((d, b));
                col += 2;
            }
            c => {
                out.push((c, b));
                col += char_col_width(c);
            }
        }
    }
    out
}

/// Visual column of `byte_end` in `line`, using the same tab/control
/// expansion as [`display_cells`].
pub(crate) fn display_col(line: &str, byte_end: usize, tabstop: usize) -> usize {
    let tabstop = tabstop.max(1);
    let limit = byte_end.min(line.len());
    let mut col = 0usize;
    for (b, ch) in line.char_indices() {
        if b >= limit {
            break;
        }
        col += match ch {
            '\t' => (col / tabstop + 1) * tabstop - col,
            c if is_ctrl_char(c) => 2,
            c => char_col_width(c),
        };
    }
    col
}

/// Inverse of [`display_col`]: byte offset shown at visual column `col`.
pub(crate) fn display_col_to_byte(line: &str, col: usize, tabstop: usize) -> usize {
    let tabstop = tabstop.max(1);
    let mut cur = 0usize;
    for (b, ch) in line.char_indices() {
        let w = match ch {
            '\t' => (cur / tabstop + 1) * tabstop - cur,
            c if is_ctrl_char(c) => 2,
            c => char_col_width(c),
        };
        if cur + w > col {
            return b;
        }
        cur += w;
    }
    line.len()
}

/// Byte offset of the start of the character just left of `byte`.
pub(crate) fn prev_char_byte(s: &str, byte: usize) -> usize {
    let mut p = byte.min(s.len());
    while p > 0 && !s.is_char_boundary(p) {
        p -= 1;
    }
    if p == 0 {
        return 0;
    }
    let mut start = p - 1;
    while start > 0 && !s.is_char_boundary(start) {
        start -= 1;
    }
    start
}

/// Byte offset of the start of the next character after `byte`.
pub(crate) fn next_char_byte(s: &str, byte: usize) -> usize {
    let len = s.len();
    if byte >= len {
        return len;
    }
    let next = byte + 1;
    (next..=len).find(|&b| s.is_char_boundary(b)).unwrap_or(len)
}

/// Move byte offset one word forward (Ctrl+Right).
pub(crate) fn next_word_byte(s: &str, byte: usize) -> usize {
    let chars: Vec<(usize, char)> = s.char_indices().collect();
    let mut i = chars.partition_point(|(b, _)| *b < byte);
    // Skip current word chars
    while i < chars.len() && !chars[i].1.is_alphanumeric() && chars[i].1 != '_' {
        i += 1;
    }
    while i < chars.len() && (chars[i].1.is_alphanumeric() || chars[i].1 == '_') {
        i += 1;
    }
    chars.get(i).map(|(b, _)| *b).unwrap_or(s.len())
}

/// Move byte offset one word backward (Ctrl+Left).
pub(crate) fn prev_word_byte(s: &str, byte: usize) -> usize {
    if byte == 0 {
        return 0;
    }
    let chars: Vec<(usize, char)> = s.char_indices().collect();
    let mut i = chars.partition_point(|(b, _)| *b < byte);
    i = i.saturating_sub(1);
    // Skip non-word
    while i > 0 && !chars[i].1.is_alphanumeric() && chars[i].1 != '_' {
        i -= 1;
    }
    // Skip word
    while i > 0 && (chars[i - 1].1.is_alphanumeric() || chars[i - 1].1 == '_') {
        i -= 1;
    }
    chars.get(i).map(|(b, _)| *b).unwrap_or(0)
}

/// Case-insensitive char equality (uses `to_lowercase`, safe across byte-length
/// differences like `ẞ` -> `ß`).
pub(crate) fn char_ci_eq(a: char, b: char) -> bool {
    let al: String = a.to_lowercase().collect();
    let bl: String = b.to_lowercase().collect();
    al == bl
}

/// Case-insensitive substring search over the ORIGINAL `text`. Returns
/// `(start_byte, end_byte)` of the first match at or after `start`. Byte
/// offsets are valid for `text` (no lowercase-copy drift). Empty pattern => None.
pub(crate) fn ci_find(text: &str, pat: &str, start: usize) -> Option<(usize, usize)> {
    let pat_chars: Vec<char> = pat.chars().collect();
    if pat_chars.is_empty() {
        return None;
    }
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    let mut idx = chars.partition_point(|(b, _)| *b < start);
    'outer: while idx + pat_chars.len() <= chars.len() {
        for (k, &pc) in pat_chars.iter().enumerate() {
            if !char_ci_eq(chars[idx + k].1, pc) {
                idx += 1;
                continue 'outer;
            }
        }
        let (s, _) = chars[idx];
        let (e, ec) = chars[idx + pat_chars.len() - 1];
        return Some((s, e + ec.len_utf8()));
    }
    None
}

pub(crate) fn is_ctrl_char(c: char) -> bool {
    c.is_ascii_control() && c != '\t'
}

pub(crate) fn auto_pair(c: char) -> Option<char> {
    match c {
        '[' => Some(']'),
        '(' => Some(')'),
        '{' => Some('}'),
        '"' => Some('"'),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ci_find_ascii() {
        let text = "Hello World";
        assert_eq!(ci_find(text, "world", 0), Some((6, 11)));
        assert_eq!(ci_find(text, "hello", 0), Some((0, 5)));
        assert_eq!(ci_find(text, "xyz", 0), None);
    }

    #[test]
    fn ci_find_offsets_valid_on_original() {
        // `ẞ` (3 bytes) lowercases to `ß` (2 bytes): byte offsets must come
        // from the ORIGINAL string, never the lowercased copy.
        let text = "abc ẞ x";
        assert_eq!(ci_find(text, "ß", 0), Some((4, 7)));
        assert_eq!(ci_find(text, "ß", 7), None);
        assert_eq!(ci_find(text, "ẞ", 0), Some((4, 7)));
    }

    #[test]
    fn ci_find_after_byte_offset() {
        let text = "foo bar foo";
        assert_eq!(ci_find(text, "foo", 5), Some((8, 11)));
        assert_eq!(ci_find(text, "foo", 8), Some((8, 11)));
    }

    #[test]
    fn ci_find_empty_pattern() {
        assert_eq!(ci_find("anything", "", 0), None);
    }

    #[test]
    fn display_cells_expands_tabs_and_escapes_controls() {
        assert_eq!(
            display_cells("\tx", 4),
            vec![(' ', 0), (' ', 0), (' ', 0), (' ', 0), ('x', 1)]
        );
        assert_eq!(display_cells("ab\tx", 4)[4], ('x', 3));
        // A stray ESC becomes two printable cells, never the raw byte.
        let cells = display_cells("a\x1b[31mb", 4);
        assert!(cells.iter().all(|(c, _)| !c.is_ascii_control()));
        assert_eq!(cells[1], ('^', 1));
        assert_eq!(cells[2], ('[', 1));
    }

    #[test]
    fn display_col_matches_expanded_width() {
        assert_eq!(display_col("\tx", 1, 4), 4);
        assert_eq!(display_col("\tx", 2, 4), 5);
        assert_eq!(display_col("\x1b", 1, 4), 2);
        assert_eq!(display_col("日本", 3, 4), 2);
    }

    #[test]
    fn display_col_to_byte_round_trips() {
        assert_eq!(display_col_to_byte("\tx", 0, 4), 0);
        assert_eq!(display_col_to_byte("\tx", 4, 4), 1);
        assert_eq!(display_col_to_byte("abc", 2, 4), 2);
        assert_eq!(display_col_to_byte("abc", 9, 4), 3);
    }

    #[test]
    fn box_drawing_and_symbols_are_one_cell() {
        // `╔══╗` lines used to be laid out two cells per glyph, which left a
        // stale cell between every character on screen.
        for c in [
            '╔', '═', '╗', '║', '─', '│', '┌', '┘', '├', '▶', '◀', '█', '░', '‹', '›', '←', '→',
            '·', '×', '§', '°', 'é', '─',
        ] {
            assert_eq!(char_col_width(c), 1, "{c:?} must be single width");
        }
    }

    #[test]
    fn cjk_and_wide_chars_stay_two_cells() {
        for c in ['日', '本', '한', 'Ａ', '，', '。', 'あ', 'ア', '漢'] {
            assert_eq!(char_col_width(c), 2, "{c:?} must be double width");
        }
    }

    #[test]
    fn box_line_columns_are_contiguous() {
        // Every glyph of a box drawing line occupies its own column, so the
        // cells come out at 0, 1, 2, … with no gaps for stale text.
        let cells = display_cells("╔══╗", 4);
        assert_eq!(cells.len(), 4);
        assert_eq!(display_col("╔══╗", "╔══╗".len(), 4), 4);
        assert_eq!(display_col_to_byte("╔══╗", 2, 4), "╔═".len());
    }

    #[test]
    fn char_ci_eq_multibyte() {
        assert!(char_ci_eq('ẞ', 'ß'));
        assert!(char_ci_eq('a', 'A'));
        assert!(!char_ci_eq('b', 'a'));
    }
}
