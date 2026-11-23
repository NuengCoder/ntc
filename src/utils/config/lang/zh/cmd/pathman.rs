use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.pathman.access_denied".into(),
        "拒绝访问 - 需要管理员权限。".into(),
    );

    m.insert("cmd.pathman.added_to_path".into(), "已添加到 {}：{}".into());

    m.insert(
        "cmd.pathman.already_in_path".into(),
        "已在 PATH 中：{}".into(),
    );

    m.insert("cmd.pathman.empty_path".into(), "（空）".into());

    m.insert(
        "cmd.pathman.example_add".into(),
        "  {} add ~/bin".into()
    );

    m.insert(
        "cmd.pathman.example_add_cwd".into(),
        "  {} add --cwd".into(),
    );

    m.insert(
        "cmd.pathman.example_move".into(),
        "  {} move 3 1               将第 3 条移到顶部".into(),
    );

    m.insert(
        "cmd.pathman.example_rm_idx".into(),
        "  {} rm 3                    从列表中移除第 3 条".into(),
    );

    m.insert(
        "cmd.pathman.example_rm_path".into(),
        "  {} rm ~/old-tools".into()
    );

    m.insert("cmd.pathman.example_show".into(), "  {} show".into());

    m.insert("cmd.pathman.examples_title".into(), "示例：".into());

    m.insert(
        "cmd.pathman.help_add".into(),
        "  {} add <路径>               向 PATH 添加目录".into(),
    );

    m.insert(
        "cmd.pathman.help_add_cwd".into(),
        "  {} add --cwd | --here      将当前目录添加到 PATH".into(),
    );

    m.insert(
        "cmd.pathman.help_move".into(),
        "  {} move <n> <m>            将第 n 条移动到第 m 位".into(),
    );

    m.insert(
        "cmd.pathman.help_rm".into(),
        "  {} rm <路径或序号>          按路径或序号从 PATH 中移除".into(),
    );

    m.insert(
        "cmd.pathman.help_show".into(),
        "  {} show                    显示带序号的 PATH 条目".into(),
    );

    m.insert(
        "cmd.pathman.idx_out_of_range".into(),
        "序号超出范围：{}（1-{}）".into(),
    );

    m.insert(
        "cmd.pathman.indices_numbers".into(),
        "索引必须是数字".into(),
    );

    m.insert(
        "cmd.pathman.mode_admin_hint".into(),
        "然后重新运行命令。或使用：mode admin".into(),
    );

    m.insert(
        "cmd.pathman.move_usage".into(),
        "用法：{} move <n> <m>".into(),
    );

    m.insert(
        "cmd.pathman.moved".into(),
        "已移动到 {} 中的第 #{} 位：{}".into(),
    );

    m.insert(
        "cmd.pathman.new_terminal".into(),
        "更改将应用于新的终端窗口。".into(),
    );

    m.insert(
        "cmd.pathman.not_found_in_path".into(),
        "PATH 中未找到：{}".into(),
    );

    m.insert(
        "cmd.pathman.path_update_fail".into(),
        "更新 PATH 失败：{}".into(),
    );

    m.insert(
        "cmd.pathman.reg_query_fail".into(),
        "查询注册表失败：{}".into(),
    );

    m.insert(
        "cmd.pathman.reg_write_fail".into(),
        "写入注册表失败：{}".into(),
    );

    m.insert(
        "cmd.pathman.removed_from_path".into(),
        "已从 {} 移除：{}".into(),
    );

    m.insert(
        "cmd.pathman.rm_usage".into(),
        "用法：usrpath rm <路径或编号>".into(),
    );

    m.insert(
        "cmd.pathman.run_as_admin".into(),
        "请以管理员身份运行终端（右键 → \"以管理员身份运行\"），".into(),
    );

    m.insert(
        "cmd.pathman.same_source_target".into(),
        "源和目标相同 \u{2014} 无需移动".into(),
    );

    m.insert("cmd.pathman.system_level".into(), "系统".into());

    m.insert("cmd.pathman.total_entries".into(), "  总计：{} 条".into());

    m.insert("cmd.pathman.usage_title".into(), "用法：".into());

    m.insert("cmd.pathman.user_level".into(), "用户".into());

    m.insert(
        "cmd.pathman.new_login_shell".into(),
        "更改将在新的会话中生效。".into(),
    );

    m.insert(
        "cmd.pathman.next_login".into(),
        "已保存。新的 PATH 将在下次登录时生效（请退出后重新登录，或启动 login shell）。".into(),
    );

    m.insert(
        "cmd.pathman.platform_unsupported".into(),
        "usrpath/syspath 不支持此平台。".into(),
    );

    m.insert(
        "cmd.pathman.shell_profile_hint".into(),
        "将这一行加入你的 shell 配置文件：{}".into(),
    );

    m.insert("cmd.pathman.store_file".into(), "  文件：{}".into());

    m.insert("cmd.pathman.sudo_fail".into(), "无法运行 sudo：{}".into());

    m.insert(
        "cmd.pathman.sudo_write".into(),
        "系统 PATH 归 root 所有 — 正在用 sudo 写入 {} …".into(),
    );
}
