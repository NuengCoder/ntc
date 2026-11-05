use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.skill.title".into(),
        "skill - ติดตั้งสกิล AI (SKILL.md) เข้าโฟลเดอร์ AI harness".into(),
    );
    m.insert(
        "help.skill.download".into(),
        "skill download <alias|owner/repo[/skill]|url> [--name <ชื่อ>] ## ดาวน์โหลดสกิลจาก GitHub"
            .into(),
    );
    m.insert(
        "help.skill.add".into(),
        "skill add <ชื่อ> to <harness> [--global] ## คัดลอกสกิล (หรือชุด) เข้า harness".into(),
    );
    m.insert(
        "help.skill.remove".into(),
        "skill remove <ชื่อ> from <harness> [--global] ## ลบสำเนาใน harness".into(),
    );
    m.insert(
        "help.skill.delete".into(),
        "skill delete <ชื่อ> ## ลบสกิลออกจากคลัง (ตรงข้ามกับ install)".into(),
    );
    m.insert(
        "help.skill.edit".into(),
        "skill edit <ชื่อ> ## เปิด SKILL.md ใน ntceditor".into(),
    );
    m.insert(
        "help.skill.list".into(),
        "skill [list] ## แสดงชุดสกิลที่ดาวน์โหลดและสำเนาใน harness".into(),
    );
    m.insert(
        "help.skill.list_all".into(),
        "skill list --all ## แสดงทุกสกิลที่ติดตั้ง ไม่ใช่แค่ชุด".into(),
    );
    m.insert(
        "help.skill.avail".into(),
        "skill avail ## แสดงสกิลที่ ntc รองรับ".into(),
    );
    m.insert(
        "help.skill.info".into(),
        "skill info <harness> ## ดูว่ามีสกิลใดในโฟลเดอร์ harness".into(),
    );
    m.insert(
        "help.skill.sub".into(),
        "skill sub <ชื่อ> [ls] ## แสดงสกิลในชุดที่ติดตั้ง".into(),
    );
    m.insert(
        "help.skill.help".into(),
        "skill help ## แสดงวิธีใช้ skill".into(),
    );
    m.insert(
        "help.skill.example1".into(),
        "skill download superpowers ## 15 สกิลจาก obra/superpowers".into(),
    );
    m.insert(
        "help.skill.example2".into(),
        "skill download frontenddev ## เฉพาะ frontend-design จาก anthropics/skills".into(),
    );
    m.insert(
        "help.skill.example3".into(),
        "skill add superpower to opencode ## .opencode/skills ในโฟลเดอร์ปัจจุบัน".into(),
    );
    m.insert(
        "help.skill.example4".into(),
        "skill add brainstorming to claude --global ## ~/.claude/skills".into(),
    );
    m.insert(
        "help.skill.example5".into(),
        "skill remove superpowers from opencode ## เก็บสำเนาในคลังไว้".into(),
    );
    m.insert(
        "help.skill.example6".into(),
        "skill download --url https://github.com/obra/superpowers ## เหมือนใช้ alias".into(),
    );
    m.insert(
        "help.skill.example7".into(),
        "skill avail ## superpowers, frontenddev, mcp-builder, sentry-fix-issues, ...".into(),
    );
    m.insert(
        "help.skill.example8".into(),
        "skill info opencode ## .opencode/skills มีอะไรอยู่ตอนนี้".into(),
    );
    m.insert(
        "help.skill.example9".into(),
        "skill sub superpowers ls ## ทุกสกิลในชุด superpowers".into(),
    );
    m.insert(
        "help.skill.example10".into(),
        "skill download --url <url> --name my-skill ## ติดตั้งด้วยชื่ออื่น".into(),
    );
    m.insert(
        "help.compact.skill".into(),
        "ติดตั้งสกิล AI เข้าโฟลเดอร์ AI harness".into(),
    );
}
