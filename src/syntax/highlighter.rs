use super::types::{detect_language, detect_language_for_path};
use crate::syntax::language::{
    has_annotation_at, has_block_comments, has_css_at_rules, has_fsharp_block_comments,
    has_pascal_comments, has_preprocessor, has_ps_block_comments, has_regex_literals,
    has_tag_syntax, has_toml_fields, has_xml_comments, is_builtin, is_char_literal_supported,
    is_constant, is_css_at_rule, is_css_property, is_function_call, is_ident_continue,
    is_ident_start, is_keyword, is_rust_attr_start, is_type_name, is_type_name_upper_heuristic,
    line_comment_prefix,
};
use crate::syntax::types::{SyntaxLanguage, Token, TokenType};

#[derive(Clone)]
pub struct SyntaxHighlighter {
    pub language: Option<SyntaxLanguage>,
    in_block_comment: bool,
    in_xml_comment: bool,
    in_ps_comment: bool,
    in_pascal_brace_comment: bool,
    in_pascal_paren_comment: bool,
    in_fsharp_comment: bool,
    in_regex: bool,
    token_cache: Vec<Option<Vec<Token>>>,
}

impl SyntaxHighlighter {
    pub fn new(ext: Option<&str>) -> Self {
        let language = ext.and_then(detect_language);
        Self {
            language,
            in_block_comment: false,
            in_xml_comment: false,
            in_ps_comment: false,
            in_pascal_brace_comment: false,
            in_pascal_paren_comment: false,
            in_fsharp_comment: false,
            in_regex: false,
            token_cache: Vec::new(),
        }
    }

    /// Build a highlighter for a real file path: known dotfile names
    /// (`.bashrc`, `.vimrc`, `.env`, …) win over the extension.
    pub fn for_path(path: &std::path::Path) -> Self {
        let mut highlighter = Self::new(None);
        highlighter.language = detect_language_for_path(path);
        highlighter
    }

    /// Switch language for a file path (name first, then extension).
    pub fn set_language_for_path(&mut self, path: &std::path::Path) {
        self.set_language(None);
        self.language = detect_language_for_path(path);
    }

    pub fn set_language(&mut self, ext: Option<&str>) {
        self.language = ext.and_then(detect_language);
        self.in_block_comment = false;
        self.in_xml_comment = false;
        self.in_ps_comment = false;
        self.in_pascal_brace_comment = false;
        self.in_pascal_paren_comment = false;
        self.in_fsharp_comment = false;
        self.in_regex = false;
        self.token_cache.clear();
    }

    pub fn resize_cache(&mut self, len: usize) {
        self.token_cache.resize(len, None);
    }

    pub fn invalidate_line(&mut self, idx: usize) {
        if idx < self.token_cache.len() {
            self.token_cache[idx] = None;
        }
    }

    pub fn invalidate_all(&mut self) {
        for slot in &mut self.token_cache {
            *slot = None;
        }
        self.in_block_comment = false;
        self.in_xml_comment = false;
        self.in_ps_comment = false;
        self.in_pascal_brace_comment = false;
        self.in_pascal_paren_comment = false;
        self.in_fsharp_comment = false;
        self.in_regex = false;
    }

    pub fn insert_line(&mut self, idx: usize) {
        if idx < self.token_cache.len() {
            self.token_cache.insert(idx, None);
        } else if idx == self.token_cache.len() {
            self.token_cache.push(None);
        }
    }

    pub fn remove_line(&mut self, idx: usize) {
        if idx < self.token_cache.len() {
            self.token_cache.remove(idx);
        }
    }

    pub fn tokenize_line(&mut self, idx: usize, line: &str) -> &[Token] {
        if self.language.is_none() {
            return &[];
        }
        if idx >= self.token_cache.len() {
            self.token_cache.resize(idx + 1, None);
        }
        if self.token_cache[idx].is_none() {
            let tokens = self.tokenize_internal(line);
            self.token_cache[idx] = Some(tokens);
        }
        self.token_cache[idx].as_deref().unwrap_or(&[])
    }

