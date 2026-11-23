//! Commented `ntconfig.toml` template shared by `local init`, `init local`,
//! and `init project`.

/// Commented template written by `local init` (and `init local`).
///
/// The local file is a thin layer on top of the global config: local entries
/// take priority, the global entries keep working for everything the local
/// file does not mention, and the global config is never modified.
pub(crate) const LOCAL_CONFIG_TEMPLATE: &str = r#"# ntc local configuration file
# Place this file in any directory to add project-specific ignore/care and
# run alias settings.
#
# INSTRUCTIONS:
# 1. Remove the '#' from lines you want to enable
# 2. Add your project-specific values
# 3. Run 'ntc' in this directory to activate
#
# PRECEDENCE:
# - Local entries win over the global ones with the same name
# - Global entries keep working for everything not listed here
# - The global config is never changed by this file
#
# Ignored directories (case-insensitive)
# ignored_directory_names = [
#     "target",
#     "node_modules",
#     ".git",
# ]

# Ignored file extensions
# ignored_extensions = [
#     "log",
#     "tmp",
#     "bak",
# ]

# Extra supported extensions (treat as text files)
# extra_supported_extensions = [
#     "myext",
#     "custom",
# ]

# Ignored specific files
# ignored_files = [
#     "Cargo.lock",
#     "package-lock.json",
# ]

# Extra supported specific files
# extra_supported_files = [
#     ".env",
#     "Dockerfile",
# ]

# Cared names are never ignored here, even when the global config ignores
# them (the global config itself stays untouched).
# cared_directory_names = [
#     "target",
# ]
#
# cared_extensions = [
#     "log",
# ]
#
# cared_files = [
#     "Cargo.lock",
# ]

# Run aliases for this project only
# [run_aliases]
# test = "cargo test"
# build = "cargo build --release"
# fb = "flutter clean && flutter pub get"
# py = "python $x.py"   # parameterised: call as py(filename)
"#;
