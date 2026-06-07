// src/syntax/mod.rs
// ntc syntax highlighting module — split from the original single-file syntax.rs

mod types;
mod language;
mod highlighter;

// Re-exports for external callers (editor, etc.)
pub use types::{Token, TokenType, SyntaxLanguage, color_for, detect_language};
pub use highlighter::SyntaxHighlighter;