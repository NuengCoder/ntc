use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "math.builtin_agg".into(),
        "  agg:  sum, min, max, avg".into(),
    );

    m.insert(
        "math.builtin_constants".into(),
        "  constants: PI, E, PHI, TAU".into(),
    );

    m.insert(
        "math.builtin_conv".into(),
        "  conv: toBinary, toHex, to8/toOctal, toDecimal, toHB".into(),
    );

    m.insert("math.builtin_header".into(), "内置函数：".into());

    m.insert(
        "math.builtin_inv_trig".into(),
        "  inv:  arcsin/asin, arccos/acos, arctan/atan, arccot/acot, arcsec/asec, arccsc/acsc"
            .into(),
    );

    m.insert(
        "math.builtin_math".into(),
        "  math: sqrt, pow(x,y), abs, floor, ceil, round, ln/log, log2, log10".into(),
    );

    m.insert(
        "math.builtin_print".into(),
        "  print: print values (like Python)".into(),
    );

    m.insert("math.builtin_rand".into(), "  rand: rand(min,max)".into());

    m.insert(
        "math.builtin_trig".into(),
        "  trig: sin, cos, tan, cot, sec, csc".into(),
    );

    m.insert("math.compiled_header".into(), "已编译：{}".into());

    m.insert("math.countdown".into(), "倒计时：{} 秒".into());

    m.insert("math.error".into(), "  {} 在第 {} 行：{}".into());

    m.insert(
        "math.example_add".into(),
        "  math 3+4*5                           → 23".into(),
    );

    m.insert(
        "math.example_fun_call".into(),
        "  math square(5)                        → 25".into(),
    );

    m.insert(
        "math.example_fun_def".into(),
        "  math fun add square(x) = x^2".into(),
    );

    m.insert(
        "math.example_hexbin".into(),
        "  math 0xFF + 0b1010                   → 265".into(),
    );

    m.insert(
        "math.example_rand".into(),
        "  math rand(1,100)                      → 42".into(),
    );

    m.insert(
        "math.example_sin".into(),
        "  math sin(PI/2)                       → 1".into(),
    );

    m.insert(
        "math.example_sqrt".into(),
        "  math sqrt(144)                        → 12".into(),
    );

    m.insert(
        "math.example_timer".into(),
        "  math timer 10                        → countdown 10s".into(),
    );

    m.insert(
        "math.example_tobinary".into(),
        "  math toBinary(255)                    → 11111111".into(),
    );

    m.insert(
        "math.example_tohb".into(),
        "  math toHB(1048576)                    → 1.00 MB".into(),
    );

    m.insert(
        "math.example_tohex".into(),
        "  math toHex(255)                       → FF".into(),
    );

    m.insert("math.examples_header".into(), "示例：".into());

    m.insert(
        "math.exec_end".into(),
        "\u{255A}\u{2550} 结束：{} \u{2550}\u{255D}".into(),
    );

    m.insert("math.failed_read".into(), "读取 '{}' 失败：{}".into());

    m.insert("math.file_not_found".into(), "未找到文件：'{}'".into());

    m.insert("math.fun_commands_header".into(), "数学函数命令：".into());

    m.insert("math.fun_list_header".into(), "用户定义的数学函数：".into());

    m.insert(
        "math.fun_none".into(),
        " 没有用户定义的函数。使用 'math fun add <名称>(<参数>) = <主体>'".into(),
    );

    m.insert("math.fun_removed".into(), " 已删除：{}".into());

    m.insert(
        "math.help_expr".into(),
        "  math <表达式>         计算数学表达式".into(),
    );

    m.insert(
        "math.help_file".into(),
        "  math <文件>.ntc.math  编译并运行数学文件".into(),
    );

    m.insert(
        "math.help_fun_add".into(),
        "  math fun add <名称>(<参数>) = <正文>   定义函数".into(),
    );

    m.insert(
        "math.help_fun_edit".into(),
        "  math fun edit <名称> = <正文>  更新函数".into(),
    );

    m.insert(
        "math.help_fun_info".into(),
        "  math fun info <名称>    显示函数详情".into(),
    );

    m.insert(
        "math.help_fun_ls".into(),
        "  math fun ls             列出所有自定义函数".into(),
    );

    m.insert(
        "math.help_fun_rm".into(),
        "  math fun rm <名称>      移除函数".into(),
    );

    m.insert(
        "math.help_func_call".into(),
        "  math <函数>(<参数>)     调用内置或自定义函数".into(),
    );

    m.insert(
        "math.help_timer".into(),
        "  math timer [秒]        秒表计时或倒计时+闹铃".into(),
    );

    m.insert("math.help_title".into(), "数学模块 - 用法：".into());

    m.insert("math.is_directory".into(), "'{}' 是目录，不是文件".into());

    m.insert("math.lap_time".into(), "圈速：{} 秒".into());

    m.insert("math.time_remaining".into(), "剩余时间：{}".into());

    m.insert("math.timer_started".into(), "计时器已启动...".into());

    m.insert("math.timer_usage".into(), "用法：math timer <秒>".into());

    m.insert("math.times_up".into(), "时间到！".into());
}
