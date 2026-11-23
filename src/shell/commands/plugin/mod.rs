//! Plugin seam for 2.5.0.
//!
//! 2.4.9 ships the manifest types, the registry type and an **empty**
//! process-wide registry. Nothing here reads a plugin directory, spawns a
//! subprocess or loads a native library; the `plugin` command does not exist
//! yet and its namespace is reserved.
//!
//! The 2.5.0 model (documented in `docs/architecture/plugins.md`) is
//! out-of-process plugins speaking JSON-RPC 2.0 over stdio, exposed only as
//! `ntc plugin <plugin-id> <command> [args...]`. That namespacing guarantees
//! zero collisions with the built-ins, and the subprocess boundary keeps a
//! plugin crash, hang or panic from taking down the shell.

// The seam is intentionally not loaded until 2.5.0: its types and the empty
// registry are exercised by unit tests now, and installation/trust will call
// them later. Allow dead code here so the forward seam does not add lints.
#![allow(dead_code)]

pub(crate) mod manifest;
pub(crate) mod registry;

use std::sync::OnceLock;

static PLUGIN_REGISTRY: OnceLock<registry::PluginRegistry> = OnceLock::new();

/// The process-wide plugin registry. Empty in 2.4.9 and never populated:
/// there is no loader. 2.5.0 adds installation/trust that fills it.
pub(crate) fn registry() -> &'static registry::PluginRegistry {
    PLUGIN_REGISTRY.get_or_init(registry::PluginRegistry::new)
}

#[cfg(test)]
mod tests {
    #[test]
    fn shipped_plugin_registry_is_empty() {
        assert!(super::registry().is_empty());
    }
}
