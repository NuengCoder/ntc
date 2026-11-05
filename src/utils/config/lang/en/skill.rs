use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "skill.help_usage".into(),
        "ntc skill <command> [args]".into(),
    );
    m.insert(
        "skill.help_download".into(),
        "  skill download <alias|owner/repo[/skill]|url> [--name <name>] [--force]  ## fetch skills into the ntc library".into(),
    );
    m.insert(
        "skill.help_install".into(),
        "  skill install | i  ## alias of skill download".into(),
    );
    m.insert(
        "skill.help_avail".into(),
        "  skill avail  ## list the skills ntc supports".into(),
    );
    m.insert(
        "skill.help_add".into(),
        "  skill add <name> to <harness> [--global] [--force]  ## copy skills into an AI harness"
            .into(),
    );
    m.insert(
        "skill.help_remove".into(),
        "  skill remove <name> from <harness> [--global] [--force]  ## remove a harness copy"
            .into(),
    );
    m.insert(
        "skill.help_delete".into(),
        "  skill delete <name> [--force]  ## delete a skill from the library (alias: uninstall | u)"
            .into(),
    );
    m.insert(
        "skill.help_edit".into(),
        "  skill edit <name>  ## open SKILL.md in ntceditor".into(),
    );
    m.insert(
        "skill.help_list".into(),
        "  skill [list]  ## list installed skill bundles".into(),
    );
    m.insert(
        "skill.help_list_all".into(),
        "  skill list --all  ## list every installed skill and its copies".into(),
    );
    m.insert(
        "skill.help_info".into(),
        "  skill info <harness>  ## show the skills a harness folder contains".into(),
    );
    m.insert(
        "skill.help_sub".into(),
        "  skill sub <name> [ls]  ## show the skills inside an installed bundle".into(),
    );
    m.insert(
        "skill.help_help".into(),
        "  skill help  ## show this help".into(),
    );
    m.insert(
        "skill.help_harnesses".into(),
        "harnesses: opencode, claude, codex, agents, freebuff, kilocode, mimocode, workbuddy, trae"
            .into(),
    );
    m.insert(
        "skill.help_scope".into(),
        "scope: project (cwd/<.harness>/skills) by default; add --global for the personal folder"
            .into(),
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
        "Unknown skill command '{}'. Run 'skill help'.".into(),
    );
    m.insert(
        "skill.unexpected_arg".into(),
        "Unexpected argument '{}' for skill. Run 'skill help'.".into(),
    );
    m.insert(
        "skill.missing_harness".into(),
        "Missing harness after to/from. Use opencode, claude, codex, agents, freebuff, kilocode, mimocode, workbuddy or trae.".into(),
    );
    m.insert(
        "skill.missing_url".into(),
        "Missing URL after --url.".into(),
    );
    m.insert(
        "skill.missing_name".into(),
        "Missing name after --name.".into(),
    );
    m.insert(
        "skill.usage_download".into(),
        "Usage: skill download <alias|owner/repo[/skill]|url> [--name <name>] [--force]".into(),
    );
    m.insert(
        "skill.usage_add".into(),
        "Usage: skill add <name> to <harness> [--global] [--force]".into(),
    );
    m.insert(
        "skill.usage_remove".into(),
        "Usage: skill remove <name> from <harness> [--global] [--force]".into(),
    );
    m.insert(
        "skill.usage_delete".into(),
        "Usage: skill delete <name> [--force]".into(),
    );
    m.insert("skill.usage_edit".into(), "Usage: skill edit <name>".into());
    m.insert(
        "skill.usage_info".into(),
        "Usage: skill info <harness>".into(),
    );
    m.insert(
        "skill.usage_sub".into(),
        "Usage: skill sub <name> [ls]".into(),
    );
    m.insert(
        "skill.empty".into(),
        "No skills installed yet. Try 'skill download superpowers' or 'skill download frontenddev'."
            .into(),
    );
    m.insert(
        "skill.installed_header".into(),
        "Skills in the ntc library:".into(),
    );
    m.insert(
        "skill.bundles_header".into(),
        "Installed skill bundles:".into(),
    );
    m.insert("skill.bundle_tag".into(), "[{}]".into());
    m.insert("skill.bundle_count".into(), "({} skills)".into());
    m.insert(
        "skill.list_all_hint".into(),
        "Run 'skill list --all' to see every skill.".into(),
    );
    m.insert(
        "skill.avail_header".into(),
        "Skills ntc can download:".into(),
    );
    m.insert("skill.avail_all".into(), "{} (all skills)".into());
    m.insert(
        "skill.avail_hint".into(),
        "Install one with 'skill download <name>'.".into(),
    );
    m.insert(
        "skill.info_header".into(),
        "Skills in the {} harness:".into(),
    );
    m.insert(
        "skill.info_empty".into(),
        "No skills found for {} (project or global folders).".into(),
    );
    m.insert("skill.info_ntc".into(), "[ntc]".into());
    m.insert("skill.info_manual".into(), "[manual]".into());
    m.insert("skill.sub_header".into(), "Skills in {}:".into());
    m.insert(
        "skill.sub_none".into(),
        "{} does not contain sub skills.".into(),
    );
    m.insert("skill.scope_project".into(), "project".into());
    m.insert("skill.scope_global".into(), "global".into());
    m.insert("skill.downloaded".into(), "added {} ({})".into());
    m.insert("skill.added".into(), "{} -> {}".into());
    m.insert("skill.removed".into(), "{} ({})".into());
    m.insert("skill.deleted".into(), "{} (library)".into());
    m.insert("skill.edit_opened".into(), "Editing {}".into());
    m.insert(
        "skill.edit_refreshed".into(),
        "Registry updated for {}.".into(),
    );
    m.insert("skill.edit_invalid".into(), "{}".into());
    m.insert(
        "skill.harness_pick".into(),
        "Copy the skill to which harness?".into(),
    );
    m.insert("skill.enter_number".into(), "Enter number (1-{}): ".into());
}
