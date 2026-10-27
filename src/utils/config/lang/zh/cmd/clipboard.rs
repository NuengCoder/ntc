use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.clipboard.access_fail".into(),
        "访问剪贴板失败：{}".into(),
    );

    m.insert(
        "cmd.clipboard.already_empty".into(),
        "剪贴板历史已为空。".into(),
    );

    m.insert(
        "cmd.clipboard.clear_cancelled".into(),
        "清除已取消。".into(),
    );

    m.insert("cmd.clipboard.clear_prompt".into(), "清除已取消。".into());

    m.insert("cmd.clipboard.cleared".into(), "剪贴板历史已清除。".into());

    m.insert(
        "cmd.clipboard.copied_entry".into(),
        "已将第 #{} 条（{}）复制到剪贴板。".into(),
    );

    m.insert("cmd.clipboard.empty".into(), "剪贴板历史为空。".into());

    m.insert(
        "cmd.clipboard.entry_not_found".into(),
        "未找到第 #{} 条（共有 {} 条）。".into(),
    );

    m.insert(
        "cmd.clipboard.help_clear".into(),
        "  clip clear         清除剪贴板历史".into(),
    );

    m.insert(
        "cmd.clipboard.help_list".into(),
        "  clip               显示剪贴板历史".into(),
    );

    m.insert(
        "cmd.clipboard.help_select".into(),
        "  clip select <N>    将第 N 条复制回剪贴板".into(),
    );

    m.insert(
        "cmd.clipboard.history_header".into(),
        "  剪贴板历史（最近 {} 条）：".into(),
    );

    m.insert(
        "cmd.clipboard.select_usage".into(),
        "用法：clip select <N>".into(),
    );

    m.insert("cmd.clipboard.set_fail".into(), "设置剪贴板失败：{}".into());

    m.insert(
        "cmd.clipboard.termux_unavailable".into(),
        "termux-clipboard-set 不可用：{}".into(),
    );

    m.insert(
        "cmd.clipboard.unknown_sub".into(),
        "未知的 clip 子命令：{}".into(),
    );
}
