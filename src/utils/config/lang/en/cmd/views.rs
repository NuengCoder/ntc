use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.views.usage".into(),
        "Usage: views [path] [-n] [--size [min-size]] [--sort <name|ext|size>]".into(),
    );

    m.insert(
        "cmd.views.invalid_size".into(),
        "Invalid size: {} (use 1024, 10KB, 5MB, ...)".into(),
    );

    m.insert(
        "cmd.views.invalid_sort".into(),
        "Missing value for --sort".into(),
    );

    m.insert(
        "cmd.views.invalid_sort_value".into(),
        "Unknown sort key: {} (use name, ext or size)".into(),
    );

    m.insert("cmd.views.empty".into(), "(empty directory)".into());
}
