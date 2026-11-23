use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.trim.title".into(), "TRIM ຄຳສັ່ງ:".into());

    m.insert(
        "help.trim.usage".into(),
        "trim <pattern> [ຂໍ້ຄວາມ] ## ລຶບທຸກຄັ້ງທີ່ພົບ <pattern> (ຈາກຂໍ້ຄວາມ ຫຼື ອິນພຸດ pipe)".into(),
    );

    m.insert(
        "help.trim.example1".into(),
        "echo nice ... bro ... | trim \" ...\" ## ຕັດ pattern ອອກຈາກຂໍ້ຄວາມທີ່ສົ່ງຜ່ານ pipe (ແບບ inline: trim \" ...\" \"nice ... bro ...\")".into(),
    );
}
