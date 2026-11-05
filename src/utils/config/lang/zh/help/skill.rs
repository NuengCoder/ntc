use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.skill.title".into(),
        "skill - 把 AI 技能 (SKILL.md) 安装到 AI harness 目录".into(),
    );
    m.insert(
        "help.skill.download".into(),
        "skill download <别名|owner/repo[/skill]|url> [--name <名称>] ## 从 GitHub 下载技能".into(),
    );
    m.insert(
        "help.skill.add".into(),
        "skill add <名称> to <harness> [--global] ## 复制技能（或合集）到 harness".into(),
    );
    m.insert(
        "help.skill.remove".into(),
        "skill remove <名称> from <harness> [--global] ## 移除 harness 中的副本".into(),
    );
    m.insert(
        "help.skill.delete".into(),
        "skill delete <名称> ## 从库中删除技能（与 install 相反）".into(),
    );
    m.insert(
        "help.skill.edit".into(),
        "skill edit <名称> ## 用 ntceditor 打开 SKILL.md".into(),
    );
    m.insert(
        "help.skill.list".into(),
        "skill [list] ## 列出已下载的合集及副本".into(),
    );
    m.insert(
        "help.skill.list_all".into(),
        "skill list --all ## 列出每个已安装技能".into(),
    );
    m.insert(
        "help.skill.avail".into(),
        "skill avail ## 列出 ntc 支持的技能".into(),
    );
    m.insert(
        "help.skill.info".into(),
        "skill info <harness> ## 查看 harness 目录里的技能".into(),
    );
    m.insert(
        "help.skill.sub".into(),
        "skill sub <名称> [ls] ## 查看已安装合集里的技能".into(),
    );
    m.insert(
        "help.skill.help".into(),
        "skill help ## 显示 skill 用法".into(),
    );
    m.insert(
        "help.skill.example1".into(),
        "skill download superpowers ## 来自 obra/superpowers 的 15 个技能".into(),
    );
    m.insert(
        "help.skill.example2".into(),
        "skill download frontenddev ## 只取 anthropics/skills 的 frontend-design".into(),
    );
    m.insert(
        "help.skill.example3".into(),
        "skill add superpower to opencode ## 当前目录的 .opencode/skills".into(),
    );
    m.insert(
        "help.skill.example4".into(),
        "skill add brainstorming to claude --global ## ~/.claude/skills".into(),
    );
    m.insert(
        "help.skill.example5".into(),
        "skill remove superpowers from opencode ## 保留库中的副本".into(),
    );
    m.insert(
        "help.skill.example6".into(),
        "skill download --url https://github.com/obra/superpowers ## 与别名等价".into(),
    );
    m.insert(
        "help.skill.example7".into(),
        "skill avail ## superpowers、frontenddev、mcp-builder、sentry-fix-issues……".into(),
    );
    m.insert(
        "help.skill.example8".into(),
        "skill info opencode ## .opencode/skills 当前包含什么".into(),
    );
    m.insert(
        "help.skill.example9".into(),
        "skill sub superpowers ls ## superpowers 合集里的每个技能".into(),
    );
    m.insert(
        "help.skill.example10".into(),
        "skill download --url <url> --name my-skill ## 用另一个名字安装".into(),
    );
    m.insert(
        "help.compact.skill".into(),
        "把 AI 技能安装到 AI harness 目录".into(),
    );
}
