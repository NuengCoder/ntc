use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "tutorial.complete_msg".into(),
        "您现在已掌握NTC的基本功能：".into(),
    );

    m.insert("tutorial.complete_title".into(), " 教程完成！ ".into());

    m.insert("tutorial.demonstrating".into(), "正在演示：".into());

    m.insert("tutorial.exit_hint".into(), "输入 'exit' 退出NTC。".into());

    m.insert(
        "tutorial.help_hint".into(),
        "输入 'help' 查看完整命令参考。".into(),
    );

    m.insert("tutorial.prompt_continue".into(), "按Enter继续".into());

    m.insert("tutorial.prompt_exit_label".into(), "退出".into());

    m.insert("tutorial.prompt_skip_label".into(), "跳过".into());

    m.insert("tutorial.return_hint".into(), "按Enter返回Shell".into());

    m.insert(
        "tutorial.step_title".into(),
        " NTC 教程 \u{2014} 步骤 {}/{} ".into(),
    );

    m.insert("tutorial.tip_label".into(), "\u{1F4A1} 提示：".into());
}
