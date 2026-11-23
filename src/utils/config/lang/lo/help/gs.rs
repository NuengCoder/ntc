use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.gs.title".into(), "GS ຄຳສັ່ງ:".into());

    m.insert(
        "help.gs.usage".into(),
        "gs <ຮູບແບບ> [-p <glob>] [-s [line|path|size]] [-d <n>] [-l <n>] [-A <n>] [-B <n>] [-I] [-x] [-h|-g|-w] [-z] ## ຄົ້ນຫາເນື້ອໃນໄຟລ໌ (grep ງ່າຍໆ)".into(),
    );

    m.insert(
        "help.gs.interactive".into(),
        "gs -I [ຮູບແບບ] [-d <n>] ## grep ແບບໂຕ້ຕອບ (10/ໜ້າ, f=ຮູບແບບ, e/Enter=ເປີດ, 0-9=ແຖວ, ?=ປຸ່ມ, x/h/g/w/z=ສະຫຼັບ)".into(),
    );

    m.insert(
        "help.gs.pipe".into(),
        "cmd | gs <ຮູບແບບ> ## ຄົ້ນຫາເນື້ອໃນທີ່ pipe ເຂົ້າມາ (qur -s <ຮູບແບບ> ກໍໄດ້)".into(),
    );

    m.insert(
        "help.gs.context".into(),
        "gs <ຮູບແບບ> -A <n> -B <n> ## ສະແດງ n ແຖວຫຼັງ/ກ່ອນແຕ່ລະຈຸດທີ່ກົງ (10 ລາຍການ ເວັ້ນແຕ່ໃຊ້ -l <n>)".into(),
    );

    m.insert(
        "help.gs.filters".into(),
        "gs -x <ຂໍ້ຄວາມກົງຕົວ> -h <ເລີ່ມ> -g <ສິ້ນສຸດ> -w <ທັງແຖວ> -z <ຕົວພິມ> ## -x ຄົ້ນຕາມຂໍ້ຄວາມຈິງ (a\\|b ຄື 'a|b' ບໍ່ແມ່ນ OR); -h/-g/-w ທຽບແຖວທີ່ຕັດຊ່ອງຫວ່າງ ແລະ ໃຊ້ຮ່ວມກັນບໍ່ໄດ້; -z ບໍ່ສົນຕົວພິມນ້ອຍໃຫຍ່".into(),
    );

    m.insert(
        "help.gs.example1".into(),
        "gs fn main ## ຄົ້ນຫາ 'fn main' ໃນໄຟລ໌".into(),
    );

    m.insert(
        "help.gs.example2".into(),
        "gs -I config ## ຜົນການຄົ້ນຫາແບບໂຕ້ຕອບ".into(),
    );

    m.insert(
        "help.gs.example3".into(),
        "gs TODO < app.log ## ຄົ້ນຫາເນື້ອໃນທີ່ pipe/redirect ເຂົ້າມາ".into(),
    );

    m.insert(
        "help.gs.example4".into(),
        "gs error\\|warning ## ຄົ້ນຫາແບບ OR: ກົງກັບຄຳໃດຄຳໜຶ່ງ (a\\|b)".into(),
    );

    m.insert(
        "help.gs.example5".into(),
        "gs main -A 3 -B 2 ## ແຖວທີ່ກົງ ພ້ອມ 3 ແຖວຕໍ່ໄປ ແລະ 2 ແຖວກ່ອນໜ້າ".into(),
    );

    m.insert(
        "help.gs.example6".into(),
        "gs -w main ## ສະເພາະແຖວທີ່ເປັນ 'main' ທັງແຖວ".into(),
    );

    m.insert(
        "help.gs.example7".into(),
        "gs -z Main -h ## ຕົວພິມກົງ: ແຖວທີ່ເລີ່ມດ້ວຍ 'Main'".into(),
    );
}
