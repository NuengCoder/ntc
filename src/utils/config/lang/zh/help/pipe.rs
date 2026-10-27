use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.pipe.title".into(), "管道与重定向：".into());

    m.insert(
        "help.pipe.pipe".into(),
        "a | b ## 将 a 的输出作为 b 的输入".into(),
    );

    m.insert(
        "help.pipe.and".into(),
        "a && b ## 仅当 a 成功时运行 b".into(),
    );

    m.insert(
        "help.pipe.or".into(),
        "a || b ## 仅当 a 失败时运行 b".into(),
    );

    m.insert(
        "help.pipe.out".into(),
        "cmd > file ## 将输出写入文件（覆盖）".into(),
    );

    m.insert(
        "help.pipe.append".into(),
        "cmd >> file ## 将输出追加到文件".into(),
    );

    m.insert("help.pipe.in".into(), "cmd < file ## 从文件读取输入".into());

    m.insert(
        "help.pipe.heredoc".into(),
        "cmd <<EOF ... EOF ## 读取输入直到 EOF 行".into(),
    );

    m.insert(
        "help.pipe.herestring".into(),
        "cmd <<< text ## 从字面文本读取输入".into(),
    );

    m.insert(
        "help.pipe.head".into(),
        "head [N] [file] ## 打印前 N 行（默认 10）".into(),
    );

    m.insert(
        "help.pipe.tail".into(),
        "tail [N] [file] ## 打印后 N 行（默认 10）".into(),
    );

    m.insert(
        "help.pipe.betw".into(),
        "betw A:B [file] ## 打印第 A 到 B 行（从 1 开始，含两端）".into(),
    );

    m.insert(
        "help.pipe.sort".into(),
        "sort [-r] [-n] [-u] [-f] [file] ## 排序行（倒序/数值/去重/忽略大小写）".into(),
    );

    m.insert(
        "help.pipe.rev".into(),
        "rev [file] ## 反转每行的字符".into(),
    );

    m.insert(
        "help.pipe.trim".into(),
        "trim <pattern> [文本] ## 删除 <pattern> 的每一次出现（文本或管道输入）".into(),
    );

    m.insert("help.pipe.filters".into(), "cmd | head|tail|betw|gs|sort|rev|count|trim|encode|qur|clip ## 管道过滤器（其标志也可补全）".into());

    m.insert(
        "help.pipe.note".into(),
        "运算符两侧需要空格；使用 - 读取 stdin".into(),
    );

    m.insert(
        "help.pipe.hint".into(),
        "提示：| 列出管道过滤器；>> 和 < 仅列出文本文件；> 不提示；<< 只提示 EOF".into(),
    );

    m.insert(
        "help.pipe.example1".into(),
        "gs main > results.txt ## 将搜索结果保存到文件".into(),
    );

    m.insert(
        "help.pipe.example2".into(),
        "view readme.md | encode b64 ## 将文件内容编码".into(),
    );

    m.insert(
        "help.pipe.example3".into(),
        "gs main | say ## 通过 say 分页查看搜索结果".into(),
    );

    m.insert(
        "help.pipe.example4".into(),
        "head 20 app.log ## 文件（或管道输入）的前 20 行".into(),
    );

    m.insert(
        "help.pipe.example5".into(),
        "gs error | sort -u | head 10 ## 排序去重后的匹配，取前 10 行".into(),
    );

    m.insert("help.pipe.example6".into(), "echo nice ... bro ... | trim \" ...\" ## 从管道文本中删除模式（内联写法：trim \" ...\" \"nice ... bro ...\"）".into());
}
