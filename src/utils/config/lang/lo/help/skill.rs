use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.skill.title".into(),
        "skill - ຕິດຕັ້ງ skill AI (SKILL.md) ເຂົ້າໂຟນເດີ AI harness".into(),
    );
    m.insert(
        "help.skill.download".into(),
        "skill download <alias|owner/repo[/skill]|url> [--name <ຊື່>] ## ດາວໂຫຼດ skill ຈາກ GitHub"
            .into(),
    );
    m.insert(
        "help.skill.add".into(),
        "skill add <ຊື່> to <harness> [--global] ## ຄັດລອກ skill (ຫຼື ຊຸດ) ເຂົ້າ harness".into(),
    );
    m.insert(
        "help.skill.remove".into(),
        "skill remove <ຊື່> from <harness> [--global] ## ລຶບສຳເນົາໃນ harness".into(),
    );
    m.insert(
        "help.skill.delete".into(),
        "skill delete <ຊື່> ## ລຶບ skill ອອກຈາກຄັງ (ກົງກັນຂ້າມກັບ install)".into(),
    );
    m.insert(
        "help.skill.edit".into(),
        "skill edit <ຊື່> ## ເປີດ SKILL.md ໃນ ntceditor".into(),
    );
    m.insert(
        "help.skill.list".into(),
        "skill [list] ## ສະແດງຊຸດ skill ທີ່ດາວໂຫຼດ ແລະ ສຳເນົາໃນ harness".into(),
    );
    m.insert(
        "help.skill.list_all".into(),
        "skill list --all ## ສະແດງທຸກ skill ທີ່ຕິດຕັ້ງ ບໍ່ສະເພາະຊຸດ".into(),
    );
    m.insert(
        "help.skill.avail".into(),
        "skill avail ## ສະແດງ skill ທີ່ ntc ຮອງຮັບ".into(),
    );
    m.insert(
        "help.skill.info".into(),
        "skill info <harness> ## ເບິ່ງວ່າໂຟນເດີ harness ມີ skill ໃດ".into(),
    );
    m.insert(
        "help.skill.sub".into(),
        "skill sub <ຊື່> [ls] ## ສະແດງ skill ໃນຊຸດທີ່ຕິດຕັ້ງ".into(),
    );
    m.insert(
        "help.skill.help".into(),
        "skill help ## ສະແດງວິທີໃຊ້ skill".into(),
    );
    m.insert(
        "help.skill.example1".into(),
        "skill download superpowers ## 15 skill ຈາກ obra/superpowers".into(),
    );
    m.insert(
        "help.skill.example2".into(),
        "skill download frontenddev ## ສະເພາະ frontend-design ຈາກ anthropics/skills".into(),
    );
    m.insert(
        "help.skill.example3".into(),
        "skill add superpower to opencode ## .opencode/skills ໃນໂຟນເດີປັດຈຸບັນ".into(),
    );
    m.insert(
        "help.skill.example4".into(),
        "skill add brainstorming to claude --global ## ~/.claude/skills".into(),
    );
    m.insert(
        "help.skill.example5".into(),
        "skill remove superpowers from opencode ## ເກັບສຳເນົາໃນຄັງໄວ້".into(),
    );
    m.insert(
        "help.skill.example6".into(),
        "skill download --url https://github.com/obra/superpowers ## ຄືກັນກັບໃຊ້ alias".into(),
    );
    m.insert(
        "help.skill.example7".into(),
        "skill avail ## superpowers, frontenddev, mcp-builder, sentry-fix-issues, ...".into(),
    );
    m.insert(
        "help.skill.example8".into(),
        "skill info opencode ## .opencode/skills ມີຫຍັງຢູ່ຕອນນີ້".into(),
    );
    m.insert(
        "help.skill.example9".into(),
        "skill sub superpowers ls ## ທຸກ skill ໃນຊຸດ superpowers".into(),
    );
    m.insert(
        "help.skill.example10".into(),
        "skill download --url <url> --name my-skill ## ຕິດຕັ້ງດ້ວຍຊື່ອື່ນ".into(),
    );
    m.insert(
        "help.compact.skill".into(),
        "ຕິດຕັ້ງ skill AI ເຂົ້າໂຟນເດີ AI harness".into(),
    );
}
