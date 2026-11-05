use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("skill.help_usage".into(), "ntc skill <命令> [参数]".into());
    m.insert(
        "skill.help_download".into(),
        "  skill download <别名|owner/repo[/skill]|url> [--name <名称>] [--force]  ## 下载技能到 ntc 库"
            .into(),
    );
    m.insert(
        "skill.help_install".into(),
        "  skill install | i  ## skill download 的别名".into(),
    );
    m.insert(
        "skill.help_avail".into(),
        "  skill avail  ## 列出 ntc 支持的技能".into(),
    );
    m.insert(
        "skill.help_add".into(),
        "  skill add <名称> to <harness> [--global] [--force]  ## 复制技能到 AI harness".into(),
    );
    m.insert(
        "skill.help_remove".into(),
        "  skill remove <名称> from <harness> [--global] [--force]  ## 移除 harness 中的副本"
            .into(),
    );
    m.insert(
        "skill.help_delete".into(),
        "  skill delete <名称> [--force]  ## 从库中删除技能（别名：uninstall | u）".into(),
    );
    m.insert(
        "skill.help_edit".into(),
        "  skill edit <名称>  ## 用 ntceditor 打开 SKILL.md".into(),
    );
    m.insert(
        "skill.help_list".into(),
        "  skill [list]  ## 列出已安装的技能合集".into(),
    );
    m.insert(
        "skill.help_list_all".into(),
        "  skill list --all  ## 列出每个已安装技能及其副本".into(),
    );
    m.insert(
        "skill.help_info".into(),
        "  skill info <harness>  ## 查看 harness 目录里有哪些技能".into(),
    );
    m.insert(
        "skill.help_sub".into(),
        "  skill sub <名称> [ls]  ## 查看已安装合集包含的技能".into(),
    );
    m.insert("skill.help_help".into(), "  skill help  ## 显示帮助".into());
    m.insert(
        "skill.help_harnesses".into(),
        "harness：opencode、claude、codex、agents、freebuff、kilocode、mimocode、workbuddy、trae"
            .into(),
    );
    m.insert(
        "skill.help_scope".into(),
        "范围：默认为 project（当前目录/.<harness>/skills）；加 --global 使用个人目录".into(),
    );
    m.insert("skill.example1".into(), "skill download superpowers".into());
    m.insert("skill.example2".into(), "skill download frontenddev".into());
    m.insert(
        "skill.example3".into(),
        "skill add superpower to opencode".into(),
    );
    m.insert(
        "skill.example4".into(),
        "skill add brainstorming to claude --global".into(),
    );
    m.insert("skill.example5".into(), "skill edit brainstorming".into());
    m.insert(
        "skill.example6".into(),
        "skill download --url https://github.com/obra/superpowers".into(),
    );
    m.insert("skill.example7".into(), "skill avail".into());
    m.insert("skill.example8".into(), "skill info opencode".into());
    m.insert("skill.example9".into(), "skill sub superpowers ls".into());
    m.insert(
        "skill.example10".into(),
        "skill download --url https://github.com/getsentry/sentry-agent-skills --name sentry"
            .into(),
    );
    m.insert(
        "skill.unknown_subcmd".into(),
        "未知的 skill 子命令 '{}'。运行 'skill help'。".into(),
    );
    m.insert(
        "skill.unexpected_arg".into(),
        "skill 不接受参数 '{}'。运行 'skill help'。".into(),
    );
    m.insert(
        "skill.missing_harness".into(),
        "to/from 后缺少 harness。可用 opencode、claude、codex、agents、freebuff、kilocode、mimocode、workbuddy、trae。"
            .into(),
    );
    m.insert("skill.missing_url".into(), "--url 后缺少 URL。".into());
    m.insert("skill.missing_name".into(), "--name 后缺少名称。".into());
    m.insert(
        "skill.usage_download".into(),
        "用法：skill download <别名|owner/repo[/skill]|url> [--name <名称>] [--force]".into(),
    );
    m.insert(
        "skill.usage_add".into(),
        "用法：skill add <名称> to <harness> [--global] [--force]".into(),
    );
    m.insert(
        "skill.usage_remove".into(),
        "用法：skill remove <名称> from <harness> [--global] [--force]".into(),
    );
    m.insert(
        "skill.usage_delete".into(),
        "用法：skill delete <名称> [--force]".into(),
    );
    m.insert("skill.usage_edit".into(), "用法：skill edit <名称>".into());
    m.insert(
        "skill.usage_info".into(),
        "用法：skill info <harness>".into(),
    );
    m.insert(
        "skill.usage_sub".into(),
        "用法：skill sub <名称> [ls]".into(),
    );
    m.insert(
        "skill.empty".into(),
        "还没有安装技能。试试 'skill download superpowers' 或 'skill download frontenddev'。"
            .into(),
    );
    m.insert("skill.installed_header".into(), "ntc 库中的技能：".into());
    m.insert("skill.bundles_header".into(), "已安装的技能合集：".into());
    m.insert("skill.bundle_tag".into(), "[{}]".into());
    m.insert("skill.bundle_count".into(), "（{} 个技能）".into());
    m.insert(
        "skill.list_all_hint".into(),
        "运行 'skill list --all' 查看每个技能。".into(),
    );
    m.insert("skill.avail_header".into(), "ntc 可下载的技能：".into());
    m.insert("skill.avail_all".into(), "{}（全部技能）".into());
    m.insert(
        "skill.avail_hint".into(),
        "用 'skill download <名称>' 安装。".into(),
    );
    m.insert("skill.info_header".into(), "{} harness 中的技能：".into());
    m.insert(
        "skill.info_empty".into(),
        "{} 中没有找到技能（项目或全局目录）。".into(),
    );
    m.insert("skill.info_ntc".into(), "[ntc]".into());
    m.insert("skill.info_manual".into(), "[手动]".into());
    m.insert("skill.sub_header".into(), "{} 包含的技能：".into());
    m.insert("skill.sub_none".into(), "{} 不包含子技能。".into());
    m.insert("skill.scope_project".into(), "项目".into());
    m.insert("skill.scope_global".into(), "全局".into());
    m.insert("skill.downloaded".into(), "已添加 {} ({})".into());
    m.insert("skill.added".into(), "{} -> {}".into());
    m.insert("skill.removed".into(), "{} ({})".into());
    m.insert("skill.deleted".into(), "{} (库)".into());
    m.insert("skill.edit_opened".into(), "正在编辑 {}".into());
    m.insert(
        "skill.edit_refreshed".into(),
        "已更新 {} 的注册信息。".into(),
    );
    m.insert("skill.edit_invalid".into(), "{}".into());
    m.insert(
        "skill.harness_pick".into(),
        "把技能复制到哪个 harness？".into(),
    );
    m.insert("skill.enter_number".into(), "输入编号 (1-{}): ".into());
}
