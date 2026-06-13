use super::ast::Parser;
use super::token::tokenize_with_pos;

// ============================================================================
// Syntax validation (used by LSP diagnostics)
// ============================================================================

/// Validate a math expression, returning `Ok(())` or `Err((message, byte_offset))`.
pub(crate) fn validate(input: &str) -> Result<(), (String, usize)> {
    let trimmed = input.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return Ok(());
    }
    let trimmed = if trimmed.starts_with("//") { "" } else { trimmed };
    if trimmed.is_empty() {
        return Ok(());
    }
    let (tokens, positions) = tokenize_with_pos(trimmed)
        .map_err(|e| {
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
