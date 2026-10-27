use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "error.failed_write_completions".into(),
        "Failed to write completions: {}".into(),
    );

    m.insert(
        "error.unsupported_shell".into(),
        "Unsupported shell: {}. Use: bash, zsh, fish, powershell".into(),
    );
}
