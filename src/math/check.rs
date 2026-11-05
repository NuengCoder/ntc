use super::ast::Parser;
use super::token::tokenize_with_pos;

// ============================================================================
// Syntax validation (used by LSP diagnostics)
// ============================================================================

/// Validate a whole `.ntc.math` file, one expression per line.
///
/// Returns `(line, column, message)` for every broken line; blank lines and
/// comments are skipped, and every problem is reported (not just the first).
pub(crate) fn check_text(content: &str) -> Vec<(usize, usize, String)> {
    let mut out = Vec::new();
    for (idx, raw) in content.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if let Err((message, offset)) = validate(line) {
            let mut offset = offset.min(line.len());
            while offset > 0 && !line.is_char_boundary(offset) {
                offset -= 1;
            }
            let col = line[..offset].chars().count() + 1;
            out.push((idx + 1, col, strip_offset(&message)));
        }
    }
    out
}

/// Drop the `at byte N` suffix the tokenizer appends — the report already
/// carries a line and column.
fn strip_offset(message: &str) -> String {
    match message.rfind(" at byte ") {
        Some(pos) => message[..pos].to_string(),
        None => message.to_string(),
    }
}

/// Validate a math expression, returning `Ok(())` or `Err((message, byte_offset))`.
pub(crate) fn validate(input: &str) -> Result<(), (String, usize)> {
    let trimmed = input.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return Ok(());
    }
    let trimmed = if trimmed.starts_with("//") {
        ""
    } else {
        trimmed
    };
    if trimmed.is_empty() {
        return Ok(());
    }
    let (tokens, positions) = tokenize_with_pos(trimmed).map_err(|e| {
        let msg = e.to_string();
        let offset = if let Some(pos) = msg.rfind("at byte ") {
            msg[pos + 8..].parse::<usize>().unwrap_or(0)
        } else {
            0
        };
        (msg, offset)
    })?;
    let mut parser = Parser::with_positions(tokens, positions);
    parser.parse_statement().map_err(|e| {
        let offset = parser.current_offset();
        (e.to_string(), offset)
    })?;
    Ok(())
}
