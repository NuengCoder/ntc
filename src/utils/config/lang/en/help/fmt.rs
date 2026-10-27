use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.fmt.json".into(),
        "fmt json <file> [to yaml|toml] ## Convert JSON to YAML or TOML".into(),
    );

    m.insert(
        "help.fmt.output".into(),
        "(add -o <file> to save output)".into(),
    );

    m.insert("help.fmt.title".into(), "FORMAT CONVERTER:".into());

    m.insert(
        "help.fmt.toml".into(),
        "fmt toml <file> [to json|yaml] ## Convert TOML to JSON or YAML".into(),
    );

    m.insert(
        "help.fmt.yaml".into(),
        "fmt yaml <file> [to json|toml] ## Convert YAML to JSON or TOML".into(),
    );

    m.insert(
        "help.fmt.example1".into(),
        "fmt json config.json to yaml ## Convert JSON to YAML".into(),
    );

    m.insert(
        "help.fmt.example2".into(),
        "fmt toml Cargo.toml to json -o output.json ## Convert TOML to JSON".into(),
    );
}
