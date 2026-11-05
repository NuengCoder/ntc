use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "skill.help_usage".into(),
        "ntc skill <ຄຳສັ່ງ> [ອາກິວເມັນ]".into(),
    );
    m.insert(
        "skill.help_download".into(),
        "  skill download <alias|owner/repo[/skill]|url> [--name <ຊື່>] [--force]  ## ດາວໂຫຼດ skill ເຂົ້າຄັງ ntc"
            .into(),
    );
    m.insert(
        "skill.help_install".into(),
        "  skill install | i  ## ຊື່ເອີ້ນອີກຂອງ skill download".into(),
    );
    m.insert(
        "skill.help_avail".into(),
        "  skill avail  ## ສະແດງ skill ທີ່ ntc ຮອງຮັບ".into(),
    );
    m.insert(
        "skill.help_add".into(),
        "  skill add <ຊື່> to <harness> [--global] [--force]  ## ຄັດລອກ skill ເຂົ້າ AI harness".into(),
    );
    m.insert(
        "skill.help_remove".into(),
        "  skill remove <ຊື່> from <harness> [--global] [--force]  ## ລຶບສຳເນົາໃນ harness".into(),
    );
    m.insert(
        "skill.help_delete".into(),
        "  skill delete <ຊື່> [--force]  ## ລຶບ skill ອອກຈາກຄັງ (ຊື່ອື່ນ: uninstall | u)".into(),
    );
    m.insert(
        "skill.help_edit".into(),
        "  skill edit <ຊື່>  ## ເປີດ SKILL.md ໃນ ntceditor".into(),
    );
    m.insert(
        "skill.help_list".into(),
        "  skill [list]  ## ສະແດງຊຸດ skill ທີ່ຕິດຕັ້ງ".into(),
    );
    m.insert(
        "skill.help_list_all".into(),
        "  skill list --all  ## ສະແດງທຸກ skill ແລະ ສຳເນົາ".into(),
    );
    m.insert(
        "skill.help_info".into(),
        "  skill info <harness>  ## ເບິ່ງວ່າໂຟນເດີ harness ມີ skill ໃດ".into(),
    );
    m.insert(
        "skill.help_sub".into(),
        "  skill sub <ຊື່> [ls]  ## ສະແດງ skill ໃນຊຸດທີ່ຕິດຕັ້ງ".into(),
    );
    m.insert(
        "skill.help_help".into(),
        "  skill help  ## ສະແດງວິທີໃຊ້".into(),
    );
    m.insert(
        "skill.help_harnesses".into(),
        "harness: opencode, claude, codex, agents, freebuff, kilocode, mimocode, workbuddy, trae"
            .into(),
    );
    m.insert(
        "skill.help_scope".into(),
        "ຂອບເຂດ: ຄ່າເລີ່ມຕົ້ນ project (ໄດເຣັກທໍຣີປັດຈຸບັນ/.<harness>/skills); ໃສ່ --global ເພື່ອໃຊ້ໂຟນເດີສ່ວນຕົວ"
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
        "ບໍ່ຮູ້ຈັກຄຳສັ່ງ skill '{}' ໃຫ້ລັນ 'skill help'".into(),
    );
    m.insert(
        "skill.unexpected_arg".into(),
        "skill ບໍ່ຮັບອາກິວເມັນ '{}' ໃຫ້ລັນ 'skill help'".into(),
    );
    m.insert(
        "skill.missing_harness".into(),
        "ຕ້ອງລະບຸ harness ຫຼັງ to/from: opencode, claude, codex, agents, freebuff, kilocode, mimocode, workbuddy ຫຼື trae"
            .into(),
    );
    m.insert("skill.missing_url".into(), "ຕ້ອງລະບຸ URL ຫຼັງ --url".into());
    m.insert("skill.missing_name".into(), "ຕ້ອງລະບຸຊື່ຫຼັງ --name".into());
    m.insert(
        "skill.usage_download".into(),
        "ວິທີໃຊ້: skill download <alias|owner/repo[/skill]|url> [--name <ຊື່>] [--force]".into(),
    );
    m.insert(
        "skill.usage_add".into(),
        "ວິທີໃຊ້: skill add <ຊື່> to <harness> [--global] [--force]".into(),
    );
    m.insert(
        "skill.usage_remove".into(),
        "ວິທີໃຊ້: skill remove <ຊື່> from <harness> [--global] [--force]".into(),
    );
    m.insert(
        "skill.usage_delete".into(),
        "ວິທີໃຊ້: skill delete <ຊື່> [--force]".into(),
    );
    m.insert("skill.usage_edit".into(), "ວິທີໃຊ້: skill edit <ຊື່>".into());
    m.insert(
        "skill.usage_info".into(),
        "ວິທີໃຊ້: skill info <harness>".into(),
    );
    m.insert("skill.usage_sub".into(), "ວິທີໃຊ້: skill sub <ຊື່> [ls]".into());
    m.insert(
        "skill.empty".into(),
        "ຍັງບໍ່ມີ skill ທີ່ຕິດຕັ້ງ ລອງ 'skill download superpowers' ຫຼື 'skill download frontenddev'".into(),
    );
    m.insert("skill.installed_header".into(), "skill ໃນຄັງ ntc:".into());
    m.insert("skill.bundles_header".into(), "ຊຸດ skill ທີ່ຕິດຕັ້ງ:".into());
    m.insert("skill.bundle_tag".into(), "[{}]".into());
    m.insert("skill.bundle_count".into(), "({} skill)".into());
    m.insert(
        "skill.list_all_hint".into(),
        "ລັນ 'skill list --all' ເພື່ອເບິ່ງທຸກ skill".into(),
    );
    m.insert("skill.avail_header".into(), "skill ທີ່ ntc ດາວໂຫຼດໄດ້:".into());
    m.insert("skill.avail_all".into(), "{} (ທຸກ skill)".into());
    m.insert(
        "skill.avail_hint".into(),
        "ຕິດຕັ້ງດ້ວຍ 'skill download <ຊື່>'".into(),
    );
    m.insert("skill.info_header".into(), "skill ໃນ harness {}:".into());
    m.insert(
        "skill.info_empty".into(),
        "ບໍ່ພົບ skill ສຳລັບ {} (ໂຟນເດີໂປຣເຈັກ ຫຼື ສ່ວນຕົວ)".into(),
    );
    m.insert("skill.info_ntc".into(), "[ntc]".into());
    m.insert("skill.info_manual".into(), "[ດ້ວຍຕົນເອງ]".into());
    m.insert("skill.sub_header".into(), "skill ໃນ {}:".into());
    m.insert("skill.sub_none".into(), "{} ບໍ່ມີ skill ຍ່ອຍ".into());
    m.insert("skill.scope_project".into(), "ໂປຣເຈັກ".into());
    m.insert("skill.scope_global".into(), "ສ່ວນຕົວ".into());
    m.insert("skill.downloaded".into(), "ເພີ່ມ {} ({})".into());
    m.insert("skill.added".into(), "{} -> {}".into());
    m.insert("skill.removed".into(), "{} ({})".into());
    m.insert("skill.deleted".into(), "{} (ຄັງ)".into());
    m.insert("skill.edit_opened".into(), "ກຳລັງແກ້ໄຂ {}".into());
    m.insert("skill.edit_refreshed".into(), "ອັບເດດທະບຽນຂອງ {} ແລ້ວ".into());
    m.insert("skill.edit_invalid".into(), "{}".into());
    m.insert(
        "skill.harness_pick".into(),
        "ຈະຄັດລອກ skill ໄປ harness ໃດ?".into(),
    );
    m.insert("skill.enter_number".into(), "ໃສ່ໝາຍເລກ (1-{}): ".into());
}
