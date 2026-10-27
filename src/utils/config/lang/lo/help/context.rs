use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.context.cr".into(), "<cr> ກົດ Enter ເພື່ອແລ່ນຄຳສັ່ງ".into());

    m.insert(
        "help.context.qur.needs_glob".into(),
        "qur <glob> [-s \"pattern\"] [-rp \"new\"] [-dp] [-d <n>] [-l <n>]".into(),
    );

    m.insert(
        "help.context.qur.select_value".into(),
        "qur -s <pattern>  (pattern ເນື້ອໃນ, ບໍ່ສົນຕົວພິມນ້ອຍໃຫຍ່)".into(),
    );

    m.insert(
        "help.context.qur.replace_value".into(),
        "qur -rp <replacement>  (ຕ້ອງໃຊ້ -s \"<pattern>\")".into(),
    );

    m.insert(
        "help.context.qur.depth_value".into(),
        "qur -d <n>  (ຄວາມເລິກສູງສຸດ, 0 = ສະເພາະໂຟນເດີປັດຈຸບັນ)".into(),
    );

    m.insert(
        "help.context.qur.limit_value".into(),
        "qur -l <n>  (ສະແດງພຽງ n ລາຍການ/ໄຟລ໌ທຳອິດ)".into(),
    );

    m.insert(
        "help.context.qur.next_select".into(),
        "qur <glob> -s \"<pattern>\" | -d <n> | -l <n>".into(),
    );

    m.insert(
        "help.context.qur.next_display_replace".into(),
        "qur <glob> -s \"<pattern>\" -dp | -rp \"<new>\" [-d <n>] [-l <n>]".into(),
    );

    m.insert(
        "help.context.qur.next_depth".into(),
        "qur <glob> -s \"<pattern>\" [-dp | -rp \"<new>\"] [-d <n>] [-l <n>]  (<cr> ເພື່ອລັນ)".into(),
    );
}
