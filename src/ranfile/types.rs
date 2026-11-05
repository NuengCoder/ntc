use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::path::Path;

/// On-disk format of a ranfile.
///
/// TOML is the default and the only format that keeps the commented template;
/// JSON and YAML exist so the same task graph can travel through tooling that
/// speaks those formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RanFormat {
    Toml,
    Json,
    Yaml,
}

impl RanFormat {
    /// Every supported format, in conversion order.
    pub(crate) const ALL: [RanFormat; 3] = [Self::Toml, Self::Json, Self::Yaml];

    /// Parse a user-facing format name (`yml` is accepted for YAML).
    pub(crate) fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_lowercase().as_str() {
            "toml" => Some(Self::Toml),
            "json" => Some(Self::Json),
            "yaml" | "yml" => Some(Self::Yaml),
            _ => None,
        }
    }

    /// Canonical extension (also the canonical name).
    pub(crate) fn ext(self) -> &'static str {
        match self {
            Self::Toml => "toml",
            Self::Json => "json",
            Self::Yaml => "yaml",
        }
    }

    /// Default file name for this format.
    pub(crate) fn file_name(self) -> &'static str {
        match self {
            Self::Toml => "NTCRANFILE.toml",
            Self::Json => "NTCRANFILE.json",
            Self::Yaml => "NTCRANFILE.yaml",
        }
    }

    /// Every file name a ranfile in this format may use (`yml` is a YAML
    /// alias, so it is listed here but never written).
    pub(crate) fn file_names(self) -> &'static [&'static str] {
        match self {
            Self::Toml => &["NTCRANFILE.toml"],
            Self::Json => &["NTCRANFILE.json"],
            Self::Yaml => &["NTCRANFILE.yaml", "NTCRANFILE.yml"],
        }
    }

    /// Format of an existing ranfile path, from its extension.
    pub(crate) fn from_path(path: &Path) -> Option<Self> {
        let ext = path.extension()?.to_str()?.to_lowercase();
        Self::parse(&ext)
    }
}

/// Ordered view of a [`Ranfile`] used for writing.
///
/// `Ranfile` keeps its maps unordered, which would make `ran fmt` output
/// churn between runs. Sorting the keys keeps converted files diffable.
#[derive(Debug, Serialize)]
pub(crate) struct RanfileOut {
    vars: BTreeMap<String, String>,
    targets: BTreeMap<String, RanTargetOut>,
}

/// One target in sorted output order.
#[derive(Debug, Serialize)]
pub(crate) struct RanTargetOut {
    deps: Vec<String>,
    cmd: String,
}

impl RanfileOut {
    /// Build the ordered form, keeping every target's dependency order.
    pub(crate) fn from_ranfile(ranfile: &Ranfile) -> Self {
        Self {
            vars: ranfile
                .vars
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
            targets: ranfile
                .targets
                .iter()
                .map(|(name, target)| {
                    (
                        name.clone(),
                        RanTargetOut {
                            deps: target.deps.clone(),
                            cmd: target.cmd.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

/// Represents a parsed NTCRANFILE.
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