    /// Pre-tokenize all lines up to `max_idx`. Useful for pre-filling the cache
    /// for small-to-medium files on load so scrolling is instant.
    pub fn pre_tokenize_up_to(&mut self, max_idx: usize, lines: &[String]) {
        if self.language.is_none() {
            return;
        }
        let end = max_idx.min(lines.len());
        if self.token_cache.len() < end {
            self.token_cache.resize(end, None);
        }
        for (idx, line) in lines.iter().enumerate().take(end) {
            if self.token_cache[idx].is_none() {
                let tokens = self.tokenize_internal(line);
                self.token_cache[idx] = Some(tokens);
            }
        }
    }

    pub fn token_type_at(&self, idx: usize, byte: usize) -> Option<TokenType> {
        self.token_cache
            .get(idx)
            .and_then(|opt| opt.as_ref())
            .and_then(|tokens| {
                tokens
                    .iter()
                    .find(|t| byte >= t.start && byte < t.end)
                    .map(|t| t.token_type)
            })
    }

    fn tokenize_internal(&mut self, line: &str) -> Vec<Token> {
        let lang = match self.language {
            Some(l) => l,
            None => return Vec::new(),
        };

        let bytes = line.as_bytes();
        let len = bytes.len();
        let mut tokens: Vec<Token> = Vec::with_capacity(line.len() / 4 + 4);
        let mut i = 0;

        let lc_prefix = line_comment_prefix(lang);
        let md_first_non_ws = if matches!(lang, SyntaxLanguage::Markdown) {
            bytes.iter().position(|b| !b.is_ascii_whitespace())
        } else {
            None
        };
        let scr_first_non_ws = if matches!(lang, SyntaxLanguage::NtcScr) {
            bytes.iter().position(|b| !b.is_ascii_whitespace())
        } else {
            None
        };
        // Vim uses `"` both for comments (start of line) and strings.
        let vim_first_non_ws = if matches!(lang, SyntaxLanguage::Vim) {
            bytes.iter().position(|b| !b.is_ascii_whitespace())
        } else {
            None
        };

        if self.in_xml_comment && has_xml_comments(lang) {
            let start = 0;
            while i + 2 < len {
                if bytes[i] == b'-' && bytes[i + 1] == b'-' && bytes[i + 2] == b'>' {
                    tokens.push(Token {
                        start,
                        end: i + 3,
                        token_type: TokenType::Comment,
                    });
                    i += 3;
                    self.in_xml_comment = false;
                    break;
                }
                i += 1;
            }
            if self.in_xml_comment {
                tokens.push(Token {
                    start,
                    end: len,
                    token_type: TokenType::Comment,
                });
                return tokens;
            }
        }

        if self.in_block_comment && has_block_comments(lang) {
            let start = 0;
            while i + 1 < len {
                if bytes[i] == b'*' && bytes[i + 1] == b'/' {
                    tokens.push(Token {
                        start,
                        end: i + 2,
                        token_type: TokenType::Comment,
                    });
                    i += 2;
                    self.in_block_comment = false;
                    break;
                }
                i += 1;
            }
            if self.in_block_comment {
                tokens.push(Token {
                    start,
                    end: len,
                    token_type: TokenType::Comment,
                });
                return tokens;
            }
        }

        if self.in_ps_comment && has_ps_block_comments(lang) {
            let start = 0;
            while i + 1 < len {
                if bytes[i] == b'#' && bytes[i + 1] == b'>' {
                    tokens.push(Token {
                        start,
                        end: i + 2,
                        token_type: TokenType::Comment,
                    });
                    i += 2;
                    self.in_ps_comment = false;
                    break;
                }
                i += 1;
            }
            if self.in_ps_comment {
                tokens.push(Token {
                    start,
                    end: len,
                    token_type: TokenType::Comment,
                });
                return tokens;
            }
        }

        // F# `(* … *)` — MUST come before the Pascal `(* … *)` branch below,
        // because both use the same delimiters. The two languages are disjoint
        // (`has_fsharp_block_comments` only returns true for `FSharp`,
        // `has_pascal_comments` only for `InnoSetup`), so order here is a
        // belt-and-braces guarantee rather than a functional necessity.
        if self.in_fsharp_comment && has_fsharp_block_comments(lang) {
            let start = 0;
            while i + 1 < len {
                if bytes[i] == b'*' && bytes[i + 1] == b')' {
                    tokens.push(Token {
                        start,
                        end: i + 2,
                        token_type: TokenType::Comment,
                    });
                    i += 2;
                    self.in_fsharp_comment = false;
                    break;
                }
                i += 1;
            }
            if self.in_fsharp_comment {
                tokens.push(Token {
                    start,
                    end: len,
                    token_type: TokenType::Comment,
                });
                return tokens;
            }
        }

        if self.in_pascal_brace_comment && has_pascal_comments(lang) {
            let start = 0;
            while i < len {
                if bytes[i] == b'}' {
                    tokens.push(Token {
                        start,
                        end: i + 1,
                        token_type: TokenType::Comment,
                    });
                    i += 1;
                    self.in_pascal_brace_comment = false;
                    break;
                }
                i += 1;
            }
            if self.in_pascal_brace_comment {
                tokens.push(Token {
                    start,
                    end: len,
                    token_type: TokenType::Comment,
                });
                return tokens;
            }
        }

        if self.in_pascal_paren_comment && has_pascal_comments(lang) {
            let start = 0;
            while i + 1 < len {
                if bytes[i] == b'*' && bytes[i + 1] == b')' {
                    tokens.push(Token {
                        start,
                        end: i + 2,
                        token_type: TokenType::Comment,
                    });
                    i += 2;
                    self.in_pascal_paren_comment = false;
                    break;
                }
                i += 1;
            }
            if self.in_pascal_paren_comment {
                tokens.push(Token {
                    start,
                    end: len,
                    token_type: TokenType::Comment,
                });
                return tokens;
            }
        }

        while i < len {
            if bytes[i].is_ascii_whitespace() {
                i += 1;
                continue;
            }

            if vim_first_non_ws == Some(i) && bytes[i] == b'"' {
                tokens.push(Token {
                    start: i,
                    end: len,
                    token_type: TokenType::Comment,
                });
                return tokens;
            }

            if !lc_prefix.is_empty()
                && i + lc_prefix.len() <= len
                && bytes[i..].starts_with(lc_prefix)
            {
                tokens.push(Token {
                    start: i,
                    end: len,
                    token_type: TokenType::Comment,
                });
                return tokens;
            }

            if matches!(lang, SyntaxLanguage::Markdown) {
                if md_first_non_ws == Some(i) && bytes[i] == b'#' {
                    tokens.push(Token {
                        start: i,
                        end: len,
                        token_type: TokenType::Keyword,
                    });
                    return tokens;
                }
                if bytes[i] == b'`' {
                    let start = i;
                    let mut backtick_count = 0;
                    while i < len && bytes[i] == b'`' {
                        i += 1;
                        backtick_count += 1;
                    }
                    while i < len {
                        if bytes[i] == b'`' {
                            let mut close_count = 0;
                            while i < len && bytes[i] == b'`' {
                                i += 1;
                                close_count += 1;
                            }
                            if close_count == backtick_count {
                                break;
                            }
                        } else {
                            i += 1;
                        }
                    }
                    tokens.push(Token {
                        start,
                        end: i,
                        token_type: TokenType::Builtin,
                    });
                    continue;
                }
            }

            if matches!(lang, SyntaxLanguage::NtcScr) {
                if scr_first_non_ws == Some(i)
                    && i + 1 < len
                    && bytes[i] == b'/'
                    && bytes[i + 1] == b'/'
                {
                    tokens.push(Token {
                        start: i,
                        end: len,
                        token_type: TokenType::Comment,
                    });
                    return tokens;
                }
                if bytes[i] == b'$' {
                    let start = i;
                    i += 1;
                    if i < len && (bytes[i] == b'@' || bytes[i] == b'*') {
                        i += 1;
                    } else if i < len && bytes[i] == b'{' {
                        let mut depth = 1;
                        i += 1;
                        while i < len && depth > 0 {
                            if bytes[i] == b'{' {
                                depth += 1;
                            } else if bytes[i] == b'}' {
                                depth -= 1;
                            }
                            i += 1;
                        }
                    } else if i < len && bytes[i].is_ascii_digit() {
                        while i < len && bytes[i].is_ascii_digit() {
                            i += 1;
                        }
                    } else if i < len && is_ident_start(bytes[i]) {
                        while i < len && is_ident_continue(bytes[i]) {
                            i += 1;
                        }
                    }
                    tokens.push(Token {
                        start,
                        end: i,
                        token_type: TokenType::Builtin,
                    });
                    continue;
                }
            }

            if has_xml_comments(lang)
                && i + 3 < len
                && bytes[i] == b'<'
                && bytes[i + 1] == b'!'
                && bytes[i + 2] == b'-'
                && bytes[i + 3] == b'-'
            {
                let start = i;
                i += 4;
                let mut closed = false;
                while i + 2 < len {
                    if bytes[i] == b'-' && bytes[i + 1] == b'-' && bytes[i + 2] == b'>' {
                        tokens.push(Token {
                            start,
                            end: i + 3,
                            token_type: TokenType::Comment,
                        });
                        i += 3;
                        closed = true;
                        break;
                    }
                    i += 1;
                }
                if !closed {
                    tokens.push(Token {
                        start,
                        end: len,
                        token_type: TokenType::Comment,
                    });
                    self.in_xml_comment = true;
                    return tokens;
                }
                continue;
            }

            if has_ps_block_comments(lang)
                && i + 1 < len
                && bytes[i] == b'<'
                && bytes[i + 1] == b'#'
            {
                let start = i;
                i += 2;
                let mut closed = false;
                while i + 1 < len {
                    if bytes[i] == b'#' && bytes[i + 1] == b'>' {
                        tokens.push(Token {
                            start,
                            end: i + 2,
                            token_type: TokenType::Comment,
                        });
                        i += 2;
                        closed = true;
                        break;
                    }
                    i += 1;
                }
                if !closed {
                    tokens.push(Token {
                        start,
                        end: len,
                        token_type: TokenType::Comment,
                    });
                    self.in_ps_comment = true;
                    return tokens;
                }
                continue;
            }

            if has_block_comments(lang) && i + 1 < len && bytes[i] == b'/' && bytes[i + 1] == b'*' {
                let start = i;
                i += 2;
                let mut closed = false;
                while i + 1 < len {
                    if bytes[i] == b'*' && bytes[i + 1] == b'/' {
                        tokens.push(Token {
                            start,
                            end: i + 2,
                            token_type: TokenType::Comment,
                        });
                        i += 2;
                        closed = true;
                        break;
                    }
                    i += 1;
                }
                if !closed {
                    tokens.push(Token {
                        start,
                        end: len,
                        token_type: TokenType::Comment,
                    });
                    self.in_block_comment = true;
                    return tokens;
                }
                continue;
            }

            // F# `(* … *)` — must be checked before the Pascal block, since
            // they share the same opening delimiter.
            if has_fsharp_block_comments(lang)
                && i + 1 < len
                && bytes[i] == b'('
                && bytes[i + 1] == b'*'
            {
                let start = i;
                i += 2;
                let mut depth: usize = 1;
                let mut closed = false;
                while i < len {
                    if i + 1 < len && bytes[i] == b'(' && bytes[i + 1] == b'*' {
                        depth += 1;
                        i += 2;
                        continue;
                    }
                    if i + 1 < len && bytes[i] == b'*' && bytes[i + 1] == b')' {
                        depth -= 1;
                        i += 2;
                        if depth == 0 {
                            tokens.push(Token {
                                start,
                                end: i,
                                token_type: TokenType::Comment,
                            });
                            closed = true;
                            break;
                        }
                        continue;
                    }
                    i += 1;
                }
                if !closed {
                    tokens.push(Token {
                        start,
                        end: len,
                        token_type: TokenType::Comment,
                    });
                    self.in_fsharp_comment = true;
                    return tokens;
                }
                continue;
            }

            if has_pascal_comments(lang) {
                if bytes[i] == b'{' {
                    let start = i;
                    i += 1;
                    let mut closed = false;
                    while i < len {
                        if bytes[i] == b'}' {
                            tokens.push(Token {
                                start,
                                end: i + 1,
                                token_type: TokenType::Comment,
                            });
                            i += 1;
                            closed = true;
                            break;
                        }
                        i += 1;
                    }
                    if !closed {
                        tokens.push(Token {
                            start,
                            end: len,
                            token_type: TokenType::Comment,
                        });
                        self.in_pascal_brace_comment = true;
                        return tokens;
                    }
                    continue;
                }
                if i + 1 < len && bytes[i] == b'(' && bytes[i + 1] == b'*' {
                    let start = i;
                    i += 2;
                    let mut closed = false;
                    while i + 1 < len {
                        if bytes[i] == b'*' && bytes[i + 1] == b')' {
                            tokens.push(Token {
                                start,
                                end: i + 2,
                                token_type: TokenType::Comment,
                            });
                            i += 2;
                            closed = true;
                            break;
                        }
                        i += 1;
                    }
                    if !closed {
                        tokens.push(Token {
                            start,
                            end: len,
                            token_type: TokenType::Comment,
                        });
                        self.in_pascal_paren_comment = true;
                        return tokens;
                    }
                    continue;
                }
            }

            if bytes[i] == b'"' {
                let start = i;
                i += 1;
                while i < len {
                    if bytes[i] == b'\\' {
                        i += 2;
                        continue;
                    }
                    if bytes[i] == b'"' {
                        i += 1;
                        break;
                    }
                    i += 1;
                }
                tokens.push(Token {
                    start,
                    end: i.min(len),
                    token_type: TokenType::String,
                });
                continue;
            }

            if bytes[i] == b'\'' {
                let use_char = is_char_literal_supported(lang);
                let start = i;
                i += 1;
                while i < len {
                    if bytes[i] == b'\\' {
                        i += 2;
                        continue;
                    }
                    if bytes[i] == b'\'' {
                        i += 1;
                        break;
                    }
                    i += 1;
                }
                tokens.push(Token {
                    start,
                    end: i.min(len),
                    token_type: if use_char {
                        TokenType::Number
                    } else {
                        TokenType::String
                    },
                });
                continue;
            }

            if bytes[i].is_ascii_digit() {
                let start = i;
                i += 1;
                while i < len
                    && (bytes[i].is_ascii_digit()
                        || bytes[i] == b'.'
                        || bytes[i] == b'_'
                        || bytes[i] == b'x'
                        || bytes[i] == b'X'
                        || bytes[i] == b'o'
                        || bytes[i] == b'O'
                        || bytes[i] == b'b'
                        || bytes[i] == b'B'
                        || bytes[i] == b'e'
                        || bytes[i] == b'E'
                        || (bytes[i] >= b'a' && bytes[i] <= b'f')
                        || (bytes[i] >= b'A' && bytes[i] <= b'F'))
                {
                    i += 1;
                }
                tokens.push(Token {
                    start,
                    end: i,
                    token_type: TokenType::Number,
                });
                continue;
            }

            if is_rust_attr_start(lang) && bytes[i] == b'#' && i + 1 < len && bytes[i + 1] == b'[' {
                let start = i;
                i += 2;
                let mut depth = 1;
                while i < len && depth > 0 {
                    if bytes[i] == b'[' {
                        depth += 1;
                    } else if bytes[i] == b']' {
                        depth -= 1;
                    } else if bytes[i] == b'"' {
                        i += 1;
                        while i < len {
                            if bytes[i] == b'\\' {
                                i += 2;
                                continue;
                            }
                            if bytes[i] == b'"' {
                                break;
                            }
                            i += 1;
                        }
                    }
                    i += 1;
                }
                tokens.push(Token {
                    start,
                    end: i.min(len),
                    token_type: TokenType::Attribute,
                });
                continue;
            }

            if has_css_at_rules(lang) && bytes[i] == b'@' {
                let start = i;
                i += 1;
                while i < len && is_ident_continue(bytes[i]) {
                    i += 1;
                }
                let word = &line[start..i];
                let tt = if is_css_at_rule(word) {
                    TokenType::Keyword
                } else {
                    TokenType::Attribute
                };
                tokens.push(Token {
                    start,
                    end: i,
                    token_type: tt,
                });
                continue;
            }

            if has_annotation_at(lang) && bytes[i] == b'@' {
                let start = i;
                i += 1;
                while i < len && is_ident_continue(bytes[i]) {
                    i += 1;
                }
                tokens.push(Token {
                    start,
                    end: i,
                    token_type: TokenType::Attribute,
                });
                continue;
            }

            if has_regex_literals(lang)
                && bytes[i] == b'/'
                && !self.in_regex
                && (i == 0
                    || !matches!(bytes[i - 1], b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b')' | b']' | b'}' | b'_'))
            {
                let start = i;
                i += 1;
                let mut escaped = false;
                while i < len {
                    if escaped {
                        escaped = false;
                        i += 1;
                        continue;
                    }
                    if bytes[i] == b'\\' {
                        escaped = true;
                        i += 1;
                        continue;
                    }
                    if bytes[i] == b'/' {
                        i += 1;
                        break;
                    }
                    if bytes[i] == b'\n' {
                        break;
                    }
                    i += 1;
                }
                while i < len && (bytes[i].is_ascii_alphabetic() || bytes[i] == b'_') {
                    i += 1;
                }
                tokens.push(Token {
                    start,
                    end: i.min(len),
                    token_type: TokenType::Regex,
                });
                continue;
            }

            if has_tag_syntax(lang) && bytes[i] == b'<' {
                let start = i;
                let is_closing = i + 1 < len && bytes[i + 1] == b'/';

                if is_closing {
                    i += 2;
                } else {
                    i += 1;
                }

                if i < len && (is_ident_start(bytes[i]) || bytes[i] == b'>') {
                    while i < len
                        && (is_ident_continue(bytes[i]) || bytes[i] == b'-' || bytes[i] == b':')
                    {
                        i += 1;
                    }
                    let mut sub_start = i;
                    while i < len && bytes[i] != b'>' {
                        if bytes[i] == b'{' {
                            let mut brace_depth = 1;
                            i += 1;
                            while i < len && brace_depth > 0 {
                                if bytes[i] == b'{' {
                                    brace_depth += 1;
                                } else if bytes[i] == b'}' {
                                    brace_depth -= 1;
                                }
                                i += 1;
                            }
                        } else if bytes[i] == b'"' || bytes[i] == b'\'' {
                            let q = bytes[i];
                            let qs = i;
                            i += 1;
                            while i < len && bytes[i] != q {
                                if bytes[i] == b'\\' {
                                    i += 2;
                                    continue;
                                }
                                i += 1;
                            }
                            if i < len {
                                i += 1;
                            }
                            if qs > sub_start {
                                tokens.push(Token {
                                    start: sub_start,
                                    end: qs,
                                    token_type: TokenType::Tag,
                                });
                            }
                            tokens.push(Token {
                                start: qs,
                                end: i,
                                token_type: TokenType::String,
                            });
                            sub_start = i;
                        } else {
                            i += 1;
                        }
                    }
                    if i < len && bytes[i] == b'>' {
                        i += 1;
                    }
                    if i > sub_start {
                        tokens.push(Token {
                            start: sub_start,
                            end: i,
                            token_type: TokenType::Tag,
                        });
                    }
                    continue;
                }
                i = start;
            }

            if has_toml_fields(lang) && bytes[i] == b'[' {
                let start = i;
                i += 1;
                let mut depth = 1;
                while i < len && depth > 0 {
                    if bytes[i] == b'[' {
                        depth += 1;
                    } else if bytes[i] == b']' {
                        depth -= 1;
                    } else if bytes[i] == b'"' {
                        i += 1;
                        while i < len && bytes[i] != b'"' {
                            if bytes[i] == b'\\' {
                                i += 1;
                            }
                            i += 1;
                        }
                        if i < len {
                            i += 1;
                        }
                        continue;
                    }
                    i += 1;
                }
                tokens.push(Token {
                    start,
                    end: i.min(len),
                    token_type: TokenType::Tag,
                });
                continue;
            }

            if has_preprocessor(lang)
                && bytes[i] == b'#'
                && i + 1 < len
                && is_ident_start(bytes[i + 1])
            {
                let start = i;
                i += 1;
                while i < len && is_ident_continue(bytes[i]) {
                    i += 1;
                }
                tokens.push(Token {
                    start,
                    end: i,
                    token_type: TokenType::Macro,
                });
                continue;
            }

            if is_ident_start(bytes[i]) {
                let start = i;
                while i < len && is_ident_continue(bytes[i]) {
                    i += 1;
                }
                let word = &line[start..i];
                let next = if i < len { Some(bytes[i]) } else { None };

                if matches!(lang, SyntaxLanguage::Rust) && next == Some(b'!') {
                    i += 1;
                    tokens.push(Token {
                        start,
                        end: i,
                        token_type: TokenType::Macro,
                    });
                    continue;
                }

                let tt = if is_keyword(lang, word) {
                    TokenType::Keyword
                } else if matches!(lang, SyntaxLanguage::NtcScr) && line[..start].trim() == "set" {
                    TokenType::Attribute
                } else if is_type_name(lang, word)
                    || (is_type_name_upper_heuristic(lang)
                        && word.starts_with(|c: char| c.is_uppercase()))
                {
                    TokenType::Type
                } else if is_constant(lang, word) {
                    TokenType::Constant
                } else if is_builtin(lang, word)
                    || (matches!(lang, SyntaxLanguage::Css) && is_css_property(word))
                {
                    TokenType::Builtin
                } else if is_function_call(word, next) {
                    TokenType::Function
                } else {
                    TokenType::Normal
                };

                tokens.push(Token {
                    start,
                    end: i,
                    token_type: tt,
                });
                continue;
            }

            if i + 1 < len {
                let two = &bytes[i..i + 2];
                if matches!(
                    two,
                    b"=="
                        | b"!="
                        | b"<="
                        | b">="
                        | b"->"
                        | b"=>"
                        | b"++"
                        | b"--"
                        | b"&&"
                        | b"||"
                        | b"<<"
                        | b">>"
                        | b"::"
                        | b".."
                        | b"|>"
                        | b"//"
                        | b"**"
                ) {
                    tokens.push(Token {
                        start: i,
                        end: i + 2,
                        token_type: TokenType::Operator,
                    });
                    i += 2;
                    continue;
                }
            }

            if i + 2 < len {
                let three = &bytes[i..i + 3];
                if matches!(
                    three,
                    b"===" | b"!==" | b"<=>" | b">>=" | b"<<=" | b"..=" | b"|>="
                ) {
                    tokens.push(Token {
                        start: i,
                        end: i + 3,
                        token_type: TokenType::Operator,
                    });
                    i += 3;
                    continue;
                }
            }

            let b = bytes[i];
            if b.is_ascii_punctuation() {
                tokens.push(Token {
                    start: i,
                    end: i + 1,
                    token_type: TokenType::Punctuation,
                });
            } else if !b.is_ascii() {
                tokens.push(Token {
                    start: i,
                    end: i + 1,
                    token_type: TokenType::Normal,
                });
            } else {
                tokens.push(Token {
                    start: i,
                    end: i + 1,
                    token_type: TokenType::Operator,
                });
            }
            i += 1;
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::types::TokenType;

    fn toks(line: &str) -> Vec<(usize, usize, TokenType)> {
        let mut h = SyntaxHighlighter::new(Some("ntc"));
        h.tokenize_line(0, line)
            .iter()
            .map(|t| (t.start, t.end, t.token_type))
            .collect()
    }

    fn has_type(line: &str, needle: TokenType) -> bool {
        toks(line).iter().any(|(_, _, tt)| *tt == needle)
    }

    #[test]
    fn detects_ntc_language() {
        assert_eq!(
            SyntaxHighlighter::new(Some("ntc")).language,
            Some(SyntaxLanguage::NtcScr)
        );
    }

    #[test]
    fn hash_comment_is_comment() {
        let t = toks("# this is a comment");
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].2, TokenType::Comment);
    }

