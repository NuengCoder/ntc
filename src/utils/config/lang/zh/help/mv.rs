use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.mv.title".into(), "MV 命令：".into());

    m.insert(
        "help.mv.usage".into(),
        "mv <源|@tp|通配符> <目标|@tp> [-f] ## 移动/重命名文件或目录（通配符源需目录目标）".into(),
    );

    m.insert(
        "help.mv.example1".into(),
        "mv old.txt new.txt ## 重命名文件".into(),
    );
}
