use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.backup.clear".into(),
        "backup clear ## 删除所有备份（需确认）".into(),
    );

    m.insert(
        "help.backup.clear_force".into(),
        "backup clear --force ## 删除所有备份（无需确认）".into(),
    );

    m.insert(
        "help.backup.create".into(),
        "backup create ## 创建当前项目的备份".into(),
    );

    m.insert(
        "help.backup.diff".into(),
        "backup diff [num] ## 比较当前项目与备份的差异".into(),
    );

    m.insert(
        "help.backup.list".into(),
        "backup list ## 列出所有备份".into(),
    );

    m.insert(
        "help.backup.recreate".into(),
        "backup recreate ## 清除全部+撤销+创建全新备份".into(),
    );

    m.insert(
        "help.backup.restore".into(),
        "backup restore [num] ## 恢复备份（未指定编号时交互式）".into(),
    );

    m.insert(
        "help.backup.restore_choose".into(),
        "backup restore --choose ## 从交互式菜单中选择备份".into(),
    );

    m.insert(
        "help.backup.restore_deleted".into(),
        "backup restore --deleted ## 仅恢复已删除的文件".into(),
    );

    m.insert("help.backup.title".into(), "备份命令：".into());

    m.insert(
        "help.backup.undo".into(),
        "backup undo ## 撤销上次恢复".into(),
    );

    m.insert(
        "help.backup.undo_cls".into(),
        "backup undo --cls ## 清除撤销历史（需确认）".into(),
    );

    m.insert(
        "help.backup.undo_force".into(),
        "backup undo --force ## 清除撤销历史（无需确认）".into(),
    );

    m.insert(
        "help.backup.example1".into(),
        "backup create ## 创建新备份".into(),
    );

    m.insert(
        "help.backup.example2".into(),
        "backup list ## 列出所有备份".into(),
    );

    m.insert(
        "help.backup.example3".into(),
        "backup restore 1 ## 恢复备份 #1".into(),
    );

    m.insert(
        "help.backup.example4".into(),
        "backup diff 1 ## 对比备份 #1".into(),
    );
}
