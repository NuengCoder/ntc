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

pub(crate) fn is_ctrl_char(c: char) -> bool {
    c.is_ascii_control() && c != '\t'
}

pub(crate) fn auto_pair(c: char) -> Option<char> {
    match c {
        '[' => Some(']'),
        '(' => Some(')'),
        '{' => Some('}'),
        '\'' => Some('\''),
        '"' => Some('"'),
        _ => None,
    }
}
