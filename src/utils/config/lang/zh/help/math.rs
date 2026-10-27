use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.math.eval".into(),
        "math <表达式> ## 计算数学表达式".into(),
    );

    m.insert(
        "help.math.file".into(),
        "math file.ntc.math ## 编译并运行数学文件".into(),
    );

    m.insert(
        "help.math.fun".into(),
        "math fun add/rm/edit/info/ls ## 管理自定义数学函数".into(),
    );

    m.insert(
        "help.math.timer".into(),
        "math timer [秒] ## 计时器或带闹钟的倒计时".into(),
    );

    m.insert("help.math.title".into(), "数学命令：".into());

    m.insert(
        "help.math.example1".into(),
        "math 3+4*5 ## 计算算术表达式".into(),
    );

    m.insert(
        "help.math.example2".into(),
        "math sin(PI/2) ## 使用函数的数学计算".into(),
    );

    m.insert(
        "help.math.example3".into(),
        "math timer 30 ## 开始 30 秒倒计时".into(),
    );

    m.insert(
        "help.math.example4".into(),
        "math fun add myfn \"x+1\" ## 添加自定义函数".into(),
    );

    m.insert(
        "help.math.example5".into(),
        "math fun ls ## 列出自定义函数".into(),
    );

    m.insert(
        "help.math.example6".into(),
        "math calc.ntc.math ## 运行数学文件".into(),
    );
}
