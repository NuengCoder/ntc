//! opencode — version listing, download source and installer.

use super::super::types::PackageSource;
use super::common::empty_source;
use super::npm_guide;
use super::Pkg;
use anyhow::Result;
use std::path::Path;

fn latest_versions() -> Result<Vec<String>> {
    Ok(vec!["latest".to_string()])
}

fn npm_only_source(version: &str) -> Option<PackageSource> {
    empty_source("opencode", version)
}

pub(crate) fn install_opencode(
    _name: &str,
    _version: &str,
    _source: &PackageSource,
    _vdir: &Path,
) -> Result<()> {
    npm_guide::print_npm_guide("opencode", "@opencode/cli")
}

pub(crate) static PKG: Pkg = Pkg {
    name: "opencode",
    termux: None,
    description_key: "ntpkg.src_opencode",
    versions: latest_versions,
    source: npm_only_source,
    install: install_opencode,
};
