/// Validate a single trimmed MQL line for basic syntax errors.
///
/// Returns `Ok(())` if the line looks fine,
/// `Err((message, byte_offset))` with an error description and position.
pub(crate) fn validate(input: &str) -> Result<(), (String, usize)> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(());
    }

    let bytes = trimmed.as_bytes();

    // ── check 1: unclosed string literal ──────────────────────────────────
    let mut in_string = false;
    let mut string_start = 0;
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'"' {
            if !in_string {
                in_string = true;
                string_start = i;
            } else if i == 0 || bytes[i - 1] != b'\\' {
                in_string = false;
            }
        }
        i += 1;
    }
    if in_string {
        return Err(("Unclosed string literal".to_string(), string_start));
    }

    // ── check 2: character literal validity (odd single quotes) ───────────
    let mut in_char = false;
    let mut char_start = 0;
    let mut j = 0;
    while j < bytes.len() {
        if bytes[j] == b'\'' {
            if !in_char {
                in_char = true;
                char_start = j;
            } else if j == 0 || bytes[j - 1] != b'\\' {
                in_char = false;
            }
        }
        j += 1;
    }
    if in_char {
        return Err(("Unclosed character literal".to_string(), char_start));
    }

    // ── check 3: misplaced colon (not part of ternary ?:) ─────────────────
    let code = trimmed.trim();
    // Skip preprocessor directives, comments, and empty blocks
    if code.starts_with('#') || code.starts_with("//") || code == "{" || code == "}" {
        return Ok(());
    }

    // ── check 4: missing semicolon ────────────────────────────────────────
    // Lines that contain code but don't end with `;`, `{`, `}`, or `:`
    // are likely missing a semicolon.
    // We skip: preprocessor, control flow keywords, function declarations,
    // and block boundaries.
    let last_char = trimmed.chars().last().unwrap_or(' ');
    if !last_char.is_whitespace()
        && last_char != ';'
        && last_char != '{'
        && last_char != '}'
        && last_char != ':'
        && last_char != ','
    {
        // Skip known line-ending-safe patterns
        let first_word = trimmed.split_whitespace().next().unwrap_or("");
        let is_control_flow = matches!(
            first_word,
            "if" | "else"
                | "for"
                | "while"
                | "do"
                | "switch"
                | "case"
                | "break"
                | "continue"
                | "return"
                | "default"
        );
        let is_preproc = trimmed.starts_with('#');
        let is_block_comment = trimmed.starts_with("/*");
        let is_block_comment_end = trimmed.ends_with("*/");
        let is_single_line_comment = trimmed.contains("//");

        if !is_control_flow
            && !is_preproc
            && !is_block_comment
            && !is_block_comment_end
            && !is_single_line_comment
            && !trimmed.starts_with("//")
        {
            // Check if the line is a function/declaration header ending with (
            let has_open_paren_no_close = trimmed.contains('(') && !trimmed.contains(')');
            if !has_open_paren_no_close {
                let offset = trimmed.len().saturating_sub(1);
                return Err(("Missing semicolon".to_string(), offset));
            }
        }
    }

    Ok(())
}
