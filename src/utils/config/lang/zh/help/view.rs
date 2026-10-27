use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.view.file_usage".into(), "view <文件|@tp/文件|@tp/目录/文件> [--head N|--tail N|--betw A:B] ## 显示文件内容（同 txtf）".into());

    m.insert(
        "help.view.example10".into(),
        "view main.rs --tail 30 ## 显示 main.rs 最后 30 行".into(),
    );

    m.insert(
        "help.view.example11".into(),
        "view main.rs --betw 10:20 ## 显示 main.rs 第 10-20 行".into(),
    );

    m.insert(
        "help.view.example12".into(),
        "view @docs/readme.md ## 显示传送点中的文件".into(),
    );

    m.insert(
        "help.view.care".into(),
        "view -c, --care ## 显示所有文件的树（不忽略）".into(),
    );

    m.insert(
        "help.view.cp".into(),
        "view -cp, --copy ## 显示树并复制到剪贴板".into(),
    );

    m.insert(
        "help.view.file_only".into(),
        "view -f, --file ## 仅显示文件（移除目录）".into(),
    );

    m.insert(
        "help.view.dir_only".into(),
        "view -dir, --directory ## 仅显示目录（移除文件）".into(),
    );

    m.insert(
        "help.view.depth".into(),
        "view -d, --depth <n> ## 使用自定义深度显示树".into(),
    );

    m.insert(
        "help.view.dirsize".into(),
        "view -ds, --dirsize [最小值] ## 显示目录大小；带值时隐藏更小的目录".into(),
    );

    m.insert(
        "help.view.head".into(),
        "view --head <n> ## 仅显示前 n 行树".into(),
    );

    m.insert(
        "help.view.tail".into(),
        "view --tail <n> ## 仅显示后 n 行树".into(),
    );

    m.insert(
        "help.view.sort".into(),
        "view --sort size|char|ext ## 排序子项（目录仍在前）".into(),
    );

    m.insert(
        "help.view.ext".into(),
        "view -ext, --ext <扩展名...> ## 仅显示匹配扩展名的树".into(),
    );

    m.insert(
        "help.view.filesize".into(),
        "view -fs, --filesize [最小值] ## 显示文件大小；带值时隐藏更小的文件".into(),
    );

    m.insert(
        "help.view.path".into(),
        "view -p, --path <路径|@tp|@tp/子路径> ## 显示指定路径或传送点路径的树".into(),
    );

    m.insert(
        "help.view.size".into(),
        "view -s, --size [最小值] ## 显示大小；带值如 10KB、1.5MB 时隐藏更小的项".into(),
    );

    m.insert(
        "help.view.time".into(),
        "view --time, -t ## 显示含执行时间的树".into(),
    );

    m.insert("help.view.title".into(), "查看命令：".into());

    m.insert(
        "help.view.tree".into(),
        "view [<目录|文件|@tp|@tp/文件|@tp/目录/>] ## 显示目录树或文件内容".into(),
    );

    m.insert("help.view.example1".into(), "view ## 显示目录树".into());

    m.insert(
        "help.view.example2".into(),
        "view --size ## 显示带大小的目录树".into(),
    );

    m.insert(
        "help.view.example3".into(),
        "view -d 2 ## 显示深度为 2 的目录树".into(),
    );

    m.insert(
        "help.view.example4".into(),
        "view -cp ## 显示目录树并复制到剪贴板".into(),
    );

    m.insert(
        "help.view.example5".into(),
        "view --time ## 显示带执行时间的目录树".into(),
    );

    m.insert(
        "help.view.example6".into(),
        "view -p C:\\code\\project ## 显示指定路径的目录树".into(),
    );

    m.insert(
        "help.view.example7".into(),
        "view @web ## 显示传送点的目录树".into(),
    );

    m.insert(
        "help.view.example8".into(),
        "view -s 10MB ## 仅显示至少 10MB 的文件/目录".into(),
    );

    m.insert(
        "help.view.example9".into(),
        "view --sort size --head 20 ## 按大小排序并显示前 20 行".into(),
    );
}
