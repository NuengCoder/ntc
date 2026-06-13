use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a parsed NTCRANFILE.toml.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Ranfile {
    #[serde(default)]
    pub(crate) vars: HashMap<String, String>,
    pub(crate) targets: HashMap<String, RanTarget>,
}

/// A single target in the ranfile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RanTarget {
    #[serde(default)]
    pub(crate) deps: Vec<String>,
    #[serde(default)]
    pub(crate) cmd: String,
}
