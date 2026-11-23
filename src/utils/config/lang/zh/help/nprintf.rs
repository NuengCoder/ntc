use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.nprintf.title".into(), r#"NPRINTF 命令："#.into());
    m.insert(
        "help.nprintf.usage".into(),
        r#"nprintf <格式> [参数...] ## Bash 风格格式化输出"#.into(),
    );
    m.insert("help.nprintf.conversions".into(), r#"  转换符"#.into());
    m.insert(
        "help.nprintf.specs".into(),
        r#"  %s 文本 · %d/%i 有符号整数 · %u 无符号 · %o 八进制 · %x/%X 十六进制 · %c 首字符"#
            .into(),
    );
    m.insert(
        "help.nprintf.specs2".into(),
        r#"  %b 参数内转义 · %q shell 引号 · %f/%e/%g/%a 浮点数 · %% 百分号本身"#.into(),
    );
    m.insert("help.nprintf.flags".into(), r#"  标志、宽度、精度"#.into());
    m.insert(
        "help.nprintf.flags_detail".into(),
        r#"  %-8s 左对齐 · %05d 补零 · %+d 显示符号 · %.2f 精度 · %*s 宽度取自参数"#.into(),
    );
    m.insert(
        "help.nprintf.escapes".into(),
        r#"  转义（每个都以反斜杠开始）"#.into(),
    );
    m.insert("help.nprintf.escapes_detail".into(), r#"  \n 换行 · \t 制表 · \r 回车 · \ 反斜杠 · \" 引号 · \xHH 十六进制 · \0NNN 八进制 · \c 停止输出"#.into());
    m.insert("help.nprintf.newline".into(), r#"  为什么需要 \n？"#.into());
    m.insert("help.nprintf.newline_detail".into(), r#"  nprintf 不会在格式后自动添加任何字符。需要换行时在结尾写 \n；不写就会继续在同一行输出。"#.into());
    m.insert("help.nprintf.quoting".into(), r#"  引号"#.into());
    m.insert(
        "help.nprintf.quoting_detail".into(),
        r#"  用双引号包住格式，空格和转义才会留在同一个参数里。"#.into(),
    );
    m.insert(
        "help.nprintf.reuse".into(),
        r#"  参数未用完时格式会重复；缺少 %s 输出空，缺少数字输出 0。"#.into(),
    );
    m.insert(
        "help.nprintf.example1".into(),
        r#"nprintf "hello %s\n" world ## 输出一行；\n 结束该行"#.into(),
    );
    m.insert(
        "help.nprintf.example2".into(),
        r#"nprintf "%s has %d items\n" cart 3 ## 文本和整数"#.into(),
    );
    m.insert(
        "help.nprintf.example3".into(),
        r#"nprintf "%-8s|%05d|%.2f\n" left 42 3.14159 ## 左对齐、补零、精度"#.into(),
    );
    m.insert(
        "help.nprintf.example4".into(),
        r#"nprintf "%b\n" "a\tb" ## %b 会解释参数里的转义"#.into(),
    );
    m.insert(
        "help.nprintf.example5".into(),
        r#"nprintf "no newline" ## 留在同一行"#.into(),
    );
}
