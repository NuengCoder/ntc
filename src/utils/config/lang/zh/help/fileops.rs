use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.fileops.cp".into(),
        "cp <源|@tp|通配符> <目标|@tp> [-f] ## 复制文件或目录（通配符源需目录目标）".into(),
    );

    m.insert(
        "help.fileops.mkd".into(),
        "mkd <目录>[, ...] ## 创建新目录".into(),
    );

    m.insert(
        "help.fileops.mkf".into(),
        "mkf <文件>[, ...] ## 创建新文件".into(),
    );

    m.insert(
        "help.fileops.rmd".into(),
        "rmd <目录>[, ...] ## 删除目录；通配符确认后删除所有匹配项".into(),
    );

    m.insert(
        "help.fileops.rmf".into(),
        "rmf <文件>[, ...] ## 删除文件（需确认）".into(),
    );

    m.insert(
        "help.fileops.rnmd".into(),
        "rnmd <旧名称> <新名称> ## 重命名目录".into(),
    );

    m.insert(
        "help.fileops.rnmf".into(),
        "rnmf <旧名称> <新名称> ## 重命名文件".into(),
    );

    m.insert(
        "help.fileops.mv".into(),
        "mv <源|@tp|通配符> <目标|@tp> [-f] ## 移动/重命名文件或目录（通配符源需目录目标）".into(),
    );

    m.insert("help.fileops.title".into(), "文件操作：".into());

    m.insert(
        "help.fileops.example1".into(),
        "mkf index.html ## 创建新文件".into(),
    );

    m.insert(
        "help.fileops.example2".into(),
        "mkd src/components ## 创建目录".into(),
    );

    m.insert(
        "help.fileops.example3".into(),
        "cp file.txt backup.txt ## 复制文件".into(),
    );
}
