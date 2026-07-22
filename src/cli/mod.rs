// src/cli/mod.rs
// ntc CLI module — split from the original single-file cli.rs

mod helpers;
mod run;

pub use run::run_cli;