    #[test]
    fn slash_slash_comment_is_comment() {
        let t = toks("// this is a comment");
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].2, TokenType::Comment);
    }

    #[test]
    fn inline_hash_comment_starts_comment() {
        assert!(toks("say hi # trailing")
            .iter()
            .any(|(_, _, tt)| *tt == TokenType::Comment));
    }

    #[test]
    fn set_statement_tokens() {
        let t = toks("set name = \"hello\"");
        assert_eq!(t[0], (0, 3, TokenType::Keyword));
        assert!(t.iter().any(|(_, _, tt)| *tt == TokenType::Attribute));
        assert!(t.iter().any(|(_, _, tt)| *tt == TokenType::String));
    }

    #[test]
    fn force_at_is_attribute() {
        assert!(has_type("@ git status", TokenType::Attribute));
    }

    #[test]
    fn variable_interpolation_is_builtin() {
        for line in &[
            "$name",
            "${name}",
            "${name:-default}",
            "$1",
            "$12",
            "$@",
            "$*",
        ] {
            let t = toks(line);
            assert_eq!(t.len(), 1, "line {:?}", line);
            assert_eq!(t[0].2, TokenType::Builtin, "line {:?}", line);
        }
    }

    #[test]
    fn hash_inside_quotes_is_not_comment() {
        let t = toks("say \"a # b\"");
        assert!(!t.iter().any(|(_, _, tt)| *tt == TokenType::Comment));
        assert!(t.iter().any(|(_, _, tt)| *tt == TokenType::String));
    }

    fn toks_path(name: &str, line: &str) -> Vec<(usize, usize, TokenType)> {
        let mut h = SyntaxHighlighter::for_path(std::path::Path::new(name));
        h.tokenize_line(0, line)
            .iter()
            .map(|t| (t.start, t.end, t.token_type))
            .collect()
    }

    #[test]
    fn dotfile_paths_pick_a_language() {
        for name in [".vimrc", ".nvimrc", ".gvimrc", "init.vim"] {
            assert_eq!(
                SyntaxHighlighter::for_path(std::path::Path::new(name)).language,
                Some(SyntaxLanguage::Vim),
                "{}",
                name
            );
        }
        for name in [".bashrc", ".zshrc", ".profile"] {
            assert_eq!(
                SyntaxHighlighter::for_path(std::path::Path::new(name)).language,
                Some(SyntaxLanguage::Bash),
                "{}",
                name
            );
        }
        for name in [".env", ".env.local", ".gitignore"] {
            assert_eq!(
                SyntaxHighlighter::for_path(std::path::Path::new(name)).language,
                Some(SyntaxLanguage::Dotenv),
                "{}",
                name
            );
        }
        assert_eq!(
            SyntaxHighlighter::for_path(std::path::Path::new("main.rs")).language,
            Some(SyntaxLanguage::Rust)
        );
    }

    #[test]
    fn vim_leading_quote_comments_but_keeps_strings() {
        let t = toks_path(".vimrc", "  \" set number");
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].2, TokenType::Comment);

        let t = toks_path(".vimrc", "let g:name = \"value\"");
        assert!(t.iter().any(|(_, _, tt)| *tt == TokenType::Keyword));
        assert!(t.iter().any(|(_, _, tt)| *tt == TokenType::String));
        assert!(!t.iter().any(|(_, _, tt)| *tt == TokenType::Comment));
    }

    #[test]
    fn bashrc_dotfile_uses_hash_comments() {
        let t = toks_path(".bashrc", "# export PATH");
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].2, TokenType::Comment);
        let t = toks_path(".bashrc", "if [ -f file ]; then");
        assert!(t.iter().any(|(_, _, tt)| *tt == TokenType::Keyword));
        assert!(!t.iter().any(|(_, _, tt)| *tt == TokenType::Comment));
    }

    // ---- F# block comments ----

    #[test]
    fn fsharp_inline_block_comment() {
        let mut h = SyntaxHighlighter::new(Some("fs"));
        let t: Vec<_> = h
            .tokenize_line(0, "let x = 1 (* note *) + 2")
            .iter()
            .map(|t| (t.start, t.end, t.token_type))
            .collect();
        assert!(t.iter().any(|(_, _, tt)| *tt == TokenType::Comment));
        assert!(!h.in_fsharp_comment);
    }

    #[test]
    fn fsharp_multiline_block_comment() {
        let mut h = SyntaxHighlighter::new(Some("fs"));
        let t0: Vec<_> = h
            .tokenize_line(0, "(* start")
            .iter()
            .map(|t| (t.start, t.end, t.token_type))
            .collect();
        assert_eq!(t0.len(), 1);
        assert_eq!(t0[0].2, TokenType::Comment);
        assert!(h.in_fsharp_comment);

        let t1: Vec<_> = h
            .tokenize_line(1, "still inside *)")
            .iter()
            .map(|t| (t.start, t.end, t.token_type))
            .collect();
        assert_eq!(t1.len(), 1);
        assert_eq!(t1[0].2, TokenType::Comment);
        assert!(!h.in_fsharp_comment);
    }

    #[test]
    fn fsharp_nested_block_comments() {
        let mut h = SyntaxHighlighter::new(Some("fs"));
        let t: Vec<_> = h
            .tokenize_line(0, "(* outer (* inner *) still outer *) let y = 2")
            .iter()
            .map(|t| (t.start, t.end, t.token_type))
            .collect();
        // The whole nested comment is one Comment token.
        assert!(t.iter().any(|(_, _, tt)| *tt == TokenType::Comment));
        assert!(!h.in_fsharp_comment);
    }
}
