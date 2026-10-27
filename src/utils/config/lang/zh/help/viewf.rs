use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.viewf.title".into(), "VIEWF 命令（仅文件）：".into());

    m.insert(
        "help.viewf.tree".into(),
        "viewf [<目录|@tp|@tp/目录/>] ## 显示深度 1 的仅文件树".into(),
    );

    m.insert("help.viewf.file_usage".into(), "viewf <文件|@tp/文件|@tp/目录/文件> [--head N|--tail N|--betw A:B] ## 显示文件内容（同 txtf）".into());

    m.insert(
        "help.viewf.size".into(),
        "viewf -s, --size [最小值] ## 显示文件大小；带值时隐藏更小的文件".into(),
    );

    m.insert(
        "help.viewf.sort".into(),
        "viewf --sort size|char|ext ## 排序文件项".into(),
    );

    m.insert(
        "help.viewf.ext".into(),
        "viewf -e, --ext <扩展名...> ## 仅显示匹配扩展名的文件".into(),
    );

    m.insert(
        "help.viewf.care".into(),
        "viewf -c, --care ## 计算大小时包含所有项".into(),
    );

    m.insert(
        "help.viewf.time".into(),
        "viewf -t, --time ## 显示执行时间".into(),
    );

    m.insert(
        "help.viewf.copy".into(),
        "viewf -cp, --copy ## 复制文件列表到剪贴板".into(),
    );

    m.insert(
        "help.viewf.example1".into(),
        "viewf ## 列出当前目录中的文件".into(),
    );

    m.insert(
        "help.viewf.example2".into(),
        "viewf src ## 列出 src 目录中的文件".into(),
    );

    m.insert(
        "help.viewf.example3".into(),
        "viewf -e rs,txt --sort size ## 按扩展名筛选并按大小排序".into(),
    );

    m.insert(
        "help.viewf.example4".into(),
        "viewf @docs/readme.md --head 20 ## 显示传送点文件".into(),
    );
}
