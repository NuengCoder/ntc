//! Plugin manifest types (the 2.5.0 contract, parsed but never loaded in
//! 2.4.9). See `docs/architecture/plugins.md` for the full transport and
//! trust model.
//!
//! A manifest is TOML, one per plugin directory:
//!
//! ```toml
//! id = "com.example.hello"
//! name = "Hello"
//! version = "0.1.0"
//! abi = 1
//! entry = "hello-plugin.exe"
//! description = "Example plugin"
//!
//! [commands.greet]
//! summary = { en = "Greet someone" }
//! usage = "plugin com.example.hello greet <name>"
//! subcommands = []
//! flags = ["--loud"]
//! ```

use serde::Deserialize;
use std::collections::BTreeMap;

/// Host API major version this build speaks. The 2.5.0 handshake returns the
/// host's supported range; a manifest whose `abi` does not match is rejected.
pub(crate) const HOST_ABI: u32 = 1;

/// A parsed plugin manifest. `id` is reverse-DNS and immutable; commands are
/// keyed by their bare name (exposed as `plugin <id> <command>`).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub abi: u32,
    pub entry: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub commands: BTreeMap<String, PluginCommand>,
}

/// One command a plugin exposes. Static subcommands/flags are declared here
/// so completion needs no round-trip; dynamic `complete` requests are a later
/// milestone.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PluginCommand {
    pub summary: LocalizedText,
    pub usage: String,
    #[serde(default)]
    pub subcommands: Vec<String>,
    #[serde(default)]
    pub flags: Vec<String>,
}

/// A locale-keyed string (`{ en = "...", th = "..." }`). Lookup prefers the
/// requested locale, then `en`, then any available entry.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(transparent)]
pub(crate) struct LocalizedText(pub BTreeMap<String, String>);

impl LocalizedText {
    pub(crate) fn get(&self, locale: &str) -> Option<&str> {
        self.0
            .get(locale)
            .or_else(|| self.0.get("en"))
            .or_else(|| self.0.values().next())
            .map(String::as_str)
    }
}

impl PluginManifest {
    /// Parse a manifest from TOML text.
    pub(crate) fn parse(text: &str) -> Result<Self, String> {
        let manifest: Self = toml::from_str(text).map_err(|e| e.to_string())?;
        manifest.validate()?;
        Ok(manifest)
    }

    /// Check the structural rules the host relies on. This is deliberately
    /// independent of any install/trust step so it can be unit-tested.
    pub(crate) fn validate(&self) -> Result<(), String> {
        if !is_reverse_dns(&self.id) {
            return Err(format!("id '{}' is not reverse-DNS", self.id));
        }
        if self.name.trim().is_empty() {
            return Err("name is empty".into());
        }
        if self.version.trim().is_empty() {
            return Err("version is empty".into());
        }
        if self.entry.trim().is_empty() {
            return Err("entry is empty".into());
        }
        if self.abi != HOST_ABI {
            return Err(format!(
                "abi {} is unsupported (host speaks {HOST_ABI})",
                self.abi
            ));
        }
        for (name, command) in &self.commands {
            if !is_command_name(name) {
                return Err(format!("command name '{name}' is invalid"));
            }
            if command.usage.trim().is_empty() {
                return Err(format!("command '{name}' has no usage"));
            }
            if command.summary.0.is_empty() {
                return Err(format!("command '{name}' has no summary"));
            }
        }
        Ok(())
    }
}

/// Reverse-DNS: at least two dot-separated labels, each non-empty and made of
/// lowercase ASCII letters, digits, `-` or `_`.
fn is_reverse_dns(id: &str) -> bool {
    let labels: Vec<&str> = id.split('.').collect();
    labels.len() >= 2
        && labels.iter().all(|label| {
            !label.is_empty()
                && label
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
        })
}

/// A bare command name: non-empty, no whitespace, not a flag.
fn is_command_name(name: &str) -> bool {
    !name.is_empty() && !name.starts_with('-') && !name.chars().any(char::is_whitespace)
}

#[cfg(test)]
mod tests {
    use super::*;

    const GOOD: &str = r#"
id = "com.example.hello"
name = "Hello"
version = "0.1.0"
abi = 1
entry = "hello-plugin.exe"
description = "Example plugin"

[commands.greet]
summary = { en = "Greet someone", th = "ทักทาย" }
usage = "plugin com.example.hello greet <name>"
flags = ["--loud"]
"#;

    #[test]
    fn parses_a_valid_manifest() {
        let m = PluginManifest::parse(GOOD).unwrap();
        assert_eq!(m.id, "com.example.hello");
        assert_eq!(m.abi, HOST_ABI);
        let greet = &m.commands["greet"];
        assert_eq!(greet.summary.get("en"), Some("Greet someone"));
        assert_eq!(greet.summary.get("th"), Some("ทักทาย"));
        assert_eq!(greet.flags, vec!["--loud"]);
        assert!(greet.subcommands.is_empty());
    }

    #[test]
    fn localized_text_falls_back_to_en_then_any() {
        let mut map = BTreeMap::new();
        map.insert("th".to_string(), "x".to_string());
        let t = LocalizedText(map);
        assert_eq!(t.get("en"), Some("x"));
        assert_eq!(t.get("zz"), Some("x"));
        assert_eq!(LocalizedText::default().get("en"), None);
    }

    #[test]
    fn rejects_bad_id() {
        for id in ["hello", "com.", ".example", "Com.Example", "com..x"] {
            let text = GOOD.replace("com.example.hello", id);
            assert!(PluginManifest::parse(&text).is_err(), "id {id} accepted");
        }
    }

    #[test]
    fn rejects_unsupported_abi() {
        let text = GOOD.replace("abi = 1", "abi = 99");
        assert!(PluginManifest::parse(&text).is_err());
    }

    #[test]
    fn rejects_empty_required_fields() {
        for (from, to) in [
            ("name = \"Hello\"", "name = \"\""),
            ("version = \"0.1.0\"", "version = \"\""),
            ("entry = \"hello-plugin.exe\"", "entry = \"\""),
        ] {
            let text = GOOD.replace(from, to);
            assert!(PluginManifest::parse(&text).is_err(), "{from} accepted");
        }
    }

    #[test]
    fn rejects_unknown_fields() {
        let text = format!("{GOOD}\nbogus = 1\n");
        assert!(PluginManifest::parse(&text).is_err());
    }

    #[test]
    fn rejects_invalid_command_names_and_missing_usage() {
        let bad_name = GOOD.replace("[commands.greet]", "[commands.\"bad name\"]");
        assert!(PluginManifest::parse(&bad_name).is_err());
        let no_usage = GOOD.replace(
            "usage = \"plugin com.example.hello greet <name>\"",
            "usage = \"\"",
        );
        assert!(PluginManifest::parse(&no_usage).is_err());
    }
}
