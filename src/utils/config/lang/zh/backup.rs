use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("backup.and_more".into(), " ... 还有 {} 个更多".into());

    m.insert(
        "backup.available_title".into(),
        " 可用备份（最新优先）".into(),
    );

    m.insert("backup.cancel_opt".into(), " 0. 取消".into());

    m.insert("backup.cancelled".into(), "清除已取消。".into());

    m.insert(
        "backup.clear_history_prompt".into(),
        "\u{26A0} 清除撤销历史？此操作无法撤销。（y/N）：".into(),
    );

    m.insert("backup.cleared".into(), "所有备份已删除。".into());

    m.insert(
        "backup.cleared_count".into(),
        "已为此项目清除 {} 个备份。".into(),
    );

    m.insert("backup.confirm".into(), "输入 'yes' 确认：".into());

    m.insert(
        "backup.create_hint".into(),
        "使用 'backup create' 创建您的第一个备份。".into(),
    );

    m.insert(
        "backup.created".into(),
        "备份已创建：#{} \u{2014} {} \u{2014} {} 个文件".into(),
    );

    m.insert(
        "backup.created_detail".into(),
        "备份 #{} 已创建：{}（{} 个文件，{}）".into(),
    );

    m.insert("backup.creating".into(), "正在创建备份...".into());

    m.insert(
        "backup.delete_extra_failed".into(),
        "删除额外文件 {} 失败：{}".into(),
    );

    m.insert(
        "backup.deleted_since".into(),
        "{} 自备份以来已被删除。".into(),
    );

    m.insert(
        "backup.entry_file_count".into(),
        " 备份 #{} \u{2014} {} \u{2014} {} \u{2014} {} 个文件".into(),
    );

    m.insert("backup.exists".into(), "备份存在：{} 个备份在 {}".into());

    m.insert("backup.failed_to_backup".into(), "备份 {} 失败：{}".into());

    m.insert("backup.file_deleted".into(), "{} 在备份后被删除。".into());

    m.insert(
        "backup.file_modified".into(),
        "{} 自上次保存以来已更改。".into(),
    );

    m.insert(
        "backup.help_clear".into(),
        " backup clear 删除所有备份（需确认）".into(),
    );

    m.insert(
        "backup.help_clear_force".into(),
        " backup clear --force 删除所有备份（无需确认）".into(),
    );

    m.insert(
        "backup.help_create".into(),
        " backup create 创建新备份".into(),
    );

    m.insert(
        "backup.help_diff".into(),
        " backup diff [num] 比较当前项目与备份的差异".into(),
    );

    m.insert("backup.help_examples".into(), "示例：".into());

    m.insert(
        "backup.help_list".into(),
        " backup list 列出所有备份".into(),
    );

    m.insert(
        "backup.help_recreate".into(),
        " backup recreate 清除所有 + 撤销历史 + 创建全新".into(),
    );

    m.insert(
        "backup.help_recreate_force".into(),
        " backup recreate --force 同上，跳过所有确认".into(),
    );

    m.insert(
        "backup.help_restore".into(),
        " backup restore [num] 恢复备份（无编号则交互选择）".into(),
    );

    m.insert(
        "backup.help_restore_choose".into(),
        " backup restore --choose 从交互菜单选择备份".into(),
    );

    m.insert(
        "backup.help_restore_choose_deleted".into(),
        " backup restore --choose --deleted 选择 + 恢复删除".into(),
    );

    m.insert(
        "backup.help_restore_deleted".into(),
        " backup restore --deleted 仅恢复自备份以来已删除的文件".into(),
    );

    m.insert(
        "backup.help_title".into(),
        "backup \u{2014} 管理项目备份".into(),
    );

    m.insert(
        "backup.help_undo".into(),
        " backup undo 撤销上次恢复".into(),
    );

    m.insert(
        "backup.help_undo_cls".into(),
        " backup undo --cls 清除撤销历史".into(),
    );

    m.insert(
        "backup.help_undo_force".into(),
        " backup undo --force 清除撤销历史（无需确认）".into(),
    );

    m.insert("backup.invalid_arg".into(), "无效的参数：{}".into());

    m.insert(
        "backup.invalid_diff_arg".into(),
        "无效的参数：{}。用法：backup diff <编号>".into(),
    );

    m.insert("backup.invalid_selection".into(), "无效的选择：{}".into());

    m.insert(
        "backup.list_entry".into(),
        " #{} \u{2014} {} \u{2014} {} \u{2014} {} 个文件".into(),
    );

    m.insert("backup.list_location".into(), "位置：{}".into());

    m.insert(
        "backup.list_title".into(),
        "此项目的备份：找到 {} 个".into(),
    );

    m.insert("backup.menu_item".into(), "备份 #{}".into());

    m.insert(
        "backup.modified_since".into(),
        "{} 自备份以来已被修改。".into(),
    );

    m.insert(
        "backup.no_deleted_files".into(),
        "没有要恢复的已删除文件 \u{2014} 项目已与备份快照匹配。".into(),
    );

    m.insert("backup.none".into(), "未找到此项目的备份。".into());

    m.insert("backup.not_found_num".into(), "备份 #{} 未找到。".into());

    m.insert(
        "backup.pick_detail".into(),
        "{} \u{2014} {} \u{2014} {} 个文件".into(),
    );

    m.insert("backup.pick_prompt".into(), "输入数字（1-{}）或 0：".into());

    m.insert("backup.pick_title".into(), "选择要恢复的备份".into());

    m.insert(
        "backup.previous_state_saved".into(),
        "已保存先前状态。使用 'unpd' 撤销此恢复。".into(),
    );

    m.insert("backup.recreate_proceed".into(), "继续？（y/N）：".into());

    m.insert(
        "backup.recreate_prompt".into(),
        "未找到现有备份。仍然创建全新备份？（y/N）：".into(),
    );

    m.insert(
        "backup.recreate_step1".into(),
        " 1. 删除所有现有备份".into(),
    );

    m.insert("backup.recreate_step2".into(), " 2. 清除撤销历史".into());

    m.insert(
        "backup.recreate_step3".into(),
        " 3. 创建当前状态的全新备份".into(),
    );

    m.insert(
        "backup.recreate_will_title".into(),
        "\u{26A0}\u{FE0F} 重建将：".into(),
    );

    m.insert(
        "backup.recreated".into(),
        "项目已重建：旧备份已清除，新备份已创建。".into(),
    );

    m.insert(
        "backup.restore_add_count".into(),
        "{} 个文件将被恢复。".into(),
    );

    m.insert("backup.restore_cancelled".into(), "恢复已取消。".into());

    m.insert("backup.restore_cancelled_2".into(), "恢复已取消。".into());

    m.insert(
        "backup.restore_confirm".into(),
        "继续恢复？（y/N）：".into(),
    );

    m.insert(
        "backup.restore_create_title".into(),
        "\u{26A0} 以下文件将被创建（{}）：".into(),
    );

    m.insert(
        "backup.restore_delete_title".into(),
        "\u{26A0} 以下文件将被删除（不在备份中）（{}）：".into(),
    );

    m.insert(
        "backup.restore_deleted_confirm".into(),
        "继续恢复已删除的文件？（y/N）：".into(),
    );

    m.insert(
        "backup.restore_deleted_title".into(),
        " 将恢复 {} 个文件（备份中存在但现已消失）：".into(),
    );

    m.insert("backup.restore_failed".into(), "恢复 {} 失败：{}".into());

    m.insert(
        "backup.restore_overwrite_count".into(),
        "{} 个文件将被覆盖。".into(),
    );

    m.insert(
        "backup.restore_overwrite_title".into(),
        "\u{26A0} 以下文件将被覆盖（{}）：".into(),
    );

    m.insert("backup.restore_preview_title".into(), "恢复预览".into());

    m.insert(
        "backup.restored_backup".into(),
        "恢复完成：备份 #{} \u{2014} 已恢复 {} 个文件".into(),
    );

    m.insert("backup.restored_date".into(), "恢复执行于：{}".into());

    m.insert(
        "backup.restored_deleted".into(),
        "已从备份 #{} 恢复已删除的文件：{} 个文件已恢复".into(),
    );

    m.insert(
        "backup.restored_result".into(),
        "已恢复备份 #{}（{}）".into(),
    );

    m.insert(
        "backup.select_entry".into(),
        " {}. 备份 #{} \u{2014} {} \u{2014} {} \u{2014} {} 个文件".into(),
    );

    m.insert(
        "backup.select_prompt".into(),
        "选择要恢复的备份（1-{}）：".into(),
    );

    m.insert(
        "backup.skip_large".into(),
        "已跳过 {}（{} MB 超过 50 MB 限制）".into(),
    );

    m.insert(
        "backup.skip_metadata".into(),
        "跳过 {}（元数据错误：{}）".into(),
    );

    m.insert(
        "backup.skip_rel_path".into(),
        "跳过 {}（无法解析相对路径）".into(),
    );

    m.insert(
        "backup.skip_symlink".into(),
        "跳过 {}（符号链接 \u{2014} 不跟随）".into(),
    );

    m.insert(
        "backup.skipped_count".into(),
        "已跳过 {} 个文件 \u{2014} 查看 ignore.txt 了解详情".into(),
    );

    m.insert("backup.skipped_error".into(), "已跳过 {}（{}）".into());

    m.insert(
        "backup.skipped_large".into(),
        "已跳过 {}（{} \u{2014} 太大）".into(),
    );

    m.insert(
        "backup.skipped_unsupported".into(),
        "已跳过 {}（不支持的文件类型）".into(),
    );

    m.insert(
        "backup.snapshot_failed".into(),
        "无法为 {} 创建快照以撤销（{}），继续".into(),
    );

    m.insert(
        "backup.some_restore_failed".into(),
        "某些文件无法恢复。撤销存储已保留。".into(),
    );

    m.insert("backup.stored_in".into(), "备份存储在：{}".into());

    m.insert(
        "backup.undo_cleared".into(),
        "撤销历史已清除（释放了 {}）".into(),
    );

    m.insert(
        "backup.undo_complete".into(),
        "撤销完成：已从上一次恢复前恢复了 {} 个文件".into(),
    );

    m.insert(
        "backup.undo_complete_detail".into(),
        "已撤销备份 #{} 的恢复（{} 已恢复，{} 已删除，{} 已重新恢复）".into(),
    );

    m.insert(
        "backup.undo_delete_failed".into(),
        "无法删除已恢复的文件 {}：{}".into(),
    );

    m.insert(
        "backup.undo_missing_original".into(),
        "无法恢复 {}（撤销存储中缺少原始文件）".into(),
    );

    m.insert(
        "backup.undo_missing_snapshot".into(),
        "无法重新恢复 {}（撤销存储中缺少快照）".into(),
    );

    m.insert("backup.undo_no_history".into(), "未找到撤销历史。".into());

    m.insert("backup.undo_none".into(), "没有要撤销的恢复操作。".into());

    m.insert(
        "backup.undo_rerestore_failed".into(),
        "无法重新恢复已删除的文件 {}：{}".into(),
    );

    m.insert(
        "backup.unknown_clear_option".into(),
        "未知的备份清除选项：{}".into(),
    );

    m.insert(
        "backup.unknown_create_option".into(),
        "未知的备份创建选项：{}".into(),
    );

    m.insert(
        "backup.unknown_recreate_option".into(),
        "未知的备份重建选项：{}".into(),
    );

    m.insert(
        "backup.unknown_undo_option".into(),
        "未知的备份撤销选项：{}".into(),
    );

    m.insert(
        "backup.usage_clear".into(),
        "用法：backup clear [--force]".into(),
    );

    m.insert(
        "backup.usage_recreate".into(),
        "用法：backup recreate [--force]".into(),
    );

    m.insert(
        "backup.usage_restore".into(),
        "用法：backup restore [--choose] [--deleted] [num]".into(),
    );

    m.insert("backup.warning_count".into(), "您有 {} 个备份：".into());

    m.insert(
        "backup.warning_title".into(),
        "\u{26A0}\u{FE0F} 警告：这将删除此项目的所有备份！".into(),
    );

    m.insert(
        "backup.would_overwrite".into(),
        "{} 将被覆盖（使用 --force 跳过）".into(),
    );

    m.insert(
        "backup.would_overwrite_deleted".into(),
        "正在恢复已删除的文件：{}".into(),
    );
}
