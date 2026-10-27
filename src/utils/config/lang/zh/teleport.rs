use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("teleport.added".into(), "保存点 '{}' 已创建 -> {}".into());

    m.insert("teleport.all_cleared".into(), "所有保存点已清除。".into());

    m.insert(
        "teleport.back_hint".into(),
        "使用 'tpb' 返回上一位置。".into(),
    );

    m.insert("teleport.back_opt".into(), "b. 传送返回".into());

    m.insert("teleport.cancel_opt".into(), "0. 取消".into());

    m.insert(
        "teleport.choose_prompt".into(),
        "输入数字传送到（1-{}）或 b/h/0：".into(),
    );

    m.insert("teleport.clear_cancelled".into(), "清除已取消。".into());

    m.insert(
        "teleport.clear_confirm".into(),
        "\u{26A0} 您确定要删除所有保存点吗？（y/N）：".into(),
    );

    m.insert(
        "teleport.clear_hint".into(),
        "使用 'tp clear' 清除历史。".into(),
    );

    m.insert("teleport.fs_failed".into(), "访问文件系统失败：{}".into());

    m.insert("teleport.history_cleared".into(), "传送历史已清除。".into());

    m.insert("teleport.history_opt".into(), "h. 显示历史".into());

    m.insert(
        "teleport.history_title".into(),
        "\u{1F4DC} 传送历史（最近的在最后）".into(),
    );

    m.insert(
        "teleport.invalid_char".into(),
        "名称中包含无效字符：'{}'".into(),
    );

    m.insert(
        "teleport.invalid_index".into(),
        "无效的索引：{}。使用 1-{}".into(),
    );

    m.insert(
        "teleport.invalid_input_interactive".into(),
        "无效的输入：{}。输入数字、'b' 或 'h'。".into(),
    );

    m.insert(
        "teleport.invalid_name".into(),
        "无效的名称：'{}' 被保留或为空".into(),
    );

    m.insert("teleport.jump_failed".into(), "无法传送：'{}'".into());

    m.insert(
        "teleport.jump_hint".into(),
        "使用 'tp jump <名称>' 传送。".into(),
    );

    m.insert("teleport.jumped".into(), "已传送到 '{}' -> {}".into());

    m.insert(
        "teleport.list_title".into(),
        "\u{1F4CC} 您的传送保存点（全局）".into(),
    );

    m.insert(
        "teleport.menu_title".into(),
        "\u{1F4CC} 传送 - 您的保存点（全局）".into(),
    );

    m.insert(
        "teleport.name_exists".into(),
        "无法重命名：'{}' 已作为保存点存在".into(),
    );

    m.insert(
        "teleport.name_taken".into(),
        "保存点 '{}' 已存在。使用 'tp edit' 更新。".into(),
    );

    m.insert(
        "teleport.name_taken_edit".into(),
        "保存点 '{}' 已存在".into(),
    );

    m.insert(
        "teleport.no_history_first".into(),
        "没有传送历史。先使用 'tp jump <名称>' 创建历史。".into(),
    );

    m.insert(
        "teleport.no_history_start".into(),
        "没有传送历史。使用 'tp jump <名称>' 开始建立历史。".into(),
    );

    m.insert(
        "teleport.no_savepoints_to_clear".into(),
        "没有要清除的保存点。".into(),
    );

    m.insert(
        "teleport.no_savepoints_yet".into(),
        "还没有保存点。使用 'tp add <名称>' 创建。".into(),
    );

    m.insert("teleport.not_found".into(), "未找到保存点：'{}'".into());

    m.insert("teleport.overwrite_cancelled".into(), "覆盖已取消。".into());

    m.insert(
        "teleport.overwrite_prompt".into(),
        "保存点 '{}' 已存在于不同位置。覆盖？（y/N）：".into(),
    );

    m.insert("teleport.prev_gone".into(), "上一位置不再存在：{}".into());

    m.insert("teleport.removed".into(), "已删除保存点：'{}'".into());

    m.insert(
        "teleport.removing_from_history".into(),
        "正在从历史中删除。使用 'tp history' 查看剩余条目。".into(),
    );

    m.insert(
        "teleport.rename_failed".into(),
        "重命名保存点失败：{}".into(),
    );

    m.insert("teleport.rename_same".into(), "新名称与旧名称相同".into());

    m.insert(
        "teleport.renamed".into(),
        "已重命名保存点 '{}' \u{2192} '{}'".into(),
    );

    m.insert(
        "teleport.renamed_with_path".into(),
        "已重命名保存点 '{}' -> '{}'（-> {}）".into(),
    );

    m.insert("teleport.teleport_cancelled".into(), "传送已取消。".into());

    m.insert("teleport.teleported_back".into(), "已传送回：{}".into());

    m.insert(
        "teleport.tpb_hint".into(),
        "使用 'tpb' 返回上一位置。".into(),
    );

    m.insert(
        "teleport.updated".into(),
        "保存点 '{}' 已更新：{} -> {}".into(),
    );
}
