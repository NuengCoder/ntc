// src/syntax/mod.rs
// ntc syntax highlighting module — split from the original single-file syntax.rs

mod highlighter;
mod language;
mod types;

// Re-exports for external callers (editor, etc.)
pub use highlighter::SyntaxHighlighter;
pub use types::{color_for, detect_language, SyntaxLanguage, Token, TokenType};
