use anyhow::Result;

// ============================================================================
// Tokens
// ============================================================================
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Token {
    Num(f64),
    Str(String),
    Ident(String),
    Op(char),
    LParen,
    RParen,
    Comma,
    Equal,
    ReturnTok,
    Eof,
}

// ============================================================================
// Tokenizer — two entry points
// ============================================================================

/// Tokenize without position tracking (used by CLI).
pub(super) fn tokenize(input: &str) -> Result<Vec<Token>> {
    let (tokens, _) = tokenize_with_pos(input)?;
    Ok(tokens)
}

/// Tokenize with byte-offset tracking (used by LSP).
pub(crate) fn tokenize_with_pos(input: &str) -> Result<(Vec<Token>, Vec<usize>)> {
    let chars: Vec<char> = input.chars().collect();
    let mut tokens = Vec::new();
    let mut positions = Vec::new();
    let mut i = 0;
    let mut byte_offset = 0;

    while i < chars.len() {
        let ch = chars[i];

        if ch == '\u{feff}' { i += 1; byte_offset += 3; continue; }

        if ch.is_whitespace() {
            byte_offset += ch.len_utf8();
            i += 1;
            continue;
        }

        if ch == '#' {
            tokens.push(Token::Eof);
            positions.push(byte_offset);
            return Ok((tokens, positions));
        }
        if ch == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
            tokens.push(Token::Eof);
            positions.push(byte_offset);
            return Ok((tokens, positions));
        }

        // String literals
        if ch == '"' {
            let str_offset = byte_offset;
            byte_offset += ch.len_utf8();
            i += 1;
            let start = i;
            while i < chars.len() && chars[i] != '"' {
                if chars[i] == '\\' && i + 1 < chars.len() {
                    byte_offset += chars[i].len_utf8();
                    i += 1;
                }
                byte_offset += chars[i].len_utf8();
                i += 1;
            }
            let raw: String = chars[start..i].iter().collect();
            // Process escapes
            let mut escaped = String::with_capacity(raw.len());
            let mut esc = raw.chars();
            while let Some(c) = esc.next() {
                if c == '\\' {
                    match esc.next() {
                        Some('n') => escaped.push('\n'),
                        Some('t') => escaped.push('\t'),
                        Some('r') => escaped.push('\r'),
                        Some('\\') => escaped.push('\\'),
                        Some('"') => escaped.push('"'),
                        Some(c) => { escaped.push('\\'); escaped.push(c); }
                        None => escaped.push('\\'),
                    }
                } else {
                    escaped.push(c);
                }
            }
            if i >= chars.len() {
                return Err(anyhow::anyhow!("Unterminated string literal"));
            }
            byte_offset += chars[i].len_utf8();
            i += 1;
            tokens.push(Token::Str(escaped));
            positions.push(str_offset);
            continue;
        }

        if "+-*/^".contains(ch) {
            tokens.push(Token::Op(ch));
            positions.push(byte_offset);
            byte_offset += ch.len_utf8();
            i += 1;
            continue;
        }

        if ch == '(' { tokens.push(Token::LParen); positions.push(byte_offset); byte_offset += ch.len_utf8(); i += 1; continue; }
        if ch == ')' { tokens.push(Token::RParen); positions.push(byte_offset); byte_offset += ch.len_utf8(); i += 1; continue; }
        if ch == ',' { tokens.push(Token::Comma); positions.push(byte_offset); byte_offset += ch.len_utf8(); i += 1; continue; }
        if ch == '=' { tokens.push(Token::Equal); positions.push(byte_offset); byte_offset += ch.len_utf8(); i += 1; continue; }

        if ch.is_ascii_digit() || (ch == '.' && i + 1 < chars.len() && chars[i + 1].is_ascii_digit()) {
            let num_offset = byte_offset;
            if ch == '0' && i + 1 < chars.len() {
                let next = chars[i + 1];
                if next == 'x' || next == 'X' {
                    byte_offset += ch.len_utf8() + chars[i + 1].len_utf8();
                    i += 2;
                    let start = i;
                    while i < chars.len() && chars[i].is_ascii_hexdigit() { byte_offset += chars[i].len_utf8(); i += 1; }
                    let s: String = chars[start..i].iter().collect();
                    let val = i64::from_str_radix(&s, 16)
                        .map_err(|_| anyhow::anyhow!("Invalid hex number: 0x{}", s))?;
                    tokens.push(Token::Num(val as f64));
                    positions.push(num_offset);
                    continue;
                } else if next == 'b' || next == 'B' {
                    byte_offset += ch.len_utf8() + chars[i + 1].len_utf8();
                    i += 2;
                    let start = i;
                    while i < chars.len() && (chars[i] == '0' || chars[i] == '1') { byte_offset += chars[i].len_utf8(); i += 1; }
                    let s: String = chars[start..i].iter().collect();
                    let val = i64::from_str_radix(&s, 2)
                        .map_err(|_| anyhow::anyhow!("Invalid binary number: 0b{}", s))?;
                    tokens.push(Token::Num(val as f64));
                    positions.push(num_offset);
                    continue;
                } else if next == 'o' || next == 'O' {
                    byte_offset += ch.len_utf8() + chars[i + 1].len_utf8();
                    i += 2;
                    let start = i;
                    while i < chars.len() && chars[i] >= '0' && chars[i] <= '7' { byte_offset += chars[i].len_utf8(); i += 1; }
                    let s: String = chars[start..i].iter().collect();
                    let val = i64::from_str_radix(&s, 8)
                        .map_err(|_| anyhow::anyhow!("Invalid octal number: 0o{}", s))?;
                    tokens.push(Token::Num(val as f64));
                    positions.push(num_offset);
                    continue;
                }
            }
            let start = i;
            if ch == '.' { byte_offset += ch.len_utf8(); i += 1; }
            while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                if chars[i] == '.' && i + 1 < chars.len() && chars[i + 1] == '.' { break; }
                byte_offset += chars[i].len_utf8();
                i += 1;
            }
            let s: String = chars[start..i].iter().collect();
            let val: f64 = s.parse().map_err(|_| anyhow::anyhow!("Invalid number: {}", s))?;
            tokens.push(Token::Num(val));
            positions.push(num_offset);
            continue;
        }

        if ch.is_alphabetic() || ch == '_' {
            let ident_offset = byte_offset;
            let start = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') { byte_offset += chars[i].len_utf8(); i += 1; }
            let s: String = chars[start..i].iter().collect();
            positions.push(ident_offset);
            match s.as_str() {
                "return" => tokens.push(Token::ReturnTok),
                _ => tokens.push(Token::Ident(s)),
            }
            continue;
        }

        return Err(anyhow::anyhow!("Unexpected character '{}' at byte {}", ch, byte_offset));
    }

    tokens.push(Token::Eof);
    positions.push(byte_offset);
    Ok((tokens, positions))
}
