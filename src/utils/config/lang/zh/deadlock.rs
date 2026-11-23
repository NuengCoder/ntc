use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "deadlock.add_usage".into(),
        "用法：add <pass-env> <real-password>".into(),
    );
    m.insert("deadlock.added".into(), "已保存 pass-env“{}”。".into());
    m.insert("deadlock.bye".into(), "deadlock 已关闭。".into());
    m.insert(
        "deadlock.confirm_master_prompt".into(),
        "确认新主密码：".into(),
    );
    m.insert(
        "deadlock.corrupt_aborted".into(),
        "保险库无法读取，必须先恢复才能打开 deadlock。".into(),
    );
    m.insert(
        "deadlock.corrupt_cancelled".into(),
        "已取消恢复；保险库未被修改。".into(),
    );
    m.insert(
        "deadlock.corrupt_kept".into(),
        "无法读取的保险库已保留为 {}。".into(),
    );
    m.insert(
        "deadlock.corrupt_notice".into(),
        "位于 {} 的保险库文件无法读取（不是有效的 deadlock 保险库）。".into(),
    );
    m.insert(
        "deadlock.corrupt_recreate_failed".into(),
        "无法将无法读取的保险库移开：{}".into(),
    );
    m.insert(
        "deadlock.corrupt_recreate_prompt".into(),
        "现在创建一个空保险库吗？无法读取的文件会保留在保险库旁边。[y/N]".into(),
    );
    m.insert(
        "deadlock.corrupt_recreated".into(),
        "已创建新的空保险库。".into(),
    );
    m.insert(
        "deadlock.corrupt_restore_failed".into(),
        "无法恢复备份：{}".into(),
    );
    m.insert(
        "deadlock.corrupt_restore_prompt".into(),
        "有可用的保险库备份副本。要恢复吗？[y/N]".into(),
    );
    m.insert(
        "deadlock.corrupt_restored".into(),
        "已从备份副本恢复保险库。".into(),
    );
    m.insert("deadlock.created".into(), "已在 {} 创建新保险库。".into());
    m.insert(
        "deadlock.default_master_hint".into(),
        "默认主密码：{} - 系统会要求你更改它。".into(),
    );
    m.insert(
        "deadlock.default_master_rejected".into(),
        "请选择默认密码以外的主密码。".into(),
    );
    m.insert(
        "deadlock.destroy_cancelled".into(),
        "已取消删除保险库。".into(),
    );
    m.insert(
        "deadlock.destroy_confirm".into(),
        "永久删除保险库和所有已保存的密钥？".into(),
    );
    m.insert(
        "deadlock.destroy_failed".into(),
        "无法删除保险库：{}".into(),
    );
    m.insert("deadlock.destroyed".into(), "保险库已删除。".into());
    m.insert("deadlock.editor_fail".into(), "无法启动行编辑器：{}".into());
    m.insert("deadlock.empty_master".into(), "主密码不能为空。".into());
    m.insert("deadlock.empty_password".into(), "密码不能为空。".into());
    m.insert(
        "deadlock.force_change_notice".into(),
        "仍在使用默认主密码；请立即设置新密码。".into(),
    );
    m.insert(
        "deadlock.forget_done".into(),
        "已从操作系统凭据存储中移除主密码。".into(),
    );
    m.insert(
        "deadlock.forget_failed".into(),
        "无法从操作系统凭据存储中移除主密码：{}".into(),
    );
    m.insert(
        "deadlock.hint_help".into(),
        "输入 'help' 查看 deadlock 命令。".into(),
    );
    m.insert(
        "deadlock.invalid_name".into(),
        "无效的 pass-env 名称“{}”（只能使用字母、数字、_ - .）。".into(),
    );
    m.insert(
        "deadlock.keychain_registered".into(),
        "主密码已注册到操作系统凭据存储（nssh 可解锁保险库；使用 'forget' 移除）。".into(),
    );
    m.insert(
        "deadlock.keychain_unavailable".into(),
        "操作系统凭据存储不可用 - nssh 将询问主密码。".into(),
    );
    m.insert(
        "deadlock.ls_empty".into(),
        "保险库为空。使用以下命令添加：add <pass-env> <real-password>".into(),
    );
    m.insert("deadlock.ls_title".into(), "PASS-ENV -> 真实密码：".into());
    m.insert(
        "deadlock.master_changed".into(),
        "主密码已更改；保险库已重新加密。".into(),
    );
    m.insert(
        "deadlock.master_mismatch".into(),
        "密码不匹配；请重试。".into(),
    );
    m.insert("deadlock.master_prompt".into(), "主密码：".into());
    m.insert("deadlock.new_master_prompt".into(), "新主密码：".into());
    m.insert(
        "deadlock.no_args".into(),
        "deadlock 会打开自己的外壳，不接受参数（使用 --help 查看命令）。".into(),
    );
    m.insert(
        "deadlock.not_found".into(),
        "没有名为“{}”的 pass-env。".into(),
    );
    m.insert("deadlock.old_master_prompt".into(), "旧主密码：".into());
    m.insert(
        "deadlock.os_protected".into(),
        "保险库文件已受到操作系统删除保护。".into(),
    );
    m.insert(
        "deadlock.os_unprotected".into(),
        "此平台/账户不支持操作系统级删除保护（文件权限仍然有效）。".into(),
    );
    m.insert("deadlock.removed".into(), "已移除 pass-env“{}”。".into());
    m.insert("deadlock.rmpw_usage".into(), "用法：rmpw <pass-env>".into());
    m.insert(
        "deadlock.too_many_attempts".into(),
        "主密码错误次数过多 - 正在中止。".into(),
    );
    m.insert(
        "deadlock.unknown_cmd".into(),
        "未知 deadlock 命令“{}”。".into(),
    );
    m.insert("deadlock.unlocked".into(), "保险库已解锁。".into());
    m.insert("deadlock.updated".into(), "已更新 pass-env“{}”。".into());
    m.insert("deadlock.value_prompt".into(), "真实密码：".into());
    m.insert(
        "deadlock.vault_missing".into(),
        "还没有 deadlock 保险库 - 运行一次 `deadlock` 创建它。".into(),
    );
    m.insert(
        "deadlock.wrong_master".into(),
        "主密码错误（第 {}/3 次尝试）。".into(),
    );
    m.insert("deadlock.wrong_master_once".into(), "主密码错误。".into());
}
