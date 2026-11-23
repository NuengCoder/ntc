use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.cp.title".into(), "CP 命令：".into());

    m.insert(
        "help.cp.usage".into(),
        "cp <源|@tp|通配符> <目标|@tp> [-f] ## 复制文件或目录（通配符源需目录目标）".into(),
    );

    m.insert(
        "help.cp.example1".into(),
        "cp file.txt backup.txt ## 复制文件".into(),
    );
}
