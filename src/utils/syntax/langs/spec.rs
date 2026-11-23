//! The one data shape every language module exposes: `SPEC`.
//!
//! A language file is a `pub(super) const SPEC: LangSpec` built from
//! [`LangSpec::new`] plus the flags that differ from the defaults, followed by
//! the word tables it customises. Nothing here is language-specific.

fn no_word(_: &str) -> bool {
    false
}

/// Per-language lexer traits and word tables.
pub(crate) struct LangSpec {
    pub(crate) line_comment: &'static [u8],
    pub(crate) block_comments: bool,
    pub(crate) xml_comments: bool,
    pub(crate) ps_block_comments: bool,
    pub(crate) fsharp_block_comments: bool,
    pub(crate) pascal_comments: bool,
    pub(crate) preprocessor: bool,
    pub(crate) char_literals: bool,
    pub(crate) regex_literals: bool,
    pub(crate) tag_syntax: bool,
    pub(crate) annotation_at: bool,
    pub(crate) css_at_rules: bool,
    pub(crate) upper_heuristic: bool,
    pub(crate) toml_fields: bool,
    pub(crate) keyword: fn(&str) -> bool,
    /// `None` means the language has no type table; `is_type_name` then falls
    /// back to the legacy "identifier starts uppercase" rule.
    pub(crate) type_name: Option<fn(&str) -> bool>,
    /// `None` means the language has no builtin table and gets the shared
    /// `self`/`this`/`true`-style literals instead.
    pub(crate) builtin: Option<fn(&str) -> bool>,
    pub(crate) constant: fn(&str) -> bool,
    pub(crate) at_rule: fn(&str) -> bool,
    pub(crate) css_property: fn(&str) -> bool,
}

impl LangSpec {
    /// `//` line comments, no block comments, no word tables: the defaults a
    /// language overrides only where it differs.
    pub(super) const fn new(line_comment: &'static [u8]) -> Self {
        Self {
            line_comment,
            block_comments: false,
            xml_comments: false,
            ps_block_comments: false,
            fsharp_block_comments: false,
            pascal_comments: false,
            preprocessor: false,
            char_literals: false,
            regex_literals: false,
            tag_syntax: false,
            annotation_at: false,
            css_at_rules: false,
            upper_heuristic: false,
            toml_fields: false,
            keyword: no_word,
            type_name: None,
            builtin: None,
            constant: no_word,
            at_rule: no_word,
            css_property: no_word,
        }
    }

    pub(super) const fn with_block_comments(mut self) -> Self {
        self.block_comments = true;
        self
    }

    pub(super) const fn with_xml_comments(mut self) -> Self {
        self.xml_comments = true;
        self
    }

    pub(super) const fn with_ps_block_comments(mut self) -> Self {
        self.ps_block_comments = true;
        self
    }

    pub(super) const fn with_fsharp_block_comments(mut self) -> Self {
        self.fsharp_block_comments = true;
        self
    }

    pub(super) const fn with_pascal_comments(mut self) -> Self {
        self.pascal_comments = true;
        self
    }

    pub(super) const fn with_preprocessor(mut self) -> Self {
        self.preprocessor = true;
        self
    }

    pub(super) const fn with_char_literals(mut self) -> Self {
        self.char_literals = true;
        self
    }

    pub(super) const fn with_regex_literals(mut self) -> Self {
        self.regex_literals = true;
        self
    }

    pub(super) const fn with_tag_syntax(mut self) -> Self {
        self.tag_syntax = true;
        self
    }

    pub(super) const fn with_annotation_at(mut self) -> Self {
        self.annotation_at = true;
        self
    }

    pub(super) const fn with_css_at_rules(mut self) -> Self {
        self.css_at_rules = true;
        self
    }

    pub(super) const fn with_upper_heuristic(mut self) -> Self {
        self.upper_heuristic = true;
        self
    }

    pub(super) const fn with_toml_fields(mut self) -> Self {
        self.toml_fields = true;
        self
    }

    pub(super) const fn keywords(mut self, f: fn(&str) -> bool) -> Self {
        self.keyword = f;
        self
    }

    pub(super) const fn type_names(mut self, f: fn(&str) -> bool) -> Self {
        self.type_name = Some(f);
        self
    }

    pub(super) const fn builtins(mut self, f: fn(&str) -> bool) -> Self {
        self.builtin = Some(f);
        self
    }

    pub(super) const fn constants(mut self, f: fn(&str) -> bool) -> Self {
        self.constant = f;
        self
    }

    pub(super) const fn at_rules(mut self, f: fn(&str) -> bool) -> Self {
        self.at_rule = f;
        self
    }

    pub(super) const fn css_properties(mut self, f: fn(&str) -> bool) -> Self {
        self.css_property = f;
        self
    }
}
