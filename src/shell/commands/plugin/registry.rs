//! The plugin registry. 2.4.9 ships it **empty**: nothing reads a plugin
//! directory, spawns a process or loads a library. The type and its
//! collision rules exist so 2.5.0 can add loading without touching the
//! built-in modules. See `docs/architecture/plugins.md`.

use std::collections::BTreeMap;

use super::manifest::PluginManifest;

/// Why a plugin registration was rejected.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum RegisterError {
    /// The manifest failed structural validation.
    Invalid(String),
    /// A plugin with this id is already registered.
    DuplicateId(String),
    /// The id or a command name collides with a built-in name or alias.
    ReservedName(String),
}

impl std::fmt::Display for RegisterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(msg) => write!(f, "invalid manifest: {msg}"),
            Self::DuplicateId(id) => write!(f, "plugin '{id}' is already registered"),
            Self::ReservedName(name) => {
                write!(f, "'{name}' collides with a built-in command or alias")
            }
        }
    }
}

/// An in-memory collection of plugin manifests keyed by id. Registration is
/// the only mutation; there is no loader in 2.4.9.
#[derive(Default)]
pub(crate) struct PluginRegistry {
    plugins: BTreeMap<String, PluginManifest>,
}

impl PluginRegistry {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.plugins.len()
    }

    pub(crate) fn get(&self, id: &str) -> Option<&PluginManifest> {
        self.plugins.get(id)
    }

    pub(crate) fn ids(&self) -> impl Iterator<Item = &str> {
        self.plugins.keys().map(String::as_str)
    }

    /// Register a manifest, rejecting anything that could shadow a built-in.
    ///
    /// Plugins are exposed as `plugin <id> <command>`, so top-level collisions
    /// are impossible by construction; these checks are defense in depth and
    /// also guard the future opt-in short-name tier. Install-time trust and
    /// run-alias collision checks are 2.5.0 concerns (see the architecture
    /// doc) and are intentionally not performed here.
    pub(crate) fn register(&mut self, manifest: PluginManifest) -> Result<(), RegisterError> {
        manifest.validate().map_err(RegisterError::Invalid)?;
        if self.plugins.contains_key(&manifest.id) {
            return Err(RegisterError::DuplicateId(manifest.id));
        }
        let builtins = crate::shell::commands::registry_data::registry();
        if builtins.is_reserved(&manifest.id) {
            return Err(RegisterError::ReservedName(manifest.id));
        }
        for name in manifest.commands.keys() {
            if builtins.is_reserved(name) {
                return Err(RegisterError::ReservedName(name.clone()));
            }
        }
        self.plugins.insert(manifest.id.clone(), manifest);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest(id: &str, command: &str) -> PluginManifest {
        let text = format!(
            r#"
id = "{id}"
name = "Test"
version = "0.1.0"
abi = 1
entry = "test.exe"

[commands.{command}]
summary = {{ en = "A command" }}
usage = "plugin {id} {command}"
"#
        );
        PluginManifest::parse(&text).unwrap()
    }

    #[test]
    fn new_registry_is_empty() {
        let r = PluginRegistry::new();
        assert!(r.is_empty());
        assert_eq!(r.len(), 0);
        assert!(r.get("com.example.x").is_none());
        assert_eq!(r.ids().count(), 0);
    }

    #[test]
    fn registers_and_looks_up_a_plugin() {
        let mut r = PluginRegistry::new();
        r.register(manifest("com.example.hello", "greet")).unwrap();
        assert_eq!(r.len(), 1);
        assert!(r.get("com.example.hello").is_some());
        assert_eq!(r.ids().collect::<Vec<_>>(), vec!["com.example.hello"]);
    }

    #[test]
    fn rejects_duplicate_id() {
        let mut r = PluginRegistry::new();
        r.register(manifest("com.example.hello", "greet")).unwrap();
        let err = r
            .register(manifest("com.example.hello", "other"))
            .unwrap_err();
        assert_eq!(err, RegisterError::DuplicateId("com.example.hello".into()));
        assert_eq!(r.len(), 1);
    }

    #[test]
    fn cannot_shadow_a_builtin_name_or_alias() {
        let builtins = crate::shell::commands::registry_data::registry();
        let mut r = PluginRegistry::new();
        // A command named like a built-in (canonical or alias) is rejected.
        for name in ["view", "cfg", "god", "r", "move"] {
            assert!(builtins.is_reserved(name), "{name} should be a builtin");
            let err = r.register(manifest("com.example.x", name)).unwrap_err();
            assert_eq!(err, RegisterError::ReservedName(name.into()));
        }
        assert!(r.is_empty());
    }

    #[test]
    fn rejects_invalid_manifest() {
        let mut r = PluginRegistry::new();
        let mut m = manifest("com.example.x", "greet");
        m.abi = 99;
        assert!(matches!(r.register(m), Err(RegisterError::Invalid(_))));
        assert!(r.is_empty());
    }
}
