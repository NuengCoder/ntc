//! Package registry lookups and the version cache.
//!
//! [`pkgs::PACKAGES`] is the single source of truth; everything here is a
//! derived view over it plus the one shared version cache.

use anyhow::Result;
use std::collections::HashMap;
use std::sync::Mutex;

use super::pkgs;
use super::pkgs::Pkg;
use super::types::PackageSource;

/// In-memory cache: runtime name -> sorted list of available versions.
/// One map for every package; the hinter reads it on each keystroke.
static VERSION_CACHE: Mutex<Option<HashMap<String, Vec<String>>>> = Mutex::new(None);

pub(crate) fn find(name: &str) -> Option<&'static Pkg> {
    pkgs::find(name)
}

pub(crate) fn names() -> impl Iterator<Item = &'static str> {
    pkgs::names()
}

pub(crate) fn with_descriptions() -> Vec<(&'static str, String)> {
    pkgs::with_descriptions()
}

/// Return cached versions only (no HTTP fetch). Used by the hinter on every keystroke.
pub(crate) fn peek_cached_versions(name: &str) -> Vec<String> {
    if let Ok(cache) = VERSION_CACHE.lock() {
        if let Some(ref map) = *cache {
            if let Some(versions) = map.get(name) {
                return versions.clone();
            }
        }
    }
    vec![]
}

/// Return available versions for a package by fetching from the upstream index.
pub(crate) fn list_available_versions(name: &str) -> Vec<String> {
    match fetch_available_versions(name) {
        Ok(versions) => versions,
        Err(e) => {
            eprintln!("ntpkg: failed to fetch versions for '{name}': {e}");
            vec![]
        }
    }
}

fn fetch_available_versions(name: &str) -> Result<Vec<String>> {
    if let Ok(cache) = VERSION_CACHE.lock() {
        if let Some(ref map) = *cache {
            if let Some(versions) = map.get(name) {
                return Ok(versions.clone());
            }
        }
    }

    let versions = match find(name) {
        Some(pkg) => (pkg.versions)()?,
        None => vec![],
    };

    if let Ok(mut cache) = VERSION_CACHE.lock() {
        cache
            .get_or_insert_with(HashMap::new)
            .insert(name.to_string(), versions.clone());
    }

    Ok(versions)
}

/// Return the download source for a package + version.
pub(crate) fn get_source(name: &str, version: &str) -> Option<PackageSource> {
    find(name).and_then(|pkg| (pkg.source)(version))
}
