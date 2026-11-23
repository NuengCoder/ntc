// src/utils/syntax/mod.rs
// ntc syntax highlighting module — split from the original single-file syntax.rs

mod highlighter;
mod langs;
mod language;
mod types;

// Re-exports for external callers (editor, etc.)
pub use highlighter::SyntaxHighlighter;
pub use types::{
    color_for, detect_language, detect_language_for_path, SyntaxLanguage, Token, TokenType,
};
