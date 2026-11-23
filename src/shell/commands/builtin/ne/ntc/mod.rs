//! Language service for `.ntc` scripts, in the same shape as `ne::mql`.
//!
//! `.ntc` grew a block language (if/while/for/loop/func), expressions with
//! builtin functions and per-file directives, so the editor now offers the
//! same assistance MQL gets: completion for keywords, variables, functions and
//! shell commands; signature help while inside a call; and hover text for
//! whatever the cursor is on.

pub(crate) mod extract;
pub(crate) mod hover;
pub(crate) mod lexicon;

pub(crate) use self::extract::{functions, variables};
pub(crate) use self::hover::call_hover;
pub(crate) use self::lexicon::{
    command_doc, command_names, function_doc, function_params, FUNCTIONS, KEYWORDS,
};
