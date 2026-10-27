use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.fmt.json".into(),
        "fmt json <file> [to yaml|toml] ## แปลง JSON เป็น YAML หรือ TOML".into(),
    );

    m.insert(
        "help.fmt.output".into(),
        "(เพิ่ม -o <file> เพื่อบันทึกผลลัพธ์)".into(),
    );

    m.insert(
        "help.fmt.title".into(),
        "ตัวแปลงรูปแบบ (FORMAT CONVERTER):".into(),
    );

    m.insert(
        "help.fmt.toml".into(),
        "fmt toml <file> [to json|yaml] ## แปลง TOML เป็น JSON หรือ YAML".into(),
    );

    m.insert(
        "help.fmt.yaml".into(),
        "fmt yaml <file> [to json|toml] ## แปลง YAML เป็น JSON หรือ TOML".into(),
    );

    m.insert(
        "help.fmt.example1".into(),
        "fmt json config.json to yaml ## แปลง JSON เป็น YAML".into(),
    );

    m.insert(
        "help.fmt.example2".into(),
        "fmt toml Cargo.toml to json -o output.json ## แปลง TOML เป็น JSON".into(),
    );
}
