use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "scr.calling_not_allowed".into(),
        "脚本内不允许嵌套调用 scr".into(),
    );

    m.insert("scr.cmd_failed".into(), "命令执行失败".into());

    m.insert(
        "scr.completed".into(),
        "脚本 '{}' 成功完成（{} 条命令）".into(),
    );

    m.insert("scr.continue_warn".into(), "在第 {} 行出错后继续".into());

    m.insert("scr.desc_args".into(), " $1..$9, $@ 位置参数".into());

    m.insert(
        "scr.desc_force".into(),
        " @ <cmd> 强制系统命令（绕过 ntc 内建命令）".into(),
    );

    m.insert(
        "scr.desc_interp".into(),
        " $var / ${var} / ${var:-default} 变量插值".into(),
    );

    m.insert("scr.desc_set".into(), " set NAME = value 定义变量".into());

    m.insert(
        "scr.done_errors".into(),
        "脚本 '{}' 完成，共 {} 个错误".into(),
    );

    m.insert("scr.error_at".into(), "错误：{}:{} — {}".into());

    m.insert("scr.executing".into(), "[{}] 执行：{}".into());

    m.insert(
        "scr.failed_abort".into(),
        "脚本 '{}' 在第 {} 行中止（{} 个错误）".into(),
    );

    m.insert(
        "scr.help_continue".into(),
        " scr -c <file.ntc> [args...] 出错后继续".into(),
    );

    m.insert("scr.help_help".into(), " scr help 显示此帮助".into());

    m.insert(
        "scr.help_init".into(),
        " scr init [name] 创建脚本模板".into(),
    );

    m.insert(
        "scr.help_ls".into(),
        " scr ls 列出当前目录中的 *.ntc 脚本".into(),
    );

    m.insert(
        "scr.help_run".into(),
        " scr <file.ntc> [args...] 运行自动化脚本".into(),
    );

    m.insert("scr.help_title".into(), "scr — ntc 自动化脚本运行器".into());

    m.insert("scr.help_usage".into(), "用法：".into());

    m.insert("scr.init_created".into(), "已创建模板 {}".into());

    m.insert("scr.init_exists".into(), "{} 已存在".into());

    m.insert("scr.ls_empty".into(), "在 {} 中未找到 *.ntc 脚本".into());

    m.insert("scr.ls_title".into(), "自动化脚本：".into());

    m.insert("scr.no_commands".into(), "{} 中没有要运行的命令".into());

    m.insert("scr.not_found".into(), "找不到脚本：{}（{}）".into());

    m.insert("scr.start".into(), "正在运行脚本 {}...".into());

    m.insert("scr.syscmd".into(), "执行：{}".into());

    m.insert("scr.syscmd_exit".into(), "命令退出代码：{}".into());

    m.insert(
        "scr.syscmd_terminated".into(),
        "命令已终止（Ctrl+C）".into(),
    );

    // ── 脚本语言 ────────────────────────────────────────────────────────
    m.insert("scr.help_syntax_header".into(), "语法：".into());

    m.insert(
        "scr.help_direct".into(),
        " <file.ntc> [args...] 直接运行脚本（无需 scr 前缀）".into(),
    );

    m.insert(
        "scr.desc_expr".into(),
        " set NAME = <expr> 值、算术、函数调用".into(),
    );

    m.insert(
        "scr.desc_if".into(),
        " if <条件> / else if / else / end 条件分支".into(),
    );

    m.insert(
        "scr.desc_while".into(),
        " while <条件> / end 条件成立时循环".into(),
    );

    m.insert(
        "scr.desc_for".into(),
        " for 变量 in <列表> / end 遍历单词".into(),
    );

    m.insert(
        "scr.desc_loop".into(),
        " loop [次数] / end 重复指定次数（默认直到 break）".into(),
    );

    m.insert(
        "scr.desc_break".into(),
        " break / continue 跳出循环或跳到下一轮".into(),
    );

    m.insert(
        "scr.desc_func".into(),
        " func 名称(参数) / end 定义函数".into(),
    );

    m.insert(
        "scr.desc_call".into(),
        " call 名称(参数) 调用函数".into(),
    );

    m.insert(
        "scr.desc_return".into(),
        " return [值] 带返回值离开函数".into(),
    );

    m.insert(
        "scr.desc_input".into(),
        " input 变量 [提示] 读取一行到变量".into(),
    );

    m.insert("scr.desc_exit".into(), " exit 停止脚本".into());

    m.insert("scr.desc_functions_header".into(), "表达式函数：".into());

    m.insert(
        "scr.desc_fn_nprintf".into(),
        " nprintf(fmt, ...) 按 nprintf 内建命令格式化".into(),
    );

    m.insert(
        "scr.desc_fn_strings".into(),
        " len / upper / lower / trim / str 字符串函数".into(),
    );

    m.insert("scr.desc_fn_math".into(), " int / abs 数值函数".into());

    m.insert(
        "scr.desc_fn_env".into(),
        " env(名称) 读取环境变量".into(),
    );

    m.insert("scr.input_prompt".into(), "> ".into());

    m.insert(
        "scr.input_eof".into(),
        "没有为 '{}' 提供输入 —— 停止脚本".into(),
    );

    m.insert("scr.parse_error".into(), "{}:{} —— {}".into());

    m.insert(
        "scr.parse_failed".into(),
        "脚本 '{}' 有 {} 个语法错误，未执行".into(),
    );

    m.insert("scr.err_expr".into(), "无效表达式 '{}'：{}".into());

    m.insert("scr.err_loop_limit".into(), "循环超过 {} 次，已停止".into());

    m.insert("scr.err_func_missing".into(), "未知函数 '{}'".into());

    m.insert(
        "scr.err_func_args".into(),
        "'{}' 需要 {} 个参数，实际 {} 个".into(),
    );

    m.insert("scr.err_recursion".into(), "'{}' 递归超过 {} 层".into());

    m.insert(
        "scr.err_break_outside_loop".into(),
        "`break` / `continue` 不能跨出函数 '{}'".into(),
    );

    m.insert(
        "scr.nested_too_deep".into(),
        "脚本嵌套超过 {} 层，已停止".into(),
    );

    m.insert(
        "scr.pipe_refused".into(),
        "'{}' 设置了 `set-pipe = fail`，因此不能被管道或重定向".into(),
    );

    m.insert(
        "scr.desc_pipe".into(),
        " set-pipe = on|off 是否允许该脚本被管道或重定向".into(),
    );

    m.insert(
        "scr.desc_showexc".into(),
        " set-showexc = on|off 是否显示 [n] Executing: 提示".into(),
    );

}
