use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.tl.dir_not_found".into(), "目录未找到：{}".into());

    m.insert("cmd.tl.dir_usage".into(), "用法：tl dir <路径>".into());

    m.insert(
        "cmd.tl.ext_not_supported".into(),
        ".{} 不受支持 - 已阻止".into(),
    );

    m.insert("cmd.tl.ext_usage".into(), "用法：tl ext <扩展名>".into());

    m.insert("cmd.tl.file_not_found".into(), "文件未找到：{}".into());

    m.insert("cmd.tl.file_usage".into(), "用法：tl file <路径>".into());

    m.insert(
        "cmd.tl.help_dir".into(),
        "  tl dir <路径>          显示目录中受支持扩展名和总行数".into(),
    );

    m.insert(
        "cmd.tl.help_ext".into(),
        "  tl ext <扩展名>        显示所有带该扩展名的文件及总行数".into(),
    );

    m.insert(
        "cmd.tl.help_file".into(),
        "  tl file <路径>         统计单个文件的行数".into(),
    );

    m.insert(
        "cmd.tl.help_sort".into(),
        "  tl ext|dir <目标> -s <-n|-l|-r>   排序输出（-n 普通、-l 按行数升序、-r 降序）".into(),
    );

    m.insert("cmd.tl.highest".into(), "最高：{}  {} 行".into());

    m.insert("cmd.tl.no_ext_files".into(), "未找到 .{} 文件".into());

    m.insert(
        "cmd.tl.no_supported_files".into(),
        "在 {} 中未找到受支持的文件".into(),
    );

    m.insert("cmd.tl.not_a_file".into(), "不是文件：{}".into());

    m.insert("cmd.tl.sort_usage".into(), "排序参数无效。请使用：-s -n（普通）| -s -l（按行数升序，从小到大）| -s -r（降序，从大到小）。".into());

    m.insert(
        "cmd.tl.total_files_lines".into(),
        "总计：{} 个文件，{} 行".into(),
    );

    m.insert("cmd.tl.unknown_sub".into(), "未知子命令 '{}'。".into());

    m.insert("cmd.tl.usage_title".into(), "用法：".into());
}
