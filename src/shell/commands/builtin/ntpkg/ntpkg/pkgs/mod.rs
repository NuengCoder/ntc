//! One module per package. Each module owns its versions, download source,
//! installer and Termux hint; [`PACKAGES`] is the only list of supported
//! packages in the codebase.

use anyhow::Result;
use std::path::Path;

use super::types::PackageSource;

mod act;
mod cloudflare;
mod devops_tool;
mod docker;
mod docker_compose;
mod dotnet;
mod ffmpeg;
mod flutter;
mod freebuff;
mod gh;
mod git;
mod go;
mod helm;
mod imagemagick;
pub(crate) mod java;
mod kilocode;
mod kind;
mod kotlin;
mod kubectl;
mod kustomize;
mod lua;
mod mimocode;
mod msys2;
mod nginx;
mod nim;
mod node;
mod omniroute;
mod opencode;
mod php;
mod python;
mod ruby;
mod rust;
mod scala;
mod sqlite;
mod sqlitestudio;
mod stockfish;
mod swift;
mod tesseract;
mod vscode;
mod zig;

pub(crate) mod common;
pub(crate) mod npm_guide;

/// Everything `ntpkg` needs to know about one package.
///
/// The `install` signature is the one every installer already had, so adding a
/// package means adding one module and one entry to [`PACKAGES`].
pub(crate) struct Pkg {
    pub(crate) name: &'static str,
    /// `pkg install` name on Termux; `None` when Termux has no package.
    pub(crate) termux: Option<&'static str>,
    /// Translation key for the one-line description (`ntpkg.src_*`).
    pub(crate) description_key: &'static str,
    pub(crate) versions: fn() -> Result<Vec<String>>,
    pub(crate) source: fn(&str) -> Option<PackageSource>,
    pub(crate) install: fn(&str, &str, &PackageSource, &Path) -> Result<()>,
}

/// Every supported package, alphabetically by `name`.
pub(crate) static PACKAGES: &[&Pkg] = &[
    &act::PKG,
    &cloudflare::PKG,
    &devops_tool::PKG,
    &docker::PKG,
    &docker_compose::PKG,
    &dotnet::PKG,
    &ffmpeg::PKG,
    &flutter::PKG,
    &freebuff::PKG,
    &gh::PKG,
    &git::PKG,
    &go::PKG,
    &helm::PKG,
    &imagemagick::PKG,
    &java::PKG,
    &kilocode::PKG,
    &kind::PKG,
    &kotlin::PKG,
    &kubectl::PKG,
    &kustomize::PKG,
    &lua::PKG,
    &mimocode::PKG,
    &msys2::PKG,
    &nginx::PKG,
    &nim::PKG,
    &node::PKG,
    &omniroute::PKG,
    &opencode::PKG,
    &php::PKG,
    &python::PKG,
    &ruby::PKG,
    &rust::PKG,
    &scala::PKG,
    &sqlite::PKG,
    &sqlitestudio::PKG,
    &stockfish::PKG,
    &swift::PKG,
    &tesseract::PKG,
    &vscode::PKG,
    &zig::PKG,
];

/// Look up a package by its CLI name.
pub(crate) fn find(name: &str) -> Option<&'static Pkg> {
    PACKAGES.iter().copied().find(|pkg| pkg.name == name)
}

/// Every supported package name, in [`PACKAGES`] order (alphabetical).
pub(crate) fn names() -> impl Iterator<Item = &'static str> {
    PACKAGES.iter().map(|pkg| pkg.name)
}

/// Every package paired with its localized one-line description.
pub(crate) fn with_descriptions() -> Vec<(&'static str, String)> {
    PACKAGES
        .iter()
        .map(|pkg| (pkg.name, crate::t!(pkg.description_key)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::PACKAGES;
    use super::{find, names};

    #[test]
    fn package_set_is_unique_and_complete() {
        assert_eq!(PACKAGES.len(), 40, "supported package count drifted");
        let mut seen = std::collections::HashSet::new();
        for pkg in PACKAGES {
            assert!(!pkg.name.is_empty(), "package with empty name");
            assert!(seen.insert(pkg.name), "duplicate package '{}'", pkg.name);
            assert!(!pkg.description_key.is_empty());
            assert!(
                pkg.description_key.starts_with("ntpkg.src_"),
                "{} uses an unexpected description key",
                pkg.name
            );
        }
    }

    #[test]
    fn every_name_round_trips_through_find() {
        for name in names() {
            let pkg = find(name).unwrap_or_else(|| panic!("find('{name}') returned None"));
            assert_eq!(pkg.name, name);
        }
        assert!(find("not-a-package").is_none());
    }

    #[test]
    fn every_description_key_resolves_in_all_language_packs() {
        use crate::utils::config::lang::{en, lo, th, zh};
        for (lang, map) in [
            ("en", en::build_map()),
            ("lo", lo::build_map()),
            ("th", th::build_map()),
            ("zh", zh::build_map()),
        ] {
            for pkg in PACKAGES {
                assert!(
                    map.contains_key(pkg.description_key),
                    "{} is missing '{}'",
                    lang,
                    pkg.description_key
                );
            }
        }
    }
}
