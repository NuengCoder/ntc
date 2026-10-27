use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.fmt.json".into(),
        "fmt json <文件> [to yaml|toml] ## 将JSON转换为YAML或TOML".into(),
    );

    m.insert(
        "help.fmt.output".into(),
        "（添加 -o <文件> 以保存输出）".into(),
    );

    m.insert("help.fmt.title".into(), "格式转换器：".into());

    m.insert(
        "help.fmt.toml".into(),
        "fmt toml <文件> [to json|yaml] ## 将TOML转换为JSON或YAML".into(),
    );

    m.insert(
        "help.fmt.yaml".into(),
        "fmt yaml <文件> [to json|toml] ## 将YAML转换为JSON或TOML".into(),
    );

    m.insert(
        "help.fmt.example1".into(),
        "fmt json config.json to yaml ## 将 JSON 转换为 YAML".into(),
    );

    m.insert(
        "help.fmt.example2".into(),
        "fmt toml Cargo.toml to json -o output.json ## 将 TOML 转换为 JSON".into(),
    );
}
