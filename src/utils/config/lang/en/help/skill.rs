use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.skill.title".into(),
        "skill - Install AI skills (SKILL.md) into AI harness folders".into(),
    );
    m.insert(
        "help.skill.download".into(),
        "skill download <alias|owner/repo[/skill]|url> [--name <name>] ## Download skills from GitHub"
            .into(),
    );
    m.insert(
        "help.skill.add".into(),
        "skill add <name> to <harness> [--global] ## Copy a skill (or bundle) into a harness"
            .into(),
    );
    m.insert(
        "help.skill.remove".into(),
        "skill remove <name> from <harness> [--global] ## Remove a harness copy".into(),
    );
    m.insert(
        "help.skill.delete".into(),
        "skill delete <name> ## Delete a skill from the library (the opposite of install)".into(),
    );
    m.insert(
        "help.skill.edit".into(),
        "skill edit <name> ## Open SKILL.md in ntceditor".into(),
    );
    m.insert(
        "help.skill.list".into(),
        "skill [list] ## List downloaded bundles and their harness copies".into(),
    );
    m.insert(
        "help.skill.list_all".into(),
        "skill list --all ## List every installed skill, not just bundles".into(),
    );
    m.insert(
        "help.skill.avail".into(),
        "skill avail ## List the skills ntc supports".into(),
    );
    m.insert(
        "help.skill.info".into(),
        "skill info <harness> ## Show the skills a harness folder contains".into(),
    );
    m.insert(
        "help.skill.sub".into(),
        "skill sub <name> [ls] ## Show the skills inside an installed bundle".into(),
    );
    m.insert(
        "help.skill.help".into(),
        "skill help ## Show skill usage".into(),
    );
    m.insert(
        "help.skill.example1".into(),
        "skill download superpowers ## 15 skills from obra/superpowers".into(),
    );
    m.insert(
        "help.skill.example2".into(),
        "skill download frontenddev ## only frontend-design from anthropics/skills".into(),
    );
    m.insert(
        "help.skill.example3".into(),
        "skill add superpower to opencode ## .opencode/skills in the current folder".into(),
    );
    m.insert(
        "help.skill.example4".into(),
        "skill add brainstorming to claude --global ## ~/.claude/skills".into(),
    );
    m.insert(
        "help.skill.example5".into(),
        "skill remove superpowers from opencode ## keep the library copy".into(),
    );
    m.insert(
        "help.skill.example6".into(),
        "skill download --url https://github.com/obra/superpowers ## same as the alias".into(),
    );
    m.insert(
        "help.skill.example7".into(),
        "skill avail ## superpowers, frontenddev, mcp-builder, sentry-fix-issues, ...".into(),
    );
    m.insert(
        "help.skill.example8".into(),
        "skill info opencode ## what .opencode/skills currently holds".into(),
    );
    m.insert(
        "help.skill.example9".into(),
        "skill sub superpowers ls ## every skill inside the superpowers bundle".into(),
    );
    m.insert(
        "help.skill.example10".into(),
        "skill download --url <url> --name my-skill ## install it under a different name".into(),
    );
    m.insert(
        "help.compact.skill".into(),
        "Install AI skills into AI harness folders".into(),
    );
}
