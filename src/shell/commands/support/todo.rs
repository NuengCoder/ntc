//! Shared todo filename extensions.

/// Extension used for a todo format (`yaml` stays `.yaml`).
pub(crate) fn todo_extension(format: &str) -> &str {
    match format {
        "md" => "md",
        "json" => "json",
        "toml" => "toml",
        "yaml" => "yaml",
        "html" => "html",
        _ => "txt",
    }
}
