// src/fake/mod.rs
//! `fake` engine — deterministic, seedable fake-data generation.
//!
//! This module contains the generators, type metadata, locales, groups and
//! formatters. The shell command adapter lives under
//! `shell::commands::builtin::fake`.

pub mod data;
pub mod format;
pub mod gen;
pub mod group;
pub mod locale;
pub mod rng;
pub mod types;
