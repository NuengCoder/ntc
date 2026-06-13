use crate::syntax::types::{Token, TokenType, SyntaxLanguage};
use crate::syntax::language::{
    is_ident_start, is_ident_continue, line_comment_prefix,
    has_block_comments, has_xml_comments, has_ps_block_comments,
    has_pascal_comments, has_preprocessor,
    is_char_literal_supported, has_regex_literals, has_tag_syntax,
    is_rust_attr_start, has_annotation_at, has_css_at_rules,
    is_css_at_rule, is_css_property, is_keyword, is_type_name,
    is_type_name_upper_heuristic, is_builtin, is_function_call,
    is_constant, has_toml_fields,
};
use super::types::detect_language;

#[derive(Clone)]
pub struct SyntaxHighlighter {
    pub language: Option<SyntaxLanguage>,
    in_block_comment: bool,
    in_xml_comment: bool,
    in_ps_comment: bool,
    in_pascal_brace_comment: bool,
    in_pascal_paren_comment: bool,
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
            in_regex: false,
            token_cache: Vec::new(),
        }
    }

    pub fn set_language(&mut self, ext: Option<&str>) {
        self.language = ext.and_then(detect_language);
        self.in_block_comment = false;
        self.in_xml_comment = false;
        self.in_ps_comment = false;
        self.in_pascal_brace_comment = false;
        self.in_pascal_paren_comment = false;
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
                    tokens.push(Token { start: i, end: len, token_type: TokenType::Keyword });
                    return tokens;
                }
                if bytes[i] == b'`' {
                    let start = i;
                    let mut backtick_count = 0;
                    while i < len && bytes[i] == b'`' { i += 1; backtick_count += 1; }
                    while i < len {
                        if bytes[i] == b'`' {
                            let mut close_count = 0;
                            while i < len && bytes[i] == b'`' { i += 1; close_count += 1; }
                            if close_count == backtick_count { break; }
                        } else { i += 1; }
                    }
                    tokens.push(Token { start, end: i, token_type: TokenType::Builtin });
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

            if has_block_comments(lang)
                && i + 1 < len
                && bytes[i] == b'/'
                && bytes[i + 1] == b'*'
            {
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

            if is_rust_attr_start(lang) && bytes[i] == b'#' && i + 1 < len && bytes[i + 1] == b'['
            {
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

            if has_regex_literals(lang) && bytes[i] == b'/' && !self.in_regex
                && (i == 0 || !matches!(bytes[i - 1], b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b')' | b']' | b'}' | b'_'))
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
                        && (is_ident_continue(bytes[i])
                            || bytes[i] == b'-'
                            || bytes[i] == b':')
                    {
                        i += 1;
                    }
                    let mut sub_start = i;
                    while i < len && bytes[i] != b'>' {
                        if bytes[i] == b'{' {
                            let mut brace_depth = 1;
                            i += 1;
                            while i < len && brace_depth > 0 {
                                if bytes[i] == b'{' { brace_depth += 1; }
                                else if bytes[i] == b'}' { brace_depth -= 1; }
                                i += 1;
                            }
                        } else if bytes[i] == b'"' || bytes[i] == b'\'' {
                            let q = bytes[i];
                            let qs = i;
                            i += 1;
                            while i < len && bytes[i] != q {
                                if bytes[i] == b'\\' { i += 2; continue; }
                                i += 1;
                            }
                            if i < len { i += 1; }
                            if qs > sub_start {
                                tokens.push(Token { start: sub_start, end: qs, token_type: TokenType::Tag });
                            }
                            tokens.push(Token { start: qs, end: i, token_type: TokenType::String });
                            sub_start = i;
                        } else {
                            i += 1;
                        }
                    }
                    if i < len && bytes[i] == b'>' { i += 1; }
                    if i > sub_start {
                        tokens.push(Token { start: sub_start, end: i, token_type: TokenType::Tag });
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
                            if bytes[i] == b'\\' { i += 1; }
                            i += 1;
                        }
                        if i < len { i += 1; }
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

            if has_preprocessor(lang) && bytes[i] == b'#' && i + 1 < len && is_ident_start(bytes[i + 1]) {
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

                #[allow(clippy::if_same_then_else)]
                let tt = if is_keyword(lang, word) {
                    TokenType::Keyword
                } else if is_type_name(lang, word) {
                    TokenType::Type
                } else if is_type_name_upper_heuristic(lang) && word.starts_with(|c: char| c.is_uppercase()) {
                    TokenType::Type
                } else if is_constant(lang, word) {
                    TokenType::Constant
                } else if is_builtin(lang, word) {
                    TokenType::Builtin
                } else if matches!(lang, SyntaxLanguage::Css) && is_css_property(word) {
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
                    b"==" | b"!=" | b"<=" | b">=" | b"->" | b"=>" | b"++" | b"--"
                        | b"&&" | b"||" | b"<<" | b">>" | b"::" | b".." | b"|>"
                        | b"//" | b"**"
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
                if matches!(three, b"===" | b"!==" | b"<=>" | b">>=" | b"<<=" | b"..=" | b"|>=") {
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
                tokens.push(Token { start: i, end: i + 1, token_type: TokenType::Punctuation });
            } else if !b.is_ascii() {
                tokens.push(Token { start: i, end: i + 1, token_type: TokenType::Normal });
            } else {
                tokens.push(Token { start: i, end: i + 1, token_type: TokenType::Operator });
            }
            i += 1;
        }

        tokens
    }
}