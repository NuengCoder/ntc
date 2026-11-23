//! Per-language lexer traits and word tables.
//!
//! `spec()` is the only dispatch table: one arm per [`SyntaxLanguage`] variant,
//! no defaults, so a new variant fails to compile until it has a module.

pub(super) mod spec;

mod asm;
mod bash;
mod bat;
mod c;
mod clojure;
mod cpp;
mod csharp;
mod css;
mod dart;
mod dotenv;
mod elixir;
mod erlang;
mod fsharp;
mod glsl;
mod go;
mod go_mod;
mod go_sum;
mod graphql;
mod haskell;
mod html;
mod inno_setup;
mod java;
mod javascript;
mod json;
mod julia;
mod kotlin;
mod lock_file;
mod lua;
mod markdown;
mod mql;
mod nim;
mod ntc_igcare;
mod ntc_math;
mod ntc_ral;
mod ntc_scr;
mod ntc_theme;
mod ocaml;
mod perl;
mod php;
mod pro;
mod ps1;
mod python;
mod r;
mod ruby;
mod rust;
mod scala;
mod sql;
mod swift;
mod txt;
mod typescript;
mod vim;
mod xml;
mod yaml;
mod zig;

use crate::utils::syntax::types::SyntaxLanguage;

pub(super) fn spec(lang: SyntaxLanguage) -> &'static spec::LangSpec {
    match lang {
        SyntaxLanguage::Rust => &rust::SPEC,
        SyntaxLanguage::Python => &python::SPEC,
        SyntaxLanguage::JavaScript => &javascript::SPEC,
        SyntaxLanguage::TypeScript => &typescript::SPEC,
        SyntaxLanguage::Go => &go::SPEC,
        SyntaxLanguage::C => &c::SPEC,
        SyntaxLanguage::Cpp => &cpp::SPEC,
        SyntaxLanguage::CSharp => &csharp::SPEC,
        SyntaxLanguage::Java => &java::SPEC,
        SyntaxLanguage::Kotlin => &kotlin::SPEC,
        SyntaxLanguage::Swift => &swift::SPEC,
        SyntaxLanguage::Dart => &dart::SPEC,
        SyntaxLanguage::Ruby => &ruby::SPEC,
        SyntaxLanguage::Bash => &bash::SPEC,
        SyntaxLanguage::Vim => &vim::SPEC,
        SyntaxLanguage::Lua => &lua::SPEC,
        SyntaxLanguage::Php => &php::SPEC,
        SyntaxLanguage::Perl => &perl::SPEC,
        SyntaxLanguage::Haskell => &haskell::SPEC,
        SyntaxLanguage::Elixir => &elixir::SPEC,
        SyntaxLanguage::Zig => &zig::SPEC,
        SyntaxLanguage::Nim => &nim::SPEC,
        SyntaxLanguage::Scala => &scala::SPEC,
        SyntaxLanguage::Clojure => &clojure::SPEC,
        SyntaxLanguage::OCaml => &ocaml::SPEC,
        SyntaxLanguage::R => &r::SPEC,
        SyntaxLanguage::Julia => &julia::SPEC,
        SyntaxLanguage::Glsl => &glsl::SPEC,
        SyntaxLanguage::Erlang => &erlang::SPEC,
        SyntaxLanguage::Xml => &xml::SPEC,
        SyntaxLanguage::Yaml => &yaml::SPEC,
        SyntaxLanguage::Pro => &pro::SPEC,
        SyntaxLanguage::Html => &html::SPEC,
        SyntaxLanguage::Json => &json::SPEC,
        SyntaxLanguage::Markdown => &markdown::SPEC,
        SyntaxLanguage::Txt => &txt::SPEC,
        SyntaxLanguage::GoMod => &go_mod::SPEC,
        SyntaxLanguage::GoSum => &go_sum::SPEC,
        SyntaxLanguage::Dotenv => &dotenv::SPEC,
        SyntaxLanguage::Css => &css::SPEC,
        SyntaxLanguage::Bat => &bat::SPEC,
        SyntaxLanguage::Ps1 => &ps1::SPEC,
        SyntaxLanguage::Sql => &sql::SPEC,
        SyntaxLanguage::FSharp => &fsharp::SPEC,
        SyntaxLanguage::Asm => &asm::SPEC,
        SyntaxLanguage::Graphql => &graphql::SPEC,
        SyntaxLanguage::NtcScr => &ntc_scr::SPEC,
        SyntaxLanguage::NtcRal => &ntc_ral::SPEC,
        SyntaxLanguage::NtcIgcare => &ntc_igcare::SPEC,
        SyntaxLanguage::NtcMath => &ntc_math::SPEC,
        SyntaxLanguage::NtcTheme => &ntc_theme::SPEC,
        SyntaxLanguage::InnoSetup => &inno_setup::SPEC,
        SyntaxLanguage::LockFile => &lock_file::SPEC,
        SyntaxLanguage::Mql => &mql::SPEC,
    }
}
