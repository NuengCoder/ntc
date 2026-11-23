use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.qur.title".into(), "QUR 命令：".into());

    m.insert(
        "help.qur.usage".into(),
        "qur <glob> [-s \"pat\"] [-rp \"new\"] [-dp] [-d <n>] ## 查询文件、选择内容、替换".into(),
    );

    m.insert(
        "help.qur.example1".into(),
        "qur *.rs -s \"TODO\" -dp ## query glob + 类似 grep 的显示".into(),
    );
}
