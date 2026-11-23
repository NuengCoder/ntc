use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.igcare.title".into(), "IGCARE 命令：".into());

    m.insert(
        "help.igcare.export_all".into(),
        "igcare export --all <n> ## 将所有设置导出到 <名称>.ntc.igcare".into(),
    );

    m.insert(
        "help.igcare.export_select".into(),
        "igcare export --select <n> ## 选择要导出的类别".into(),
    );

    m.insert(
        "help.igcare.import".into(),
        "igcare import <文件> ## 从 .ntc.igcare 文件导入设置".into(),
    );

    m.insert(
        "help.igcare.make".into(),
        "igcare make [名称] ## 在当前目录创建 igcare 模板".into(),
    );

    m.insert(
        "help.igcare.example1".into(),
        "igcare make ## 创建 igcare 模板".into(),
    );
}
