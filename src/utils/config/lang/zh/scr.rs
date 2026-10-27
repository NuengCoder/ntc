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
}
