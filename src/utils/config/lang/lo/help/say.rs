use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.say.title".into(), "SAY ຄຳສັ່ງ:".into());

    m.insert(
        "help.say.usage".into(),
        "say <text> [--loop N] [-n|-t] ## ພິມຂໍ້ຄວາມ (ບໍ່ redirect ໄຟລ໌)".into(),
    );

    m.insert(
        "help.say.example1".into(),
        "say \"hello\" ## ພິມຂໍ້ຄວາມ".into(),
    );
}
