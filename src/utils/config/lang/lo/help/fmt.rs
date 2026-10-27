use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.fmt.json".into(),
        "fmt json <file> [to yaml|toml] ## ແປງ JSON ເປັນ YAML ຫຼື TOML".into(),
    );

    m.insert(
        "help.fmt.output".into(),
        "(ເພີ່ມ -o <file> ເພື່ອບັນທຶກຜົນລັບ)".into(),
    );

    m.insert("help.fmt.title".into(), "FORMAT CONVERTER:".into());

    m.insert(
        "help.fmt.toml".into(),
        "fmt toml <file> [to json|yaml] ## ແປງ TOML ເປັນ JSON ຫຼື YAML".into(),
    );

    m.insert(
        "help.fmt.yaml".into(),
        "fmt yaml <file> [to json|toml] ## ແປງ YAML ເປັນ JSON ຫຼື TOML".into(),
    );

    m.insert(
        "help.fmt.example1".into(),
        "fmt json config.json to yaml ## ແປງ JSON ເປັນ YAML".into(),
    );

    m.insert(
        "help.fmt.example2".into(),
        "fmt toml Cargo.toml to json -o output.json ## ແປງ TOML ເປັນ JSON".into(),
    );
}
