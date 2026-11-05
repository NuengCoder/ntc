use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "skill.help_usage".into(),
        "ntc skill <คำสั่ง> [อาร์กิวเมนต์]".into(),
    );
    m.insert(
        "skill.help_download".into(),
        "  skill download <alias|owner/repo[/skill]|url> [--name <ชื่อ>] [--force]  ## ดาวน์โหลดสกิลเข้าคลัง ntc"
            .into(),
    );
    m.insert(
        "skill.help_install".into(),
        "  skill install | i  ## ชื่อเรียกอีกชื่อของ skill download".into(),
    );
    m.insert(
        "skill.help_avail".into(),
        "  skill avail  ## แสดงสกิลที่ ntc รองรับ".into(),
    );
    m.insert(
        "skill.help_add".into(),
        "  skill add <ชื่อ> to <harness> [--global] [--force]  ## คัดลอกสกิลเข้า AI harness".into(),
    );
    m.insert(
        "skill.help_remove".into(),
        "  skill remove <ชื่อ> from <harness> [--global] [--force]  ## ลบสำเนาใน harness".into(),
    );
    m.insert(
        "skill.help_delete".into(),
        "  skill delete <ชื่อ> [--force]  ## ลบสกิลออกจากคลัง (ชื่ออื่น: uninstall | u)".into(),
    );
    m.insert(
        "skill.help_edit".into(),
        "  skill edit <ชื่อ>  ## เปิด SKILL.md ใน ntceditor".into(),
    );
    m.insert(
        "skill.help_list".into(),
        "  skill [list]  ## แสดงชุดสกิลที่ติดตั้ง".into(),
    );
    m.insert(
        "skill.help_list_all".into(),
        "  skill list --all  ## แสดงทุกสกิลและสำเนา".into(),
    );
    m.insert(
        "skill.help_info".into(),
        "  skill info <harness>  ## ดูว่ามีสกิลใดในโฟลเดอร์ harness".into(),
    );
    m.insert(
        "skill.help_sub".into(),
        "  skill sub <ชื่อ> [ls]  ## แสดงสกิลในชุดที่ติดตั้ง".into(),
    );
    m.insert("skill.help_help".into(), "  skill help  ## แสดงวิธีใช้".into());
    m.insert(
        "skill.help_harnesses".into(),
        "harness: opencode, claude, codex, agents, freebuff, kilocode, mimocode, workbuddy, trae"
            .into(),
    );
    m.insert(
        "skill.help_scope".into(),
        "ขอบเขต: ค่าเริ่มต้น project (ไดเรกทอรีปัจจุบัน/.<harness>/skills); ใส่ --global เพื่อใช้โฟลเดอร์ส่วนตัว"
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
        "ไม่รู้จักคำสั่ง skill '{}' ให้รัน 'skill help'".into(),
    );
    m.insert(
        "skill.unexpected_arg".into(),
        "skill ไม่รับอาร์กิวเมนต์ '{}' ให้รัน 'skill help'".into(),
    );
    m.insert(
        "skill.missing_harness".into(),
        "ต้องระบุ harness หลัง to/from: opencode, claude, codex, agents, freebuff, kilocode, mimocode, workbuddy หรือ trae"
            .into(),
    );
    m.insert("skill.missing_url".into(), "ต้องระบุ URL หลัง --url".into());
    m.insert("skill.missing_name".into(), "ต้องระบุชื่อหลัง --name".into());
    m.insert(
        "skill.usage_download".into(),
        "วิธีใช้: skill download <alias|owner/repo[/skill]|url> [--name <ชื่อ>] [--force]".into(),
    );
    m.insert(
        "skill.usage_add".into(),
        "วิธีใช้: skill add <ชื่อ> to <harness> [--global] [--force]".into(),
    );
    m.insert(
        "skill.usage_remove".into(),
        "วิธีใช้: skill remove <ชื่อ> from <harness> [--global] [--force]".into(),
    );
    m.insert(
        "skill.usage_delete".into(),
        "วิธีใช้: skill delete <ชื่อ> [--force]".into(),
    );
    m.insert("skill.usage_edit".into(), "วิธีใช้: skill edit <ชื่อ>".into());
    m.insert(
        "skill.usage_info".into(),
        "วิธีใช้: skill info <harness>".into(),
    );
    m.insert("skill.usage_sub".into(), "วิธีใช้: skill sub <ชื่อ> [ls]".into());
    m.insert(
        "skill.empty".into(),
        "ยังไม่มีสกิลที่ติดตั้ง ลอง 'skill download superpowers' หรือ 'skill download frontenddev'".into(),
    );
    m.insert("skill.installed_header".into(), "สกิลในคลัง ntc:".into());
    m.insert("skill.bundles_header".into(), "ชุดสกิลที่ติดตั้ง:".into());
    m.insert("skill.bundle_tag".into(), "[{}]".into());
    m.insert("skill.bundle_count".into(), "({} สกิล)".into());
    m.insert(
        "skill.list_all_hint".into(),
        "รัน 'skill list --all' เพื่อดูทุกสกิล".into(),
    );
    m.insert("skill.avail_header".into(), "สกิลที่ ntc ดาวน์โหลดได้:".into());
    m.insert("skill.avail_all".into(), "{} (ทุกสกิล)".into());
    m.insert(
        "skill.avail_hint".into(),
        "ติดตั้งด้วย 'skill download <ชื่อ>'".into(),
    );
    m.insert("skill.info_header".into(), "สกิลใน harness {}:".into());
    m.insert(
        "skill.info_empty".into(),
        "ไม่พบสกิลสำหรับ {} (โฟลเดอร์โปรเจกต์หรือส่วนตัว)".into(),
    );
    m.insert("skill.info_ntc".into(), "[ntc]".into());
    m.insert("skill.info_manual".into(), "[เอง]".into());
    m.insert("skill.sub_header".into(), "สกิลใน {}:".into());
    m.insert("skill.sub_none".into(), "{} ไม่มีสกิลย่อย".into());
    m.insert("skill.scope_project".into(), "โปรเจกต์".into());
    m.insert("skill.scope_global".into(), "ส่วนตัว".into());
    m.insert("skill.downloaded".into(), "เพิ่ม {} ({})".into());
    m.insert("skill.added".into(), "{} -> {}".into());
    m.insert("skill.removed".into(), "{} ({})".into());
    m.insert("skill.deleted".into(), "{} (คลัง)".into());
    m.insert("skill.edit_opened".into(), "กำลังแก้ไข {}".into());
    m.insert(
        "skill.edit_refreshed".into(),
        "อัปเดตทะเบียนของ {} แล้ว".into(),
    );
    m.insert("skill.edit_invalid".into(), "{}".into());
    m.insert(
        "skill.harness_pick".into(),
        "จะคัดลอกสกิลไปยัง harness ใด?".into(),
    );
    m.insert("skill.enter_number".into(), "ใส่หมายเลข (1-{}): ".into());
}
