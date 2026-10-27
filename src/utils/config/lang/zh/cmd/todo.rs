use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.todo.add_usage".into(),
        "用法：todo add \"任务名称\" [-> N]".into(),
    );

    m.insert("cmd.todo.all_cleared".into(), "所有任务已清除。".into());

    m.insert("cmd.todo.cancelled".into(), "已取消。".into());

    m.insert(
        "cmd.todo.clear_prompt".into(),
        "清除所有 {} 个任务？[y/N] ".into(),
    );

    m.insert("cmd.todo.empty_name".into(), "任务名称不能为空".into());

    m.insert(
        "cmd.todo.help_add".into(),
        "  todo add \"名称\" [-> N]  添加任务（可选在第 N 个位置）".into(),
    );

    m.insert(
        "cmd.todo.help_clear".into(),
        "  todo clear              清除所有任务（需确认）".into(),
    );

    m.insert(
        "cmd.todo.help_info".into(),
        "  todo info [N]           显示所有任务或第 N 个任务".into(),
    );

    m.insert(
        "cmd.todo.help_rm".into(),
        "  todo rm <N>             移除第 N 个任务".into(),
    );

    m.insert(
        "cmd.todo.help_show".into(),
        "  todo                    显示所有任务".into(),
    );

    m.insert(
        "cmd.todo.info_usage".into(),
        "用法：todo info <N> - N 必须是数字".into(),
    );

    m.insert(
        "cmd.todo.invalid_position".into(),
        "无效的位置：'{}' 必须是数字".into(),
    );

    m.insert("cmd.todo.list_title".into(), "待办列表：".into());

    m.insert(
        "cmd.todo.missing_position".into(),
        "-> 后缺少位置数字".into(),
    );

    m.insert("cmd.todo.missing_quote".into(), "缺少结束引号".into());

    m.insert(
        "cmd.todo.must_be_quoted".into(),
        "任务名称必须用引号括起来".into(),
    );

    m.insert("cmd.todo.no_tasks".into(), "  {}".into());

    m.insert("cmd.todo.none_to_clear".into(), "没有要清除的任务。".into());

    m.insert(
        "cmd.todo.one_based".into(),
        "任务编号从 1 开始（最小 1）".into(),
    );

    m.insert(
        "cmd.todo.position_one_based".into(),
        "位置必须是 1 起始（最小为 1）".into(),
    );

    m.insert("cmd.todo.rm_usage".into(), "用法：todo rm <N>".into());

    m.insert(
        "cmd.todo.rm_usage_num".into(),
        "用法：todo rm <N> - N 必须是数字".into(),
    );

    m.insert("cmd.todo.task_added".into(), "已添加任务：{}".into());

    m.insert(
        "cmd.todo.task_not_found".into(),
        "未找到第 #{} 个任务（共有 {} 个任务）".into(),
    );

    m.insert("cmd.todo.task_removed".into(), "已移除任务：{}".into());

    m.insert(
        "cmd.todo.unexpected_text".into(),
        "任务名称后出现意外文本：'{}'".into(),
    );

    m.insert(
        "cmd.todo.unknown_sub".into(),
        "未知的 todo 子命令：'{}'".into(),
    );

    m.insert("cmd.todo.usage_title".into(), "用法：".into());

    m.insert(
        "cmd.todo.help_done".into(),
        "  todo done <N>           标记任务 N 为已完成".into(),
    );

    m.insert(
        "cmd.todo.help_undo".into(),
        "  todo undo <N>           重新打开任务 N".into(),
    );

    m.insert(
        "cmd.todo.help_edit".into(),
        "  todo edit <N> \"名称\"    重命名任务 N".into(),
    );

    m.insert("cmd.todo.done_usage".into(), "用法：todo done <N>".into());

    m.insert("cmd.todo.undo_usage".into(), "用法：todo undo <N>".into());

    m.insert(
        "cmd.todo.edit_usage".into(),
        "用法：todo edit <N> \"新名称\"".into(),
    );

    m.insert("cmd.todo.task_done".into(), "任务已完成：{}".into());

    m.insert("cmd.todo.task_undone".into(), "任务已重新打开：{}".into());

    m.insert("cmd.todo.task_edited".into(), "任务已更新：{}".into());

    m.insert("cmd.todo.already_done".into(), "任务 #{} 已经完成".into());

    m.insert(
        "cmd.todo.already_open".into(),
        "任务 #{} 已经是未完成状态".into(),
    );
}
