use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.editor.ne".into(),
        "ne, ntceditor <文件> ## 在内置文本编辑器中打开已有文件".into(),
    );

    m.insert(
        "help.editor.ne_hyper".into(),
        "ne --hyper <名称> --lang <l> [-I] ## 创建C/C++ hyper项目（-I 打开项目目录）".into(),
    );

    m.insert(
        "help.editor.ne_init".into(),
        "ne --init <文件> [<文件> ...] [-I <文件>] ## 创建模板文件；-I 打开其中一个".into(),
    );

    m.insert(
        "help.editor.ne_line".into(),
        "ne, ntceditor <文件>:<n> ## 在第n行打开文件".into(),
    );

    m.insert(
        "help.editor.ne_super".into(),
        "ne --super <文件> [<文件> ...] [-I <文件>] ## 创建超级模板文件；-I 打开其中一个".into(),
    );

    m.insert(
        "help.editor.ne_welcome".into(),
        "ne, ntceditor ## 在当前目录打开欢迎屏幕".into(),
    );

    m.insert("help.editor.title".into(), "编辑器命令：".into());

    m.insert(
        "help.editor.example1".into(),
        "ne main.rs ## 在编辑器中打开 main.rs".into(),
    );

    m.insert(
        "help.editor.example2".into(),
        "ne src/main.rs:42 ## 在 42 行打开 main.rs".into(),
    );

    m.insert(
        "help.editor.example3".into(),
        "ne --init a.py b.html -I a.py ## 创建模板并打开 a.py".into(),
    );

    m.insert(
        "help.editor.example4".into(),
        "ne --hyper myproject --lang c ## 创建 C 语言 hyper 项目".into(),
    );
}
