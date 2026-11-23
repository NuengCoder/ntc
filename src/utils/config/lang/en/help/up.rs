use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.up.title".into(), "UP COMMAND:".into());

    m.insert(
        "help.up.usage".into(),
        "up <file> <url> ## Upload file (multipart POST; --put, --raw, -a, --bearer, -F, -H, -d)"
            .into(),
    );

    m.insert(
        "help.up.example1".into(),
        "up ./file.zip https://example.com/upload ## Upload a file".into(),
    );
}
