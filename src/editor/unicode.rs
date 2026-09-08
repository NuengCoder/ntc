// ── unicode helpers ──────────────────────────────────────────────────────────

/// Visual width of a character in a terminal (1 or 2 for CJK / fullwidth).
pub(crate) fn char_col_width(c: char) -> usize {
    let cp = c as u32;
    if cp >= 0x1100 && !(0x115F..0x11A7).contains(&cp) && !(0x11A8..0x1200).contains(&cp)
        || (0x2E80..0xA4D7).contains(&cp)
        || (0xA960..0xA97D).contains(&cp)
        || (0xAC00..0xD7A4).contains(&cp)
        || (0xF900..0xFB00).contains(&cp)
        || (0xFE10..0xFE1F).contains(&cp)
        || (0xFE30..0xFE70).contains(&cp)
        || (0xFF01..0xFF61).contains(&cp)
        || (0xFFE0..0xFFE7).contains(&cp)
        || (0x1B000..0x1B100).contains(&cp)
        || (0x1F200..0x1F300).contains(&cp)
        || (0x20000..0x30000).contains(&cp)
        || (0x30000..0x40000).contains(&cp)
    {
        2
    } else {
        1
    }
}

/// Number of visual columns occupied by the first `byte_end` bytes of `s`.
pub(crate) fn byte_to_col(s: &str, byte_end: usize) -> usize {
    s[..byte_end.min(s.len())].chars().map(char_col_width).sum()
}

/// Convert a visual column (relative to text start) to byte offset in `s`.
pub(crate) fn col_to_byte(s: &str, col: usize) -> usize {
    let mut acc = 0usize;
    for ch in s.chars() {
        let w = char_col_width(ch);
        if acc + w > col {
            break;
        }
        acc += w;
    }
    acc
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
    fn char_ci_eq_multibyte() {
        assert!(char_ci_eq('ẞ', 'ß'));
        assert!(char_ci_eq('a', 'A'));
        assert!(!char_ci_eq('b', 'a'));
    }
}
