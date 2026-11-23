//! Html lexer traits and word tables.

use super::spec::LangSpec;

pub(super) const SPEC: LangSpec = LangSpec::new(b"").with_xml_comments().with_tag_syntax();
