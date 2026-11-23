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

    m.insert("tutorial.tip_label".into(), "提示：".into());

    m.insert("tutorial.command_label".into(), "命令：".into());

    m.insert(
        "tutorial.command_usage".into(),
        "请自己输入后按 Enter \u{2014} ntc 不会替你运行。".into(),
    );

    m.insert(
        "tutorial.already_done".into(),
        "本课已完成。使用 `tutorial` 查看菜单，或使用 `tutorial restart` 重做。".into(),
    );
    m.insert("tutorial.also_try_fmt".into(), "也可以试试：{}".into());
    m.insert(
        "tutorial.blocked_cmd".into(),
        "`{}` 在教程模式中被阻止。请使用课程命令或 `tutorial`。".into(),
    );
    m.insert(
        "tutorial.blocked_outside".into(),
        "你在教程文件夹之外。使用 `cd {}` 返回，或使用 `tutorial exit` 退出教程模式。".into(),
    );
    m.insert("tutorial.goal_label".into(), "目标：".into());
    m.insert("tutorial.hint_label".into(), "提示：".into());
    m.insert(
        "tutorial.lesson_done".into(),
        "课程完成：{} — 使用 `tutorial` 选择下一课。".into(),
    );
    m.insert(
        "tutorial.menu_goal".into(),
        "输入一次 `tutorial <lesson>`，然后自己完成目标。".into(),
    );
    m.insert(
        "tutorial.menu_hint".into(),
        "卡住：tutorial hint · 跳过：tutorial skip · 状态：tutorial status · 离开：tutorial exit"
            .into(),
    );
    m.insert("tutorial.menu_title".into(), "NTC 教程 — 选择课程".into());
    m.insert(
        "tutorial.mode_off".into(),
        "教程模式已关闭。返回 {}。".into(),
    );
    m.insert(
        "tutorial.mode_on".into(),
        "教程模式已开启 — 命令由你输入；ntc 只检查目标。".into(),
    );
    m.insert(
        "tutorial.not_active".into(),
        "教程模式未启用。运行 `tutorial` 开始。".into(),
    );
    m.insert(
        "tutorial.not_in_lesson".into(),
        "尚未选择课程。使用 `tutorial <lesson>` 选择。".into(),
    );
    m.insert("tutorial.prompt_fmt".into(), "tutorial [{} {}/{}] ".into());
    m.insert("tutorial.prompt_none".into(), "tutorial ".into());
    m.insert("tutorial.skipped".into(), "已跳过。".into());
    m.insert("tutorial.status_fmt".into(), "课程 {} · 步骤 {}/{}".into());
    m.insert("tutorial.step_done".into(), "步骤完成。".into());
    m.insert("tutorial.step_label".into(), "步骤 {}/{}".into());
    m.insert(
        "tutorial.unknown_lesson".into(),
        "未知课程“{}”。可用课程：{}".into(),
    );
    m.insert("tutorial.usage".into(), "用法：\n  tutorial                 启动教程沙盒并显示课程菜单\n  tutorial <lesson>        开始课程（键名或编号）\n  tutorial list            显示课程菜单\n  tutorial hint            为当前步骤提供一条提示\n  tutorial skip            跳过当前步骤\n  tutorial restart         重新开始当前课程\n  tutorial status          查看当前课程和步骤\n  tutorial exit            退出教程模式\n\n课程：运行 `tutorial list` 查看完整菜单（每个 ntc 命令都有课程）。\n教程模式下只运行实验命令；离开教程文件夹后\n除 cd、tutorial、help 和 exit 外的命令都会被阻止。".into());
    m.insert("tutorial.ws_ready".into(), "教程工作区：{}".into());
}
