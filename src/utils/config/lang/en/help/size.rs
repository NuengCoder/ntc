use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.size.care".into(),
        "size --care ## Calculate size including everything (care mode)".into(),
    );

    m.insert(
        "help.size.path".into(),
        "size -p, --path <path|@tp> ## Show size of a specific file, path or teleport savepoint"
            .into(),
    );

    m.insert(
        "help.size.size".into(),
        "size ## Show current directory size".into(),
    );

    m.insert(
        "help.size.time".into(),
        "size -t, --time ## Show elapsed time".into(),
    );

    m.insert("help.size.title".into(), "SIZE COMMAND:".into());

    m.insert(
        "help.size.example1".into(),
        "size ## Show current directory size".into(),
    );

    m.insert(
        "help.size.example2".into(),
        "size ../src ## Show size of another directory".into(),
    );

    m.insert(
        "help.size.example3".into(),
        "size Cargo.toml ## Show size of a file".into(),
    );

    m.insert(
        "help.size.example4".into(),
        "size @web --care -t ## Care mode with elapsed time for a savepoint".into(),
    );
}
