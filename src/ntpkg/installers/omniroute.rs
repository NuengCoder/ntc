use anyhow::Result;
use std::path::Path;

use super::super::types::PackageSource;
use super::npm_guide;

pub(crate) fn install_omniroute(
    _name: &str,
    _version: &str,
    _source: &PackageSource,
    _vdir: &Path,
) -> Result<()> {
    npm_guide::print_npm_guide("omniroute", "omniroute")
}
